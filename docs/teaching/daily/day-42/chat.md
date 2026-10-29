# Chat Day42｜進房後顯示真實房名與資料讀取失敗狀態

> 2026-10-28；產品唯讀來源 `williechen/rust_ai_ring@0b2288e4996b6d80b00e7a77e2cd023835306528`；文件撰寫時 `next_day.chat=42`。Day41 的匿名選房→登入回原房已見於最新產品 `room_auth.rs`、`chat_ui`，本篇不重複。**僅為待使用者套用的方案；未執行產品 Red／Green／GUI／CI。**

## ① 功能目標、現況、真實呼叫鏈

進入 `/room/{id}` 後，聊天室標題目前顯示不易閱讀的 ID。正式產品已提供 `GET /api/rooms/{id}`，由 `chat_server::http::get_room` 經 `AppState.room_repository.find_by_id` 讀 SQLite，回傳 `{id,category_room_id,name}`；但 `chat_ui::ChatPage` 尚未使用。Day42 將標題改成房間的真實名稱；若資料讀取失敗，顯示明確的「無法取得房間名稱」，不捏造房名。登入、WS、歷史、房主 ACL 不在本篇變更；若 ID 不存在，既有 WS 授權仍須另外驗證，不能宣稱已完成逐房 ACL。

正式路徑：`ChatPage` → `room_api::room_name` → `GET /api/rooms/{id}` → `http::get_room` → `room_repository.find_by_id` → SQLite。只使用同站 fetch，不加入平行服務或額外 DB 查詢路由。

## Refactor｜Chat 房間列表顯示狀態的獨立責任

**修改前**：`chat_ui/src/lib.rs` 同時承擔 `room_status_text` 的純狀態決策與 Leptos 元件組裝，`room_list_state.rs` 已持有 `room_list_feedback`、搜尋結果保留／清除及首次載入規則。**修改後**：把 `room_status_text` 的完整函式搬到 `chat_ui/src/room_list_state.rs`，`lib.rs` 改為 `use room_list_state::{filter_rooms, room_status_text};`；不改公開呼叫介面或 DOM，保持現有 loading、error、empty、filtered、found 行為。

```rust
pub(super) fn room_status_text(status: &str, loaded: bool, total: usize, visible: usize) -> String {
    if matches!(status, "查詢中…" | "載入中…") || status.ends_with("，請重試") {
        return status.to_owned();
    }
    room_list_feedback(loaded, total, visible).to_owned()
}
```

新測試 `day42_status_refactor_tests::loading_error_empty_and_found_contract_survives_move` 逐項驗證五種狀態。原 `room_list_state` 六組搜尋、錯誤保留與首次載入測試、`lib.rs` 既有 SSR／hydrate／Chat Socket 測試全部保留。無新 Cargo dependency、fixture 或 CI workflow；這是 Chat 自身分層，不計入全庫 G-01/G-02。[完整 Refactor patch](chat-day42-refactor.patch) 先於 Green 套用。

下一個 Chat Refactor 工作包應盤點 `room_api` 的重複 fetch/error 轉換與正式呼叫者，避免在本篇擴及其他產品。

## ② Red → 完整 Green（核心預估 15 分鐘）

### Red：先鎖定真實房名與失敗回饋

在 `chat_apps/chat_ui/src/lib.rs` 增加 `day42_room_heading_tests::real_name_and_failed_detail_have_distinct_visible_states`，要求成功時呈現「房間 技術討論」，失敗時顯示「房間 room-1（無法取得房間名稱）」。舊來源不存在 `room_heading`，是預期缺符號編譯 Red；**沒有實際執行 Red**。原 Day16 房間身份、Day39 清單與 Day41 登入回跳測試不刪。

```rust
#[cfg(test)]
mod day42_room_heading_tests {
    use super::room_heading;
    #[test]
    fn real_name_and_failed_detail_have_distinct_visible_states() {
        assert_eq!(room_heading("room-1", Ok("技術討論".into())), "房間 技術討論");
        assert_eq!(room_heading("room-1", Err("404".into())), "房間 room-1（無法取得房間名稱）");
    }
}
```

### Green：真實 HTTP、資料契約與 Leptos 畫面接線

