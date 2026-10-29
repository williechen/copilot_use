# Chat Day37｜房間搜尋成功後結束載入狀態，保留失敗重試

> 交付日：2026-10-25。程式來源：`williechen/rust_ai_ring/main@5c713553daee4055738239f047d6c774fcc7a7a1`（唯讀）；文件：`williechen/copilot_use/main`。起始 `next_day.chat=37`。本篇為**待使用者套用、待測試、未驗收**的完整最小 TDD 切片，不宣稱新 Green 已在產品庫執行。

## ① 學習大綱：真實 UI 查詢完成卻仍顯示「查詢中」

承接 [Day36 r2：WebSocket Session 撤銷](../day-36/chat-r2.md)：該篇功能已存在於來源 main 並有局部實測，本篇不重教、不覆寫。現行 `chat_apps/chat_ui/src/lib.rs::RoomList` 的 `query_room` 將 `loading=true` 且 `status="查詢中…"`，在 WASM 分支呼叫 `room_api::search_rooms` 後，成功路徑只更新 `rows` 和 `has_snapshot`，沒清掉 status；錯誤路徑才改 status，兩者最後都清 loading。畫面中同時使用 status、filter feedback，成功後可能殘留不真實的進度文字。

**本日一個功能切片**：成功 HTTP 查詢更新房間清單、載入旗標及符合實際資料的狀態；失敗保存上次成功清單與搜尋字串，顯示可重試錯誤；避免引入第二個 API、移動不存在的 crate 或改變登入／選房契約。真鏈為 Leptos `RoomList` → WASM `room_api::search_rooms` → `GET /api/rooms?q=…` → `chat_server` 房間 HTTP route → 現有 SQLite adapter。這項修正不等於逐房 ACL 已完成。

## ② Red → 完整最小 Green → Refactor

### Red：新增狀態轉換測試

在 `chat_apps/chat_ui/src/lib.rs` 的 `#[cfg(test)] mod day37_search_tests` 新增；先只加以下測試，預期編譯失敗 `cannot find function finish_room_query`，而不是聲稱既有測試失敗。

```rust
#[cfg(test)]
mod day37_search_tests {
    use super::finish_room_query;

    #[test]
    fn success_clears_loading_message_and_accepts_actual_rows() {
        let result = finish_room_query(
            vec![("old".into(), "原房間".into())],
            "貓".into(),
            Ok(vec![("new".into(), "貓房".into())]),
        );
        assert_eq!(result.0, vec![("new".into(), "貓房".into())]);
        assert!(result.1.is_empty()); // 成功才清除舊的查詢條件錯誤
        assert_eq!(result.2, "請選擇房間");
    }

    #[test]
    fn failure_keeps_last_snapshot_and_retry_context() {
        let old = vec![("old".into(), "原房間".into())];
        let result = finish_room_query(old.clone(), "貓".into(), Err("離線".into()));
        assert_eq!(result, (old, "貓".into(), "離線，請重試".into()));
    }

    #[test]
    fn empty_success_has_explicit_empty_feedback() {
        let result = finish_room_query(vec![], "貓".into(), Ok(vec![]));
        assert_eq!(result.2, "尚無房間");
    }
}
```

### Green：復用現有語義，接入實際 WASM caller

本檔已存在 `refresh_result_view(previous, previous_query, result)` 且有測試／真 caller；**不新建重複 helper**。因此上述 Red 中的名稱 `finish_room_query` 應直接改用既有的 `refresh_result_view`，測試由 missing-symbol Red 演進為對實際函式的回歸測試。三個斷言均使用該現有函式，不必改動 `refresh_result_view` 的本體或 Cargo 設定。真正缺口在 `RoomList::query_room` 的成功 caller；以以下程式**完整取代**該 closure 中 `#[cfg(target_arch = "wasm32")]` 的 `wasm_bindgen_futures::spawn_local(async move { ... });` 區塊：

```rust
#[cfg(target_arch = "wasm32")]
wasm_bindgen_futures::spawn_local(async move {
    let submitted = query.get_untracked();
    let previous = rows.get_untracked();
    let result = room_api::search_rooms(&submitted).await;
    let success = result.is_ok();
    let (next_rows, _retry_query, next_status) =
        refresh_result_view(previous, submitted, result);
    if success {
        set_has_snapshot.set(true);
    }
    set_rows.set(next_rows);
    set_status.set(next_status);
    set_loading.set(false);
});
```

