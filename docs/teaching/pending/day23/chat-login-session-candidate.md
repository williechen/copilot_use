# Chat Day23 登入 + SQLite Session 最小 Green 候選（待執行驗證）

**狀態：pending／非正式文章／不得推進 next_day。** 來源 `rust_ai_chat/main 5bcef70219b480a7999fdd0fbed5384d8d28d9be`；封存 Day22 不修改。此文件與 `chat-signup-sqlite-contract-candidate.md` 配合，前者需要先在真實遷移 SQLite 通過註冊測試。本篇提供「有帳號→輸入密碼→驗證雜湊→持久 Session→受保護個人頁」同一資料流，**未經編譯或執行，因此不能宣稱完整 Green 通過**。

## 1. 學習大綱／現況

- 現有 `apps/server/src/lib.rs` 合併 `signup::routes(db.clone())`；`/signin` 在 `crates/ui/src/lib.rs` 只有會員註冊連結；`AppState` 目前沒有使用者身分或 Session。
- 本日選一個前後端真正有用途的最小切片：從 /signin 點選「帳密登入」進 /login，送出真實帳密；驗證 SQLite `mbr_user.user_pd` 的 Argon2id 後，建立 SQLite 內持久且不存明文 token 的 Session，再以 HttpOnly cookie 開啟 /account。未登入 /account 401，錯誤密碼不得建立 Session。
- 後續仍未完成：原 /signin 直接表單與導回目標房、房間 ACL、WebSocket Session、閒置續期、改密碼撤銷、帳號等待、密碼重設、管理員首次改密碼。不得以 /account 成功冒稱整個聊天室受保護。
- 本檔 source 基底 Cargo workspace 0.2.22／edition 2024，Argon2 0.6、Axum 0.8、SQLx 0.9、Tokio 1.54、UUID 1.28。增加 `sha2.workspace = true` 到 `apps/server/Cargo.toml`，workspace 已宣告 sha2 0.11；lock 應以原 repo 更新後的實際解析值核實。本候選未驗 rustc/MSRV、SSR/WASM、Windows/macOS。
- Red/Green 主體估 15 分鐘，Refactor/證據驗證另計；測試與部署未執行。

## 2. 實作內容：Red → Green → Refactor

### Red：先寫與同一 Green 對應的行為測試

先添加 `mod signin;`，在 `apps/server/src/signin.rs` 新增下列測試；將 `signin::routes(pool.clone())` merge 到 application，測試引用下述待新增的 routes。在未加入 Green 時，預期因 routes 符號不存在而編譯失敗；本輪未實際執行 Red。

```rust
#[cfg(test)]
mod day23_tests {
    use super::*;
    use axum::{body::Body, http::Request};
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
            "INSERT INTO mbr_user (id,user_acnt,user_pd,user_email,user_name,user_status,user_sexy,user_birthday,is_init_pd,created_at,updated_at) VALUES ('member-1','learner',?1,'learner@example.test','Learner','active','', '2000-01-01',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)"
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
    let hashed = format!("{:x}", Sha256::digest(token.as_bytes()));
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
    let hashed = format!("{:x}", Sha256::digest(token.as_bytes()));
    let member: Option<String> = sqlx::query_scalar(
        "SELECT u.id FROM mbr_session s JOIN mbr_user u ON u.id=s.mbr_user_id WHERE s.token_hash=?1 AND s.expires_at > unixepoch() AND u.user_status='active'"
    ).bind(hashed).fetch_optional(&pool).await
        .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;
    member.ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(Html("<!doctype html><html lang=\"zh-Hant\"><meta charset=\"utf-8\"><h1>已登入</h1><a href=\"/\">回首頁</a></html>"))
}

#[cfg(test)]
mod day23_tests {
    // 將上面 Red 測試模組完整貼在此處，保持測試與 Green 在同一模組。
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

`apps/server/Cargo.toml` 的 `[dependencies]` 增加 `sha2.workspace = true`。既有 `argon2.workspace = true`、`tower` dev-dependency 保留；不要引入舊 `rand_core 0.6` 或重新建立 Day22 signup.rs。新的 `Cargo.lock` 必須在隔離可編譯環境重新解析／核對。

### Refactor／安全與舊 Day 補教

- 本切片故意使用獨立 `/login`、`/account`，避免改寫舊 SSR `/signin` 路由。後續可將表單併入既有 Leptos SignInPage，並由 Session/ACL 控制 `/room/:id`、API、WS；**目前尚未實現**。
- 密碼驗證在 blocking 執行緒；避免將密碼、token 明文寫入 log／DB；SQLite 只存 digest。錯誤密碼與無此帳號均回 401，但需要額外補登入限流與帳號等待，且可考慮 dummy Argon2 驗證以避免帳號存在性時間差。部署至非 loopback 時需要 TLS/Secure cookie、可信 origin/config、CSRF 策略、Session 撤銷與閒置續期。
- 欲提供真實可註冊→登入流程，前置 `mbr_user.user_sexy/user_birthday NOT NULL` 必須用可驗證的新前向 migration 解決；參照 `chat-signup-sqlite-contract-candidate.md`。未解決不得宣稱產品端到端通過。
- FIFO 舊 Day22 更正候選已存在於 `docs/teaching/pending/day23/chat-day22-corrections.md`；**必須在本日真正完成的新 Red／Green／Refactor 之後**才附至正式教學第2節，每篇不超過一個補教單元，不改 Day22 原文。

## 3. 套件版本

來源 manifest：workspace 0.2.22、edition 2024；Argon2 `0.6`、Axum `0.8`、SQLx `0.9`、Tokio `1.54`、UUID `1.28`、SHA2 `0.11`。這些是 Cargo.toml 的範圍，不冒充 lock 精確解析版。需要實際讀取本次 SHA 的 Cargo.lock 與版本來源，並在正式文章依最新穩定版、相容性和鎖版補充。

## 4. 驗證項目與實際結果

預期：Red 缺 routes 時編譯失敗；Green 在測試資料庫中得到 303 + HttpOnly Session cookie、可讀 /account 200、未登入 401、錯誤密碼 401；重啟或新 pool 仍可使用（需要額外檔案 SQLite integration test）。

預計指令：
```bash
cargo test --locked -p server day23_password_login -- --nocapture
cargo check --locked -p server --features ssr
cargo test --locked -p server
```

**實際：未執行**。執行環境無法連 github.com clone，原始錯誤：
`fatal: unable to access 'https://github.com/williechen/rust_ai_chat.git/': Could not resolve host: github.com`。
因此本頁不能稱 Green 通過、不能當可發表的完整已驗證文章，且不可更新 next_day。舊 CI/Test persistence 失敗與本新切片不具同 SHA 覆蓋關係，待核對 jobs。

## 確切續接點

先用隔離工作樹對新 migration、signin.rs、Cargo、SSR App 路由執行真正 Red → Green；補齊 cookie 安全、session 重啟與註冊前置 migration 測試。確認完整可套用後將本文同源程序與 Day22 FIFO 更正整合成 Notion 四節正式原始 Day23；讀回完整程式碼，然後只在 GitHub 最新 blob/branch 條件吻合時把 Chat 23→24，同步 manifest 並讀回。Chat 完成才按 Pet、Analytics 順序進行。
