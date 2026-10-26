# Analytics Day38 r4｜機器上報拒絕重複 Authorization Header

> **2026-10-26 r4修訂**。產品唯讀來源 `9b89656f964b99e0cb89ba421855704a35c5ecb9`。功能Green與本案Refactor已在完整隔離候選驗證；產品checkout未套用。Red類型、測試／DB結果、環境失敗與未驗收項目分列於④及證據檔。

本修訂保留[原版](analytics.md)，以當前精確來源提供[功能Green完整patch](analytics-r2.patch)。核心60分鐘只計②主Red／Green；同篇本案Refactor獨立段落與其餘作業另計。這是同Day修訂，next_day維持39。

從未改過的來源使用完整patch；若先手動加入Red測試，依Green正文補程式，不重複套入已含測試的patch。來源有本地變更先審查差異，不覆寫。完整patch的SHA256：`89e2bd9fe90c59c6949ce952aa39787eea51093d759f01ee725e45e7be71969f`。

規則精確來源：copilot_use `1e6b8dcfa49d08b4a0af13ee4b602ac7664d2a64`，共用規則blob `34a2847c8628388f5bdb47c64a5f613b80ed3606`；本案Refactor與②Red／Green並列，功能程式與patch沿用r2精確來源。

## ① 功能說明與上一 Day 缺口

[Day37 r3](../day-37/analytics-r3.md) 已將**人員 Session Cookie** 的歧義拒絕接到兩種 summary Router；最新來源仍保留此修補，不重教、不重新宣稱已驗收。另一個**獨立的機器入口** `analytics_apps/analytics_server/src/lib.rs::ingest` 目前用 `headers.get(AUTHORIZATION)` 取第一個值，`IngestTokens::authorized` 僅比對那個字串。若同一請求帶兩條 `Authorization`，第一條合法就可能被當成合法事件；代理、下游與稽核系統對多值解讀可能不同。本日收斂 HTTP 邊界：**必須恰有一條可解析 Authorization，且其 Bearer token 與事件 source_app 相符**。未授權請求不得寫入 `product_events`；合法單一 Chat／Pet 來源仍可各自上報。

正式鏈：Chat／Pet 機器客戶端 → `POST /api/v1/events` → `analytics_server::ingest`（Axum HTTP／header）→ `IngestTokens::authorized` → `pg_store::persist_event`（既有 SQLx PostgreSQL transaction／冪等）→ 201/200/409 ACK。此路由是機器上報，**不能要求人員 Cookie**；`/api/v1/events/{source}/summary` 則繼續使用 Day37 的人員 Session。Leptos Dashboard、Application／Domain 空範本不因本篇變成已接通；未實作的短效服務憑證／配對／輪替、管理員與檢視者角色／來源 ACL 仍待完成。

## Refactor

### Analytics：三組PG整合測試共用準備程式

本段只處理Analytics自身，與②Red／Green並列，不以當日功能切片或Green完成作為重構邊界。新增tests/support的Router建構與HTTP傳輸工具，由首次登入、Dashboard來源及query Cookie三組真PG測試使用。bootstrap、場景SQL與全部狀態碼／Cookie／DB／頁面斷言留在原測試，各sqlx::test仍使用原隔離DB。Day37的多Cookie header request helper仍留在該測試，不硬套單Cookie工具。這是Analytics自己的測試去重，沒有跨案fixture或全庫workflow。

### 完整變更

精確產品來源：`9b89656f964b99e0cb89ba421855704a35c5ecb9`。下列patch包含必要的新模組、完整程式／測試與正式caller接線，不是只有目錄或介面骨架。沒有新增Cargo依賴、features、lock、migration或全庫workflow。

[下載原始patch](analytics-r4-refactor.patch)（SHA256：`04dbd2a9cf90a5b0f4d5c875af91a1bd43dc1fd2e400dbcd249487b41b58ee1f`）。請使用原始檔以保留既有CRLF；先檢查再套用，不覆寫本地變更。

