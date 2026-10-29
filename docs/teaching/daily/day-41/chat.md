# Chat Day41｜公開房間選擇後登入並返回原房間

> 2026-10-27；唯讀產品來源 `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`；文件起始 parent `d5fa50b6a5978977bc7306098969845ce855bf9e`。next_day=41；[上一篇 Day40 r2](../day-40/chat-r2.md) 已處理有效 Session 首頁入口，不重教。**方案已撰寫，產品尚未套用或驗收。**

## ① 功能目標／現況／前置

公開房間列表可匿名讀取；目前 `GET /room/{room_id}` 未登入回 401，登入成功一律前往 `/account`，選房意圖遺失。本篇使匿名點真 UUID 房間 → 303 同站 `/signin?next=/room/{uuid}` → Leptos 表單隱藏 next → 驗證帳密與 SQLite Session → 303 原房間。未指定或惡意 next 一律進 `/account`；WS／寫入 API 仍維持 401，不轉址。看得到清單不代表逐房 ACL；房主／授權尚未完成。

正式呼叫鏈：`chat_ui::RoomList` → `chat_server::room_auth::require_room_member` → `chat_ui::SignInPage` SSR/hydrate → `chat_server::signin::login` → SQLite Session → `ssr::app_handler`。註冊完成清除選房目標、個人管理與既有會員政策不變。

## Refactor｜Chat 自身 Session Cookie 讀取責任（獨立於今日功能）

**修改前**：`chat_apps/chat_server/src/auth_session.rs` 的 `active_member`、`csrf_for_logout`、`csrf_for_create_room` 重複解析相同 Cookie，未來錯誤策略易偏離。**修改後**：在同檔新增下列完整 helper，三個原 caller 的 `let raw = ...` 各換成 `let raw = session_cookie(headers)?;`，保留所有後續 SQLite SQL、token digest、CSRF domain separator、停用與逾期邏輯。

```rust
fn session_cookie(headers: &HeaderMap) -> Result<String, StatusCode> {
    headers.get(header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| cookie_token(value, "chat_session"))
        .ok_or(StatusCode::UNAUTHORIZED)
}
```

**測試承接**：保留 `session_adapter_obeys_revocation_and_account_state` 全部有效斷言；新增 `cookie_parser_and_both_csrf_callers_keep_the_same_contract`，涵蓋無 Cookie、非法 token、合法 token、logout／create-room CSRF 的精確 digest。原 imports 仍使用，沒有新增依賴／空 crate／其他專案。完整可套用 [Chat Refactor patch](chat-day41-refactor.patch)，含 helper、三個 caller 及新增測試全文。下一步核對 Chat HTTP/WS ACL caller 與 fixture；本項不計入跨案工程 G-01/G-02。

## ② Red → 完整最小 Green

### Red｜可觀測行為、舊斷言不刪

`chat_apps/chat_server/src/room_auth.rs` 新增 `day41_room_return_tests::anonymous_room_link_redirects_but_invalid_room_id_is_rejected`：真 UUID 匿名房間 GET 必須 303 且 Location 精確 `/signin?next=/room/{uuid}`；非法 UUID 401。舊 `day25_tests` 仍測 `require_member` 的 401/有效 Session。`signin.rs` 在原 Day23 登入 SQLite Router 測試後加上真帳密帶 next 返回原房間，以及外站 next 只能返回 `/account`；保留原錯誤帳密 401、預設 `/account`、Cookie 與 Session 落庫斷言。Red 在舊程式下因缺符號／固定導向而失敗；**本次沒有實際執行 Red**，環境失敗不算行為 Red。

### Green｜完整檔案與接線

[Chat Green patch](chat-day41-green.patch) 含下列四個來源檔的完整 unified diff，必須在 Refactor patch 後套用：

1. `chat_apps/chat_server/src/room_auth.rs`：`response` import 加 `IntoResponse, Redirect`；新增 `require_room_member`，有效 Session 走 `next.run`，401 時只允許路徑 `/room/{UUID}` 導回本機登入，DB 503 原樣傳回。核心完整函式：

