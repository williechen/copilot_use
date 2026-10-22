## 1. 學習大綱與真實現況

原始教學 Day 25：保護 Chat 聊天室頁 GET /room/\{room\_id\}，不再讓未登入者直接進入。來源 rust\_ai\_chat/main SHA 08b891a22c1934a2bf0b1edbb103c9b16bcf12de，workspace 0.2.23。現有 apps/server/src/[signin.rs](http://signin.rs) 已實作 /login、/account、Argon2 驗證與 SQLite 持久 mbr\_session；apps/server/src/[lib.rs](http://lib.rs) 卻直接公開 GET /room/\{room\_id\}。隔離驗證報告中，未登入該 URL 回 200。這個 Day 只驗證已登入身分，房間 ACL、WebSocket/API 守門、登出、房間列表仍未完成；不重建 Login/Session。教學主體 15 分鐘，共用時間另計。

## 2. Red／完整最小 Green／Refactor

Red：先加入以下測試；Green 尚未實作時，使用無條件 [next.run](http://next.run)(request).await stub，沒有 cookie 的 GET 會回 200，與預期 401 不符。這是預期 Red，並未實際跑過。

檔案 apps/server/src/room\_[auth.rs](http://auth.rs)（先寫以下 test module）：

```rs
#[cfg(test)]
mod day25_tests {
    use super::*;
    use axum::{Router, body::Body, http::{Request, StatusCode}, routing::get};
    use sha2::{Digest, Sha256};
    use tower::ServiceExt;

    #[tokio::test]
    async fn room_page_requires_active_sqlite_session() {
        let db = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1).connect("sqlite::memory:").await.unwrap();
        sqlx::raw_sql(include_str!("../../../migrations/20261018073931_create_auth_user.sql"))
            .execute(&db).await.unwrap();
        sqlx::raw_sql(include_str!("../../../migrations/20261020152737_mbr_sessions.sql"))
            .execute(&db).await.unwrap();
        sqlx::query("INSERT INTO mbr_user (id,user_acnt,user_pd,user_email,user_name,user_status,user_sexy,user_birthday,is_init_pd,created_at,updated_at) VALUES ('m1','member','unused','m1@example.test','Member','active','', '2000-01-01',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
            .execute(&db).await.unwrap();
        let token = "a".repeat(64);
        let hash = Sha256::digest(token.as_bytes()).iter()
            .map(|b| format!("{b:02x}")).collect::<String>();
        sqlx::query("INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at) VALUES('s1','m1',?1,unixepoch()+60)")
            .bind(hash).execute(&db).await.unwrap();
        let app = Router::new()
            .route("/room/{room_id}", get(|| async { StatusCode::OK }))
            .route_layer(axum::middleware::from_fn_with_state(db.clone(), require_member));
        for (cookie, expected) in [
            (None, StatusCode::UNAUTHORIZED),
            (Some("chat_session=invalid".to_owned()), StatusCode::UNAUTHORIZED),
            (Some(format!("chat_session={token}")), StatusCode::OK),
        ] {
            let mut request = Request::get("/room/real-room");
            if let Some(value) = cookie { request = request.header("cookie", value); }
            let response = app.clone().oneshot(request.body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), expected);
        }
        sqlx::query("UPDATE mbr_session SET expires_at=unixepoch()-1 WHERE id='s1'")
            .execute(&db).await.unwrap();
        let expired = app.oneshot(Request::get("/room/real-room")
            .header("cookie", format!("chat_session={token}"))
            .body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(expired.status(), StatusCode::UNAUTHORIZED);
    }
}
```

Green：在同一檔案的測試模組上方完整加入：

```rs
use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

pub async fn require_member(
    State(pool): State<SqlitePool>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let raw = request.headers().get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|cookie| cookie.split(';')
            .filter_map(|segment| segment.trim().split_once('='))
            .find(|(name, _)| *name == "chat_session")
            .map(|(_, value)| value.to_owned()))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if raw.len() != 64 || !raw.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let digest = Sha256::digest(raw.as_bytes()).iter()
        .map(|b| format!("{b:02x}")).collect::<String>();
    let member: Option<String> = sqlx::query_scalar(
        "SELECT u.id FROM mbr_session s JOIN mbr_user u ON u.id=s.mbr_user_id WHERE s.token_hash=?1 AND s.expires_at>unixepoch() AND u.user_status='active'"
    ).bind(digest).fetch_optional(&pool).await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    member.ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(next.run(request).await)
}
```

在 apps/server/src/[lib.rs](http://lib.rs) 的 pub mod signin; 附近加一行：

```rs
pub mod room_auth;
```

在 let state = state::AppState::new(...) 之後、let app = Router::new() 之前新增：

```rs
let guarded_room = Router::new()
    .route("/room/{room_id}", get(ssr::app_handler))
    .route_layer(axum::middleware::from_fn_with_state(
        db.clone(), room_auth::require_member,
    ));
```

<b>刪掉原本主 Router 的那一行公開 .route("/room/\{room\_id\}", get(ssr::app\_handler))；不要留下重複路由。</b> 在主 Router 的 .with\_state(state) 之前新增：

```rs
.merge(guarded_room)
```

原本 .with\_state(state).merge(signup\_routes).merge(signin\_routes) 保留。測試 stub 的 Red 與上述實作 Green 不可以同時在正式版本留下 stub。完整檔案形式是 Green 區塊緊接 Red 的 #\[cfg(test)\] module。

Refactor／功能限制：未受保護的 /api/rooms、/ws/\{room\_id\} 仍有繞過風險，Day 25 不宣稱全站授權。後續必須重用 Session 檢查、將 member\_id 傳進應用層，再對 mbr\_room 進行房間成員檢查，不能只依頁面 Cookie 判斷已授權進房。HTTP cookie token 不寫入 log；正式部署另需 TLS、Secure Cookie、CSRF/Origin 設計。Day24 及更早文章封存，不修改；本篇不消耗 Pet/Analytics Day24 FIFO 補教。

## 3. Cargo 與套件版本

來源 manifest：workspace 0.2.23，edition 2024，axum 0.8、sqlx 0.9、sha2 0.11、tokio 1.54、argon2 0.6、uuid 1.28。apps/server/Cargo.toml 已有 sha2.workspace=true，測試用 tower 0.5 with util。本切片不新增套件與 migration；精確 lock 版本、MSRV、Tauri 或部署環境未在本輪重測。

## 4. 命令、預期與實際結果

```bash
cargo test --locked -p server room_page_requires_active_sqlite_session -- --nocapture
cargo test --locked -p server
cargo check --locked -p server --features ssr
cargo fmt --all -- --check
```

預期：未登入 401，偽造 cookie 401，有效未過期會員 200，過期 401。Red（stub）預期失敗，Green 預期通過；<b>本次均未執行，不能聲稱測試成功</b>。該 test 只驗證中介層與 SQLite，真 Axum SSR、API、WS、Session 重啟及 UI 需獨立端到端驗證。

來源 SHA 08b891a22c1934a2bf0b1edbb103c9b16bcf12de 的既有 CI：[Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39595880206) completed/success；[Rust AI Chat](https://github.com/williechen/rust_ai_chat/actions/runs/39595880201) completed/success；[Persistence](https://github.com/williechen/rust_ai_chat/actions/runs/39595880327) completed/success；[Node](https://github.com/williechen/rust_ai_chat/actions/runs/39595880309) completed/failure。它們沒有執行本文新程式，不構成 Day25 Green 證據。[Day24 隔離測試報告](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-21-day24-main-08b891a/REPORT.md) 記錄 workspace 127 pass / 1 ignored 與未登入 room=200。教學程式未套用 main，產品整合 false，功能驗收 false。

### Day25 需求覆核更正

登入、登出、Cookie 與 Session 必須共用一套實作；各專案只透過儲存介面與設定區分 SQLite／PostgreSQL、Cookie 名稱及政策，不代表共用登入狀態。本文 room\_auth 仍重複解析 Cookie、雜湊及查詢 Session，尚未符合這項要求，不能作為共用套件的完成範例。必須先抽取並以實際呼叫端驗證共用 Session，再提供可執行的 Red／Green 與回歸紀錄。

目前僅文章發布及全文讀回完成。Red／Green、共用登入套件及功能驗收均未完成；進度中的 Day26 不代表這些項目已驗收，不得據此繼續遞增或恢復排程。Day24 及以前保持封存。