```diff
--- a/analytics_apps/analytics_server/tests/day23_password_login.rs
+++ b/analytics_apps/analytics_server/tests/day23_password_login.rs
@@ -1,52 +1,8 @@
 #![cfg(feature = "postgres-integration")]
-use analytics_server::{IngestState, IngestTokens, password_login};
-use axum::{
-    body::Body,
-    http::{Request, StatusCode, header},
-};
-use std::sync::Arc;
-use tower::ServiceExt;
-fn app(pool: sqlx::PgPool) -> axum::Router {
-    password_login::build(IngestState {
-        pool,
-        tokens: Arc::new(IngestTokens {
-            chat: "fixture-chat".into(),
-            pet: "fixture-pet".into(),
-        }),
-    })
-}
-async fn send(
-    app: axum::Router,
-    path: &str,
-    body: Option<&str>,
-    cookie: Option<&str>,
-) -> axum::response::Response {
-    let mut r = if body.is_some() {
-        Request::post(path)
-            .header("origin", "http://127.0.0.1:3100")
-            .header("content-type", "application/x-www-form-urlencoded")
-    } else {
-        Request::get(path)
-    };
-    if let Some(c) = cookie {
-        r = r.header(header::COOKIE, c);
-    }
-    app.oneshot(
-        r.body(body.map_or_else(Body::empty, |s| Body::from(s.to_owned())))
-            .unwrap(),
-    )
-    .await
-    .unwrap()
-}
-fn cookie(r: &axum::response::Response) -> String {
-    r.headers()[header::SET_COOKIE]
-        .to_str()
-        .unwrap()
-        .split(';')
-        .next()
-        .unwrap()
-        .to_owned()
-}
+mod support;
+use support::http::{send, response_cookie as cookie};
+use axum::http::{StatusCode, header};
+fn app(pool: sqlx::PgPool) -> axum::Router { support::app(pool, "fixture-chat", "fixture-pet") }
 #[sqlx::test(migrations = "../migrations")]
 async fn day23_bootstrapped_password_is_restricted_before_first_change(pool: sqlx::PgPool) {
     analytics_server::bootstrap::initialize(&pool, "lesson-only-password".into())
--- a/analytics_apps/analytics_server/tests/day27_dashboard_source.rs
+++ b/analytics_apps/analytics_server/tests/day27_dashboard_source.rs
@@ -1,59 +1,9 @@
-// apps/analytics_server/tests/day27_dashboard_source.rs
 #![cfg(feature = "postgres-integration")]
-use analytics_server::{IngestState, IngestTokens, bootstrap, password_login};
-use axum::{
-    body::{Body, to_bytes},
-    http::{Request, StatusCode, header},
-};
-use std::sync::Arc;
-use tower::ServiceExt;
-
-fn app(pool: sqlx::PgPool) -> axum::Router {
-    password_login::build(IngestState {
-        pool,
-        tokens: Arc::new(IngestTokens {
-            chat: "test-chat".into(),
-            pet: "test-pet".into(),
-        }),
-    })
-}
-async fn get(app: axum::Router, path: &str, cookie: Option<&str>) -> axum::response::Response {
-    let mut request = Request::get(path);
-    if let Some(cookie) = cookie {
-        request = request.header(header::COOKIE, cookie);
-    }
-    app.oneshot(request.body(Body::empty()).unwrap())
-        .await
-        .unwrap()
-}
-async fn post(
-    app: axum::Router,
-    path: &str,
-    form: String,
-    cookie: Option<&str>,
-) -> axum::response::Response {
-    let mut req = Request::post(path)
-        .header(header::ORIGIN, "http://127.0.0.1:3100")
-        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
-    if let Some(cookie) = cookie {
-        req = req.header(header::COOKIE, cookie);
-    }
-    app.oneshot(req.body(Body::from(form)).unwrap())
-        .await
-        .unwrap()
-}
-fn response_cookie(r: &axum::response::Response) -> String {
-    r.headers()[header::SET_COOKIE]
-        .to_str()
-        .unwrap()
-        .split(';')
-        .next()
-        .unwrap()
-        .to_owned()
-}
-async fn body_text(r: axum::response::Response) -> String {
-    String::from_utf8(to_bytes(r.into_body(), 32768).await.unwrap().to_vec()).unwrap()
-}
+mod support;
+use support::http::{get, post_form as post, response_cookie, body_text};
+use analytics_server::bootstrap;
+use axum::http::{StatusCode, header};
+fn app(pool: sqlx::PgPool) -> axum::Router { support::app(pool, "test-chat", "test-pet") }
 #[sqlx::test(migrations = "../migrations")]
 async fn day27_session_admin_can_switch_real_source_counts(pool: sqlx::PgPool) {
     bootstrap::initialize(&pool, "first-password-27".into())
--- a/analytics_apps/analytics_server/tests/day37_query_cookie.rs
+++ b/analytics_apps/analytics_server/tests/day37_query_cookie.rs
@@ -1,11 +1,11 @@
 #![cfg(feature = "postgres-integration")]
-use analytics_server::{IngestState, IngestTokens, bootstrap, password_login};
+mod support;
+use analytics_server::bootstrap;
 use axum::{
     Router,
     body::Body,
     http::{Request, StatusCode, header},
 };
-use std::sync::Arc;
 use tower::ServiceExt;
 
 async fn query(
@@ -47,13 +47,7 @@
     .execute(&pool)
     .await
     .unwrap();
-    let app = password_login::build(IngestState {
-        pool: pool.clone(),
-        tokens: Arc::new(IngestTokens {
-            chat: "fixture-chat".into(),
-            pet: "fixture-pet".into(),
-        }),
-    });
+    let app = support::app(pool.clone(), "fixture-chat", "fixture-pet");
     let login = app
         .clone()
         .oneshot(
@@ -67,13 +61,7 @@
         .unwrap();
     assert_eq!(login.status(), StatusCode::SEE_OTHER);
     assert_eq!(login.headers()[header::LOCATION], "/dashboard");
-    let cookie = login.headers()[header::SET_COOKIE]
-        .to_str()
-        .unwrap()
-        .split(';')
-        .next()
-        .unwrap()
-        .to_owned();
+    let cookie = support::http::response_cookie(&login);
     let invalid = format!("analytics_session={}", "0".repeat(64));
     let paths = [
         "/api/v1/events/web_chat/summary",
--- /dev/null
+++ b/analytics_apps/analytics_server/tests/support/http.rs
@@ -0,0 +1,53 @@
+// Each integration binary intentionally uses a different subset of shared transport helpers.
+#![allow(dead_code)]
+use axum::{
+    Router,
+    body::{Body, to_bytes},
+    http::{Request, header},
+    response::Response,
+};
+use tower::ServiceExt;
+
+pub async fn send(app: Router, path: &str, form: Option<&str>, cookie: Option<&str>) -> Response {
+    let mut request = if form.is_some() {
+        Request::post(path)
+            .header(header::ORIGIN, "http://127.0.0.1:3100")
+            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
+    } else {
+        Request::get(path)
+    };
+    if let Some(cookie) = cookie {
+        request = request.header(header::COOKIE, cookie);
+    }
+    app.oneshot(
+        request
+            .body(form.map_or_else(Body::empty, |s| Body::from(s.to_owned())))
+            .unwrap(),
+    )
+    .await
+    .unwrap()
+}
+pub async fn get(app: Router, path: &str, cookie: Option<&str>) -> Response {
+    send(app, path, None, cookie).await
+}
+pub async fn post_form(app: Router, path: &str, form: String, cookie: Option<&str>) -> Response {
+    send(app, path, Some(&form), cookie).await
+}
+pub fn response_cookie(response: &Response) -> String {
+    response.headers()[header::SET_COOKIE]
+        .to_str()
+        .unwrap()
+        .split(';')
+        .next()
+        .unwrap()
+        .to_owned()
+}
+pub async fn body_text(response: Response) -> String {
+    String::from_utf8(
+        to_bytes(response.into_body(), 32768)
+            .await
+            .unwrap()
+            .to_vec(),
+    )
+    .unwrap()
+}
--- /dev/null
+++ b/analytics_apps/analytics_server/tests/support/mod.rs
@@ -0,0 +1,12 @@
+pub mod http;
+use analytics_server::{IngestState, IngestTokens, password_login};
+use std::sync::Arc;
+pub fn app(pool: sqlx::PgPool, chat: &str, pet: &str) -> axum::Router {
+    password_login::build(IngestState {
+        pool,
+        tokens: Arc::new(IngestTokens {
+            chat: chat.into(),
+            pet: pet.into(),
+        }),
+    })
+}
--- /dev/null
+++ b/analytics_apps/analytics_server/tests/support_http_contract.rs
@@ -0,0 +1,80 @@
+#[path = "support/http.rs"]
+mod http;
+use axum::{
+    Router,
+    body::Body,
+    http::{Request, StatusCode, header},
+    routing::any,
+};
+
+async fn echo(request: Request<Body>) -> String {
+    let method = request.method().to_string();
+    let path = request.uri().to_string();
+    let origin = request
+        .headers()
+        .get(header::ORIGIN)
+        .and_then(|v| v.to_str().ok())
+        .unwrap_or("")
+        .to_owned();
+    let content_type = request
+        .headers()
+        .get(header::CONTENT_TYPE)
+        .and_then(|v| v.to_str().ok())
+        .unwrap_or("")
+        .to_owned();
+    let cookie = request
+        .headers()
+        .get(header::COOKIE)
+        .and_then(|v| v.to_str().ok())
+        .unwrap_or("")
+        .to_owned();
+    let body = axum::body::to_bytes(request.into_body(), 4096)
+        .await
+        .unwrap();
+    format!(
+        "{method}|{path}|{origin}|{content_type}|{cookie}|{}",
+        String::from_utf8(body.to_vec()).unwrap()
+    )
+}
+#[tokio::test]
+async fn request_setup_preserves_get_and_form_wire_contracts() {
+    let app = Router::new().route("/echo", any(echo));
+    let get = http::get(app.clone(), "/echo?source=web_chat", None).await;
+    assert_eq!(get.status(), StatusCode::OK);
+    assert_eq!(http::body_text(get).await, "GET|/echo?source=web_chat||||");
+    let form = "username=ADMIN&password=a%26b";
+    let post = http::post_form(
+        app.clone(),
+        "/echo",
+        form.into(),
+        Some("analytics_session=fixture"),
+    )
+    .await;
+    assert_eq!(
+        http::body_text(post).await,
+        "POST|/echo|http://127.0.0.1:3100|application/x-www-form-urlencoded|analytics_session=fixture|username=ADMIN&password=a%26b"
+    );
+    let with_cookie = http::send(
+        app,
+        "/echo",
+        None,
+        Some("theme=dark; analytics_session=fixture"),
+    )
+    .await;
+    assert_eq!(
+        http::body_text(with_cookie).await,
+        "GET|/echo|||theme=dark; analytics_session=fixture|"
+    );
+}
+#[tokio::test]
+async fn response_helpers_keep_cookie_and_body_contracts() {
+    let response = axum::response::Response::builder()
+        .header(
+            header::SET_COOKIE,
+            "analytics_session=abc; HttpOnly; SameSite=Strict",
+        )
+        .body(Body::from("登入完成"))
+        .unwrap();
+    assert_eq!(http::response_cookie(&response), "analytics_session=abc");
+    assert_eq!(http::body_text(response).await, "登入完成");
+}
```

