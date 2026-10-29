# Pet Day38｜多隻生活窗各自漫遊：逐 UUID 啟動、取代舊任務並停止固定 pet label

> **2026-10-26 教學交付**。唯一程式來源 [rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9](https://github.com/williechen/rust_ai_ring/tree/9b89656f964b99e0cb89ba421855704a35c5ecb9)；產品唯讀，程式由使用者套用。以下 Red／Green／原生 GUI／CI 均**尚未執行**；文件交付不是產品驗收。沿用 `daily-tdd-minimum-v1`、`functional-layering-v1`、`test-refactor-v1`。

## ① 功能、上一篇缺口與正式呼叫鏈

[Day37 r3](../day-37/pet-r3.md) 已在最新來源看到 `open_saved_pet` 的先保存再開窗、失敗補償與生命週期 lock；本日**不重建**那一段。現行 `pet_apps/pet_server/src/lib.rs::application().setup` 只在啟動時 `PET_ENABLE_ROAM` 為 true 就啟一個 `roam_desktop(app)`，而 `pet_apps/pet_server/src/roam.rs::roaming_window_label()` 永遠回 `"pet"`。真實生活窗由 `pet_windows::label` 產生 `pet-{UUID.simple()}`；因此即使重啟載入多隻生活窗，漫遊工作仍找不到窗口。Day18 舊測試還在斷言固定 `"pet"`，應遷移而非持續維護錯誤實作。

**本篇最小功能切片**：只要全域 opt-in `PET_ENABLE_ROAM=true/1`，每次成功建立一隻新的 `pet-{UUID}` 生活窗，便為該窗啟動**唯一**漫遊任務；同 ID 關閉又重開時替換舊 token，舊任務不再移動新窗；關閉後任務自然退出，不能移動 `main` 控制窗。既有邊界反彈／意外外部位移／停止判斷不改。**逐隻持久化漫遊開關、坐下／拖曳暫停、跨螢幕回程、直接關窗 DB 保存**尚未完成，不能把這次全域 opt-in 當逐寵物政策完成。

真鏈：`pet_ui` Leptos CSR 的 `open_saved_pet` IPC／控制窗選取 → `pet_server::pet_windows::open_saved_pet` → `PetStore`／`pet_persistence` SQLite → `pet_windows::open` 建立 UUID Tauri 原生生活窗 → `roam::spawn_for_pet` → `roam_desktop` 對**同一 label**位移。啟動時 `store.uncaged()` 也經既有 `pet_windows::open`，所以無須第二套 startup 漫遊路徑。UI／DB／Pet Domain 原樣重用，不預建空 Application Port。

## ② Red → 完整最小 Green → Refactor

### Red：取代固定 label 的錯誤斷言，驗證每 ID 任務隔離

在 `pet_apps/pet_server/src/roam.rs` 的現有 `#[cfg(test)] mod tests` 中，**刪除只驗證舊錯誤契約**的 `day18_roaming_selects_pet_life_window`，以以下兩個完整測試替換；其它位移／邊界測試保留。新增的 `register_task`／`finish_task` 尚不存在，預期先為缺符號編譯 Red；舊 `"pet"` 斷言本來通過，不可宣稱它是新 Red。兩測試使用獨立 UUID 避免並行污染。

```rust
    #[test]
    fn day38_window_label_is_per_pet_not_control_or_fixed_pet() {
        let id = uuid::Uuid::new_v4().to_string();
        let label = crate::pet_windows::label(&id).unwrap();
        assert_eq!(label, format!("pet-{}", uuid::Uuid::parse_str(&id).unwrap().simple()));
        assert_ne!(label, "pet");
        assert_ne!(label, "main");
    }

    #[test]
    fn day38_reopen_cancels_only_previous_task_for_same_label() {
        use std::sync::atomic::Ordering;
        let a = format!("pet-{}", uuid::Uuid::new_v4().simple());
        let b = format!("pet-{}", uuid::Uuid::new_v4().simple());
        let first = register_task(&a).unwrap();
        let other = register_task(&b).unwrap();
        let second = register_task(&a).unwrap();
        assert!(!first.load(Ordering::Acquire));
        assert!(second.load(Ordering::Acquire));
        assert!(other.load(Ordering::Acquire));
        finish_task(&a, &first);
        assert!(roam_tasks().lock().unwrap().contains_key(&a));
        finish_task(&a, &second);
        finish_task(&b, &other);
        assert!(!roam_tasks().lock().unwrap().contains_key(&a));
        assert!(!roam_tasks().lock().unwrap().contains_key(&b));
    }
```

### Green A：在原生 Adapter 維持每個 label 的可取消任務

`pet_apps/pet_server/src/roam.rs`：保留 `use tauri::{Manager, PhysicalPosition, PhysicalRect, PhysicalSize};` 及所有 `bounded_next_position` 等既有函式。**刪除**原本 `PET_WINDOW_LABEL` 常數與 `roaming_window_label()` 函式，將下列 imports／狀態／函式加在檔案開頭 imports 之後：

```rust
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
};

fn roam_tasks() -> &'static Mutex<HashMap<String, Arc<AtomicBool>>> {
    static TASKS: OnceLock<Mutex<HashMap<String, Arc<AtomicBool>>>> = OnceLock::new();
    TASKS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn register_task(label: &str) -> Option<Arc<AtomicBool>> {
    let mut tasks = roam_tasks().lock().ok()?;
    let active = Arc::new(AtomicBool::new(true));
    if let Some(previous) = tasks.insert(label.to_owned(), active.clone()) {
        previous.store(false, Ordering::Release);
    }
    Some(active)
}

fn finish_task(label: &str, active: &Arc<AtomicBool>) {
    if let Ok(mut tasks) = roam_tasks().lock() {
        if tasks.get(label).is_some_and(|current| Arc::ptr_eq(current, active)) {
            tasks.remove(label);
        }
    }
}

pub(crate) fn spawn_for_pet(app: tauri::AppHandle, label: String) {
    if !crate::roam_enabled() {
        return;
    }
    let Some(active) = register_task(&label) else {
        eprintln!("desktop roaming unavailable: task registry poisoned");
        return;
    };
    tauri::async_runtime::spawn(async move {
        roam_desktop(app, &label, &active).await;
        finish_task(&label, &active);
    });
}
```

同檔**完整取代**原 `pub async fn roam_desktop(app: tauri::AppHandle)` 的**函式簽名與迴圈入口**：

```rust
async fn roam_desktop(app: tauri::AppHandle, label: &str, active: &AtomicBool) {
    let mut dx = ROAM_INITIAL_DX;
    let mut dy = ROAM_INITIAL_DY;
    let mut stalled_attempts = 0_u8;

    loop {
        if !active.load(Ordering::Acquire) {
            break;
        }
        let window = match app.get_webview_window(label) {
            Some(window) => window,
            None => {
                eprintln!("desktop roaming unavailable: pet window not found: {label}");
                break;
            }
        };
        // 原函式從 `let before = match window.outer_position()` 起
        // 到 `tokio::time::sleep(ROAM_INTERVAL).await;` 止，全部原樣保留。
```

上面是**定位用片段**而非整個函式的新替代品；為免只交付半段 Green，原函式的完整下半段如下。將它直接接在上面片段的註解後，然後閉合迴圈／函式：

```rust
        let before = match window.outer_position() {
            Ok(position) => position,
            Err(error) => { eprintln!("desktop roaming unavailable: {error}"); break; }
        };
        let window_size = match window.outer_size() {
            Ok(size) => size,
            Err(error) => { eprintln!("desktop roaming unavailable: {error}"); break; }
        };
        let monitor = match window.current_monitor() {
            Ok(Some(monitor)) => monitor,
            Ok(None) => { eprintln!("desktop roaming unavailable: no monitor found"); break; }
            Err(error) => { eprintln!("desktop roaming unavailable: {error}"); break; }
        };
        let requested =
            bounded_next_position(before, window_size, monitor.work_area(), &mut dx, &mut dy);
        // 取消後不可再送出新的位移；不持有 registry Mutex 跨 await。
        if !active.load(Ordering::Acquire) { break; }
        if let Err(error) = window.set_position(requested) {
            eprintln!("desktop roaming unavailable: {error}");
            break;
        }
        if no_roaming_space(dx, dy) {
            eprintln!("desktop roaming stopped: pet window exceeds available work area");
            break;
        }
        tokio::time::sleep(ROAM_VERIFY_INTERVAL).await;
        if !active.load(Ordering::Acquire) { break; }
        let after = match window.outer_position() {
            Ok(position) => position,
            Err(error) => {
                eprintln!("desktop roaming verification unavailable: {error}");
                break;
            }
        };
        if unexpected_displacement(requested, after) {
            eprintln!("desktop roaming stopped: unexpected window displacement");
            break;
        }
        if after == before {
            stalled_attempts += 1;
            if stalled_attempts >= ROAM_STALL_LIMIT {
                eprintln!(
                    "desktop roaming unavailable: window position did not change after                      {ROAM_STALL_LIMIT} attempts"
                );
                break;
            }
        } else {
            stalled_attempts = 0;
        }
        tokio::time::sleep(ROAM_INTERVAL).await;
    }
}
```

**注意：** `register_task`／`finish_task` 不複製任何業務狀態到 Domain；這是 Tauri 原生移動 task 的 OS adapter lifecycle。每個 label 的 token 只在任務結束時移除；舊任務結束時用 `Arc::ptr_eq` 防止清掉重開後的新 token。被取消的 task 可能已送出當下位移，因此本切片不承諾同步原子停移；真正拖曳暫停另設互動協定。

### Green B：接回真實建立／啟動呼叫端，移除舊全域入口

`pet_apps/pet_server/src/pet_windows.rs`：原 `fn label(id: &str) -> Result<String,String>` 改成 `pub(crate) fn label(...)`，**函式本體完全保留**。在 `pub fn open(app,pet)` 最後這段：

```rust
    if let Err(e) = result {
        machines.remove(&pet.id);
        return Err(e.to_string());
    }
    Ok(())
```

完整替換成：

```rust
    if let Err(e) = result {
        machines.remove(&pet.id);
        return Err(e.to_string());
    }
    // 僅真正建立新生活窗後啟動；既有窗 show/focus 早退不重複 spawn。
    crate::roam::spawn_for_pet(app.clone(), name);
    Ok(())
```

`pet_apps/pet_server/src/lib.rs`：將 `use crate::{chat::AiState, roam::roam_desktop};` 改成 `use crate::chat::AiState;`。在 `application().setup` 尾端**完整刪除**舊區塊：

```rust
            if roam_enabled() {
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    roam_desktop(app_handle).await;
                });
            }
```

`roam_enabled`／`roam_enabled_value` 保留供新 `spawn_for_pet` 與原測試使用。現在 startup 的 `store.uncaged() → pet_windows::open` 與手動 `open_saved_pet → pet_windows::open` 都使用同一任務入口；`select_pet` 的 `open` 亦不會為既有窗重啟任務。保留 `pet_windows` 既有生命週期 lock、SQLx、Runtime、200×200 Tauri 設定、IPC 註冊與 Leptos CSR 的真實 caller。不要移動 DB adapter 來湊齊目錄。

### Refactor：測試／依賴／CI 責任

| 舊入口與測試 | 本次處理 | 有效斷言去向 |
| --- | --- | --- |
| `roam::tests::day18_roaming_selects_pet_life_window` | 確認舊固定 label 契約已失效，替換 | Day38 UUID label 測試，不能仍期待 `"pet"` |
| `roam::tests` 其餘位移／工作區測試 | 全部保留 | 越界、單軸、外部拖移、停滯與極值 |
| `roam_setting_tests` | 保留 | `PET_ENABLE_ROAM` opt-in，不誤認已持久逐寵物 |
| 新任務註冊／取代測試 | 新增 | 相同 ID 重開取消舊任務、不同 ID 隔離、清理不誤刪 |
| 原生三窗與重啟 GUI | 必須另驗；未執行 | 真實 Tauri 生活窗可移動、控制窗不移動、回籠停止 |

四 workspace Cargo.toml/Cargo.lock 已核對；只改 `pet_apps/pet_server/src/{roam,pet_windows,lib}.rs`，Cargo package `pet-server`、crate `pet_server`，`pet-persistence` 已有真 caller，`pet-application` 名稱規則已接通，無需修改。現有 `tauri=2.12`、`tokio=1.54/full`、`uuid=1.29/v4` 與 `std::sync` 足夠，**不新增 Cargo normal/build/dev/target dependencies、features、imports 以外的檔案、migrations、Leptos metadata、CI workflow**。Pet UI `pet-ui` 仍是 Leptos CSR + Tauri；其 Rust imports 不變。

## ③ 交付日官方最新穩定版本

2026-10-26 05:35 台北時間，官方 crates.io API 已核對（[當日版本／lock 原始證據](../../../source-review/version-refresh-2026-10-26/evidence.json)）；最新 9b89656f964b99e0cb89ba421855704a35c5ecb9 直接依賴未變。[Rust 1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)（10/15）、[tauri 2.12.5](https://crates.io/crates/tauri/2.12.5)（MSRV 1.90）、[tauri-build 2.7.4](https://crates.io/crates/tauri-build/2.7.4)（1.90）、[tokio 1.54.1](https://crates.io/crates/tokio/1.54.1)（1.85）、[uuid 1.29.0](https://crates.io/crates/uuid/1.29.0)（1.89）、[sqlx 0.9.0](https://crates.io/crates/sqlx/0.9.0)（1.94）。Pet lock 與上述直接依賴已相符，不做升級。Linux Tauri 需 WebKitGTK 4.1、GTK3、libsoup3 等原生依賴；工具相容不能取代 GUI 實測。

## ④ 驗證命令、預期／實際與上一 Day 更正

```bash
cd rust_ai_ring/pet_apps
# 原生開發套件須先依 Tauri v2 官方 prerequisite 安裝。
cargo test --locked -p pet-server roam::tests
cargo test --locked -p pet-server
cargo test --locked -p pet-persistence
cargo check --locked -p pet-ui --target wasm32-unknown-unknown
# 另用 Tauri 真實桌面：PET_ENABLE_ROAM=1；建立／出籠三隻，
# 檢查每個 pet-{UUID} 自己位移、main 不移動、重複開窗不重複 task、
# 回籠／重開／重啟與手動拖曳情況，觀察原生 log。
```

| Gate | 預期 | 本次實際 |
| --- | --- | --- |
| Red | 新函式缺符號；舊固定 `pet` label 找不到真生活窗 | 未執行；後者為已知來源缺口 |
| Green／原位移回歸 | Day38 task 測試與原工作區測試皆通過 | 未執行 |
| Pet SQLite／CSR | 原有 persistence 與 Leptos UI 不回歸 | 未執行 |
| 原生三窗 GUI | 各窗位移、取消與重開、控制窗不位移 | 未執行 |
| CI／Gate2 | 原生 job 獨立，失敗不阻擋 Chat／Analytics DB job | 未改 workflow、未驗收 |

**上一 Day 補充：** Day37 先保存再開窗與補償已在本次來源；不把其 closure 測試當真 native fault injection。舊 Day18 固定 label 斷言是過時測試，只有在正式 caller 改成 UUID 後才移除；其餘斷言全保留。後續逐寵物持久漫遊、直接關窗 DB 同步與 UI 控制仍屬獨立切片。本篇不修改 rust_ai_ring、舊文章、`last_full_comparison`。
