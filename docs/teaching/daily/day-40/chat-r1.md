# Chat Day40 r1｜首頁依有效 Session 顯示「登入／管理」

> 2026-10-26修訂。最新產品SHA `ff14ca6224f7c5a65c799bf4a82a743fa79c880c`；原教學與patch基底 `bb259aaf76542554a3a704d9c4f4a3a219d65cbf`。本r1更正套用問題並保存實際本地命令，原[Day40文章](chat.md)與patch原bytes保留。Day41、FIFO與last_full_comparison不變。

## 最新核對與閱讀方式

**最新程式有路由回歸：/api/session已接，但誤刪/account，舊登入測試得到404而非401。** HomePage雖顯示管理連結，既有管理handler已沒有註冊；不能標成功。

下面①／Refactor／②保留「從bb259aa重現原始缺口」的完整TDD教學；它們描述教學前狀態，不表示最新來源仍全部未實作。當次Red未重新執行，預期Red不能冒充實際失敗證據。實測結果與當前未完成項集中於④。[Day39後續勘誤](../day-39/chat-r3.md)及[歷史更正總表](historical-corrections.md)保留各SHA範圍。

## ① bb259aa 教學基底的目標與缺口｜ 目標、現況與完成條件

未登入、Session 過期或帳號停用時，首頁顯示「登入」並前往 `/signin`；有效登入才顯示「管理」並前往既有 `/account`。首頁公開房間列表仍可匿名讀取。檢查使用既有 SQLite `mbr_session` 與 `mbr_user`，不把 cookie 存到 WASM 或把前台判斷當伺服器授權。載入中／無法確認狀態有明確文案；SSR 初始採保守登入入口，hydrate 後更新，SSR 不宣稱已確認身份。房間 ACL、選房後保留目標仍未接通，留待後續。

精確呼叫鏈：`HomePage` → `room_api::session_status` → `GET /api/session` → `signin::session_status` → `auth_session::active_member` → Chat SQLite。新端點只回 204／401，不回 token、會員資料；原 `/account` 仍自行驗 Session。核心60分鐘只計② Red／Green，本段、Refactor、版本、驗證、提交另計。

## Refactor

**獨立既有架構工作：移除 Chat 登入 adapter 重複雜湊實作**，不依賴 Day40 Session API。原 `chat_server/src/signin.rs` 自行以 SHA256 手寫 `token_digest`，而 `auth_session.rs` 與 `logout.rs` 已使用 `shared::auth::token_digest`。兩種實作分散使登入寫入的 hash 與後續讀取／撤銷的 hash 可能漂移。改為相同函式，保持原 SHA256 bytes／hex 行為與既有 DB token_hash 欄位，不改 cookie 格式、登入時效、Schema 或跨案 crate。

完整修改：`chat_apps/chat_server/src/signin.rs` 將 `use sha2::{Digest, Sha256};` 改為 `use shared::auth::token_digest;`，**刪除**原本的 `fn token_digest(token: &str) -> String`，既有 `let hashed = token_digest(&token);` 不變。這是登入路徑的單一責任整合，不是② Green 後的修飾；完整逐行可套用內容見 [Chat Refactor patch](chat-day40-refactor.patch)。

原 Day23 密碼登入→寫入 `mbr_session.token_hash`、Day25/27 Session、Day37/39 SSR 登入斷言保留；Day40 新 endpoint 只在②加入，不能算重構的測試。以既有 `signin::day23_tests`、`auth_session::tests`、`logout` 回歸確認登入 hash／登出 hash 一致。此 Refactor 不修改 Cargo、lock、migrations、module 或其他產品。**原版交付時未實際執行 Cargo；本r1實測詳見④**；套用後先獨立測試本包再套 Green。下一個 Chat Refactor 應沿登入／Session 與房間 ACL 的真 caller 整理，勿把 Analytics/Pet 的 CI 放入本段。

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

