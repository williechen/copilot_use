## 1. 學習大綱
本次更新核對（main `2eddd6428cda6c81c53b1e261d0846ebcf56e6a0`）：新 migration 20261018100114 使用 SQLite 不支援的 MODIFY COLUMN。SQLx 新建與從1853876升級均失敗；清除 persistence 套件後 0 passed／5 failed。Chat 啟動亦受 migration 阻擋，尚不能驗收房間功能。搜尋三筆只顯示兩筆與 SSR 提示失敗仍是未修的1853876基線；本輪沒有宣稱最新服務已可供瀏覽器驗證。 [本輪命令與證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-2eddd64/REPORT.md)。以下1853876／候選80ba01e的數字與程式保留為上一輪基線；候選未含新migration，105通過不可當作最新main通過。正式完整驗收仍未通過，未新增規則或Day。
SQLite 修正候選 `e33971f508e9612770ad72960b62a8b090efd53e`（工作分支，未合併main）：以重建表取代不支援的ALTER，保留有效資料／ID，owner_id對齊pet_owner_id並強制外鍵。新建／有效資料升級／缺少owner ID／孤兒owner四案通過，後兩者須拒絕並rollback。最新main仍失敗；此候選只修SQLite migration，未包含之前Chat搜尋與Analytics Cookie候選。完整SQL與重現命令見 [修正與證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-2eddd64/REPORT.md)。
本次複驗來源 rust_ai_chat/main `1853876e0eb1cfe8e4562be8b3c661b55f1bc54b`；修正候選 `80ba01e6d965d0e3ff10bef18a5cbd17f01e8f05`，工作分支 codex/day18-revalidate-1853876，尚未合併 main。Day18 已發布，next_day 為 19，本次修訂不新增 Day。
目標仍是首頁名稱查詢：按鈕 → GET /api/rooms → 完整 rows → filter_rooms（trim、忽略英文大小寫、名稱包含）→ 保留真實 ID 的房間連結；成功／空結果／失敗後可重試。原發布來源 98f30d8 的「未發請求」是歷史觀察；目前 main 已修正借用且 SSR/WASM 乾淨建置通過；但完整名稱查詢會先由 filter_room 取所有完全相同名稱，漏掉既有 filter_rooms 名稱包含邏輯原本應保留的其他房間。本次 API 有 3 筆匹配，瀏覽器只列 2 筆。先前 abebbef 的 E0382 已解除，保留歷史證據而非當成目前失敗。
本切片不等於登入／ACL／歷史／整個產品通過。原 Red/Green 學習估時 15 分鐘；此次契約修正與回歸另估 20 分鐘，正式驗證耗時不算成使用者學習估時。
## 2. 實作內容
### Red：本次已實際執行
在 main 乾淨建置成功後，以獨立測試房間呼叫 API 並操作瀏覽器：兩筆同名「Rust 技術交流」及一筆「Rust 技術交流延伸」都符合既有名稱包含篩選，但查詢完整房名只回兩筆完全同名。這是本輪已實測的 Red；中文含空白與空白查全部已通過，不再說它們仍失敗。現有 day18_query_filter_preserves_room_ids 已存在且可通過，這不是新的 Red；不重複新增同名測試。
另執行 SSR 測試，day16_layout_tests::room_page_has_real_room_identity_and_multiline_composer 因缺少「歷史載入尚未接通」失敗。歷史功能仍未接通，故修正畫面提示而非移除測試要求。
### 完整最小 Green：候選程式，尚未套用 main
在 crates/ui/src/room_[api.rs](http://api.rs) 刪除整個未再使用的 filter_room 函式；create_room、list_rooms 的 HTTP／JSON／ID 驗證保持既有實作。名稱搜尋由既有 filter_rooms 處理，不再用完全同名預先篩選先縮減 rows。這修正漏列既有匹配房間的行為，沒有增加名稱搜尋規則。
將 RoomList 的 query_room 閉包完整替換如下：
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
                    set_status.set(
                        if value.is_empty() {
                            "尚無房間"
                        } else if filter_rooms(&value, &query.get_untracked()).is_empty() {
                            "沒有符合條件的房間"
                        } else {
                            "查詢完成，請選擇房間"
                        }
                        .into(),
                    );
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
按鈕 disabled/loading、輸入與 `filter_rooms(&rows.get(), &query.get())` 連結生成保留。HTTP 失敗不清空 rows；空白顯示全部，無相符名稱明示空結果。ChatPage 的 messages 列表之前恢復這一行，照實揭露現況：
```rust
<p>"歷史載入尚未接通"</p>
```
[完整可套用 patch](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-1853876/day18-functional-fixes.patch) 包含本篇必要修改與相關回歸測試；其 parent 是本次 main，並非宣稱已合併。
### Refactor／舊 Day 更正補充
Day17／Day18 原有純篩選測試不能證明查詢按鈕接線。原發布時缺 GET／loading 不解除，與 abebbef 的 E0382、最新 1853876 的匹配房間漏列，是不同版本的缺陷。本次保留兩者來源身分，不把修正候選的通過回填到 main。讀取／重新整理與查詢的共用 helper 尚未重構。
## 3. 套件版本
Rust 1.99.1／Cargo 1.99.1；Cargo.lock：Axum 0.8.9、Leptos 0.8.22、SQLx 0.9.0、Tokio 1.53.3、Tauri 2.12.4、tauri-build 2.7.3、wasm-bindgen 0.2.129、web-sys 0.3.106、Tower 0.5.3。Ubuntu 26.04.1／PostgreSQL 18.6／Node 22.22.1／cargo-leptos 0.3.11／Trunk 0.21.14。本次不新增產品依賴；MSRV／跨平台未另行驗證。
## 4. 驗證項目
main：`cargo test --locked -p ui --features ssr -p stats-server -p desktop-pet`，合計 58 passed／1 failed／1 ignored；Chat SSR 子集 3 passed／1 failed。`cargo leptos build --cargo-locked` 通過；瀏覽器中文含空白／空白通過，但完整房名 3 筆匹配只列 2 筆，未通過完整查詢流程。
候選：上一輪完整回歸 105 passed／0 failed／1 ignored；SSR/WASM 建置與瀏覽器中文含空白、空白查全部、rUsT、無相符、HTTP 503 保留列表與重試通過。本輪再次執行的精確命令、結果、產物與 SHA 統整於 [本次正式複驗與原始證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-1853876/REPORT.md)，候選證據與 main 分列。
CI 按最新 main SHA 與 abebbef 歷史失敗分開記錄 workflow/job、時間與連結；不以 Test All 的預設 UI 測試取代 SSR 行為測試或 WASM hydrate build。此篇教學的正式功能結論仍是 main 未通過。
產品剩餘：GET /signin 404，帳號／個人管理／server ACL、保留選房登入、歷史、跨瀏覽器 WS、手機／鍵盤等未完成或未驗證。必需修改集中在本篇 Green 與原產品項目，沒有增加規則。
