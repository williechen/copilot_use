# Pet Day44 修訂｜以活動判斷出籠，不新增欄位

2026-10-30 使用者確認資料契約。產品唯讀來源 `7ce39f0166a2120539e5908b33c88322a930c35a`。原已發布方案新增 `auto_release_on_start`，不符合新要求，已撤回並 [封存](../../../archive/packages-2026-10-30/08-pet-activity-contract-superseded-2026-10-30.zip)；原始 Gate1 讀回只證明當時正文，不代表本修訂已有完整替代 Green。**本修訂的 Green 待完成／待實測，保留原檔；不新增或倒退 Day。**

## ① 目前狀態的唯一判斷

| 該寵物的活動集合 | 判斷 |
| --- | --- |
| 無活動紀錄 | 回籠 |
| 全部 is_activity=false | 回籠 |
| 任一活動 is_activity=true | 出籠 |
| 多筆 true，不論活動種類 | 仍是同一隻出籠寵物，不重複計數或開窗 |

這是「存在至少一筆 true」，不是「只能有一筆 true」，也不限散步。先核對寵物存在且屬於目前主人；找不到或越權須回錯誤，不能假裝回籠。使用既有欄位，沒有新增資料欄位的 migration。

```sql
SELECT p.id,
       EXISTS (
           SELECT 1
           FROM pet_activity a
           WHERE a.pet_id = p.id
             AND a.owner_id = p.owner_id
             AND a.is_activity = 1
       ) AS has_active_activity
FROM pet_profile p
WHERE p.id = ?1 AND p.owner_id = ?2;
```

`has_active_activity` 只是查詢結果別名，不是新增資料欄位；此 SQL 表達目標契約，尚未接入產品。建立寵物時先在同一 DB 交易建立「散步」「坐下」兩筆活動，同 pet_id／owner_id；失敗整筆回滾。活動存在不代表 true：初始回籠時兩者皆 false；若初始出籠須明確啟用選定活動。其後出籠／切換使用既有活動，不反覆插入，一般查詢不新增或重建活動；讀到散步／坐下同時 true 時，由後端正規化流程將散步改為 false。重複出籠沿既有冪等／聚焦行為。回籠／原生關窗須停止該寵物所有進行中活動，不能只停散步，不能影響 B。`is_default_closure` 保留預設關籠的原語意，不再讀寫為目前狀態；當次回籠不修改預設資格。舊資料的預設資格與活動轉換須另行驗證，不能臆測或覆蓋。

## 散步／坐下與漫遊許可

| 活動狀態 | 出／回籠 | 漫遊 |
| --- | --- | --- |
| 散步 true、坐下 false | 出籠 | 允許，仍受睡覺／拖曳等暫停條件限制 |
| 散步 false、坐下 true | 出籠 | 不允許；坐在原地，保留細微待機動畫 |
| 兩者 false，且沒有其他活動 true | 回籠 | 不允許 |
| 兩者 false，但其他活動 true | 仍出籠 | 不因其他活動而自行開啟漫遊 |
| 散步、坐下同時 true | 坐下優先，仍為出籠 | 關閉散步並保留坐下 true；停止漫遊，將散步 false 寫回 |

散步與坐下同時為 true 時，以坐下優先：保留坐下 true，將該寵物的散步持久化為 false，停止漫遊；寵物仍出籠，不影響其他活動或其他寵物。這是正常正規化規則，不當成活動衝突。 寫回失敗時停止漫遊並回報保存失敗，不能顯示已完成正規化；這不是把兩活動 true 當成錯誤，也不是回籠。

切換到散步時啟用散步、停止坐下；切換到坐下時停止散步、啟用坐下。不能先把全部活動清空讓中間狀態被誤認回籠。回籠才停止該寵物所有活動。漫遊是活動導出的許可，不新增獨立持久化旗標；睡覺／拖曳可在 runtime 暫停位移，不能因此把活動資料清空或關閉生活窗。

新寵物建立須與兩個基本活動同一交易提交，保留主人／寵物隔離。舊寵物缺少這兩活動的補齊策略、既有活動與預設設定的相容性，仍須另做保資料驗證，不刪舊活動、不從不相關活動猜測漫遊。

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

## ② 替代 Green 狀態

原 `pet-day44-green.patch` 已從現行目錄撤下。新 Green 必須將 pet_persistence::set_caged／read_caged／list_uncaged、PetStore::active／select、Tauri IPC、控制台、原生關窗與啟動組裝改接上述活動契約，沿用原鎖／補償與正式 caller。不得再新增 auto_release_on_start、is_caged 或其他狀態欄位。

新 Green 未在本次交付；本篇不將資料規則／SQL 範例當可套用完整產品修補。先完成本案 Day44 修訂再接續 Pet 新 Day；next_day 沿已同步值保持 45，修訂不另加 Day。

## ③ 版本與相容性

不新增依賴、不修改產品 Cargo.lock、不執行正式 DB migration。原方案與版本說明可由封存讀回；本次需求修正沒有重新核對官方最新穩定版本或 MSRV。獨立 Refactor 的完整原 patch 保留於 [pet-day44-refactor.patch](pet-day44-refactor.patch)，尚未在本次重跑套用／Rust/native 驗證。

## ④ 必要驗證與界線

替代 Green 必須驗證：建立時兩活動同交易提交／失敗回滾、散步允許漫遊、坐下不漫遊但仍出籠、切換冪等／坐下優先正規化、舊寵物補資料、無活動、全 false、散步 true、其他活動 true、多筆 true、A/B 主人與 UUID 隔離、重複出籠、停止單一活動但其他仍 true、回籠停止全部活動、越權／缺少寵物、DB／開關窗失敗補償、重啟及舊資料升級。保留既有狀態查詢／原生 A/B 有效斷言，改接真活動資料後重新實測；舊旗標模型的成功不能追認新契約。

[已確認流程](../../../requirements/USER_FLOWS.md) · [待辦](../../../FUTURE_WORK.md#pet-lifecycle) · [目前狀態](../../current/STATUS.md)。產品未修改、未提交／推送，Chat／Analytics Day44 原文與 patch 不變。

[隔離 SQLite 契約探針](../../current/pet-activity-contract.json) 的 12 項資料判斷通過，含非散步 true、多筆 true、停止散步但其他仍 true、A/B／owner 隔離、全部停止及 rollback；沒有新增欄位，也未改預設設定。這只驗證目標 SQL 規則，不是完整替代 Green、正式 caller／GUI 或舊庫遷移驗收。
