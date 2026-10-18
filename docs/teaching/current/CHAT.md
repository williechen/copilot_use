## 1. 學習大綱
本輪main `fa0888ea1e1ea93415a4ba7f16972938969b1be3`；修正候選 `2f60c731ca78c689cb78abbd7bea239146ed3aaa` 在獨立分支 codex/full-acceptance-63f66e7，未合併或推送main。原Day18頁修訂，next_day各19不遞增。
目標維持首頁名稱查詢完整切片：RoomList按鈕 → GET /api/rooms → 完整rows → 既有filter_rooms（trim、忽略英文大小寫、名稱包含）→ 真實ID連結。main完整workspace為126 passed／1 failed／0 ignored，失敗為SSR缺少「歷史載入尚未接通」。瀏覽器完整房名3匹配只顯示2；部分中文含空白／rUsT／空白查全部通過，無相符缺明確提示。真實WS七案、HTTP503保留清單與重試通過。候選129 passed／0 failed／0 ignored、名稱完整3筆與空結果提示通過；未合併main。
本輪並查核新增服務分庫：Chat使用root六份SQLite migrations，Pet另用自己三份。新建可以啟動；4198fb1／7917f8b舊Chat DB因missing applied migration91733升級失敗。重建空DB不是既有資料升級驗收。
原Red／Green學習15分鐘；候選套用與回歸另估20分鐘，migration兼容為另項工作，未擴大成本篇全部產品驗收。
本次fa0888e新增chat_action_log：Chat六份migrations／20張業務表，Pet三份／9張，Analytics三份／15張。P0的table與field可依功能需求增減或修改，現有schema不是固定限制。新建及9a95a10→本版升級通過；action log缺少或NULL主鍵仍能插入、任意操作者ID可接受，且無Rust寫入呼叫端。候選以前向migration加NOT NULL，保留有效舊row，遇NULL舊row停止且保留原資料；操作者身分綁定、低敏稽核與真實寫入仍未完成。舊Chat91733／舊PG31升級仍失敗；本輪未操作使用者資料庫。
## 2. 實作內容
### 已執行的 Red
獨立SQLite API有兩筆「Rust 技術交流」及一筆「Rust 技術交流延伸」。main filter_room先收集所有完全同名，導致按完整名稱後只顯示2筆，原包含匹配應有3筆。目前filter_room比較的是room_name完全相等，並非room ID；問題是預先縮小了名稱包含搜尋的候選集合。這不是舊98f30d8「沒有HTTP請求」，也不是794da6e「只取第一筆」；來源分開。
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
[以本次main為基底的完整patch](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-full-fa0888e/functional-fixes.patch)包含以上完整修改與回歸測試；基底正是本輪main。HTTP失敗不清空舊rows，成功後才更新；空白列全、大小寫與中文由既有filter處理。這些程式已在候選跑到Green，尚未套用main。
### Refactor與歷史更正
Day17原教學保留98f30d8發布身分；現在入口已接HTTP，不再要求重做已存在的Day18測試。列表讀取／篩選目前分層清楚，避免在API層提前縮減名稱結果。舊patch或候選不可直接當成最新main已修。
### action log欄位修正與完整重測
本次新增表的真實Red：省略id與明確NULL都被SQLite接受。不要覆寫已套用的20261018122554原migration；新增20261018130000_require_chat_action_id.sql：
```sql
-- Forward-only repair. Preserve valid rows; NULL legacy IDs abort the migration
-- transaction so an operator can repair identity without silently losing logs.
CREATE TABLE chat_action_log_required_id (
    id TEXT NOT NULL PRIMARY KEY,
    action_type VARCHAR(255) NOT NULL,
    action_content TEXT,
    ref_table VARCHAR(255),
    ref_id TEXT,
    ref_column VARCHAR(255),
    created_user_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO chat_action_log_required_id
    (id, action_type, action_content, ref_table, ref_id, ref_column, created_user_id, created_at)
SELECT id, action_type, action_content, ref_table, ref_id, ref_column, created_user_id, created_at
FROM chat_action_log;
DROP TABLE chat_action_log;
ALTER TABLE chat_action_log_required_id RENAME TO chat_action_log;
```
有效舊資料須保留；有NULL舊ID時INSERT失敗、整份migration交易回滾，先由操作者修復既有身分，再重試；不可默默刪row或偽造操作者。以下測試放在crates/persistence/src/[lib.rs](http://lib.rs)既有tests模組內：
```rust
#[sqlx::test(migrations = "../../migrations")]
async fn chat_action_log_requires_stable_identity(pool: SqlitePool) {
    sqlx::query("INSERT INTO chat_action_log(id, action_type, created_user_id) VALUES ('fixture-log', 'send', 'fixture-user')")
        .execute(&pool).await.unwrap();
    for sql in [
        "INSERT INTO chat_action_log(action_type, created_user_id) VALUES ('send', 'fixture-user')",
        "INSERT INTO chat_action_log(id, action_type, created_user_id) VALUES (NULL, 'send', 'fixture-user')",
        "UPDATE chat_action_log SET id = NULL WHERE id = 'fixture-log'",
        "INSERT INTO chat_action_log(id, action_type, created_user_id) VALUES ('fixture-log', 'send', 'fixture-user')",
    ] {
        assert!(
            sqlx::query(sql).execute(&pool).await.is_err(),
            "accepted invalid log identity: {sql}"
        );
    }
    let count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM chat_action_log WHERE id = 'fixture-log'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 1);
}
```
本輪另外實測有效舊row升級與重新連線讀回、NULL舊row失敗後原資料／結構／migration history保留，共六案通過。created_user_id目前只要求非空，仍沒有綁定真實已授權人員；這個Green只修穩定紀錄身分，不宣稱完整稽核流程完成。
## 3. 套件版本
本輪實際 Rust／Cargo 1.99.1；SQLx 0.9.0、Axum 0.8.9、Leptos 0.8.22、Tokio 1.53.3、Tauri 2.12.4、tauri-build 2.7.3、wasm-bindgen 0.2.129（Cargo.lock）。cargo-leptos 0.3.11、Trunk 0.21.14；Ubuntu 26.04.1、PostgreSQL 18.6。本輪WebSocket測試用Node 22.22.1，先前Node24為歷史環境；未另驗跨平台／MSRV。候選沒有新增產品依賴。
## 4. 驗證項目
主線命令：fixture PG環境 `cargo test --locked --workspace --exclude desktop-pet-frontend --features ui/ssr,stats-server/postgres-integration --no-fail-fast -- --include-ignored`，126 passed／1 failed／0 ignored；其中SSR 3 passed／1 failed。候選同命令129 passed／0 failed／0 ignored，包含全部doc-tests與PG integration，不把0 tests當覆蓋。
主線與候選`cargo leptos build --cargo-locked`皆通過。主線瀏覽器完整名稱3→2失敗；中文含空白3筆、rUsT4筆、空白5筆通過；主線503保留5筆並重試成功。候選完整名稱3筆、無相符提示與503重試通過，AX與截圖保存。
主線真實WS：ping/pong、跨房拒絕、空白拒絕、同房typing、兩客戶同message ID、另一房無洩漏、重連七案通過，SQLite持久化row已讀回。這是同一測試環境的兩個WS客戶，不能稱跨瀏覽器產品驗收。HTTP全空白／201byte／NUL房名均400；/signin及/api/messages為404，匿名建房201不代表帳號權限通過。
舊Chat DB升級仍失敗；完整帳號／ACL、歷史、跨瀏覽器、行動／鍵盤等原產品要求未完成或未驗證。CI精確SHA／workflow/job與覆蓋、全部命令與原始錯誤見[本輪完整證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-full-fa0888e/REPORT.md)；正式完整功能維持未驗收。
最新main CI快照：17個jobs success、0個尚未完成；精確SHA／時間／job網址見本輪報告。其中Test All/test-ui只跑cargo test --locked -p ui，未啟用SSR；Rust AI Chat/build-chat負責建置，沒有執行本輪失敗的SSR行為斷言。精確job連結與workflow原文見本輪報告，CI綠燈不覆蓋這個失敗。
主線fmt通過、嚴格all-targets Clippy exit101（[admin.rs:254](http://admin.rs:254) collapsible_if）；候選已整理logout與refresh條件，但嚴格Clippy仍exit101於apps/desktop_pet/src/[lib.rs:110](http://lib.rs:110) collapsible_if，未宣稱品質全通過。完整命令／結果見本輪證據。
