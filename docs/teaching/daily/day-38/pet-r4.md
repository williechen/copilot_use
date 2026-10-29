# Pet Day38 r4｜多隻生活窗各自漫遊：逐 UUID 啟動、取代舊任務並停止固定 pet label

> **2026-10-26 r4修訂**。產品唯讀來源 `9b89656f964b99e0cb89ba421855704a35c5ecb9`。功能Green與本案Refactor已在完整隔離候選驗證；產品checkout未套用。Red類型、測試／DB結果、環境失敗與未驗收項目分列於④及證據檔。

本修訂保留[原版](pet.md)，以當前精確來源提供[功能Green完整patch](pet-r2.patch)。核心60分鐘只計②主Red／Green；同篇本案Refactor獨立段落與其餘作業另計。這是同Day修訂，next_day維持39。

從未改過的來源使用完整patch；若先手動加入Red測試，依Green正文補程式，不重複套入已含測試的patch。來源有本地變更先審查差異，不覆寫。完整patch的SHA256：`3272c80e9f32df8edefebee31dff3f5f60c7fa8fadb5dd55183b02fb4ffe56d7`。

規則精確來源：copilot_use `1e6b8dcfa49d08b4a0af13ee4b602ac7664d2a64`，共用規則blob `34a2847c8628388f5bdb47c64a5f613b80ed3606`；本案Refactor與②Red／Green並列，功能程式與patch沿用r2精確來源。

## ① 功能、上一篇缺口與正式呼叫鏈

[Day37 r3](../day-37/pet-r3.md) 已在最新來源看到 `open_saved_pet` 的先保存再開窗、失敗補償與生命週期 lock；本日**不重建**那一段。現行 `pet_apps/pet_server/src/lib.rs::application().setup` 只在啟動時 `PET_ENABLE_ROAM` 為 true 就啟一個 `roam_desktop(app)`，而 `pet_apps/pet_server/src/roam.rs::roaming_window_label()` 永遠回 `"pet"`。真實生活窗由 `pet_windows::label` 產生 `pet-{UUID.simple()}`；因此即使重啟載入多隻生活窗，漫遊工作仍找不到窗口。Day18 舊測試還在斷言固定 `"pet"`，應遷移而非持續維護錯誤實作。

**本篇最小功能切片**：只要全域 opt-in `PET_ENABLE_ROAM=true/1`，每次成功建立一隻新的 `pet-{UUID}` 生活窗，便為該窗啟動**唯一**漫遊任務；同 ID 關閉又重開時替換舊 token，舊任務於取消檢查點退出；已在途的位移可能完成，非原子停移；關閉後任務自然退出，不能移動 `main` 控制窗。既有邊界反彈／意外外部位移／停止判斷不改。**逐隻持久化漫遊開關、坐下／拖曳暫停、跨螢幕回程、直接關窗 DB 保存**尚未完成，不能把這次全域 opt-in 當逐寵物政策完成。

真鏈：`pet_ui` Leptos CSR 的 `open_saved_pet` IPC／控制窗選取 → `pet_server::pet_windows::open_saved_pet` → `PetStore`／`pet_persistence` SQLite → `pet_windows::open` 建立 UUID Tauri 原生生活窗 → `roam::spawn_for_pet` → `roam_desktop` 對**同一 label**位移。啟動時 `store.uncaged()` 也經既有 `pet_windows::open`，所以無須第二套 startup 漫遊路徑。UI／DB／Pet Domain 原樣重用，不預建空 Application Port。

## Refactor

### Pet：開窗補償流程移入Application

本段只處理Pet自身，與②Red／Green並列，不以當日功能切片或Green完成作為重構邊界。把原pet_windows內的persist_then_open移入pet-application::lifecycle，由既有正式open_saved_pet caller直接import呼叫；closure參數維持原持久化／開窗邊界，不把Tauri或SQLx型別帶入Application。全域lifecycle mutex、Runtime、視窗與正式IPC仍留在server，不能以fake取代真鎖測試。

### 完整變更

精確產品來源：`9b89656f964b99e0cb89ba421855704a35c5ecb9`。下列patch包含必要的新模組、完整程式／測試與正式caller接線，不是只有目錄或介面骨架。沒有新增Cargo依賴、features、lock、migration或全庫workflow。

[下載原始patch](pet-r4-refactor.patch)（SHA256：`e539b8702b9b0171a92be047ab8c4e9f01b4fb99889e9952b3ea6fdc46588dfb`）。請使用原始檔以保留既有CRLF；先檢查再套用，不覆寫本地變更。