### 舊斷言如何承接

| 原測試／責任 | 新位置與保留內容 |
| --- | --- |
| day23 bootstrap/session SQL/status/header assertions | all assertion text retained in original test bodies |
| day27 Dashboard source counts/first-change/CSRF assertions | all retained; helper owns no scenario assertions |
| day37 two real query routes/cookie ambiguity/restricted/logout | all retained; multi-header query helper stays local to protect its distinct contract |

不同故障模式與跨層接線保留，不以測試數減少當覆蓋完成，也不把既有行為重構假稱新功能Red。

### 實際驗證與使用者回歸命令

完整隔離server／persistence與PostgreSQL18.6：48項通過、1 ignored另行執行通過；首次登入、Dashboard來源及query Cookie三組真PG測試均通過。另在原始來源加新Red測試，重複Authorization實際得到201而預期401；合併候選Green後該正式Router測試通過。真瀏覽器與角色／來源ACL完整功能未驗收。

本案patch已在精確來源通過git apply --check及完整修改檔的Rust語法解析。也已在隔離快照實際確認本案Refactor與②功能Green patch可按兩種順序套用；不必假設Green先完成。差異相容性檢查本身不代表編譯；本次另在完整隔離合併候選執行下述Cargo與DB驗證。

```bash
# 在rust_ai_ring儲存庫根目錄檢查乾淨基準；套用由使用者執行。
git apply --check /path/to/analytics-r4-refactor.patch
# 套用後在使用者環境重跑；本次相應命令已在隔離候選執行，非原產品checkout。
cd analytics_apps
cargo test --locked -p analytics-server --test support_http_contract
cargo test --locked -p analytics-server --features postgres-integration --test day23_password_login --test day27_dashboard_source --test day37_query_cookie
```

