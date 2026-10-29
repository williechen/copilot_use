# Pet Day43｜控制台逐隻顯示回籠狀態，不再覆蓋全域訊息

> 2026-10-29；唯讀產品 [rust_ai_ring@c6708c4](https://github.com/williechen/rust_ai_ring/tree/c6708c42f21ac919e57a379848b4977d27a1c3d6)；文件起點 `copilot_use/main@d6a701e`；發布前 `next_day.pet=42`（交付後條件同步至 43）。**正式文件教學（Gate1）：產品尚未套用 Day43 變更，Gate2 尚未驗收。**

## ① 目標／前置與正式呼叫鏈

Day42 已由 `pet_ui::main::App` 每列「查詢回籠狀態」呼叫 `bridge::pet_cage_status_by_id` → `index.html::petCageStatusById` → Tauri `pet_server::pet_cage_status_by_id` → `PetStore::read_caged` → SQLite。但結果寫到共用 `new_pet_feedback`，查詢 B 會覆蓋 A 的可見訊息，不能同時比較兩隻。新增以 UUID 為 key 的控制台顯示狀態，成功／錯誤只更新該列；重新載入清單時清空舊快照，選取／出籠／回籠成功時清除該隻過期查詢結果。**不更改** DB 回籠旗標、預設出籠政策、Tauri command、native window 或漫遊。

## Refactor｜Pet 原生選單 UUID 解析歸屬（獨立）

修改前 `pet_server/src/lib.rs` 同時包含 Tauri setup、事件處理與純 `control_menu_pet_id` 字串解析；修改後把解析移到 `pet_server/src/menu_scope.rs`，在 lib.rs 宣告 `mod menu_scope;`、`use crate::menu_scope::control_menu_pet_id;`，保持正式 `app.on_menu_event` caller、原 `open_control_menu_is_scoped_to_one_valid_pet` 斷言及錯誤路徑不變。新增不同 UUID、非法 UUID、錯 action 的 fail-closed 測試；不觸及跨 workspace、共用 crates、全庫 CI。

```rust
pub(crate) fn control_menu_pet_id(menu_id: &str) -> Option<String> {
    let id = menu_id.strip_prefix("pet:")?.strip_suffix(":open_control")?;
    uuid::Uuid::parse_str(id).ok().map(|id| id.to_string())
}
```

完整新增 module、caller 和測試見 [Pet Refactor patch](pet-day43-refactor.patch)。下一步盤點 native menu event 與 Runtime 逐 ID 鎖的錯誤注入；未執行 native 測試不視為重構驗收。

### Refactor 逐檔完整 patch、正式 Tauri caller 與測試

此為原生選單純解析函式完整遷移：`pet_server/src/lib.rs` 維持 `on_menu_event` 行為，新增 `menu_scope.rs`，原 UUID 有效斷言保留，新增非法 ID 與跨 UUID 分流測試。以下為**完整修改單元與 patch 本文**；附件 [Pet Refactor patch](pet-day43-refactor.patch)。

```diff
diff --git a/pet_apps/pet_server/src/lib.rs b/pet_apps/pet_server/src/lib.rs
--- a/pet_apps/pet_server/src/lib.rs
+++ b/pet_apps/pet_server/src/lib.rs
@@ -1,5 +1,6 @@
 mod chat;
-mod file_organizer;
+mod file_organizer;
+mod menu_scope;
 pub mod pet_store;
 pub mod pet_windows;
 mod roam;
@@ -11,7 +12,8 @@
 use tauri::{Emitter, Manager, State};
 
 use crate::chat::AiState;
-use crate::roam::roam_enabled_value;
+use crate::roam::roam_enabled_value;
+use crate::menu_scope::control_menu_pet_id;
 
 // 派發寵物指令並更新狀態
 fn dispatch_pet_command(
@@ -44,12 +46,6 @@
     roam_enabled_value(value.as_deref())
 }
 
-fn control_menu_pet_id(menu_id: &str) -> Option<String> {
-    let id = menu_id
-        .strip_prefix("pet:")?
-        .strip_suffix(":open_control")?;
-    uuid::Uuid::parse_str(id).ok().map(|id| id.to_string())
-}
 
 // 取得寵物狀態
 #[tauri::command]
diff --git a/pet_apps/pet_server/src/menu_scope.rs b/pet_apps/pet_server/src/menu_scope.rs
new file mode 100644
--- /dev/null
+++ b/pet_apps/pet_server/src/menu_scope.rs
@@ -0,0 +1,21 @@
+pub(crate) fn control_menu_pet_id(menu_id: &str) -> Option<String> {
+    let id = menu_id
+        .strip_prefix("pet:")?
+        .strip_suffix(":open_control")?;
+    uuid::Uuid::parse_str(id).ok().map(|id| id.to_string())
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    #[test]
+    fn native_control_menu_accepts_only_scoped_uuid() {
+        let a = "00000000-0000-0000-0000-000000000001";
+        let b = "00000000-0000-0000-0000-000000000002";
+        assert_eq!(control_menu_pet_id(&format!("pet:{a}:open_control")), Some(a.into()));
+        assert_eq!(control_menu_pet_id(&format!("pet:{b}:open_control")), Some(b.into()));
+        for invalid in ["open_control", "pet:bad:open_control", "pet:00000000-0000-0000-0000-000000000001:sleep", "pet::open_control"] {
+            assert_eq!(control_menu_pet_id(invalid), None);
+        }
+    }
+}
```

## ② Red → 完整 Green（核心預估 15 分鐘）

**Red：** 在 `pet_ui/src/cage_status.rs` 新增 `two_pets_keep_independent_status_and_failure_is_not_false`，依序寫入 A=出籠、B=回籠、A=SQLite 失敗；要求 B 仍為回籠且 A 顯示錯誤，不得把失敗轉 false。原程式沒有逐 UUID map 與顯示單元，是缺符號／可見行為 Red；**未實際執行**。Day41/42 Node bridge、PetStore SQLite、native lifecycle 舊斷言全部保留。

**Green：** `pet_ui/src/cage_status.rs` 的完整純函式：

```rust
pub(crate) fn record_cage_status(
    statuses: &mut std::collections::HashMap<String, String>,
    id: &str,
    result: Result<bool, String>,
) {
    let text = match result {
        Ok(true) => "已回籠".to_owned(),
        Ok(false) => "已出籠".to_owned(),
        Err(error) => format!("查詢回籠狀態失敗：{error}"),
    };
    statuses.insert(id.to_owned(), text);
}
```

`pet_ui/src/main.rs` 宣告 `mod cage_status;`、import `HashMap`／`record_cage_status`，在 `App` 新增 `let cage_status = RwSignal::new(HashMap::<String,String>::new());`。原逐列查詢 closure 改為：

```rust
let result = bridge::pet_cage_status_by_id(&id).await;
cage_status.update(|statuses| record_cage_status(statuses, &id, result));
```

在同一個 `<li>` 的查詢按鈕後顯示 `<p role="status" aria-live="polite">{move || cage_status.get().get(&display_id).cloned().unwrap_or_default()}</p>`。重新載入清單清空 map；出籠、回籠、選取成功只移除該 UUID 的舊狀態。完整 [Pet Green patch](pet-day43-green.patch) 包含所有 UI 接線、純函式及測試。沒有新 dependency、Cargo feature、migration 或 JS bridge。原 `pets_busy` 排他保留；SQLite 錯誤明示，不冒稱 native 成功。

### Red／Green 逐檔完整變更與失敗隔離斷言

以下是完整 Green diff，含真正 Leptos 控制台逐 UUID 顯示、清除過期結果，以及新測試如何區分 `Err` 與 `Ok(false)`。原 Day41/42 bridge／SQLite 斷言不刪。**新 Red／Green 尚未實際執行**；附件 [Pet Green patch](pet-day43-green.patch)。

```diff
diff --git a/pet_apps/pet_ui/src/main.rs b/pet_apps/pet_ui/src/main.rs
--- a/pet_apps/pet_ui/src/main.rs
+++ b/pet_apps/pet_ui/src/main.rs
@@ -1,9 +1,12 @@
-mod bridge;
+mod bridge;
+mod cage_status;
 mod component;
 mod duplicate_preview;
 mod file_scan_view;
 
-pub use component::*;
+pub use component::*;
+use cage_status::record_cage_status;
+use std::collections::HashMap;
 
 use leptos::mount::mount_to_body;
 use leptos::{prelude::*, task::spawn_local};
@@ -52,7 +55,8 @@
     let (new_pet_feedback, set_new_pet_feedback) = signal(String::new());
 
     let (saved_pets, set_saved_pets) = signal(Vec::<bridge::SavedPet>::new());
-    let (pets_busy, set_pets_busy) = signal(false);
+    let (pets_busy, set_pets_busy) = signal(false);
+    let cage_status = RwSignal::new(HashMap::<String, String>::new());
 
     let (active_pet, set_active_pet) = signal::<Option<bridge::SavedPet>>(None);
     let profile_events = RwSignal::new(0_u64);
@@ -218,8 +222,9 @@
                         spawn_local(async move {
                             match bridge::list_pets().await {
                                 Ok(rows) => {
-                                    set_saved_pets.set(rows);
-                                    set_new_pet_feedback.set("已重新載入清單".into());
+                                    set_saved_pets.set(rows);
+                                    cage_status.set(HashMap::new());
+                                    set_new_pet_feedback.set("已重新載入清單".into());
                                 }
                                 Err(error) => set_new_pet_feedback.set(format!("讀取清單失敗：{error}")),
                             }
@@ -234,7 +239,8 @@
                             let open_id=id.clone();
                             let cage_id=id.clone();
                             let find_id=id.clone();
-                            let status_id=id.clone();
+                            let status_id=id.clone();
+                            let display_id=id.clone();
                             view! { <li>{format!("{}／{}（{}）", pet.name, pet.species, pet.id)}
                                 <button type="button" disabled=move || pets_busy.get()
                                     on:click=move |_| {
@@ -242,7 +248,7 @@
                                         set_pets_busy.set(true); let id=id.clone();
                                         spawn_local(async move {
                                             match bridge::select_pet(&id).await {
-                                                Ok(pet) => { set_active_pet.set(Some(pet)); set_new_pet_feedback.set("生活窗選取已儲存".into()); },
+                                                Ok(pet) => { cage_status.update(|statuses| { statuses.remove(&id); }); set_active_pet.set(Some(pet)); set_new_pet_feedback.set("生活窗選取已儲存".into()); },
                                                 Err(message) => set_new_pet_feedback.set(message),
                                             }
                                             set_pets_busy.set(false);
@@ -253,7 +259,7 @@
                                         if pets_busy.get_untracked() {return;}
                                         set_pets_busy.set(true);let id=open_id.clone();
                                         spawn_local(async move {
-                                            match bridge::open_saved_pet(&id).await {Ok(())=>set_new_pet_feedback.set("已出籠；重複操作只聚焦既有生活窗".into()),Err(e)=>set_new_pet_feedback.set(e)}
+                                            match bridge::open_saved_pet(&id).await {Ok(())=>{ cage_status.update(|statuses| { statuses.remove(&id); }); set_new_pet_feedback.set("已出籠；重複操作只聚焦既有生活窗".into()); },Err(e)=>set_new_pet_feedback.set(e)}
                                             set_pets_busy.set(false);
                                         });
                                     }>"出籠"</button>
@@ -263,14 +269,12 @@
                                         set_pets_busy.set(true);
                                         let id = status_id.clone();
                                         spawn_local(async move {
-                                            match bridge::pet_cage_status_by_id(&id).await {
-                                                Ok(true) => set_new_pet_feedback.set(format!("{id}：已回籠")),
-                                                Ok(false) => set_new_pet_feedback.set(format!("{id}：已出籠")),
-                                                Err(error) => set_new_pet_feedback.set(format!("查詢回籠狀態失敗：{error}")),
-                                            }
+                                            let result = bridge::pet_cage_status_by_id(&id).await;
+                                            cage_status.update(|statuses| record_cage_status(statuses, &id, result));
                                             set_pets_busy.set(false);
                                         });
-                                    }>"查詢回籠狀態"</button>
+                                    }>"查詢回籠狀態"</button>
+                                <p role="status" aria-live="polite">{move || cage_status.get().get(&display_id).cloned().unwrap_or_default()}</p>
                                 <button type="button" disabled=move || pets_busy.get()
                                     on:click=move |_| {
                                         if pets_busy.get_untracked() { return; }
@@ -278,8 +282,9 @@
                                         let id = cage_id.clone();
                                         spawn_local(async move {
                                             match bridge::cage_saved_pet_by_id(&id).await {
-                                                Ok(()) => {
-                                                    if active_pet.get_untracked().as_ref()
+                                                Ok(()) => {
+                                                    cage_status.update(|statuses| { statuses.remove(&id); });
+                                                    if active_pet.get_untracked().as_ref()
                                                         .is_some_and(|pet| pet.id == id) {
                                                         set_active_pet.set(None);
                                                         set_snapshot.set(None);
diff --git a/pet_apps/pet_ui/src/cage_status.rs b/pet_apps/pet_ui/src/cage_status.rs
new file mode 100644
--- /dev/null
+++ b/pet_apps/pet_ui/src/cage_status.rs
@@ -0,0 +1,31 @@
+use std::collections::HashMap;
+
+/// Store the result under the queried UUID only; an IPC failure is not false.
+pub(crate) fn record_cage_status(
+    statuses: &mut HashMap<String, String>,
+    id: &str,
+    result: Result<bool, String>,
+) {
+    let text = match result {
+        Ok(true) => "已回籠".to_owned(),
+        Ok(false) => "已出籠".to_owned(),
+        Err(error) => format!("查詢回籠狀態失敗：{error}"),
+    };
+    statuses.insert(id.to_owned(), text);
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    #[test]
+    fn two_pets_keep_independent_status_and_failure_is_not_false() {
+        let mut statuses = HashMap::new();
+        record_cage_status(&mut statuses, "pet-a", Ok(false));
+        record_cage_status(&mut statuses, "pet-b", Ok(true));
+        assert_eq!(statuses["pet-a"], "已出籠");
+        assert_eq!(statuses["pet-b"], "已回籠");
+        record_cage_status(&mut statuses, "pet-a", Err("SQLite read failed".into()));
+        assert!(statuses["pet-a"].contains("查詢回籠狀態失敗"));
+        assert_eq!(statuses["pet-b"], "已回籠");
+    }
+}
```

## ③ 版本（2026-10-29）

Rust stable [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)、Tauri [2.12.5](https://docs.rs/crate/tauri/latest)、Leptos [0.8.22](https://docs.rs/crate/leptos/latest)、SQLx [0.9.0](https://docs.rs/crate/sqlx/latest)、Tokio [1.54.1](https://docs.rs/crate/tokio/latest)、UUID [1.29.0](https://docs.rs/crate/uuid/latest)；Tauri CLI 當日最新穩定版為 [2.12.4（2026-10-21）](https://docs.rs/crate/tauri-cli/latest)（3.0.0-alpha 為預發行，不採用）；本機 CLI 版本與 native WebKitGTK/OS 套件仍未核對。產品四 workspace `0.2.42`，Pet lock resolved 依 [Day42 最新版本表](../day-42/versions-2026-10-29.md)。本輪無實機遞迴 MSRV／native／WASM 驗證，未宣稱相容性通過、不升級產品；當日官方版號、現有 lock 與已知限制另見 [Day43 版本基線](versions.md)。

## ④ 驗證、套用、驗收與下一步

已做：精確 GitHub 來源、正式 Tauri caller／Leptos 清單、既有 Node bridge、原斷言讀回；生成逐檔 patch 與來源片段唯一比對。**未做**：真 `git apply --check`、Red、Cargo、Node、SQLite、Tauri GUI／WebKit、CI；Gate1 文件交付後按獨立讀回證據結案；Gate2 未執行。隔離 checkout `c6708c42f21ac919e57a379848b4977d27a1c3d6` 後先 Refactor 再 Green：

```bash
git apply --check pet-day43-refactor.patch && git apply pet-day43-refactor.patch
git apply --check pet-day43-green.patch && git apply pet-day43-green.patch
cargo fmt --all --manifest-path pet_apps/Cargo.toml -- --check
cargo test --locked --manifest-path pet_apps/Cargo.toml -p pet-server
cargo test --locked --manifest-path pet_apps/Cargo.toml -p pet-ui
cargo check --locked --manifest-path pet_apps/Cargo.toml -p pet-ui --target wasm32-unknown-unknown
node --test pet_apps/pet_ui/tests/*.test.mjs
```

人工驗收：A/B 出籠→分別查狀態→B 回籠→A 訊息不受影響→重新查 B→SQLite 故障只顯示 B 錯誤→重載清單清除舊顯示；核對原生視窗、重啟與預設資格仍為既有待辦。核心 15 分鐘只計 Red/Green；Refactor 與其餘另計。Gate1 文件正式交付／產品套用／功能驗收分列；本案按已交付證據條件同步 `next_day`，FIFO 與 `last_full_comparison` 不變。


## 文件交付與來源證據

原 [Day43 候選](../../drafts/day-43/pet.md) 的歷史 commit、blob 保留；六份 patch 的嚴格上下文／增刪行數檢查見 [候選證據](../../drafts/day-43/candidate-evidence.json)，不等於原生 `git apply --check` 或 Cargo。先 Refactor 後 Green，不修改產品 repo；版本差異與 Gate1/Gate2 邊界見 [版本基線](versions.md)，本文及 patch 的精確讀回記在 [交付證據](delivery-verification.json)。

## 2026-10-29 局部原生驗收增量

在 `8b882696c5548accb9d98a32b27db35e11109c74` 隔離副本完成人工 A/B 初始查詢、A 單獨出籠／回籠及重啟保留狀態，逐步對照相同 UUID 的 SQLite 與程序 FD。這不等於本文完整逐列顯示、錯誤與重載情境通過；原文與 patch 保留。見 [Pet 原生驗收證據與限制](../../../source-review/pet-native-2026-10-29/README.md)。