```rust
pub async fn require_room_member(
    State(pool): State<SqlitePool>, request: Request, next: Next
) -> Result<Response, StatusCode> {
    match active_member(&pool, request.headers()).await {
        Ok(_) => Ok(next.run(request).await),
        Err(StatusCode::UNAUTHORIZED) => {
            let id = request.uri().path().strip_prefix("/room/")
                .and_then(|value| uuid::Uuid::parse_str(value).ok())
                .ok_or(StatusCode::UNAUTHORIZED)?;
            Ok(Redirect::to(&format!("/signin?next=/room/{id}")).into_response())
        }
        Err(error) => Err(error),
    }
}
```

2. `chat_apps/chat_server/src/lib.rs`：**只**把 `guarded_room` middleware 指向 `room_auth::require_room_member`；`guarded_ws`、`/api/rooms`、SSr Router、State 不動。
3. `chat_apps/chat_server/src/signin.rs`：`Credentials` 新增 `#[serde(default)] next: String`；建立 Session 後 `LOCATION` 改成 `safe_room_return(&input.next).unwrap_or("/account")`，完整驗證單元：

```rust
fn safe_room_return(next: &str) -> Option<&str> {
    let id = next.strip_prefix("/room/")?;
    let parsed = uuid::Uuid::parse_str(id).ok()?;
    (parsed.to_string() == id).then_some(next)
}
```

4. `chat_apps/chat_ui/src/lib.rs`：Router hooks 引入 `use_query_map`；`SignInPage` 加 `let query = use_query_map(); let next = move || query.read().get("next").unwrap_or_default();`，既有 POST 表單內新增 `<input type="hidden" name="next" value=next />`。保持 SSR/hydrate，不建立另一套 HTML/JS 登入頁。

Cargo/feature/import：原 `axum 0.8`、`leptos 0.8`、`leptos_router 0.8`、`uuid 1.29`、`sqlx 0.9` 已宣告；僅 Rust import 修改如上，無新 crate、migration、lock 變動。舊房間自動載入、搜尋、會員、CSRF、WS、註冊與回歸測試全保留。回復先逆 Green，再逆 Refactor。

## ③ 版本核對（2026-10-27）

Rust stable **1.99.0**（2026-10-01；https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/）；axum **0.8.9**（https://docs.rs/crate/axum/latest）；Leptos **0.8.22**（https://docs.rs/crate/leptos/latest）；leptos_router **0.8.17**（https://docs.rs/leptos_router/latest）；SQLx **0.9.0**（https://docs.rs/crate/sqlx/latest）；Tokio **1.54.1**（https://docs.rs/crate/tokio/1.54.1）；Argon2 **0.6.0**（https://docs.rs/crate/argon2/latest）；UUID **1.29.0**（https://docs.rs/crate/uuid/latest）。四 workspace 的 Cargo.lock 已精確讀取；Chat resolved 為上述版本。未執行本機版本/MSRV/WASM 相容驗證，不自動升級。

## ④ 實際驗證、未完成與下一步

**已實際執行**：精確來源 SHA 與五檔內容讀取、文字修改 preimage 唯一比對、unified patch 產生；尚未執行 `git apply`、Cargo、SQLite Router、瀏覽器或產品 CI。

```bash
# cwd: rust_ai_ring 根；隔離副本，checkout 9c767978b20bd56e1c2182dac2f37e6ae9b7d988
git apply --check /path/to/chat-day41-refactor.patch
git apply /path/to/chat-day41-refactor.patch
git apply --check /path/to/chat-day41-green.patch
git apply /path/to/chat-day41-green.patch
cd chat_apps
cargo fmt --all -- --check
cargo check --locked -p chat-server --features ssr
cargo test --locked -p chat-server --features ssr
cargo test --locked -p chat-ui --features ssr
cargo check --locked --target wasm32-unknown-unknown -p chat-ui --features hydrate
```

瀏覽器需驗匿名首頁→房間→登入→原房間、無 next→管理、惡意 next、過期/停用 Session、WS/API 仍 401；逐房 ACL/房主/歷史、實際 GUI/CI 尚未驗收。方案／套用／Gate2 分列。核心 Red/Green 15 分鐘；Refactor、版本、驗證、提交另計。不增加額外 Day、不動 FIFO 與 `last_full_comparison`。
