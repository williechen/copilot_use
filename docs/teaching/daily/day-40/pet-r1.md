# Pet Day40 r1｜直接關閉生活窗等同逐隻回籠

> 2026-10-26修訂。最新產品SHA `ff14ca6224f7c5a65c799bf4a82a743fa79c880c`；原教學與patch基底 `bb259aaf76542554a3a704d9c4f4a3a219d65cbf`。本r1更正套用問題並保存實際本地命令，原[Day40文章](pet.md)與patch原bytes保留。Day41、FIFO與last_full_comparison不變。

## 最新核對與閱讀方式

**最新程式只有部分套用：原生生活窗 CloseRequested 分支缺失，不能宣稱按X已回籠。** Runtime清理與label helper已見；三個舊測試在最新碼被刪除。

下面①／Refactor／②保留「從bb259aa重現原始缺口」的完整TDD教學；它們描述教學前狀態，不表示最新來源仍全部未實作。當次Red未重新執行，預期Red不能冒充實際失敗證據。實測結果與當前未完成項集中於④。[Day39後續勘誤](../day-39/pet-r3.md)及[歷史更正總表](historical-corrections.md)保留各SHA範圍。

## ① bb259aa 教學基底的目標與缺口｜ 目標、真實缺口與前置條件

控制窗 `main` 的 X 維持只隱藏；寵物生活窗 `pet-<uuid>` 按 X 必須與 `cage_saved_pet` 同樣先將該 UUID 寫入 SQLite 回籠標記，再銷毀自己的視窗，其他生活窗仍運作。DB 失敗不能假稱已回籠、不能直接消滅窗；destroy 失敗使用既有用例補償。現行 `pet_server/src/lib.rs` 的 `on_window_event` 只在 `Destroyed` 清 Runtime，未處理使用者按 X 前的 DB 寫入，導致重啟可能復活。這是 Day40 功能 Red；Day39 找回不重做。

正式呼叫鏈：Tauri `WindowEvent::CloseRequested` → UUID label guard → `pet_application::lifecycle::persist_then_close` → `PetStore::set_caged` → `pet_persistence::set_caged` → SQLite → native `window.destroy` → `WindowEvent::Destroyed` → `Runtime::remove`。Tauri handler 不傳入 Application 任何 Tauri 型別；先防止原生立即 close，再於 async task 取得原生命週期鎖。控制窗、選取及其他 UUID 不受影響。核心60分鐘僅②。

## Refactor

**獨立舊行為重構：集中 Runtime 銷毀時的漫遊任務停止**。Day38/39 已有逐 UUID `roam::stop_for_pet` 與 task registry；但舊 `Runtime::remove` 只移除 `PetMachine`，由 cage IPC 或 native Destroyed 呼叫時，漫遊 task 仍須等下一輪發現視窗消失才結束。把 stop 責任收進既有 `Runtime::remove`，與②新 X 行為獨立：即使不套 Green，原有 `cage_saved_pet`／`Destroyed` 也共用同一清理邊界。這不是當日 Green 後的小整理。

在 `pet_apps/pet_server/src/pet_windows.rs`，**完整替換** Runtime 的 `remove` 方法，保持原 Mutex 邊界、UUID 格式與原有 caller：

```rust
pub fn remove(&self, id: &str) {
    // Lifecycle cleanup belongs to Runtime, regardless of cage IPC or native Destroyed.
    if let Ok(name) = label(id) {
        crate::roam::stop_for_pet(&name);
    }
    if let Ok(mut pets) = self.machines.lock() {
        pets.remove(id);
    }
}
```

在 `pet_apps/pet_server/src/roam.rs` 既有 `mod tests` 內新增完整回歸測試，沿用 `register_task`／`finish_task` 與既有 task registry，不造假 native window：

```rust
    #[test]
    fn day40_runtime_cleanup_cancels_only_the_removed_pet() {
        let a = uuid::Uuid::new_v4().to_string();
        let b = uuid::Uuid::new_v4().to_string();
        let la = crate::pet_windows::label(&a).unwrap();
        let lb = crate::pet_windows::label(&b).unwrap();
        let aa = register_task(&la).unwrap();
        let bb = register_task(&lb).unwrap();
        let runtime = crate::pet_windows::Runtime::default();
        runtime.remove(&a);
        assert!(!aa.load(Ordering::Acquire));
        assert!(bb.load(Ordering::Acquire));
        stop_for_pet(&lb);
        finish_task(&la, &aa);
        finish_task(&lb, &bb);
    }

```

