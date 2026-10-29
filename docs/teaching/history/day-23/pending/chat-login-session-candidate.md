# Chat Day23 登入 + SQLite Session 最小 Green 候選（本地驗證修訂）

**狀態：pending／非正式文章／不得推進 next_day。** 來源 `rust_ai_chat/main f38360cd537daf6dc20e1d3868221bb56390cc7c`；封存 Day22 不修改。此文件與 `chat-signup-sqlite-contract-candidate.md` 配合，註冊候選已重建：最新 main 的選填欄位已 nullable，不再新增重建 mbr_user 的 migration。本篇提供「有帳號→輸入密碼→驗證雜湊→持久 Session→受保護個人頁」同一資料流，**本地候選測試已執行；尚未發布，仍非完整產品驗收**。

## 1. 學習大綱／現況

- 現有 `apps/server/src/lib.rs` 合併 `signup::routes(db.clone())`；`/signin` 在 `crates/ui/src/lib.rs` 只有會員註冊連結；`AppState` 目前沒有使用者身分或 Session。
- 本日選一個前後端真正有用途的最小切片：從 /signin 點選「帳密登入」進 /login，送出真實帳密；驗證 SQLite `mbr_user.user_pd` 的 Argon2id 後，建立 SQLite 內持久且不存明文 token 的 Session，再以 HttpOnly cookie 開啟 /account。未登入 /account 401，錯誤密碼不得建立 Session。
- 後續仍未完成：原 /signin 直接表單與導回目標房、房間 ACL、WebSocket Session、閒置續期、改密碼撤銷、帳號等待、密碼重設、管理員首次改密碼。不得以 /account 成功冒稱整個聊天室受保護。
- 本檔 source 基底 Cargo workspace 0.2.22／edition 2024，Argon2 0.6、Axum 0.8、SQLx 0.9、Tokio 1.54、UUID 1.28。增加 `sha2.workspace = true` 到 `apps/server/Cargo.toml`，workspace 已宣告 sha2 0.11；lock 應以原 repo 更新後的實際解析值核實。本地 rustc／cargo 1.99.1；SSR 實測見第4節，MSRV／WASM／Windows／macOS 未重驗。
- Red/Green 主體估 15 分鐘，Refactor/證據驗證另計；測試結果見第4節；部署未執行。

## 2. 實作內容：Red → Green → Refactor

### Red：先寫與同一 Green 對應的行為測試

先添加 `mod signin;`，在 `apps/server/src/signin.rs` 新增下列測試；將 `signin::routes(pool.clone())` merge 到 application，測試引用下述待新增的 routes。Red 階段先加本篇 migration 與 sha2 依賴、解析 lock，保留測試明確的 StatusCode／PasswordHasher 匯入；Green 階段以完整檔案取代，勿重複貼入測試模組。在未加入 Green 時，預期因 routes 符號不存在而編譯失敗；本輪實際執行得到 E0425（缺 routes），屬預期符號缺失 Red，並非行為失敗。

```rust
#[cfg(test)]
mod day23_tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}};
    use argon2::PasswordHasher;
    use tower::ServiceExt;

    #[tokio::test]
    async fn day23_password_login_creates_persisted_session_and_guards_account() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new().max_connections(1)
            .connect("sqlite::memory:").await.unwrap();
        sqlx::raw_sql(include_str!("../../../migrations/20261018073931_create_auth_user.sql"))
            .execute(&pool).await.unwrap();
        sqlx::raw_sql(include_str!("../../../migrations/20261020180000_mbr_sessions.sql"))
            .execute(&pool).await.unwrap();
        let encoded = argon2::Argon2::default()
            .hash_password(b"lesson-only-password").unwrap().to_string();
        sqlx::query(
            "INSERT INTO mbr_user (id,user_acnt,user_pd,user_email,user_name,user_status,user_sexy,user_birthday,is_init_pd,created_at,updated_at) VALUES ('member-1','learner',?1,'learner@example.test','Learner','active',NULL,NULL,FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)"
        ).bind(encoded).execute(&pool).await.unwrap();

        let app = routes(pool.clone());
        let denied = app.clone().oneshot(Request::get("/account")
            .body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(denied.status(), StatusCode::UNAUTHORIZED);

        let bad = app.clone().oneshot(Request::post("/login")
            .header("origin","http://127.0.0.1:3000")
            .header("content-type","application/x-www-form-urlencoded")
            .body(Body::from("username=learner&password=wrong-password")).unwrap())
            .await.unwrap();
        assert_eq!(bad.status(), StatusCode::UNAUTHORIZED);

        let ok = app.clone().oneshot(Request::post("/login")
            .header("origin","http://127.0.0.1:3000")
            .header("content-type","application/x-www-form-urlencoded")
            .body(Body::from("username=LEARNER&password=lesson-only-password")).unwrap())
            .await.unwrap();
        assert_eq!(ok.status(), StatusCode::SEE_OTHER);
        assert_eq!(ok.headers()["location"], "/account");
        let cookie = ok.headers()[axum::http::header::SET_COOKIE].to_str().unwrap()
            .split(';').next().unwrap().to_owned();
        let allowed = app.oneshot(Request::get("/account")
            .header(axum::http::header::COOKIE,cookie)
            .body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(allowed.status(), StatusCode::OK);
        let (count,): (i64,) = sqlx::query_as("SELECT count(*) FROM mbr_session")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count, 1);
    }
}
```

