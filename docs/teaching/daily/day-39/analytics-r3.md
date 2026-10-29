# Analytics Day39 r3｜Session 匯出指定來源 CSV：現行 Persistence 接線與真 PG 驗證

> 2026-10-26：按最新已實作程式及本地驗證修訂；此為前一輪本地修訂，隨本次Day40歷史勘誤提交；下文未push等敘述限原驗證時點。原[初版](analytics.md)、[r1](analytics-r1.md)、[r2](analytics-r2.md)及舊 patch 保留。Day40、FIFO、last_full_comparison 均不變。

產品來源 `williechen/rust_ai_ring@bb259aaf76542554a3a704d9c4f4a3a219d65cbf`；文件編修基底 `d8d9f4ddf46d923c6abcc83ef0b86e5439eee4c4`。本文程式逐段取自此精確來源，[來源片段與 hashes](r3-source-excerpts.json)可核對。共用規則與需求依文件基底版本；本修訂整理既有 Day39，沒有新增一天的功能或重構計功。

## ① 目標、現況與前置條件

管理員以正式人員 Session 匯出選定來源的聚合 CSV；機器憑證不能代替 Session。最新程式已具備 Dashboard Leptos 連結、正式路由、CSV handler、Persistence 聚合與完整 Router／PG 測試，不再標示為未套用候選。

前置：PostgreSQL、獨立可建立 SQLx 測試資料庫的帳號、正式 migrations；不得用產品庫。首次改密碼限制、來源白名單及登出後拒絕仍由正式路徑強制。

## Refactor

### SQL 責任集中於 Persistence；修正 r2 的相容入口描述

舊基底將只按來源聚合留在 server 的 pg_store，exact-kind 已在 Persistence。最新程式將 `summary_for_source` 移到既有 Persistence，讓 Dashboard、JSON API、CSV 共用 SQL 與 `EventSummary`。與 r2 候選不同，**目前 pg_store 只有私有 use，不是 public re-export**；`lib.rs`、`password_login.rs`、`export.rs` 均直接 import Persistence。不能沿用 r2「舊 pg_store 公開函式仍相容」的說明。

完整 Persistence 檔（含舊 exact-kind、rollback 及新來源隔離／排序測試）：

