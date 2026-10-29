# Pet Day40｜直接關閉生活窗等同逐隻回籠

> 2026-10-26；產品唯一來源 `williechen/rust_ai_ring/main@bb259aaf76542554a3a704d9c4f4a3a219d65cbf`；產品保持唯讀。接續 [Day39 r2 逐 UUID 找回](../day-39/pet-r2.md)，以最新已存在的 `persist_then_close`、`PetStore`、`Runtime` 與 Tauri native `on_window_event` 為基準。兩個 patch 順序為 Refactor → Green，使用者自行套用。

## ① 目標、真實缺口與前置條件

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

保留 Day24 多寵物隔離、Day28 回籠、Day36 重啟、Day38/39 task registry 及找回斷言；本包新增「移除 A 停 A、不停 B」，不刪原測試、不變更 `Cargo.toml`／lock／migrations／Tauri module。完整 [Pet Refactor patch](pet-day40-refactor.patch)。實際執行尚未完成；下一個 Pet Refactor 應逐 ID 收斂 `PetStore` 與 Runtime 的恢復/回籠失敗模式，與跨案共用 CI 無關。

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

## ③ 當日官方穩定版核對（2026-10-26）

| 技術 | Manifest／Lock | 官方最新穩定 | 採用決策 |
| --- | --- | --- | --- |
| Rust | edition 2024／本機未查 | 1.99.1（10/15） | native build 須核對實機 |
| Tauri | 2.12／2.12.5 | 2.12.5（10/21），3 alpha 非穩定 | `Window::destroy` 需真 GUI |
| Leptos | 0.8／0.8.22 | 0.8.22（10/05） | CSR 不改 |
| SQLx | 0.9／0.9.0 | 0.9.0（05/21） | SQLite schema 不改 |
| Tokio | 1.54／1.54.1 | 1.54.1（10/21） | 使用既有 spawn |
| UUID | 1.29／1.29.0 | 1.29.0（10/23） | 既有 simple label |

官方：[Rust](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) · [Tauri](https://docs.rs/crate/tauri/latest) · [Leptos](https://docs.rs/crate/leptos/latest) · [SQLx](https://docs.rs/crate/sqlx/latest) · [Tokio](https://docs.rs/crate/tokio/latest) · [UUID](https://docs.rs/crate/uuid/latest)。本次無新依賴／feature，未升級產品，native WebKit／MSRV／WASM 本機工具版本未讀。

## ④ 實際驗證與未完成

已核對精確來源、原生事件入口、既有 Application/SQLite caller、四 workspace 目錄與依賴，產生完整 patch 及 Red 測試。**未實際 `git apply`、Cargo、原生 GUI、SQLite 故障注入或 CI**，不得將靜態 patch 與 Day39 舊驗證當成 Day40 Green。文件是方案；使用者套用／產品 Gate2 另記。保留 Day39 r2、FIFO、有效斷言與 `last_full_comparison`。跨案契約／共用 crates／全庫 CI 僅列 [工程獨立項目](../../../engineering/README.md)，不在 Pet Refactor 計功。
