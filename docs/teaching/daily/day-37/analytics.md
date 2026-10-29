# Analytics Day37｜統計查詢 Session Guard 一致性：全部類型與精確類型共用保護

> 2026-10-25；程式只讀 `williechen/rust_ai_ring/main@5c713553daee4055738239f047d6c774fcc7a7a1`；文件 `williechen/copilot_use/main`。承接 [Day36 r2](../day-36/analytics-r2.md)。本文為**未套用、未測試、未通過版本 Gate**的後續 TDD 草稿，無任何功能驗收宣稱。

## ① 學習大綱

真實路由在 `analytics_apps/analytics_server/src/password_login.rs::build`：`GET /api/v1/events/{source}/summary` 經 `summary`，精確事件類型的 `GET /api/v1/events/{source}/summary/{event_type}` 經 `crate::source_kind_summary`；後者 Day36 已修復為 `require_query_session` 並在 PostgreSQL 實測。前者目前仍直接呼叫同模組私有 `session` 並自行判斷首次密碼變更限制，授權規則有重複實作風險。真鏈：管理者 Cookie → Axum Session middleware/handler → PostgreSQL `uac_login_session` 與 `uac_user` → `pg_store::summary_for_source` → JSON，Dashboard Leptos SSR 使用相同已登入 session，但角色/來源權限還沒完成。唯一功能切片：統一兩種查詢路徑的 guard，並保留管理頁重導、服務機器 token 攝取與首次改密碼保護。

## ② Red → 完整最小 Green → Refactor

### Red：新寫 HTTP Guard 回歸測試

在 `analytics_apps/analytics_server/src/password_login.rs` 最後加入測試（無 DB 的純驗證只覆蓋 Cookie 格式，不能冒充正式 Session 查詢）：

```rust
#[cfg(test)]
mod day37_guard_tests {
    use super::*;
    #[test]
    fn invalid_cookie_cannot_yield_login_token() {
        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, "analytics_session=source-token".parse().unwrap());
        assert_eq!(raw_cookie(&headers), None);
        headers.insert(header::COOKIE,
            format!("analytics_session={}", "a".repeat(64)).parse().unwrap());
        assert_eq!(raw_cookie(&headers), Some("a".repeat(64)));
    }
}
```

此測試在現行原始碼**預期已通過，不能當作失敗 Red**。真正 Red 必須對兩個正式 Router 路由使用隔離 PG 建立 `restricted` 人員 Session、已登出 Session 與來源 token，斷言兩路都分別回 403、401、401，並檢查查詢前授權生效。本篇目前未提供上述整合測試及 fixture 的完整建置，因此尚未達 Gate 1。

### Green：共用真實 Session Query Guard，不變動 SQLx 聚合

在 `analytics_apps/analytics_server/src/password_login.rs` 以以下完整函式取代原 `summary`：

```rust
async fn summary(
    State(state): State<IngestState>,
    Path(source): Path<String>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    // 與 Day36 精確事件類型正式 API 共用管理員 Session 保護。
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

`require_query_session`、`renewed`、`summary_for_source` 均已在本檔可見，`use crate::{IngestState,pg_store::summary_for_source}`、`axum::Json`/`Path`/`HeaderMap`/`StatusCode`/`Response` 均已存在；不增 imports、crate 或依賴，`POST /api/v1/events` 的機器 token contract 保持不變。這項替換保留原回應、Cookie 更新以及 401/403 的語義，避免複製原 restricted 邏輯。

### Refactor、過往問題與架構限制

這是既有 `analytics_server` 內部 guard 的小型重用，不把 `analytics_application`、`analytics_domain`、`analytics_ui` 空範本聲稱已接通。`analytics_persistence` 既有精確事件 adapter 已有真實 caller；本日不搬 SQLx 資料層。`analytics_apps/Cargo.toml`、`analytics_server/Cargo.toml` 原有 `axum`、`sqlx/postgres`、`argon2`、`leptos/ssr`、`postgres-integration` 等皆不新增或更改，`Cargo.lock` 不動。Analytics 未設定 Chat 的 `[[workspace.metadata.leptos]]`，不能虛構。上一 Day 的角色／來源 ACL、SSE、CSRF、真正多帳號登入仍未實作；本次只是避免 Session query guard 兩處漂移，不等於角色 ACL 已完成。精確來源 SHA 與歷史文章保留，不覆寫 Day36。

## ③ 套件與官方穩定版本核對

本次來源宣告 `axum 0.8`、`sqlx 0.9`、`argon2 0.6`、`leptos 0.8`、`tokio 1.54`；`analytics_apps/Cargo.lock` resolved `axum 0.8.9`、`sqlx 0.9.0`、`argon2 0.6.0`、`leptos 0.8.22`、`tokio 1.54.1`。Rust Edition 2024；2026-10-25 Rust 官方最新穩定為 [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)；本機 rustc 版本未知。各 crate 官方當日最新穩定版尚未逐一查明（[axum](https://crates.io/crates/axum)、[sqlx](https://crates.io/crates/sqlx)、[argon2](https://crates.io/crates/argon2)、[leptos](https://crates.io/crates/leptos)、[tokio](https://crates.io/crates/tokio)），不得冒稱 lock 即最新版本；SQLx 0.9 migration/PostgreSQL runtime、native toolchain 兼容性未重測。**文件 Gate 1 未完成**。

## ④ 驗證命令、預期與實際

使用者套用後在正確 workspace 執行：

```bash
cd /home/willie/Documents/rust_ai_ring/analytics_apps
cargo test -p analytics-server day37_guard_tests
cargo test -p analytics-server --features postgres-integration
cargo test -p analytics-persistence --features postgres-integration
cargo check -p analytics-server
```

真資料庫測試必須使用受隔離的 PostgreSQL `DATABASE_URL`，先確認 `analytics_apps/migrations`，絕不可接正式 DB。**預期**：原兩種 summary 均要求已完成首次改密碼的有效人員 Session，來源 token 不能讀；真 PgPool 查詢結果不變。**實際**：此次程式唯讀，新測試、native check、PG、HTTP、Leptos 瀏覽器與 CI 均未執行。本稿未完成正式 Router 整合 Red，因此不標 Gate 1 通過、不推進 Day、不封存。
