# Pet Day44 修訂r4｜活動模型已在main，補齊舊活動代碼升級

## ① 現況與正式caller

唯一產品唯讀`4b1008b95bc3e170381fcd1db84695e421f036bd`。先前活動Green已由遠端採用，main本地110項、Node12與WASM通過，正式runtime籠態由pet_activity導出，既有profile欄位只處理啟動預設。資料儲存代碼`walk`／`sit`，字典顯示「散步／坐下」；本篇補齊舊中文活動值的forward升級與真回歸驗證。next_day維持45，不新增Day。

| 活動條件（同pet_id／owner_id） | 籠態／漫遊 |
| --- | --- |
| 任一活動true，含其他活動或多筆true | 出籠，不限walk |
| walk true、sit false | 出籠，允許漫遊 |
| sit true，含walk亦true | 坐下優先；持久化walk=false，仍出籠但不漫遊 |
| 全false／無活動 | 回籠 |

正式鏈：Leptos ActivityControls→bridge／index.html invoke→Tauri逐UUID command→PetStore→activity交易。新建profile與walk／sit兩筆inactive同交易，隨後既有UI明確select／出籠；重複出籠沿用既有動作，不重複插入。回籠／Native X停止該隻全部活動，不動其他寵物或預設；`is_default_closure`僅作既有啟動預設，不新增欄位。startup只恢復預設可出籠者，其他停止活動；原生效果失敗還原精確活動與選取snapshot。漫遊每次位移前鎖內核對DB／generation，受既有PET_ENABLE_ROAM opt-in、睡覺與拖曳暫停限制。這些caller已在main，native視覺旅程仍待驗。

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

本案 Refactor 已在7ce基準真check／apply，且最新4b1008b已有實作。保留幾何邊界／負座標舊斷言；最新main110項與候選112項皆涵蓋此模組。此幾何重構獨立於活動Green，不抵算跨案工程。



## ② Red → 完整forward Green

### 為何需要本修訂

最新main已把新活動改成walk／sit，但舊庫可能有「散步／坐下」。第4版migration會建立缺少的英文基本活動，卻不轉換原中文列；舊sit未被英文sit優先規則辨認。升級example也還用中文代碼檢查新資料。不能以fresh DB或main110項成功宣稱舊庫相容完成。

本修訂**不改任何既有migration**。新增第5版data-only migration轉換兩個已知舊標籤並正規化坐下優先；保留活動ID、pet／owner、其他活動、預設、字典及第4版checksum。可能已由第4版建立的同類型多列不刪除，正式操作會作用於該pet／owner的所有同類型列；這不新增活動或狀態欄位。

同一回歸測試在最後主分支加測時行為Red：原ID仍為中文且walk／sit同為true；完整patch後轉為英文且walk=false、sit=true，其他play保留。另測既有字典主鍵碰撞時整筆migration回滾、不覆寫原資料。測試沒有刪除既有有效斷言。

### 完整逐檔程式／測試附件

[pet-day44-legacy-code-upgrade-r4.patch](pet-day44-legacy-code-upgrade-r4.patch)以4b1008b為唯一套用基準。它包含新增migration、完整兩項SQLx回歸測試、升級example修正；其餘活動caller與依賴已在main，是本patch前置，不能再套舊r3 Green或原Refactor。下列為附件全文：