[逐檔來源blob、斷言對照及patch檢查](analytics-r4-refactor-evidence.json) · [候選測試結果與範圍](r4-refactor-tests.json)。[完整本地Red／Green與workspace結果](r4-local-validation.json)。所有結果只屬隔離合併候選內容；rust_ai_ring保持唯讀，產品未套用，瀏覽器／GUI與正式CI未驗收。若失敗或patch來源漂移，保留既有程式，重核差異再套用，不把局部通過追認完整產品。

### 下一步

後續依Analytics自己的Session／查詢授權、PG Adapter、UI與測試fixture邊界逐步調整；跨案憑證契約與全庫CI另列全專案項目。 本次有實際程式／測試變更，仍不是本案全部重構完成；依本案累積進度接續，不新增Day、不結案FIFO、不更新last_full_comparison。

## ② 功能實作：Red → 完整 Green

### Red：新增真正 Router + 真 PostgreSQL 的 Header 歧義測試

新增 `analytics_apps/analytics_server/tests/day38_ingest_authorization.rs` **完整檔案**。來源已存在的正常攝取／去重測試是回歸基準，不當作新 Red。這個新測試在未改生產碼時，`[合法,非法]` 與 `[合法,合法]` 多條 Header 預期 401，但原 `headers.get` 只看第一個，預期實際 201；**本r4已實際驗證這個行為Red，見④**。測試使用 `#[sqlx::test]` 的隔離 PG fixture，不複製事件 SQL 演算法。

