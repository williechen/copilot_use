## 1. 學習大綱：註冊成功後停在完成頁

本日最小切片是「註冊提交→SQLite 提交→獨立完成頁→使用者按回首頁」，成功保持登出，不自動進個人頁或聊天室。來源為 rust_ai_chat/main `c965681eeb2b4791dba8d157624424b7f274b2fd`，workspace 0.2.23；教學分支基準 `f3b32b6a27d06db72e40f7d92a1530f52d549342`。目前原始碼註冊成功仍導向 `/signin`，與已確認的完成頁需求不一致。本篇先補這個註冊流程缺口；登入本身已在 main，房間 ACL 與登入後返回目標仍未接通，不能宣稱 Chat 基本門檻全部完成。

真實呼叫鏈為 `apps/server/src/signup.rs::routes` 的 POST `/signup` → Argon2 加鹽雜湊 → 同一 SQLite 交易新增會員並撤銷目前 cookie 的 Session → 303 `/signup/complete` → GET 完成頁 → 明確的首頁連結。沒有另建資料庫或偽造前端成功。完成頁不帶帳號、Email、房間目標及密碼，不含自動導頁腳本。直接開完成頁只是通用訊息，不證明帳號已建立。

規則版本 daily-tdd-minimum-v1；Notion 共同規則更新時間 `2026-10-20T16:32:31.684Z`，GitHub 規則 blob `ebebad6b3d671b7cdfa48a5f121a9958010b15b4`。Day23（含）以前封存，全文不回寫；產品來源只讀，完整 patch 由使用者審閱後套用。copilot_use 自身產品具有不同 Application／Adapter 契約，本篇不是產品整合。主體 Red／Green 估 10 分鐘，Refactor 5 分鐘，版本與驗證閱讀另 8 分鐘。

前置與阻擋：最新 main 的 `migrations/20261018094242_create_ai_person.sql` 在空白 SQLite 得到 `near "CREATE": syntax error`，因此全新資料庫啟動失敗。本篇不修改歷史 migration、不刪舊資料或調 checksum；實測切片使用本日真正需要的三份 SQL（auth、member identity 唯一索引、session），不能當成整套 fresh migration 通過。已具有正確版本 schema 的環境才能驗此流程；其餘環境先人工處理 migration 問題。

## 2. 實作內容：Red → 完整最小 Green → Refactor

### Red：確認真正的使用者導向失敗

新增下面測試檔後，先在未改 handler 的隔離副本執行 `cargo test --locked -p server --test day24_signup_complete`。實際行為 Red：預期 `/signup/complete`，實際 `/signin`；不是匯入或編譯失敗。第二項測試隨 Green 補入，驗證既有 Session 撤銷與會員新增同交易：撤銷被資料庫拒絕時，不留下新會員。

測試檔 `apps/server/tests/day24_signup_complete.rs` 完整內容如下。先手動加入此檔時，只套 Green 的 handler 部分；或在乾淨副本一次套下方完整 patch，不能重複新增測試。