```diff
diff --git a/pet_apps/migrations/20261030140000_normalize_activity_codes.sql b/pet_apps/migrations/20261030140000_normalize_activity_codes.sql
new file mode 100644
index 0000000..d84b1ea
--- /dev/null
+++ b/pet_apps/migrations/20261030140000_normalize_activity_codes.sql
@@ -0,0 +1,6 @@
+-- Preserve all activity IDs, owner links, dictionary rows and deployed migration checksums.
+UPDATE pet_activity SET activity_type=CASE activity_type WHEN '散步' THEN 'walk' WHEN '坐下' THEN 'sit' END
+WHERE activity_type IN ('散步','坐下');
+UPDATE pet_activity AS w SET is_activity=0
+WHERE w.activity_type='walk' AND w.is_activity=1
+AND EXISTS(SELECT 1 FROM pet_activity s WHERE s.pet_id=w.pet_id AND s.owner_id=w.owner_id AND s.activity_type='sit' AND s.is_activity=1);
diff --git a/pet_apps/pet_server/examples/day44_upgrade.rs b/pet_apps/pet_server/examples/day44_upgrade.rs
index 9d12c4d..3fa25aa 100644
--- a/pet_apps/pet_server/examples/day44_upgrade.rs
+++ b/pet_apps/pet_server/examples/day44_upgrade.rs
@@ -63,9 +63,9 @@ async fn main() -> Result<(), Box<dyn std::error::Error>> {
         .fetch_all(&pool)
         .await?
     );
-    let missing:i64=sqlx::query_scalar("SELECT count(*) FROM pet_profile p WHERE NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=p.id AND a.owner_id=p.owner_id AND a.activity_type='散步') OR NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=p.id AND a.owner_id=p.owner_id AND a.activity_type='坐下')").fetch_one(&pool).await?;
+    let missing:i64=sqlx::query_scalar("SELECT count(*) FROM pet_profile p WHERE NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=p.id AND a.owner_id=p.owner_id AND a.activity_type='walk') OR NOT EXISTS(SELECT 1 FROM pet_activity a WHERE a.pet_id=p.id AND a.owner_id=p.owner_id AND a.activity_type='sit')").fetch_one(&pool).await?;
     assert_eq!(missing, 0);
-    let conflicts:i64=sqlx::query_scalar("SELECT count(*) FROM pet_activity w WHERE w.activity_type='散步' AND w.is_activity=1 AND EXISTS(SELECT 1 FROM pet_activity s WHERE s.pet_id=w.pet_id AND s.owner_id=w.owner_id AND s.activity_type='坐下' AND s.is_activity=1)").fetch_one(&pool).await?;
+    let conflicts:i64=sqlx::query_scalar("SELECT count(*) FROM pet_activity w WHERE w.activity_type='walk' AND w.is_activity=1 AND EXISTS(SELECT 1 FROM pet_activity s WHERE s.pet_id=w.pet_id AND s.owner_id=w.owner_id AND s.activity_type='sit' AND s.is_activity=1)").fetch_one(&pool).await?;
     assert_eq!(conflicts, 0);
     if args.get(4).is_some_and(|s| s == "seed") {
         let out = pet_persistence::activity::status(&pool, "old-out").await?;
diff --git a/pet_apps/pet_server/tests/day44_activity_migration.rs b/pet_apps/pet_server/tests/day44_activity_migration.rs
new file mode 100644
index 0000000..bbb2ea8
--- /dev/null
+++ b/pet_apps/pet_server/tests/day44_activity_migration.rs
@@ -0,0 +1,96 @@
+use sqlx::{SqlitePool, migrate::Migrator, sqlite::SqlitePoolOptions};
+
+async fn old_database() -> (SqlitePool, Migrator) {
+    let pool = SqlitePoolOptions::new()
+        .max_connections(1)
+        .connect("sqlite::memory:")
+        .await
+        .unwrap();
+    let all = sqlx::migrate!("../migrations");
+    let old = Migrator::with_migrations(
+        all.iter()
+            .filter(|m| m.version < 20261030060000)
+            .cloned()
+            .collect(),
+    );
+    old.run(&pool).await.unwrap();
+    sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','Owner')")
+        .execute(&pool)
+        .await
+        .unwrap();
+    sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id,is_default_closure) VALUES('a','A','tortoise','local-owner',0)").execute(&pool).await.unwrap();
+    (pool, all)
+}
+
+#[tokio::test]
+async fn old_display_names_migrate_to_codes_with_sit_priority_and_preserve_dictionary() {
+    let (pool, all) = old_database().await;
+    sqlx::query("INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity) VALUES('w','a','local-owner','散步',1),('s','a','local-owner','坐下',1),('p','a','local-owner','play',1)").execute(&pool).await.unwrap();
+    sqlx::query("INSERT INTO codemap(id,category,code,value) VALUES('custom-walk','activity','walk','自訂散步')").execute(&pool).await.unwrap();
+    all.run(&pool)
+        .await
+        .expect("new migration must run on SQLite");
+    all.run(&pool).await.unwrap();
+    let activities: Vec<(String, String, i64)> =
+        sqlx::query_as("SELECT id,activity_type,is_activity FROM pet_activity WHERE id IN ('w','s','p') ORDER BY id")
+            .fetch_all(&pool)
+            .await
+            .unwrap();
+    assert_eq!(
+        activities,
+        vec![
+            ("p".into(), "play".into(), 1),
+            ("s".into(), "sit".into(), 1),
+            ("w".into(), "walk".into(), 0)
+        ]
+    );
+    let custom: String = sqlx::query_scalar("SELECT value FROM codemap WHERE id='custom-walk'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(custom, "自訂散步");
+    let dictionary_count: i64 = sqlx::query_scalar("SELECT count(*) FROM codemap")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(dictionary_count, 3); // existing row plus main's two initial dictionary rows, none deleted
+    let activity_count: i64 = sqlx::query_scalar("SELECT count(*) FROM pet_activity")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(activity_count, 5); // three original IDs plus the two basic rows created by deployed migration 4
+    let timestamp: Option<String> =
+        sqlx::query_scalar("SELECT created_at FROM codemap WHERE code='sit'")
+            .fetch_one(&pool)
+            .await
+            .unwrap();
+    assert!(timestamp.is_some_and(|s| !s.is_empty()));
+    let status = pet_persistence::activity::status(&pool, "a").await.unwrap();
+    assert!(status.outside && !status.roaming && !status.default_caged);
+    pool.close().await;
+}
+
+#[tokio::test]
+async fn dictionary_id_collision_rolls_back_every_activity_change() {
+    let (pool, all) = old_database().await;
+    sqlx::query("INSERT INTO pet_activity(id,pet_id,owner_id,activity_type,is_activity) VALUES('w','a','local-owner','散步',1)").execute(&pool).await.unwrap();
+    sqlx::query("INSERT INTO codemap(id,category,code,value) VALUES('initial-sit','other','keep','untouched')").execute(&pool).await.unwrap();
+    assert!(all.run(&pool).await.is_err());
+    let activities: Vec<(String, String, i64)> =
+        sqlx::query_as("SELECT id,activity_type,is_activity FROM pet_activity")
+            .fetch_all(&pool)
+            .await
+            .unwrap();
+    assert_eq!(activities, vec![("w".into(), "散步".into(), 1)]);
+    let dictionary: Vec<(String, String)> = sqlx::query_as("SELECT code,value FROM codemap")
+        .fetch_all(&pool)
+        .await
+        .unwrap();
+    assert_eq!(dictionary, vec![("keep".into(), "untouched".into())]);
+    let versions: i64 = sqlx::query_scalar("SELECT count(*) FROM _sqlx_migrations WHERE success=1")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(versions, 3);
+    pool.close().await;
+}
```

