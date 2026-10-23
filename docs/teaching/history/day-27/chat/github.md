# Chat Day 27｜登入後帳號頁與房間頁共用同一 SQLite Session 驗證介面

## ① 學習大綱與真實現況

來源：`williechen/rust_ai_chat` `main@8cf82626c9c82939a3a56eab32e24a8a5b5b00ed`（唯讀）、`williechen/copilot_use` `main@c3891c77a47e7f3b140ecbde3ff7eae38f8f418c`；規則 `daily-tdd-minimum-v1 / github-only-v1`。本篇需由使用者套用，並非來源庫已變更或測試完成。**主體 Red + Green 預算 15 分鐘**；Refactor／版本核對與驗證另估 10–20 分鐘。

本次核對 `apps/server/src/lib.rs`、`signin.rs::account`、`room_auth.rs::require_member`、`logout.rs`、`crates/shared/src/auth.rs` 及既有 SQLite `mbr_session` schema：Day26 的 `POST /logout` 已於 main 實作，不重教；但是 `account` 與房間 middleware 仍各自重複 Cookie、SHA256、active user + expiry SQL 驗證。若一處撤銷／帳號狀態政策改變，另一處可能不一致。早期使用者流程需要兩個真實頁面共用同一資料庫認證 Port；本日切片只收斂這兩個呼叫端，不冒稱 API/WS 房間 ACL 已完成。來源包版本見第③節。

## ② 真 Red → 完整最小 Green → Refactor／FIFO

**Red（預期編譯失敗，未實際執行）：** 首先增加 `auth_session.rs` 測試中的 `active_member`／`csrf_for_logout` 呼叫；在未新增這兩個符號時編譯預期失敗。Green 對同一測試提供完整 Chat SQLite adapter，且把實際 `GET /account` 和 `GET /room/{room_id}` middleware 呼叫端接入，沒有重新建造另一套 Cookie parser。

**Green A — 新增 `apps/server/src/auth_session.rs`（先貼完整 production 部分）：**

```rust
use axum::http::{HeaderMap, StatusCode, header};
use shared::auth::{cookie_token, token_digest};
use sqlx::SqlitePool;

/// Chat SQLite adapter: caller supplies this application's independent DB pool.
/// No plaintext bearer token is ever stored in SQLite.
pub async fn active_member(
    pool: &SqlitePool,
    headers: &HeaderMap,
) -> Result<String, StatusCode> {
    let raw = headers.get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|value| cookie_token(value, "chat_session"))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let member: Option<String> = sqlx::query_scalar(
        "SELECT u.id FROM mbr_session s
         JOIN mbr_user u ON u.id=s.mbr_user_id
         WHERE s.token_hash=?1 AND s.expires_at>unixepoch()
           AND u.user_status='active'"
    )
    .bind(token_digest(&raw))
    .fetch_optional(pool).await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    member.ok_or(StatusCode::UNAUTHORIZED)
}

pub fn csrf_for_logout(headers: &HeaderMap) -> Result<String, StatusCode> {
    let raw = headers.get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| cookie_token(v, "chat_session"))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(token_digest(&format!("logout:{raw}")))
}
```

緊接於同一檔加入以下完整測試：

```rust
// Add to apps/server/src/auth_session.rs at the bottom:
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn session_adapter_obeys_revocation_and_account_state() {
        let db = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1).connect("sqlite::memory:").await.unwrap();
        sqlx::raw_sql(include_str!("../../../migrations/20261018073931_create_auth_user.sql"))
            .execute(&db).await.unwrap();
        sqlx::raw_sql(include_str!("../../../migrations/20261020152737_mbr_sessions.sql"))
            .execute(&db).await.unwrap();
        sqlx::query("INSERT INTO mbr_user
            (id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at)
            VALUES('member-27','learner','hash','lesson@example.invalid','Learner','active',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
            .execute(&db).await.unwrap();
        let raw = "a".repeat(64);
        sqlx::query("INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at)
            VALUES('session-27','member-27',?1,unixepoch()+60)")
            .bind(token_digest(&raw)).execute(&db).await.unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE,
            format!("other=0; chat_session={raw}").parse().unwrap());
        assert_eq!(active_member(&db, &headers).await.unwrap(), "member-27");
        assert_eq!(csrf_for_logout(&headers).unwrap(),
            token_digest(&format!("logout:{raw}")));
        sqlx::query("UPDATE mbr_user SET user_status='disabled' WHERE id='member-27'")
            .execute(&db).await.unwrap();
        assert_eq!(active_member(&db, &headers).await.unwrap_err(),
            StatusCode::UNAUTHORIZED);
        sqlx::query("UPDATE mbr_user SET user_status='active' WHERE id='member-27'")
            .execute(&db).await.unwrap();
        sqlx::query("DELETE FROM mbr_session WHERE id='session-27'")
            .execute(&db).await.unwrap();
        assert_eq!(active_member(&db, &headers).await.unwrap_err(),
            StatusCode::UNAUTHORIZED);
    }
}
```

