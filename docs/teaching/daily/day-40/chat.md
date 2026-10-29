# Chat Day40｜首頁依有效 Session 顯示「登入／管理」

> 2026-10-26；唯一產品來源 `williechen/rust_ai_ring/main@bb259aaf76542554a3a704d9c4f4a3a219d65cbf`。文件交付不等於產品套用。依 Day39 r2 [首次自動載入](../day-39/chat-r2.md) 後，首頁仍固定顯示「登入」；本篇補上真 Session 判斷。先套 Refactor，再套 Green；兩者是不同工作包，不互抵。

## ① 目標、現況與完成條件

未登入、Session 過期或帳號停用時，首頁顯示「登入」並前往 `/signin`；有效登入才顯示「管理」並前往既有 `/account`。首頁公開房間列表仍可匿名讀取。檢查使用既有 SQLite `mbr_session` 與 `mbr_user`，不把 cookie 存到 WASM 或把前台判斷當伺服器授權。載入中／無法確認狀態有明確文案；SSR 初始採保守登入入口，hydrate 後更新，SSR 不宣稱已確認身份。房間 ACL、選房後保留目標仍未接通，留待後續。

精確呼叫鏈：`HomePage` → `room_api::session_status` → `GET /api/session` → `signin::session_status` → `auth_session::active_member` → Chat SQLite。新端點只回 204／401，不回 token、會員資料；原 `/account` 仍自行驗 Session。核心60分鐘只計② Red／Green，本段、Refactor、版本、驗證、提交另計。

## Refactor

**獨立既有架構工作：移除 Chat 登入 adapter 重複雜湊實作**，不依賴 Day40 Session API。原 `chat_server/src/signin.rs` 自行以 SHA256 手寫 `token_digest`，而 `auth_session.rs` 與 `logout.rs` 已使用 `shared::auth::token_digest`。兩種實作分散使登入寫入的 hash 與後續讀取／撤銷的 hash 可能漂移。改為相同函式，保持原 SHA256 bytes／hex 行為與既有 DB token_hash 欄位，不改 cookie 格式、登入時效、Schema 或跨案 crate。

完整修改：`chat_apps/chat_server/src/signin.rs` 將 `use sha2::{Digest, Sha256};` 改為 `use shared::auth::token_digest;`，**刪除**原本的 `fn token_digest(token: &str) -> String`，既有 `let hashed = token_digest(&token);` 不變。這是登入路徑的單一責任整合，不是② Green 後的修飾；完整逐行可套用內容見 [Chat Refactor patch](chat-day40-refactor.patch)。

原 Day23 密碼登入→寫入 `mbr_session.token_hash`、Day25/27 Session、Day37/39 SSR 登入斷言保留；Day40 新 endpoint 只在②加入，不能算重構的測試。以既有 `signin::day23_tests`、`auth_session::tests`、`logout` 回歸確認登入 hash／登出 hash 一致。此 Refactor 不修改 Cargo、lock、migrations、module 或其他產品。**本次未實際執行 Cargo**；套用後先獨立測試本包再套 Green。下一個 Chat Refactor 應沿登入／Session 與房間 ACL 的真 caller 整理，勿把 Analytics/Pet 的 CI 放入本段。

## ② Red → 完整最小 Green

### Red：先加入失敗斷言

在 `chat_apps/chat_server/src/signin.rs` 加入下方完整測試模組（新增端點前 `GET /api/session` 得 404，而預期 204；這是**行為 Red**）。使用真 SQLite migration，測匿名 401、有效 Session 204／no-store、停用 401、過期 401。額外在 `chat_ui/src/lib.rs` 測四種首頁狀態映射（新增符號前為**編譯 Red**）。既有測試通過僅算回歸，不算 Red。