```diff
--- a/pet_apps/pet_application/src/lib.rs
+++ b/pet_apps/pet_application/src/lib.rs
@@ -1,3 +1,4 @@
+pub mod lifecycle;
 /// 建立寵物的輸入規則；不持有 Tauri、SQLx 或 OS driver。
 pub fn normalize_new_pet_name(input: &str) -> Result<String, String> {
     // 必須在 trim 前檢查；避免尾端換行被當一般空白悄悄吃掉。
--- /dev/null
+++ b/pet_apps/pet_application/src/lifecycle.rs
@@ -0,0 +1,118 @@
+/// Persistence and window closures are the boundary; no Tauri/SQLx types enter Application.
+pub async fn persist_then_open<W, F, O>(
+    old_caged: bool,
+    mut write: W,
+    open_window: O,
+) -> Result<(), String>
+where
+    W: FnMut(bool) -> F,
+    F: std::future::Future<Output = Result<(), String>>,
+    O: FnOnce() -> Result<(), String>,
+{
+    write(false).await?;
+    if let Err(error) = open_window() {
+        return match write(old_caged).await {
+            Ok(()) => Err(format!("開窗失敗，已恢復原回籠值：{error}")),
+            Err(rollback) => Err(format!("開窗失敗：{error}；DB 補償失敗：{rollback}")),
+        };
+    }
+    Ok(())
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use std::{
+        cell::{Cell, RefCell},
+        future::{Future, ready},
+        task::{Context, Poll, Waker},
+    };
+    // These fixtures use ready futures only; fail explicitly on Pending rather than pretending
+    // this is a production executor.
+    fn complete_ready<F: Future>(future: F) -> F::Output {
+        let mut future = std::pin::pin!(future);
+        match future
+            .as_mut()
+            .poll(&mut Context::from_waker(Waker::noop()))
+        {
+            Poll::Ready(value) => value,
+            Poll::Pending => panic!("fixture unexpectedly yielded"),
+        }
+    }
+    #[test]
+    fn persist_failure_never_opens_or_closes_a_window() {
+        complete_ready(async {
+            let opened = Cell::new(false);
+            let writes = RefCell::new(Vec::new());
+            let result = persist_then_open(
+                true,
+                |flag| {
+                    writes.borrow_mut().push(flag);
+                    ready(Err("DB".into()))
+                },
+                || {
+                    opened.set(true);
+                    Ok(())
+                },
+            )
+            .await;
+            assert!(result.is_err());
+            assert!(!opened.get());
+            assert_eq!(*writes.borrow(), vec![false]);
+        });
+    }
+
+    #[test]
+    fn open_failure_restores_the_actual_old_flag() {
+        complete_ready(async {
+            for old in [true, false] {
+                let writes = RefCell::new(Vec::new());
+                let result = persist_then_open(
+                    old,
+                    |flag| {
+                        writes.borrow_mut().push(flag);
+                        ready(Ok(()))
+                    },
+                    || Err("native".into()),
+                )
+                .await;
+                assert!(result.unwrap_err().contains("native"));
+                assert_eq!(*writes.borrow(), vec![false, old]);
+            }
+        });
+    }
+
+    #[test]
+    fn success_writes_once_and_rollback_error_is_visible() {
+        complete_ready(async {
+            let writes = RefCell::new(Vec::new());
+            persist_then_open(
+                true,
+                |flag| {
+                    writes.borrow_mut().push(flag);
+                    ready(Ok(()))
+                },
+                || Ok(()),
+            )
+            .await
+            .unwrap();
+            assert_eq!(*writes.borrow(), vec![false]);
+            let calls = Cell::new(0);
+            let error = persist_then_open(
+                true,
+                |_| {
+                    calls.set(calls.get() + 1);
+                    ready(if calls.get() == 1 {
+                        Ok(())
+                    } else {
+                        Err("rollback".into())
+                    })
+                },
+                || Err("native".into()),
+            )
+            .await
+            .unwrap_err();
+            assert!(error.contains("native") && error.contains("rollback"));
+        });
+    }
+}
--- a/pet_apps/pet_server/src/pet_windows.rs
+++ b/pet_apps/pet_server/src/pet_windows.rs
@@ -1,3 +1,4 @@
+use pet_application::lifecycle::persist_then_open;
 use crate::pet_store::{PetStore, SavedPet};
 use pet_domain::{PetCommand, PetMachine, PetSnapshot};
 use std::{collections::HashMap, sync::Mutex};
@@ -11,25 +12,7 @@
     LIFECYCLE.lock().await
 }
 
-async fn persist_then_open<W, F, O>(
-    old_caged: bool,
-    mut write: W,
-    open_window: O,
-) -> Result<(), String>
-where
-    W: FnMut(bool) -> F,
-    F: std::future::Future<Output = Result<(), String>>,
-    O: FnOnce() -> Result<(), String>,
-{
-    write(false).await?;
-    if let Err(error) = open_window() {
-        return match write(old_caged).await {
-            Ok(()) => Err(format!("開窗失敗，已恢復原回籠值：{error}")),
-            Err(rollback) => Err(format!("開窗失敗：{error}；DB 補償失敗：{rollback}")),
-        };
-    }
-    Ok(())
-}
+
 
 #[derive(Default)]
 pub struct Runtime {
@@ -206,85 +189,11 @@
     }
 }
 
+
+
 #[cfg(test)]
-mod day37_open_tests {
+mod lifecycle_lock_tests {
     use super::*;
-    use std::{
-        cell::{Cell, RefCell},
-        future::ready,
-    };
-
-    #[tokio::test]
-    async fn persist_failure_never_opens_or_closes_a_window() {
-        let opened = Cell::new(false);
-        let writes = RefCell::new(Vec::new());
-        let result = persist_then_open(
-            true,
-            |flag| {
-                writes.borrow_mut().push(flag);
-                ready(Err("DB".into()))
-            },
-            || {
-                opened.set(true);
-                Ok(())
-            },
-        )
-        .await;
-        assert!(result.is_err());
-        assert!(!opened.get());
-        assert_eq!(*writes.borrow(), vec![false]);
-    }
-
-    #[tokio::test]
-    async fn open_failure_restores_the_actual_old_flag() {
-        for old in [true, false] {
-            let writes = RefCell::new(Vec::new());
-            let result = persist_then_open(
-                old,
-                |flag| {
-                    writes.borrow_mut().push(flag);
-                    ready(Ok(()))
-                },
-                || Err("native".into()),
-            )
-            .await;
-            assert!(result.unwrap_err().contains("native"));
-            assert_eq!(*writes.borrow(), vec![false, old]);
-        }
-    }
-
-    #[tokio::test]
-    async fn success_writes_once_and_rollback_error_is_visible() {
-        let writes = RefCell::new(Vec::new());
-        persist_then_open(
-            true,
-            |flag| {
-                writes.borrow_mut().push(flag);
-                ready(Ok(()))
-            },
-            || Ok(()),
-        )
-        .await
-        .unwrap();
-        assert_eq!(*writes.borrow(), vec![false]);
-        let calls = Cell::new(0);
-        let error = persist_then_open(
-            true,
-            |_| {
-                calls.set(calls.get() + 1);
-                ready(if calls.get() == 1 {
-                    Ok(())
-                } else {
-                    Err("rollback".into())
-                })
-            },
-            || Err("native".into()),
-        )
-        .await
-        .unwrap_err();
-        assert!(error.contains("native") && error.contains("rollback"));
-    }
-
     #[tokio::test]
     async fn lifecycle_lock_excludes_a_second_operation() {
         let first = lifecycle_guard().await;
```

