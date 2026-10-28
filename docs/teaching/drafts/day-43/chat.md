# Chat Day43 候選｜房間搜尋一鍵清除並恢復已載入清單

> 2026-10-29；唯讀產品來源 [rust_ai_ring@c6708c4](https://github.com/williechen/rust_ai_ring/tree/c6708c42f21ac919e57a379848b4977d27a1c3d6)；文件來源起點 copilot_use/main `d6a701eb8de9eaeb04d4145e436ce75485bb967e`；`next_day.chat=42`。**這是新 Day43 候選草稿，不是已發布 Gate1，也不是產品已套用。**

## ① 目標／現況／前置

Day42 已將房名與錯誤狀態接入正式 Chat UI。現有 `RoomList` 可搜尋、讀取並在失敗時保留上次成功清單，但使用者輸入搜尋詞後，缺少不發送新 HTTP 請求即可回到完整快照的明確操作。新增「清除搜尋」按鈕：載入完成後清空查詢詞、恢復全部已載入房間；載入中禁用，不更動 rows、Session、HTTP/WS/SQLite 或登入 ACL。清除前後均沿用 `room_list_state::filter_rooms` 與 `room_list_feedback`，沒有第二份清單。原始需求仍以公開房間→登入→授權進房為優先；此切片不宣稱完成逐房 ACL。

## Refactor｜Chat Session 入口狀態搬層（與功能 Green 獨立）

修改前 `chat_ui/src/lib.rs` 同時持有 `SessionEntry` 四狀態與 `session_entry_display` 的純導覽決策，`HomePage` 則負責 Leptos signals 和 `room_api::session_status`。修改後將原完整決策搬至新 `chat_ui/src/session_entry.rs`，在 lib.rs 宣告 `mod session_entry;` 並 `use session_entry::{session_entry_display, SessionEntry};`。原 Day40 `entry_does_not_claim_manage_without_confirmed_session` 保留，新增測試確認 Checking、SignedOut、Unavailable 均不冒稱管理，SignedIn 才顯示管理。沒有新增 crate、feature、workflow；不碰 Pet、Analytics 或全庫工程項目。

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum SessionEntry { Checking, SignedOut, SignedIn, Unavailable }

pub(super) fn session_entry_display(state: SessionEntry) -> (&'static str, &'static str, &'static str) {
    match state {
        SessionEntry::Checking => ("/signin", "登入", "確認登入狀態中"),
        SessionEntry::SignedOut => ("/signin", "登入", ""),
        SessionEntry::SignedIn => ("/account", "管理", ""),
        SessionEntry::Unavailable => ("/signin", "登入", "無法確認登入狀態；可使用登入入口"),
    }
}
```

完整 module、呼叫端、原斷言承接與新增測試見 [Refactor patch](chat-day43-refactor.patch)。先套 Refactor；回復時在 Green 之後逆序撤回。下一個 Chat Refactor 應盤點房間 API 重複的 fetch/error 轉換，保留現有 HTTP 契約與真呼叫者。

## ② Red → 完整 Green（核心預估 15 分鐘）

**Red：** 新增 `room_list_state::day43_clear_room_search_tests`：兩個已載入房間、搜尋 Rust 僅一筆；清除後查詢為空且兩筆仍在；未載入時仍提示「請先讀取房間列表」。另在現有 SSR Router fixture 的 `signin_contract.rs` 新增 `day43_room_list_has_clear_search_control`，要求真實 `RoomList` 有「清除搜尋」按鈕。原 main 缺 `clear_room_search` 與按鈕，屬缺符號／可見行為 Red；**未實際執行 Red**。Day37/38/39/40/42 的搜尋、錯誤、Session、房名斷言全部保留。

**Green：** `room_list_state.rs` 增加完整純決策單元：

```rust
pub(super) fn clear_room_search(loaded: bool, rows: &[(String, String)]) -> (String, String) {
    (String::new(), room_list_feedback(loaded, rows.len(), rows.len()).to_owned())
}
```

`chat_ui/src/lib.rs::RoomList` 在兩個既有查詢按鈕後新增以下真 Leptos 操作；沿用 `rows`、`query`、`has_snapshot`、`loading`、`status` signals：

```rust
<button
    type="button"
    disabled=move || loading.get() || query.get().is_empty()
    on:click=move |_| {
        let (cleared, feedback) =
            clear_room_search(has_snapshot.get_untracked(), &rows.get_untracked());
        set_query.set(cleared);
        set_status.set(feedback);
    }
>"清除搜尋"</button>
```

完整 [Green patch](chat-day43-green.patch) 包含 module import、按鈕、純函式及兩組新測試；不修改 Cargo.toml、Cargo.lock、migration 或服務端。既有查詢錯誤時保留快照的斷言不刪；清除只重置本地顯示，不代表重新成功查詢或新增授權。

## ③ 版本與相容性（2026-10-29）

官方 Rust stable [1.99.1（2026-10-15）](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)，Leptos [0.8.22](https://docs.rs/crate/leptos/latest)、Leptos Router [0.8.17](https://docs.rs/crate/leptos_router/latest)、Axum [0.8.9](https://docs.rs/crate/axum/latest)、SQLx [0.9.0](https://docs.rs/crate/sqlx/latest)、Tokio [1.54.1](https://docs.rs/crate/tokio/latest)、UUID [1.29.0](https://docs.rs/crate/uuid/latest)。來源四 workspace version `0.2.42`；Chat lock resolved 見 [Day42 最新 lock](../../daily/day-42/versions-2026-10-29.md)。本輪沒有讀回實際 rustc、全遞迴 MSRV 或 SSR/WASM 編譯；版本相容 Gate 未完成，不自動升級。

## ④ 驗證／套用／驗收與下一步

已做：GitHub 精確產品來源、正式 UI caller、SSR fixture、規則及前篇讀回；patch 原始片段唯一比對與逐行 hunk 建構。**未做**：真實 `git apply --check`、Red、`cargo fmt/check/test`、SSR/WASM、瀏覽器、HTTP/SQLite/CI；不得宣稱 Green 通過。隔離副本於 `c6708c42f21ac919e57a379848b4977d27a1c3d6` 先套 Refactor 再 Green，執行：

```bash
git apply --check chat-day43-refactor.patch && git apply chat-day43-refactor.patch
git apply --check chat-day43-green.patch && git apply chat-day43-green.patch
cargo fmt --all --manifest-path chat_apps/Cargo.toml -- --check
cargo test --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --features ssr
cargo check --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --features hydrate --target wasm32-unknown-unknown
```

人工驗收：首頁首次自動載入→輸入 Rust 看到部分房間→按清除恢復原清單→重試錯誤後保留上次快照→載入中按鈕禁用→匿名點房仍需登入與 ACL。核心 15 分鐘只計 Red/Green；Refactor、版本、補教、提交、驗收另計。**候選方案已寫入文件／產品未套用／Gate1、Gate2 均未通過**；不更新 `next_day`、FIFO 或 `last_full_comparison`。
