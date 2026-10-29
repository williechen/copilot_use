# Pet Day44｜逐隻設定「啟動時預設出籠」，回籠不再取消預設資格

> 2026-10-30（Asia/Taipei）｜產品唯讀 `williechen/rust_ai_ring@7ce39f0166a2120539e5908b33c88322a930c35a`；文件起點 `copilot_use@6bd009acfefb07293e7f3bbe777ea60450a52635`；expected `next_day.pet=44`。本篇是可套用的 Gate1 方案，**不是已套用產品或原生 GUI Gate2 驗收**。

## ① 目標、現況與使用者操作

Day43 已接通控制台逐 UUID「查詢回籠狀態」，但目前 `pet_profile.is_default_closure` 同時被當成「目前回籠」與「重啟自動出籠」：`pet_server::application::setup` 讀 `PetStore::uncaged`，所以按「回籠這隻」後，重啟也不再自動出籠，違反**回籠不取消預設資格**。本篇讓控制台每隻寵物可「查詢預設出籠」「設為啟動時出籠」「取消啟動時出籠」，結果由真正 Tauri IPC → SQLite 持久化並在重啟生效。可同時設定 A、B，回籠 A 不會改 A 的預設資格；啟動時 A、B 各自出籠；未設定者不自動出籠。UI 逐 UUID 顯示錯誤，失敗不冒充 false。

**相容策略**：新 migration `20261030060000_auto_release_preference.sql` 新增 `auto_release_on_start`，只在首次 migration 把**舊資料目前未回籠**者 backfill 為預設資格，以保留升級前的啟動集合；升級後新寵物預設 false，必須在控制台明確選取資格。原 `is_default_closure` 繼續只代表當下回籠／出籠。不要修改既有 migration checksum；實際舊 SQLite 備份升級仍須驗證。

正式路徑：`pet_ui/src/main.rs::App` → `bridge.rs` wasm_bindgen → `pet_ui/index.html` 的 `window.getPetAutoRelease/setPetAutoRelease` → Tauri `pet_server/src/lib.rs` commands → `PetStore` → `pet_persistence` SQLx → SQLite。啟動路徑：`application::setup` → `PetStore::auto_release_pets` → `persist_then_open`（先寫目前出籠狀態、失敗回滾）→ `pet_windows::open`（逐 UUID 原生窗）。原有出／入籠、選取、A/B Runtime 與原生 CloseRequested 流程不刪。

## Refactor｜Pet 原生「找回寵物」位置計算搬層

這是**與本日功能 Green 無依賴的獨立 Pet 重構**。原 `pet_server/src/pet_windows.rs` 混合 Tauri 視窗生命週期、Runtime、原生 show/focus、漫遊與純 `centered_position` 幾何運算。將幾何函式原樣移到 `pet_server/src/window_position.rs`，由 `lib.rs` 宣告 `mod window_position;`，原 `find_saved_pet` caller `use crate::window_position::centered_position;`；native 窗口、主螢幕工作區、漫遊啟停及錯誤傳遞**不變**。不碰 Chat／Analytics、共用 crates 或全庫 CI。

**有效斷言承接**：原 `pet_windows::tests::day39_find_position_respects_primary_work_area_and_negative_origins` 保留在原測試模組，繼續從正式 caller 看到同函式；原 Day24 多寵物、Day40 window label、Day43 menu UUID、lifecycle lock、原生 A/B 驗證責任保留。新模組補「工作區小於視窗」「負座標」純測試。逐檔完整 Refactor patch：

```diff
diff --git a/pet_apps/pet_server/src/lib.rs b/pet_apps/pet_server/src/lib.rs
--- a/pet_apps/pet_server/src/lib.rs
+++ b/pet_apps/pet_server/src/lib.rs
@@ -4,6 +4,7 @@
 pub mod pet_store;
 pub mod pet_windows;
 mod roam;
+mod window_position;
 use pet_application::normalize_new_pet_name;
 
 use ai_core::{ChatRequest, MockChatModel};
diff --git a/pet_apps/pet_server/src/pet_windows.rs b/pet_apps/pet_server/src/pet_windows.rs
--- a/pet_apps/pet_server/src/pet_windows.rs
+++ b/pet_apps/pet_server/src/pet_windows.rs
@@ -1,181 +1,170 @@
 use crate::pet_store::{PetStore, SavedPet};
-use pet_application::lifecycle::persist_then_open;
-use pet_domain::{PetCommand, PetMachine, PetSnapshot};
-use std::{collections::HashMap, sync::Mutex};
-use tauri::{Emitter, Manager, PhysicalPosition, PhysicalRect, PhysicalSize, State};
-
-// 產品目前單程序；全域序列化所有出／入籠、選取與啟動恢復。
-// 不持有 std::sync::MutexGuard 跨 await；Tauri command future 仍可 Send。
-static LIFECYCLE: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());
-
-pub(crate) async fn lifecycle_guard() -> tokio::sync::MutexGuard<'static, ()> {
-    LIFECYCLE.lock().await
-}
-
-#[derive(Default)]
-pub struct Runtime {
-    machines: Mutex<HashMap<String, PetMachine>>,
-}
-
-impl Runtime {
-    pub fn snapshot(&self, id: &str) -> Result<PetSnapshot, String> {
-        self.machines
-            .lock()
-            .map_err(|_| "寵物狀態無法使用")?
-            .get(id)
-            .map(PetMachine::snapshot)
-            .ok_or_else(|| "寵物尚未出籠".into())
-    }
-    pub fn dispatch(&self, id: &str, command: PetCommand) -> Result<PetSnapshot, String> {
-        let mut pets = self.machines.lock().map_err(|_| "寵物狀態無法使用")?;
-        let pet = pets.get_mut(id).ok_or("寵物尚未出籠")?;
-        pet.dispatch(command).map_err(|e| e.to_string())?;
-        Ok(pet.snapshot())
-    }
-    pub fn remove(&self, id: &str) {
-        // Lifecycle cleanup belongs to Runtime, regardless of cage IPC or native Destroyed.
-        if let Ok(name) = label(id) {
-            crate::roam::stop_for_pet(&name);
-        }
-        if let Ok(mut pets) = self.machines.lock() {
-            pets.remove(id);
-        }
-    }
-}
-
-pub(crate) fn label(id: &str) -> Result<String, String> {
-    let id = uuid::Uuid::parse_str(id).map_err(|_| "寵物ID格式錯誤")?;
-    Ok(format!("pet-{}", id.simple()))
-}
-
-pub(crate) fn pet_id_from_window_label(window_label: &str) -> Option<String> {
-    window_label
-        .strip_prefix("pet-")
-        .and_then(|raw| uuid::Uuid::parse_str(raw).ok())
-        .map(|id| id.to_string())
-}
-
-pub fn open(app: &tauri::AppHandle, pet: &SavedPet) -> Result<(), String> {
-    let name = label(&pet.id)?;
-    let runtime = app.state::<Runtime>();
-    // 同 ID 開窗序列化；重複出籠只 show/focus，不重新建立或重設狀態。
-    let mut machines = runtime.machines.lock().map_err(|_| "寵物狀態無法使用")?;
-    if let Some(window) = app.get_webview_window(&name) {
-        window.show().map_err(|e| e.to_string())?;
-        return window.set_focus().map_err(|e| e.to_string());
-    }
-    let count = machines.len();
-    machines.insert(pet.id.clone(), PetMachine::default());
-    let result = tauri::WebviewWindowBuilder::new(
-        app,
-        &name,
-        tauri::WebviewUrl::App(format!("index.html?view=pet&pet_id={}", pet.id).into()),
-    )
-    .title(&pet.name)
-    .inner_size(200.0, 200.0)
-    .position(
-        80.0 + (count % 4) as f64 * 220.0,
-        80.0 + (count / 4) as f64 * 220.0,
-    )
-    .resizable(false)
-    .decorations(false)
-    .transparent(true)
-    .always_on_top(true)
-    .shadow(false)
-    .build();
-    if let Err(e) = result {
-        machines.remove(&pet.id);
-        return Err(e.to_string());
-    }
-    // 僅真正建立新生活窗後啟動；既有窗 show/focus 早退不重複 spawn。
-    crate::roam::spawn_for_pet(app.clone(), name);
-    Ok(())
-}
-
-pub fn dispatch_and_emit(
-    app: &tauri::AppHandle,
-    id: &str,
-    command: PetCommand,
-) -> Result<PetSnapshot, String> {
-    let snapshot = app.state::<Runtime>().dispatch(id, command)?;
-    app.emit(
-        "saved-pet-state",
-        serde_json::json!({"id":id,"snapshot":snapshot}),
-    )
-    .map_err(|e| e.to_string())?;
-    Ok(snapshot)
-}
-
-#[tauri::command]
-pub async fn saved_pet_profile(id: String, store: State<'_, PetStore>) -> Result<SavedPet, String> {
-    store.profile(&id).await
-}
-
-#[tauri::command]
-pub fn get_saved_pet_state(id: String, runtime: State<'_, Runtime>) -> Result<PetSnapshot, String> {
-    runtime.snapshot(&id)
-}
-
-#[tauri::command]
-pub fn send_saved_pet_command(
-    id: String,
-    command: PetCommand,
-    app: tauri::AppHandle,
-) -> Result<PetSnapshot, String> {
-    dispatch_and_emit(&app, &id, command)
-}
-
-#[tauri::command]
-pub async fn open_saved_pet(
-    id: String,
-    app: tauri::AppHandle,
-    store: State<'_, PetStore>,
-) -> Result<(), String> {
-    let _operation = lifecycle_guard().await;
-    let pet = store.profile(&id).await?;
-    let old_caged = store.read_caged(&id).await?;
-    persist_then_open(
-        old_caged,
-        |flag| store.set_caged(&id, flag),
-        || open(&app, &pet),
-    )
-    .await
-}
-
-#[tauri::command]
-pub async fn cage_saved_pet(
-    id: String,
-    app: tauri::AppHandle,
-    store: State<'_, PetStore>,
-) -> Result<(), String> {
-    let _operation = lifecycle_guard().await;
-    store.profile(&id).await?;
-    let window = app.get_webview_window(&label(&id)?);
-    pet_application::lifecycle::persist_then_close(
-        |flag| store.set_caged(&id, flag),
-        || match window {
-            Some(window) => window.destroy().map_err(|e| e.to_string()),
-            None => Ok(()),
-        },
-    )
-    .await?;
-    app.state::<Runtime>().remove(&id);
-    Ok(())
-}
-
-/// Place one existing pet window inside the primary monitor's work area.
-fn centered_position(
-    area: &PhysicalRect<i32, u32>,
-    size: PhysicalSize<u32>,
-) -> PhysicalPosition<i32> {
-    let x = i64::from(area.position.x) + i64::from(area.size.width.saturating_sub(size.width) / 2);
-    let y =
-        i64::from(area.position.y) + i64::from(area.size.height.saturating_sub(size.height) / 2);
-    PhysicalPosition {
-        x: x.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
-        y: y.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
-    }
-}
+use crate::window_position::centered_position;
+use pet_application::lifecycle::persist_then_open;
+use pet_domain::{PetCommand, PetMachine, PetSnapshot};
+use std::{collections::HashMap, sync::Mutex};
+use tauri::{Emitter, Manager, PhysicalPosition, PhysicalRect, PhysicalSize, State};
+
+// 產品目前單程序；全域序列化所有出／入籠、選取與啟動恢復。
+// 不持有 std::sync::MutexGuard 跨 await；Tauri command future 仍可 Send。
+static LIFECYCLE: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());
+
+pub(crate) async fn lifecycle_guard() -> tokio::sync::MutexGuard<'static, ()> {
+    LIFECYCLE.lock().await
+}
+
+#[derive(Default)]
+pub struct Runtime {
+    machines: Mutex<HashMap<String, PetMachine>>,
+}
+
+impl Runtime {
+    pub fn snapshot(&self, id: &str) -> Result<PetSnapshot, String> {
+        self.machines
+            .lock()
+            .map_err(|_| "寵物狀態無法使用")?
+            .get(id)
+            .map(PetMachine::snapshot)
+            .ok_or_else(|| "寵物尚未出籠".into())
+    }
+    pub fn dispatch(&self, id: &str, command: PetCommand) -> Result<PetSnapshot, String> {
+        let mut pets = self.machines.lock().map_err(|_| "寵物狀態無法使用")?;
+        let pet = pets.get_mut(id).ok_or("寵物尚未出籠")?;
+        pet.dispatch(command).map_err(|e| e.to_string())?;
+        Ok(pet.snapshot())
+    }
+    pub fn remove(&self, id: &str) {
+        // Lifecycle cleanup belongs to Runtime, regardless of cage IPC or native Destroyed.
+        if let Ok(name) = label(id) {
+            crate::roam::stop_for_pet(&name);
+        }
+        if let Ok(mut pets) = self.machines.lock() {
+            pets.remove(id);
+        }
+    }
+}
+
+pub(crate) fn label(id: &str) -> Result<String, String> {
+    let id = uuid::Uuid::parse_str(id).map_err(|_| "寵物ID格式錯誤")?;
+    Ok(format!("pet-{}", id.simple()))
+}
+
+pub(crate) fn pet_id_from_window_label(window_label: &str) -> Option<String> {
+    window_label
+        .strip_prefix("pet-")
+        .and_then(|raw| uuid::Uuid::parse_str(raw).ok())
+        .map(|id| id.to_string())
+}
+
+pub fn open(app: &tauri::AppHandle, pet: &SavedPet) -> Result<(), String> {
+    let name = label(&pet.id)?;
+    let runtime = app.state::<Runtime>();
+    // 同 ID 開窗序列化；重複出籠只 show/focus，不重新建立或重設狀態。
+    let mut machines = runtime.machines.lock().map_err(|_| "寵物狀態無法使用")?;
+    if let Some(window) = app.get_webview_window(&name) {
+        window.show().map_err(|e| e.to_string())?;
+        return window.set_focus().map_err(|e| e.to_string());
+    }
+    let count = machines.len();
+    machines.insert(pet.id.clone(), PetMachine::default());
+    let result = tauri::WebviewWindowBuilder::new(
+        app,
+        &name,
+        tauri::WebviewUrl::App(format!("index.html?view=pet&pet_id={}", pet.id).into()),
+    )
+    .title(&pet.name)
+    .inner_size(200.0, 200.0)
+    .position(
+        80.0 + (count % 4) as f64 * 220.0,
+        80.0 + (count / 4) as f64 * 220.0,
+    )
+    .resizable(false)
+    .decorations(false)
+    .transparent(true)
+    .always_on_top(true)
+    .shadow(false)
+    .build();
+    if let Err(e) = result {
+        machines.remove(&pet.id);
+        return Err(e.to_string());
+    }
+    // 僅真正建立新生活窗後啟動；既有窗 show/focus 早退不重複 spawn。
+    crate::roam::spawn_for_pet(app.clone(), name);
+    Ok(())
+}
+
+pub fn dispatch_and_emit(
+    app: &tauri::AppHandle,
+    id: &str,
+    command: PetCommand,
+) -> Result<PetSnapshot, String> {
+    let snapshot = app.state::<Runtime>().dispatch(id, command)?;
+    app.emit(
+        "saved-pet-state",
+        serde_json::json!({"id":id,"snapshot":snapshot}),
+    )
+    .map_err(|e| e.to_string())?;
+    Ok(snapshot)
+}
+
+#[tauri::command]
+pub async fn saved_pet_profile(id: String, store: State<'_, PetStore>) -> Result<SavedPet, String> {
+    store.profile(&id).await
+}
+
+#[tauri::command]
+pub fn get_saved_pet_state(id: String, runtime: State<'_, Runtime>) -> Result<PetSnapshot, String> {
+    runtime.snapshot(&id)
+}
+
+#[tauri::command]
+pub fn send_saved_pet_command(
+    id: String,
+    command: PetCommand,
+    app: tauri::AppHandle,
+) -> Result<PetSnapshot, String> {
+    dispatch_and_emit(&app, &id, command)
+}
+
+#[tauri::command]
+pub async fn open_saved_pet(
+    id: String,
+    app: tauri::AppHandle,
+    store: State<'_, PetStore>,
+) -> Result<(), String> {
+    let _operation = lifecycle_guard().await;
+    let pet = store.profile(&id).await?;
+    let old_caged = store.read_caged(&id).await?;
+    persist_then_open(
+        old_caged,
+        |flag| store.set_caged(&id, flag),
+        || open(&app, &pet),
+    )
+    .await
+}
+
+#[tauri::command]
+pub async fn cage_saved_pet(
+    id: String,
+    app: tauri::AppHandle,
+    store: State<'_, PetStore>,
+) -> Result<(), String> {
+    let _operation = lifecycle_guard().await;
+    store.profile(&id).await?;
+    let window = app.get_webview_window(&label(&id)?);
+    pet_application::lifecycle::persist_then_close(
+        |flag| store.set_caged(&id, flag),
+        || match window {
+            Some(window) => window.destroy().map_err(|e| e.to_string()),
+            None => Ok(()),
+        },
+    )
+    .await?;
+    app.state::<Runtime>().remove(&id);
+    Ok(())
+}
+
+/// Place one existing pet window inside the primary monitor's work area.
 
 #[tauri::command]
 pub async fn find_saved_pet(
diff --git a/pet_apps/pet_server/src/window_position.rs b/pet_apps/pet_server/src/window_position.rs
new file mode 100644
--- /dev/null
+++ b/pet_apps/pet_server/src/window_position.rs
@@ -0,0 +1,31 @@
+use tauri::{PhysicalPosition, PhysicalRect, PhysicalSize};
+
+/// Pure native-window geometry. The caller owns show/focus and roam pause/resume.
+pub(crate) fn centered_position(
+    area: &PhysicalRect<i32, u32>,
+    size: PhysicalSize<u32>,
+) -> PhysicalPosition<i32> {
+    let x = i64::from(area.position.x) + i64::from(area.size.width.saturating_sub(size.width) / 2);
+    let y =
+        i64::from(area.position.y) + i64::from(area.size.height.saturating_sub(size.height) / 2);
+    PhysicalPosition {
+        x: x.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
+        y: y.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    #[test]
+    fn small_work_area_never_places_window_before_its_origin() {
+        let area = PhysicalRect {
+            position: PhysicalPosition { x: -300, y: 20 },
+            size: PhysicalSize { width: 100, height: 100 },
+        };
+        assert_eq!(centered_position(&area, PhysicalSize { width: 200, height: 300 }),
+            PhysicalPosition { x: -300, y: 20 });
+        assert_eq!(centered_position(&area, PhysicalSize { width: 20, height: 20 }),
+            PhysicalPosition { x: -260, y: 60 });
+    }
+}
```