### 舊斷言如何承接

| 原測試／責任 | 新位置與保留內容 |
| --- | --- |
| 3 former day37_open_tests pure tests | moved byte-equivalent assertions to pet_application::lifecycle::tests, ready fixture instead of tokio runtime |
| lifecycle_lock_excludes_a_second_operation | retained on real server LIFECYCLE mutex; not moved to fake |
| Runtime/UUID/SQLite/IPC tests | unchanged; new Application tests do not replace native/DB tests |

不同故障模式與跨層接線保留，不以測試數減少當覆蓋完成，也不把既有行為重構假稱新功能Red。

### 實際驗證與使用者回歸命令

完整隔離workspace以正式Cargo驗證：pet-application 4項、pet-server／pet-persistence 60項通過；原始來源加Red測試實際出現預期缺符號編譯Red。原生編譯先遇LLD Bus error，bfd重試確認tmpfs配額不足，搬移本次自建快取後通過。真實生活窗GUI未驗收；沒有修改產品linker設定。

本案patch已在精確來源通過git apply --check及完整修改檔的Rust語法解析。也已在隔離快照實際確認本案Refactor與②功能Green patch可按兩種順序套用；不必假設Green先完成。差異相容性檢查本身不代表編譯；本次另在完整隔離合併候選執行下述Cargo與DB驗證。

```bash
# 在rust_ai_ring儲存庫根目錄檢查乾淨基準；套用由使用者執行。
git apply --check /path/to/pet-r4-refactor.patch
# 套用後在使用者環境重跑；本次相應命令已在隔離候選執行，非原產品checkout。
cd pet_apps
cargo test --locked -p pet-application
cargo test --locked -p pet-server
cargo test --locked -p pet-persistence
```

