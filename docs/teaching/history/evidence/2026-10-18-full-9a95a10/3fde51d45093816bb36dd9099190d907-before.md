## 1. 學習大綱
本輪main `63f66e7742ecd09cd5199a47dbacc9df666af416`；修正候選 `3eace139da323fc906133636fae93e4d54159173` 在獨立分支 codex/full-acceptance-63f66e7，未合併或推送main。原Day18頁修訂，next_day各19不遞增。
目標維持首頁名稱查詢完整切片：RoomList按鈕 → GET /api/rooms → 完整rows → 既有filter_rooms（trim、忽略英文大小寫、名稱包含）→ 真實ID連結。main完整workspace為126 passed／1 failed／0 ignored，失敗為SSR缺少「歷史載入尚未接通」。瀏覽器完整房名3匹配只顯示2；部分中文含空白／rUsT／空白查全部通過，無相符缺明確提示。真實WS七案、HTTP503保留清單與重試通過。候選128 passed／0 failed／0 ignored、名稱完整3筆與空結果提示通過；未合併main。
本輪並查核新增服務分庫：Chat使用root四份SQLite migrations，Pet另用自己兩份。新建可以啟動；4198fb1／7917f8b舊Chat DB因missing applied migration91733升級失敗。重建空DB不是既有資料升級驗收。
原Red／Green學習15分鐘；候選套用與回歸另估20分鐘，migration兼容為另項工作，未擴大成本篇全部產品驗收。
## 2. 實作內容
### 已執行的 Red
獨立SQLite API有兩筆「Rust 技術交流」及一筆「Rust 技術交流延伸」。main filter_room先收集所有完全同名，導致按完整名稱後只顯示2筆，原包含匹配應有3筆。這不是舊98f30d8「沒有HTTP請求」，也不是794da6e「只取第一筆」；來源分開。
無相符名稱時清單為空但仍說「查詢完成，請選擇房間」。SSR day16_layout_tests::room_page_has_real_room_identity_and_multiline_composer實際失敗，缺少歷史尚未接通提示。純filter測試已存在且通過，不重複當Red或新增同名符號。
### 完整最小 Green
在crates/ui/src/room_[api.rs](http://api.rs)刪除整個filter_room函式，保留既有create_room／list_rooms。RoomList的query_room閉包完整改為：
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
保留畫面現有`filter_rooms(&rows.get(), &query.get())`、disabled／loading與真實ID連結。ChatPage messages列表前恢復這一行，照實告知歷史未接通：
```rust
<p>"歷史載入尚未接通"</p>
```
[以本次main為parent的完整patch](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-full-63f66e7/functional-fixes.patch)包含以上完整修改與回歸測試；parent正是本輪main。HTTP失敗不清空舊rows，成功後才更新；空白列全、大小寫與中文由既有filter處理。這些程式已在候選跑到Green，尚未套用main。
### Refactor與歷史更正
Day17原教學保留98f30d8發布身分；現在入口已接HTTP，不再要求重做已存在的Day18測試。列表讀取／篩選目前分層清楚，避免在API層提前縮減名稱結果。舊patch或候選不可直接當成最新main已修。
## 3. 套件版本
本輪實際 Rust／Cargo 1.99.1；SQLx 0.9.0、Axum 0.8.9、Leptos 0.8.22、Tokio 1.53.3、Tauri 2.12.4、tauri-build 2.7.3、wasm-bindgen 0.2.129（Cargo.lock）。cargo-leptos 0.3.11、Trunk 0.21.14；Ubuntu 26.04.1、PostgreSQL 18.6。測試用 bundled Node 24.19.0，與先前 Node22 紀錄分開；未另驗跨平台／MSRV。候選沒有新增產品依賴。
## 4. 驗證項目
主線命令：fixture PG環境 `cargo test --locked --workspace --exclude desktop-pet-frontend --features ui/ssr,stats-server/postgres-integration --no-fail-fast -- --include-ignored`，126 passed／1 failed／0 ignored；其中SSR 3 passed／1 failed。候選同命令128 passed／0 failed／0 ignored，包含全部doc-tests與PG integration，不把0 tests當覆蓋。
主線與候選`cargo leptos build --cargo-locked`皆通過。主線瀏覽器完整名稱3→2失敗；中文含空白3筆、rUsT4筆、空白5筆通過；503保留3筆並重試成功。候選完整名稱3筆、無相符提示與503重試通過，AX與截圖保存。
主線真實WS：ping/pong、跨房拒絕、空白拒絕、同房typing、兩客戶同message ID、另一房無洩漏、重連七案通過，SQLite持久化row已讀回。這是同一測試環境的兩個WS客戶，不能稱跨瀏覽器產品驗收。HTTP全空白／201byte／NUL房名均400；/signin及/api/messages為404，匿名建房201不代表帳號權限通過。
舊Chat DB升級仍失敗；完整帳號／ACL、歷史、跨瀏覽器、行動／鍵盤等原產品要求未完成或未驗證。CI精確SHA／workflow/job與覆蓋、全部命令與原始錯誤見[本輪完整證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-full-63f66e7/REPORT.md)；正式完整功能維持未驗收。
最新main的17個CI jobs皆success；其中Test All/test-ui只跑cargo test --locked -p ui，未啟用SSR；Rust AI Chat/build-chat負責建置，沒有執行本輪失敗的SSR行為斷言。精確job連結與workflow原文見本輪報告，CI綠燈不覆蓋這個失敗。