本案 Refactor 後續：把逐隻 Native X／IPC 的補償與錯誤注入整理成專案 fixture；此日沒有搬動跨 workspace Port 或全庫 workflow。驗證 `cargo test --manifest-path pet_apps/Cargo.toml -p pet-server window_position` 與 `-p pet-server day39_find_position` **未執行**。

## ② Red → 完整 Green（核心預估 15 分鐘；未實測）

**Red**：在原產品不存在 `auto_release_on_start`、`get_pet_auto_release/set_pet_auto_release`、對應控制台按鈕；新的 `pet_persistence::day44_auto_release_tests` 要求「A 設預設 → A 回籠 → 預設仍 true」「A/B 同時預設」「其他 owner 不可改」「取消 A 只剩 B」「DB 失敗不可成功」，與 `auto_release_status::tests` 的 IPC Err 不冒充 false。未套 Green 前先是缺符號／缺 schema 編譯 Red；行為 Red 需在完成 Red fixture 後實跑才算，**本次沒有實跑**。保留舊 `list_uncaged` 及 Day35/36/37/42/43 斷言，不能因改啟動政策刪除。

**Green**：migration 與 SQLx Port 寫／讀／啟動集合；`PetStore` 組裝；兩個 Tauri commands 與 `generate_handler!`；JS 與 Rust wasm bridge；控制台逐 UUID 查詢、明確設置／取消、逐列回饋；啟動時改讀預設集合並用既有 `persist_then_open` 保護原回籠旗標，某隻失敗不阻止其他隻。**以下是全部修改單元的完整 diff（先套 Refactor）**，不是摘要或未接線範例：