來源：[`analytics_apps/analytics_persistence/src/lib.rs`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/analytics_apps/analytics_persistence/src/lib.rs#L1)。

```rust
/// PostgreSQL 聚合查詢結果，供 JSON API 與 Leptos SSR 共用。
#[derive(Debug, serde::Serialize, sqlx::FromRow, PartialEq, Eq)]
pub struct EventSummary {
    pub source_app: String,
    pub event_type: String,
    pub total: i64,
}

pub async fn summary_for_source(
    pool: &sqlx::PgPool,
    source: &str,
) -> Result<Vec<EventSummary>, sqlx::Error> {
    sqlx::query_as::<_, EventSummary>(
        "SELECT source_app, event_type, COUNT(*)::bigint AS total
         FROM product_events
         WHERE source_app = $1
         GROUP BY source_app, event_type
         ORDER BY event_type",
    )
    .bind(source)
    .fetch_all(pool)
    .await
}

/// 僅聚合已提交的精確來源與事件類型；輸入不拼入 SQL。
pub async fn summary_for_source_and_type(
    pool: &sqlx::PgPool,
    source: &str,
    event_type: &str,
) -> Result<Vec<EventSummary>, sqlx::Error> {
    sqlx::query_as::<_, EventSummary>(
        "SELECT source_app, event_type, COUNT(*)::bigint AS total
         FROM product_events
         WHERE source_app = $1 AND event_type = $2
         GROUP BY source_app, event_type",
    )
    .bind(source)
    .bind(event_type)
    .fetch_all(pool)
    .await
}

#[cfg(all(test, feature = "postgres-integration"))]
mod day35_exact_kind_adapter_tests {
    use super::{EventSummary, summary_for_source_and_type};

    #[sqlx::test(migrations = "../migrations")]
    async fn exact_kind_counts_committed_rows_only_for_requested_source(pool: sqlx::PgPool) {
        for (source, kind) in [
            ("web_chat", "chat.message.persisted"),
            ("web_chat", "chat.message.persisted"),
            ("web_chat", "chat.room.created"),
            ("desktop_pet", "chat.message.persisted"),
        ] {
            sqlx::query(
                "INSERT INTO product_events (source_app,event_id,schema_version,event_type)
                 VALUES ($1,$2,1,$3)",
            )
            .bind(source)
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(kind)
            .execute(&pool)
            .await
            .unwrap();
        }
        let chat = summary_for_source_and_type(&pool, "web_chat", "chat.message.persisted")
            .await
            .unwrap();
        assert_eq!(
            chat,
            vec![EventSummary {
                source_app: "web_chat".into(),
                event_type: "chat.message.persisted".into(),
                total: 2,
            }]
        );
        let pet = summary_for_source_and_type(&pool, "desktop_pet", "chat.message.persisted")
            .await
            .unwrap();
        assert_eq!(pet[0].total, 1);
        assert_eq!(pet.len(), 1);
        assert!(
            summary_for_source_and_type(&pool, "web_chat", "' OR TRUE --")
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            summary_for_source_and_type(&pool, "web_chat", "not.recorded")
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn rolled_back_event_is_not_counted(pool: sqlx::PgPool) {
        let mut tx = pool.begin().await.unwrap();
        sqlx::query(
            "INSERT INTO product_events (source_app,event_id,schema_version,event_type)
             VALUES ('web_chat',$1,1,'chat.message.persisted')",
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
        tx.rollback().await.unwrap();
        assert!(
            summary_for_source_and_type(&pool, "web_chat", "chat.message.persisted")
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn all_kinds_are_sorted_and_scoped_to_one_source(pool: sqlx::PgPool) {
        for (source, kind) in [
            ("web_chat", "z.last"),
            ("web_chat", "a.first"),
            ("web_chat", "a.first"),
            ("desktop_pet", "pet.only"),
        ] {
            sqlx::query("INSERT INTO product_events(source_app,event_id,schema_version,event_type) VALUES($1,$2,1,$3)")
                .bind(source).bind(uuid::Uuid::new_v4().to_string()).bind(kind)
                .execute(&pool).await.unwrap();
        }
        let rows = super::summary_for_source(&pool, "web_chat").await.unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(
            (&rows[0].event_type, rows[0].total),
            (&"a.first".to_owned(), 2)
        );
        assert_eq!(
            (&rows[1].event_type, rows[1].total),
            (&"z.last".to_owned(), 1)
        );
        assert!(rows.iter().all(|r| r.source_app == "web_chat"));
        for source in ["not.recorded", "' OR TRUE --"] {
            assert!(
                super::summary_for_source(&pool, source)
                    .await
                    .unwrap()
                    .is_empty()
            );
        }
    }
}
```

server 現行 imports 以來源原文呈現：

來源：[`analytics_apps/analytics_server/src/pg_store.rs`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/analytics_apps/analytics_server/src/pg_store.rs#L1)。

```rust
use analytics_persistence::summary_for_source;
use sqlx::PgPool;

use crate::{EventInput, valid_event};
```

來源：[`analytics_apps/analytics_server/src/password_login.rs`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/analytics_apps/analytics_server/src/password_login.rs#L1)。

```rust
use crate::IngestState;
use analytics_persistence::summary_for_source;
use analytics_persistence::summary_for_source_and_type;
```

原 COUNT、group by、來源 bind 與排序斷言保留；注入形狀來源必回空集合，transaction rollback／exact-kind 測試仍執行。這次是已實作邊界的來源對齊，不新增重構完成量。下一步依既有人員 Session／角色、用例 DTO／Port 與查詢 caller 做漸進整理，不能因 SQL 搬檔宣稱整個 Analytics 分層已完成。

## ② Red → 現行 Green、授權與回應

### Red 邊界

原 Red 是新 CSV 路由未註冊時的 Router 404，與匯出成功 200 的斷言不符；本次沒有移除路由重跑 Red。下列完整測試在最新來源為實際 Green，不挪用既有成功測試充作 Red。

### Green：完整 export.rs

現行 CSV handler 直接用 Persistence，並將成功回應經 `password_login::renewed(response, &raw)` 續期 Session；不能沿舊片段丟掉這個正式 caller。CSV 有 BOM、CRLF、雙引號轉義與公式前綴保護；以下含完整 imports、handler 及 escaping 測試：

來源：[`analytics_apps/analytics_server/src/export.rs`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/analytics_apps/analytics_server/src/export.rs#L1)。

```rust
use crate::IngestState;
use analytics_persistence::EventSummary;
use analytics_persistence::summary_for_source;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};

fn csv_cell(value: &str) -> String {
    // Spreadsheet applications may interpret leading operators as formulas.
    let safe = if value
        .chars()
        .next()
        .is_some_and(|c| matches!(c, '=' | '+' | '-' | '@' | '\t' | '\r'))
    {
        format!("'{value}")
    } else {
        value.to_owned()
    };
    format!("\"{}\"", safe.replace('"', "\"\""))
}

pub fn render_csv(rows: &[EventSummary]) -> String {
    let mut csv = String::from("\u{feff}source_app,event_type,total\r\n");
    for row in rows {
        csv.push_str(&format!(
            "{},{},{}\r\n",
            csv_cell(&row.source_app),
            csv_cell(&row.event_type),
            row.total,
        ));
    }
    csv
}

/// Human Session is required; machine Bearer tokens never authorize CSV export.
pub async fn export_summary(
    State(state): State<IngestState>,
    Path(source): Path<String>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    let raw = crate::password_login::require_query_session(&state, &headers).await?;
    if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
        return Err(StatusCode::BAD_REQUEST);
    }
    let rows = summary_for_source(&state.pool, &source)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let mut response = render_csv(&rows).into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        "text/csv; charset=utf-8".parse().expect("static MIME"),
    );
    response.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"analytics-{source}-summary.csv\"")
            .parse()
            .expect("source allowlist"),
    );
    Ok(crate::password_login::renewed(response, &raw))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn csv_escapes_quotes_and_spreadsheet_formulas() {
        let csv = render_csv(&[
            EventSummary {
                source_app: "web_chat".into(),
                event_type: "=1+1".into(),
                total: 2,
            },
            EventSummary {
                source_app: "desktop_pet".into(),
                event_type: "say \"hi\",\nagain".into(),
                total: 1,
            },
        ]);
        assert!(csv.starts_with("\u{feff}source_app,event_type,total\r\n"));
        assert!(csv.contains("\"web_chat\",\"'=1+1\",2\r\n"));
        assert!(csv.contains("\"desktop_pet\",\"say \"\"hi\"\",\nagain\",1\r\n"));
    }
}
```

`analytics_server/src/lib.rs` 宣告 `pub mod export;`。以下完整 `password_login::build` 包含路由、no-store middleware 與正式 state；Dashboard 的 Leptos 連結為 `<a href=format!("/api/v1/events/{source}/export.csv")>"匯出目前來源 CSV"</a>`，沒有新增手刻 HTML 頁面。

來源：[`analytics_apps/analytics_server/src/password_login.rs::build`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/analytics_apps/analytics_server/src/password_login.rs#L456)。

```rust
pub fn build(state: IngestState) -> Router {
    Router::new()
        .route("/login", get(login_page).post(login))
        .route("/logout", post(logout))
        .route("/change-password", get(change_page).post(change_password))
        .route("/", get(dashboard))
        .route("/dashboard", get(dashboard))
        .route("/api/v1/events/{source}/summary", get(summary))
        .route(
            "/api/v1/events/{source}/export.csv",
            get(crate::export::export_summary),
        )
        .route(
            "/api/v1/events/{source}/summary/{event_type}",
            get(crate::source_kind_summary),
        )
        .route("/healthz", get(crate::health))
        .route("/api/v1/events", post(crate::ingest))
        .layer(DefaultBodyLimit::max(4096))
        .layer(axum::middleware::map_response(
            |mut r: Response| async move {
                r.headers_mut()
                    .insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
                r
            },
        ))
        .with_state(state)
}
```

### 完整真 Router／PG 測試與 fixture

測試透過 Tower oneshot 呼叫正式 handler，SQLx 以正式 migrations 建每測試獨立 PG 庫；不是 stub SQL、不是靜態 hunk 比對，也還不是瀏覽器網路下載驗收。

來源：[`analytics_apps/analytics_server/tests/day39_export_csv.rs`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/analytics_apps/analytics_server/tests/day39_export_csv.rs#L1)。

```rust
#![cfg(feature = "postgres-integration")]
mod support;

use analytics_server::bootstrap;
use axum::http::{StatusCode, header};
use support::http::{body_text, get, post_form, response_cookie};

#[sqlx::test(migrations = "../migrations")]
async fn day39_authenticated_admin_exports_only_selected_source_csv(pool: sqlx::PgPool) {
    bootstrap::initialize(&pool, "first-password-39".into())
        .await
        .unwrap();
    for (source, kind) in [
        ("web_chat", "chat.message.persisted"),
        ("web_chat", "chat.room.created"),
        ("desktop_pet", "pet.sleep.started"),
    ] {
        sqlx::query("INSERT INTO product_events(source_app,event_id,schema_version,event_type) VALUES($1,$2,1,$3)")
            .bind(source).bind(uuid::Uuid::new_v4().to_string()).bind(kind)
            .execute(&pool).await.unwrap();
    }
    let app = support::app(pool.clone(), "fixture-chat", "fixture-pet");
    let url = "/api/v1/events/web_chat/export.csv";
    let anonymous = get(app.clone(), url, None).await;
    assert_eq!(anonymous.status(), StatusCode::UNAUTHORIZED);
    let machine = get(app.clone(), url, Some("analytics_session=fixture-chat")).await;
    assert_eq!(machine.status(), StatusCode::UNAUTHORIZED);

    let login = post_form(
        app.clone(),
        "/login",
        "username=admin&password=first-password-39".into(),
        None,
    )
    .await;
    assert_eq!(login.status(), StatusCode::SEE_OTHER);
    let first = response_cookie(&login);
    assert_eq!(
        get(app.clone(), url, Some(&first)).await.status(),
        StatusCode::FORBIDDEN
    );
    let change = get(app.clone(), "/change-password", Some(&first)).await;
    let csrf = body_text(change)
        .await
        .split("name=\"csrf\" value=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap()
        .to_owned();
    let changed = post_form(app.clone(), "/change-password",
        format!("current_password=first-password-39&new_password=next-password-39&confirm_password=next-password-39&email=day39%40example.test&csrf={csrf}"),
        Some(&first)).await;
    assert_eq!(changed.status(), StatusCode::SEE_OTHER);
    let login = post_form(
        app.clone(),
        "/login",
        "username=admin&password=next-password-39".into(),
        None,
    )
    .await;
    assert_eq!(login.status(), StatusCode::SEE_OTHER);
    let cookie = response_cookie(&login);
    let page = get(app.clone(), "/dashboard?source=web_chat", Some(&cookie)).await;
    assert_eq!(page.status(), StatusCode::OK);
    assert!(
        body_text(page)
            .await
            .contains("/api/v1/events/web_chat/export.csv")
    );

    let csv = get(app.clone(), url, Some(&cookie)).await;
    assert_eq!(csv.status(), StatusCode::OK);
    assert_eq!(
        csv.headers()[header::CONTENT_TYPE],
        "text/csv; charset=utf-8"
    );
    assert_eq!(
        csv.headers()[header::CONTENT_DISPOSITION],
        "attachment; filename=\"analytics-web_chat-summary.csv\""
    );
    let body = body_text(csv).await;
    assert!(body.contains("chat.message.persisted"));
    assert!(body.contains("chat.room.created"));
    assert!(!body.contains("pet.sleep.started"));
    let pet = get(
        app.clone(),
        "/api/v1/events/desktop_pet/export.csv",
        Some(&cookie),
    )
    .await;
    assert_eq!(pet.status(), StatusCode::OK);
    let body = body_text(pet).await;
    assert!(body.contains("pet.sleep.started"));
    assert!(!body.contains("chat.message.persisted"));
    assert_eq!(
        get(
            app.clone(),
            "/api/v1/events/unknown/export.csv",
            Some(&cookie)
        )
        .await
        .status(),
        StatusCode::BAD_REQUEST
    );

    let logout = post_form(app.clone(), "/logout", String::new(), Some(&cookie)).await;
    assert_eq!(logout.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        get(app, url, Some(&cookie)).await.status(),
        StatusCode::UNAUTHORIZED
    );
}
```

fixture 只組裝正式 Router／state，傳輸 helper 重用且不複製業務：

來源：[`analytics_apps/analytics_server/tests/support/mod.rs`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/analytics_apps/analytics_server/tests/support/mod.rs#L1)。

```rust
pub mod http;

use analytics_server::{IngestState, IngestTokens, password_login};
use std::sync::Arc;

pub fn app(pool: sqlx::PgPool, chat: &str, pet: &str) -> axum::Router {
    password_login::build(IngestState {
        pool,
        tokens: Arc::new(IngestTokens {
            chat: chat.into(),
            pet: pet.into(),
        }),
    })
}
```

來源：[`analytics_apps/analytics_server/tests/support/http.rs`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/analytics_apps/analytics_server/tests/support/http.rs#L1)。

```rust
// Each integration binary intentionally uses a different subset of shared transport helpers.
#![allow(dead_code)]
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, header},
    response::Response,
};
use tower::ServiceExt;

pub async fn send(app: Router, path: &str, form: Option<&str>, cookie: Option<&str>) -> Response {
    let mut request = if form.is_some() {
        Request::post(path)
            .header(header::ORIGIN, "http://127.0.0.1:3100")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
    } else {
        Request::get(path)
    };
    if let Some(cookie) = cookie {
        request = request.header(header::COOKIE, cookie);
    }
    app.oneshot(
        request
            .body(form.map_or_else(Body::empty, |s| Body::from(s.to_owned())))
            .unwrap(),
    )
    .await
    .unwrap()
}

pub async fn get(app: Router, path: &str, cookie: Option<&str>) -> Response {
    send(app, path, None, cookie).await
}
pub async fn post_form(app: Router, path: &str, form: String, cookie: Option<&str>) -> Response {
    send(app, path, Some(&form), cookie).await
}

pub fn response_cookie(response: &Response) -> String {
    response.headers()[header::SET_COOKIE]
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_owned()
}

pub async fn body_text(response: Response) -> String {
    String::from_utf8(
        to_bytes(response.into_body(), 32768)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap()
}
```

原 admin、首次改密碼、Cookie、多值 Authorization、冪等、rollback 與來源切換測試均保留。機器冒用測試實際使用假 Session Cookie，不能寫成已完整測過每種 Bearer 請求變體；Day38 ingest Authorization 是另一個責任。roles／來源 ACL、下載稽核與真試算表匯入尚未由本切片完成。

Cargo 接線維持 `analytics-server` 依賴 `analytics-persistence`；server 的 `postgres-integration` feature 傳入 Persistence，同時選兩個 package 才有本次完整範圍。沒有新增 migration、feature、lock 或全庫 workflow。

## ③ 來源版本與執行環境

| 項目 | 精確來源或已執行環境 |
| --- | --- |
| Rust / Cargo | 實測 1.99.1；edition 2024 |
| axum | Cargo.lock resolved `0.8.9` |
| leptos | Cargo.lock resolved `0.8.22` |
| sqlx | Cargo.lock resolved `0.9.0` |
| tokio | Cargo.lock resolved `1.54.1` |
| uuid | Cargo.lock resolved `1.29.0` |
| PostgreSQL | 實測 18.6，隔離資料庫與正式 migrations |

四個 workspace 的 manifest／lock 已重新讀取；沒有改版本、依賴或 features。本文列的是來源 resolved 及當次實測版本，不宣稱本輪另做官方「最新穩定版」查核或升級；r2 的官方查核表保留其原日期與範圍。

## ④ 實測命令、證據與未完成

最新精確 SHA 實測：**完整 PG suite 51 通過、1 ignored；該 ignored 另以 --ignored --exact 跑 1 通過，合計 52 個不同測試通過；server check 通過**；`cargo fmt --all -- --check` exit 0。這些已於前一輪驗證實際執行，本輪編修只重核來源與保存證據，沒有重跑或重複計功。

| 命令 | exit code | 原始 log |
| --- | --- | --- |
| `cargo fmt --all -- --check` | 0 | [log](r3-local-validation/logs/latest-analytics-fmt.log) |
| `cargo check --offline --locked -p analytics-server` | 0 | [log](r3-local-validation/logs/latest-analytics-check.log) |
| `cargo test --offline --locked -p analytics-server --lib export::tests` | 0 | [log](r3-local-validation/logs/latest-analytics-test-csv.log) |
| `cargo test --offline --locked -p analytics-server -p analytics-persistence --features postgres-integration` | 0 | [log](r3-local-validation/logs/latest-analytics-test-pg.log) |
| `cargo test --offline --locked -p analytics-server --features postgres-integration --lib admin::tests::login_query_postgres_and_logout_flow -- --ignored --exact` | 0 | [log](r3-local-validation/logs/latest-analytics-test-ignored.log) |

當次工作目錄：`/home/willie/Documents/Codex/2026-10-26/referenced-chatgpt-conversation-this-is-an/work/latest/analytics_apps`。完整工作目錄、環境覆寫、耗時及 log SHA256 見[逐命令紀錄](r3-local-validation/commands.jsonl)。表中 Cargo 以 `--offline --locked` 執行；快取命中不代表跳過測試，log 保留實際測試名稱與結果。

教學舊基底 `8fc4b27` 另有隔離候選：Refactor → Green 的兩份 patch 均先 `git apply --check` exit 0，再 `git apply` exit 0，之後實際 Cargo check/test 通過；但三案候選 fmt 均 exit 1，Chat/Pet 的 diff whitespace 檢查亦失敗。這些舊 patch 不適合重貼最新來源；最新來源正向套用檢查全部失敗，且不等同舊 patch 逐位元結果。新程式直接以最新 SHA 驗證。

尚未執行：真瀏覽器登入／下載、Excel／LibreOffice 中文與公式匯入、使用者角色與來源 ACL、SSE／下載稽核、舊產品 DB 前向升級與 CI。單獨 CSV 單元測試已在完整 suite 內，統計時不重複計數；ignored 舊測試已另外補跑通過。

[完整兩組驗證摘要](r3-local-validation/README.md)保留初次工作目錄被刪除與空間處理中斷；中斷不計成功。`rust_ai_ring` 本輪文件編修保持唯讀；未 push、未執行新 CI、未封存／宣稱 Gate 2 全部完成。