```rust
#![cfg(feature = "postgres-integration")]

use analytics_server::{IngestState, IngestTokens, build_app};
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode, header},
};
use std::sync::Arc;
use tower::ServiceExt;

async fn post_event(app: &Router, source: &str, auth: &[&str]) -> StatusCode {
    let body = serde_json::json!({
        "schema_version": 1,
        "event_id": uuid::Uuid::new_v4(),
        "source_app": source,
        "event_type": "test.ingest.authorization"
    });
    let mut request = Request::post("/api/v1/events")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    for value in auth {
        request.headers_mut().append(
            header::AUTHORIZATION,
            value.parse().unwrap(),
        );
    }
    app.clone().oneshot(request).await.unwrap().status()
}

#[sqlx::test(migrations = "../migrations")]
async fn ambiguous_machine_authorization_is_rejected_before_db_write(
    pool: sqlx::PgPool,
) {
    let app = build_app(IngestState {
        pool: pool.clone(),
        tokens: Arc::new(IngestTokens {
            chat: "fixture-chat".into(),
            pet: "fixture-pet".into(),
        }),
    });

    assert_eq!(
        post_event(&app, "web_chat", &["Bearer fixture-chat"]).await,
        StatusCode::CREATED
    );
    for auth in [
        vec!["Bearer fixture-chat", "Bearer fixture-pet"],
        vec!["Bearer fixture-chat", "Bearer fixture-chat"],
        vec!["Bearer fixture-pet", "Bearer fixture-chat"],
        vec!["Bearer fixture-pet"],
        vec![],
    ] {
        assert_eq!(
            post_event(&app, "web_chat", &auth).await,
            StatusCode::UNAUTHORIZED,
            "authorization headers: {auth:?}"
        );
    }
    assert_eq!(
        post_event(&app, "desktop_pet", &["Bearer fixture-pet"]).await,
        StatusCode::CREATED
    );
    assert_eq!(
        post_event(&app, "desktop_pet", &["Bearer fixture-chat"]).await,
        StatusCode::UNAUTHORIZED
    );
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM product_events")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 2, "only the two valid single-header events persist");
}
```

### Green：集中唯一 Header 解析，正式 ingest 和測試 helper 共用