完整可套用 [Chat Green patch](chat-day40-green-r1.patch) 修改三個真實檔案，依序：

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
git apply --check /path/to/chat-day40-green-r1.patch
git apply /path/to/chat-day40-green-r1.patch
cd chat_apps
cargo fmt --all -- --check
cargo test --locked -p chat-server --lib day40_session_entry_tests
cargo test --locked -p chat-ui --features ssr day40_home_entry_tests
cargo test --locked -p chat-server --features ssr
cargo check --locked -p chat-ui --target wasm32-unknown-unknown --features hydrate
cargo check --locked -p chat-server --features ssr
```

瀏覽器需測匿名公開列表、登入後管理、登出後登入、停用／過期 Session、網路失敗保守入口及 reload；真房間進入 ACL／選房目標保留是下一篇缺口。若不相容，以原始 `git apply -R` 分別回復，避免變更 DB。


### 最新程式路由回歸與完整修改單元核對

最新routes把既有 `.route("/account", get(account))` 換掉而非並存，完整workspace測試exit101。教學候選保留舊路由而通過；[最新SHA最小修復patch](chat-day40-followup-ff14ca6.patch)只補回既有route，handler、授權及舊斷言不改。補接方案另外隔離驗證，原始最新碼的失敗仍保留。

原始Green在三個檔案尾端預期多一個空白行，標準git apply失敗。修訂patch以精確原檔重生，沒有以ignore-whitespace或強行套用掩蓋原始錯誤；正確順序仍為原Refactor→修訂Green。

來源：[`chat_apps/chat_server/src/signin.rs::session_status`](https://github.com/williechen/rust_ai_ring/blob/ff14ca6224f7c5a65c799bf4a82a743fa79c880c/chat_apps/chat_server/src/signin.rs#L29)。

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

來源：[`chat_apps/chat_ui/src/room_api.rs::session_status`](https://github.com/williechen/rust_ai_ring/blob/ff14ca6224f7c5a65c799bf4a82a743fa79c880c/chat_apps/chat_ui/src/room_api.rs#L96)。

```rust
pub async fn session_status() -> Result<bool, String> {
    let window = web_sys::window().ok_or("無法取得瀏覽器")?;
    let value = JsFuture::from(window.fetch_with_str("/api/session"))
        .await
        .map_err(|_| "無法確認登入狀態")?;
    let response: web_sys::Response = value.dyn_into().map_err(|_| "無效 Session 回應")?;
    match response.status() {
        204 => Ok(true),
        401 => Ok(false),
        code => Err(format!("Session 查詢失敗：HTTP {code}")),
    }
}
```

來源：[`chat_apps/chat_ui/src/lib.rs::session_entry_display`](https://github.com/williechen/rust_ai_ring/blob/ff14ca6224f7c5a65c799bf4a82a743fa79c880c/chat_apps/chat_ui/src/lib.rs#L125)。

```rust
fn session_entry_display(state: SessionEntry) -> (&'static str, &'static str, &'static str) {
    match state {
        SessionEntry::Checking => ("/signin", "登入", "確認登入狀態中"),
        SessionEntry::SignedOut => ("/signin", "登入", ""),
        SessionEntry::SignedIn => ("/account", "管理", ""),
        SessionEntry::Unavailable => ("/signin", "登入", "無法確認登入狀態；可使用登入入口"),
    }
}
```

來源：[`chat_apps/chat_ui/src/lib.rs::HomePage`](https://github.com/williechen/rust_ai_ring/blob/ff14ca6224f7c5a65c799bf4a82a743fa79c880c/chat_apps/chat_ui/src/lib.rs#L134)。

```rust
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

Refactor正式接線為 `use shared::auth::token_digest;`，舊signin本地hash函式已移除；登入寫入仍使用同名函式。當次文件patch不改共享crate與其行為。

## ③ 當日版本與實機核對

Rust／Cargo實測1.99.1，WASM target已安裝；PostgreSQL實測18.6，GTK／WebKit依賴實測見[工具log](local-validation/commands.jsonl)。以下為2026-10-26重新核對的官方穩定頁與當次來源lock；不升級依賴或改Cargo.lock。