```rs
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use tower::ServiceExt;
#[tokio::test]
async fn day24_signup_completes_without_automatic_login_or_navigation() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    for sql in [include_str!("../../../migrations/20261018073931_create_auth_user.sql"),
                include_str!("../../../migrations/20261019221857_member_identity.sql"),
                include_str!("../../../migrations/20261020152737_mbr_sessions.sql")] {
        sqlx::raw_sql(sql).execute(&pool).await.unwrap();
    }
    let app = server::signup::routes(pool.clone());
    let response=app.clone().oneshot(Request::post("/signup").header("origin","http://127.0.0.1:3000").header("content-type","application/x-www-form-urlencoded").body(Body::from("username=day24&email=day24%40example.test&display_name=Learner&password=lesson-only-password")).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(response.headers()["location"], "/signup/complete");
    assert!(
        response.headers()["set-cookie"]
            .to_str()
            .unwrap()
            .contains("Max-Age=0")
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM mbr_session")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    let page = app
        .clone()
        .oneshot(
            Request::get("/signup/complete")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(page.status(), StatusCode::OK);
    assert!(!page.headers().contains_key("location"));
    let html =
        String::from_utf8(to_bytes(page.into_body(), 10000).await.unwrap().to_vec()).unwrap();
    assert!(html.contains("註冊完成"));
    assert!(html.contains("目前保持登出"));
    assert!(html.contains(r#"href="/""#));
    assert!(!html.to_lowercase().contains(r#"http-equiv="refresh""#));
    assert!(!html.contains("<script"));
    let duplicate=app.oneshot(Request::post("/signup").header("origin","http://127.0.0.1:3000").header("content-type","application/x-www-form-urlencoded").body(Body::from("username=DAY24&email=other%40example.test&display_name=Learner&password=lesson-only-password")).unwrap()).await.unwrap();
    assert_eq!(duplicate.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn day24_existing_browser_session_revocation_is_atomic() {
    use sha2::{Digest, Sha256};
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    for sql in [include_str!("../../../migrations/20261018073931_create_auth_user.sql"),
                include_str!("../../../migrations/20261019221857_member_identity.sql"),
                include_str!("../../../migrations/20261020152737_mbr_sessions.sql")] {
        sqlx::raw_sql(sql).execute(&pool).await.unwrap();
    }
    let app = server::signup::routes(pool.clone());
    let make = |name: &str, cookie: Option<String>| {
        let mut req = Request::post("/signup")
            .header("origin", "http://127.0.0.1:3000")
            .header("content-type", "application/x-www-form-urlencoded");
        if let Some(c) = cookie {
            req = req.header("cookie", c);
        }
        req.body(Body::from(format!("username={name}&email={name}%40example.test&display_name=Learner&password=lesson-only-password"))).unwrap()
    };
    assert_eq!(
        app.clone()
            .oneshot(make("older", None))
            .await
            .unwrap()
            .status(),
        StatusCode::SEE_OTHER
    );
    let raw = "a".repeat(64);
    let hash: String = Sha256::digest(raw.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    sqlx::query("INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at) SELECT 'old-session',id,?1,unixepoch()+1000 FROM mbr_user WHERE user_acnt='older'").bind(hash).execute(&pool).await.unwrap();
    sqlx::raw_sql("CREATE TRIGGER test_revoke_failure BEFORE DELETE ON mbr_session BEGIN SELECT RAISE(ABORT,'fixture'); END;").execute(&pool).await.unwrap();
    assert_eq!(
        app.clone()
            .oneshot(make("newer", Some(format!("chat_session={raw}"))))
            .await
            .unwrap()
            .status(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM mbr_user WHERE user_acnt='newer'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    sqlx::query("DROP TRIGGER test_revoke_failure")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        app.oneshot(make("newer", Some(format!("chat_session={raw}"))))
            .await
            .unwrap()
            .status(),
        StatusCode::SEE_OTHER
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM mbr_session")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}
```

### Green：完整修改

保留原始碼 Argon2 0.6 API、帳號／Email 驗證與不分大小寫唯一索引。只有會員寫入與目前瀏覽器 Session 撤銷都成功才提交並轉完成頁；錯誤仍沿用原本狀態碼。成功回應清除 cookie；失敗不把使用者誤登出。完成頁使用固定 HTML、no-store，唯一下一步是回首頁。清除當前 cookie 不等於撤銷此帳號的所有裝置；本篇只撤銷本次瀏覽器原 Session。

以下是可套用的完整 patch，含必要 handler、既有導向斷言調整及新增測試。沒有修改依賴、lock 或 migration。原始 patch 已針對精確基準的 Git index 執行 apply check。

