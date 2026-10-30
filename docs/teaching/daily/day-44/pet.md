# Pet Day44 修訂 r3｜活動決定出籠，坐下優先

來源唯讀 `6b34c42f74fc1585b1b864fb7dd6969318270e0f`，產品未套用本修補。完整替代Green現已交付且隔離實測；原新增欄位版本只保留在[第八封存包](../../../archive/packages-2026-10-30/08-pet-activity-contract-superseded-2026-10-30.zip)。next_day保持45，修訂不新增Day。以下功能、獨立Refactor與產品驗收分列。

## ① 目標、契約與正式呼叫鏈

`pet_activity` 同pet_id／owner_id任一 `is_activity=true` 就是出籠，不限散步、可以多筆true；全部false／無活動才回籠。不存在或非本機主人的寵物回錯誤，不能偽裝回籠。`is_default_closure` **只作既有啟動預設**，不再讀寫為目前籠態；沒有新增欄位或第二份目前狀態。

| 活動 | 籠態／移動 |
| --- | --- |
| 散步true、坐下false | 出籠，允許漫遊 |
| 坐下true | 出籠，不漫遊；若散步亦true，保留坐下並持久化散步false |
| 其他活動true | 出籠，不自行開啟漫遊 |
| 全false／無活動 | 回籠 |

新建由 `PetStore::create` 同交易新增owner（若必要）、profile與散步／坐下兩筆inactive活動，任何一步失敗整筆rollback。現有UI之後明確select／出籠，不把「活動記錄存在」當活動已啟用。重複出籠只保留已有動作／聚焦，不重複插入；坐下者再出籠仍坐下。回籠／Native X停止該隻全部活動，不動其他寵物或預設資格。

呼叫鏈：Leptos `ActivityControls` → bridge／index.html真invoke → Tauri UUID command → PetStore → persistence activity交易。runtime使用同一lifecycle鎖；DB先保存，原生視窗／選取失敗則還原**完整活動快照及選取slot**，不能只用bool補償而丟失坐下或其他活動。Native X與右鍵回籠共用同一實作。walk／sit命令重啟該ID的唯一漫遊generation；每次位移前在同鎖中重新核對DB與generation，正規化失敗停止移動。拖曳／睡覺只暫停位移，不回籠。

散步是許可；實際漫遊仍受產品既有 `PET_ENABLE_ROAM` 環境opt-in及runtime暫停條件限制，未新增DB漫遊旗標。控制台按UUID查詢／切換動作及既有預設欄位；共用Day43狀態map與busy，避免舊籠態或跨寵物覆寫。

啟動只恢復預設可出籠者；其他非預設寵物停止活動，避免沒有視窗卻顯示出籠。預設者保留既有坐下／其他活動，無活動才啟用散步。回籠不取消預設資格。

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

本案 Refactor 已在7ce基準真check／apply，且最新6b34已有實作。保留幾何邊界／負座標舊斷言；最新main100項與候選110項皆涵蓋此模組。此幾何重構獨立於活動Green，不抵算跨案工程。


## ② Red → 完整 Green

### 同一建立行為測試的實際 Red／Green

以下只呼叫基準已存在的API。未補新符號當Red；基準6b34真實失敗是活動集合為空，預期兩筆inactive。完全相同測試在候選通過。第一次Green因共用target的舊依賴編譯失敗另有紀錄；強制本地crate重建後才取得成功。此誤差不算行為Red。

```rust
use pet_server::pet_store::PetStore;
#[sqlx::test(migrations="../migrations")]
async fn new_pet_has_two_inactive_base_activities(pool:sqlx::SqlitePool) {
    let store=PetStore{pool:pool.clone()};let pet=store.create("驗收寵物").await.unwrap();
    let activities:Vec<(String,i64)>=sqlx::query_as("SELECT activity_type,is_activity FROM pet_activity WHERE pet_id=?1 AND owner_id='local-owner' ORDER BY activity_type")
        .bind(&pet.id).fetch_all(&pool).await.unwrap();
    assert_eq!(activities,vec![("坐下".into(),0),("散步".into(),0)]);
}
```

### 完整逐檔實作／測試／接線

最新6b34已含上節幾何Refactor，直接套用 [Green r3](pet-day44-activity-green-r3.patch)。**不要再套原Refactor**。若重現原7ce教學，先原Refactor，再套[after-refactor專用Green](pet-day44-activity-green-r3-after-refactor.patch)；兩份Green擇一，不能疊加。兩條路徑均真check／apply，最新路徑14檔與受測來源hash完全相同。

```bash
# 在6b34的隔離可寫clone，不在原產品
git apply --check /path/to/pet-day44-activity-green-r3.patch
git apply /path/to/pet-day44-activity-green-r3.patch
cargo metadata --locked --no-deps --manifest-path pet_apps/Cargo.toml --format-version 1
cargo test --locked --workspace --manifest-path pet_apps/Cargo.toml
```

此patch包含全部新增／修改單元：純Application快照補償、SQLx activity與舊API接線、Store建寵／select／startup、Tauri/native/menu/roam、Leptos與JS invoke、資料migration、10項真SQLite整合測試、2項Node bridge測試及保資料升級example。Cargo依賴與features不變；CSR／native沿既有workspace，不新增架構。舊布林helper保留相容測試，正式native caller改用精確snapshot補償。