**Green B — 將 `apps/server/src/room_auth.rs` 的開頭 imports 與 `require_member` 函式替換為下面內容；保留原有 `#[cfg(test)] mod day25_tests` 整段，不要刪除舊測試。** 既有測試模組使用 `Sha256`／`Digest`，需在該測試模組 imports 內新增 `use sha2::{Digest, Sha256};`。

```rust
use crate::auth_session::active_member;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use sqlx::SqlitePool;

/// Keep the existing #[cfg(test)] mod day25_tests unchanged.
pub async fn require_member(
    State(pool): State<SqlitePool>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    active_member(&pool, request.headers()).await?;
    Ok(next.run(request).await)
}
```

**Green C — `apps/server/src/signin.rs` 將既有 `account` 函式整段替換**，仍使用現有 router `/account`，登入後使用者可看到真正提交到 `/logout` 的表單。現有 login handler 保留；將 `signin.rs::token_digest` 內實作改為呼叫已存在 `shared::auth::token_digest(token)`，其餘 login SQL 不動。

```rust
async fn account(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
) -> Result<Html<String>, StatusCode> {
    crate::auth_session::active_member(&pool, &headers).await?;
    let csrf = crate::auth_session::csrf_for_logout(&headers)?;
    Ok(Html(format!(
        r#"<!doctype html>
<html lang="zh-Hant"><head><meta charset="utf-8"><title>已登入</title></head>
<body><h1>已登入</h1>
<form action="/logout" method="post">
<input type="hidden" name="csrf" value="{csrf}">
<button type="submit">登出</button></form>
<a href="/">回首頁</a></body></html>"#
    )))
}
```

**Green D — `apps/server/src/lib.rs` 的模組宣告加入 `pub mod auth_session;`**（與 `pub mod room_auth;` 同層），無須修改既有 Router 註冊，因為 `account` 和 `room_auth` 仍暴露原介面。

**Refactor：** `crates/shared/src/auth.rs` 負責 Cookie 與 SHA；`apps/server/src/auth_session.rs` 是 SQLite Session 檢查 adapter，註冊、登入、頁面、登出共用對應的 Session 資料表，保留每個系統自己的有效期限及登入政策。Analytics 使用 PostgreSQL，不應 import 此 SQLite adapter。需下一個完整切片把 `logout`、Chat API/WS，Analytics PostgreSQL 路由整合至符合各自 policy 的 Port；本日不是跨系統共用認證完成。

**FIFO 舊 Day25 補教：** `chat-day25-shared-auth-20261022` 維持 pending。Day25 的房間頁 Session Gate 已存在，不能重教／覆寫；本篇證明共用前置的一部分而非共享登入／登出／Session 全程達成，沒有充足證據結案。

## ③ Cargo、套件版本

來源 workspace edition 2024、版本 0.2.25；`axum 0.8`、`sqlx 0.9`（SQLite）、`sha2 0.11`、`tokio 1.54`、`serde 1.0`。本篇不增加 crate 或 migration，直接使用 `server` 既有依賴、`shared::auth`。沒有查明本次 Cargo.lock 的精確套件 resolved versions、MSRV 或 stable compiler 測試；不能以 Cargo.toml version requirements 代替 lock 證據。

## ④ 驗證：命令、預期／實際、CI/SHA、限制

| 項目 | 預期 | 本次實際 |
| --- | --- | --- |
| `cargo test -p server session_adapter_obeys_revocation_and_account_state` | 有效／停用／撤銷一致 | 未執行 |
| `cargo test -p server` | 舊 Day23 / Day25 頁面測試與本日測試通過 | 未執行 |
| `cargo check -p server --features ssr` | 原 SSR 與 middleware 編譯不退化 | 未執行 |
| 真實瀏覽器登入 → /account → /room/:id → logout | 兩頁同一 Session，登出後都拒絕 | 未部署／未驗收 |

`rust_ai_chat/main@8cf82626c9c82939a3a56eab32e24a8a5b5b00ed` CI：`Rust AI Chat` [Run 39785099385](https://github.com/williechen/rust_ai_chat/actions/runs/39785099385) completed/success；`Test All` [39785099220](https://github.com/williechen/rust_ai_chat/actions/runs/39785099220) completed/success；`Node` [39785099280](https://github.com/williechen/rust_ai_chat/actions/runs/39785099280) completed/failure。這些僅測來源提交，不含本文尚未套用的 Green；沒有把舊 Success 當新功能驗收。Chat API `/api/rooms` 建房與 `/ws/{room_id}` 目前仍未透過會員／房間 ACL，之後先補實際權限檢查，再進階 UI。
