## 1. 學習大綱
本篇原教學與以下程式以98f30d8發布，保留歷史身分。當前main `63f66e7742ecd09cd5199a47dbacc9df666af416`：main完整workspace為126 passed／1 failed／0 ignored，失敗為SSR缺少「歷史載入尚未接通」。瀏覽器完整房名3匹配只顯示2；部分中文含空白／rUsT／空白查全部通過，無相符缺明確提示。真實WS七案、HTTP503保留清單與重試通過。候選128 passed／0 failed／0 ignored、名稱完整3筆與空結果提示通過；未合併main。
完整當前修正與本輪驗證集中於 <mention-page url="https://app.notion.com/p/3fde51d45093816bb36dd9099190d907"/>；本篇原發布時專用Red／Green未執行的事實不改寫。
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
原發布時Day17專用測試與GUI未執行，仍是歷史紀錄。本輪實際重測目前已有入口、Day18及完整workspace，不以day17篩選後0 tests冒稱通過。
main完整workspace為126 passed／1 failed／0 ignored，失敗為SSR缺少「歷史載入尚未接通」。瀏覽器完整房名3匹配只顯示2；部分中文含空白／rUsT／空白查全部通過，無相符缺明確提示。真實WS七案、HTTP503保留清單與重試通過。候選128 passed／0 failed／0 ignored、名稱完整3筆與空結果提示通過；未合併main。
精確命令、候選完整patch、產品缺口與CI見[本輪完整證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-full-63f66e7/REPORT.md)；沒有增加Day或規則。
