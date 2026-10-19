## 1. 學習大綱
**目標**：房間搜尋文字仍在輸入框時，按「讀取／重新整理房間」成功後應顯示剛從後端載入的完整快照，而不是再被舊搜尋字串二次過濾。刷新失敗仍保留原文字、原連結與錯誤提示，沿用 Day19 的錯誤保護。
原教學基底 **rust_ai_chat/main=0cd9f7995c196a7359864a9395c0fe2372d09222**（歷史 Red 基準）；目前程式 **rust_ai_chat/main=c92831e105fb0000860f995ea288e29631b5fed8**（唯讀）；**copilot_use/teaching/rust-ai-chat-alignment=092066d59837a790cf18eb5bbfe1c2abd09321d1**；規則 **daily-tdd-minimum-v1**。入口：`HomePage/RoomPage → RoomList → reload button → room_api::list_rooms → search_rooms("") → GET /api/rooms?q= → Axum list_rooms → RoomRepository::list/SQLite → JSON → rows → filter_rooms(rows,query) → /room/{id}`。在原教學基底上，`RoomList` 刷新時更新 rows 卻保留舊 query，因此全量回應可能被二次過濾；最新主線已加入 `refresh_result_view` 並於刷新事件更新 rows/query/status，已修正這項缺口。獨立「查詢房間」仍維持搜尋條件，不修改查詢 API。主體 Red 5 分鐘、Green 10 分鐘，三專案共用彈性 15 分鐘；Refactor、建置和原生／瀏覽器驗證另計。
### 2026-10-19｜r2 真實程式與測試核對
最新來源 `c92831e105fb0000860f995ea288e29631b5fed8`：`crates/ui/src/lib.rs` blob `5317dbfb2df9c16881d89ef4885aa8f1da136d4c`，已存在 `refresh_result_view(previous, previous_query, result)`，`RoomList` 的 reload callback 於 `room_api::list_rooms().await` 回來後設定 `set_rows`、`set_query`、`set_status`；`day20_refresh_success_reveals_full_snapshot` 與 `day20_refresh_failure_keeps_query_and_rows` 也均存在。`crates/ui/src/room_api.rs` 的 `list_rooms()` 確實轉呼叫 `search_rooms("")`；後端資料仍由 `/api/rooms` 讀取。新程式已合入 main，部署到使用者環境仍未驗證。本篇下列 Red／Green 為從舊基底學習的歷史重現步驟；**在目前 main 直接再新增同名 helper 或測試會重複定義**，不可照貼第二份。
來源此 SHA 的 [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39202728570)／`test-ui`（job `117898951180`）成功；[Rust AI Chat](https://github.com/williechen/rust_ai_chat/actions/runs/39202728449)／`check-chat`、`build-chat` 均成功。CI 已含本日程式，不等於瀏覽器 200→503→200、真 SQLite 與正式產品整合驗收。特別留意 async 請求發出後使用者再鍵入的新 query 可能在回應完成時被清空；此競態不是本切片已解決事項。
## 2. 實作內容
### Red（先新增測試）
在 `crates/ui/src/lib.rs` 已存在的 `#[cfg(test)] mod day16_room_search_tests` 內新增以下測試（先不要加入 Green helper）。預期編譯 **E0425**（找不到待新增的 `refresh_result_view`），屬明確預期的符號缺失 Red，**本次未執行**，不可冒稱實測：
```rust
#[test]
fn day20_refresh_success_reveals_full_snapshot() {
    let old = vec![("a".into(), "Rust".into())];
    let fresh = vec![
        ("a".into(), "Rust".into()),
        ("b".into(), "Pet".into()),
    ];
    let (rows, query, status) =
        super::refresh_result_view(old, "Rust".into(), Ok(fresh.clone()));
    assert_eq!(rows, fresh);
    assert_eq!(query, "");
    assert_eq!(status, "請選擇房間");
    assert_eq!(super::filter_rooms(&rows, &query).len(), 2);
}

#[test]
fn day20_refresh_failure_keeps_query_and_rows() {
    let old = vec![("a".into(), "Rust".into())];
    let (rows, query, status) = super::refresh_result_view(
        old.clone(), "Rust".into(), Err("503".into()),
    );
    assert_eq!(rows, old);
    assert_eq!(query, "Rust");
    assert_eq!(status, "503，請重試");
}
```
### 完整最小 Green
在 `crates/ui/src/lib.rs` 的現有 `refreshed_room_list` **後方**新增 helper。保留原有函式（其他 Day19 tests 仍會使用），不可只換一個無呼叫者的測試 helper：
```rust
fn refresh_result_view(
    previous: Vec<(String, String)>,
    previous_query: String,
    result: Result<Vec<(String, String)>, String>,
) -> (Vec<(String, String)>, String, String) {
    match result {
        Ok(next) => {
            let status = if next.is_empty() { "尚無房間" }
                         else { "請選擇房間" };
            (next, String::new(), status.to_owned())
        }
        Err(error) => (previous, previous_query, format!("{error}，請重試")),
    }
}
```
在現有 `#[component] fn RoomList()` 的第二個按鈕「讀取／重新整理房間」之 `#[cfg(target_arch = "wasm32")]` async 區塊，只把這一段：
```rust
let previous = rows.get_untracked();
let result = room_api::list_rooms().await;
let (next_rows, next_status) = refreshed_room_list(previous, result);
set_rows.set(next_rows);
set_status.set(next_status);
set_loading.set(false);
```
**完整替換**成下面一段；其餘 RoomList 保持不變：
```rust
let previous = rows.get_untracked();
let previous_query = query.get_untracked();
let result = room_api::list_rooms().await;
let (next_rows, next_query, next_status) =
    refresh_result_view(previous, previous_query, result);
set_rows.set(next_rows);
set_query.set(next_query);
set_status.set(next_status);
set_loading.set(false);
```
這是完整最小變更（新增純 helper＋既有事件呼叫端＋兩個測試），不是要求重建整個 RoomList。原 `prop:value=move || query.get()` 與 `filter_rooms(&rows.get(), &query.get())` 原封不動，透過成功清空 query 讓全部 rows 顯示。失敗分支保留 query 和 rows；並沿用 Day19 的載入互斥。成功空陣列清空舊清單。
### Refactor／補教
可把 Day19 的 `refreshed_room_list` 委派給新的 `refresh_result_view(previous,String::new(),result)`，但本次 Green 不強制，避免破壞既有測試。注意「搜尋房間」仍同時在伺服器與畫面依 query 篩選；本日只處理明確的全量刷新。瀏覽器焦點／輸入中的新文字可能在 await 中變動，未實作競態序號保護，屬後續需求而不是此切片已驗收。
## 3. 套件版本
原教學基底 version **0.2.19**；現行主線 `Cargo.toml` version **0.2.20**，edition 2024；`leptos = 0.8`、`leptos_router = 0.8`、`tokio = 1.53` 為 workspace 宣告版本（確切解析依原 SHA 的 Cargo.lock）。無新增 crate、無 migration。原教學曾使用 Linux rustc 1.99.1，但本次未重新執行 toolchain；MSRV 與其他 OS 未驗證。
## 4. 驗證項目
```bash
cargo test --locked -p ui --features ssr day20
cargo test --locked -p ui --features ssr
cargo check --locked -p ui --features hydrate --target wasm32-unknown-unknown
```
預期：Day20 兩項 Green tests 均通過；成功刷新顯示全部 rows、清空 query；刷新失敗保留舊清單與 query。**本次沒有在本地重跑以上命令**。目前 `c92831e105fb0000860f995ea288e29631b5fed8` 已有 Day20 兩項測試、helper 與事件接線；[最新 Test All/test-ui](https://github.com/williechen/rust_ai_chat/actions/runs/39202728570) 成功且涵蓋已合入的 Day20 程式，[Rust AI Chat check/build](https://github.com/williechen/rust_ai_chat/actions/runs/39202728449) 亦成功。原教學基底 `0cd9f799…` 的測試與 CI 僅是歷史證據；新的瀏覽器全流程仍未執行；完整瀏覽器情境「搜尋 Rust → 全量刷新成功 → 顯示 Rust+Pet」、「搜尋 Rust → 503 → 保留 Rust」仍待驗證。Day20 程式已套用 `rust_ai_chat/main`；使用者環境部署、跨服務產品整合與功能驗收仍未完成，三者不得混為一談。