```rust
#[cfg(test)]
mod day40_session_entry_tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn session_endpoint_checks_active_revoked_and_expired_sqlite_session() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1).connect("sqlite::memory:").await.unwrap();
        chat_persistence::migrate(&pool).await.unwrap();
        sqlx::query("INSERT INTO mbr_user(id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at)
            VALUES('day40-member','day40','hash','day40@example.invalid','Day40','active',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
            .execute(&pool).await.unwrap();
        let raw = "a".repeat(64);
        sqlx::query("INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at)
            VALUES('day40-session','day40-member',?1,unixepoch()+60)")
            .bind(shared::auth::token_digest(&raw))
            .execute(&pool).await.unwrap();
        let app = routes(pool.clone());
        let anonymous = app.clone().oneshot(Request::get("/api/session")
            .body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(anonymous.status(), StatusCode::UNAUTHORIZED);
        let live = app.clone().oneshot(Request::get("/api/session")
            .header("cookie", format!("chat_session={raw}"))
            .body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(live.status(), StatusCode::NO_CONTENT);
        assert_eq!(live.headers()[header::CACHE_CONTROL], "no-store");
        sqlx::query("UPDATE mbr_user SET user_status='disabled' WHERE id='day40-member'")
            .execute(&pool).await.unwrap();
        let disabled = app.clone().oneshot(Request::get("/api/session")
            .header("cookie", format!("chat_session={raw}"))
            .body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(disabled.status(), StatusCode::UNAUTHORIZED);
        sqlx::query("UPDATE mbr_user SET user_status='active' WHERE id='day40-member'")
            .execute(&pool).await.unwrap();
        sqlx::query("UPDATE mbr_session SET expires_at=unixepoch()-1 WHERE id='day40-session'")
            .execute(&pool).await.unwrap();
        let expired = app.oneshot(Request::get("/api/session")
            .header("cookie", format!("chat_session={raw}"))
            .body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(expired.status(), StatusCode::UNAUTHORIZED);
    }
}
```

```rust
#[cfg(test)]
mod day40_home_entry_tests {
    use super::*;
    #[test]
    fn entry_does_not_claim_manage_without_confirmed_session() {
        assert_eq!(session_entry_display(SessionEntry::Checking).1, "登入");
        assert_eq!(session_entry_display(SessionEntry::SignedOut).0, "/signin");
        assert_eq!(session_entry_display(SessionEntry::Unavailable).0, "/signin");
        assert_eq!(session_entry_display(SessionEntry::SignedIn), ("/account", "管理", ""));
    }
}
```

### Green：Server／WASM API／Leptos 正式入口

完整可套用 [Chat Green patch](chat-day40-green.patch) 修改三個真實檔案，依序：

1. `chat_server/src/signin.rs` 的 `routes` 新增 `.route("/api/session", get(session_status))`，並新增完整 handler：

```rust
/// Valid, active SQLite member Session; never expose the bearer token.
async fn session_status(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, StatusCode> {
    crate::auth_session::active_member(&pool, &headers).await?;
    Ok((StatusCode::NO_CONTENT, [(header::CACHE_CONTROL, "no-store")]))
}
```

2. `chat_ui/src/room_api.rs` 保留現有 `web_sys`、`JsFuture`、`JsCast` imports 與搜尋 API；新增：

```rust
pub async fn session_status() -> Result<bool, String> {
    let window = web_sys::window().ok_or("無法取得瀏覽器")?;
    let value = JsFuture::from(window.fetch_with_str("/api/session"))
        .await.map_err(|_| "無法確認登入狀態")?;
    let response: web_sys::Response = value.dyn_into().map_err(|_| "無效 Session 回應")?;
    match response.status() {
        204 => Ok(true),
        401 => Ok(false),
        code => Err(format!("Session 查詢失敗：HTTP {code}")),
    }
}
```

3. `chat_ui/src/lib.rs` 在 `HomePage` 前新增狀態契約，`HomePage` 中依真實 fetch 切換入口，保持 `RoomList` 及其 Day39 自動讀取原樣：

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SessionEntry { Checking, SignedOut, SignedIn, Unavailable }