保留 Day24 多寵物隔離、Day28 回籠、Day36 重啟、Day38/39 task registry 及找回斷言；本包新增「移除 A 停 A、不停 B」，不刪原測試、不變更 `Cargo.toml`／lock／migrations／Tauri module。完整 [Pet Refactor patch](pet-day40-refactor.patch)。原版交付時尚未執行；本r1實測詳見④；下一個 Pet Refactor 應逐 ID 收斂 `PetStore` 與 Runtime 的恢復/回籠失敗模式，與跨案共用 CI 無關。

## ② Red → 完整最小 Green

### Red｜先加入 native label 契約與失敗場景

在 `pet_windows.rs::tests` 加入下列**完整**測試；新增 helper 前為編譯 Red。原生行為 Red 另以 GUI 測試：A/B 同時出籠，按 A 的 X，重啟後 A 不恢復而 B 保持；現行 source 的 Destroyed 只清 Runtime，DB 仍為未回籠，因此這是可重現的功能 Red。DB 故障注入時按 X 必須保留窗，不得假裝成功。純 label 測試不能替代 GUI Red。

```rust
    #[test]
    fn day40_close_event_accepts_only_uuid_pet_windows() {
        let id = uuid::Uuid::new_v4();
        assert_eq!(
            pet_id_from_window_label(&format!("pet-{}", id.simple())),
            Some(id.to_string())
        );
        assert_eq!(pet_id_from_window_label("main"), None);
        assert_eq!(pet_id_from_window_label("pet-invalid"), None);
        assert_eq!(pet_id_from_window_label("pet-../../x"), None);
    }

```

### Green｜正式 Tauri handler 與既有 Application 用例

完整 [Pet Green patch](pet-day40-green.patch) 修改 `pet_windows.rs` 與 `pet_server/src/lib.rs`，不建立第二套產品。

1. 在 `pet_windows.rs` 的 `label` 後新增完整反解析 helper，僅接受合法 UUID；現有 `Runtime`、`SavedPet` 與 `open` 不改：

```rust
pub(crate) fn pet_id_from_window_label(window_label: &str) -> Option<String> {
    window_label
        .strip_prefix("pet-")
        .and_then(|raw| uuid::Uuid::parse_str(raw).ok())
        .map(|id| id.to_string())
}

```

2. 將 `cage_saved_pet` 的既有 `persist_then_close` closure 中 `Some(window) => window.close().map_err(|e| e.to_string())` **精確替換**為 `Some(window) => window.destroy().map_err(|e| e.to_string())`。原因：已在 IPC 內先持久化，不能再進入攔截 X 的 `CloseRequested` 造成重複寫入。其餘完整 command、`lifecycle_guard`、profile guard、`Runtime.remove` 保持。

3. 在 `pet_server/src/lib.rs::application` 的 `.on_window_event` 保留 `main` 既有 hide 分支；以以下**完整區塊**取代舊 `Destroyed` only 區塊：

```rust
            // Native X/close is the same cage transaction as the explicit IPC command.
            // The control window keeps its existing hide-on-close behavior above.
            if let Some(id) = pet_windows::pet_id_from_window_label(window.label()) {
                match event {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        let window = window.clone();
                        let store = window
                            .app_handle()
                            .state::<pet_store::PetStore>()
                            .inner()
                            .clone();
                        tauri::async_runtime::spawn(async move {
                            let _operation = pet_windows::lifecycle_guard().await;
                            if let Err(error) = pet_application::lifecycle::persist_then_close(
                                |flag| store.set_caged(&id, flag),
                                || window.destroy().map_err(|e| e.to_string()),
                            ).await {
                                // DB failure must leave the window alive; never silently claim cage.
                                eprintln!("生活窗直接關閉／回籠失敗：{error}");
                            }
                        });
                    }
                    tauri::WindowEvent::Destroyed => {
                        window.app_handle().state::<pet_windows::Runtime>().remove(&id);
                    }
                    _ => {}
                }
            }
```

使用既有 `pet-application`／`pet-persistence` 依賴與 `tauri::Manager`、`State`、`tokio`；不新增 Cargo feature、imports、migration 或 Tauri command。正式呼叫者是 native window event，非假 helper。任何寫入失敗保持窗存在，log 原始錯誤；若 destroy 失敗，`persist_then_close` 會回寫 false 並回報雙重失敗。控制窗仍只 hide；其他 UUID 的 DB／Runtime／roam 不動。

