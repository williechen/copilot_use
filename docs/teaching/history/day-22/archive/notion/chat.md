## 1. 學習大綱
Day 22 r2 延續原頁：會員註冊表單 → Axum POST /signup → 加鹽 Argon2id → SQLite mbr_user → 303 /signin。主體 Red／Green 估 15 分鐘；Refactor 3 分鐘、來源／版本／驗證閱讀另估 8 分鐘。登入、Session、聊天室授權仍未接通，註冊完成不等於 P0 驗收。
來源 rust_ai_chat/main `da5155265bf66ef9d495fd38913ad6cb93e1bac8`；教學分支基底 `b3db92fcc1ff488b758eee5702f320d623d603b8`。規則 daily-tdd-minimum-v1，正本 Notion 共同頁最後修改 2026-10-19T08:19:19.122Z。2026-10-20 Asia/Taipei 重新核對原碼、呼叫端、migration、Day21 與三專案入口。入口歷史 next_day=19/17 與目前 learning-state=22 不一致；屬歷史敘述，不據此回退。GitHub 規則 blob 記於本輪證據。
現有 /signin 只有提示；mbr_user 已有欄位但沒有註冊呼叫者。copilot_use 使用 /api/v1/auth/register、chat_users 與不同 AppState，本日範例不可直接套用產品，也不宣稱產品已整合。rust_ai_chat 全庫唯讀，只提供由使用者套用的完整修改。
前置：既有 SQLite migrations 可成功執行、可編譯 native SSR；若 checksum mismatch 或舊帳號大小寫／Email 衝突，停下人工處理，不清空 DB、不改舊 migration。
## 2. 實作內容
### Red：先加測試
先在 apps/server/src/[lib.rs](http://lib.rs) 宣告 `mod signup;`，新增 apps/server/src/[signup.rs](http://signup.rs)，只放以下測試；先加入後述 dev-dependency。執行 `cargo test -p server day22_signup`，此時缺少 routes 等符號，預期編譯失敗。本輪未實測此 Red，不把環境或匯入錯誤當成行為 Red。測試涵蓋 POST、DB、大小寫衝突、密碼驗證及錯誤 Origin。
```rust
#[cfg(test)]
mod day22_tests {
    use super::*;
    use argon2::{PasswordHash, PasswordVerifier};
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn day22_signup_persists_and_rejects_duplicate() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new().max_connections(1)
            .connect("sqlite::memory:").await.unwrap();
        sqlx::raw_sql(include_str!("../../../migrations/20261018073931_create_auth_user.sql"))
            .execute(&pool).await.unwrap();
        sqlx::raw_sql(include_str!("../../../migrations/20261020060000_member_identity.sql"))
            .execute(&pool).await.unwrap();
        let app = routes(pool.clone());
        let body = "username=learner&email=learner%40example.test&display_name=Learner&password=lesson-only-password";
        for (input, expected) in [
            (body.to_owned(), StatusCode::SEE_OTHER),
            (body.replace("learner", "LEARNER"), StatusCode::CONFLICT),
            (body.replace("username=learner", "username=x"), StatusCode::UNPROCESSABLE_ENTITY),
        ] {
            let response = app.clone().oneshot(Request::post("/signup")
                .header("origin", "http://127.0.0.1:3000")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(input)).unwrap()).await.unwrap();
            assert_eq!(response.status(), expected);
            if expected == StatusCode::SEE_OTHER { assert_eq!(response.headers()["location"], "/signin"); }
        }
        let (count,): (i64,) = sqlx::query_as("SELECT count(*) FROM mbr_user").fetch_one(&pool).await.unwrap();
        assert_eq!(count, 1);
        let (hash,): (String,) = sqlx::query_as("SELECT user_pd FROM mbr_user").fetch_one(&pool).await.unwrap();
        assert!(hash.starts_with("$argon2id$"));
        Argon2::default().verify_password(b"lesson-only-password", &PasswordHash::new(&hash).unwrap()).unwrap();
        let response = app.oneshot(Request::post("/signup")
            .header("origin", "http://evil.invalid")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(body)).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
```
### Green：唯一索引及完整處理器
新增 migrations/20261020060000_member_identity.sql，既有 persistence::migrate 會套用。重複值造成 migration 失敗須人工裁定，不自動修資料。
```sql
-- 若既有帳號或 Email 大小寫衝突，migration 失敗並交由人工處理；不得刪資料。
CREATE UNIQUE INDEX uq_mbr_user_account_ci ON mbr_user(lower(user_acnt));
CREATE UNIQUE INDEX uq_mbr_user_email_ci ON mbr_user(lower(user_email));
```
在 apps/server/Cargo.toml 的 dependencies 加 `argon2 = "0.5"` 及 `rand_core = { version = "0.6", features = ["getrandom"] }`（啟用 OsRng）；新增 dev-dependencies 的 `tower = { version = "0.5", features = ["util"] }`。更新並提交 Cargo.lock；不要先用 --locked 阻止新增依賴。
將以下完整程式放在 apps/server/src/[signup.rs](http://signup.rs) 的測試之前：
```rust
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
use axum::{Router, Form, extract::{State, DefaultBodyLimit}, http::{HeaderMap, StatusCode}, response::{Html, Redirect}, routing::get};
use sqlx::SqlitePool;

#[derive(serde::Deserialize)]
struct Signup { username: String, email: String, display_name: String, password: String }

pub fn routes(pool: SqlitePool) -> Router {
    Router::new().route("/signup", get(page).post(signup))
        .layer(DefaultBodyLimit::max(4096)).with_state(pool)
}

async fn page() -> Html<&'static str> {
    Html(r#"<!doctype html><html lang="zh-Hant"><meta charset="utf-8"><title>會員註冊</title>
<h1>會員註冊</h1><form method="post" action="/signup">
<label>帳號<input name="username" required minlength="3" maxlength="32" autocomplete="username"></label>
<label>Email<input name="email" type="email" required maxlength="254" autocomplete="email"></label>
<label>顯示名稱<input name="display_name" required maxlength="32"></label>
<label>密碼<input name="password" type="password" required minlength="12" maxlength="128" autocomplete="new-password"></label>
<button type="submit">建立會員</button></form><a href="/signin">回登入頁</a></html>"#)
}

async fn signup(State(pool): State<SqlitePool>, headers: HeaderMap, Form(form): Form<Signup>) -> Result<Redirect, StatusCode> {
    if headers.get("origin").and_then(|v| v.to_str().ok()) != Some("http://127.0.0.1:3000") {
        return Err(StatusCode::FORBIDDEN);
    }
    let username = form.username.trim().to_owned();
    let email = form.email.trim().to_ascii_lowercase();
    let display = form.display_name.trim().to_owned();
    // 本切片 Email 為基本格式檢查；驗證信另列後續。
    let email_parts: Vec<_> = email.split('@').collect();
    if !(3..=32).contains(&username.len())
        || !username.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
        || !(1..=32).contains(&display.chars().count())
        || !email.is_ascii() || email.len() > 254 || email_parts.len() != 2
        || email_parts[0].is_empty() || !email_parts[1].contains('.')
        || email_parts[1].starts_with('.') || email_parts[1].ends_with('.')
        || email.bytes().any(|b| b.is_ascii_whitespace() || b.is_ascii_control())
        || !(12..=128).contains(&form.password.len()) {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    let password_hash = tokio::task::spawn_blocking(move || {
        Argon2::default().hash_password(form.password.as_bytes(), &SaltString::generate(&mut OsRng))
            .map(|hash| hash.to_string()).map_err(|_| ())
    }).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query("INSERT INTO mbr_user (id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at) VALUES (?1,?2,?3,?4,?5,'active',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
        .bind(uuid::Uuid::new_v4().to_string()).bind(username).bind(password_hash).bind(email).bind(display)
        .execute(&pool).await.map_err(|e| match e {
            sqlx::Error::Database(ref db) if db.is_unique_violation() => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;
    Ok(Redirect::to("/signin"))
}
```
apps/server/src/[lib.rs](http://lib.rs) 保留原路由；在 AppState 建立之後加入 `let signup_routes = signup::routes(db.clone());`，將原 `.with_state(state);` 改為：
```rust
.with_state(state)
.merge(signup_routes);
```
crates/ui/src/[lib.rs](http://lib.rs) 的 SignInPage，在回首頁連結前新增可操作入口：
```rust
<a href="/signup" rel="external">"建立會員"</a>
```
GET /signup 由獨立原生 HTML 表單回應，正常瀏覽器 submit 會附 Origin；本切片只接受既有 listener 的 http://127.0.0.1:3000。改部署來源需同時核對設定與 CSRF 契約。POST 限 4 KiB、驗證欄位後於 blocking thread 計算雜湊；唯一性由 DB 保障，409 不洩漏內部 SQL。Email 僅基本格式檢查，驗證信／寄信重設仍待完成。成功不建立 Session，回到目前仍不可登入的提示頁。
### Refactor 與舊 Day 補充
保留 AppState::new 原簽名與既有 fixture，註冊路由獨立持有 pool。Day21 的提示頁是前置接線，不是登入完成。本輪原草稿的跳脫程式及空函式以本頁完整範例取代；沒有回寫 Day21。既有 makeup_queue 若無項目不新增補教任務。
## 3. 套件版本
**2026-10-20 版本差異勘誤（尚未重驗 Green）**：目前 rust_ai_chat/main `cabacb75e946b03c67c455ce13a7a4ea85652f8e` 的 workspace 為 0.2.22／edition 2024。Cargo.toml：Axum 0.8、SQLx 0.9、Tokio 1.54、UUID 1.28、Leptos 0.8，`argon2 = "0.6"` 已在 workspace dependencies 並由 server 使用；server 測試依賴 tower 0.5（util）。Cargo.lock 解析 Argon2 0.6.0、Tokio 1.54.0。原 Day22 範例中 `argon2 = "0.5"`、獨立 `rand_core 0.6`、新增 [signup.rs](http://signup.rs)／migration 的步驟是舊來源候選，**不可直接套用新版 main**；須與最新 [signup.rs](http://signup.rs)、既有 migration、SaltString 及隨機鹽 API 重新合併並編譯測試。本篇舊 SHA／CI／通過紀錄只屬歷史證據，尚無新版 Green／產品驗收。
## 4. 驗證項目
命令 `cargo test -p server day22_signup`：預期 303、mbr_user 一筆、Argon2id 可驗密碼、重複 409、無效欄位 422、跨來源 403。實際結果以本輪附註為準；候選在隔離目錄，未寫入 rust_ai_chat。
來源 CI [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39302157333) 精確 SHA 為上列 main，2026-10-19T18:23:57Z 開始；test-server job 成功，18:24:00Z–18:26:21Z，命令 cargo test --locked -p server。CI 未包含本日新增程式，不證明 Day22 Green、瀏覽器、真實使用者 DB、重啟或 Session 通過。
待使用者套用後：瀏覽器由登入提示頁進註冊、提交、獨立查 DB；重啟確認帳號仍在；確認舊重複資料升級會保留並中止。缺口：錯誤友善頁、驗證信、登入、Session、授權房間、產品整合與完整功能驗收。不得以本篇發布替代這些驗收。
本輪實際驗證附註：隔離候選 cargo test -p server day22_signup：1 passed／0 failed／0 ignored；其他 target 的 0 測試未計入。首次編譯因 OsRng 缺 getrandom 功能失敗，已補明確 rand_core 依賴並重跑通過。Red 未執行。未套用 main、未整合產品、未做瀏覽器或完整功能驗收。
版本與 SSR 補充：候選 lock 解析 Argon2 0.5.3／rand_core 0.6.4、Axum 0.8.9／SQLx 0.9.0／Tokio 1.53.3／Leptos 0.8.22。cargo check -p server --features ssr 通過，檢查含原 AppState 與註冊路由合併，不代替瀏覽器驗收。
本輪來源證據：GitHub 規則 blob 9f5efa59fee115405d2c3e5892c0460fed5bbc0f（教學基底 b3db92fcc1ff488b758eee5702f320d623d603b8）；隔離教學候選 commit 43d445ef367460ce1bb4e01f9b0c90d66f0db77a，未推送 rust_ai_chat。完整 patch SHA-256：8aedb7c8d4e01ce568201a779b05c7ff873ab8ace0c92b01ab4f328dcf43c50a，已對精確 main 基底 git apply --check 通過。copilot_use 基底 CI Verify copilot_use / run 39332356560 成功，compose-e2e skipped；僅屬產品既有程式證據，不代替本日教學測試。讀回與完整命令紀錄存教學分支 docs/teaching/day22/。
