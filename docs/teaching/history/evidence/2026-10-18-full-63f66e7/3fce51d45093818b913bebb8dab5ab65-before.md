## 1. 學習大綱
本次更新核對（main `7917f8b2e05ed74703ff9a83b62732ce04461da7`）：本輪只改兩份root SQLite migrations。新建SQLx遷移、persistence 5案、重新編譯／啟動及/health、/api/rooms HTTP200通過；從6e1c791既有DB升級與啟動仍因91733 checksum改變失敗。UI搜尋／SSR程式與lock無變，前輪3匹配→2顯示及歷史提示斷言缺口仍未修，本輪未重跑WASM／瀏覽器。 [本輪命令與27個schema案例](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-7917f8b/REPORT.md)。以下6e1c791及更早版本／候選保留為歷史基線，不回填為最新main通過。完整功能仍未驗收；未新增規則或Day。
歷史候選e33971f只適用舊2eddd64；最新main已修改建表與ID契約並刪除ALTER檔，該patch不可直接套用。先前候選成功結果保留原SHA，不代表本輪checksum升級問題已解決。
本篇以下原教學以 98f30d8 為發布來源，保留歷史身分。當前複驗 main `1853876e0eb1cfe8e4562be8b3c661b55f1bc54b`：名稱查詢已接 HTTP，但 main `1853876e0eb1cfe8e4562be8b3c661b55f1bc54b` 的 SSR/WASM 已乾淨建置成功，中文含空白與空白查詢通過；但 filter_room 完全匹配時只取同名筆數，API 的 3 筆名稱包含匹配房間在瀏覽器只列 2 筆；SSR 仍有 1 項「歷史載入尚未接通」提示斷言失敗。main 未通過。工作分支 `80ba01e6d965d0e3ff10bef18a5cbd17f01e8f05` 以 list_rooms／既有 filter_rooms 修正，候選完整回歸 105 passed／1 ignored，SSR/WASM 成功；候選不等於已套用 main。 完整當前修改與證據統整於 <mention-page url="https://app.notion.com/p/3fde51d45093816bb36dd9099190d907"/>。
目標：首頁輸入房間名稱後按「查詢房間」，實際讀取伺服器房間清單、篩選結果、恢復按鈕狀態；錯誤時提供可重試提示。入口 crates/ui/src/[lib.rs](http://lib.rs)::RoomList → room_api::list_rooms → GET /api/rooms → filter_rooms → \<ul\>。原始 query_room 只設定 loading=true 並清空 rows，永遠不呼叫 API；「重新整理」已能呼叫 API，應重用，不創建第二個 HTTP client。預估 Red+Green 20 分鐘。來源 rust_ai_chat/main 98f30d8b3593d80103b1a4bdbb9f0cb8cb08069e（參考庫唯讀）；Notion 現行共同規則 3f2e51d4-5093-81dd-b8a8-ce7cb4cd3e60。這是教學程式碼，未套用產品、未在本次環境執行測試，不宣稱功能正式驗收。
## 2. 實作內容
以下為發布時的 Red／Green；當前來源已有部分套用，不能重複新增既有符號或把這些片段單獨當成最新完整修正。
### Red
在 crates/ui/src/[lib.rs](http://lib.rs) 的既有 day16_room_search_tests 中加入測試，要求查詢能在空白關鍵字時顯示完整列表、在中文關鍵字時只顯示相符房間；另以瀏覽器整合測試確認按查詢後真正 GET /api/rooms、loading 最終恢復（目前預期失敗，尚未執行）。
```rust
#[test]
fn day17_query_uses_existing_room_filter() {
    let rooms = vec![
        ("r1".to_owned(), "Rust 技術交流".to_owned()),
        ("r2".to_owned(), "生活閒聊".to_owned()),
    ];
    assert_eq!(super::filter_rooms(&rooms, " 技術 ").len(), 1);
    assert_eq!(super::filter_rooms(&rooms, " ").len(), 2);
}
```
注意：此純函式測試在現有程式可能已通過，**真正的 Red 是瀏覽器點擊後無 GET、loading 不解除**；不能把純函式通過冒稱 Red。
### 最小 Green
將 crates/ui/src/[lib.rs](http://lib.rs) 的 RoomList 中 **整個 query_room 閉包**替換成以下內容，其他 RoomList、既有「讀取／重新整理房間」按鈕及 room_[api.rs](http://api.rs) 保留。這是完整的查詢事件邏輯，不更動既有路由。
```rust
let query_room = move || {
    // 防止重複查詢；空白關鍵字代表顯示全部房間。
    if loading.get_untracked() { return; }
    set_loading.set(true);
    set_status.set("查詢中…".into());
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(async move {
        // 直接重用既有 GET /api/rooms；顯示層的 filter_rooms 依 query 自動篩選。
        match room_api::list_rooms().await {
            Ok(value) => {
                set_status.set(if value.is_empty() { "尚無房間" }
                    else { "查詢完成，請選擇房間" }.into());
                set_rows.set(value);
            }
            Err(message) => {
                // 保留舊列表，避免暫時網路錯誤把可用內容清掉。
                set_status.set(format!("查詢失敗：{message}，請重試"));
            }
        }
        set_loading.set(false);
    });
    #[cfg(not(target_arch = "wasm32"))]
    {
        // SSR 不執行瀏覽器 fetch，避免按鈕永久 disabled。
        set_loading.set(false);
    }
};
```
移除原本 `if query.get().trim().is_empty() { return; }` 與 `set_rows.set(Vec::new())`；既有 `filter_rooms(&rows.get(), &query.get())` 不變。真實鏈：click → query_room → room_api::list_rooms → GET /api/rooms → set_rows → filter_rooms → room links；HTTP 失敗 → status → loading=false。
### Refactor
後續把「查詢」與「重新整理」抽成同一個 request helper，避免兩個按鈕維護重複 fetch；本日先不擴大修改。Day16 已封存，不回填。
### 舊 Day 更正補充
Day16 已有純搜尋篩選測試，不等於「查詢房間」事件已連接 HTTP；Day17 明確補齊此差異。
## 3. 套件版本
crates/ui/Cargo.toml 使用 leptos、leptos_router workspace 版本；WASM 專用 wasm-bindgen 0.2、wasm-bindgen-futures 0.4、web-sys 0.3；本切片不新增相依。實際 lock/MSRV 與官方 stable 尚待命令核實，不虛報已查核；SSR 端不執行 web_sys fetch。
## 4. 驗證項目
原發布時的 Day17 專用測試與 GUI 並未執行，這一歷史事實保留。本次實際回歸使用目前已有的 Day18 測試及入口，而非把 day17 名稱篩選得到的 0 tests 算成通過。
本輪 main：UI SSR 3 passed／1 failed，SSR/WASM clean build 通過；名稱查詢 3 筆匹配只顯示 2 筆失敗；來源主要套件共 58 passed／1 failed／1 ignored。候選本輪完整回歸 105 passed／0 failed／1 ignored，另有候選瀏覽器與 HTTP 失敗重試證據。main 的瀏覽器新產物已建置並實測，仍漏列一筆。
後續實作仍是名稱查詢與 SSR 提示的實際修正，並補登入／個人管理／server ACL、保留目標房間、歷史、跨瀏覽器 WS 等原產品功能；GET /signin 實測 404，匿名建房 201 不是帳號權限驗收通過。
[正式複驗與原始證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-1853876/REPORT.md)；當前完整程式與可套用 patch 見 <mention-page url="https://app.notion.com/p/3fde51d45093816bb36dd9099190d907"/>。Day17／Day18 修訂沒有新增 Day，也沒有改變原產品完成條件。