[逐檔來源blob、斷言對照及patch檢查](pet-r4-refactor-evidence.json) · [候選測試結果與範圍](r4-refactor-tests.json)。[完整本地Red／Green與workspace結果](r4-local-validation.json)。所有結果只屬隔離合併候選內容；rust_ai_ring保持唯讀，產品未套用，瀏覽器／GUI與正式CI未驗收。若失敗或patch來源漂移，保留既有程式，重核差異再套用，不把局部通過追認完整產品。

### 下一步

後續依Pet自己的回籠、Runtime／逐ID鎖、SQLite與視窗Adapter相依繼續搬用例與測試；全庫native／DB job隔離另列全專案項目。 本次有實際程式／測試變更，仍不是本案全部重構完成；依本案累積進度接續，不新增Day、不結案FIFO、不更新last_full_comparison。

## ② 功能實作：Red → 完整 Green

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

### Green 回歸檢查：測試與依賴

| 舊入口與測試 | 本次處理 | 有效斷言去向 |
| --- | --- | --- |
| `roam::tests::day18_roaming_selects_pet_life_window` | 確認舊固定 label 契約已失效，替換 | Day38 UUID label 測試，不能仍期待 `"pet"` |
| `roam::tests` 其餘位移／工作區測試 | 全部保留 | 越界、單軸、外部拖移、停滯與極值 |
| `roam_setting_tests` | 保留 | `PET_ENABLE_ROAM` opt-in，不誤認已持久逐寵物 |
| 新任務註冊／取代測試 | 新增 | 相同 ID 重開取消舊任務、不同 ID 隔離、清理不誤刪 |
| 原生三窗與重啟 GUI | 必須另驗；未執行 | 真實 Tauri 生活窗可移動、控制窗不移動、回籠停止 |

四 workspace Cargo.toml/Cargo.lock 已核對；只改 `pet_apps/pet_server/src/{roam,pet_windows,lib}.rs`，Cargo package `pet-server`、crate `pet_server`，`pet-persistence` 已有真 caller，`pet-application` 名稱規則已接通，無需修改。現有 `tauri=2.12`、`tokio=1.54/full`、`uuid=1.29/v4` 與 `std::sync` 足夠，**不新增 Cargo normal/build/dev/target dependencies、features、imports 以外的檔案、migrations、Leptos metadata、CI workflow**。Pet UI `pet-ui` 仍是 Leptos CSR + Tauri；其 Rust imports 不變。

## ③ 當日官方穩定版本與採用判斷

2026-10-26 台北日期，crates/Rust於 `2026-10-25T23:01:56.157211+00:00` 重查；排除預發行與 yanked。[完整宣告／lock／官方網址／原始回應SHA](r2-versions.json)。來源9b與648外部宣告及resolved一致；Chat／Pet／Analytics三workspace為0.2.37，共用crates維持0.2.35。

| 套件／工具 | manifest 宣告 | lock resolved／已安裝 | 官方穩定版／MSRV |
| --- | --- | --- | --- |
| tauri | `2.12` | 2.12.5 | [2.12.5](https://crates.io/crates/tauri/2.12.5)／1.90 |
| tauri-build | `2.7` | 2.7.4 | [2.7.4](https://crates.io/crates/tauri-build/2.7.4)／1.90 |
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
| Red | 新函式缺符號；舊固定 `pet` label 找不到真生活窗 | 本次已觀察預期缺符號編譯Red；固定label為來源缺口 |
| Green／原位移回歸 | Day38 task與本案既有原生／資料層回歸 | server／persistence 60項通過；Application另4項通過 |
| Pet SQLite／CSR | 原有資料與UI範圍分列 | SQLite含於60項通過；CSR／真GUI未新跑 |
| 原生三窗 GUI | 各窗位移、取消與重開、控制窗不位移 | 未執行 |
| CI／Gate2 | 套用後重跑相關原生與UI驗證 | 未套用、未驗收 |

**上一 Day 補充：** Day37 先保存再開窗與補償已在本次來源；不把其 closure 測試當真 native fault injection。舊 Day18 固定 label 斷言是過時測試，只有在正式 caller 改成 UUID 後才移除；其餘斷言全保留。後續逐寵物持久漫遊、直接關窗 DB 同步與 UI 控制仍屬獨立切片。本篇不修改 rust_ai_ring、舊文章、`last_full_comparison`。

本r4已在完整精確來源的隔離副本套用本案Refactor與功能Green，實際完成④所列驗證。原產品checkout與遠端main未套用；未把候選測試追認為瀏覽器／GUI或GitHub Actions驗收。[完整本地證據](r4-local-validation.json)保留原始錯誤與恢復，未更新last_full_comparison。
