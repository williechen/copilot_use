# Analytics Day 27｜已登入管理員在後台切換 Chat／Pet 真實事件摘要

## ① 大綱、來源、真實操作缺口

- 最新學習庫 `rust_ai_chat/main@8cf82626c9c82939a3a56eab32e24a8a5b5b00ed`（唯讀）；GitHub 文章正本 `copilot_use/main@8910726bc8d7930eaf56998d54203ff95d8d87cb`；規則版本 `daily-tdd-minimum-v1 / github-only-v1`，規則 blob `bb26b0f3b867a7d3da20fc408dd3ea987c9f41b0`。
- 目前 `apps/stats_server/src/bootstrap.rs` → `password_login.rs::login` → PostgreSQL `uac_login_session` → 首次 `change_password` → `/dashboard` 的持久化鏈已存在；Day26 的新增 migration 亦已在學習庫 main。現有 `GET /api/v1/events/{source}/summary` 可用已登入 Session 查兩來源，但 `dashboard()` 實際只查 `"web_chat"` 且沒有切換入口；管理員無法在畫面看 Pet 事件。
- 本日一個最小 Green：同一受 Session 保護的 `GET /dashboard?source=web_chat|desktop_pet`，由已登入管理員選取來源，沿用 `pg_store::summary_for_source` 的 SQL parameter binding，既有 `crate::summary_total` 加總；未登入／強制改密不能看到資料。不新增 DB schema、秘密、權限繞道。這只對既有唯一 admin 開放，不代表完整 role/source ACL。
- 主體學習 Red + Green 15 分鐘，Refactor、驗證／DB 啟動另估 15–30 分鐘，實際環境以測試結果為準。

## ② 真 Red、完整最小 Green、Refactor／FIFO

### Red（預期 HTTP 行為失敗，沒有冒充實測）

新增下列獨立整合測試，在原始 `dashboard()` 固定 `"web_chat"` 的程式下，選取 `?source=desktop_pet` 時網頁仍顯示 Chat 數量；會在 `Pet 已提交事件總數 2` 的 assertion **失敗**。因此本次 Red 的性質為**預期斷言失敗**，不是編譯錯誤；本次未執行測試。

新檔 `apps/stats_server/tests/day27_dashboard_source.rs` 全文：