```diff
diff --git a/pet_apps/pet_server/src/lib.rs b/pet_apps/pet_server/src/lib.rs
--- a/pet_apps/pet_server/src/lib.rs
+++ b/pet_apps/pet_server/src/lib.rs
@@ -126,190 +126,221 @@
 }
 
 #[tauri::command]
-async fn list_pets(
-    store: State<'_, pet_store::PetStore>,
-) -> Result<Vec<pet_store::SavedPet>, String> {
-    store.list().await
-}
-
-#[tauri::command]
-async fn active_pet(
-    store: State<'_, pet_store::PetStore>,
-) -> Result<Option<pet_store::SavedPet>, String> {
-    store.active().await
-}
-
-#[tauri::command]
-async fn select_pet(
-    id: String,
-    app: tauri::AppHandle,
-    store: State<'_, pet_store::PetStore>,
-) -> Result<pet_store::SavedPet, String> {
-    let _operation = pet_windows::lifecycle_guard().await;
-    let saved = store.select(&id).await?;
-    pet_windows::open(&app, &saved)?;
-    app.emit("pet-profile-changed", &saved)
-        .map_err(|_| "選取已儲存，通知失敗，請重新載入生活窗".to_owned())?;
-    Ok(saved)
-}
-
-pub fn application(db: sqlx::SqlitePool) {
-    tauri::Builder::default()
-        .on_window_event(|window, event| {
-            // 控制台關閉僅隱藏，讓既有 show/focus IPC 能恢復同一個 main。
-            if window.label() == "main" {
-                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
-                    api.prevent_close();
-                    if let Err(error) = window.hide() {
-                        // 隱藏失敗時仍保留視窗；不把錯誤當成銷毀成功。
-                        eprintln!("隱藏控制視窗失敗：{error}");
-                    }
-                }
-                return;
-            }
-
-            // Native X/close is the same cage transaction as the explicit IPC command.
-            // The control window keeps its existing hide-on-close behavior above.
-            if let Some(id) = pet_windows::pet_id_from_window_label(window.label()) {
-                match event {
-                    tauri::WindowEvent::CloseRequested { api, .. } => {
-                        api.prevent_close();
-                        let window = window.clone();
-                        let store = window
-                            .app_handle()
-                            .state::<pet_store::PetStore>()
-                            .inner()
-                            .clone();
-                        tauri::async_runtime::spawn(async move {
-                            let _operation = pet_windows::lifecycle_guard().await;
-                            if let Err(error) = pet_application::lifecycle::persist_then_close(
-                                |flag| store.set_caged(&id, flag),
-                                || window.destroy().map_err(|e| e.to_string()),
-                            )
-                            .await
-                            {
-                                // DB failure must leave the window alive; never silently claim cage.
-                                eprintln!("生活窗直接關閉／回籠失敗：{error}");
-                            }
-                        });
-                    }
-                    tauri::WindowEvent::Destroyed => {
-                        window
-                            .app_handle()
-                            .state::<pet_windows::Runtime>()
-                            .remove(&id);
-                    }
-                    _ => {}
-                }
-            }
-        })
-        .manage(Mutex::new(PetMachine::default()))
-        .manage(pet_windows::Runtime::default())
-        .manage(AiState::new(Arc::new(MockChatModel)))
-        .manage(pet_store::PetStore { pool: db })
-        .setup(|app| {
-            let handle = app.handle().clone();
-            let store = app.state::<pet_store::PetStore>().inner().clone();
-            tauri::async_runtime::spawn(async move {
-                let _operation = pet_windows::lifecycle_guard().await;
-                match store.uncaged().await {
-                    Ok(pets) => {
-                        for pet in pets {
-                            // 每個 UUID 由既有 open() 建立獨立原生視窗與 Runtime。
-                            // 單隻失敗不應讓其他未回籠寵物無法恢復。
-                            if let Err(error) = pet_windows::open(&handle, &pet) {
-                                eprintln!("重啟恢復寵物 {} 失敗：{error}", pet.id);
-                            }
-                        }
-                    }
-                    Err(error) => {
-                        // DB 失敗時不可誤認沒有寵物，更不可將其標成回籠。
-                        eprintln!("重啟讀取出籠寵物失敗：{error}");
-                    }
-                }
-            });
-            /*
-             * JS Menu.new() 建立的 native menu item
-             * 仍會產生 native menu event。
-             *
-             * Linux 不再依賴 JS action callback
-             * 才能 dispatch PetCommand。
-             */
-            app.on_menu_event(|app_handle, event| {
-                // Day32：由生活窗原生選單傳入該隻寵物的 UUID。
-                if let Some(id) = control_menu_pet_id(event.id().0.as_str()) {
-                    let handle = app_handle.clone();
-                    let store = app_handle.state::<pet_store::PetStore>().inner().clone();
-                    tauri::async_runtime::spawn(async move {
-                        let _operation = pet_windows::lifecycle_guard().await;
-                        match store.select(&id).await {
-                            Ok(saved) => {
-                                // 控制窗原有 Leptos 監聽此事件，選取成功才通知。
-                                if let Err(error) = handle.emit("pet-profile-changed", &saved) {
-                                    eprintln!("Day32 選取通知失敗：{error}");
-                                    return;
-                                }
-                                if let Err(error) = open_control_window(handle) {
-                                    eprintln!("Day32 無法開啟控制窗：{error}");
-                                }
-                            }
-                            Err(error) => eprintln!("Day32 選取寵物失敗：{error}"),
-                        }
-                    });
-                    return;
-                }
-
-                if event.id().0.as_str() == "open_control" {
-                    let _ = open_control_window(app_handle.clone());
-                    return;
-                }
-
-                if let Some(scoped) = event.id().0.strip_prefix("pet:") {
-                    if let Some((id, action)) = scoped.split_once(':') {
-                        let command = match action {
-                            "sleep" => Some(PetCommand::Sleep),
-                            "wake" => Some(PetCommand::Wake),
-                            "interact" => Some(PetCommand::Interact),
-                            _ => None,
-                        };
-                        if let Some(command) = command {
-                            let _ = pet_windows::dispatch_and_emit(app_handle, id, command);
-                        }
-                    }
-                    return;
-                }
-
-                let command = match event.id().0.as_str() {
-                    "interact" => Some(PetCommand::Interact),
-                    "sleep" => Some(PetCommand::Sleep),
-                    "wake" => Some(PetCommand::Wake),
-                    _ => None,
-                };
-
-                let Some(command) = command else {
-                    return;
-                };
-
-                let pet = app_handle.state::<Mutex<PetMachine>>();
-
-                if let Err(error) = dispatch_pet_command(app_handle, pet.inner(), command) {
-                    eprintln!("pet context menu command failed: {error}");
-                }
-            });
-            Ok(())
-        })
-        .invoke_handler(tauri::generate_handler![
-            get_pet_state,
-            send_pet_command,
-            chat_with_pet,
-            scan_file_preview,
-            scan_duplicate_preview,
-            open_control_window,
-            quit_pet,
-            validate_new_pet_name,
-            create_pet,
-            list_pets,
-            pet_cage_status_by_id,
+async fn get_pet_auto_release(
+    id: String,
+    store: State<'_, pet_store::PetStore>,
+) -> Result<bool, String> {
+    store.read_auto_release(&id).await
+}
+
+#[tauri::command]
+async fn set_pet_auto_release(
+    id: String,
+    enabled: bool,
+    store: State<'_, pet_store::PetStore>,
+) -> Result<bool, String> {
+    store.set_auto_release(&id, enabled).await?;
+    store.read_auto_release(&id).await
+}
+
+#[tauri::command]
+async fn list_pets(
+    store: State<'_, pet_store::PetStore>,
+) -> Result<Vec<pet_store::SavedPet>, String> {
+    store.list().await
+}
+
+#[tauri::command]
+async fn active_pet(
+    store: State<'_, pet_store::PetStore>,
+) -> Result<Option<pet_store::SavedPet>, String> {
+    store.active().await
+}
+
+#[tauri::command]
+async fn select_pet(
+    id: String,
+    app: tauri::AppHandle,
+    store: State<'_, pet_store::PetStore>,
+) -> Result<pet_store::SavedPet, String> {
+    let _operation = pet_windows::lifecycle_guard().await;
+    let saved = store.select(&id).await?;
+    pet_windows::open(&app, &saved)?;
+    app.emit("pet-profile-changed", &saved)
+        .map_err(|_| "選取已儲存，通知失敗，請重新載入生活窗".to_owned())?;
+    Ok(saved)
+}
+
+pub fn application(db: sqlx::SqlitePool) {
+    tauri::Builder::default()
+        .on_window_event(|window, event| {
+            // 控制台關閉僅隱藏，讓既有 show/focus IPC 能恢復同一個 main。
+            if window.label() == "main" {
+                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
+                    api.prevent_close();
+                    if let Err(error) = window.hide() {
+                        // 隱藏失敗時仍保留視窗；不把錯誤當成銷毀成功。
+                        eprintln!("隱藏控制視窗失敗：{error}");
+                    }
+                }
+                return;
+            }
+
+            // Native X/close is the same cage transaction as the explicit IPC command.
+            // The control window keeps its existing hide-on-close behavior above.
+            if let Some(id) = pet_windows::pet_id_from_window_label(window.label()) {
+                match event {
+                    tauri::WindowEvent::CloseRequested { api, .. } => {
+                        api.prevent_close();
+                        let window = window.clone();
+                        let store = window
+                            .app_handle()
+                            .state::<pet_store::PetStore>()
+                            .inner()
+                            .clone();
+                        tauri::async_runtime::spawn(async move {
+                            let _operation = pet_windows::lifecycle_guard().await;
+                            if let Err(error) = pet_application::lifecycle::persist_then_close(
+                                |flag| store.set_caged(&id, flag),
+                                || window.destroy().map_err(|e| e.to_string()),
+                            )
+                            .await
+                            {
+                                // DB failure must leave the window alive; never silently claim cage.
+                                eprintln!("生活窗直接關閉／回籠失敗：{error}");
+                            }
+                        });
+                    }
+                    tauri::WindowEvent::Destroyed => {
+                        window
+                            .app_handle()
+                            .state::<pet_windows::Runtime>()
+                            .remove(&id);
+                    }
+                    _ => {}
+                }
+            }
+        })
+        .manage(Mutex::new(PetMachine::default()))
+        .manage(pet_windows::Runtime::default())
+        .manage(AiState::new(Arc::new(MockChatModel)))
+        .manage(pet_store::PetStore { pool: db })
+        .setup(|app| {
+            let handle = app.handle().clone();
+            let store = app.state::<pet_store::PetStore>().inner().clone();
+            tauri::async_runtime::spawn(async move {
+                let _operation = pet_windows::lifecycle_guard().await;
+                match store.auto_release_pets().await {
+                    Ok(pets) => {
+                        for pet in pets {
+                            // 每個 UUID 由既有 open() 建立獨立原生視窗與 Runtime。
+                            // 單隻失敗不應讓其他未回籠寵物無法恢復。
+                            let old_caged = match store.read_caged(&pet.id).await {
+                                Ok(flag) => flag,
+                                Err(error) => {
+                                    eprintln!("重啟讀取寵物 {} 回籠狀態失敗：{error}", pet.id);
+                                    continue;
+                                }
+                            };
+                            if let Err(error) = pet_application::lifecycle::persist_then_open(
+                                old_caged,
+                                |flag| store.set_caged(&pet.id, flag),
+                                || pet_windows::open(&handle, &pet),
+                            ).await {
+                                eprintln!("重啟預設出籠寵物 {} 失敗：{error}", pet.id);
+                            }
+                        }
+                    }
+                    Err(error) => {
+                        // DB 失敗時不可誤認沒有寵物，更不可將其標成回籠。
+                        eprintln!("重啟讀取出籠寵物失敗：{error}");
+                    }
+                }
+            });
+            /*
+             * JS Menu.new() 建立的 native menu item
+             * 仍會產生 native menu event。
+             *
+             * Linux 不再依賴 JS action callback
+             * 才能 dispatch PetCommand。
+             */
+            app.on_menu_event(|app_handle, event| {
+                // Day32：由生活窗原生選單傳入該隻寵物的 UUID。
+                if let Some(id) = control_menu_pet_id(event.id().0.as_str()) {
+                    let handle = app_handle.clone();
+                    let store = app_handle.state::<pet_store::PetStore>().inner().clone();
+                    tauri::async_runtime::spawn(async move {
+                        let _operation = pet_windows::lifecycle_guard().await;
+                        match store.select(&id).await {
+                            Ok(saved) => {
+                                // 控制窗原有 Leptos 監聽此事件，選取成功才通知。
+                                if let Err(error) = handle.emit("pet-profile-changed", &saved) {
+                                    eprintln!("Day32 選取通知失敗：{error}");
+                                    return;
+                                }
+                                if let Err(error) = open_control_window(handle) {
+                                    eprintln!("Day32 無法開啟控制窗：{error}");
+                                }
+                            }
+                            Err(error) => eprintln!("Day32 選取寵物失敗：{error}"),
+                        }
+                    });
+                    return;
+                }
+
+                if event.id().0.as_str() == "open_control" {
+                    let _ = open_control_window(app_handle.clone());
+                    return;
+                }
+
+                if let Some(scoped) = event.id().0.strip_prefix("pet:") {
+                    if let Some((id, action)) = scoped.split_once(':') {
+                        let command = match action {
+                            "sleep" => Some(PetCommand::Sleep),
+                            "wake" => Some(PetCommand::Wake),
+                            "interact" => Some(PetCommand::Interact),
+                            _ => None,
+                        };
+                        if let Some(command) = command {
+                            let _ = pet_windows::dispatch_and_emit(app_handle, id, command);
+                        }
+                    }
+                    return;
+                }
+
+                let command = match event.id().0.as_str() {
+                    "interact" => Some(PetCommand::Interact),
+                    "sleep" => Some(PetCommand::Sleep),
+                    "wake" => Some(PetCommand::Wake),
+                    _ => None,
+                };
+
+                let Some(command) = command else {
+                    return;
+                };
+
+                let pet = app_handle.state::<Mutex<PetMachine>>();
+
+                if let Err(error) = dispatch_pet_command(app_handle, pet.inner(), command) {
+                    eprintln!("pet context menu command failed: {error}");
+                }
+            });
+            Ok(())
+        })
+        .invoke_handler(tauri::generate_handler![
+            get_pet_state,
+            send_pet_command,
+            chat_with_pet,
+            scan_file_preview,
+            scan_duplicate_preview,
+            open_control_window,
+            quit_pet,
+            validate_new_pet_name,
+            create_pet,
+            list_pets,
+            pet_cage_status_by_id,
+            get_pet_auto_release,
+            set_pet_auto_release,
             active_pet,
             select_pet,
             pet_windows::saved_pet_profile,
diff --git a/pet_apps/pet_server/src/pet_store.rs b/pet_apps/pet_server/src/pet_store.rs
--- a/pet_apps/pet_server/src/pet_store.rs
+++ b/pet_apps/pet_server/src/pet_store.rs
@@ -160,6 +160,21 @@
             .collect())
     }
 
+    pub async fn auto_release_pets(&self) -> Result<Vec<SavedPet>, String> {
+        let rows = pet_persistence::list_auto_release(&self.pool).await?;
+        Ok(rows.into_iter().map(|p| SavedPet {
+            id: p.id, name: p.name, species: p.species,
+        }).collect())
+    }
+
+    pub async fn set_auto_release(&self, id: &str, enabled: bool) -> Result<(), String> {
+        pet_persistence::set_auto_release(&self.pool, id, enabled).await
+    }
+
+    pub async fn read_auto_release(&self, id: &str) -> Result<bool, String> {
+        pet_persistence::read_auto_release(&self.pool, id).await
+    }
+
     pub async fn set_caged(&self, id: &str, caged: bool) -> Result<(), String> {
         pet_persistence::set_caged(&self.pool, id, caged).await
     }
diff --git a/pet_apps/pet_persistence/src/lib.rs b/pet_apps/pet_persistence/src/lib.rs
--- a/pet_apps/pet_persistence/src/lib.rs
+++ b/pet_apps/pet_persistence/src/lib.rs
@@ -33,128 +33,208 @@
     }
 }
 
-/// 重啟查詢 DTO；不依賴 Tauri 或 server 的 SavedPet。
-#[derive(Debug, Clone, PartialEq, Eq)]
-pub struct UncagedPet {
-    pub id: String,
-    pub name: String,
-    pub species: String,
-}
-
-/// 只查本機 owner 的未回籠寵物；slot=1 不是出籠清單。
-pub async fn list_uncaged(pool: &sqlx::SqlitePool) -> Result<Vec<UncagedPet>, String> {
-    let rows: Vec<(String, String, String)> = sqlx::query_as(
-        r#"SELECT id
-            , name
-            , species
-        FROM pet_profile
-        WHERE owner_id='local-owner'
-            AND COALESCE(is_default_closure, 0)=0
-        ORDER BY name
-            , id"#,
-    )
-    .fetch_all(pool)
-    .await
-    .map_err(|_| "無法讀取出籠寵物".to_owned())?;
-    Ok(rows
-        .into_iter()
-        .map(|(id, name, species)| UncagedPet { id, name, species })
-        .collect())
-}
-
-#[cfg(test)]
-mod day35_cage_adapter_tests {
-    use super::set_caged;
-
-    #[sqlx::test(migrations = "../migrations")]
-    async fn only_target_pet_changes_and_missing_id_fails(pool: sqlx::SqlitePool) {
-        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','本機主人')")
-            .execute(&pool)
-            .await
-            .unwrap();
-        for id in ["pet-a", "pet-b", "pet-c"] {
-            sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id) VALUES(?1,?2,'tortoise','local-owner')")
-                .bind(id).bind(id).execute(&pool).await.unwrap();
-        }
-        set_caged(&pool, "pet-b", true).await.unwrap();
-        for (id, expected) in [("pet-a", 0_i64), ("pet-b", 1), ("pet-c", 0)] {
-            let flag: i64 =
-                sqlx::query_scalar("SELECT is_default_closure FROM pet_profile WHERE id=?1")
-                    .bind(id)
-                    .fetch_one(&pool)
-                    .await
-                    .unwrap();
-            assert_eq!(flag, expected, "{id}");
-        }
-        assert!(set_caged(&pool, "missing-id", true).await.is_err());
-        set_caged(&pool, "pet-b", false).await.unwrap();
-        let flag: i64 =
-            sqlx::query_scalar("SELECT is_default_closure FROM pet_profile WHERE id='pet-b'")
-                .fetch_one(&pool)
-                .await
-                .unwrap();
-        assert_eq!(flag, 0);
-    }
-
-    #[sqlx::test(migrations = "../migrations")]
-    async fn failed_database_update_is_not_success(pool: sqlx::SqlitePool) {
-        pool.close().await;
-        assert!(set_caged(&pool, "pet-a", true).await.is_err());
-    }
-}
-
-#[cfg(test)]
-mod day36_restore_adapter_tests {
-    use super::list_uncaged;
-    #[sqlx::test(migrations = "../migrations")]
-    async fn reads_only_uncaged_local_owner(pool: sqlx::SqlitePool) {
-        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','本機')")
-            .execute(&pool)
-            .await
-            .unwrap();
-        for (id, caged) in [("pet-a", false), ("pet-b", true), ("pet-c", false)] {
-            sqlx::query(
-                "INSERT INTO pet_profile(id,name,species,owner_id,is_default_closure)
-                 VALUES(?1,?2,'tortoise','local-owner',?3)",
-            )
-            .bind(id)
-            .bind(id)
-            .bind(caged)
-            .execute(&pool)
-            .await
-            .unwrap();
-        }
-        let rows = list_uncaged(&pool).await.unwrap();
-        assert_eq!(
-            rows.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(),
-            vec!["pet-a", "pet-c"]
-        );
-        pool.close().await;
-        assert!(list_uncaged(&pool).await.is_err());
-    }
-}
-
-#[cfg(test)]
-mod day37_open_state_tests {
-    use super::*;
-    #[sqlx::test(migrations = "../migrations")]
-    async fn read_flag_and_fail_closed_for_missing_pet(pool: sqlx::SqlitePool) {
-        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','本機')")
-            .execute(&pool)
-            .await
-            .unwrap();
-        sqlx::query(
-            "INSERT INTO pet_profile(id,name,species,owner_id,is_default_closure)
-            VALUES('a','A','tortoise','local-owner',1)",
-        )
-        .execute(&pool)
-        .await
-        .unwrap();
-        assert!(read_caged(&pool, "a").await.unwrap());
-        set_caged(&pool, "a", false).await.unwrap();
-        assert!(!read_caged(&pool, "a").await.unwrap());
-        assert!(read_caged(&pool, "missing").await.is_err());
-        pool.close().await;
-        assert!(read_caged(&pool, "a").await.is_err());
+
+/// Owner-scoped restart preference, independent from the current cage flag.
+pub async fn set_auto_release(
+    pool: &sqlx::SqlitePool, id: &str, enabled: bool,
+) -> Result<(), String> {
+    let updated = sqlx::query(
+        "UPDATE pet_profile SET auto_release_on_start=?1,
+         updated_at=CURRENT_TIMESTAMP
+         WHERE id=?2 AND owner_id='local-owner'"
+    ).bind(enabled).bind(id).execute(pool).await
+        .map_err(|_| "無法保存預設出籠資格".to_owned())?;
+    if updated.rows_affected() != 1 {
+        return Err("找不到寵物".into());
+    }
+    Ok(())
+}
+
+pub async fn read_auto_release(pool: &sqlx::SqlitePool, id: &str) -> Result<bool, String> {
+    let value: Option<i64> = sqlx::query_scalar(
+        "SELECT auto_release_on_start FROM pet_profile
+         WHERE id=?1 AND owner_id='local-owner'"
+    ).bind(id).fetch_optional(pool).await
+        .map_err(|_| "無法讀取預設出籠資格".to_owned())?;
+    match value {
+        Some(0) => Ok(false),
+        Some(1) => Ok(true),
+        Some(_) => Err("預設出籠資格資料不合法".into()),
+        None => Err("找不到寵物".into()),
+    }
+}
+
+#[derive(Debug, Clone, PartialEq, Eq)]
+pub struct AutoReleasePet {
+    pub id: String,
+    pub name: String,
+    pub species: String,
+}
+
+/// Restart reads preference, not current cage status or selected slot.
+pub async fn list_auto_release(pool: &sqlx::SqlitePool) -> Result<Vec<AutoReleasePet>, String> {
+    let rows: Vec<(String, String, String)> = sqlx::query_as(
+        "SELECT id,name,species FROM pet_profile
+         WHERE owner_id='local-owner' AND auto_release_on_start=1
+         ORDER BY name,id"
+    ).fetch_all(pool).await
+        .map_err(|_| "無法讀取預設出籠寵物".to_owned())?;
+    Ok(rows.into_iter().map(|(id,name,species)| AutoReleasePet { id,name,species }).collect())
+}
+
+/// 重啟查詢 DTO；不依賴 Tauri 或 server 的 SavedPet。
+#[derive(Debug, Clone, PartialEq, Eq)]
+pub struct UncagedPet {
+    pub id: String,
+    pub name: String,
+    pub species: String,
+}
+
+/// 只查本機 owner 的未回籠寵物；slot=1 不是出籠清單。
+pub async fn list_uncaged(pool: &sqlx::SqlitePool) -> Result<Vec<UncagedPet>, String> {
+    let rows: Vec<(String, String, String)> = sqlx::query_as(
+        r#"SELECT id
+            , name
+            , species
+        FROM pet_profile
+        WHERE owner_id='local-owner'
+            AND COALESCE(is_default_closure, 0)=0
+        ORDER BY name
+            , id"#,
+    )
+    .fetch_all(pool)
+    .await
+    .map_err(|_| "無法讀取出籠寵物".to_owned())?;
+    Ok(rows
+        .into_iter()
+        .map(|(id, name, species)| UncagedPet { id, name, species })
+        .collect())
+}
+
+#[cfg(test)]
+mod day35_cage_adapter_tests {
+    use super::set_caged;
+
+    #[sqlx::test(migrations = "../migrations")]
+    async fn only_target_pet_changes_and_missing_id_fails(pool: sqlx::SqlitePool) {
+        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','本機主人')")
+            .execute(&pool)
+            .await
+            .unwrap();
+        for id in ["pet-a", "pet-b", "pet-c"] {
+            sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id) VALUES(?1,?2,'tortoise','local-owner')")
+                .bind(id).bind(id).execute(&pool).await.unwrap();
+        }
+        set_caged(&pool, "pet-b", true).await.unwrap();
+        for (id, expected) in [("pet-a", 0_i64), ("pet-b", 1), ("pet-c", 0)] {
+            let flag: i64 =
+                sqlx::query_scalar("SELECT is_default_closure FROM pet_profile WHERE id=?1")
+                    .bind(id)
+                    .fetch_one(&pool)
+                    .await
+                    .unwrap();
+            assert_eq!(flag, expected, "{id}");
+        }
+        assert!(set_caged(&pool, "missing-id", true).await.is_err());
+        set_caged(&pool, "pet-b", false).await.unwrap();
+        let flag: i64 =
+            sqlx::query_scalar("SELECT is_default_closure FROM pet_profile WHERE id='pet-b'")
+                .fetch_one(&pool)
+                .await
+                .unwrap();
+        assert_eq!(flag, 0);
+    }
+
+    #[sqlx::test(migrations = "../migrations")]
+    async fn failed_database_update_is_not_success(pool: sqlx::SqlitePool) {
+        pool.close().await;
+        assert!(set_caged(&pool, "pet-a", true).await.is_err());
+    }
+}
+
+#[cfg(test)]
+mod day36_restore_adapter_tests {
+    use super::list_uncaged;
+    #[sqlx::test(migrations = "../migrations")]
+    async fn reads_only_uncaged_local_owner(pool: sqlx::SqlitePool) {
+        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','本機')")
+            .execute(&pool)
+            .await
+            .unwrap();
+        for (id, caged) in [("pet-a", false), ("pet-b", true), ("pet-c", false)] {
+            sqlx::query(
+                "INSERT INTO pet_profile(id,name,species,owner_id,is_default_closure)
+                 VALUES(?1,?2,'tortoise','local-owner',?3)",
+            )
+            .bind(id)
+            .bind(id)
+            .bind(caged)
+            .execute(&pool)
+            .await
+            .unwrap();
+        }
+        let rows = list_uncaged(&pool).await.unwrap();
+        assert_eq!(
+            rows.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(),
+            vec!["pet-a", "pet-c"]
+        );
+        pool.close().await;
+        assert!(list_uncaged(&pool).await.is_err());
+    }
+}
+
+#[cfg(test)]
+mod day37_open_state_tests {
+    use super::*;
+    #[sqlx::test(migrations = "../migrations")]
+    async fn read_flag_and_fail_closed_for_missing_pet(pool: sqlx::SqlitePool) {
+        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','本機')")
+            .execute(&pool)
+            .await
+            .unwrap();
+        sqlx::query(
+            "INSERT INTO pet_profile(id,name,species,owner_id,is_default_closure)
+            VALUES('a','A','tortoise','local-owner',1)",
+        )
+        .execute(&pool)
+        .await
+        .unwrap();
+        assert!(read_caged(&pool, "a").await.unwrap());
+        set_caged(&pool, "a", false).await.unwrap();
+        assert!(!read_caged(&pool, "a").await.unwrap());
+        assert!(read_caged(&pool, "missing").await.is_err());
+        pool.close().await;
+        assert!(read_caged(&pool, "a").await.is_err());
+    }
+}
+
+#[cfg(test)]
+mod day44_auto_release_tests {
+    use super::*;
+    #[sqlx::test(migrations = "../migrations")]
+    async fn restart_preference_survives_cage_and_is_owner_scoped(pool: sqlx::SqlitePool) {
+        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','本機')")
+            .execute(&pool).await.unwrap();
+        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('other-owner','其他')")
+            .execute(&pool).await.unwrap();
+        for (id, owner) in [("pet-a","local-owner"),("pet-b","local-owner"),("pet-x","other-owner")] {
+            sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id)
+                VALUES(?1,?2,'tortoise',?3)")
+                .bind(id).bind(id).bind(owner).execute(&pool).await.unwrap();
+        }
+        assert!(!read_auto_release(&pool,"pet-a").await.unwrap());
+        set_auto_release(&pool,"pet-a",true).await.unwrap();
+        set_auto_release(&pool,"pet-b",true).await.unwrap();
+        set_caged(&pool,"pet-a",true).await.unwrap();
+        assert!(read_caged(&pool,"pet-a").await.unwrap());
+        assert!(read_auto_release(&pool,"pet-a").await.unwrap());
+        assert_eq!(list_auto_release(&pool).await.unwrap().iter()
+            .map(|p|p.id.as_str()).collect::<Vec<_>>(), vec!["pet-a","pet-b"]);
+        assert!(set_auto_release(&pool,"pet-x",true).await.is_err());
+        assert!(read_auto_release(&pool,"pet-x").await.is_err());
+        set_auto_release(&pool,"pet-a",false).await.unwrap();
+        assert_eq!(list_auto_release(&pool).await.unwrap().len(),1);
+        pool.close().await;
+        assert!(read_auto_release(&pool,"pet-a").await.is_err());
     }
 }
diff --git a/pet_apps/pet_ui/index.html b/pet_apps/pet_ui/index.html
--- a/pet_apps/pet_ui/index.html
+++ b/pet_apps/pet_ui/index.html
@@ -41,6 +41,9 @@
 
     window.petCageStatusById = (id) => invoke("pet_cage_status_by_id", { id });
 
+    window.getPetAutoRelease = (id) => invoke("get_pet_auto_release", { id });
+    window.setPetAutoRelease = (id, enabled) => invoke("set_pet_auto_release", { id, enabled });
+
     window.findSavedPet = (id) => invoke("find_saved_pet", { id });
 
     window.chatWithPet = (message) => {
diff --git a/pet_apps/pet_ui/src/bridge.rs b/pet_apps/pet_ui/src/bridge.rs
--- a/pet_apps/pet_ui/src/bridge.rs
+++ b/pet_apps/pet_ui/src/bridge.rs
@@ -84,189 +84,205 @@
     #[wasm_bindgen(js_namespace=window, js_name=petCageStatusById)]
     fn pet_cage_status_by_id_js(id: &str) -> Promise;
 
-    #[wasm_bindgen(js_namespace=window, js_name=findSavedPet)]
-    fn find_saved_pet_js(id: &str) -> Promise;
-
-    #[wasm_bindgen(js_namespace=window, js_name=activePet)]
-    fn active_pet_js() -> Promise;
-
-    #[wasm_bindgen(js_namespace=window, js_name=selectPet)]
-    fn select_pet_js(id: &str) -> Promise;
-
-    #[wasm_bindgen(js_namespace=window, js_name=listenPetProfile)]
-    fn listen_pet_profile_js(callback: &Function) -> Promise;
-}
-
-// 將 JavaScript 的錯誤轉換為 Rust 的字串
-fn js_error(value: JsValue) -> String {
-    value.as_string().unwrap_or_else(|| format!("{value:?}"))
-}
-
-// 獲取寵物的當前狀態
-pub async fn get_pet_state() -> Result<PetSnapshot, String> {
-    let value = JsFuture::from(get_pet_state_js()).await.map_err(js_error)?;
-
-    serde_wasm_bindgen::from_value(value).map_err(|error| error.to_string())
-}
-
-// 發送指令給寵物並獲取更新後的狀態
-pub async fn send_pet_command(command: PetCommand) -> Result<PetSnapshot, String> {
-    let command = serde_wasm_bindgen::to_value(&command).map_err(|error| error.to_string())?;
-
-    let value = JsFuture::from(send_pet_command_js(command))
-        .await
-        .map_err(js_error)?;
-
-    serde_wasm_bindgen::from_value(value).map_err(|error| error.to_string())
-}
-
-// 監聽寵物狀態的變化
-pub async fn listen_pet_state(
-    mut on_snapshot: impl FnMut(PetSnapshot) + 'static,
-) -> Result<(), String> {
-    let callback = Closure::<dyn FnMut(JsValue)>::new(move |value| {
-        if let Ok(snapshot) = serde_wasm_bindgen::from_value(value) {
-            on_snapshot(snapshot);
-        }
-    });
-
-    JsFuture::from(listen_pet_state_js(callback.as_ref().unchecked_ref()))
-        .await
-        .map_err(js_error)?;
-
-    // listener 跟著整個 pet app 存活。
-    callback.forget();
-
-    Ok(())
-}
-
-// 顯示寵物的右鍵選單
-pub async fn show_pet_context_menu(x: i32, y: i32) -> Result<(), String> {
-    JsFuture::from(show_pet_context_menu_js(x, y))
-        .await
-        .map_err(js_error)?;
-
-    Ok(())
-}
-
-// 與寵物進行聊天
-pub async fn chat_with_pet(message: String) -> Result<String, String> {
-    let value = JsFuture::from(chat_with_pet_js(&message))
-        .await
-        .map_err(js_error)?;
-
-    value
-        .as_string()
-        .ok_or_else(|| "Failed to convert chat_with_pet result to string".to_string())
-}
-
-pub async fn scan_file_preview() -> Result<String, String> {
-    let value = JsFuture::from(scan_file_preview_js())
-        .await
-        .map_err(js_error)?;
-    let view: crate::file_scan_view::FileScanView =
-        serde_wasm_bindgen::from_value(value).map_err(|error| error.to_string())?;
-    Ok(view.summary())
-}
-
-pub async fn scan_duplicate_preview() -> Result<String, String> {
-    let value = JsFuture::from(scan_duplicate_preview_js())
-        .await
-        .map_err(js_error)?;
-    let result: crate::duplicate_preview::DuplicatePreviewView =
-        serde_wasm_bindgen::from_value(value).map_err(|e| e.to_string())?;
-    Ok(result.summary())
-}
-
-pub async fn open_controls() -> Result<(), String> {
-    JsFuture::from(open_controls_js()).await.map_err(js_error)?;
-
-    Ok(())
-}
-
-pub async fn quit_pet() -> Result<(), String> {
-    JsFuture::from(quit_pet_js()).await.map_err(js_error)?;
-
-    Ok(())
-}
-
-pub async fn create_pet(name: String) -> Result<SavedPet, String> {
-    let value = JsFuture::from(create_pet_js(&name))
-        .await
-        .map_err(js_error)?;
-    serde_wasm_bindgen::from_value(value)
-        .map_err(|_| "建立寵物回應格式錯誤；請重新整理清單確認是否已建立".into())
-}
-
-#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
-pub struct SavedPet {
-    pub id: String,
-    pub name: String,
-    pub species: String,
-}
-
-pub async fn list_pets() -> Result<Vec<SavedPet>, String> {
-    let value = JsFuture::from(list_pets_js()).await.map_err(js_error)?;
-    serde_wasm_bindgen::from_value(value).map_err(|_| "寵物清單格式錯誤".into())
-}
-
-pub async fn active_pet() -> Result<Option<SavedPet>, String> {
-    let value = JsFuture::from(active_pet_js()).await.map_err(js_error)?;
-    serde_wasm_bindgen::from_value(value).map_err(|_| "生活窗資料格式錯誤".into())
-}
-
-pub async fn select_pet(id: &str) -> Result<SavedPet, String> {
-    let value = JsFuture::from(select_pet_js(id)).await.map_err(js_error)?;
-    serde_wasm_bindgen::from_value(value).map_err(|_| "選取資料格式錯誤".into())
-}
-
-pub async fn listen_pet_profile(
-    mut callback: impl FnMut(SavedPet) + 'static,
-) -> Result<(), String> {
-    let closure = Closure::<dyn FnMut(JsValue)>::new(move |v| {
-        if let Ok(pet) = serde_wasm_bindgen::from_value(v) {
-            callback(pet);
-        }
-    });
-    JsFuture::from(listen_pet_profile_js(closure.as_ref().unchecked_ref()))
-        .await
-        .map_err(js_error)?;
-    closure.forget(); // 與此單一 App 視窗同壽命。
-    Ok(())
-}
-
-pub async fn open_saved_pet(id: &str) -> Result<(), String> {
-    JsFuture::from(open_saved_pet_js(id))
-        .await
-        .map_err(js_error)?;
-    Ok(())
-}
-
-pub async fn cage_saved_pet() -> Result<(), String> {
-    JsFuture::from(cage_saved_pet_js())
-        .await
-        .map_err(js_error)?;
-    Ok(())
-}
-
-pub async fn cage_saved_pet_by_id(id: &str) -> Result<(), String> {
-    JsFuture::from(cage_saved_pet_by_id_js(id))
-        .await
-        .map_err(js_error)?;
-    Ok(())
-}
-
-/// Query the existing SQLite cage flag through the registered Tauri IPC command.
-pub async fn pet_cage_status_by_id(id: &str) -> Result<bool, String> {
-    let value = JsFuture::from(pet_cage_status_by_id_js(id))
-        .await
-        .map_err(js_error)?;
-    value.as_bool().ok_or_else(|| "回籠狀態回應格式錯誤".into())
-}
-
-pub async fn find_saved_pet(id: &str) -> Result<(), String> {
-    JsFuture::from(find_saved_pet_js(id))
-        .await
-        .map_err(js_error)?;
-    Ok(())
+    #[wasm_bindgen(js_namespace=window, js_name=getPetAutoRelease)]
+    fn get_pet_auto_release_js(id: &str) -> Promise;
+
+    #[wasm_bindgen(js_namespace=window, js_name=setPetAutoRelease)]
+    fn set_pet_auto_release_js(id: &str, enabled: bool) -> Promise;
+
+    #[wasm_bindgen(js_namespace=window, js_name=findSavedPet)]
+    fn find_saved_pet_js(id: &str) -> Promise;
+
+    #[wasm_bindgen(js_namespace=window, js_name=activePet)]
+    fn active_pet_js() -> Promise;
+
+    #[wasm_bindgen(js_namespace=window, js_name=selectPet)]
+    fn select_pet_js(id: &str) -> Promise;
+
+    #[wasm_bindgen(js_namespace=window, js_name=listenPetProfile)]
+    fn listen_pet_profile_js(callback: &Function) -> Promise;
+}
+
+// 將 JavaScript 的錯誤轉換為 Rust 的字串
+fn js_error(value: JsValue) -> String {
+    value.as_string().unwrap_or_else(|| format!("{value:?}"))
+}
+
+// 獲取寵物的當前狀態
+pub async fn get_pet_state() -> Result<PetSnapshot, String> {
+    let value = JsFuture::from(get_pet_state_js()).await.map_err(js_error)?;
+
+    serde_wasm_bindgen::from_value(value).map_err(|error| error.to_string())
+}
+
+// 發送指令給寵物並獲取更新後的狀態
+pub async fn send_pet_command(command: PetCommand) -> Result<PetSnapshot, String> {
+    let command = serde_wasm_bindgen::to_value(&command).map_err(|error| error.to_string())?;
+
+    let value = JsFuture::from(send_pet_command_js(command))
+        .await
+        .map_err(js_error)?;
+
+    serde_wasm_bindgen::from_value(value).map_err(|error| error.to_string())
+}
+
+// 監聽寵物狀態的變化
+pub async fn listen_pet_state(
+    mut on_snapshot: impl FnMut(PetSnapshot) + 'static,
+) -> Result<(), String> {
+    let callback = Closure::<dyn FnMut(JsValue)>::new(move |value| {
+        if let Ok(snapshot) = serde_wasm_bindgen::from_value(value) {
+            on_snapshot(snapshot);
+        }
+    });
+
+    JsFuture::from(listen_pet_state_js(callback.as_ref().unchecked_ref()))
+        .await
+        .map_err(js_error)?;
+
+    // listener 跟著整個 pet app 存活。
+    callback.forget();
+
+    Ok(())
+}
+
+// 顯示寵物的右鍵選單
+pub async fn show_pet_context_menu(x: i32, y: i32) -> Result<(), String> {
+    JsFuture::from(show_pet_context_menu_js(x, y))
+        .await
+        .map_err(js_error)?;
+
+    Ok(())
+}
+
+// 與寵物進行聊天
+pub async fn chat_with_pet(message: String) -> Result<String, String> {
+    let value = JsFuture::from(chat_with_pet_js(&message))
+        .await
+        .map_err(js_error)?;
+
+    value
+        .as_string()
+        .ok_or_else(|| "Failed to convert chat_with_pet result to string".to_string())
+}
+
+pub async fn scan_file_preview() -> Result<String, String> {
+    let value = JsFuture::from(scan_file_preview_js())
+        .await
+        .map_err(js_error)?;
+    let view: crate::file_scan_view::FileScanView =
+        serde_wasm_bindgen::from_value(value).map_err(|error| error.to_string())?;
+    Ok(view.summary())
+}
+
+pub async fn scan_duplicate_preview() -> Result<String, String> {
+    let value = JsFuture::from(scan_duplicate_preview_js())
+        .await
+        .map_err(js_error)?;
+    let result: crate::duplicate_preview::DuplicatePreviewView =
+        serde_wasm_bindgen::from_value(value).map_err(|e| e.to_string())?;
+    Ok(result.summary())
+}
+
+pub async fn open_controls() -> Result<(), String> {
+    JsFuture::from(open_controls_js()).await.map_err(js_error)?;
+
+    Ok(())
+}
+
+pub async fn quit_pet() -> Result<(), String> {
+    JsFuture::from(quit_pet_js()).await.map_err(js_error)?;
+
+    Ok(())
+}
+
+pub async fn create_pet(name: String) -> Result<SavedPet, String> {
+    let value = JsFuture::from(create_pet_js(&name))
+        .await
+        .map_err(js_error)?;
+    serde_wasm_bindgen::from_value(value)
+        .map_err(|_| "建立寵物回應格式錯誤；請重新整理清單確認是否已建立".into())
+}
+
+#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
+pub struct SavedPet {
+    pub id: String,
+    pub name: String,
+    pub species: String,
+}
+
+pub async fn list_pets() -> Result<Vec<SavedPet>, String> {
+    let value = JsFuture::from(list_pets_js()).await.map_err(js_error)?;
+    serde_wasm_bindgen::from_value(value).map_err(|_| "寵物清單格式錯誤".into())
+}
+
+pub async fn active_pet() -> Result<Option<SavedPet>, String> {
+    let value = JsFuture::from(active_pet_js()).await.map_err(js_error)?;
+    serde_wasm_bindgen::from_value(value).map_err(|_| "生活窗資料格式錯誤".into())
+}
+
+pub async fn select_pet(id: &str) -> Result<SavedPet, String> {
+    let value = JsFuture::from(select_pet_js(id)).await.map_err(js_error)?;
+    serde_wasm_bindgen::from_value(value).map_err(|_| "選取資料格式錯誤".into())
+}
+
+pub async fn listen_pet_profile(
+    mut callback: impl FnMut(SavedPet) + 'static,
+) -> Result<(), String> {
+    let closure = Closure::<dyn FnMut(JsValue)>::new(move |v| {
+        if let Ok(pet) = serde_wasm_bindgen::from_value(v) {
+            callback(pet);
+        }
+    });
+    JsFuture::from(listen_pet_profile_js(closure.as_ref().unchecked_ref()))
+        .await
+        .map_err(js_error)?;
+    closure.forget(); // 與此單一 App 視窗同壽命。
+    Ok(())
+}
+
+pub async fn open_saved_pet(id: &str) -> Result<(), String> {
+    JsFuture::from(open_saved_pet_js(id))
+        .await
+        .map_err(js_error)?;
+    Ok(())
+}
+
+pub async fn cage_saved_pet() -> Result<(), String> {
+    JsFuture::from(cage_saved_pet_js())
+        .await
+        .map_err(js_error)?;
+    Ok(())
+}
+
+pub async fn cage_saved_pet_by_id(id: &str) -> Result<(), String> {
+    JsFuture::from(cage_saved_pet_by_id_js(id))
+        .await
+        .map_err(js_error)?;
+    Ok(())
+}
+
+/// Query the existing SQLite cage flag through the registered Tauri IPC command.
+pub async fn pet_cage_status_by_id(id: &str) -> Result<bool, String> {
+    let value = JsFuture::from(pet_cage_status_by_id_js(id))
+        .await
+        .map_err(js_error)?;
+    value.as_bool().ok_or_else(|| "回籠狀態回應格式錯誤".into())
+}
+
+pub async fn find_saved_pet(id: &str) -> Result<(), String> {
+    JsFuture::from(find_saved_pet_js(id))
+        .await
+        .map_err(js_error)?;
+    Ok(())
+}
+
+pub async fn get_pet_auto_release(id: &str) -> Result<bool, String> {
+    let value = JsFuture::from(get_pet_auto_release_js(id)).await.map_err(js_error)?;
+    value.as_bool().ok_or_else(|| "預設出籠回應不是布林值".into())
+}
+
+pub async fn set_pet_auto_release(id: &str, enabled: bool) -> Result<bool, String> {
+    let value = JsFuture::from(set_pet_auto_release_js(id, enabled)).await.map_err(js_error)?;
+    value.as_bool().ok_or_else(|| "預設出籠回應不是布林值".into())
 }
diff --git a/pet_apps/pet_ui/src/main.rs b/pet_apps/pet_ui/src/main.rs
--- a/pet_apps/pet_ui/src/main.rs
+++ b/pet_apps/pet_ui/src/main.rs
@@ -1,295 +1,339 @@
-mod bridge;
-mod cage_status;
-mod component;
-mod duplicate_preview;
-mod file_scan_view;
-
-pub use component::*;
-
-use leptos::mount::mount_to_body;
-use leptos::{prelude::*, task::spawn_local};
-use pet_domain::PetSnapshot;
-use std::collections::HashMap;
-
-use crate::cage_status::record_cage_status;
-
-fn accept_scan_result(request_generation: u64, current_generation: u64) -> bool {
-    request_generation == current_generation
-}
-
-fn main() {
-    mount_to_body(|| {
-        view! {
-            <App />
-        }
-    });
-}
-
-#[component]
-fn App() -> impl IntoView {
-    let (snapshot, set_snapshot) = signal::<Option<PetSnapshot>>(None);
-
-    let (error, set_error) = signal::<Option<String>>(None);
-
-    let (blinking, set_blinking) = signal(false);
-
-    let (animation_phase, set_animation_phase) = signal(false);
-
-    let (chat_open, set_chat_open) = signal(false);
-    let (chat_input, set_chat_input) = signal(String::new());
-
-    let (chat_loading, set_chat_loading) = signal(false);
-    let (chat_response, set_chat_response) = signal::<Option<String>>(None);
-    let (chat_error, set_chat_error) = signal::<Option<String>>(None);
-
-    let (duplicate_status, set_duplicate_status) = signal::<Option<String>>(None);
-    let (duplicate_loading, set_duplicate_loading) = signal(false);
-
-    let (scan_loading, set_scan_loading) = signal(false);
-    let (scan_status, set_scan_status) = signal::<Option<String>>(None);
-
-    // 放在 App 的 signals 區塊；確認只針對目前顯示的掃描結果。
-    let (confirmed, set_confirmed) = signal(false);
-    let (generation, set_generation) = signal(0_u64);
-
-    let (new_pet_name, set_new_pet_name) = signal(String::new());
-    let (new_pet_feedback, set_new_pet_feedback) = signal(String::new());
-
-    let (saved_pets, set_saved_pets) = signal(Vec::<bridge::SavedPet>::new());
-    let (pets_busy, set_pets_busy) = signal(false);
-    let cage_status = RwSignal::new(HashMap::<String, String>::new());
-
-    let (active_pet, set_active_pet) = signal::<Option<bridge::SavedPet>>(None);
-    let profile_events = RwSignal::new(0_u64);
-    spawn_local(async move {
-        if let Err(message) = bridge::listen_pet_profile(move |pet| {
-            profile_events.update(|n| *n += 1);
-            set_active_pet.set(Some(pet));
-            set_snapshot.set(None);
-            set_error.set(None);
-            spawn_local(async move {
-                if let Ok(incoming) = bridge::get_pet_state().await {
-                    set_snapshot.set(Some(incoming));
-                }
-            });
-        })
-        .await
-        {
-            set_error.set(Some(message));
-        }
-        let generation = profile_events.get_untracked();
-        match bridge::active_pet().await {
-            Ok(pet) if profile_events.get_untracked() == generation => set_active_pet.set(pet),
-            Ok(_) => (), // 載入期間已收到較新的選取事件，不覆寫。
-            Err(message) => set_error.set(Some(message)),
-        }
-    });
-
-    if !bridge::is_pet_window() {
-        set_pets_busy.set(true);
-        spawn_local(async move {
-            match bridge::list_pets().await {
-                Ok(rows) => {
-                    set_saved_pets.set(rows);
-                    cage_status.set(HashMap::new());
-                    set_new_pet_feedback.set("已載入儲存清單".into());
-                }
-                Err(error) => set_new_pet_feedback.set(error),
-            }
-            set_pets_busy.set(false);
-        });
-    }
-
-    {
-        let snapshot_reader = snapshot;
-        let snapshot_writer = set_snapshot;
-        let error_writer = set_error;
-
-        spawn_local(async move {
-            let result = bridge::listen_pet_state(move |incoming| {
-                apply_snapshot(incoming, snapshot_reader, snapshot_writer);
-            })
-            .await;
-
-            if let Err(message) = result {
-                error_writer.set(Some(message));
-            }
-        });
-
-        spawn_local(async move {
-            match bridge::get_pet_state().await {
-                Ok(incoming) => {
-                    apply_snapshot(incoming, snapshot_reader, snapshot_writer);
-                }
-
-                Err(message) => {
-                    error_writer.set(Some(message));
-                }
-            }
-        });
-    }
-
-    if bridge::is_pet_window() {
-        return view! {
-            <main class="pet-life">
-                <DragHandle />
-                <BlinkController
-                    snapshot
-                    set_blinking
-                />
-                <AnimationClock set_animation_phase />
-
-                <p title=move || active_pet.get().map(|p|p.id).unwrap_or_default()>
-                    {move || active_pet.get().map(|p|p.name).unwrap_or_else(|| "請在控制台選擇生活窗寵物".into())}
-                </p>
-                <Show when=move || active_pet.get().is_some() fallback=|| ()>
-                    <PetStatus snapshot blinking animation_phase />
-                </Show>
-                <button type="button" on:click=move |_| spawn_local(async move {
-                    if let Err(e)=bridge::cage_saved_pet().await {set_error.set(Some(e));}
-               })>"回籠子"</button>
-                <p role="alert">{move || error.get().unwrap_or_default()}</p>
-                <button
-                    type="button"
-                    on:click=move |_| spawn_local(async move{
-                        if bridge::open_controls().await.is_err(){
-                            set_error.set(Some("無法開啟控制台".into()));
-                        }
-                    })
-                >"控制台"</button>
-            </main>
-        }
-        .into_any();
-    }
-
-    view! {
-        <main class="control-shell">
-            <h1>"寵物控制台"</h1>
-            <p>"控制台關閉後寵物仍運作；檔案功能目前只提供唯讀預覽"</p>
-
-            <section>
-                <h2>"新增寵物"</h2>
-                <label>"寵物名稱"
-                    <input
-                        prop:value=move || new_pet_name.get()
-                        on:input=move |ev| set_new_pet_name.set(event_target_value(&ev)) />
-                </label>
-                <button
-                    type="button"
-                    disabled=move || pets_busy.get()
-                    on:click=move |_| {
-                        if pets_busy.get_untracked() { return; }
-                        set_pets_busy.set(true);
-                        let proposed = new_pet_name.get_untracked();
-                        // apps/desktop_pet/frontend/src/main.rs
-                        // 僅替換「建立寵物」按鈕原有的 spawn_local(async move { ... }) 閉包。
-                        spawn_local(async move {
-                            match bridge::create_pet(proposed).await {
-                                Ok(created) => {
-                                    // 先更新清單，不因後續開窗失敗重送 INSERT。
-                                    match bridge::list_pets().await {
-                                        Ok(rows) => set_saved_pets.set(rows),
-                                        Err(_) => set_new_pet_feedback.set(
-                                            "寵物已儲存，但清單讀回失敗；請重新載入，勿重送建立。".into()
-                                        ),
-                                    }
-                                    // select_pet Tauri command 既有實作會先儲存
-                                    // local_pet_selection，再呼叫 pet_windows::open。
-                                    match bridge::select_pet(&created.id).await {
-                                        Ok(selected) => {
-                                            set_active_pet.set(Some(selected));
-                                            set_new_pet_name.set(String::new());
-                                            set_new_pet_feedback.set(
-                                                format!("已建立並出籠：{}（{}）",created.name,created.id)
-                                            );
-                                        }
-                                        Err(error) => set_new_pet_feedback.set(format!(
-                                            "寵物已儲存：{}（{}）；選取／出籠未完成：{}。請從清單重試，不要再建立。",
-                                            created.name, created.id, error
-                                        )),
-                                    }
-                                }
-                                Err(error) => set_new_pet_feedback.set(format!("建立寵物失敗：{error}")),
-                            }
-                            set_pets_busy.set(false);
-                        });
-                    }
-                >"建立寵物"</button>
-                <p role="status">{move || new_pet_feedback.get()}</p>
-                 <h3>"已儲存寵物"</h3>
-                <button type="button" disabled=move || pets_busy.get()
-                    on:click=move |_| {
-                        if pets_busy.get_untracked() { return; }
-                        set_pets_busy.set(true);
-                        spawn_local(async move {
-                            match bridge::list_pets().await {
-                                Ok(rows) => {
-                                    set_saved_pets.set(rows);
-                                    set_new_pet_feedback.set("已重新載入清單".into());
-                                }
-                                Err(error) => set_new_pet_feedback.set(format!("讀取清單失敗：{error}")),
-                            }
-                            set_pets_busy.set(false);
-                        });
-                    }>"重新載入清單"</button>
-                <Show when=move || pets_busy.get() fallback=|| ()><p>"處理中…"</p></Show>
-                <ul>
-                    <For each=move || saved_pets.get() key=|pet| pet.id.clone()
-                        children=move |pet| {
-                            let id=pet.id.clone();
-                            let open_id=id.clone();
-                            let cage_id=id.clone();
-                            let find_id=id.clone();
-                            let status_id=id.clone();
-                            let display_id=id.clone();
-                            view! { <li>{format!("{}／{}（{}）", pet.name, pet.species, pet.id)}
-                                <button type="button" disabled=move || pets_busy.get()
-                                    on:click=move |_| {
-                                        if pets_busy.get_untracked() { return; }
-                                        set_pets_busy.set(true); let id=id.clone();
-                                        spawn_local(async move {
-                                            match bridge::select_pet(&id).await {
-                                                Ok(pet) => {
-                                                    cage_status.update(|statuses| {
-                                                        statuses.remove(&id);
-                                                    });
-                                                    set_active_pet.set(Some(pet));
-                                                    set_new_pet_feedback.set("生活窗選取已儲存".into());
-                                                },
-                                                Err(message) => set_new_pet_feedback.set(message),
-                                            }
-                                            set_pets_busy.set(false);
-                                        });
-                                    }>"選為生活窗寵物"</button>
-                                <button type="button" disabled=move || pets_busy.get()
-                                    on:click=move |_| {
-                                        if pets_busy.get_untracked() {return;}
-                                        set_pets_busy.set(true);let id=open_id.clone();
-                                        spawn_local(async move {
-                                            match bridge::open_saved_pet(&id).await {
-                                                Ok(())=>{
-                                                    cage_status.update(|statuses| {
-                                                        statuses.remove(&id);
-                                                    });
-                                                    set_new_pet_feedback.set("已出籠；重複操作只聚焦既有生活窗".into());
-                                                }
-                                                ,Err(e)=>set_new_pet_feedback.set(e)
-                                            }
-                                            set_pets_busy.set(false);
-                                        });
-                                    }>"出籠"</button>
-                                <button type="button" disabled=move || pets_busy.get()
-                                    on:click=move |_| {
-                                        if pets_busy.get_untracked() { return; }
-                                        set_pets_busy.set(true);
-                                        let id = status_id.clone();
-                                        spawn_local(async move {
-                                            let result = bridge::pet_cage_status_by_id(&id).await;
-                                            cage_status.update(|statuses| record_cage_status(statuses, &id, result));
-                                            set_pets_busy.set(false);
-                                        });
-                                    }>"查詢回籠狀態"</button>
-                                <p role="status" aria-live="polite">{move || cage_status.get().get(&display_id).cloned().unwrap_or_default()}</p>
+mod auto_release_status;
+mod bridge;
+mod cage_status;
+mod component;
+mod duplicate_preview;
+mod file_scan_view;
+
+pub use component::*;
+
+use leptos::mount::mount_to_body;
+use leptos::{prelude::*, task::spawn_local};
+use pet_domain::PetSnapshot;
+use std::collections::HashMap;
+
+use crate::auto_release_status::record_auto_release;
+use crate::cage_status::record_cage_status;
+
+fn accept_scan_result(request_generation: u64, current_generation: u64) -> bool {
+    request_generation == current_generation
+}
+
+fn main() {
+    mount_to_body(|| {
+        view! {
+            <App />
+        }
+    });
+}
+
+#[component]
+fn App() -> impl IntoView {
+    let (snapshot, set_snapshot) = signal::<Option<PetSnapshot>>(None);
+
+    let (error, set_error) = signal::<Option<String>>(None);
+
+    let (blinking, set_blinking) = signal(false);
+
+    let (animation_phase, set_animation_phase) = signal(false);
+
+    let (chat_open, set_chat_open) = signal(false);
+    let (chat_input, set_chat_input) = signal(String::new());
+
+    let (chat_loading, set_chat_loading) = signal(false);
+    let (chat_response, set_chat_response) = signal::<Option<String>>(None);
+    let (chat_error, set_chat_error) = signal::<Option<String>>(None);
+
+    let (duplicate_status, set_duplicate_status) = signal::<Option<String>>(None);
+    let (duplicate_loading, set_duplicate_loading) = signal(false);
+
+    let (scan_loading, set_scan_loading) = signal(false);
+    let (scan_status, set_scan_status) = signal::<Option<String>>(None);
+
+    // 放在 App 的 signals 區塊；確認只針對目前顯示的掃描結果。
+    let (confirmed, set_confirmed) = signal(false);
+    let (generation, set_generation) = signal(0_u64);
+
+    let (new_pet_name, set_new_pet_name) = signal(String::new());
+    let (new_pet_feedback, set_new_pet_feedback) = signal(String::new());
+
+    let (saved_pets, set_saved_pets) = signal(Vec::<bridge::SavedPet>::new());
+    let (pets_busy, set_pets_busy) = signal(false);
+    let cage_status = RwSignal::new(HashMap::<String, String>::new());
+    let auto_release_status = RwSignal::new(HashMap::<String, String>::new());
+
+    let (active_pet, set_active_pet) = signal::<Option<bridge::SavedPet>>(None);
+    let profile_events = RwSignal::new(0_u64);
+    spawn_local(async move {
+        if let Err(message) = bridge::listen_pet_profile(move |pet| {
+            profile_events.update(|n| *n += 1);
+            set_active_pet.set(Some(pet));
+            set_snapshot.set(None);
+            set_error.set(None);
+            spawn_local(async move {
+                if let Ok(incoming) = bridge::get_pet_state().await {
+                    set_snapshot.set(Some(incoming));
+                }
+            });
+        })
+        .await
+        {
+            set_error.set(Some(message));
+        }
+        let generation = profile_events.get_untracked();
+        match bridge::active_pet().await {
+            Ok(pet) if profile_events.get_untracked() == generation => set_active_pet.set(pet),
+            Ok(_) => (), // 載入期間已收到較新的選取事件，不覆寫。
+            Err(message) => set_error.set(Some(message)),
+        }
+    });
+
+    if !bridge::is_pet_window() {
+        set_pets_busy.set(true);
+        spawn_local(async move {
+            match bridge::list_pets().await {
+                Ok(rows) => {
+                    set_saved_pets.set(rows);
+                    cage_status.set(HashMap::new());
+                    auto_release_status.set(HashMap::new());
+                    set_new_pet_feedback.set("已載入儲存清單".into());
+                }
+                Err(error) => set_new_pet_feedback.set(error),
+            }
+            set_pets_busy.set(false);
+        });
+    }
+
+    {
+        let snapshot_reader = snapshot;
+        let snapshot_writer = set_snapshot;
+        let error_writer = set_error;
+
+        spawn_local(async move {
+            let result = bridge::listen_pet_state(move |incoming| {
+                apply_snapshot(incoming, snapshot_reader, snapshot_writer);
+            })
+            .await;
+
+            if let Err(message) = result {
+                error_writer.set(Some(message));
+            }
+        });
+
+        spawn_local(async move {
+            match bridge::get_pet_state().await {
+                Ok(incoming) => {
+                    apply_snapshot(incoming, snapshot_reader, snapshot_writer);
+                }
+
+                Err(message) => {
+                    error_writer.set(Some(message));
+                }
+            }
+        });
+    }
+
+    if bridge::is_pet_window() {
+        return view! {
+            <main class="pet-life">
+                <DragHandle />
+                <BlinkController
+                    snapshot
+                    set_blinking
+                />
+                <AnimationClock set_animation_phase />
+
+                <p title=move || active_pet.get().map(|p|p.id).unwrap_or_default()>
+                    {move || active_pet.get().map(|p|p.name).unwrap_or_else(|| "請在控制台選擇生活窗寵物".into())}
+                </p>
+                <Show when=move || active_pet.get().is_some() fallback=|| ()>
+                    <PetStatus snapshot blinking animation_phase />
+                </Show>
+                <button type="button" on:click=move |_| spawn_local(async move {
+                    if let Err(e)=bridge::cage_saved_pet().await {set_error.set(Some(e));}
+               })>"回籠子"</button>
+                <p role="alert">{move || error.get().unwrap_or_default()}</p>
+                <button
+                    type="button"
+                    on:click=move |_| spawn_local(async move{
+                        if bridge::open_controls().await.is_err(){
+                            set_error.set(Some("無法開啟控制台".into()));
+                        }
+                    })
+                >"控制台"</button>
+            </main>
+        }
+        .into_any();
+    }
+
+    view! {
+        <main class="control-shell">
+            <h1>"寵物控制台"</h1>
+            <p>"控制台關閉後寵物仍運作；檔案功能目前只提供唯讀預覽"</p>
+
+            <section>
+                <h2>"新增寵物"</h2>
+                <label>"寵物名稱"
+                    <input
+                        prop:value=move || new_pet_name.get()
+                        on:input=move |ev| set_new_pet_name.set(event_target_value(&ev)) />
+                </label>
+                <button
+                    type="button"
+                    disabled=move || pets_busy.get()
+                    on:click=move |_| {
+                        if pets_busy.get_untracked() { return; }
+                        set_pets_busy.set(true);
+                        let proposed = new_pet_name.get_untracked();
+                        // apps/desktop_pet/frontend/src/main.rs
+                        // 僅替換「建立寵物」按鈕原有的 spawn_local(async move { ... }) 閉包。
+                        spawn_local(async move {
+                            match bridge::create_pet(proposed).await {
+                                Ok(created) => {
+                                    // 先更新清單，不因後續開窗失敗重送 INSERT。
+                                    match bridge::list_pets().await {
+                                        Ok(rows) => set_saved_pets.set(rows),
+                                        Err(_) => set_new_pet_feedback.set(
+                                            "寵物已儲存，但清單讀回失敗；請重新載入，勿重送建立。".into()
+                                        ),
+                                    }
+                                    // select_pet Tauri command 既有實作會先儲存
+                                    // local_pet_selection，再呼叫 pet_windows::open。
+                                    match bridge::select_pet(&created.id).await {
+                                        Ok(selected) => {
+                                            set_active_pet.set(Some(selected));
+                                            set_new_pet_name.set(String::new());
+                                            set_new_pet_feedback.set(
+                                                format!("已建立並出籠：{}（{}）",created.name,created.id)
+                                            );
+                                        }
+                                        Err(error) => set_new_pet_feedback.set(format!(
+                                            "寵物已儲存：{}（{}）；選取／出籠未完成：{}。請從清單重試，不要再建立。",
+                                            created.name, created.id, error
+                                        )),
+                                    }
+                                }
+                                Err(error) => set_new_pet_feedback.set(format!("建立寵物失敗：{error}")),
+                            }
+                            set_pets_busy.set(false);
+                        });
+                    }
+                >"建立寵物"</button>
+                <p role="status">{move || new_pet_feedback.get()}</p>
+                 <h3>"已儲存寵物"</h3>
+                <button type="button" disabled=move || pets_busy.get()
+                    on:click=move |_| {
+                        if pets_busy.get_untracked() { return; }
+                        set_pets_busy.set(true);
+                        spawn_local(async move {
+                            match bridge::list_pets().await {
+                                Ok(rows) => {
+                                    set_saved_pets.set(rows);
+                                    auto_release_status.set(HashMap::new());
+                                    set_new_pet_feedback.set("已重新載入清單".into());
+                                }
+                                Err(error) => set_new_pet_feedback.set(format!("讀取清單失敗：{error}")),
+                            }
+                            set_pets_busy.set(false);
+                        });
+                    }>"重新載入清單"</button>
+                <Show when=move || pets_busy.get() fallback=|| ()><p>"處理中…"</p></Show>
+                <ul>
+                    <For each=move || saved_pets.get() key=|pet| pet.id.clone()
+                        children=move |pet| {
+                            let id=pet.id.clone();
+                            let open_id=id.clone();
+                            let cage_id=id.clone();
+                            let find_id=id.clone();
+                            let status_id=id.clone();
+                            let display_id=id.clone();
+                            let pref_get_id=id.clone();
+                            let pref_set_id=id.clone();
+                            let pref_unset_id=id.clone();
+                            let pref_display_id=id.clone();
+                            view! { <li>{format!("{}／{}（{}）", pet.name, pet.species, pet.id)}
+                                <button type="button" disabled=move || pets_busy.get()
+                                    on:click=move |_| {
+                                        if pets_busy.get_untracked() { return; }
+                                        set_pets_busy.set(true); let id=id.clone();
+                                        spawn_local(async move {
+                                            match bridge::select_pet(&id).await {
+                                                Ok(pet) => {
+                                                    cage_status.update(|statuses| {
+                                                        statuses.remove(&id);
+                                                    });
+                                                    set_active_pet.set(Some(pet));
+                                                    set_new_pet_feedback.set("生活窗選取已儲存".into());
+                                                },
+                                                Err(message) => set_new_pet_feedback.set(message),
+                                            }
+                                            set_pets_busy.set(false);
+                                        });
+                                    }>"選為生活窗寵物"</button>
+                                <button type="button" disabled=move || pets_busy.get()
+                                    on:click=move |_| {
+                                        if pets_busy.get_untracked() {return;}
+                                        set_pets_busy.set(true);let id=open_id.clone();
+                                        spawn_local(async move {
+                                            match bridge::open_saved_pet(&id).await {
+                                                Ok(())=>{
+                                                    cage_status.update(|statuses| {
+                                                        statuses.remove(&id);
+                                                    });
+                                                    set_new_pet_feedback.set("已出籠；重複操作只聚焦既有生活窗".into());
+                                                }
+                                                ,Err(e)=>set_new_pet_feedback.set(e)
+                                            }
+                                            set_pets_busy.set(false);
+                                        });
+                                    }>"出籠"</button>
+                                <button type="button" disabled=move || pets_busy.get()
+                                    on:click=move |_| {
+                                        if pets_busy.get_untracked() { return; }
+                                        set_pets_busy.set(true);
+                                        let id = status_id.clone();
+                                        spawn_local(async move {
+                                            let result = bridge::pet_cage_status_by_id(&id).await;
+                                            cage_status.update(|statuses| record_cage_status(statuses, &id, result));
+                                            set_pets_busy.set(false);
+                                        });
+                                    }>"查詢回籠狀態"</button>
+                                <p role="status" aria-live="polite">{move || cage_status.get().get(&display_id).cloned().unwrap_or_default()}</p>
+                                <button type="button" disabled=move || pets_busy.get()
+                                    on:click=move |_| {
+                                        if pets_busy.get_untracked() { return; }
+                                        set_pets_busy.set(true);
+                                        let id=pref_get_id.clone();
+                                        spawn_local(async move {
+                                            let result=bridge::get_pet_auto_release(&id).await;
+                                            auto_release_status.update(|s| record_auto_release(s,&id,result));
+                                            set_pets_busy.set(false);
+                                        });
+                                    }>"查詢預設出籠"</button>
+                                <button type="button" disabled=move || pets_busy.get()
+                                    on:click=move |_| {
+                                        if pets_busy.get_untracked() { return; }
+                                        set_pets_busy.set(true);
+                                        let id=pref_set_id.clone();
+                                        spawn_local(async move {
+                                            let result=bridge::set_pet_auto_release(&id,true).await;
+                                            auto_release_status.update(|s| record_auto_release(s,&id,result));
+                                            set_pets_busy.set(false);
+                                        });
+                                    }>"設為啟動時出籠"</button>
+                                <button type="button" disabled=move || pets_busy.get()
+                                    on:click=move |_| {
+                                        if pets_busy.get_untracked() { return; }
+                                        set_pets_busy.set(true);
+                                        let id=pref_unset_id.clone();
+                                        spawn_local(async move {
+                                            let result=bridge::set_pet_auto_release(&id,false).await;
+                                            auto_release_status.update(|s| record_auto_release(s,&id,result));
+                                            set_pets_busy.set(false);
+                                        });
+                                    }>"取消啟動時出籠"</button>
+                                <p role="status" aria-live="polite">{move || auto_release_status.get().get(&pref_display_id).cloned().unwrap_or_default()}</p>
+
                                 <button type="button" disabled=move || pets_busy.get()
                                     on:click=move |_| {
                                         if pets_busy.get_untracked() { return; }
diff --git a/pet_apps/migrations/20261030060000_auto_release_preference.sql b/pet_apps/migrations/20261030060000_auto_release_preference.sql
new file mode 100644
--- /dev/null
+++ b/pet_apps/migrations/20261030060000_auto_release_preference.sql
@@ -0,0 +1,9 @@
+-- Separate restart preference from current cage state; never rewrite the original migration.
+ALTER TABLE pet_profile
+    ADD COLUMN auto_release_on_start BOOLEAN NOT NULL DEFAULT FALSE;
+
+-- Preserve the pre-migration restart set for existing rows. Future cage actions
+-- only change is_default_closure, never this preference.
+UPDATE pet_profile
+SET auto_release_on_start =
+    CASE WHEN COALESCE(is_default_closure, 0) = 0 THEN 1 ELSE 0 END;
diff --git a/pet_apps/pet_ui/src/auto_release_status.rs b/pet_apps/pet_ui/src/auto_release_status.rs
new file mode 100644
--- /dev/null
+++ b/pet_apps/pet_ui/src/auto_release_status.rs
@@ -0,0 +1,29 @@
+use std::collections::HashMap;
+
+/// Per-UUID feedback; an IPC error is not an unchecked/false preference.
+pub(crate) fn record_auto_release(
+    statuses: &mut HashMap<String, String>,
+    id: &str,
+    result: Result<bool, String>,
+) {
+    let text = match result {
+        Ok(true) => "啟動時預設出籠：是".to_owned(),
+        Ok(false) => "啟動時預設出籠：否".to_owned(),
+        Err(error) => format!("預設出籠資格操作失敗：{error}"),
+    };
+    statuses.insert(id.to_owned(), text);
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    #[test]
+    fn independent_preferences_and_ipc_failures_do_not_claim_false() {
+        let mut statuses = HashMap::new();
+        record_auto_release(&mut statuses, "a", Ok(true));
+        record_auto_release(&mut statuses, "b", Ok(false));
+        record_auto_release(&mut statuses, "a", Err("SQLite".into()));
+        assert!(statuses["a"].contains("操作失敗"));
+        assert_eq!(statuses["b"], "啟動時預設出籠：否");
+    }
+}
```