### Green A：新 SQLite migration（不得改舊 migration）

檔案 `migrations/20261020180000_mbr_sessions.sql`。建立新表，憑證只存 SHA-256 digest，登入成功才插入。需核對這個 timestamp 尚未被最新分支使用。

```sql
CREATE TABLE mbr_session (
    id TEXT NOT NULL PRIMARY KEY,
    mbr_user_id TEXT NOT NULL REFERENCES mbr_user(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
);
CREATE INDEX idx_mbr_session_user_expires
    ON mbr_session(mbr_user_id, expires_at);
```

### Green B：完整 HTTP handler

檔案：`apps/server/src/signin.rs`。在以下原生 HTML 教學切片，認證 cookie 使用 `HttpOnly; SameSite=Lax; Path=/`，來源站點僅限既有 loopback listener。HTTPS 上線前必須另設定 `Secure` 與正確 domain/proxy origin；本範例不是可直接對外部署的完整網路安全設計。

```rust
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    Form, Router,
    extract::{DefaultBodyLimit, State},
    http::{HeaderMap, StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

fn token_digest(token: &str) -> String {
    Sha256::digest(token.as_bytes()).iter().map(|byte| format!("{byte:02x}")).collect()
}

#[derive(Deserialize)]
struct Credentials { username: String, password: String }

pub fn routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/login", get(login_page).post(login))
        .route("/account", get(account))
        .layer(DefaultBodyLimit::max(4096))
        .with_state(pool)
}

async fn login_page() -> Html<&'static str> {
    Html(r#"<!doctype html><html lang="zh-Hant"><head><meta charset="utf-8">
    <title>帳密登入</title></head><body><h1>帳密登入</h1>
    <form action="/login" method="post">
    <label>帳號<input name="username" autocomplete="username" required></label>
    <label>密碼<input name="password" type="password" autocomplete="current-password" required></label>
    <button type="submit">登入</button></form><a href="/">回首頁</a></body></html>"#)
}

async fn login(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Form(input): Form<Credentials>,
) -> Result<Response, StatusCode> {
    if headers.get(header::ORIGIN).and_then(|v|v.to_str().ok())
        != Some("http://127.0.0.1:3000") {
        return Err(StatusCode::FORBIDDEN);
    }
    let name = input.username.trim();
    if name.is_empty() || name.len()>32 || input.password.is_empty()
        || input.password.len()>128 {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let found: Option<(String,String,bool)> = sqlx::query_as(
        "SELECT id,user_pd,(user_status='active') FROM mbr_user WHERE lower(user_acnt)=lower(?1)"
    ).bind(name).fetch_optional(&pool).await
        .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;
    let (user_id, hash, active) = found.ok_or(StatusCode::UNAUTHORIZED)?;
    if !active { return Err(StatusCode::UNAUTHORIZED); }
    let password = input.password;
    let verified = tokio::task::spawn_blocking(move || {
        let parsed = PasswordHash::new(&hash).map_err(|_| ())?;
        Argon2::default().verify_password(password.as_bytes(), &parsed)
            .map(|_| true).map_err(|_| ())
    }).await.map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;
    if verified.is_err() { return Err(StatusCode::UNAUTHORIZED); }

    // 兩個獨立 UUIDv4 組成不可預測的 session bearer；不直接入庫。
    let token = format!("{}{}", uuid::Uuid::new_v4().simple(), uuid::Uuid::new_v4().simple());
    let hashed = token_digest(&token);
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO mbr_session (id,mbr_user_id,token_hash,expires_at) VALUES (?1,?2,?3,unixepoch()+86400)"
    ).bind(id).bind(user_id).bind(hashed).execute(&pool).await
        .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;
    let cookie = format!("chat_session={token}; HttpOnly; SameSite=Lax; Path=/; Max-Age=86400");
    Ok((StatusCode::SEE_OTHER,
        [(header::LOCATION, "/account"), (header::SET_COOKIE, cookie.as_str())]
    ).into_response())
}

async fn account(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
) -> Result<Html<&'static str>,StatusCode> {
    let token = headers.get(header::COOKIE).and_then(|v|v.to_str().ok())
        .and_then(|h| h.split(';').filter_map(|x|x.trim().split_once('='))
            .find(|(key,_)|*key=="chat_session").map(|(_,value)|value.to_owned()))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if token.len()!=64 || !token.bytes().all(|x|x.is_ascii_hexdigit()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let hashed = token_digest(&token);
    let member: Option<String> = sqlx::query_scalar(
        "SELECT u.id FROM mbr_session s JOIN mbr_user u ON u.id=s.mbr_user_id WHERE s.token_hash=?1 AND s.expires_at > unixepoch() AND u.user_status='active'"
    ).bind(hashed).fetch_optional(&pool).await
        .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;
    member.ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(Html("<!doctype html><html lang=\"zh-Hant\"><meta charset=\"utf-8\"><h1>已登入</h1><a href=\"/\">回首頁</a></html>"))
}

#[cfg(test)]
mod day23_tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use argon2::PasswordHasher;
    use tower::ServiceExt;

    #[tokio::test]
    async fn day23_password_login_creates_persisted_session_and_guards_account() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new().max_connections(1)
            .connect("sqlite::memory:").await.unwrap();
        sqlx::raw_sql(include_str!("../../../migrations/20261018073931_create_auth_user.sql"))
            .execute(&pool).await.unwrap();
        sqlx::raw_sql(include_str!("../../../migrations/20261020180000_mbr_sessions.sql"))
            .execute(&pool).await.unwrap();
        let encoded = argon2::Argon2::default()
            .hash_password(b"lesson-only-password").unwrap().to_string();
        sqlx::query(
            "INSERT INTO mbr_user (id,user_acnt,user_pd,user_email,user_name,user_status,user_sexy,user_birthday,is_init_pd,created_at,updated_at) VALUES ('member-1','learner',?1,'learner@example.test','Learner','active',NULL,NULL,FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)"
        ).bind(encoded).execute(&pool).await.unwrap();

        let app = routes(pool.clone());
        let denied = app.clone().oneshot(Request::get("/account")
            .body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(denied.status(), StatusCode::UNAUTHORIZED);

        let bad = app.clone().oneshot(Request::post("/login")
            .header("origin","http://127.0.0.1:3000")
            .header("content-type","application/x-www-form-urlencoded")
            .body(Body::from("username=learner&password=wrong-password")).unwrap())
            .await.unwrap();
        assert_eq!(bad.status(), StatusCode::UNAUTHORIZED);

        let ok = app.clone().oneshot(Request::post("/login")
            .header("origin","http://127.0.0.1:3000")
            .header("content-type","application/x-www-form-urlencoded")
            .body(Body::from("username=LEARNER&password=lesson-only-password")).unwrap())
            .await.unwrap();
        assert_eq!(ok.status(), StatusCode::SEE_OTHER);
        assert_eq!(ok.headers()["location"], "/account");
        let cookie = ok.headers()[axum::http::header::SET_COOKIE].to_str().unwrap()
            .split(';').next().unwrap().to_owned();
        let allowed = app.oneshot(Request::get("/account")
            .header(axum::http::header::COOKIE,cookie)
            .body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(allowed.status(), StatusCode::OK);
        let (count,): (i64,) = sqlx::query_as("SELECT count(*) FROM mbr_session")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count, 1);
    }
}
```