在 `analytics_apps/analytics_server/src/lib.rs` 的 `impl IngestTokens` **之前**新增完整函式；沿用檔首已有 `use axum::http::{HeaderMap, header::AUTHORIZATION};`，不新增 import 或套件：

```rust
fn single_authorization(headers: &HeaderMap) -> Option<&str> {
    let mut values = headers.get_all(AUTHORIZATION).iter();
    let first = values.next()?.to_str().ok()?;
    if values.next().is_some() {
        return None;
    }
    Some(first)
}
```

在**正式** `pub async fn ingest` 內，把原本這段：

```rust
    let authorization = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
```

替換成：

```rust
    let authorization = single_authorization(&headers).unwrap_or("");
```

保留後續原始碼**完整不變**，它仍是正式路由與真正 PG Adapter：

```rust
    if !state.tokens.authorized(&event.source_app, authorization) {
        return StatusCode::UNAUTHORIZED;
    }
    match pg_store::persist_event(&state.pool, &event).await {
        Ok(outcome) => outcome_status(outcome),
        Err(PersistError::Invalid) => StatusCode::BAD_REQUEST,
        Err(PersistError::Database(_)) => StatusCode::SERVICE_UNAVAILABLE,
    }
```

同檔 `#[cfg(test)] async fn source_summary` 的原 `headers.get(AUTHORIZATION)` 解析段也換成同一行 `let authorization = single_authorization(&headers).unwrap_or("");`，避免測試 helper 持續維護較寬鬆的 Header 契約；該 helper **不是正式 query 路由**，仍僅 `cfg(test)`，不能藉它宣稱業務查詢可用機器 token。這是完整Green所需的共用解析，無新增對外API／第二套認證。

**驗證順序**：先判斷事件 schema／source/type（原 `valid_event`），再拒絕 0 或 >1 個 Authorization，才允許 SQLx 寫入。單值 token 的固定長度比較原樣保留；拒絕多值不是短效憑證／防重播的完整方案。

### Cargo、模組、真 caller 與測試保留

四個 workspace `chat_apps`、`pet_apps`、`analytics_apps`、`crates` 各有獨立 Cargo.toml/Cargo.lock。此切片改 `analytics_apps/analytics_server/src/lib.rs`，新增 `analytics_apps/analytics_server/tests/day38_ingest_authorization.rs`；package `analytics-server`、`analytics-persistence`，Rust crates `analytics_server`／`analytics_persistence`。現有 `axum`、`sqlx`（postgres/macros/runtime-tokio）、`uuid`、`serde_json`、`tower`（dev）已足夠；`postgres-integration` feature 已連到 persistence。**不新增 Cargo dependencies、features、imports、migrations、Leptos metadata 或 UI**。正式 `build_app → password_login::build → /api/v1/events` 不改路由，只有 ingest 使用新解析器；機器 token 不授權 Dashboard。

| 既有測試／入口 | 處置 | 保留契約 |
| --- | --- | --- |
| `postgres_event_store`／`idempotency_contract` | 保留回歸 | 真 PG 寫入／衝突／冪等與錯誤 |
| `day37_query_cookie` | 保留回歸 | 人員 Cookie 唯一性、summary 401／403／200 |
| `source_summary`（test-only） | 共用解析器，不提升成產品路由 | 測試專用來源 token 邊界 |
| 新 `day38_ingest_authorization` | 新增 PG Router 測試 | 重複 Header 拒絕、單值兩來源成功、DB 不受污染 |

## ③ 當日官方穩定版本與採用判斷

2026-10-26 台北日期，crates/Rust於 `2026-10-25T23:01:56.157211+00:00` 重查；排除預發行與 yanked。[完整宣告／lock／官方網址／原始回應SHA](r2-versions.json)。來源9b與648外部宣告及resolved一致；Chat／Pet／Analytics三workspace為0.2.37，共用crates維持0.2.35。