**舊測試去向**：`pet_persistence::day36_restore_adapter_tests` 仍驗證「未回籠集合」的資料層舊契約，但它不再代表啟動政策；新的 `day44_auto_release_tests` 驗證「預設資格集合」。`pet_application::lifecycle` 既有開窗失敗 rollback 測試直接承接，不複製成假 native test；`pet_server::tests` 與 Node bridge 原測試不刪。新 UI 純單元測試只證明顯示文字，Tauri/SQLite/GUI 還需 Gate2。

## ③ 2026-10-30 官方穩定版與四 workspace 相容性

官方 Rust release 頁確認 Rust stable **1.99.1（2026-10-15）**，`pet_apps/rust-toolchain.toml` 為 `stable`，但本次無已安裝 rustc/cargo。已讀 `pet_apps/Cargo.toml`／`pet_server/Cargo.toml`／`pet_ui/Cargo.toml`／`pet_apps/Cargo.lock@b9805c0aefb65df8972bd6d3d17547ec2781f6f5`。下表「宣告 → lock → 最新 stable」皆不表示已升級：

| 套件／工具 | manifest → lock → 最新 stable | 官方來源 |
| --- | --- | --- |
| Rust | stable → 未安裝可核 → 1.99.1 | https://blog.rust-lang.org/releases/ |
| Tauri | ^2.12 → 2.12.5 → 2.12.5（MSRV ≥1.90） | https://docs.rs/crate/tauri/latest |
| tauri-build | ^2.7 → 2.7.4 → 2.7.4（3.0 alpha 排除） | https://docs.rs/crate/tauri-build/latest |
| Leptos CSR | ^0.8 → 0.8.22 → 0.8.22（0.9 RC 排除） | https://docs.rs/crate/leptos/latest |
| SQLx SQLite | ^0.9 → 0.9.0 → 0.9.0（MSRV 1.94） | https://docs.rs/crate/sqlx/latest |
| Tokio | ^1.54 → 1.54.1 → 1.54.1 | https://docs.rs/crate/tokio/latest |
| UUID | ^1.29 → 1.29.0 → 1.29.0 | https://docs.rs/crate/uuid/latest |
| serde / serde_json | ^1 / ^1 → 1.0.229 / 1.0.154 → 相同 | https://docs.rs/crate/serde/latest · https://docs.rs/crate/serde_json/latest |
| wasm-bindgen / web-sys | ^0.2 / ^0.3 → 0.2.129 / 0.3.106 → 相同 | https://docs.rs/crate/wasm-bindgen/latest · https://docs.rs/crate/web-sys/latest |
| gloo-timers | ^0.4 → 0.4.0 → 0.4.0 | https://docs.rs/crate/gloo-timers/latest |