路由接線：`apps/server/src/lib.rs` 新增 `pub mod signin;`，在 DB pool 與 signup_routes 初始化後新增：

```rust
let signin_routes = signin::routes(db.clone());
```

將原尾端 `.merge(signup_routes);` 改成：

```rust
.merge(signup_routes)
.merge(signin_routes);
```

前端呼叫者：`crates/ui/src/lib.rs` 的 `SignInPage` 在建立會員連結旁加入真實入口：

```rust
<p><a href="/login" rel="external">"帳密登入"</a></p>
```

`apps/server/Cargo.toml` 的 `[dependencies]` 增加 `sha2.workspace = true`。既有 `argon2.workspace = true`、`tower` dev-dependency 保留；不要引入舊 `rand_core 0.6` 或重新建立 Day22 signup.rs。本輪 `cargo test --offline` 已解析隔離 lock：server dependencies 只新增 `"sha2 0.11.1"`，無其他套件版本變更；先解析再執行 `--locked`。完整差異見同目錄 validation/candidate.patch。

### Refactor／安全與舊 Day 補教

- 本切片故意使用獨立 `/login`、`/account`，避免改寫舊 SSR `/signin` 路由。後續可將表單併入既有 Leptos SignInPage，並由 Session/ACL 控制 `/room/:id`、API、WS；**目前尚未實現**。
- 密碼驗證在 blocking 執行緒；避免將密碼、token 明文寫入 log／DB；SQLite 只存 digest。錯誤密碼與無此帳號均回 401，但需要額外補登入限流與帳號等待，且可考慮 dummy Argon2 驗證以避免帳號存在性時間差。部署至非 loopback 時需要 TLS/Secure cookie、可信 origin/config、CSRF 策略、Session 撤銷與閒置續期。
- 最新 main 的 `mbr_user.user_sexy/user_birthday` 已 nullable，fresh DB 註冊→登入已實測；不再套用舊候選的 DROP/RENAME migration。既有 DB checksum 升級仍失敗，參照 `chat-signup-sqlite-contract-candidate.md`；不能宣稱既有使用者端到端升級通過。
- FIFO 舊 Day22 更正候選已存在於 `docs/teaching/pending/day23/chat-day22-corrections.md`；**必須在本日新 Red／Green／Refactor 之後**才附至正式教學第2節，每篇不超過一個補教單元，不改 Day22 原文。