| 套件／工具 | manifest 宣告 | lock resolved／已安裝 | 官方穩定版／MSRV |
| --- | --- | --- | --- |
| axum | `0.8` | 0.8.9 | [0.8.9](https://crates.io/crates/axum/0.8.9)／1.80 |
| sqlx | `0.9` | 0.9.0 | [0.9.0](https://crates.io/crates/sqlx/0.9.0)／1.94.0 |
| tokio | `1.54` | 1.54.1 | [1.54.1](https://crates.io/crates/tokio/1.54.1)／1.85 |
| uuid | `1.29` | 1.29.0 | [1.29.0](https://crates.io/crates/uuid/1.29.0)／1.89.0 |
| tower | `0.5` | 0.5.3 | [0.5.3](https://crates.io/crates/tower/0.5.3)／1.64.0 |
| serde | `1.0` | 1.0.229 | [1.0.229](https://crates.io/crates/serde/1.0.229)／1.56 |
| serde_json | `1.0` | 1.0.154 | [1.0.154](https://crates.io/crates/serde_json/1.0.154)／1.71 |
| argon2 | `0.6` | 0.6.0 | [0.6.0](https://crates.io/crates/argon2/0.6.0)／1.85 |
| sha2 | `0.11` | 0.10.9,0.11.1 | [0.11.1](https://crates.io/crates/sha2/0.11.1)／1.88 |

Rust官方stable 1.99.1，[官方channel](https://static.rust-lang.org/dist/channel-rust-stable.toml)，本機rustc/cargo同版。最高所列MSRV不高於本機Rust，但不等於SSR／WASM／native／DB建置通過。相符resolved維持；若有差異優先以最新穩定版驗證遷移，本包不改lock、安裝工具或升級。

PostgreSQL官方穩定18.6（排除19RC）；已安裝psql18.6是客戶端，本次專用測試DB已核驗伺服器18.6並執行真PG測試。來源workflow使用postgres:latest，並未因工程proposal改成18；本篇用隔離PG18測試方案。SQLx CLI未安裝，官方0.9.0，須與庫同版遷移驗證，不自動安裝或跑migration。
[PostgreSQL官方支援版本](https://www.postgresql.org/support/versioning/)。

資料庫／native當日官方來源、已安裝探測與採用理由見[版本詳細核對](../../../source-review/2026-10-26-versions-and-summary-refresh.md)，不能以metadata宣稱功能已驗收。
## ④ 驗證命令、狀態與舊 Day 更正

```bash
cd rust_ai_ring/analytics_apps
# DATABASE_URL 指向可建立 SQLx 測試 DB 的隔離 PostgreSQL 18 測試角色。
export DATABASE_URL='postgres://postgres:postgres@localhost:5432/analytics_ci'
cargo test --locked -p analytics-server --features postgres-integration --test day38_ingest_authorization
cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration
# 原略過項依實際 fixture 環境另執行；不把 ignored 計為通過。
# CI沿產品現行workflow追蹤；全專案CI另見全專案獨立項目。
```

| Gate | 預期 | 本次實際 |
| --- | --- | --- |
| Red | 多條 Header 以合法 token 開頭會被舊正式 ingest 接受，測試失敗 | 實際觀察201／401行為Red，測試如預期失敗 |
| Green PG／Router | 單值Chat/Pet 201、重複／跨來源401、DB僅兩筆 | 隔離候選正式Router＋PG測試通過 |
| 原有PG／Cookie／冪等 | 保留既有正式路徑與斷言 | 完整48項通過；ignored1項另跑通過 |
| 人員權限／GUI | Query 仍用 Session，Leptos／角色／來源 ACL 另驗 | 未驗收 |

**Day37 更正補充**：人員 Cookie 歧義已修正，不代表機器 Authorization 歧義已修正；本篇是新的獨立入口切片。Day35／36 的精確類型查詢、Dashboard Session 仍保留。固定來源 token 是過渡，不能取代服務配對、短效憑證、輪替與撤銷。本次不更動既有 Day 正文、來源、原 commit/blob/SHA256、`last_full_comparison`；修訂Day38文件與可套用內容，不新增Day；全專案CI另列獨立項目。

本r4已在完整精確來源的隔離副本套用本案Refactor與功能Green，實際完成④所列驗證。原產品checkout與遠端main未套用；未把候選測試追認為瀏覽器／GUI或GitHub Actions驗收。[完整本地證據](r4-local-validation.json)保留原始錯誤與恢復，未更新last_full_comparison。