四 workspace 的 `Cargo.lock` 已按當次精確 SHA 讀取，`crates` 仍是獨立 workspace；本篇沒有改跨 workspace 依賴。Native Linux WebKitGTK、Tauri 2 runtime、WASM target、Node 與真 SQLite schema 需要分開驗證；僅從 lock 無法證明相容。

## ④ 驗證／交付／回復與剩餘缺口

**Gate1**：GitHub 真實來源逐檔全文讀取、依原文產生兩份順序 patch、正式 UI→JS→Tauri→SQLx→啟動 caller 與測試完整列出；提交後另記精確 commit／blob／SHA256／全文讀回。靜態字串匹配不是 `git apply`、Cargo、SQLite 或 GUI 實測。

**Gate2 全部未執行**：本環境無本地產品 checkout、Rust/Cargo、SQLite 專用測試 DB、Tauri native GUI／WASM／Node、產品 CI 執行權限；沒有修改 `rust_ai_ring`。使用者在隔離副本、**備份現有 SQLite** 後按 Refactor→Green 套用，執行：
```bash
git apply --check pet-day44-refactor.patch
git apply pet-day44-refactor.patch
git apply --check pet-day44-green.patch
git apply pet-day44-green.patch
cargo fmt --manifest-path pet_apps/Cargo.toml --all -- --check
cargo test --manifest-path pet_apps/Cargo.toml --workspace
cargo check --manifest-path pet_apps/Cargo.toml -p pet-ui --target wasm32-unknown-unknown
cargo check --manifest-path pet_apps/Cargo.toml -p pet-server
```
再做有資料舊 SQLite migration（備份／重啟／冪等／checksum）、控制台逐 A/B 設定、A 回籠後重新啟動、三隻同時出籠、取消預設、native CloseRequested、失敗補償與多螢幕測試。**DB migration 不提供破壞資料的自動 down；回復需使用升級前備份**。新寵物需手動設為預設；未來可增強首次建立預設 UX。原 Day25 單幀圖片 FIFO `pet-day25-single-image-20261022` 仍 pending；本篇不補完、不增加額外 Day。核心60分鐘僅 Red/Green 預估15分鐘，Refactor、版本、交付另計。
