Here is the result of "fetch" for the Page with URL https://app.notion.com/p/3fde51d4509381d2aac7ea4b356c213b as of 2026-10-18T22:14:02.744Z:
<page url="https://app.notion.com/p/3fde51d4509381d2aac7ea4b356c213b">
<ancestor-path></ancestor-path>
<properties>
{"title":"2026-10-19 Chat Day19 教學暫存"}
</properties>
<iconMetadata>null</iconMetadata>
<content>
## 學習大綱
待補全文。
## 2. 實作內容
請求失敗時保留上次的房間列表。
## 1. 學習大綱（完整版本）
**目標**：Chat 重新整理 GET /api/rooms 失敗時，保留上次成功載入的房間連結與 ID；成功時才替換列表。Red 5 分鐘、Green 10 分鐘＝主體 15 分鐘；Refactor 5 分鐘、瀏覽器驗證 10 分鐘另計。
**來源**：rust_ai_chat/main `f692c2ebe7b414fe49858c1ab1c4a33ce7c3dfee`；copilot_use 教學分支 `1144668617559449ebe7703fd3a3a90927827c66`；規則 `daily-tdd-minimum-v1`（blob `d33e42581be48d495ff7a2e32ac97821ab08df6e`）。本篇是**未套用／未實測**的可套用 Green 教學。
**實際呼叫鏈**：`crates/ui/src/lib.rs::RoomList` → `room_api::list_rooms()` → `room_api::search_rooms("")` → fetch GET /api/rooms?q= → `apps/server/src/lib.rs` Axum route → `apps/server/src/http.rs::list_rooms` → RoomRepository／SQLite → JSON(id,name,category_room_id) → Leptos rows → 房間 ID 連結。現有重新整理按鈕在 fetch 前執行 `set_rows.set(Vec::new())`，遇 503 即使舊結果有效仍消失；查詢按鈕已有保留舊結果行為。本日只修重新整理，不改 SQL、ID 契約或搜尋 API。
## 2. 實作內容（Red → Green → Refactor）
**Red**：在 `crates/ui/src/lib.rs` 既有 `day16_room_search_tests` 模組加入測試，先預期缺少 helper 的編譯錯誤；這是待操作的 Red，**本輪沒有執行測試**。
```rust
#[test]
fn day19_refresh_failure_keeps_previous_room_links() {
    let old = vec![("room_1".into(), "Rust 技術交流".into())];
    let (rows, status) =
        super::refreshed_room_list(old.clone(), Err("503".into()));
    assert_eq!(rows, old);
    assert!(status.contains("請重試"));

    let (rows, status) = super::refreshed_room_list(
        old,
        Ok(vec![("room_2".into(), "生活閒聊".into())]),
    );
    assert_eq!(rows[0].0, "room_2");
    assert_eq!(status, "請選擇房間");
}
```
**Green helper**：在同檔 `RoomList` 前新增純函式，成功替換，失敗保留：
```rust
fn refreshed_room_list(
    previous: Vec<(String, String)>,
    result: Result<Vec<(String, String)>, String>,
) -> (Vec<(String, String)>, String) {
    match result {
        Ok(value) => {
            let status = if value.is_empty() {
                "尚無房間"
            } else {
                "請選擇房間"
            };
            (value, status.to_owned())
        }
        Err(message) => (
            previous,
            format!("讀取失敗：{message}，請重試"),
        ),
    }
}
```
**Green UI 接線**：刪除重新整理按鈕在 HTTP 請求前清空 rows 的操作；只有成功才替換，失敗保留舊結果。
```rust
let previous = rows.get_untracked();
let result = room_api::list_rooms().await;
let (next_rows, next_status) = refreshed_room_list(previous, result);
set_rows.set(next_rows);
set_status.set(next_status);
set_loading.set(false);
```
## 3. 套件版本
crates/ui/Cargo.toml：Leptos／leptos-router 採 workspace 版本；WASM 依賴 web-sys 0.3、wasm-bindgen 0.2、wasm-bindgen-futures 0.4。無新增套件或 migration。Cargo.lock 精確解析版、MSRV 未重查。
## 4. 驗證項目
- cargo test --locked -p ui --features ssr day19_refresh_failure_keeps_previous_room_links：預期 Red 缺 helper 編譯失敗、Green 後通過；實際未執行。
- cargo test --locked -p ui --features ssr；cargo leptos build --cargo-locked：實際未執行，既有 SSR 提示斷言缺口另列。
- 瀏覽器：200 載入→503 重新整理保留房間 ID→200 重試成功；未執行。
- 來源 f692c2e 的 GitHub Actions 六個 workflow 成功，但不含 Day19 修改。
- 本頁為私人草稿，正式發布受阻；next_day 仍 19，程式未套用、產品未整合、完整功能未驗收。
</content>
</page>