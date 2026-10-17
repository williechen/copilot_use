# 2026-10-17｜P0 Chat Day 16｜清楚選房與聊天

## 1. 學習大綱
修訂 r2（2026-10-17）。來源：`rust_ai_chat/main` `b2733b7f78dce1da1aa07555122384c27109e4e8`；本機工作目錄仍為 `fced2861`，本輪僅 fetch／靜態讀取，未 checkout、未改原始碼。
**狀態：舊版 Day16 的 Red 已部分套用；本次主流程為待套用教學計畫，未提供已驗證的完整實作。** 最新 SHA 的 18 個 checks 為 16 success／2 failure（`test-ui`、`test-stats-server`）；不沿用 Day15 CI 作為本篇通過證據。三篇主體只優先交付可操作的基本流程，搜尋／總數／競態是相關附加工作。

交付流程：首頁顯示真實房間清單 → 選房並看清目前房間 → 在訊息區閱讀本次連線收到的訊息 → 多行輸入並送出 → 收到伺服器回覆。基本畫面包含房間側欄、目前房間標題、訊息區與輸入區；小螢幕仍能選房及送訊。
既有基礎為 `GET /api/rooms`、`room_api::list_rooms`、`/room/:room_id`、WebSocket。未提供的歷史訊息、時間、在線人數與個人身分不可用假資料補齊。名稱搜尋可附加，但不是本日主體。
## 2. 實作內容
### Red：先定義可操作流程
保留已套用的 `day16_room_search_tests`（見本節末），另外驗證：可從真實列表進房；畫面指出目前房間；空訊息不能送；連線失敗有可見狀態且保留草稿；切房後不再接收舊房間更新。瀏覽器流程須實際操作，純函式測試不能替代。
### Green：接通選房與聊天
1. 在 `crates/ui/src/lib.rs` 的 `HomePage`／`RoomList` 保留現有 API，顯示載入、空列表與錯誤重試；房間名稱與 id 來自 API。選房連結沿用既有路由。
2. `RoomPage`／`ChatPage` 形成清楚的房間標題、訊息清單、文字輸入與送出區。多行輸入可先用明確的送出按鈕，避免 Enter 行為混亂；訊息依真實作者標籤顯示，無法判定本人時不可全部標「我」。
3. `crates/ui/src/chat_socket.rs` 提供可見的連線中／已連線／失敗狀態；URL 依目前頁面的 host 與 http/https 選 ws/wss，避免寫死 localhost。成功送出才清空草稿。切換房間時釋放舊 socket 與事件 handler。
4. 在上述流程通順後，再完成已套用搜尋 Red 所要求的 `filter_rooms` 並接入清單。搜尋只過濾既有 API 結果，不新增虛構分類或總數。
### Refactor：根目錄 tests 的 Chat 批次
`tests/chat/persist_publish.rs` → `crates/chat_application/tests/persist_publish.rs`；`tests/chat/product_event_contract.rs` → `crates/chat_application/tests/product_event_contract.rs`。既有 `apps/server/tests/root_hub_contract.rs` 保留。

搬移前，以 `tests/Cargo.toml` 的 target、測試名稱與數量建立基準；搬移時保留原斷言，調整目的 package 的 dev-dependencies，再移除原 `[[test]]` 與不再使用的相依。不要拆分原始碼內嵌單元測試。同步 `.github/workflows/test.yml`：目的測試須在具備原 DB／環境設定的 job 執行；不得將需要 DB 的案例直接丟給未配置 DB 的 job。其他根目錄批次尚未完成時保留 `workspace-tests` 與 CI；全部驗證後才移除根目錄測試容器、workspace member 與空目錄，更新 lockfile。測試名稱／數量不得無故減少或重複，既有失敗與功能變更另記，不在純搬移中偷改斷言。

### 舊 Day 更正補充與已套用紀錄
Day15 原文封存。舊版 Day16 只聚焦搜尋，無法代表完整聊天流程；本修訂重整尚未實作的主流程。以下 Red 已在來源 SHA 套用，原樣保留；尚缺函式實作，不宣稱 Green：
```rust
#[cfg(test)]
mod day16_room_search_tests {
    use super::filter_rooms;

    #[test]
    fn search_matches_chinese_names_and_preserves_ids() {
        let rooms = vec![
            ("room_1".to_owned(), "Rust 技術交流".to_owned()),
            ("room_2".to_owned(), "生活閒聊".to_owned()),
        ];
        assert_eq!(filter_rooms(&rooms, " 技術 "), vec![rooms[0].clone()]);
        assert_eq!(filter_rooms(&rooms, ""), rooms);
        assert!(filter_rooms(&rooms, "不存在").is_empty());
    }
}
```
## 3. 套件版本
沿用來源 workspace 0.2.15／edition 2024；Leptos 與 leptos_router 宣告 0.8，web-sys 0.3。精確解析版本依來源 Cargo.lock。未新增搜尋套件；未核實官方最新 stable 或 MSRV。
## 4. 驗證項目
`cargo test -p ui`；`cargo check -p ui --features ssr`；`cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features hydrate`；搬移後 `cargo test -p chat-application --test persist_publish --test product_event_contract`。瀏覽器驗收：列表→選房→送訊→切房、斷線保留草稿、手機畫面。
本輪只核對來源、已套用 Red 與遠端 CI，**上述新主流程、Green、tests 搬移及驗收命令均未執行**。來源 Red 的預期失敗與非預期錯誤要分開記錄；修正後重新檢查同一 SHA 的結果，不能刪除測試讓 CI 變綠。完成條件是主流程可操作及相關驗證通過，不是只有頁面發布或 helper 測試通過。