```diff
diff --git a/apps/server/src/signup.rs b/apps/server/src/signup.rs
index 9b6cbd4..76a0b38 100644
--- a/apps/server/src/signup.rs
+++ b/apps/server/src/signup.rs
@@ -3,7 +3,7 @@ use axum::{
     Form, Router,
     extract::{DefaultBodyLimit, State},
     http::{HeaderMap, StatusCode},
-    response::{Html, Redirect},
+    response::{Html, IntoResponse, Redirect, Response},
     routing::get,
 };
 use sqlx::SqlitePool;
@@ -19,10 +19,20 @@ struct Signup {
 pub fn routes(pool: SqlitePool) -> Router {
     Router::new()
         .route("/signup", get(page).post(signup))
+        .route("/signup/complete", get(complete))
         .layer(DefaultBodyLimit::max(4096))
         .with_state(pool)
 }
 
+async fn complete() -> impl IntoResponse {
+    (
+        [("cache-control", "no-store")],
+        Html(
+            r#"<!doctype html><html lang="zh-Hant"><head><meta charset="utf-8"><title>註冊完成</title></head><body><h1>註冊完成</h1><p>目前保持登出，請回首頁重新選擇聊天室，再登入。</p><a href="/">回首頁</a></body></html>"#,
+        ),
+    )
+}
+
 async fn page() -> Html<&'static str> {
     Html(
         r#"<!doctype html>
@@ -62,7 +72,7 @@ async fn signup(
     State(pool): State<SqlitePool>,
     headers: HeaderMap,
     Form(form): Form<Signup>,
-) -> Result<Redirect, StatusCode> {
+) -> Result<Response, StatusCode> {
     // 檢查請求來源，僅允許來自 http://127.0.0.1:3000 的請求
     if headers.get("origin").and_then(|v| v.to_str().ok()) != Some("http://127.0.0.1:3000") {
         return Err(StatusCode::FORBIDDEN);
@@ -104,6 +114,10 @@ async fn signup(
     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
 
+    let mut tx = pool
+        .begin()
+        .await
+        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
     sqlx::query(
         r#"INSERT INTO mbr_user (id
             ,user_acnt
@@ -121,14 +135,50 @@ async fn signup(
     .bind(password_hash)
     .bind(email)
     .bind(display)
-    .execute(&pool)
+    .execute(&mut *tx)
     .await
     .map_err(|e| match e {
         sqlx::Error::Database(ref db) if db.is_unique_violation() => StatusCode::CONFLICT,
         _ => StatusCode::INTERNAL_SERVER_ERROR,
     })?;
 
-    Ok(Redirect::to("/signin"))
+    // 成功註冊不沿用目前瀏覽器的帳號或選房；資料與撤銷同交易提交。
+    if let Some(token) = headers
+        .get("cookie")
+        .and_then(|h| h.to_str().ok())
+        .and_then(|h| {
+            h.split(';')
+                .filter_map(|v| v.trim().split_once('='))
+                .find(|(k, _)| *k == "chat_session")
+                .map(|(_, v)| v)
+        })
+        .filter(|v| v.len() == 64 && v.bytes().all(|b| b.is_ascii_hexdigit()))
+    {
+        use sha2::{Digest, Sha256};
+        let hashed: String = Sha256::digest(token.as_bytes())
+            .iter()
+            .map(|b| format!("{b:02x}"))
+            .collect();
+        sqlx::query("DELETE FROM mbr_session WHERE token_hash=?1")
+            .bind(hashed)
+            .execute(&mut *tx)
+            .await
+            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
+    }
+    tx.commit()
+        .await
+        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
+    Ok((
+        [
+            (
+                "set-cookie",
+                "chat_session=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0",
+            ),
+            ("cache-control", "no-store"),
+        ],
+        Redirect::to("/signup/complete"),
+    )
+        .into_response())
 }
 
 #[cfg(test)]
@@ -180,7 +230,7 @@ mod day22_tests {
                 .unwrap();
             assert_eq!(response.status(), expected);
             if expected == StatusCode::SEE_OTHER {
-                assert_eq!(response.headers()["location"], "/signin");
+                assert_eq!(response.headers()["location"], "/signup/complete");
             }
         }
         let (count,): (i64,) = sqlx::query_as("SELECT count(*) FROM mbr_user")
diff --git a/apps/server/tests/day24_signup_complete.rs b/apps/server/tests/day24_signup_complete.rs
new file mode 100644
index 0000000..f468900
--- /dev/null
+++ b/apps/server/tests/day24_signup_complete.rs
@@ -0,0 +1,122 @@
+use axum::{
+    body::{Body, to_bytes},
+    http::{Request, StatusCode},
+};
+use tower::ServiceExt;
+#[tokio::test]
+async fn day24_signup_completes_without_automatic_login_or_navigation() {
+    let pool = sqlx::sqlite::SqlitePoolOptions::new()
+        .max_connections(1)
+        .connect("sqlite::memory:")
+        .await
+        .unwrap();
+    for sql in [include_str!("../../../migrations/20261018073931_create_auth_user.sql"),
+                include_str!("../../../migrations/20261019221857_member_identity.sql"),
+                include_str!("../../../migrations/20261020152737_mbr_sessions.sql")] {
+        sqlx::raw_sql(sql).execute(&pool).await.unwrap();
+    }
+    let app = server::signup::routes(pool.clone());
+    let response=app.clone().oneshot(Request::post("/signup").header("origin","http://127.0.0.1:3000").header("content-type","application/x-www-form-urlencoded").body(Body::from("username=day24&email=day24%40example.test&display_name=Learner&password=lesson-only-password")).unwrap()).await.unwrap();
+    assert_eq!(response.status(), StatusCode::SEE_OTHER);
+    assert_eq!(response.headers()["location"], "/signup/complete");
+    assert!(
+        response.headers()["set-cookie"]
+            .to_str()
+            .unwrap()
+            .contains("Max-Age=0")
+    );
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM mbr_session")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count, 0);
+    let page = app
+        .clone()
+        .oneshot(
+            Request::get("/signup/complete")
+                .body(Body::empty())
+                .unwrap(),
+        )
+        .await
+        .unwrap();
+    assert_eq!(page.status(), StatusCode::OK);
+    assert!(!page.headers().contains_key("location"));
+    let html =
+        String::from_utf8(to_bytes(page.into_body(), 10000).await.unwrap().to_vec()).unwrap();
+    assert!(html.contains("註冊完成"));
+    assert!(html.contains("目前保持登出"));
+    assert!(html.contains(r#"href="/""#));
+    assert!(!html.to_lowercase().contains(r#"http-equiv="refresh""#));
+    assert!(!html.contains("<script"));
+    let duplicate=app.oneshot(Request::post("/signup").header("origin","http://127.0.0.1:3000").header("content-type","application/x-www-form-urlencoded").body(Body::from("username=DAY24&email=other%40example.test&display_name=Learner&password=lesson-only-password")).unwrap()).await.unwrap();
+    assert_eq!(duplicate.status(), StatusCode::CONFLICT);
+}
+
+#[tokio::test]
+async fn day24_existing_browser_session_revocation_is_atomic() {
+    use sha2::{Digest, Sha256};
+    let pool = sqlx::sqlite::SqlitePoolOptions::new()
+        .max_connections(1)
+        .connect("sqlite::memory:")
+        .await
+        .unwrap();
+    for sql in [include_str!("../../../migrations/20261018073931_create_auth_user.sql"),
+                include_str!("../../../migrations/20261019221857_member_identity.sql"),
+                include_str!("../../../migrations/20261020152737_mbr_sessions.sql")] {
+        sqlx::raw_sql(sql).execute(&pool).await.unwrap();
+    }
+    let app = server::signup::routes(pool.clone());
+    let make = |name: &str, cookie: Option<String>| {
+        let mut req = Request::post("/signup")
+            .header("origin", "http://127.0.0.1:3000")
+            .header("content-type", "application/x-www-form-urlencoded");
+        if let Some(c) = cookie {
+            req = req.header("cookie", c);
+        }
+        req.body(Body::from(format!("username={name}&email={name}%40example.test&display_name=Learner&password=lesson-only-password"))).unwrap()
+    };
+    assert_eq!(
+        app.clone()
+            .oneshot(make("older", None))
+            .await
+            .unwrap()
+            .status(),
+        StatusCode::SEE_OTHER
+    );
+    let raw = "a".repeat(64);
+    let hash: String = Sha256::digest(raw.as_bytes())
+        .iter()
+        .map(|b| format!("{b:02x}"))
+        .collect();
+    sqlx::query("INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at) SELECT 'old-session',id,?1,unixepoch()+1000 FROM mbr_user WHERE user_acnt='older'").bind(hash).execute(&pool).await.unwrap();
+    sqlx::raw_sql("CREATE TRIGGER test_revoke_failure BEFORE DELETE ON mbr_session BEGIN SELECT RAISE(ABORT,'fixture'); END;").execute(&pool).await.unwrap();
+    assert_eq!(
+        app.clone()
+            .oneshot(make("newer", Some(format!("chat_session={raw}"))))
+            .await
+            .unwrap()
+            .status(),
+        StatusCode::INTERNAL_SERVER_ERROR
+    );
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM mbr_user WHERE user_acnt='newer'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count, 0);
+    sqlx::query("DROP TRIGGER test_revoke_failure")
+        .execute(&pool)
+        .await
+        .unwrap();
+    assert_eq!(
+        app.oneshot(make("newer", Some(format!("chat_session={raw}"))))
+            .await
+            .unwrap()
+            .status(),
+        StatusCode::SEE_OTHER
+    );
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM mbr_session")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count, 0);
+}
```