```diff
diff --git a/pet_apps/migrations/20261030060000_activity_lifecycle.sql b/pet_apps/migrations/20261030060000_activity_lifecycle.sql
new file mode 100644
index 0000000..e291154
--- /dev/null
+++ b/pet_apps/migrations/20261030060000_activity_lifecycle.sql
@@ -0,0 +1,17 @@
+-- Data-only forward migration: no columns, no deletion, no edits to deployed checksums.
+-- For legacy profiles with no activities, preserve the old outside state once as walking.
+INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity,is_use_ai)
+SELECT 'initial-walk:' || p.id,p.id,p.owner_id,'散步',
+       CASE WHEN COALESCE(p.is_default_closure,0)=0
+                 AND NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=p.id AND a.owner_id=p.owner_id)
+            THEN 1 ELSE 0 END,0
+FROM pet_profile p
+WHERE NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=p.id AND a.owner_id=p.owner_id AND a.activity_type='散步');
+INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity,is_use_ai)
+SELECT 'initial-sit:' || p.id,p.id,p.owner_id,'坐下',0,0
+FROM pet_profile p
+WHERE NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=p.id AND a.owner_id=p.owner_id AND a.activity_type='坐下');
+-- Sitting wins even if there are multiple records for either activity type.
+UPDATE pet_activity AS w SET is_activity=0
+WHERE w.activity_type='散步' AND w.is_activity=1
+AND EXISTS(SELECT 1 FROM pet_activity s WHERE s.pet_id=w.pet_id AND s.owner_id=w.owner_id AND s.activity_type='坐下' AND s.is_activity=1);
diff --git a/pet_apps/pet_application/src/lifecycle.rs b/pet_apps/pet_application/src/lifecycle.rs
index 1645c4a..e9ef386 100644
--- a/pet_apps/pet_application/src/lifecycle.rs
+++ b/pet_apps/pet_application/src/lifecycle.rs
@@ -231,3 +231,27 @@ mod tests {
         });
     }
 }
+
+/// Preserve an exact activity snapshot on native failure; a single cage boolean
+/// cannot restore sitting or simultaneous non-walking activities.
+pub async fn persist_then_apply<W, WF, A, R, RF>(
+    write: W,
+    apply: A,
+    restore: R,
+) -> Result<(), String>
+where
+    W: FnOnce() -> WF,
+    WF: std::future::Future<Output = Result<(), String>>,
+    A: FnOnce() -> Result<(), String>,
+    R: FnOnce() -> RF,
+    RF: std::future::Future<Output = Result<(), String>>,
+{
+    write().await?;
+    if let Err(error) = apply() {
+        return match restore().await {
+            Ok(()) => Err(format!("原生操作失敗，已恢復活動狀態：{error}")),
+            Err(rollback) => Err(format!("原生操作失敗：{error}；活動補償失敗：{rollback}")),
+        };
+    }
+    Ok(())
+}
diff --git a/pet_apps/pet_persistence/src/activity.rs b/pet_apps/pet_persistence/src/activity.rs
new file mode 100644
index 0000000..2f84902
--- /dev/null
+++ b/pet_apps/pet_persistence/src/activity.rs
@@ -0,0 +1,211 @@
+use sqlx::{SqliteConnection, SqlitePool};
+
+pub const WALK: &str = "散步";
+pub const SIT: &str = "坐下";
+
+#[derive(Debug, Clone, serde::Serialize)]
+pub struct ActivityStatus {
+    pub outside: bool,
+    pub roaming: bool,
+    pub default_caged: bool,
+}
+
+#[derive(Debug, Clone)]
+pub struct ActivitySnapshot {
+    values: Vec<(String, Option<i64>)>,
+    selected: Option<String>,
+}
+
+async fn local(conn: &mut SqliteConnection, id: &str) -> Result<(), String> {
+    let found: bool = sqlx::query_scalar(
+        "SELECT EXISTS(SELECT 1 FROM pet_profile WHERE id=?1 AND owner_id='local-owner')",
+    )
+    .bind(id)
+    .fetch_one(conn)
+    .await
+    .map_err(|_| "無法驗證寵物".to_owned())?;
+    if found {
+        Ok(())
+    } else {
+        Err("找不到寵物".into())
+    }
+}
+
+/// Shared by profile creation and legacy write paths, inside the caller's transaction.
+/// Repeated operations never add a duplicate basic activity; other types are preserved.
+pub async fn ensure_pair(conn: &mut SqliteConnection, id: &str) -> Result<(), String> {
+    local(conn, id).await?;
+    for kind in [WALK, SIT] {
+        sqlx::query("INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity,is_use_ai) SELECT ?1,?2,'local-owner',?3,0,0 WHERE NOT EXISTS(SELECT 1 FROM pet_activity WHERE pet_id=?2 AND owner_id='local-owner' AND activity_type=?3)")
+            .bind(uuid::Uuid::new_v4().to_string()).bind(id).bind(kind).execute(&mut *conn).await.map_err(|_| "無法建立基本活動".to_owned())?;
+    }
+    Ok(())
+}
+
+async fn normalize(conn: &mut SqliteConnection, id: &str) -> Result<(), String> {
+    sqlx::query("UPDATE pet_activity SET is_activity=0,updated_at=CURRENT_TIMESTAMP WHERE pet_id=?1 AND owner_id='local-owner' AND activity_type=?2 AND is_activity=1 AND EXISTS(SELECT 1 FROM pet_activity s WHERE s.pet_id=?1 AND s.owner_id='local-owner' AND s.activity_type=?3 AND s.is_activity=1)")
+        .bind(id).bind(WALK).bind(SIT).execute(conn).await.map_err(|_| "無法保存坐下優先狀態".to_owned())?;
+    Ok(())
+}
+
+async fn status_in(conn: &mut SqliteConnection, id: &str) -> Result<ActivityStatus, String> {
+    local(&mut *conn, id).await?;
+    let (outside,walk,sit): (bool,bool,bool) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM pet_activity WHERE pet_id=?1 AND owner_id='local-owner' AND is_activity=1),EXISTS(SELECT 1 FROM pet_activity WHERE pet_id=?1 AND owner_id='local-owner' AND activity_type=?2 AND is_activity=1),EXISTS(SELECT 1 FROM pet_activity WHERE pet_id=?1 AND owner_id='local-owner' AND activity_type=?3 AND is_activity=1)")
+        .bind(id).bind(WALK).bind(SIT).fetch_one(&mut *conn).await.map_err(|_| "無法讀取活動狀態".to_owned())?;
+    if walk && sit {
+        normalize(&mut *conn, id).await?;
+    }
+    let pref: i64=sqlx::query_scalar("SELECT COALESCE(is_default_closure,0) FROM pet_profile WHERE id=?1 AND owner_id='local-owner'")
+        .bind(id).fetch_one(&mut *conn).await.map_err(|_| "無法讀取預設資格".to_owned())?;
+    if !matches!(pref, 0 | 1) {
+        return Err("預設資格資料不合法".into());
+    }
+    Ok(ActivityStatus {
+        outside,
+        roaming: walk && !sit,
+        default_caged: pref == 1,
+    })
+}
+
+pub async fn status(pool: &SqlitePool, id: &str) -> Result<ActivityStatus, String> {
+    let mut tx = pool.begin().await.map_err(|_| "無法讀取活動".to_owned())?;
+    let state = status_in(&mut tx, id).await?;
+    tx.commit()
+        .await
+        .map_err(|_| "活動讀取提交失敗".to_owned())?;
+    Ok(state)
+}
+
+pub async fn set_caged_in(
+    conn: &mut SqliteConnection,
+    id: &str,
+    caged: bool,
+) -> Result<(), String> {
+    ensure_pair(conn, id).await?;
+    normalize(conn, id).await?;
+    if caged {
+        sqlx::query("UPDATE pet_activity SET is_activity=0,updated_at=CURRENT_TIMESTAMP WHERE pet_id=?1 AND owner_id='local-owner' AND is_activity=1")
+            .bind(id).execute(conn).await.map_err(|_| "無法停止寵物活動".to_owned())?;
+    } else {
+        // Preserve sitting/other active modes on repeated open; only caged pets start walking.
+        let active:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pet_activity WHERE pet_id=?1 AND owner_id='local-owner' AND is_activity=1)")
+            .bind(id).fetch_one(&mut *conn).await.map_err(|_| "無法讀取活動".to_owned())?;
+        if !active {
+            sqlx::query("UPDATE pet_activity SET is_activity=1,updated_at=CURRENT_TIMESTAMP WHERE pet_id=?1 AND owner_id='local-owner' AND activity_type=?2")
+                .bind(id).bind(WALK).execute(conn).await.map_err(|_| "無法開始散步".to_owned())?;
+        }
+    }
+    Ok(())
+}
+
+pub async fn set_caged(pool: &SqlitePool, id: &str, caged: bool) -> Result<(), String> {
+    let mut tx = pool
+        .begin()
+        .await
+        .map_err(|_| "無法開始活動交易".to_owned())?;
+    set_caged_in(&mut tx, id, caged).await?;
+    tx.commit().await.map_err(|_| "無法提交活動".to_owned())
+}
+
+pub async fn switch(pool: &SqlitePool, id: &str, walking: bool) -> Result<ActivityStatus, String> {
+    let mut tx = pool
+        .begin()
+        .await
+        .map_err(|_| "無法開始活動交易".to_owned())?;
+    ensure_pair(&mut tx, id).await?;
+    sqlx::query("UPDATE pet_activity SET is_activity=(activity_type=?2),updated_at=CURRENT_TIMESTAMP WHERE pet_id=?1 AND owner_id='local-owner' AND activity_type IN (?3,?4)")
+        .bind(id).bind(if walking {WALK} else {SIT}).bind(WALK).bind(SIT).execute(&mut *tx).await.map_err(|_|"無法切換活動".to_owned())?;
+    let state = status_in(&mut tx, id).await?;
+    tx.commit()
+        .await
+        .map_err(|_| "無法提交活動切換".to_owned())?;
+    Ok(state)
+}
+
+pub async fn set_default_caged(
+    pool: &SqlitePool,
+    id: &str,
+    caged: bool,
+) -> Result<ActivityStatus, String> {
+    let mut tx = pool
+        .begin()
+        .await
+        .map_err(|_| "無法開始預設資格交易".to_owned())?;
+    local(&mut tx, id).await?;
+    sqlx::query("UPDATE pet_profile SET is_default_closure=?1,updated_at=CURRENT_TIMESTAMP WHERE id=?2 AND owner_id='local-owner'")
+        .bind(caged).bind(id).execute(&mut *tx).await.map_err(|_|"無法保存預設資格".to_owned())?;
+    let state = status_in(&mut tx, id).await?;
+    tx.commit()
+        .await
+        .map_err(|_| "預設資格提交失敗".to_owned())?;
+    Ok(state)
+}
+
+pub async fn prepare_startup(pool: &SqlitePool) -> Result<Vec<(String, String, String)>, String> {
+    let mut tx = pool
+        .begin()
+        .await
+        .map_err(|_| "無法開始啟動交易".to_owned())?;
+    let invalid:i64=sqlx::query_scalar("SELECT count(*) FROM pet_profile WHERE owner_id='local-owner' AND COALESCE(is_default_closure,0) NOT IN (0,1)")
+        .fetch_one(&mut *tx).await.map_err(|_|"無法讀取預設資格".to_owned())?;
+    if invalid != 0 {
+        return Err("預設資格資料不合法".into());
+    }
+    sqlx::query("UPDATE pet_activity SET is_activity=0,updated_at=CURRENT_TIMESTAMP WHERE owner_id='local-owner' AND is_activity=1 AND EXISTS(SELECT 1 FROM pet_profile p WHERE p.id=pet_activity.pet_id AND p.owner_id=pet_activity.owner_id AND p.is_default_closure=1)")
+        .execute(&mut *tx).await.map_err(|_|"無法準備啟動活動".to_owned())?;
+    let rows=sqlx::query_as("SELECT id,name,species FROM pet_profile WHERE owner_id='local-owner' AND COALESCE(is_default_closure,0)=0 ORDER BY name,id")
+        .fetch_all(&mut *tx).await.map_err(|_|"無法讀取啟動寵物".to_owned())?;
+    tx.commit()
+        .await
+        .map_err(|_| "啟動活動提交失敗".to_owned())?;
+    Ok(rows)
+}
+
+pub async fn snapshot(pool: &SqlitePool, id: &str) -> Result<ActivitySnapshot, String> {
+    // Resolve sit priority before capturing a compensating state.
+    status(pool, id).await?;
+    let mut tx = pool
+        .begin()
+        .await
+        .map_err(|_| "無法讀取活動快照".to_owned())?;
+    ensure_pair(&mut tx, id).await?;
+    let values=sqlx::query_as("SELECT id,is_activity FROM pet_activity WHERE pet_id=?1 AND owner_id='local-owner' ORDER BY id")
+        .bind(id).fetch_all(&mut *tx).await.map_err(|_| "無法讀取活動快照".to_owned())?;
+    let selected = sqlx::query_scalar("SELECT pet_id FROM local_pet_selection WHERE slot=1")
+        .fetch_optional(&mut *tx)
+        .await
+        .map_err(|_| "無法讀取選取快照".to_owned())?;
+    tx.commit().await.map_err(|_| "快照提交失敗".to_owned())?;
+    Ok(ActivitySnapshot { values, selected })
+}
+
+pub async fn restore(
+    pool: &SqlitePool,
+    id: &str,
+    snapshot: &ActivitySnapshot,
+) -> Result<(), String> {
+    let mut tx = pool
+        .begin()
+        .await
+        .map_err(|_| "無法開始活動補償".to_owned())?;
+    local(&mut tx, id).await?;
+    for (activity, value) in &snapshot.values {
+        let result=sqlx::query("UPDATE pet_activity SET is_activity=?1 WHERE id=?2 AND pet_id=?3 AND owner_id='local-owner'")
+            .bind(value).bind(activity).bind(id).execute(&mut *tx).await.map_err(|_| "活動補償失敗".to_owned())?;
+        if result.rows_affected() != 1 {
+            return Err("活動快照已變更，無法補償".into());
+        }
+    }
+    sqlx::query("DELETE FROM local_pet_selection WHERE slot=1")
+        .execute(&mut *tx)
+        .await
+        .map_err(|_| "選取補償失敗".to_owned())?;
+    if let Some(selected) = &snapshot.selected {
+        sqlx::query("INSERT INTO local_pet_selection(slot,pet_id) VALUES(1,?1)")
+            .bind(selected)
+            .execute(&mut *tx)
+            .await
+            .map_err(|_| "選取補償失敗".to_owned())?;
+    }
+    tx.commit().await.map_err(|_| "活動補償提交失敗".to_owned())
+}
diff --git a/pet_apps/pet_persistence/src/lib.rs b/pet_apps/pet_persistence/src/lib.rs
index 7d4719b..8c6a173 100644
--- a/pet_apps/pet_persistence/src/lib.rs
+++ b/pet_apps/pet_persistence/src/lib.rs
@@ -1,67 +1,24 @@
-/// 只負責逐寵物回籠標記；Tauri 視窗／Runtime 不屬於資料層。
-pub async fn set_caged(pool: &sqlx::SqlitePool, id: &str, caged: bool) -> Result<(), String> {
-    let result = sqlx::query(
-        "UPDATE pet_profile SET is_default_closure=?1,
-         updated_at=CURRENT_TIMESTAMP
-         WHERE id=?2 AND owner_id='local-owner'",
-    )
-    .bind(caged)
-    .bind(id)
-    .execute(pool)
-    .await
-    .map_err(|_| "無法保存回籠狀態".to_owned())?;
-    if result.rows_affected() != 1 {
-        return Err("找不到可回籠的寵物".into());
-    }
-    Ok(())
-}
+pub mod activity;
+pub use activity::set_caged;
+pub use activity::{ActivitySnapshot, ActivityStatus};
 
 pub async fn read_caged(pool: &sqlx::SqlitePool, id: &str) -> Result<bool, String> {
-    let value: Option<i64> = sqlx::query_scalar(
-        "SELECT is_default_closure FROM pet_profile
-         WHERE id=?1 AND owner_id='local-owner'",
-    )
-    .bind(id)
-    .fetch_optional(pool)
-    .await
-    .map_err(|_| "無法讀取回籠狀態".to_owned())?;
-    match value {
-        Some(0) => Ok(false),
-        Some(1) => Ok(true),
-        Some(_) => Err("回籠狀態資料不合法".into()),
-        None => Err("找不到寵物".into()),
-    }
+    Ok(!activity::status(pool, id).await?.outside)
 }
-
-/// 重啟查詢 DTO；不依賴 Tauri 或 server 的 SavedPet。
 #[derive(Debug, Clone, PartialEq, Eq)]
 pub struct UncagedPet {
     pub id: String,
     pub name: String,
     pub species: String,
 }
-
-/// 只查本機 owner 的未回籠寵物；slot=1 不是出籠清單。
 pub async fn list_uncaged(pool: &sqlx::SqlitePool) -> Result<Vec<UncagedPet>, String> {
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
+    let rows:Vec<(String,String,String)>=sqlx::query_as("SELECT p.id,p.name,p.species FROM pet_profile p WHERE p.owner_id='local-owner' AND EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=p.id AND a.owner_id=p.owner_id AND a.is_activity=1) ORDER BY p.name,p.id")
+        .fetch_all(pool).await.map_err(|_| "無法讀取出籠寵物".to_owned())?;
     Ok(rows
         .into_iter()
         .map(|(id, name, species)| UncagedPet { id, name, species })
         .collect())
 }
-
 #[cfg(test)]
 mod day35_cage_adapter_tests {
     use super::set_caged;
@@ -75,11 +32,12 @@ mod day35_cage_adapter_tests {
         for id in ["pet-a", "pet-b", "pet-c"] {
             sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id) VALUES(?1,?2,'tortoise','local-owner')")
                 .bind(id).bind(id).execute(&pool).await.unwrap();
+            set_caged(&pool, id, false).await.unwrap();
         }
         set_caged(&pool, "pet-b", true).await.unwrap();
         for (id, expected) in [("pet-a", 0_i64), ("pet-b", 1), ("pet-c", 0)] {
             let flag: i64 =
-                sqlx::query_scalar("SELECT is_default_closure FROM pet_profile WHERE id=?1")
+                sqlx::query_scalar("SELECT NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=pet_profile.id AND a.owner_id=pet_profile.owner_id AND a.is_activity=1) FROM pet_profile WHERE id=?1")
                     .bind(id)
                     .fetch_one(&pool)
                     .await
@@ -89,7 +47,7 @@ mod day35_cage_adapter_tests {
         assert!(set_caged(&pool, "missing-id", true).await.is_err());
         set_caged(&pool, "pet-b", false).await.unwrap();
         let flag: i64 =
-            sqlx::query_scalar("SELECT is_default_closure FROM pet_profile WHERE id='pet-b'")
+            sqlx::query_scalar("SELECT NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id='pet-b' AND a.owner_id='local-owner' AND a.is_activity=1)")
                 .fetch_one(&pool)
                 .await
                 .unwrap();
@@ -124,6 +82,9 @@ mod day36_restore_adapter_tests {
             .await
             .unwrap();
         }
+        for id in ["pet-a", "pet-c"] {
+            super::set_caged(&pool, id, false).await.unwrap();
+        }
         let rows = list_uncaged(&pool).await.unwrap();
         assert_eq!(
             rows.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(),
diff --git a/pet_apps/pet_server/examples/day44_upgrade.rs b/pet_apps/pet_server/examples/day44_upgrade.rs
new file mode 100644
index 0000000..9d12c4d
--- /dev/null
+++ b/pet_apps/pet_server/examples/day44_upgrade.rs
@@ -0,0 +1,88 @@
+use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
+#[tokio::main]
+async fn main() -> Result<(), Box<dyn std::error::Error>> {
+    let args: Vec<String> = std::env::args().collect();
+    let pool = SqlitePool::connect_with(
+        SqliteConnectOptions::new()
+            .filename(&args[1])
+            .create_if_missing(true),
+    )
+    .await?;
+    if args.get(4).is_some_and(|s| s == "seed") {
+        sqlx::migrate::Migrator::new(std::path::Path::new(&args[2]))
+            .await?
+            .run(&pool)
+            .await?;
+        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','驗收主人')")
+            .execute(&pool)
+            .await?;
+        sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id,is_default_closure) VALUES('old-out','Old Out','tortoise','local-owner',0),('old-in','Old In','tortoise','local-owner',1),('old-sit','Old Sit','tortoise','local-owner',0)").execute(&pool).await?;
+        sqlx::query("INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity) VALUES('legacy-walk','old-sit','local-owner','散步',1),('legacy-sit','old-sit','local-owner','坐下',1),('legacy-play','old-sit','local-owner','玩耍',1)").execute(&pool).await?;
+    }
+    let profile_count: i64 = sqlx::query_scalar("SELECT count(*) FROM pet_profile")
+        .fetch_one(&pool)
+        .await?;
+    let schema: Vec<(String, String)> =
+        sqlx::query_as("SELECT name,sql FROM sqlite_master WHERE type='table' ORDER BY name")
+            .fetch_all(&pool)
+            .await?;
+    let before: Vec<(String, Option<i64>)> =
+        sqlx::query_as("SELECT id,is_default_closure FROM pet_profile ORDER BY id")
+            .fetch_all(&pool)
+            .await?;
+    let m = sqlx::migrate::Migrator::new(std::path::Path::new(&args[3])).await?;
+    if args.get(4).is_some_and(|s| s == "tamper") {
+        sqlx::query("UPDATE _sqlx_migrations SET checksum=zeroblob(length(checksum)) WHERE version=(SELECT min(version) FROM _sqlx_migrations)").execute(&pool).await?;
+        assert!(m.run(&pool).await.is_err());
+        assert_eq!(
+            before,
+            sqlx::query_as::<_, (String, Option<i64>)>(
+                "SELECT id,is_default_closure FROM pet_profile ORDER BY id"
+            )
+            .fetch_all(&pool)
+            .await?
+        );
+        println!("checksum drift rejected and profile preferences preserved");
+        return Ok(());
+    }
+    m.run(&pool).await?;
+    m.run(&pool).await?;
+    assert_eq!(
+        schema,
+        sqlx::query_as::<_, (String, String)>(
+            "SELECT name,sql FROM sqlite_master WHERE type='table' ORDER BY name"
+        )
+        .fetch_all(&pool)
+        .await?
+    );
+    assert_eq!(
+        before,
+        sqlx::query_as::<_, (String, Option<i64>)>(
+            "SELECT id,is_default_closure FROM pet_profile ORDER BY id"
+        )
+        .fetch_all(&pool)
+        .await?
+    );
+    let missing:i64=sqlx::query_scalar("SELECT count(*) FROM pet_profile p WHERE NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=p.id AND a.owner_id=p.owner_id AND a.activity_type='散步') OR NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=p.id AND a.owner_id=p.owner_id AND a.activity_type='坐下')").fetch_one(&pool).await?;
+    assert_eq!(missing, 0);
+    let conflicts:i64=sqlx::query_scalar("SELECT count(*) FROM pet_activity w WHERE w.activity_type='散步' AND w.is_activity=1 AND EXISTS(SELECT 1 FROM pet_activity s WHERE s.pet_id=w.pet_id AND s.owner_id=w.owner_id AND s.activity_type='坐下' AND s.is_activity=1)").fetch_one(&pool).await?;
+    assert_eq!(conflicts, 0);
+    if args.get(4).is_some_and(|s| s == "seed") {
+        let out = pet_persistence::activity::status(&pool, "old-out").await?;
+        assert!(out.outside && out.roaming);
+        let caged = pet_persistence::activity::status(&pool, "old-in").await?;
+        assert!(!caged.outside);
+        let sit = pet_persistence::activity::status(&pool, "old-sit").await?;
+        assert!(sit.outside && !sit.roaming);
+        let play: i64 =
+            sqlx::query_scalar("SELECT is_activity FROM pet_activity WHERE id='legacy-play'")
+                .fetch_one(&pool)
+                .await?;
+        assert_eq!(play, 1);
+    }
+    println!(
+        "upgrade/idempotence passed; profiles={profile_count}; profile defaults unchanged; schema/columns unchanged; two basic activities present; sit priority preserved"
+    );
+    pool.close().await;
+    Ok(())
+}
diff --git a/pet_apps/pet_server/src/lib.rs b/pet_apps/pet_server/src/lib.rs
index d710661..438b79a 100644
--- a/pet_apps/pet_server/src/lib.rs
+++ b/pet_apps/pet_server/src/lib.rs
@@ -146,8 +146,14 @@ async fn select_pet(
     store: State<'_, pet_store::PetStore>,
 ) -> Result<pet_store::SavedPet, String> {
     let _operation = pet_windows::lifecycle_guard().await;
-    let saved = store.select(&id).await?;
-    pet_windows::open(&app, &saved)?;
+    let saved = store.profile(&id).await?;
+    let snapshot = store.snapshot(&id).await?;
+    pet_application::lifecycle::persist_then_apply(
+        || async { store.select(&id).await.map(|_| ()) },
+        || pet_windows::open(&app, &saved),
+        || store.restore(&id, &snapshot),
+    )
+    .await?;
     app.emit("pet-profile-changed", &saved)
         .map_err(|_| "選取已儲存，通知失敗，請重新載入生活窗".to_owned())?;
     Ok(saved)
@@ -181,12 +187,8 @@ pub fn application(db: sqlx::SqlitePool) {
                             .inner()
                             .clone();
                         tauri::async_runtime::spawn(async move {
-                            let _operation = pet_windows::lifecycle_guard().await;
-                            if let Err(error) = pet_application::lifecycle::persist_then_close(
-                                |flag| store.set_caged(&id, flag),
-                                || window.destroy().map_err(|e| e.to_string()),
-                            )
-                            .await
+                            if let Err(error) =
+                                pet_windows::cage_with_store(window.app_handle(), &store, &id).await
                             {
                                 // DB failure must leave the window alive; never silently claim cage.
                                 eprintln!("生活窗直接關閉／回籠失敗：{error}");
@@ -212,12 +214,22 @@ pub fn application(db: sqlx::SqlitePool) {
             let store = app.state::<pet_store::PetStore>().inner().clone();
             tauri::async_runtime::spawn(async move {
                 let _operation = pet_windows::lifecycle_guard().await;
-                match store.uncaged().await {
+                match store.prepare_startup().await {
                     Ok(pets) => {
                         for pet in pets {
                             // 每個 UUID 由既有 open() 建立獨立原生視窗與 Runtime。
                             // 單隻失敗不應讓其他未回籠寵物無法恢復。
-                            if let Err(error) = pet_windows::open(&handle, &pet) {
+                            let result = async {
+                                let snapshot = store.snapshot(&pet.id).await?;
+                                pet_application::lifecycle::persist_then_apply(
+                                    || store.set_caged(&pet.id, false),
+                                    || pet_windows::open(&handle, &pet),
+                                    || store.restore(&pet.id, &snapshot),
+                                )
+                                .await
+                            }
+                            .await;
+                            if let Err(error) = result {
                                 eprintln!("重啟恢復寵物 {} 失敗：{error}", pet.id);
                             }
                         }
@@ -266,6 +278,30 @@ pub fn application(db: sqlx::SqlitePool) {
 
                 if let Some(scoped) = event.id().0.strip_prefix("pet:") {
                     if let Some((id, action)) = scoped.split_once(':') {
+                        if matches!(action, "walk" | "sit" | "cage") {
+                            let handle = app_handle.clone();
+                            let id = id.to_owned();
+                            let action = action.to_owned();
+                            let store = handle.state::<pet_store::PetStore>().inner().clone();
+                            tauri::async_runtime::spawn(async move {
+                                let result = if action == "cage" {
+                                    pet_windows::cage_with_store(&handle, &store, &id).await
+                                } else {
+                                    pet_windows::switch_with_store(
+                                        &handle,
+                                        &store,
+                                        &id,
+                                        action == "walk",
+                                    )
+                                    .await
+                                    .map(|_| ())
+                                };
+                                if let Err(error) = result {
+                                    eprintln!("寵物活動操作失敗：{error}");
+                                }
+                            });
+                            return;
+                        }
                         let command = match action {
                             "sleep" => Some(PetCommand::Sleep),
                             "wake" => Some(PetCommand::Wake),
@@ -312,6 +348,9 @@ pub fn application(db: sqlx::SqlitePool) {
             pet_cage_status_by_id,
             active_pet,
             select_pet,
+            pet_windows::pet_activity_status,
+            pet_windows::set_pet_walk,
+            pet_windows::set_pet_default_caged,
             pet_windows::saved_pet_profile,
             pet_windows::open_saved_pet,
             pet_windows::get_saved_pet_state,
diff --git a/pet_apps/pet_server/src/pet_store.rs b/pet_apps/pet_server/src/pet_store.rs
index b6cc67f..3b2058c 100644
--- a/pet_apps/pet_server/src/pet_store.rs
+++ b/pet_apps/pet_server/src/pet_store.rs
@@ -38,7 +38,7 @@ impl PetStore {
                     ON p.id=s.pet_id 
                 WHERE s.slot=1 
                     AND p.owner_id='local-owner'
-                    AND COALESCE(p.is_default_closure,0)=0"#,
+                    AND EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=p.id AND a.owner_id=p.owner_id AND a.is_activity=1)"#,
         )
         .fetch_optional(&self.pool)
         .await
@@ -78,15 +78,7 @@ impl PetStore {
         .await
         .map_err(|_| "無法儲存選取".to_owned())?;
 
-        sqlx::query(
-            "UPDATE pet_profile SET is_default_closure=FALSE,
-     updated_at=CURRENT_TIMESTAMP
-     WHERE id=?1 AND owner_id='local-owner'",
-        )
-        .bind(id)
-        .execute(&mut *tx)
-        .await
-        .map_err(|_| "無法恢復出籠狀態".to_owned())?;
+        pet_persistence::activity::set_caged_in(&mut tx, id, false).await?;
 
         tx.commit().await.map_err(|_| "選取提交失敗".to_owned())?;
         Ok(SavedPet {
@@ -125,9 +117,10 @@ impl PetStore {
         r#"INSERT INTO pet_owner(id,name) VALUES('local-owner','本機使用者') ON CONFLICT(id) DO NOTHING"#
     ).execute(&mut *tx).await.map_err(|_| "無法儲存主人")?;
         sqlx::query(
-        r#"INSERT INTO pet_profile(id,name,species,owner_id) VALUES(?1,?2,'tortoise','local-owner')"#
+        r#"INSERT INTO pet_profile(id,name,species,owner_id,is_default_closure) VALUES(?1,?2,'tortoise','local-owner',1)"#
     ).bind(&id).bind(&name)
       .execute(&mut *tx).await.map_err(|_| "無法儲存寵物")?;
+        pet_persistence::activity::ensure_pair(&mut tx, &id).await?;
         tx.commit().await.map_err(|_| "寵物提交失敗")?;
         let (saved_id, saved_name, species): (String, String, String) = sqlx::query_as(
             r#"SELECT id
@@ -167,6 +160,49 @@ impl PetStore {
     pub async fn read_caged(&self, id: &str) -> Result<bool, String> {
         pet_persistence::read_caged(&self.pool, id).await
     }
+    pub async fn activity_status(
+        &self,
+        id: &str,
+    ) -> Result<pet_persistence::ActivityStatus, String> {
+        pet_persistence::activity::status(&self.pool, id).await
+    }
+    pub async fn snapshot(&self, id: &str) -> Result<pet_persistence::ActivitySnapshot, String> {
+        pet_persistence::activity::snapshot(&self.pool, id).await
+    }
+    pub async fn restore(
+        &self,
+        id: &str,
+        s: &pet_persistence::ActivitySnapshot,
+    ) -> Result<(), String> {
+        pet_persistence::activity::restore(&self.pool, id, s).await
+    }
+    pub async fn switch_activity(
+        &self,
+        id: &str,
+        walking: bool,
+    ) -> Result<pet_persistence::ActivityStatus, String> {
+        pet_persistence::activity::switch(&self.pool, id, walking).await
+    }
+    pub async fn set_default_caged(
+        &self,
+        id: &str,
+        caged: bool,
+    ) -> Result<pet_persistence::ActivityStatus, String> {
+        pet_persistence::activity::set_default_caged(&self.pool, id, caged).await
+    }
+    pub async fn prepare_startup(&self) -> Result<Vec<SavedPet>, String> {
+        pet_persistence::activity::prepare_startup(&self.pool)
+            .await
+            .map(|rows| {
+                rows.into_iter()
+                    .map(|(id, name, species)| SavedPet { id, name, species })
+                    .collect()
+            })
+    }
+    pub async fn default_pets(&self) -> Result<Vec<SavedPet>, String> {
+        sqlx::query_as::<_,(String,String,String)>("SELECT id,name,species FROM pet_profile WHERE owner_id='local-owner' AND COALESCE(is_default_closure,0)=0 ORDER BY name,id")
+            .fetch_all(&self.pool).await.map(|r|r.into_iter().map(|(id,name,species)|SavedPet{id,name,species}).collect()).map_err(|_|"無法讀取啟動預設寵物".into())
+    }
 }
 
 #[cfg(test)]
@@ -289,15 +325,16 @@ mod day35_cage_caller_tests {
         let store = PetStore { pool: pool.clone() };
         let first = store.create("小橘").await.unwrap();
         let second = store.create("阿龜").await.unwrap();
+        store.set_caged(&first.id, false).await.unwrap();
         store.set_caged(&second.id, true).await.unwrap();
         let first_flag: i64 =
-            sqlx::query_scalar("SELECT is_default_closure FROM pet_profile WHERE id=?1")
+            sqlx::query_scalar("SELECT NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=pet_profile.id AND a.owner_id=pet_profile.owner_id AND a.is_activity=1) FROM pet_profile WHERE id=?1")
                 .bind(&first.id)
                 .fetch_one(&pool)
                 .await
                 .unwrap();
         let second_flag: i64 =
-            sqlx::query_scalar("SELECT is_default_closure FROM pet_profile WHERE id=?1")
+            sqlx::query_scalar("SELECT NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=pet_profile.id AND a.owner_id=pet_profile.owner_id AND a.is_activity=1) FROM pet_profile WHERE id=?1")
                 .bind(&second.id)
                 .fetch_one(&pool)
                 .await
diff --git a/pet_apps/pet_server/src/pet_windows.rs b/pet_apps/pet_server/src/pet_windows.rs
index 01c3d44..d3d2ab1 100644
--- a/pet_apps/pet_server/src/pet_windows.rs
+++ b/pet_apps/pet_server/src/pet_windows.rs
@@ -1,9 +1,11 @@
 use crate::pet_store::{PetStore, SavedPet};
 use crate::window_position::centered_position;
-use pet_application::lifecycle::persist_then_open;
+use pet_application::lifecycle::persist_then_apply;
 use pet_domain::{PetCommand, PetMachine, PetSnapshot};
 use std::{collections::HashMap, sync::Mutex};
-use tauri::{Emitter, Manager, PhysicalPosition, PhysicalRect, PhysicalSize, State};
+use tauri::{Emitter, Manager, State};
+#[cfg(test)]
+use tauri::{PhysicalPosition, PhysicalRect, PhysicalSize};
 
 // 產品目前單程序；全域序列化所有出／入籠、選取與啟動恢復。
 // 不持有 std::sync::MutexGuard 跨 await；Tauri command future 仍可 Send。
@@ -134,11 +136,11 @@ pub async fn open_saved_pet(
 ) -> Result<(), String> {
     let _operation = lifecycle_guard().await;
     let pet = store.profile(&id).await?;
-    let old_caged = store.read_caged(&id).await?;
-    persist_then_open(
-        old_caged,
-        |flag| store.set_caged(&id, flag),
+    let snapshot = store.snapshot(&id).await?;
+    persist_then_apply(
+        || store.set_caged(&id, false),
         || open(&app, &pet),
+        || store.restore(&id, &snapshot),
     )
     .await
 }
@@ -148,22 +150,80 @@ pub async fn cage_saved_pet(
     id: String,
     app: tauri::AppHandle,
     store: State<'_, PetStore>,
+) -> Result<(), String> {
+    cage_with_store(&app, &store, &id).await
+}
+
+pub async fn cage_with_store(
+    app: &tauri::AppHandle,
+    store: &PetStore,
+    id: &str,
 ) -> Result<(), String> {
     let _operation = lifecycle_guard().await;
-    store.profile(&id).await?;
-    let window = app.get_webview_window(&label(&id)?);
-    pet_application::lifecycle::persist_then_close(
-        |flag| store.set_caged(&id, flag),
+    store.profile(id).await?;
+    let snapshot = store.snapshot(id).await?;
+    let window = app.get_webview_window(&label(id)?);
+    persist_then_apply(
+        || store.set_caged(id, true),
         || match window {
-            Some(window) => window.destroy().map_err(|e| e.to_string()),
+            Some(w) => w.destroy().map_err(|e| e.to_string()),
             None => Ok(()),
         },
+        || store.restore(id, &snapshot),
     )
     .await?;
-    app.state::<Runtime>().remove(&id);
+    app.state::<Runtime>().remove(id);
     Ok(())
 }
 
+pub async fn switch_with_store(
+    app: &tauri::AppHandle,
+    store: &PetStore,
+    id: &str,
+    walking: bool,
+) -> Result<pet_persistence::ActivityStatus, String> {
+    let _operation = lifecycle_guard().await;
+    let pet = store.profile(id).await?;
+    let snapshot = store.snapshot(id).await?;
+    persist_then_apply(
+        || async { store.switch_activity(id, walking).await.map(|_| ()) },
+        || open(app, &pet),
+        || store.restore(id, &snapshot),
+    )
+    .await?;
+    crate::roam::spawn_for_pet(app.clone(), label(id)?);
+    store.activity_status(id).await
+}
+
+#[tauri::command]
+pub async fn set_pet_walk(
+    id: String,
+    walking: bool,
+    app: tauri::AppHandle,
+    store: State<'_, PetStore>,
+) -> Result<pet_persistence::ActivityStatus, String> {
+    switch_with_store(&app, &store, &id, walking).await
+}
+
+#[tauri::command]
+pub async fn pet_activity_status(
+    id: String,
+    store: State<'_, PetStore>,
+) -> Result<pet_persistence::ActivityStatus, String> {
+    let _operation = lifecycle_guard().await;
+    store.activity_status(&id).await
+}
+
+#[tauri::command]
+pub async fn set_pet_default_caged(
+    id: String,
+    caged: bool,
+    store: State<'_, PetStore>,
+) -> Result<pet_persistence::ActivityStatus, String> {
+    let _operation = lifecycle_guard().await;
+    store.set_default_caged(&id, caged).await
+}
+
 #[tauri::command]
 pub async fn find_saved_pet(
     id: String,
diff --git a/pet_apps/pet_server/src/roam.rs b/pet_apps/pet_server/src/roam.rs
index c4ae287..7ba9efa 100644
--- a/pet_apps/pet_server/src/roam.rs
+++ b/pet_apps/pet_server/src/roam.rs
@@ -135,6 +135,10 @@ async fn roam_desktop(app: tauri::AppHandle, label: &str, active: &AtomicBool) {
             break;
         }
 
+        let lifecycle = crate::pet_windows::lifecycle_guard().await;
+        if !active.load(Ordering::Acquire) {
+            break;
+        }
         let window = match app.get_webview_window(label) {
             Some(window) => window,
             None => {
@@ -143,6 +147,26 @@ async fn roam_desktop(app: tauri::AppHandle, label: &str, active: &AtomicBool) {
             }
         };
 
+        let Some(id) = crate::pet_windows::pet_id_from_window_label(label) else {
+            break;
+        };
+        let store = app.state::<crate::pet_store::PetStore>().inner().clone();
+        let status = match store.activity_status(&id).await {
+            Ok(status) => status,
+            Err(error) => {
+                eprintln!("漫遊活動讀取失敗，停止位移：{error}");
+                break;
+            }
+        };
+        let sleeping = app
+            .state::<crate::pet_windows::Runtime>()
+            .snapshot(&id)
+            .is_ok_and(|s| s.state == pet_domain::PetState::Sleeping);
+        if !status.roaming || sleeping {
+            drop(lifecycle);
+            tokio::time::sleep(ROAM_INTERVAL).await;
+            continue;
+        }
         let before = match window.outer_position() {
             Ok(position) => position,
             Err(error) => {
@@ -213,6 +237,7 @@ async fn roam_desktop(app: tauri::AppHandle, label: &str, active: &AtomicBool) {
             stalled_attempts = 0;
         }
 
+        drop(lifecycle);
         tokio::time::sleep(ROAM_INTERVAL).await;
     }
 }
diff --git a/pet_apps/pet_server/tests/day44_activity_lifecycle.rs b/pet_apps/pet_server/tests/day44_activity_lifecycle.rs
new file mode 100644
index 0000000..83cb6ec
--- /dev/null
+++ b/pet_apps/pet_server/tests/day44_activity_lifecycle.rs
@@ -0,0 +1,208 @@
+use pet_persistence::activity::{self, SIT, WALK};
+use pet_server::pet_store::PetStore;
+
+async fn flags(pool: &sqlx::SqlitePool, id: &str) -> Vec<(String, Option<i64>)> {
+    sqlx::query_as("SELECT activity_type,is_activity FROM pet_activity WHERE pet_id=?1 AND owner_id='local-owner' ORDER BY activity_type,id")
+        .bind(id).fetch_all(pool).await.unwrap()
+}
+
+#[sqlx::test(migrations = "../migrations")]
+async fn creation_has_two_inactive_activities_and_no_new_columns(pool: sqlx::SqlitePool) {
+    let store = PetStore { pool: pool.clone() };
+    let p = store.create("A").await.unwrap();
+    assert_eq!(
+        flags(&pool, &p.id).await,
+        vec![(SIT.into(), Some(0)), (WALK.into(), Some(0))]
+    );
+    assert!(store.read_caged(&p.id).await.unwrap());
+    assert!(store.activity_status(&p.id).await.unwrap().default_caged);
+    assert_eq!(
+        sqlx::query("PRAGMA table_info(pet_profile)")
+            .fetch_all(&pool)
+            .await
+            .unwrap()
+            .len(),
+        11
+    );
+    assert_eq!(
+        sqlx::query("PRAGMA table_info(pet_activity)")
+            .fetch_all(&pool)
+            .await
+            .unwrap()
+            .len(),
+        8
+    );
+}
+
+#[sqlx::test(migrations = "../migrations")]
+async fn failure_creating_second_activity_rolls_back_owner_profile_and_first_activity(
+    pool: sqlx::SqlitePool,
+) {
+    sqlx::query("CREATE TRIGGER reject_sit BEFORE INSERT ON pet_activity WHEN NEW.activity_type='坐下' BEGIN SELECT RAISE(ABORT,'injected failure'); END")
+        .execute(&pool).await.unwrap();
+    let store = PetStore { pool: pool.clone() };
+    assert!(store.create("A").await.is_err());
+    let counts:(i64,i64,i64)=sqlx::query_as("SELECT (SELECT count(*) FROM pet_owner),(SELECT count(*) FROM pet_profile),(SELECT count(*) FROM pet_activity)").fetch_one(&pool).await.unwrap();
+    assert_eq!(counts, (0, 0, 0));
+}
+
+#[sqlx::test(migrations = "../migrations")]
+async fn any_active_type_means_outside_and_sit_wins_and_persists(pool: sqlx::SqlitePool) {
+    let store = PetStore { pool: pool.clone() };
+    let p = store.create("A").await.unwrap();
+    sqlx::query("INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity) VALUES('other',?1,'local-owner','玩耍',1)").bind(&p.id).execute(&pool).await.unwrap();
+    let s = store.activity_status(&p.id).await.unwrap();
+    assert!(s.outside);
+    assert!(!s.roaming);
+    assert!(!store.read_caged(&p.id).await.unwrap());
+    store.switch_activity(&p.id, true).await.unwrap();
+    assert!(store.activity_status(&p.id).await.unwrap().roaming);
+    sqlx::query("UPDATE pet_activity SET is_activity=1 WHERE pet_id=?1 AND activity_type='坐下'")
+        .bind(&p.id)
+        .execute(&pool)
+        .await
+        .unwrap();
+    let s = store.activity_status(&p.id).await.unwrap();
+    assert!(s.outside);
+    assert!(!s.roaming);
+    let walking:i64=sqlx::query_scalar("SELECT count(*) FROM pet_activity WHERE pet_id=?1 AND activity_type='散步' AND is_activity=1").bind(&p.id).fetch_one(&pool).await.unwrap();
+    assert_eq!(walking, 0);
+    assert!(store.activity_status(&p.id).await.unwrap().default_caged);
+}
+
+#[sqlx::test(migrations = "../migrations")]
+async fn repeats_do_not_duplicate_and_cage_stops_only_target_without_changing_defaults(
+    pool: sqlx::SqlitePool,
+) {
+    let store = PetStore { pool: pool.clone() };
+    let a = store.create("A").await.unwrap();
+    let b = store.create("B").await.unwrap();
+    store.set_default_caged(&a.id, false).await.unwrap();
+    store.switch_activity(&a.id, false).await.unwrap();
+    store.switch_activity(&b.id, true).await.unwrap();
+    for _ in 0..3 {
+        store.select(&a.id).await.unwrap();
+        store.set_caged(&a.id, false).await.unwrap();
+    }
+    assert_eq!(flags(&pool, &a.id).await.len(), 2);
+    assert!(!store.activity_status(&a.id).await.unwrap().roaming);
+    sqlx::query("INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity) VALUES('play',?1,'local-owner','玩耍',1)").bind(&a.id).execute(&pool).await.unwrap();
+    store.set_caged(&a.id, true).await.unwrap();
+    assert!(store.read_caged(&a.id).await.unwrap());
+    assert!(!store.read_caged(&b.id).await.unwrap());
+    assert!(!store.activity_status(&a.id).await.unwrap().default_caged);
+    assert_eq!(store.default_pets().await.unwrap(), vec![a]);
+}
+
+#[sqlx::test(migrations = "../migrations")]
+async fn missing_and_foreign_pet_fail_closed_and_owner_mismatch_does_not_leak(
+    pool: sqlx::SqlitePool,
+) {
+    let store = PetStore { pool: pool.clone() };
+    let a = store.create("A").await.unwrap();
+    sqlx::query("INSERT INTO pet_owner(id,name) VALUES('other-owner','Other')")
+        .execute(&pool)
+        .await
+        .unwrap();
+    sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id) VALUES('foreign','X','tortoise','other-owner')").execute(&pool).await.unwrap();
+    sqlx::query("INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity) VALUES('mismatch',?1,'other-owner','散步',1)").bind(&a.id).execute(&pool).await.unwrap();
+    assert!(store.read_caged(&a.id).await.unwrap());
+    for id in ["foreign", "missing"] {
+        assert!(store.read_caged(id).await.is_err());
+        assert!(store.switch_activity(id, true).await.is_err());
+        assert!(store.set_default_caged(id, false).await.is_err());
+    }
+    assert!(store.uncaged().await.unwrap().is_empty());
+}
+
+#[sqlx::test(migrations = "../migrations")]
+async fn native_failure_restores_exact_sitting_and_other_activity_snapshot(pool: sqlx::SqlitePool) {
+    let store = PetStore { pool: pool.clone() };
+    let a = store.create("A").await.unwrap();
+    store.select(&a.id).await.unwrap();
+    store.switch_activity(&a.id, false).await.unwrap();
+    sqlx::query("INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity) VALUES('play',?1,'local-owner','玩耍',1)").bind(&a.id).execute(&pool).await.unwrap();
+    let before = flags(&pool, &a.id).await;
+    let snapshot = store.snapshot(&a.id).await.unwrap();
+    let result = pet_application::lifecycle::persist_then_apply(
+        || store.set_caged(&a.id, true),
+        || Err("native close failure".into()),
+        || store.restore(&a.id, &snapshot),
+    )
+    .await;
+    assert!(result.unwrap_err().contains("native close failure"));
+    assert_eq!(flags(&pool, &a.id).await, before);
+    let s = store.activity_status(&a.id).await.unwrap();
+    assert!(s.outside);
+    assert!(!s.roaming);
+    assert!(s.default_caged);
+}
+
+#[sqlx::test(migrations = "../migrations")]
+async fn read_and_normalization_failure_never_reports_success(pool: sqlx::SqlitePool) {
+    let store = PetStore { pool: pool.clone() };
+    let a = store.create("A").await.unwrap();
+    sqlx::query("UPDATE pet_activity SET is_activity=1 WHERE pet_id=?1")
+        .bind(&a.id)
+        .execute(&pool)
+        .await
+        .unwrap();
+    sqlx::query("CREATE TRIGGER reject_normalization BEFORE UPDATE ON pet_activity BEGIN SELECT RAISE(ABORT,'db failure'); END").execute(&pool).await.unwrap();
+    assert!(store.activity_status(&a.id).await.is_err());
+    pool.close().await;
+    assert!(activity::status(&pool, &a.id).await.is_err());
+}
+
+#[sqlx::test(migrations = "../migrations")]
+async fn startup_uses_existing_defaults_and_cages_non_default_activities(pool: sqlx::SqlitePool) {
+    let store = PetStore { pool: pool.clone() };
+    let a = store.create("A").await.unwrap();
+    let b = store.create("B").await.unwrap();
+    store.set_default_caged(&a.id, false).await.unwrap();
+    store.switch_activity(&a.id, false).await.unwrap();
+    store.switch_activity(&b.id, true).await.unwrap();
+    // Caging A does not clear its startup preference; B is active but not a default pet.
+    store.set_caged(&a.id, true).await.unwrap();
+    let restarted = PetStore { pool: pool.clone() };
+    let defaults = restarted.prepare_startup().await.unwrap();
+    assert_eq!(defaults, vec![a.clone()]);
+    assert!(restarted.read_caged(&b.id).await.unwrap());
+    for pet in defaults {
+        restarted.set_caged(&pet.id, false).await.unwrap();
+    }
+    assert_eq!(restarted.uncaged().await.unwrap(), vec![a]);
+}
+
+#[sqlx::test(migrations = "../migrations")]
+async fn default_change_and_normalization_are_one_transaction(pool: sqlx::SqlitePool) {
+    let store = PetStore { pool: pool.clone() };
+    let a = store.create("A").await.unwrap();
+    sqlx::query("UPDATE pet_activity SET is_activity=1 WHERE pet_id=?1")
+        .bind(&a.id)
+        .execute(&pool)
+        .await
+        .unwrap();
+    sqlx::query("CREATE TRIGGER reject_walk_update BEFORE UPDATE ON pet_activity WHEN OLD.activity_type='散步' BEGIN SELECT RAISE(ABORT,'normalization fails'); END").execute(&pool).await.unwrap();
+    assert!(store.set_default_caged(&a.id, false).await.is_err());
+    let default: i64 = sqlx::query_scalar("SELECT is_default_closure FROM pet_profile WHERE id=?1")
+        .bind(&a.id)
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(default, 1);
+}
+
+#[sqlx::test(migrations = "../migrations")]
+async fn failed_mode_validation_rolls_back_both_activity_changes(pool: sqlx::SqlitePool) {
+    let store = PetStore { pool: pool.clone() };
+    let a = store.create("A").await.unwrap();
+    store.switch_activity(&a.id, false).await.unwrap();
+    let before = flags(&pool, &a.id).await;
+    sqlx::query("UPDATE pet_profile SET is_default_closure=2 WHERE id=?1")
+        .bind(&a.id)
+        .execute(&pool)
+        .await
+        .unwrap();
+    assert!(store.switch_activity(&a.id, true).await.is_err());
+    assert_eq!(flags(&pool, &a.id).await, before);
+}
diff --git a/pet_apps/pet_ui/index.html b/pet_apps/pet_ui/index.html
index f6bdc48..8288c6c 100644
--- a/pet_apps/pet_ui/index.html
+++ b/pet_apps/pet_ui/index.html
@@ -40,6 +40,10 @@
     window.cageSavedPetById = (id) => invoke("cage_saved_pet", { id });
 
     window.petCageStatusById = (id) => invoke("pet_cage_status_by_id", { id });
+    window.petActivityStatus = (id) => invoke("pet_activity_status", { id });
+    window.setPetWalk = (id, walking) => invoke("set_pet_walk", { id, walking });
+    window.setPetDefaultCaged = (id, caged) => invoke("set_pet_default_caged", { id, caged });
+
 
     window.findSavedPet = (id) => invoke("find_saved_pet", { id });
 
@@ -75,6 +79,9 @@
         items: [
           { id: `pet:${id}:open_control`, text: "開啟控制窗" },
           { id: `pet:${id}:interact`, text: "互動" },
+          { id: `pet:${id}:walk`, text: "散步（允許漫遊）" },
+          { id: `pet:${id}:sit`, text: "坐下（停止漫遊）" },
+          { id: `pet:${id}:cage`, text: "回籠子" },
           { id: `pet:${id}:sleep`, text: "睡覺" },
           { id: `pet:${id}:wake`, text: "醒來" },
         ],
diff --git a/pet_apps/pet_ui/src/bridge.rs b/pet_apps/pet_ui/src/bridge.rs
index 5afd8c9..da6dbf7 100644
--- a/pet_apps/pet_ui/src/bridge.rs
+++ b/pet_apps/pet_ui/src/bridge.rs
@@ -83,6 +83,12 @@ extern "C" {
 
     #[wasm_bindgen(js_namespace=window, js_name=petCageStatusById)]
     fn pet_cage_status_by_id_js(id: &str) -> Promise;
+    #[wasm_bindgen(js_namespace=window,js_name=petActivityStatus)]
+    fn pet_activity_status_js(id: &str) -> Promise;
+    #[wasm_bindgen(js_namespace=window,js_name=setPetWalk)]
+    fn set_pet_walk_js(id: &str, walking: bool) -> Promise;
+    #[wasm_bindgen(js_namespace=window,js_name=setPetDefaultCaged)]
+    fn set_pet_default_caged_js(id: &str, caged: bool) -> Promise;
 
     #[wasm_bindgen(js_namespace=window, js_name=findSavedPet)]
     fn find_saved_pet_js(id: &str) -> Promise;
@@ -270,3 +276,21 @@ pub async fn find_saved_pet(id: &str) -> Result<(), String> {
         .map_err(js_error)?;
     Ok(())
 }
+
+#[derive(serde::Deserialize)]
+pub struct ActivityStatus {
+    pub outside: bool,
+    pub roaming: bool,
+    pub default_caged: bool,
+}
+pub async fn activity_action(id: &str, choice: i8) -> Result<ActivityStatus, String> {
+    let promise = match choice {
+        1 => set_pet_walk_js(id, true),
+        2 => set_pet_walk_js(id, false),
+        3 => set_pet_default_caged_js(id, false),
+        4 => set_pet_default_caged_js(id, true),
+        _ => pet_activity_status_js(id),
+    };
+    let value = JsFuture::from(promise).await.map_err(js_error)?;
+    serde_wasm_bindgen::from_value(value).map_err(|_| "活動資料格式錯誤".into())
+}
diff --git a/pet_apps/pet_ui/src/main.rs b/pet_apps/pet_ui/src/main.rs
index df3cb78..de2a417 100644
--- a/pet_apps/pet_ui/src/main.rs
+++ b/pet_apps/pet_ui/src/main.rs
@@ -242,6 +242,7 @@ fn App() -> impl IntoView {
                             let find_id=id.clone();
                             let status_id=id.clone();
                             let display_id=id.clone();
+                            let activity_id=id.clone();
                             view! { <li>{format!("{}／{}（{}）", pet.name, pet.species, pet.id)}
                                 <button type="button" disabled=move || pets_busy.get()
                                     on:click=move |_| {
@@ -324,6 +325,7 @@ fn App() -> impl IntoView {
                                                 set_pets_busy.set(false);
                                             });
                                         }>"找回寵物"</button>
+                                    <ActivityControls id=activity_id busy=pets_busy set_busy=set_pets_busy statuses=cage_status />
                             </li> }
                         } />
                 </ul>
@@ -490,3 +492,51 @@ mod day16_scan_generation_tests {
         assert!(!accept_scan_result(3, 4));
     }
 }
+
+#[component]
+fn ActivityControls(
+    id: String,
+    busy: ReadSignal<bool>,
+    set_busy: WriteSignal<bool>,
+    statuses: RwSignal<HashMap<String, String>>,
+) -> impl IntoView {
+    let id = StoredValue::new(id);
+    let action = move |choice: i8| {
+        if busy.get_untracked() {
+            return;
+        }
+        set_busy.set(true);
+        let id = id.get_value();
+        spawn_local(async move {
+            let text = match bridge::activity_action(&id, choice).await {
+                Ok(s) => format!(
+                    "{}；{}；啟動預設{}",
+                    if s.outside { "已出籠" } else { "已回籠" },
+                    if s.roaming {
+                        "散步，可漫遊"
+                    } else {
+                        "不漫遊"
+                    },
+                    if s.default_caged {
+                        "留在籠內"
+                    } else {
+                        "出籠"
+                    }
+                ),
+                Err(error) => format!("活動操作失敗：{error}"),
+            };
+            statuses.update(|values| {
+                values.insert(id, text);
+            });
+            set_busy.set(false);
+        });
+    };
+    view! {<div>
+        <button type="button" disabled=move||busy.get() on:click=move |_|action(0)>"查詢活動與預設"</button>
+        <button type="button" disabled=move||busy.get() on:click=move |_|action(1)>"散步（可漫遊）"</button>
+        <button type="button" disabled=move||busy.get() on:click=move |_|action(2)>"坐下（停止漫遊）"</button>
+        <button type="button" disabled=move||busy.get() on:click=move |_|action(3)>"啟動時預設出籠"</button>
+        <button type="button" disabled=move||busy.get() on:click=move |_|action(4)>"啟動時預設留籠"</button>
+
+    </div>}
+}
diff --git a/pet_apps/pet_ui/tests/day44-activity.test.mjs b/pet_apps/pet_ui/tests/day44-activity.test.mjs
new file mode 100644
index 0000000..22595bf
--- /dev/null
+++ b/pet_apps/pet_ui/tests/day44-activity.test.mjs
@@ -0,0 +1,17 @@
+import assert from "node:assert/strict";
+import {readFileSync} from "node:fs";
+import test from "node:test";
+import vm from "node:vm";
+const script=readFileSync(new URL("../index.html",import.meta.url),"utf8").match(/<script>([\s\S]*?)<\/script>/)[1];
+function boot(){
+  const calls=[];
+  const window={__TAURI__:{core:{invoke:async(name,args)=>{calls.push([name,structuredClone(args)]);if(args.id==="broken")throw Error("SQLite unavailable");return {outside:true,roaming:!!args.walking,default_caged:true};}},event:{listen:async()=>()=>{}}}};
+  vm.runInNewContext(script,{window,location:{search:""},URLSearchParams});return{window,calls};
+}
+test("walk/sit and defaults reach scoped native commands with booleans",async()=>{
+ const{window,calls}=boot();await window.setPetWalk("A",true);await window.setPetWalk("A",false);await window.setPetDefaultCaged("B",false);await window.petActivityStatus("B");
+ assert.deepEqual(calls,[["set_pet_walk",{id:"A",walking:true}],["set_pet_walk",{id:"A",walking:false}],["set_pet_default_caged",{id:"B",caged:false}],["pet_activity_status",{id:"B"}]]);
+});
+test("failed activity and default writes remain rejected promises",async()=>{
+ const{window}=boot();await assert.rejects(window.setPetWalk("broken",true),/SQLite unavailable/);await assert.rejects(window.setPetDefaultCaged("broken",false),/SQLite unavailable/);
+});
```