fn session_entry_display(state: SessionEntry) -> (&'static str, &'static str, &'static str) {
    match state {
        SessionEntry::Checking => ("/signin", "登入", "確認登入狀態中"),
        SessionEntry::SignedOut => ("/signin", "登入", ""),
        SessionEntry::SignedIn => ("/account", "管理", ""),
        SessionEntry::Unavailable => ("/signin", "登入", "無法確認登入狀態；可使用登入入口"),
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (entry, set_entry) = signal(SessionEntry::Checking);
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let next = match room_api::session_status().await {
                Ok(true) => SessionEntry::SignedIn,
                Ok(false) => SessionEntry::SignedOut,
                Err(_) => SessionEntry::Unavailable,
            };
            set_entry.set(next);
        });
    });
    #[cfg(not(target_arch = "wasm32"))]
    let _ = set_entry;
    view! {
        <p>"Welcome to Rust AI Chat!"</p>
        <p>
            <a href=move || session_entry_display(entry.get()).0 rel="external">
                {move || session_entry_display(entry.get()).1}
            </a>
            <span role="status">{move || session_entry_display(entry.get()).2}</span>
        </p>
        <RoomList />
    }
}
```

`chat-server` 已直接依賴 `chat-ui`、`shared`、`sqlx`、`axum`；`chat-ui` 已具 WASM fetch／`wasm_bindgen_futures`，不新增 Cargo dependency／feature／lock 或 Leptos metadata。正式呼叫者是 `HomePage`，不是僅測試 helper。原 Session、登出、房間 API、WS middleware 均不移除。

### 斷言承接、套用順序與下一步

先保留所有舊登入、Session、房間列表與 SSR 斷言；新增真 Router 204／401 與首頁狀態斷言，不以刪舊測試換全綠。兩份 patch 以此來源為基準，**先 Refactor 再 Green**。候選產品仍需自行實際 `git apply --check`、套用、fmt／build／test，無執行證據不能稱 Green 已通過。

```bash
# rust_ai_ring repository 根目錄；僅在使用者自己的隔離工作樹
git apply --check /path/to/chat-day40-refactor.patch
git apply /path/to/chat-day40-refactor.patch
cd chat_apps
cargo test --locked -p chat-server --lib
cd ..
git apply --check /path/to/chat-day40-green.patch
git apply /path/to/chat-day40-green.patch
cd chat_apps
cargo fmt --all -- --check
cargo test --locked -p chat-server --lib day40_session_entry_tests
cargo test --locked -p chat-ui --features ssr day40_home_entry_tests
cargo test --locked -p chat-server --features ssr
cargo check --locked -p chat-ui --target wasm32-unknown-unknown --features hydrate
cargo check --locked -p chat-server --features ssr
```

瀏覽器需測匿名公開列表、登入後管理、登出後登入、停用／過期 Session、網路失敗保守入口及 reload；真房間進入 ACL／選房目標保留是下一篇缺口。若不相容，以原始 `git apply -R` 分別回復，避免變更 DB。

## ③ 當日官方穩定版核對（2026-10-26）

| 技術 | 本來源宣告／Lock | 官方最新穩定（日期） | 判斷 |
| --- | --- | --- | --- |
| Rust | edition 2024／未讀本機 rustc | 1.99.1（10/15） | 需於實機確認 MSRV |
| Axum | 0.8／0.8.9 | 0.8.9（04/14） | 使用既有 Router |
| Leptos | 0.8／0.8.22 | 0.8.22（10/05） | SSR/hydrate 不改 |
| SQLx | 0.9／0.9.0 | 0.9.0（05/21） | SQLite migration 不改 |
| Tokio | 1.54／1.54.1 | 1.54.1（10/21） | 使用既有測試 runtime |
| UUID | 1.29／1.29.0 | 1.29.0（10/23） | Session 格式不改 |

官方：[Rust](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) · [Axum](https://docs.rs/crate/axum/latest) · [Leptos](https://docs.rs/crate/leptos/latest) · [SQLx](https://docs.rs/crate/sqlx/latest) · [Tokio](https://docs.rs/crate/tokio/latest) · [UUID](https://docs.rs/crate/uuid/latest)。本篇不變更 Cargo.lock；本機安裝版本、完整 native/WASM MSRV 與執行結果未核對，不假稱已升級。

## ④ 驗證／交付狀態

已核對精確來源檔案、現行 caller、依賴與 lock，產生完整 Refactor／Green patch 及新增斷言。**尚未在隔離產品 checkout 實際套用、編譯、跑測試或瀏覽器驗收**；靜態組合不等於 Cargo Green。交付是方案，產品套用／Gate2 仍待使用者驗證；原 Day39 r2、歷史證據、FIFO 與 `last_full_comparison` 均保留。跨案契約、共用 crates 與全庫 CI 僅由 [工程總頁](../../../engineering/README.md) 追蹤，不在 Chat Refactor 重複計功。
