## 1. 學習大綱
目標：讓首頁「查詢房間」按鈕真正讀取 GET /api/rooms，成功顯示經關鍵字篩選的房間，失敗保留舊列表並解除 loading。Day17 已教過查詢閉包候選但來源 main 尚未套用；Day18 採同一功能的可套用、可回歸切片，不冒稱 Day17 已實作。主體 Red/Green 15 分鐘。
現況（rust_ai_chat/main 98f30d8b3593d80103b1a4bdbb9f0cb8cb08069e）：crates/ui/src/[lib.rs:218](http://lib.rs:218)-287 的 RoomList，query_room 只清空 rows、設 loading=true；既有重新整理按鈕才呼叫 room_api::list_rooms。crates/ui/src/room_[api.rs:57](http://api.rs:57)-87 已以瀏覽器 fetch GET /api/rooms，檢查 HTTP、解析 JSON 與房間 ID；RoomList 的 filter_rooms 將 rows/query 轉成連結。呼叫鏈：button on:click → query_room → room_api::list_rooms → fetch /api/rooms → set_rows → filter_rooms → /room/\{id\}。伺服器端不執行瀏覽器 fetch。
## 2. 實作內容
### Red（先寫，尚未執行）
在 crates/ui/src/[lib.rs](http://lib.rs) 既有 #\[cfg(test)\] mod day16_room_search_tests 增加測試，驗證空白關鍵字顯示全部、中文查詢僅顯示符合房間。純 filter_rooms 測試本身可能已通過，**真正 Red** 是瀏覽器 Network 點擊「查詢房間」沒有 GET /api/rooms 且 loading 永遠為 true；測試計畫需以瀏覽器整合測試先觀察失敗，不能把預期寫成實測。
```rust
#[test]
fn day18_query_filter_preserves_room_ids() {
    let rows = vec![
        ("room_1".to_string(), "Rust 技術交流".to_string()),
        ("room_2".to_string(), "生活閒聊".to_string()),
    ];
    assert_eq!(super::filter_rooms(&rows, " 技術 "), vec![rows[0].clone()]);
    assert_eq!(super::filter_rooms(&rows, " "), rows);
}
```
### 完整最小 Green
將 crates/ui/src/[lib.rs:225](http://lib.rs:225)-232 的整個 query_room 閉包替換如下（其餘 view 與 filter_rooms 不變）：
```rust
let query_room = move || {
    if loading.get_untracked() {
        return;
    }
    set_loading.set(true);
    set_status.set("查詢中…".into());
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(async move {
        match room_api::list_rooms().await {
            Ok(value) => {
                set_status.set(if value.is_empty() {
                    "尚無房間"
                } else {
                    "查詢完成，請選擇房間"
                }.into());
                set_rows.set(value);
            }
            Err(message) => {
                set_status.set(format!("查詢失敗：{message}，請重試"));
            }
        }
        set_loading.set(false);
    });
    #[cfg(not(target_arch = "wasm32"))]
    {
        set_loading.set(false);
    }
};
```
按鈕原本 on:click=move \|_\| \{ query_room() \}、disabled=move \|\| loading.get()、搜尋輸入、filter_rooms 及 room_[api.rs](http://api.rs) 均保留。空白關鍵字可查全部；成功後按目前 query 篩選；失敗保留先前 rows；WASM 成功或失敗都解除 loading。這是完整事件閉包替換，不代表已在產品實際套用。
### Refactor／補教
下一個切片可把「查詢」與「讀取／重新整理」合併為一個共用 async request helper，並補瀏覽器網路失敗/重試測試；本日不重構既有重新整理按鈕。Day17 未套用部分以本篇教學補足，舊頁保留。
## 3. 套件版本
crates/ui/Cargo.toml：Leptos、leptos_router 為 workspace 版本；WASM 依賴 wasm-bindgen 0.2、wasm-bindgen-futures 0.4、web-sys 0.3；不新增套件。精確 lock/MSRV 本次未執行命令確認。
## 4. 驗證項目
來源 SHA：rust_ai_chat/main 98f30d8b3593d80103b1a4bdbb9f0cb8cb08069e（唯讀）；教學規則 daily-tdd-minimum-v1；教學分支發布前 SHA 0641e463633bb1fc05d8afb3272708ff181cc6a9。
命令：`cargo test -p ui --features ssr day18_query_filter_preserves_room_ids`；預期測試通過，實際未執行。`cargo build -p ui --target wasm32-unknown-unknown --features hydrate`；預期編譯通過，實際未執行。瀏覽器：點查詢後 Network 應有 GET /api/rooms，空白顯示全部、中文只顯示相符、HTTP 失敗保留列表、loading 恢復；實際未執行。CI：未取得匹配本篇修改的 workflow/job 成功證據，不標記通過。教學發布≠程式套用≠產品整合≠正式驗收；三者均未驗證。
待完成：實際套用 patch、編譯與 GUI/HTTP 端到端驗證，按 SHA 對應記錄結果。