### 舊斷言承接與保資料升級

原持久化／Store讀寫舊旗標的狀態斷言改接真活動fixture；保留A/B隔離、選取、冪等、rollback、幾何／label與既有生命週期故障測試。新10項整合測試補足建立兩活動與第二筆失敗原子性、任意活動true、坐下優先、重複操作／只回籠A、owner／缺失拒絕、精確快照還原、查詢／正規化失敗、預設啟動與交易rollback。2項Node測試真正執行bridge invoke與拒絕回應；不是grep判斷。

新migration只新增缺少的基本活動／正規化，**schema欄位完全不變、原migration不改**。沒有任何活動的legacy寵物才依舊uncaged值一次補散步，避免既有活動被猜測覆寫；有活動者尊重其真資料。profile預設值、選取、無關表與其他活動保留。舊欄位曾混用，無法從現值推回遺失的歷史預設意圖，須由使用者核對。升級前必須備份；本次只對唯讀online backup的副本測試，沒有寫原庫。

## ③ 版本與相容性

Rust1.99.1，lock解析Leptos0.8.22、SQLx0.9.0、Tauri2.12.5、wasm-bindgen0.2.129；四manifest／lock未變。官方對照：[Rust](https://blog.rust-lang.org/releases/)、[Leptos](https://docs.rs/crate/leptos/latest)、[SQLx](https://docs.rs/crate/sqlx/latest)、[Tauri](https://docs.rs/crate/tauri/latest)。只確認本次Linux與WASM實測，不宣稱所有CLI／native依賴最新或完整MSRV／三平台相容。

## ④ 實測、風險與待完成

| 層級 | 結果 |
| --- | --- |
| 主分支6b34 | Pet100；尚用舊runtime旗標，不符合新活動契約 |
| 隔離候選Green r3 | metadata／fmt／check／clippy通過，workspace110通過 |
| 前端 | WASM check、Node12、Trunk release build通過；初次offline缺匹配工具已記錄 |
| SQLite升級 | populated舊schema、實際本機庫唯讀backup副本、冪等、checksum漂移拒絕通過；非活動資料hash相同 |
| Native／使用者GUI | **未執行新模型**；舊A/B人工驗收不追認本候選 |

全部命令、exit、基準、失敗與重試見[本次證據](../../../source-review/local-validation-2026-10-30-day44/README.md)。受限於native操控能力，三隻同時出籠、真Native X／關控制窗、坐下後真OS位移停止、多螢幕、並發與真開窗／DB錯誤補償仍待人工／CI驗收；純helper故障測試不代表OS操作完成。產品未提交／推送，完整Gate2未完成，文章保留原檔。下一步由使用者套用並取得新SHA，再驗完整native旅程；不倒退next_day、不結案FIFO。

[需求](../../../requirements/USER_FLOWS.md) · [待辦](../../../FUTURE_WORK.md#pet-lifecycle) · [目前狀態](../../current/STATUS.md)