### 斷言承接與驗證命令

```bash
# rust_ai_ring repository 根目錄；僅在使用者自己的隔離工作樹
git apply --check /path/to/pet-day40-refactor.patch
git apply /path/to/pet-day40-refactor.patch
cd pet_apps
cargo test --locked -p pet-server day40_runtime_cleanup
cd ..
git apply --check /path/to/pet-day40-green.patch
git apply /path/to/pet-day40-green.patch
cd pet_apps
cargo fmt --all -- --check
cargo test --locked -p pet-application
cargo test --locked -p pet-persistence
cargo test --locked -p pet-server day40_close_event
cargo check --locked -p pet-server
cargo check --locked -p pet-ui --target wasm32-unknown-unknown
node --test pet_ui/tests/*.test.mjs
```

Native GUI：同時出籠 A/B/C，按 A 的 X → SQLite A=true，B/C=false、B/C 仍活動；重啟 A 不恢復、B/C 恢復；回籠 IPC 與 X 均不可雙重觸發；SQLite 唯讀／鎖定失敗時 A 窗保留且錯誤可追；控制窗 X 只 hide，重開聚焦既有控制窗。多次快速 X 與程式退出的競態需額外驗證。未跑 GUI 不得宣稱此切片驗收。回復順序：先 Green 逆套，再 Refactor 逆套，避免半套 native lifecycle。


### 最新程式缺口與可審閱修復附件

最新 `ff14ca6` 的 `application` 仍只在生活窗 `Destroyed` 清Runtime，沒有生活窗 `CloseRequested` 的prevent_close／DB先寫入；因此helper測試通過也不能證明按X回籠。原始Pet patch在bb基底確實包含完整native分支且實際可套用；最新碼與候選不是同一份結果。

此外，最新碼移除了 `day24_more_than_two_pet_states_are_independent_and_caging_is_local`、`day24_window_labels_are_derived_from_each_uuid_not_a_two_window_limit`、`day39_find_position_respects_primary_work_area_and_negative_origins`，沒有測試遷移證據。不能用降低測試數換全綠，r1保留此缺口。

[以ff14ca6為基底的補接與舊斷言恢復patch](pet-day40-followup-ff14ca6.patch)只補真native事件與恢復上述三個測試；已在額外隔離worktree檢查／套用，原始產品未改。其編譯與測試結果另見④，仍不代替真GUI。