| 技術 | 官方核對 |
| --- | --- |
| Rust | [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) |
| Axum | [0.8.9](https://docs.rs/crate/axum/latest) |
| Leptos | [0.8.22](https://docs.rs/crate/leptos/latest) |
| SQLx | [0.9.0](https://docs.rs/crate/sqlx/latest) |
| Tokio | [1.54.1](https://docs.rs/crate/tokio/latest) |
| UUID | [1.29.0](https://docs.rs/crate/uuid/latest) |

原版的「本機未讀」已由本次工具實測補正；未做release產物、完整MSRV矩陣或DB升級，不能由穩定版metadata推定它們通過。

## ④ 真正本地驗證與未完成

本次結果分為下列來源；**最新碼失敗與未接通項不由候選成功抵消**。

| 範圍／命令 | exit | 結果／log |
| --- | --- | --- |
| candidate-chat-fmt：`cargo fmt --all -- --check` | 1 |  [log](local-validation/day40-logs/candidate-chat-fmt.log) |
| candidate-chat-check：`cargo check --offline --locked -p chat-server --features ssr` | 0 |  [log](local-validation/day40-logs/candidate-chat-check.log) |
| candidate-chat-test：`cargo test --offline --locked --workspace --features ssr` | 0 | 6passed/0failed/0ignored；0passed/0failed/0ignored；5passed/0failed/0ignored；25passed/0failed/0ignored；0passed/0failed/0ignored；2passed/0failed/0ignored；1passed/0failed/0ignored；4passed/0failed/0ignored；16passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored [log](local-validation/day40-logs/candidate-chat-test.log) |
| candidate-chat-wasm：`cargo check --offline --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown` | 0 |  [log](local-validation/day40-logs/candidate-chat-wasm.log) |
| latest-chat-fmt：`cargo fmt --all -- --check` | 0 |  [log](local-validation/day40-logs/latest-chat-fmt.log) |
| latest-chat-check：`cargo check --offline --locked -p chat-server --features ssr` | 0 |  [log](local-validation/day40-logs/latest-chat-check.log) |
| latest-chat-test：`cargo test --offline --locked --workspace --features ssr` | 101 | 6passed/0failed/0ignored；0passed/0failed/0ignored；5passed/0failed/0ignored；24passed/1failed/0ignored [log](local-validation/day40-logs/latest-chat-test.log) |
| latest-chat-wasm：`cargo check --offline --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown` | 0 |  [log](local-validation/day40-logs/latest-chat-wasm.log) |
| latest-chat-complete：`cargo test --offline --locked --workspace --features ssr --no-fail-fast` | 101 | 6passed/0failed/0ignored；0passed/0failed/0ignored；5passed/0failed/0ignored；24passed/1failed/0ignored；0passed/0failed/0ignored；2passed/0failed/0ignored；0passed/1failed/0ignored；4passed/0failed/0ignored；16passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored [log](local-validation/day40-logs/latest-chat-complete.log) |
| followup-chat-fmt：`cargo fmt --all -- --check` | 0 |  [log](local-validation/day40-logs/followup-chat-fmt.log) |
| followup-chat-check：`cargo check --offline --locked -p chat-server --features ssr` | 0 |  [log](local-validation/day40-logs/followup-chat-check.log) |
| followup-chat-test：`cargo test --offline --locked --workspace --features ssr` | 0 | 6passed/0failed/0ignored；0passed/0failed/0ignored；5passed/0failed/0ignored；25passed/0failed/0ignored；0passed/0failed/0ignored；2passed/0failed/0ignored；1passed/0failed/0ignored；4passed/0failed/0ignored；16passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored [log](local-validation/day40-logs/followup-chat-test.log) |

每條命令的精確cwd、時間、環境覆寫與完整原始錯誤見[JSONL](local-validation/commands.jsonl)；[兩組範圍及結果](local-validation/README.md)。Cargo使用--offline --locked，沒有改lock或升級依賴。

最新/account路由回歸使整體測試失敗；修復候選結果另列，不宣稱產品已修復。真瀏覽器登入／管理切換、失效Session、loading與網路失敗、房間ACL仍未驗收。

GitHub文件提交與原始程式套用分開；本輪僅更新產品checkout至遠端SHA，驗證皆在隔離worktree，沒有提交／推送產品程式。原始文章與patch保留，靜態hunk檢查不追認為當時編譯成功。