```rust
// apps/stats_server/tests/day27_dashboard_source.rs
#![cfg(feature = "postgres-integration")]
use axum::{
    body::{Body,to_bytes},
    http::{header,Request,StatusCode},
};
use stats_server::{bootstrap,password_login,IngestState,IngestTokens};
use std::sync::Arc;
use tower::ServiceExt;

fn app(pool: sqlx::PgPool) -> axum::Router {
    password_login::build(IngestState {
        pool,
        tokens: Arc::new(IngestTokens {
            chat:"test-chat".into(),pet:"test-pet".into(),
        }),
    })
}
async fn get(app: axum::Router,path:&str,cookie:Option<&str>)
    -> axum::response::Response
{
    let mut request=Request::get(path);
    if let Some(cookie)=cookie {
        request=request.header(header::COOKIE,cookie);
    }
    app.oneshot(request.body(Body::empty()).unwrap()).await.unwrap()
}
async fn post(app:axum::Router,path:&str,form:String,cookie:Option<&str>)
    -> axum::response::Response
{
    let mut req=Request::post(path)
        .header(header::ORIGIN,"http://127.0.0.1:3100")
        .header(header::CONTENT_TYPE,"application/x-www-form-urlencoded");
    if let Some(cookie)=cookie {req=req.header(header::COOKIE,cookie);}
    app.oneshot(req.body(Body::from(form)).unwrap()).await.unwrap()
}
fn response_cookie(r:&axum::response::Response)->String {
    r.headers()[header::SET_COOKIE].to_str().unwrap()
        .split(';').next().unwrap().to_owned()
}
async fn body_text(r:axum::response::Response)->String {
    String::from_utf8(to_bytes(r.into_body(),32768).await.unwrap().to_vec()).unwrap()
}
#[sqlx::test(migrations="./migrations")]
async fn day27_session_admin_can_switch_real_source_counts(
    pool:sqlx::PgPool
) {
    bootstrap::initialize(&pool,"first-password-27".into()).await.unwrap();
    for (source,event_type) in [
        ("web_chat","chat.message.persisted"),
        ("desktop_pet","pet.cleanup.succeeded"),
        ("desktop_pet","pet.sleep.started"),
    ] {
        sqlx::query(
            "INSERT INTO product_events(source_app,event_id,schema_version,event_type)
             VALUES($1,$2,1,$3)"
        )
        .bind(source).bind(uuid::Uuid::new_v4().to_string()).bind(event_type)
        .execute(&pool).await.unwrap();
    }
    let denied=get(app(pool.clone()),"/dashboard?source=desktop_pet",None).await;
    assert_eq!(denied.status(),StatusCode::SEE_OTHER);
    assert_eq!(denied.headers()[header::LOCATION],"/login");

    let initial=post(app(pool.clone()),"/login",
        "username=admin&password=first-password-27".into(),None).await;
    assert_eq!(initial.status(),StatusCode::SEE_OTHER);
    let first_cookie=response_cookie(&initial);
    let restricted=get(app(pool.clone()),
        "/dashboard?source=desktop_pet",Some(&first_cookie)).await;
    assert_eq!(restricted.status(),StatusCode::SEE_OTHER);
    assert_eq!(restricted.headers()[header::LOCATION],"/change-password");

    let change=get(app(pool.clone()),"/change-password",Some(&first_cookie)).await;
    assert_eq!(change.status(),StatusCode::OK);
    let html=body_text(change).await;
    let csrf=html.split("name=\"csrf\" value=\"").nth(1).unwrap()
        .split('"').next().unwrap();
    let changed=post(app(pool.clone()),"/change-password",
        format!("current_password=first-password-27&new_password=next-password-27&confirm_password=next-password-27&email=day27%40example.test&csrf={csrf}"),
        Some(&first_cookie)).await;
    assert_eq!(changed.status(),StatusCode::SEE_OTHER);

    let login=post(app(pool.clone()),"/login",
        "username=admin&password=next-password-27".into(),None).await;
    assert_eq!(login.status(),StatusCode::SEE_OTHER);
    assert_eq!(login.headers()[header::LOCATION],"/dashboard");
    let cookie=response_cookie(&login);

    let chat=get(app(pool.clone()),"/dashboard",Some(&cookie)).await;
    assert_eq!(chat.status(),StatusCode::OK);
    let chat_html=body_text(chat).await;
    assert!(chat_html.contains("Chat 已提交事件總數 1"));

    let pet=get(app(pool.clone()),"/dashboard?source=desktop_pet",
        Some(&cookie)).await;
    assert_eq!(pet.status(),StatusCode::OK);
    let pet_html=body_text(pet).await;
    assert!(pet_html.contains("Pet 已提交事件總數 2"));
    assert!(pet_html.contains("/dashboard?source=web_chat"));

    let forbidden=get(app(pool),"/dashboard?source=unknown",Some(&cookie)).await;
    assert_eq!(forbidden.status(),StatusCode::BAD_REQUEST);
}
```

使用 `#[sqlx::test(migrations="./migrations")]` 自動建立隔離 PG fixture，測未登入 303、首次強制改密 303、後續登入 200、Chat 一筆／Pet 兩筆及未知來源 400。不把 API-only 測試冒充可操作後台。

### Green：沿用既有登入權限，將來源接到管理頁

修改 `apps/stats_server/src/password_login.rs` 原 imports：
```rust
use axum::{
    Form, Json, Router,
    extract::{DefaultBodyLimit, Path, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
```

保留原檔的 `serde::Deserialize`、`session()` 與 `renewed()`。以以下結構與函式**完整替換**既有 `async fn dashboard`：

```rust
// apps/stats_server/src/password_login.rs：替換原 dashboard 函式，
// 並將 extract imports 調整為 {DefaultBodyLimit, Path, Query, State}。
#[derive(Default, Deserialize)]
struct DashboardSourceQuery {
    source: Option<String>,
}

async fn dashboard(
    State(state): State<IngestState>,
    headers: HeaderMap,
    Query(query): Query<DashboardSourceQuery>,
) -> Response {
    let (restricted, raw) = match session(&state, &headers).await {
        Ok(v) => v,
        Err(StatusCode::UNAUTHORIZED) => return Redirect::to("/login").into_response(),
        Err(e) => return e.into_response(),
    };
    if restricted {
        return renewed(Redirect::to("/change-password"), &raw);
    }
    let source = query.source.as_deref().unwrap_or("web_chat");
    let source_label = match source {
        "web_chat" => "Chat",
        "desktop_pet" => "Pet",
        _ => return StatusCode::BAD_REQUEST.into_response(),
    };
    let rows = match pg_store::summary_for_source(&state.pool, source).await {
        Ok(v) => v,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    let Some(total) = crate::summary_total(&rows) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    renewed(
        Html(format!(
            r#"<!doctype html>
<html lang="zh-Hant"><head><meta charset="utf-8">
<title>Analytics 管理後台</title></head>
<body>
<h1>Analytics 管理後台</h1>
<nav aria-label="事件來源">
<a href="/dashboard?source=web_chat">Chat 事件</a>
<a href="/dashboard?source=desktop_pet">Pet 事件</a>
</nav>
<a href="/change-password">修改密碼與Email</a>
<p>已驗證的管理員：{source_label} 已提交事件總數 {total}</p>
<p>事件來源已篩選；詳細報表與角色／來源 ACL 尚未接通。</p>
<form method="post" action="/logout">
<button>登出</button></form>
</body></html>"#
        )),
        &raw,
    )
}
```