來源：[`pet_apps/pet_server/src/pet_windows.rs::pet_id_from_window_label`](https://github.com/williechen/rust_ai_ring/blob/ff14ca6224f7c5a65c799bf4a82a743fa79c880c/pet_apps/pet_server/src/pet_windows.rs#L50)。

```rust
pub(crate) fn pet_id_from_window_label(window_label: &str) -> Option<String> {
    window_label
        .strip_prefix("pet-")
        .and_then(|raw| uuid::Uuid::parse_str(raw).ok())
        .map(|id| id.to_string())
}
```

## ③ 當日版本與實機核對

Rust／Cargo實測1.99.1，WASM target已安裝；PostgreSQL實測18.6，GTK／WebKit依賴實測見[工具log](local-validation/commands.jsonl)。以下為2026-10-26重新核對的官方穩定頁與當次來源lock；不升級依賴或改Cargo.lock。

| 技術 | 官方核對 |
| --- | --- |
| Rust | [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) |
| Leptos | [0.8.22](https://docs.rs/crate/leptos/latest) |
| SQLx | [0.9.0](https://docs.rs/crate/sqlx/latest) |
| Tokio | [1.54.1](https://docs.rs/crate/tokio/latest) |
| UUID | [1.29.0](https://docs.rs/crate/uuid/latest) |
| Tauri | [2.12.5](https://docs.rs/crate/tauri/latest) |

原版的「本機未讀」已由本次工具實測補正；未做release產物、完整MSRV矩陣或DB升級，不能由穩定版metadata推定它們通過。

## ④ 真正本地驗證與未完成

本次結果分為下列來源；**最新碼失敗與未接通項不由候選成功抵消**。

| 範圍／命令 | exit | 結果／log |
| --- | --- | --- |
| candidate-pet-fmt：`cargo fmt --all -- --check` | 1 |  [log](local-validation/day40-logs/candidate-pet-fmt.log) |
| candidate-pet-application：`cargo test --offline --locked -p pet-application` | 0 | 8passed/0failed/0ignored；0passed/0failed/0ignored [log](local-validation/day40-logs/candidate-pet-application.log) |
| candidate-pet-sqlite：`cargo test --offline --locked -p pet-persistence` | 0 | 4passed/0failed/0ignored；0passed/0failed/0ignored [log](local-validation/day40-logs/candidate-pet-sqlite.log) |
| candidate-pet-check：`cargo check --offline --locked -p pet-server` | 0 |  [log](local-validation/day40-logs/candidate-pet-check.log) |
| candidate-pet-test：`cargo test --offline --locked -p pet-server` | 0 | 39passed/0failed/0ignored；0passed/0failed/0ignored；5passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；5passed/0failed/0ignored；4passed/0failed/0ignored；4passed/0failed/0ignored；0passed/0failed/0ignored [log](local-validation/day40-logs/candidate-pet-test.log) |
| candidate-pet-wasm：`cargo check --offline --locked -p pet-ui --target wasm32-unknown-unknown` | 0 |  [log](local-validation/day40-logs/candidate-pet-wasm.log) |
| candidate-pet-node：`node --test pet_ui/tests/day33-open-controls-selects-origin.test.mjs pet_ui/tests/duplicate-preview-contract.test.mjs pet_ui/tests/scan-preview-contract.test.mjs pet_ui/tests/scan-preview-no-path.test.mjs` | 0 |  [log](local-validation/day40-logs/candidate-pet-node.log) |
| latest-pet-fmt：`cargo fmt --all -- --check` | 0 |  [log](local-validation/day40-logs/latest-pet-fmt.log) |
| latest-pet-application：`cargo test --offline --locked -p pet-application` | 0 | 8passed/0failed/0ignored；0passed/0failed/0ignored [log](local-validation/day40-logs/latest-pet-application.log) |
| latest-pet-sqlite：`cargo test --offline --locked -p pet-persistence` | 0 | 4passed/0failed/0ignored；0passed/0failed/0ignored [log](local-validation/day40-logs/latest-pet-sqlite.log) |
| latest-pet-check：`cargo check --offline --locked -p pet-server` | 0 |  [log](local-validation/day40-logs/latest-pet-check.log) |
| latest-pet-test：`cargo test --offline --locked -p pet-server` | 0 | 36passed/0failed/0ignored；0passed/0failed/0ignored；5passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；5passed/0failed/0ignored；4passed/0failed/0ignored；4passed/0failed/0ignored；0passed/0failed/0ignored [log](local-validation/day40-logs/latest-pet-test.log) |
| latest-pet-wasm：`cargo check --offline --locked -p pet-ui --target wasm32-unknown-unknown` | 0 |  [log](local-validation/day40-logs/latest-pet-wasm.log) |
| latest-pet-node：`node --test pet_ui/tests/day33-open-controls-selects-origin.test.mjs pet_ui/tests/duplicate-preview-contract.test.mjs pet_ui/tests/scan-preview-contract.test.mjs pet_ui/tests/scan-preview-no-path.test.mjs` | 0 |  [log](local-validation/day40-logs/latest-pet-node.log) |
| followup-pet-fmt：`cargo fmt --all -- --check` | 1 |  [log](local-validation/day40-logs/followup-pet-fmt.log) |
| followup-pet-check：`cargo check --offline --locked -p pet-server` | 0 |  [log](local-validation/day40-logs/followup-pet-check.log) |
| followup-pet-test：`cargo test --offline --locked -p pet-server` | 0 | 39passed/0failed/0ignored；0passed/0failed/0ignored；5passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；5passed/0failed/0ignored；4passed/0failed/0ignored；4passed/0failed/0ignored；0passed/0failed/0ignored [log](local-validation/day40-logs/followup-pet-test.log) |

每條命令的精確cwd、時間、環境覆寫與完整原始錯誤見[JSONL](local-validation/commands.jsonl)；[兩組範圍及結果](local-validation/README.md)。Cargo使用--offline --locked，沒有改lock或升級依賴。

最新來源的X持久化缺口、三個舊斷言移除均未在產品修復。補接附件仍需使用者套用；真多窗、SQLite失敗、destroy失敗、重啟與快速X競態尚未執行GUI。

GitHub文件提交與原始程式套用分開；本輪僅更新產品checkout至遠端SHA，驗證皆在隔離worktree，沒有提交／推送產品程式。原始文章與patch保留，靜態hunk檢查不追認為當時編譯成功。