### Refactor：把結果畫面與寫入責任分開

`complete` 只呈現固定結果；`signup` 負責驗證、加鹽、交易和導向。不要將清除 cookie 當作資料庫撤銷，也不要先提交會員再嘗試撤銷，否則失敗時重送註冊可能衝突。測試故意用 trigger 製造撤銷失敗，證明交易回滾。姓名／帳號等輸入不插入完成 HTML，避免增加輸出跳脫責任。

### 舊 Day 更正補充

本日未重開任何封存文章；Day22 補教佇列已在 Day23 完成。本篇僅說明最新 main 的 fresh migration 阻擋，沒有提供未經驗證的舊庫修復程式，也不將其標成已解決。舊 SHA 的正式驗證不能直接沿用到本日 SHA。

## 3. 套件版本與平台界線

本日無新增依賴，`Cargo.toml`／`Cargo.lock` 原樣保留。實際核對官方 registry：Axum 0.8.9、Argon2 0.6.0、SQLx 0.9.0、Leptos 0.8.22、Serde 1.0.229、Tokio 1.54.0、UUID 1.28.0 均與本次查得最新 stable 相同；SHA2 lock 同時有 0.10.9 與 0.11.1，直接 workspace 採 0.11，不因傳遞依賴另升級 lock。[官方套件版本資料](https://crates.io/api/v1/crates/argon2)。原始逐套件查詢與 lock 對照見驗證附件。

本機 Rust/Cargo 1.99.1、edition 2024，native Linux x86_64 編譯測試；沒有另跑最低 MSRV、Windows、macOS。SSR check 通過；本日修改伺服器與靜態 HTML，沒有修改 WASM，但也沒有以先前 WASM 成功冒稱本日重新建置。SQLite schema 切片可用不代表已修好 root 全套 migration。

## 4. 驗證項目、證據與未完成

- `cargo test --locked -p server --test day24_signup_complete`：基準 Red 的實際導向 `/signin` 不符合預期；原始 log 保留。
- `cargo test --locked -p server day2`：Green 共 5 項通過、0 失敗（3 項既有 library + 2 項新流程）。涵蓋註冊、新頁、維持登出、重複帳號、舊 cookie 撤銷、故障回滾；fixture 真正套 auth／identity／session SQL，沒有用假 HTTP 回應。
- `cargo check --locked -p server --features ssr`：通過，有既有 UI unused warnings。
- `git apply --cached --check candidate.patch`：精確 c965681 基準通過。
- 全套 root migration 在空白記憶體 SQLite：失敗於 20261018094242。這是已知產品阻擋，不是成功驗收。
- 發布 gate 的 11 項測試通過：拒絕簡短測試內容、全文截斷、程式碼空白差異、缺節、索引缺漏、不符預期 Day、衝突重試；成功重試保持冪等。這些是發布機制測試，仍須實際寫入本篇並全文讀回才可推進。

精確來源 SHA 的 CI 如下；CI 不執行這份隔離教學 patch，成功只涵蓋其 job，本地 DB 與流程結果另列：

- [Rust AI Chat](https://github.com/williechen/rust_ai_chat/actions/runs/39465675559)：success；更新 2026-10-20T17:38:52Z；jobs build-chat=success, check-chat=success
- [Rust Desktop Pet](https://github.com/williechen/rust_ai_chat/actions/runs/39465675538)：success；更新 2026-10-20T17:34:10Z；jobs check-pet=success, build-pet=success
- [Persistence](https://github.com/williechen/rust_ai_chat/actions/runs/39465675580)：failure；更新 2026-10-20T17:25:52Z；jobs test-persistence=failure
- [Node](https://github.com/williechen/rust_ai_chat/actions/runs/39465675860)：success；更新 2026-10-20T17:23:44Z；jobs test-node=success
- [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39465675661)：failure；更新 2026-10-20T17:26:17Z；jobs test-ai-core=success, test-chat-domain=success, test-server=failure, test-chat-application=success, test-desktop-pet=success, test-pet-domain=success, test-shared=success, test-ui=success, test-stats-server=success
- [Rust Dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39465675519)：success；更新 2026-10-20T17:26:41Z；jobs check-dashboard=success, build-dashboard=success

[完整 patch、原始測試／migration／CI／套件證據](https://github.com/williechen/copilot_use/tree/teaching/rust-ai-chat-alignment/docs/teaching/day24/chat-validation)。本篇未執行真瀏覽器、程序重啟、聊天室授權或真實舊庫升級，不宣稱產品整合與完整驗收。下一步依最新來源決定會員選房／權限鏈，不預排新 Day。

本篇是完整正式教學，不是寫入測試短文。只有四節全文、逐字程式區塊、來源 SHA 與 Chat 入口索引均讀回一致，並重新核對分支與 state blob 後，才允許 Chat 的 next_day 24→25；Pet、Analytics、makeup_queue、last_full_comparison 保留。排程恢復需三篇全部完成相同驗證，本次不建立或恢復排程。