`password_login::build` 現有 `route("/",get(dashboard))`、`route("/dashboard",get(dashboard))` 不必改；Query 對沒有 `source` 的請求以 `web_chat` 作為預設。登入檢查順序仍先 `session()`，再限制 `source`，避免未登入透過不同參數探測資料。字串只能在精確白名單 `web_chat | desktop_pet` 內選擇，HTML 僅輸出程式內固定標籤（Chat、Pet），不把任意 query 插入畫面，SQL 繼續使用 `.bind(source)`。

### Refactor 與 Day25／Day26 邊界

來源解析封裝到 `DashboardSourceQuery`，權限與來源分開：第一層 Session/密碼初始化 Gate、第二層來源白名單、第三層資料聚合。後續真正多角色需以 PostgreSQL ACL 決定可選來源，不能將「來源白名單」宣稱角色授權完成。重複的 Cookie/hash/Session 日後經共享 Auth Port 與各系統獨立政策重構，此篇沒有自行複製新的驗證機制。

Analytics 既有 Day24 更正已結案，**本篇不新增或重發舊 Day 補教**；Day25（含）以前封存檔維持不變，Day26 schema 功能不是今天的 Red。本次沒有操作真實 DB，沒有替換歷史 migration，亦沒有執行刪除重建。既有資料保留／checksum 現場狀態未重新驗收。

## ③ Cargo、依賴與執行環境

- 根 `Cargo.toml` workspace edition 2024、version 0.2.25；`axum 0.8`、`sqlx 0.9`（postgres，macros，runtime-tokio）、`tokio 1.54`、`serde 1.0`、`argon2 0.6`、`sha2 0.11`；無新增依賴。
- `apps/stats_server/Cargo.toml` 已存在 `postgres-integration` feature；測試使用既有 `tower 0.5`、`uuid`、`sqlx::test`。精確 lock resolved versions、stable toolchain/MSRV 尚未核驗，不能當作已建置證據。

## ④ 執行命令、預期／實際、CI／限制

| 命令／流程 | 預期 | 本次實際 |
| --- | --- | --- |
| `DATABASE_URL=postgres://... cargo test -p stats-server --features postgres-integration --test day27_dashboard_source` | 真實 Session + PG fixture；來源 Chat=1、Pet=2；未知 400 | 未執行 |
| `DATABASE_URL=postgres://... cargo test -p stats-server --features postgres-integration -- --include-ignored` | **完整 PG target**（包含所有 ignored）均通過 | 未執行 |
| `cargo check -p stats-server` | handler/Query 組合可編譯 | 未執行 |
| 管理員實際瀏覽器：初始化、首次改密、後台切換 Chat/Pet | 對應已提交事件，各來源不混雜 | 未部署／未驗收 |
| 只讀 DB 核對 `SELECT version,checksum,success FROM _sqlx_migrations ORDER BY version;` | 最新 migration 與歷史 checksum 對齊 | 未連線／未驗證 |

唯一 CI 來源：`rust_ai_chat/main@8cf82626c9c82939a3a56eab32e24a8a5b5b00ed` [Rust Dashboard Run 39785099458](https://github.com/williechen/rust_ai_chat/actions/runs/39785099458) completed/success、[Test All 39785099220](https://github.com/williechen/rust_ai_chat/actions/runs/39785099220) completed/success；均是**來源 main** 的既有測試，不包含本篇未套用的新 UI Green。未將 `copilot_use` Actions 當 CI 證據。PostgreSQL 歷史 checksum、現存資料與 31-tests（或更多）的當前完整實跑、角色／來源 ACL、前端登入等待與產品驗收仍待驗證。**文章成功不等於程式已套用**。
