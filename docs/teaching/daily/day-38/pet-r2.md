# Pet Day38 r2｜多隻生活窗各自漫遊：逐 UUID 啟動、取代舊任務並停止固定 pet label

> **2026-10-26 教學交付**。唯一程式來源 [rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9](https://github.com/williechen/rust_ai_ring/tree/9b89656f964b99e0cb89ba421855704a35c5ecb9)；產品唯讀，程式由使用者套用。以下 Red／Green／原生 GUI／CI 均**尚未執行**；文件交付不是產品驗收。沿用 [合併後共用規則](../../../COMMON_LEARNING_RULES.md)。

本修訂保留[原版](pet.md)，以當前精確來源提供[完整可套用patch](pet-r2.patch)。核心60分鐘只計②主Red／Green；局部Refactor、工程與其他交付另計。這是同Day修訂，next_day維持39。

從未改過的來源使用完整patch；若先手動加入Red測試，依Green正文補程式，不重複套入已含測試的patch。來源有本地變更先審查差異，不覆寫。完整patch的SHA256：`3272c80e9f32df8edefebee31dff3f5f60c7fa8fadb5dd55183b02fb4ffe56d7`。

規則精確來源：copilot_use `43b0c4f4eb5d6b65d6d3155a473b2a673d944046`，共用規則 blob `420147c5202bc2dd44fe44a4bb83da2c1d91d5cf`。

## ① 功能、上一篇缺口與正式呼叫鏈

[Day37 r3](../day-37/pet-r3.md) 已在最新來源看到 `open_saved_pet` 的先保存再開窗、失敗補償與生命週期 lock；本日**不重建**那一段。現行 `pet_apps/pet_server/src/lib.rs::application().setup` 只在啟動時 `PET_ENABLE_ROAM` 為 true 就啟一個 `roam_desktop(app)`，而 `pet_apps/pet_server/src/roam.rs::roaming_window_label()` 永遠回 `"pet"`。真實生活窗由 `pet_windows::label` 產生 `pet-{UUID.simple()}`；因此即使重啟載入多隻生活窗，漫遊工作仍找不到窗口。Day18 舊測試還在斷言固定 `"pet"`，應遷移而非持續維護錯誤實作。

**本篇最小功能切片**：只要全域 opt-in `PET_ENABLE_ROAM=true/1`，每次成功建立一隻新的 `pet-{UUID}` 生活窗，便為該窗啟動**唯一**漫遊任務；同 ID 關閉又重開時替換舊 token，舊任務於取消檢查點退出；已在途的位移可能完成，非原子停移；關閉後任務自然退出，不能移動 `main` 控制窗。既有邊界反彈／意外外部位移／停止判斷不改。**逐隻持久化漫遊開關、坐下／拖曳暫停、跨螢幕回程、直接關窗 DB 保存**尚未完成，不能把這次全域 opt-in 當逐寵物政策完成。

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

同檔**完整取代**原 `pub async fn roam_desktop(app: tauri::AppHandle)` 的**完整函式**：

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

### 局部 Refactor：本功能測試與依賴

| 舊入口與測試 | 本次處理 | 有效斷言去向 |
| --- | --- | --- |
| `roam::tests::day18_roaming_selects_pet_life_window` | 確認舊固定 label 契約已失效，替換 | Day38 UUID label 測試，不能仍期待 `"pet"` |
| `roam::tests` 其餘位移／工作區測試 | 全部保留 | 越界、單軸、外部拖移、停滯與極值 |
| `roam_setting_tests` | 保留 | `PET_ENABLE_ROAM` opt-in，不誤認已持久逐寵物 |
| 新任務註冊／取代測試 | 新增 | 相同 ID 重開取消舊任務、不同 ID 隔離、清理不誤刪 |
| 原生三窗與重啟 GUI | 必須另驗；未執行 | 真實 Tauri 生活窗可移動、控制窗不移動、回籠停止 |

四 workspace Cargo.toml/Cargo.lock 已核對；只改 `pet_apps/pet_server/src/{roam,pet_windows,lib}.rs`，Cargo package `pet-server`、crate `pet_server`，`pet-persistence` 已有真 caller，`pet-application` 名稱規則已接通，無需修改。現有 `tauri=2.12`、`tokio=1.54/full`、`uuid=1.29/v4` 與 `std::sync` 足夠，**不新增 Cargo normal/build/dev/target dependencies、features、imports 以外的檔案、migrations、Leptos metadata、CI workflow**。Pet UI `pet-ui` 仍是 Leptos CSR + Tauri；其 Rust imports 不變。

## ③ 當日官方穩定版本與採用判斷

2026-10-26 台北日期，crates/Rust於 `2026-10-25T23:01:56.157211+00:00` 重查；排除預發行與 yanked。[完整宣告／lock／官方網址／原始回應SHA](r2-versions.json)。來源9b與648外部宣告及resolved一致；9b內部package為0.2.37。

| 套件／工具 | manifest 宣告 | lock resolved／已安裝 | 官方穩定版／MSRV |
| --- | --- | --- | --- |
| tauri | `見package/target表` | 2.12.5 | [2.12.5](https://crates.io/crates/tauri/2.12.5)／1.90 |
| tauri-build | `見package/target表` | 2.7.4 | [2.7.4](https://crates.io/crates/tauri-build/2.7.4)／1.90 |
| leptos | `0.8` | 0.8.22 | [0.8.22](https://crates.io/crates/leptos/0.8.22)／1.88 |
| tokio | `1.54` | 1.54.1 | [1.54.1](https://crates.io/crates/tokio/1.54.1)／1.85 |
| uuid | `1.29` | 1.29.0 | [1.29.0](https://crates.io/crates/uuid/1.29.0)／1.89.0 |
| sqlx | `0.9` | 0.9.0 | [0.9.0](https://crates.io/crates/sqlx/0.9.0)／1.94.0 |
| serde | `1.0` | 1.0.229 | [1.0.229](https://crates.io/crates/serde/1.0.229)／1.56 |
| serde_json | `1.0` | 1.0.154 | [1.0.154](https://crates.io/crates/serde_json/1.0.154)／1.71 |

Rust官方stable 1.99.1，[官方channel](https://static.rust-lang.org/dist/channel-rust-stable.toml)，本機rustc/cargo同版。最高所列MSRV不高於本機Rust，但不等於SSR／WASM／native／DB建置通過。相符resolved維持；若有差異優先以最新穩定版驗證遷移，本包不改lock、安裝工具或升級。

工具／native：Tauri CLI已安裝2.12.3，官方2.12.4（MSRV1.90）；Trunk已安裝0.21.14，官方0.22.0（MSRV1.96.1），建議各自測試升級後固定相容版本，本包未升級。Linux WebKitGTK官方2.54.2／本機2.52.6；GTK3官方3.24.52；GLib官方2.90.1／本機2.88。SQLite官方3.54.1，CLI未裝；libsqlite3-sys resolved0.37.0對官方0.39.1，不能假稱bundled引擎已更新。
[Tauri官方Linux prerequisites](https://v2.tauri.app/start/prerequisites/) · [WebKitGTK](https://webkitgtk.org/) · [GTK3官方索引](https://download.gnome.org/sources/gtk/3.24/) · [GLib](https://download.gnome.org/sources/glib/)；須另驗WebKit/CSR資源、原生視窗、Wayland與X11能力，套件新版不保證視窗可程式移動。

資料庫／native當日官方來源、已安裝探測與採用理由見[版本詳細核對](../../../source-review/2026-10-26-versions-and-summary-refresh.md)，不能以metadata宣稱功能已驗收。
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
| CI／Gate2 | 套用後重跑相關原生與UI驗證 | 未套用、未驗收 |

**上一 Day 補充：** Day37 先保存再開窗與補償已在本次來源；不把其 closure 測試當真 native fault injection。舊 Day18 固定 label 斷言是過時測試，只有在正式 caller 改成 UUID 後才移除；其餘斷言全保留。後續逐寵物持久漫遊、直接關窗 DB 同步與 UI 控制仍屬獨立切片。本篇不修改 rust_ai_ring、舊文章、`last_full_comparison`。

本次新增靜態驗證：在隔離來源快照，`git apply --check`通過；完整修改檔以rustfmt解析通過。[逐檔來源blob與結果](r2-static-checks.json)。這兩項不代表編譯、Red失敗、Green通過、DB、native、GUI或CI；上述仍未執行。產品倉庫未套用本候選內容。