## 3. 套件版本

本輪 crates.io 官方 API max_stable_version 核對與 lock 一致；未修改來源 lock。精確解析：Argon2 0.6.0、password-hash 0.6.1、Axum 0.8.9、SQLx 0.9.0、Tokio 1.54.0、UUID 1.28.0、SHA2 0.11.1、Leptos 0.8.22、tower 0.5.3。來源 lock 另有 SHA2 0.10.9，server 明確使用 workspace 的 0.11.1。

來源 manifest：workspace 0.2.22、edition 2024；Argon2 `0.6`、Axum `0.8`、SQLx `0.9`、Tokio `1.54`、UUID `1.28`、SHA2 `0.11`。這些是 Cargo.toml 的範圍，不冒充 lock 精確解析版。需要實際讀取本次 SHA 的 Cargo.lock 與版本來源，並在正式文章依最新穩定版、相容性和鎖版補充。

## 4. 驗證項目與實際結果

教學來源 main `f38360cd537daf6dc20e1d3868221bb56390cc7c`；copilot_use 教學基底 `fc4a72825dac985e7d0a8e8e72edb7848349caa0`。完整可重現 patch、額外測試與 log 在同目錄 `validation/`。

- 原 main：`cargo test --locked -p server --features ssr`，library 8 passed／6 failed；短帳號 409≠422，另外五項因 `rooms.desc NOT NULL` 失敗。失敗停止後續 target，不把未執行算通過。
- 原候選：解析新增 sha2 依賴後，`cargo test --offline -p server day23_password_login` 因兩處 E0277 失敗。SHA2 0.11.1 digest 的 Array 未實作 LowerHex；第2節已改用逐 byte 十六進位 helper。這是 API／型別錯誤，不算 TDD Red。
- 正確 Red：只保留測試與明確 imports，`cargo test --locked -p server --lib day23_password_login` 得到缺 `routes` 的 E0425。
- 修正 Green：`cargo test --locked -p server day23 -- --nocapture`，5 項測試通過（登入單元 1、schema 契約 2、檔案 SQLite／checksum 2）；其他 target 的 0 測試不計入。
- 覆蓋：完整 migration runner、真實註冊→登入、大小寫帳號、303、HttpOnly/SameSite cookie、只存 token digest、pool 關閉重開後仍可存取、未登入／偽造／過期／停用拒絕、錯密碼與錯 Origin 不建 Session、foreign_key_check、舊庫 checksum 阻擋。
- SSR：`cargo check --locked -p server --features ssr` 已通過（既有 UI warnings），見 validation/ssr.log；本次已加入 SignInPage 真實 /login 連結與 server merge 接線。

**剩餘需求差異：** 本切片固定 24 小時到期，尚未實作「操作續期、後台設定閒置逾時」，不能當正式 Session 規則完成。/account 只有已登入提示，尚非完整個人管理；房間/API/WS 尚未受此 Session 保護；註冊仍 303 /signin，尚非現行需求的獨立完成訊息頁。限流、改密碼撤銷、閒置續期、選房導回、瀏覽器、程序重啟、MSRV／WASM／跨平台、真實使用者 DB 升級均未完成驗收。pool 重開不等於 OS 程序重啟。

同 SHA CI 已讀取，結果及時間在 validation/ci.json；不以舊 SHA CI 證明本地候選。第2節的新程式沒有推送 rust_ai_chat，也沒有修改 copilot_use 產品。Day22 FIFO 補教保留為獨立單元，發文時放在本日 Refactor 後且每篇最多一個；不回寫封存頁。

目前只更新未封存 Day23 候選，未做 Notion 正式發布／全文讀回、未改 next_day 或 makeup_queue、未改 last_full_comparison、未建立或修改排程。Chat 尚未完成發布，不進入 Pet／Analytics。