1. `chat_apps/chat_ui/src/room_api.rs`：新增完整 `room_name(id)`，先限制 ID 為既有清單使用的 ASCII 英數／連字號／底線，呼叫同站 `/api/rooms/{id}`，檢查 HTTP 成功、JSON 的 `id` 與請求一致，再回傳 `name`；HTTP、JSON、ID 不一致或缺欄位均回 `Err`。函式與 imports 的完整可套用修改見 Green patch。
2. `chat_apps/chat_ui/src/lib.rs`：新增以下完整轉換單元；`ChatPage` 的初始標題為 `房間 {id}`，WASM hydration 透過 `room_api::room_name` 更新訊號；SSR 不觸發瀏覽器 fetch，既有 `ChatSocket`、訊息與輸入區原封不動。原 `<h2>{format!("房間 {heading}")}</h2>` 改為 `<h2>{move || room_title.get()}</h2>`。

```rust
fn room_heading(id: &str, result: Result<String, String>) -> String {
    match result {
        Ok(name) => format!("房間 {name}"),
        Err(_) => format!("房間 {id}（無法取得房間名稱）"),
    }
}
```

```rust
let (room_title, set_room_title) = signal(format!("房間 {room_id}"));
#[cfg(target_arch = "wasm32")]
{
    let requested = room_id.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let result = room_api::room_name(&requested).await;
        set_room_title.set(room_heading(&requested, result));
    });
}
#[cfg(not(target_arch = "wasm32"))]
let _ = set_room_title;
```

[完整 Green patch](chat-day42-green.patch) 只改上述兩個檔案，包含新測試與所有修改單元。套用順序：先 Refactor，後 Green；回復順序相反。`chat-ui` 原有 `leptos 0.8`、`web-sys`、`wasm-bindgen-futures`、`serde_json` 已宣告；不新增 migration、Cargo feature 或 lock 變更。

## ③ 當日版本／相容性

2026-10-28 官方 Rust stable 為 **1.99.1**（2026-10-15，[Rust release notes](https://doc.rust-lang.org/releases.html)），不是上一篇記錄的 1.99.0。現行 workspace 宣告 `leptos = "0.8"`、`leptos_router = "0.8"`、`axum = "0.8"`、`sqlx = "0.9"`、`tokio = "1.54"`、`uuid = "1.29"`；框架參考 [Leptos](https://docs.rs/crate/leptos/latest)、[Axum](https://docs.rs/crate/axum/latest)、[SQLx](https://docs.rs/crate/sqlx/latest)。精確 `Cargo.lock` resolved／本機 Rust／MSRV／WASM 相容性**未完成本輪逐項核對**，不得視為版本 Gate 已通過；不自動升級。

## ④ 實際驗證、限制、下一步

**已做**：讀回最新產品三個目標來源檔及 workspace manifest、檢查原始替換片段唯一存在，生成 Refactor 與 Green unified diff 並分別從 GitHub 精確提交讀回。**未做**：`git apply --check`、Cargo、SQLite、HTTP、SSR／WASM build、瀏覽器與 CI；上述 Red/Green 僅為待套用測試方案，不是已通過測試。

```bash
# 隔離 checkout；rust_ai_ring 主庫保持唯讀
git checkout 0b2288e4996b6d80b00e7a77e2cd023835306528
git apply --check chat-day42-refactor.patch
git apply chat-day42-refactor.patch
git apply --check chat-day42-green.patch
git apply chat-day42-green.patch
cargo fmt --all --manifest-path chat_apps/Cargo.toml -- --check
cargo test --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --features ssr
cargo check --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --features hydrate --target wasm32-unknown-unknown
```

瀏覽器驗證：有效會員進入真實房間，確認真房名、重載、失敗／404 回饋、切房後標題不被舊請求覆寫；再測 WS／未登入回跳及房間 ACL。**特別注意**：目前非同步查詢尚未引入 request generation 防止切房競態，必須在產品驗收前核實路由生命週期，若存在覆寫即補修，不冒稱已驗收。核心 Red／Green 預估 15 分鐘，Refactor、版本、驗證與提交另計。方案／使用者套用／Gate2 分列，不更新 FIFO 或 `last_full_comparison`。