```bash
# 在4b1008b的隔離可寫clone，DB也必須是備份副本
git apply --check /path/to/pet-day44-legacy-code-upgrade-r4.patch
git apply /path/to/pet-day44-legacy-code-upgrade-r4.patch
cargo metadata --locked --no-deps --format-version 1 --manifest-path pet_apps/Cargo.toml
cargo test --locked --workspace --manifest-path pet_apps/Cargo.toml
```

新增migration檔後必須確保SQLx migration巨集的消費端重新編譯；本次已重建本地Rust來源，保留命令與來源證據。原第4版遇到既有字典主鍵碰撞仍安全拒絕，需人工核對後另做保資料處理；本patch不自動覆寫、不刪字典或繞過checksum。SQLite升級本次只用合成舊schema及原庫mode=ro online backup副本，沒有寫原庫；原本已有的字典列與其他表前後相同。

## ③ 版本／相依

四workspace Cargo.toml／Cargo.lock均未變，沿本輪實際工具與鎖定套件；[版本與metadata](../../../source-review/local-validation-2026-10-30-day44-r2/final-main/commands.json)。SQLx0.9／Leptos0.8.22／Tauri2.12.5及Rust1.99.1沿既有鎖定來源，不宣稱完整官方最新版本／MSRV／跨平台Gate已完成。

## ④ 實測與未完成

| 層級 | 最後來源／候選實測 |
| --- | --- |
| main 4b1008b | Pet110、Node12、WASM通過 |
| 同來源＋forward r4 | 四workspace metadata；Pet fmt／check／clippy、workspace112、Node12、WASM、Trunk dev建置通過 |
| DB | 舊3版→main4版＋forward5版，合成／實際SQLite備份副本、冪等、checksum拒絕通過 |
| 新模型native GUI | 未執行；舊A/B人工結果不追認新模型 |

已真reverse／check／apply後驗證，原第4版及lock byte-for-byte不變。[全部命令／exit、Red→Green、清理與資料保留](../../../source-review/local-validation-2026-10-30-day44-r2/README.md)。forward patch沒有推送產品；三隻、原生X／關控制台、真OS失敗補償、漫遊／多螢幕／併發與完整GUI仍待驗，完整Gate2=false。既有FIFO不結案、不改last_full_comparison。

[需求](../../../requirements/USER_FLOWS.md) · [待辦](../../../FUTURE_WORK.md#pet-lifecycle) · [STATUS](../../current/STATUS.md)
