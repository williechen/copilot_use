# Analytics Day37 r3｜拒絕歧義 Session Cookie，兩種統計路由共用 Guard（驗證更新）

> **2026-10-25 驗證更新**；精確來源 [`rust_ai_ring@648b90323af7b381d6a79dc2fd08a53b63173a43`](https://github.com/williechen/rust_ai_ring/tree/648b90323af7b381d6a79dc2fd08a53b63173a43)。本次產品快照唯讀，未推送產品。**本次結果：完整 PostgreSQL targets 45 passed／1 ignored，原略過項另跑1 passed；Day37 真 Router／兩種 summary 的重複 Cookie 驗證通過。** 已見修正 parser 與共用 guard；沒有以 Cookie helper 代替正式 HTTP Router。CI仍被先前 Pet native 依賴失敗阻擋；角色／來源 ACL、SSE及瀏覽器未驗收。
>
> 本修訂保留 [Day37 r2 原文](analytics-r2.md) 與其 commit/blob/SHA256；只更新驗證狀態，不新增 Day。三案 next_day 維持38，Gate2未完整通過、不封存。[完整驗證與失敗恢復](../../../source-review/2026-10-25-day37-verification-648b903.md) · [原始 logs／AX／版本證據包](../../../source-review/day37-verification-648b903/evidence.zip)。本篇獨立提交與精確讀回結果見 `analytics-r3-readback.json`。

## ① 學習大綱

已核對初始化／登入／強制改密碼／有效 Session／Leptos Dashboard 均有真 caller。Day36 已將精確事件統計路由改為人員 Session。原 Day37 草稿提出未篩選 summary 改用 `require_query_session`，但既有 `session` 加 restricted 判斷**行為相同**；原 Cookie 單測也已可通過，不能當新增 TDD Red。

本修訂只完成一個 Session 邊界切片：`raw_cookie` 目前 `.get(COOKIE)`／`.find(...)` 取第一個 session 值，對同名重複或多條 Cookie 可能默認選一個。改成拒絕歧義，不論順序／相同值；合法單值 session 仍可讀。正式兩種 summary 路由使用同一 `require_query_session`；這是同切片的去重 Refactor，不另宣稱新 ACL 功能。

真鏈：Leptos Dashboard 使用登入 Cookie，`password_login::build` 路由 → `session`／guard → PostgreSQL 人員／session 驗證 → `pg_store::summary_for_source` 或 `analytics_persistence::summary_for_source_and_type` → JSON／Cookie 續期。Dashboard 的 SSR 資料仍沿既有 session，parser 修補也會保護它；不新增 Axum HTML 畫面、不把空的 analytics-ui/application/domain 稱已接線。角色／來源 ACL 仍缺，管理員名稱檢查不是完整角色授權。

## ② Red → 完整 Green → Refactor

以下完整 Red／Green／Refactor 保留 r2 的 `5c713553` 教材基準；其中「來源尚無」「預期失敗」只限原教材時點。當前648b903已含Green，不要重複套用；本次跑的是已實作版本的測試，沒有重演原始Red。程式與產品判定以下方第④節及驗證報告為準。

### Red：以真正 Router、真 PostgreSQL Session 驗歧義 Cookie

新增完整測試檔 `analytics_apps/analytics_server/tests/day37_query_cookie.rs`，沿現有 `postgres-integration` fixture 與 migration 路徑；不測私有 parser 代替正式路由。來源程式成功單值、source token 拒絕等是基準斷言；**真正 Red 是有效 Cookie 在前、重複值在後或第二條 header 時，期待 401，來源仍預期 200**。此為行為 Red，不是缺符號，也不是已失敗的實測宣稱。

```rust
#![cfg(feature = "postgres-integration")]
use analytics_server::{bootstrap, password_login, IngestState, IngestTokens};
use axum::{body::Body, http::{header, Request, StatusCode}, Router};
use std::sync::Arc;
use tower::ServiceExt;

async fn query(app: &Router, path: &str, cookies: &[String], bearer: bool)
    -> axum::response::Response
{
    let mut request = Request::get(path).body(Body::empty()).unwrap();
    for cookie in cookies {
        request.headers_mut().append(header::COOKIE, cookie.parse().unwrap());
    }
    if bearer {
        request.headers_mut().insert(header::AUTHORIZATION,
            "Bearer fixture-chat".parse().unwrap());
    }
    app.clone().oneshot(request).await.unwrap()
}

#[sqlx::test(migrations = "../migrations")]
async fn day37_real_query_routes_reject_ambiguous_session_cookies(pool: sqlx::PgPool) {
    bootstrap::initialize(&pool, "day37-first-password".into()).await.unwrap();
    // Fixture 狀態：已完成首次改密碼；不把直接 SQL 當 UI 流程驗收。
    sqlx::query("UPDATE uac_user SET is_init_pd=false WHERE lower(user_acnt)='admin'")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO product_events(source_app,event_id,schema_version,event_type)
        VALUES('web_chat',$1,1,'chat.message.persisted')")
        .bind(uuid::Uuid::new_v4().to_string()).execute(&pool).await.unwrap();
    let app = password_login::build(IngestState {
        pool: pool.clone(),
        tokens: Arc::new(IngestTokens {chat:"fixture-chat".into(),pet:"fixture-pet".into()}),
    });
    let login = app.clone().oneshot(Request::post("/login")
        .header(header::ORIGIN,"http://127.0.0.1:3100")
        .header(header::CONTENT_TYPE,"application/x-www-form-urlencoded")
        .body(Body::from("username=admin&password=day37-first-password")).unwrap())
        .await.unwrap();
    assert_eq!(login.status(), StatusCode::SEE_OTHER);
    assert_eq!(login.headers()[header::LOCATION], "/dashboard");
    let cookie = login.headers()[header::SET_COOKIE].to_str().unwrap()
        .split(';').next().unwrap().to_owned();
    let invalid = format!("analytics_session={}", "0".repeat(64));
    let paths = ["/api/v1/events/web_chat/summary",
        "/api/v1/events/web_chat/summary/chat.message.persisted"];
    for path in paths {
        assert_eq!(query(&app,path,&[],false).await.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(query(&app,path,&[],true).await.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(query(&app,path,&[cookie.clone()],false).await.status(), StatusCode::OK);
        for ambiguous in [
            vec![format!("{cookie}; {invalid}")],
            vec![cookie.clone(), invalid.clone()],
            vec![invalid.clone(), cookie.clone()],
            vec![format!("{cookie}; {cookie}")],
            vec![format!("{cookie}; analytics_session")],
        ] {
            let response = query(&app,path,&ambiguous,false).await;
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{path}");
            assert!(!response.headers().contains_key(header::SET_COOKIE));
        }
        // 不同 cookie 名稱仍合法；拒絕歧義不應撤銷原有效 session。
        assert_eq!(query(&app,path,&[format!("theme=dark; {cookie}")],false)
            .await.status(), StatusCode::OK);
    }
    sqlx::query("UPDATE uac_user SET is_init_pd=true WHERE lower(user_acnt)='admin'")
        .execute(&pool).await.unwrap();
    for path in paths {
        assert_eq!(query(&app,path,&[cookie.clone()],false).await.status(), StatusCode::FORBIDDEN);
    }
    sqlx::query("UPDATE uac_user SET is_init_pd=false WHERE lower(user_acnt)='admin'")
        .execute(&pool).await.unwrap();
    let logout = app.clone().oneshot(Request::post("/logout")
        .header(header::ORIGIN,"http://127.0.0.1:3100")
        .header(header::COOKIE,&cookie).body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(logout.status(),StatusCode::SEE_OTHER);
    for path in paths {
        assert_eq!(query(&app,path,&[cookie.clone()],false).await.status(), StatusCode::UNAUTHORIZED);
    }
}
```

測試不輸出 Cookie／token；fixture 直接設定首次密碼狀態只建立測試前提，不能宣稱走過 UI 改密碼。`bootstrap::initialize` 是既有 fixture 真 caller，保留它與原測試。

### Green：只接受唯一且格式有效的人員 Session

在 `analytics_apps/analytics_server/src/password_login.rs` **完整取代 `raw_cookie`**，imports 原已含 HeaderMap／header：

```rust
fn raw_cookie(headers: &HeaderMap) -> Option<String> {
    let mut session = None;
    // HTTP 可以有多條 Cookie，不能只讀第一條。
    for value in headers.get_all(header::COOKIE).iter() {
        let line = value.to_str().ok()?;
        for part in line.split(';').map(str::trim) {
            if let Some((name, raw)) = part.split_once('=') {
                if name != "analytics_session" {
                    continue;
                }
                if session.is_some()
                    || raw.len() != 64
                    || !raw.bytes().all(|c| c.is_ascii_hexdigit())
                {
                    return None;
                }
                session = Some(raw.to_owned());
            } else if part == "analytics_session" {
                return None;
            }
        }
    }
    session
}
```

所有使用 `raw_cookie` 的登入後流程使用同一格式政策，無 Cookie 或重複同名直接 None，`session` 在任何 PostgreSQL 查詢之前回 401；未知 cookie 名稱允許，token 保持 64 位十六進位。不同 Path 所送同名 cookie 也拒絕；使用者需清除舊 cookie 後重登，不從順序猜授權。此修補不自動修正共享登入實作的既有架構債。

### Refactor：未篩選統計也重用 Guard

同檔 **完整替換 `summary`**：

```rust
async fn summary(
    State(state): State<IngestState>,
    Path(source): Path<String>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    let raw = require_query_session(&state, &headers).await?;
    if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
        return Err(StatusCode::BAD_REQUEST);
    }
    let rows = summary_for_source(&state.pool, &source)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(renewed(Json(rows), &raw))
}
```

精確類型 `analytics_server/src/lib.rs::source_kind_summary` 已呼叫同一 guard，不再重複套用 Day36；路由 `.route("/api/v1/events/{source}/summary", get(summary))` 與 exact-type 註冊保留。`require_query_session` 先檢有效帳號／期限／密碼 fingerprint，再拒絕 restricted；本次不改 SQLx Session SQL、續期、來源／事件型別校驗、聚合查詢與 Cookie 屬性。`POST /api/v1/events` 繼續使用機器 token，不套人員 Cookie parser；成功 ingestion 並不代表後台登入。

Cargo：workspace `analytics_apps`；package `analytics-server`、crate `analytics_server`；既有 axum／sqlx postgres／argon2／tokio／sha2、dev tower util、uuid 與 `postgres-integration` feature 足夠，不新增 imports（產品）、dependencies 或 features。四個 Cargo manifests／lockfiles 不改；沒有 migration、新 DB 或 cli 安裝。Analytics 未設定 Chat 的 `[[workspace.metadata.leptos]]`，不虛構共用 metadata。

### 上一 Day 未完成事項與舊稿更正補充

[Day36 r2](../day-36/analytics-r2.md) 的來源 token 隔離已存在且曾局部實測；本篇不重教。原 Day37 的 Cookie 回歸測試是基準，不是 Red；新測試固定走正式 build Router，Green 影響的就是相同路由 parser。角色／來源 ACL、帳號管理、多帳號登入、權限立即撤銷、CSRF、SSE 與瀏覽器 UI 完整流程仍未驗，沿 [既有 FUTURE_WORK](../../../FUTURE_WORK.md) 追蹤，不新增平行清單或結案 FIFO。共用 guard 不代表已補上來源 ACL。

## ③ 套件與軟體版本

交付日 2026-10-25，直接讀官方 crates.io API 的 versions，排除預發行與 yanked；不是以 Cargo.lock 猜測最新版。逐項官方網址、發行日期與 MSRV 原始回應見 [本次官方版本重新查核](../../../source-review/day37-verification-648b903/official-versions.json)。四個 workspace 的 manifest／lock 已讀；下表為本篇使用範圍。

| 項目 | manifest 宣告 | lock resolved | 當日官方最新穩定 | MSRV |
| --- | --- | --- | --- | --- |
| axum | `0.8` | 0.8.9 | [0.8.9](https://crates.io/crates/axum/0.8.9) | 1.80 |
| sqlx | `0.9` | 0.9.0 | [0.9.0](https://crates.io/crates/sqlx/0.9.0) | 1.94.0 |
| argon2 | `0.6` | 0.6.0 | [0.6.0](https://crates.io/crates/argon2/0.6.0) | 1.85 |
| leptos | `0.8 / ssr` | 0.8.22 | [0.8.22](https://crates.io/crates/leptos/0.8.22) | 1.88 |
| tokio | `1.54` | 1.54.1 | [1.54.1](https://crates.io/crates/tokio/1.54.1) | 1.85 |
| tower | `0.5 / util` | 0.5.3 | [0.5.3](https://crates.io/crates/tower/0.5.3) | 1.64.0 |
| sha2 | `0.11` | 0.10.9, 0.11.1 | [0.11.1](https://crates.io/crates/sha2/0.11.1) | 1.88 |
| serde | `1.0` | 1.0.229 | [1.0.229](https://crates.io/crates/serde/1.0.229) | 1.56 |
| serde_json | `1.0` | 1.0.154 | [1.0.154](https://crates.io/crates/serde_json/1.0.154) | 1.71 |
| uuid | `1.29` | 1.29.0 | [1.29.0](https://crates.io/crates/uuid/1.29.0) | 1.89.0 |

Rust 官方 stable manifest 為 **1.99.1**（2026-10-15），[官方公告](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)；本機 `rustc 1.99.1`、`cargo 1.99.1`，Edition 2024，workspace 未固定 rust-version。所有上列已宣告 MSRV 均不高於本機 toolchain。建議採表列當日穩定版本；產品 lock 已相符者保留，無需 cargo update；相同 lock 的 native／WASM 相容仍須第④節實測，不能從 MSRV 推定成功。sha2 0.10.9 若見於 lock 屬並存的傳遞依賴，不任意強制升成 0.11。

PostgreSQL 當日官方最新穩定 **18.6**（[版本政策](https://www.postgresql.org/support/versioning/)、[完整 release notes](https://www.postgresql.org/docs/release/)）；19 RC1 是預發行，排除。`pg_config --version` 本機 client/dev package 18.6，不代表已連線取得 server_version。現有 PG SQL、型別與 migrations 不變，建議使用隔離 18.6 依 SQLx 0.9.0 全新建庫測試；本次未啟動／修改／刪建任何資料庫。版本相符不是整合測試成功。
當日版本資料已於本次重新查核（見連結的 checked_at）；本次測試快照的四個 workspace 來源雜湊記於 summary.json。原 r2 版本檔與查核時間不覆寫。

## ④ 本次實際驗證、失敗與界線

**本次結果：完整 PostgreSQL targets 45 passed／1 ignored，原略過項另跑1 passed；Day37 真 Router／兩種 summary 的重複 Cookie 驗證通過。** 已見修正 parser 與共用 guard；沒有以 Cookie helper 代替正式 HTTP Router。CI仍被先前 Pet native 依賴失敗阻擋；角色／來源 ACL、SSE及瀏覽器未驗收。

本次在隔離 Git 快照的對應 workspace 執行（Cargo cache 在快照外，不變更來源；完整退出碼與 log見證據包）：

```bash
# DATABASE_URL 指向本次獨立測試 DB，非正式 DB；由該角色持有並可建立 SQLx fixture。
cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration
# 原略過項的程式實際讀取 DATABASE_URL，不能只照舊 ignore 註記設另一變數。
cargo test --locked -p analytics-server --features postgres-integration --lib admin::tests::login_query_postgres_and_logout_flow -- --ignored
```

PG先遇anonymous peer拒絕與postgres schema權限不足；新建獨立DB後45項通過，1項略過另跑通過。Ignored註記與實際環境變數不一致的失敗也保留，不隱藏重試。正式Day37 Router整合測試通過不是角色ACL或Leptos瀏覽器驗收。

同648b903共有6個workflow成功、1個Persistence失敗。失敗job121196631294在Test Pet Persistence執行包含pet-server的測試，因缺glib-2.0.pc／gobject-2.0.pc而exit101；後續已配置的完整Analytics PG步驟被skipped。本機測試成功不能替代CI，也沒有在本次修正或重跑workflow。[CI快照](../../../source-review/day37-verification-648b903/ci.json)。

保留產品實作、局部測試、CI與功能驗收的獨立認定；不修改last_full_comparison、FIFO、舊文章或封存位置。