注意：初始 `set_loading.set(true)`、`set_status.set("查詢中…")`、重複請求守衛及非 WASM 分支保留；成功顯示「尚無房間／請選擇房間」，失敗顯示錯誤並保存上次 `rows`，不以錯誤結果覆蓋已知清單。實際 HTTP 函式 `chat_apps/chat_ui/src/room_api.rs::search_rooms` 不改；它仍經瀏覽器 fetch 並解析原有房間 ID/name JSON。

**測試最終完整程式**：將上述三個測試的 `use super::finish_room_query;` 替換為 `use super::refresh_result_view;`，所有 `finish_room_query(` 呼叫替換為 `refresh_result_view(`，其餘測試碼不動。這避免為測試創造第二套實作。若想一次貼上，可直接依這兩處精確替換貼入前述完整區塊。

### Refactor、過往修正與接線

沿用原本 `refresh_result_view`，不刪除它的既有測試或 `refreshed_room_list`，後者需依其他 caller 核實後才可清理。初始 UI 顯示與結果提示存在兩個概念；本篇只修復查詢中的 status 殘留，不將兩段狀態系統全面合併當成已完成。後續須補完成真正可存取的單一狀態模型與真實查詢/重試瀏覽器測試。Day36 r2 的完整驗收缺口（原子撤銷競態、房間 ACL、歷史、瀏覽器）原樣保留，不當作本次測試通過。

依 `chat_apps/Cargo.toml` 與 `chat_apps/chat_ui/Cargo.toml`：Cargo package `chat-ui`、Rust crate `chat_ui`、imports `leptos::prelude::*` 已存在；WASM `wasm-bindgen-futures` 與 `web-sys`、`leptos`/router 依賴及 `ssr`/`hydrate` feature 均不新增或變更。`[[workspace.metadata.leptos]]` 的 `bin-package="chat-server"`、`lib-package="chat-ui"`、資源名稱 `rust_ai_chat` 亦**原樣保留**（後者不是現行 repository 名稱）。`chat_apps/Cargo.lock` 無需修改。其它 Pet/Analytics/crates workspace 無修改。

## ③ 套件與軟體版本（2026-10-25 當日核對）

| 項目 | 來源宣告／lock | 當日官方最新穩定 | 判斷 |
| --- | --- | --- | --- |
| Rust | Edition 2024；本機 `rustc` 未讀 | 1.99.1（[Rust 官方 releases](https://blog.rust-lang.org/releases/)） | 只建議在實際環境核對 toolchain，不自動升級 |
| Leptos | `0.8`／`0.8.22` | [crates.io leptos](https://crates.io/crates/leptos) 當日精確最新版本仍需獨立查讀 | 現行 lock 可重現，本篇不升級 |
| leptos_router | `0.8`／`0.8.17` | [crates.io leptos_router](https://crates.io/crates/leptos_router) 當日精確最新版本仍需獨立查讀 | 與既有 0.8 Leptos 相容性須以實跑為準 |
| wasm-bindgen | `0.2`／`0.2.129` | [crates.io wasm-bindgen](https://crates.io/crates/wasm-bindgen) 當日精確最新版本仍需獨立查讀 | 保留 lock；WASM 實際驗證必要 |
| wasm-bindgen-futures / web-sys | `0.4` / `0.3`，lock 解決版本待完整核對 | [futures](https://crates.io/crates/wasm-bindgen-futures)、[web-sys](https://crates.io/crates/web-sys) 尚未完成精確當日版本核對 | 不宣稱 Gate 1 版本完整通過 |

官方 crate 的當日最新版仍有未核實項，**文件 Gate 1 尚未完成**；本稿不得作為已達交付 Gate 的文章。Rust／WASM MSRV 與 native target 兼容性未實測。

## ④ 驗證項目及交付界線

在產品工作樹由使用者套用後執行（repo 根目錄沒有 Cargo.toml）：

```bash
cd /home/willie/Documents/rust_ai_ring/chat_apps
cargo test -p chat-ui --features ssr day37_search_tests
cargo test -p chat-ui --features ssr
cargo check -p chat-ui --target wasm32-unknown-unknown --features hydrate
cargo check -p chat-server --features ssr
```

**預期**：上述 3 項新測試成功；WASM 及 SSR 編譯仍通過，瀏覽器真搜尋成功時不殘留查詢中、失敗仍能重試。**實際**：本次未套用 rust_ai_ring，以上命令皆未執行；新 Red/Green 均為教學預期，非現有測試證據。CI、真 HTTP、瀏覽器及功能驗收未執行。服務端 ACL、Cookie、歷史及回饋狀態合併不屬本篇已完成事項。

來源 `5c713553daee4055738239f047d6c774fcc7a7a1`；規則 `docs/COMMON_LEARNING_RULES.md`；交付僅限 `copilot_use` 文件，產品唯讀。
