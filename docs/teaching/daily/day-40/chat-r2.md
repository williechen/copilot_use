# Chat Day40 r2｜管理路由已補回，舊失敗保留原SHA

2026-10-26；最新來源 `rust_ai_ring@9c767978b20bd56e1c2182dac2f37e6ae9b7d988`。本篇更正[r1](chat-r1.md)在舊SHA的狀態，保留原文、原patch與失敗證據；不新增Day或重構計功。完整原始Red／Green教學仍在r1，不能在已套用程式重貼修復patch。

## ① 最新狀態與修訂範圍

最新 `signin::routes` 已同時註冊 `/account` 和 `/api/session`，不再是「最新碼仍回404」。前輪ff14ca6的登入／登出2個失敗確實發生，不能刪掉舊log或改成當時已通過。這次只快轉更新本地，沒有代替使用者改產品程式。

目前版本不代表已完成真瀏覽器登入／管理切換、房間ACL或完整GUI驗收。

## Refactor

既有Day40重構仍是signin共用 `shared::auth::token_digest`；修復這次路由回歸沒有新增hash函式、schema或dependencies。`/account` 仍由原handler透過真Session授權，新增Session查詢不能取代舊管理路由。

## ② 修復已套用的正式caller與斷言

以下是最新來源的完整routes：

來源：[`chat_apps/chat_server/src/signin.rs::routes`](https://github.com/williechen/rust_ai_ring/blob/9c767978b20bd56e1c2182dac2f37e6ae9b7d988/chat_apps/chat_server/src/signin.rs#L19)。

```rust
pub fn routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/login", get(login_page).post(login))
        .route("/account", get(account))
        .route("/api/session", get(session_status))
        .route("/account/rooms", axum::routing::post(account_create_room))
        .layer(DefaultBodyLimit::max(4096))
        .with_state(pool)
}
```

對照：舊來源把/account換成/api/session，現在為並存；教學修復patch套在b0後的signin.rs與最新來源逐bytes相同。這是來源比對，不是本輪編譯測試。

新API仍只查有效Session並回204／401、no-store；管理handler自身授權不移除：

來源：[`chat_apps/chat_server/src/signin.rs::session_status`](https://github.com/williechen/rust_ai_ring/blob/9c767978b20bd56e1c2182dac2f37e6ae9b7d988/chat_apps/chat_server/src/signin.rs#L30)。

```rust
async fn session_status(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, StatusCode> {
    crate::auth_session::active_member(&pool, &headers).await?;
    Ok((
        StatusCode::NO_CONTENT,
        [(header::CACHE_CONTROL, "no-store")],
    ))
}
```

既有登入／管理測試和登出測試均保留，沒有修改斷言來接受404。前輪補回route的隔離候選workspace 59項通過及fmt/check通過，仍只屬該候選輸入。原始Red未重跑，本輪未將來源相同推論成新SHA已跑。

## ③ 版本與依賴

本次只改routes一行；四workspace manifests／locks沒有變動，沿[r1官方版本核對](r1-versions.json)。本輪不宣稱更新framework或新的官方版本普查。

## ④ 本輪實際檢查與待驗

三案 `cargo fmt --all -- --check` 均exit0。將已發布修復patch在乾淨b0副本實際check／apply後，Chat修改檔與最新SHA完全相同；指令、cwd、exit及hash見[本輪紀錄](source-update-9c76797/verification.json)。

**本輪沒有執行cargo check/test、真瀏覽器或CI。** 前輪57通過／2失敗與修復候選59通過分別保留其來源，不能標成最新SHA的重跑。下一步若做新來源驗收，執行workspace SSR、WASM與瀏覽器；仍需檢查Session失效、登出、/account以及房間授權。[現況](../../current/STATUS.md)統一區分已修復與待驗收，Day41不變。
