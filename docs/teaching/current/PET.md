## 1. 學習大綱：兩隻以上同時出籠，各有自己的生活窗

這是 Pet Day24 r2，依使用者確認把原 r1 的單生活窗切片補成「任意已儲存寵物依 ID 出籠→各建原生生活窗→獨立狀態／事件→單隻回籠」。不是寫死兩隻；實際 GUI 使用三隻同時顯示，另以十二個 UUID 驗證標籤沒有兩窗上限。視窗數仍受系統記憶體與桌面空間限制，不宣稱無限資源。

來源 `rust_ai_chat/main`：`4f0cd00b8d571bdaf120298efebf0d62b795498f`，workspace 0.2.23。教學初始基準 `f3b32b6a27d06db72e40f7d92a1530f52d549342`；規則 daily-tdd-minimum-v1、GitHub blob `ebebad6b3d671b7cdfa48a5f121a9958010b15b4`，共同 Notion 更新 `2026-10-20T16:32:31.684Z`。發布前 c965681→4f0cd00 的差異只有 root AI migration；Pet 程式、依賴與 SQL 未改，已於更新基準重驗。Day23（含）以前正文不動，r1 原文／patch／讀回證據保存在本日 revisions；修訂沿用同一 Notion 頁面，next_day 維持25，不再遞增。

真實主線：控制台建立並讀回多筆三欄 `SavedPet` → 各列按「出籠」→ JS／Rust bridge → Tauri `open_saved_pet` → 查核 `local-owner` profile → 以 `pet-UUID` 動態建窗 → URL 固定帶該隻 ID → 查自己的 profile 與 FSM。單隻狀態由 Runtime HashMap 以 ID 分流；事件 envelope 包含 ID，前端只接受自己的事件，原生右鍵 menu ID 也綁定該隻。這不是只把相同畫面複製三份。

「出籠」不更換其他生活窗，也不把清單第一筆當預設；已出籠再按只顯示並聚焦。生活窗的「回籠子」關閉自己、移除自己的活動狀態，保留 SQLite profile 與選取設定。關控制窗只隱藏控制窗。原靜態匿名 pet 視窗移除，只保留 main 控制窗；啟動只載入明確保存的單一預設身分。

本篇仍未實作多個啟動預設、逐寵物持久 FSM／漫遊開關、跨螢幕自然漫遊、完整外觀素材。舊 roam 只認 `pet` 標籤，動態生活窗不套此舊迴圈；明確啟用舊環境開關也會找不到舊窗並停止，不會移動控制窗或錯移其他寵物。初始物種僅支援既有建立值 tortoise，以烏龜 emoji 顯示與簡單狀態動畫，不冒稱自訂素材引擎。`copilot_use` 產品的 registry／DesktopState 契約另行整合，不能直接視為已套用。

主體 Red／Green 估25分鐘（含r2補接線），Refactor／版本／驗證閱讀另15分鐘；三案主體合計約50分鐘，不以複雜度填滿時間。SQLite 前置、舊 migration/checksum 邊界與 r1 相同：新增選取表，不修改任何歷史 SQL、不刪舊庫。

## 2. 實作內容：Red → 完整最小 Green → Refactor

### Red：重現不是只有兩隻的需求

最初 store Red 是缺少 `active/select` 的 E0599，原始 log 保留。r2 另外對 r1 原生程式做行為 Red：建立三隻、逐次選為生活窗寵物，預期控制窗加三個生活窗共四窗，實際只有兩窗（控制窗＋同一生活窗），下列斷言失敗。這是已重現的產品行為限制，不拿既有通過測試冒稱 Red。

```py
# 完整 helper、建三筆資料、驅動與清理見 multi-native-red-test.py。
handles = ses('GET', '/window/handles')
assert len(handles) == 4, 'expected control plus three life windows; actual=' + str(len(handles))
# r1 實際：AssertionError ... actual=2
```

### Green：依每隻真實 ID 動態建立完整呼叫鏈

以下為精確 4f0cd00 基準的完整 r2 patch，包含 r1 的 store／選取 migration／測試，再加入動態視窗 Runtime、獨立狀態命令、IPC／event／原生 menu ID 分流、動態視窗 capability、控制窗出籠與生活窗回籠、200×200可操作佈局。已套過 r1 時請先比對差異，不要把完整 patch 重貼到已修改工作樹；原始 r1 與 r2 patch 均保留。對乾淨基準 apply check 通過，沒有更新 Cargo 或 lock。

```diff
diff --git a/apps/desktop_pet/capabilities/default.json b/apps/desktop_pet/capabilities/default.json
index b893b2e..ef634db 100644
--- a/apps/desktop_pet/capabilities/default.json
+++ b/apps/desktop_pet/capabilities/default.json
@@ -2,9 +2,12 @@
   "$schema": "../gen/schemas/desktop-schema.json",
   "identifier": "default",
   "description": "Default permissions for Rust AI Desktop Pet",
-  "windows": ["main", "pet"],
+  "windows": [
+    "main",
+    "pet-*"
+  ],
   "permissions": [
     "core:default",
     "core:window:allow-start-dragging"
   ]
-}
\ No newline at end of file
+}
diff --git a/apps/desktop_pet/frontend/index.html b/apps/desktop_pet/frontend/index.html
index 7242117..fc92d82 100644
--- a/apps/desktop_pet/frontend/index.html
+++ b/apps/desktop_pet/frontend/index.html
@@ -17,20 +17,19 @@
     const { invoke } = window.__TAURI__.core;
     const { listen } = window.__TAURI__.event;
 
-    window.getPetState = () => {
-      return invoke("get_pet_state");
-    };
-
-    window.sendPetCommand = (command) => {
-      return invoke("send_pet_command", { command });
-    };
-
-    window.listenPetState = (callback) => {
-      return listen(
-        "pet://state-changed",
-        (event) => callback(event.payload),
-      );
+    const windowPetId = new URLSearchParams(location.search).get("pet_id");
+    const currentPetId = async () => {
+      const id = windowPetId || (await invoke("active_pet"))?.id;
+      if (!id) throw "請先選擇並出籠寵物";
+      return id;
     };
+    window.getPetState = async () => invoke("get_saved_pet_state", { id: await currentPetId() });
+    window.sendPetCommand = async (command) => invoke("send_saved_pet_command", { id: await currentPetId(), command });
+    window.listenPetState = (callback) => listen("saved-pet-state", async (event) => {
+      try { if (event.payload.id === await currentPetId()) callback(event.payload.snapshot); } catch (_) {}
+    });
+    window.openSavedPet = (id) => invoke("open_saved_pet", { id });
+    window.cageSavedPet = async () => invoke("cage_saved_pet", { id: await currentPetId() });
 
     window.chatWithPet = (message) => {
       return invoke("chat_with_pet", { message });
@@ -60,15 +59,15 @@
       petContextMenu = await Menu.new({
         items: [
           {
-            id: "interact",
+            id: `pet:${await currentPetId()}:interact`,
             text: "互動",
           },
           {
-            id: "sleep",
+            id: `pet:${await currentPetId()}:sleep`,
             text: "睡覺",
           },
           {
-            id: "wake",
+            id: `pet:${await currentPetId()}:wake`,
             text: "醒來",
           },
         ],
@@ -104,6 +103,11 @@
       return invoke("scan_duplicate_preview");
     };
 
+    window.activePet = () => windowPetId ? invoke("saved_pet_profile", { id: windowPetId }) : invoke("active_pet");
+    window.selectPet = (id) => invoke("select_pet", { id });
+    window.listenPetProfile = (callback) => window.__TAURI__.event.listen(
+      "pet-profile-changed", (event) => { if (!windowPetId || event.payload.id === windowPetId) callback(event.payload); });
+
     window.listPets = () => {
       return invoke("list_pets");
     };
diff --git a/apps/desktop_pet/frontend/src/bridge.rs b/apps/desktop_pet/frontend/src/bridge.rs
index 0a6e104..6dca3f5 100644
--- a/apps/desktop_pet/frontend/src/bridge.rs
+++ b/apps/desktop_pet/frontend/src/bridge.rs
@@ -5,6 +5,17 @@ use wasm_bindgen_futures::JsFuture;
 
 #[wasm_bindgen]
 extern "C" {
+    #[wasm_bindgen(js_namespace=window, js_name=openSavedPet)]
+    fn open_saved_pet_js(id: &str) -> Promise;
+    #[wasm_bindgen(js_namespace=window, js_name=cageSavedPet)]
+    fn cage_saved_pet_js() -> Promise;
+
+    #[wasm_bindgen(js_namespace=window, js_name=activePet)]
+    fn active_pet_js() -> Promise;
+    #[wasm_bindgen(js_namespace=window, js_name=selectPet)]
+    fn select_pet_js(id: &str) -> Promise;
+    #[wasm_bindgen(js_namespace=window, js_name=listenPetProfile)]
+    fn listen_pet_profile_js(callback: &Function) -> Promise;
 
     #[wasm_bindgen(
         js_namespace=window,
@@ -184,3 +195,39 @@ pub async fn list_pets() -> Result<Vec<SavedPet>, String> {
     let value = JsFuture::from(list_pets_js()).await.map_err(js_error)?;
     serde_wasm_bindgen::from_value(value).map_err(|_| "寵物清單格式錯誤".into())
 }
+
+pub async fn active_pet() -> Result<Option<SavedPet>, String> {
+    let value = JsFuture::from(active_pet_js()).await.map_err(js_error)?;
+    serde_wasm_bindgen::from_value(value).map_err(|_| "生活窗資料格式錯誤".into())
+}
+pub async fn select_pet(id: &str) -> Result<SavedPet, String> {
+    let value = JsFuture::from(select_pet_js(id)).await.map_err(js_error)?;
+    serde_wasm_bindgen::from_value(value).map_err(|_| "選取資料格式錯誤".into())
+}
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
+pub async fn cage_saved_pet() -> Result<(), String> {
+    JsFuture::from(cage_saved_pet_js())
+        .await
+        .map_err(js_error)?;
+    Ok(())
+}
diff --git a/apps/desktop_pet/frontend/src/component/activity.rs b/apps/desktop_pet/frontend/src/component/activity.rs
index c09a246..6ed0383 100644
--- a/apps/desktop_pet/frontend/src/component/activity.rs
+++ b/apps/desktop_pet/frontend/src/component/activity.rs
@@ -118,12 +118,12 @@ pub fn PetStatus(
     };
 
     let face = Signal::derive(move || match pose() {
-        Some(PetPose::Idle) => "😺",
-        Some(PetPose::Blink) => "😻",
-        Some(PetPose::InteractA) => "😸",
-        Some(PetPose::InteractB) => "😹",
-        Some(PetPose::SleepA) => "😴",
-        Some(PetPose::SleepB) => "😪",
+        Some(PetPose::Idle) => "🐢",
+        Some(PetPose::Blink) => "🐢·",
+        Some(PetPose::InteractA) => "🐢♪",
+        Some(PetPose::InteractB) => "🐢♫",
+        Some(PetPose::SleepA) => "🐢💤",
+        Some(PetPose::SleepB) => "🐢z",
         None => "🐾",
     });
 
diff --git a/apps/desktop_pet/frontend/src/main.rs b/apps/desktop_pet/frontend/src/main.rs
index 080f06b..8e1951b 100644
--- a/apps/desktop_pet/frontend/src/main.rs
+++ b/apps/desktop_pet/frontend/src/main.rs
@@ -54,6 +54,32 @@ fn App() -> impl IntoView {
     let (saved_pets, set_saved_pets) = signal(Vec::<bridge::SavedPet>::new());
     let (pets_busy, set_pets_busy) = signal(false);
 
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
     if !bridge::is_pet_window() {
         set_pets_busy.set(true);
         spawn_local(async move {
@@ -106,7 +132,16 @@ fn App() -> impl IntoView {
                     set_blinking
                 />
                 <AnimationClock set_animation_phase />
-                <PetStatus snapshot blinking animation_phase />
+                <p title=move || active_pet.get().map(|p|p.id).unwrap_or_default()>
+                    {move || active_pet.get().map(|p|p.name).unwrap_or_else(|| "請在控制台選擇生活窗寵物".into())}
+                </p>
+                <Show when=move || active_pet.get().is_some() fallback=|| ()>
+                    <PetStatus snapshot blinking animation_phase />
+                </Show>
+                <button type="button" on:click=move |_| spawn_local(async move {
+                    if let Err(e)=bridge::cage_saved_pet().await {set_error.set(Some(e));}
+                })>"回籠子"</button>
+                <p role="alert">{move || error.get().unwrap_or_default()}</p>
                 <button
                     type="button"
                     on:click=move |_| spawn_local(async move{
@@ -173,7 +208,32 @@ fn App() -> impl IntoView {
                 <Show when=move || pets_busy.get() fallback=|| ()><p>"處理中…"</p></Show>
                 <ul>
                     <For each=move || saved_pets.get() key=|pet| pet.id.clone()
-                        children=|pet| view! { <li>{format!("{}／{}（{}）", pet.name, pet.species, pet.id)}</li> } />
+                        children=move |pet| {
+                            let id=pet.id.clone(); let open_id=id.clone();
+                            view! { <li>{format!("{}／{}（{}）", pet.name, pet.species, pet.id)}
+                                <button type="button" disabled=move || pets_busy.get()
+                                    on:click=move |_| {
+                                        if pets_busy.get_untracked() { return; }
+                                        set_pets_busy.set(true); let id=id.clone();
+                                        spawn_local(async move {
+                                            match bridge::select_pet(&id).await {
+                                                Ok(pet) => { set_active_pet.set(Some(pet)); set_new_pet_feedback.set("生活窗選取已儲存".into()); },
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
+                                            match bridge::open_saved_pet(&id).await {Ok(())=>set_new_pet_feedback.set("已出籠；重複操作只聚焦既有生活窗".into()),Err(e)=>set_new_pet_feedback.set(e)}
+                                            set_pets_busy.set(false);
+                                        });
+                                    }>"出籠"</button>
+                            </li> }
+                        } />
                 </ul>
             </section>
 
diff --git a/apps/desktop_pet/frontend/style.css b/apps/desktop_pet/frontend/style.css
index 99ead04..cf027b8 100644
--- a/apps/desktop_pet/frontend/style.css
+++ b/apps/desktop_pet/frontend/style.css
@@ -199,4 +199,11 @@ body {
 
 .pet-life .pet-face{
   line-height:50px;
-}
\ No newline at end of file
+}
+/* Each dynamic life window remains usable within the existing 200x200 shell. */
+.pet-life > p { margin:2px 0; font-size:12px; line-height:16px; white-space:nowrap; overflow:hidden; text-overflow:ellipsis; }
+.pet-life > p:empty { display:none; }
+.pet-life .pet-status strong { font-size:12px; line-height:14px; }
+.pet-life .pet-status small { font-size:10px; line-height:12px; }
+.pet-life > button { display:inline-block; padding:2px 6px; margin:2px; }
+.pet-life > p, .pet-life strong, .pet-life small { color:white; text-shadow:0 1px 3px #000; }
diff --git a/apps/desktop_pet/migrations/20261021070000_active_pet.sql b/apps/desktop_pet/migrations/20261021070000_active_pet.sql
new file mode 100644
index 0000000..3eb9a01
--- /dev/null
+++ b/apps/desktop_pet/migrations/20261021070000_active_pet.sql
@@ -0,0 +1,5 @@
+-- Additive selection for the existing single life window. No arbitrary default.
+CREATE TABLE local_pet_selection (
+ slot INTEGER PRIMARY KEY CHECK(slot=1),
+ pet_id TEXT NOT NULL REFERENCES pet_profile(id) ON DELETE CASCADE
+);
diff --git a/apps/desktop_pet/src/lib.rs b/apps/desktop_pet/src/lib.rs
index ad65ded..ad642f1 100644
--- a/apps/desktop_pet/src/lib.rs
+++ b/apps/desktop_pet/src/lib.rs
@@ -1,6 +1,7 @@
 mod chat;
 mod file_organizer;
 pub mod pet_store;
+pub mod pet_windows;
 mod roam;
 
 use ai_core::{ChatRequest, MockChatModel};
@@ -128,9 +129,28 @@ async fn list_pets(
     store.list().await
 }
 
+#[tauri::command]
+async fn active_pet(store: State<'_, pet_store::PetStore>) -> Result<Option<pet_store::SavedPet>, String> {
+    store.active().await
+}
+#[tauri::command]
+async fn select_pet(id: String, app: tauri::AppHandle, store: State<'_, pet_store::PetStore>) -> Result<pet_store::SavedPet, String> {
+    let saved=store.select(&id).await?;
+    pet_windows::open(&app,&saved)?;
+    app.emit("pet-profile-changed", &saved).map_err(|_| "選取已儲存，通知失敗，請重新載入生活窗".to_owned())?;
+    Ok(saved)
+}
+
 pub fn application(db: sqlx::SqlitePool) {
     tauri::Builder::default()
         .on_window_event(|window, event| {
+            if window.label().starts_with("pet-") {
+                if let tauri::WindowEvent::Destroyed=event {
+                    if let Some(id)=window.label().strip_prefix("pet-").and_then(|s| uuid::Uuid::parse_str(s).ok()) {
+                        window.app_handle().state::<pet_windows::Runtime>().remove(&id.to_string());
+                    }
+                }
+            }
             if window.label() == "main" {
                 if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                     api.prevent_close();
@@ -139,9 +159,17 @@ pub fn application(db: sqlx::SqlitePool) {
             }
         })
         .manage(Mutex::new(PetMachine::default()))
+        .manage(pet_windows::Runtime::default())
         .manage(AiState::new(Arc::new(MockChatModel)))
         .manage(pet_store::PetStore { pool: db })
         .setup(|app| {
+            let handle=app.handle().clone();
+            let store=app.state::<pet_store::PetStore>().inner().clone();
+            tauri::async_runtime::spawn(async move {
+                if let Ok(Some(pet))=store.active().await {
+                    if let Err(e)=pet_windows::open(&handle,&pet) { eprintln!("載入預設寵物失敗：{e}"); }
+                }
+            });
             /*
              * JS Menu.new() 建立的 native menu item
              * 仍會產生 native menu event。
@@ -154,6 +182,13 @@ pub fn application(db: sqlx::SqlitePool) {
                     let _ = open_control_window(app_handle.clone());
                     return;
                 }
+                if let Some(scoped)=event.id().0.strip_prefix("pet:") {
+                    if let Some((id,action))=scoped.split_once(':') {
+                        let command=match action {"sleep"=>Some(PetCommand::Sleep),"wake"=>Some(PetCommand::Wake),"interact"=>Some(PetCommand::Interact),_=>None};
+                        if let Some(command)=command {let _=pet_windows::dispatch_and_emit(app_handle,id,command);}
+                    }
+                    return;
+                }
                 let command = match event.id().0.as_str() {
                     "interact" => Some(PetCommand::Interact),
                     "sleep" => Some(PetCommand::Sleep),
@@ -191,6 +226,13 @@ pub fn application(db: sqlx::SqlitePool) {
             validate_new_pet_name,
             create_pet,
             list_pets,
+            active_pet,
+            select_pet,
+            pet_windows::saved_pet_profile,
+            pet_windows::open_saved_pet,
+            pet_windows::get_saved_pet_state,
+            pet_windows::send_saved_pet_command,
+            pet_windows::cage_saved_pet,
         ])
         .run(tauri::generate_context!())
         .expect("failed to run Rust AI Desktop Pet");
diff --git a/apps/desktop_pet/src/pet_store.rs b/apps/desktop_pet/src/pet_store.rs
index 7f66eae..64e3bf8 100644
--- a/apps/desktop_pet/src/pet_store.rs
+++ b/apps/desktop_pet/src/pet_store.rs
@@ -11,6 +11,48 @@ pub struct PetStore {
 }
 
 impl PetStore {
+    pub async fn profile(&self, id: &str) -> Result<SavedPet, String> {
+        let row: Option<(String, String, String)> = sqlx::query_as(
+            "SELECT id,name,species FROM pet_profile WHERE id=?1 AND owner_id='local-owner'",
+        )
+        .bind(id)
+        .fetch_optional(&self.pool)
+        .await
+        .map_err(|_| "無法讀取寵物")?;
+        row.map(|(id, name, species)| SavedPet { id, name, species })
+            .ok_or_else(|| "找不到寵物".into())
+    }
+
+    pub async fn active(&self) -> Result<Option<SavedPet>, String> {
+        let row: Option<(String,String,String)> = sqlx::query_as(
+            "SELECT p.id,p.name,p.species FROM local_pet_selection s JOIN pet_profile p ON p.id=s.pet_id WHERE s.slot=1 AND p.owner_id='local-owner'")
+            .fetch_optional(&self.pool).await.map_err(|_| "無法讀取生活窗寵物".to_owned())?;
+        Ok(row.map(|(id, name, species)| SavedPet { id, name, species }))
+    }
+    pub async fn select(&self, id: &str) -> Result<SavedPet, String> {
+        let mut tx = self
+            .pool
+            .begin()
+            .await
+            .map_err(|_| "無法開始選取".to_owned())?;
+        let row: (String, String, String) = sqlx::query_as(
+            "SELECT id,name,species FROM pet_profile WHERE id=?1 AND owner_id='local-owner'",
+        )
+        .bind(id)
+        .fetch_optional(&mut *tx)
+        .await
+        .map_err(|_| "無法查詢寵物".to_owned())?
+        .ok_or_else(|| "找不到可選的寵物".to_owned())?;
+        sqlx::query("INSERT INTO local_pet_selection(slot,pet_id) VALUES(1,?1) ON CONFLICT(slot) DO UPDATE SET pet_id=excluded.pet_id")
+            .bind(id).execute(&mut *tx).await.map_err(|_| "無法儲存選取".to_owned())?;
+        tx.commit().await.map_err(|_| "選取提交失敗".to_owned())?;
+        Ok(SavedPet {
+            id: row.0,
+            name: row.1,
+            species: row.2,
+        })
+    }
+
     pub async fn list(&self) -> Result<Vec<SavedPet>, String> {
         let rows: Vec<(String, String, String)> = sqlx::query_as(
             "SELECT id,name,species FROM pet_profile WHERE owner_id='local-owner' ORDER BY name,id",
diff --git a/apps/desktop_pet/src/pet_windows.rs b/apps/desktop_pet/src/pet_windows.rs
new file mode 100644
index 0000000..3b09131
--- /dev/null
+++ b/apps/desktop_pet/src/pet_windows.rs
@@ -0,0 +1,146 @@
+use crate::pet_store::{PetStore, SavedPet};
+use pet_domain::{PetCommand, PetMachine, PetSnapshot};
+use std::{collections::HashMap, sync::Mutex};
+use tauri::{Emitter, Manager, State};
+
+#[derive(Default)]
+pub struct Runtime {
+    machines: Mutex<HashMap<String, PetMachine>>,
+}
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
+        if let Ok(mut pets) = self.machines.lock() {
+            pets.remove(id);
+        }
+    }
+}
+fn label(id: &str) -> Result<String, String> {
+    let id = uuid::Uuid::parse_str(id).map_err(|_| "寵物ID格式錯誤")?;
+    Ok(format!("pet-{}", id.simple()))
+}
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
+    Ok(())
+}
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
+#[tauri::command]
+pub async fn saved_pet_profile(id: String, store: State<'_, PetStore>) -> Result<SavedPet, String> {
+    store.profile(&id).await
+}
+#[tauri::command]
+pub async fn open_saved_pet(
+    id: String,
+    app: tauri::AppHandle,
+    store: State<'_, PetStore>,
+) -> Result<(), String> {
+    let pet = store.profile(&id).await?;
+    open(&app, &pet)
+}
+#[tauri::command]
+pub fn get_saved_pet_state(id: String, runtime: State<'_, Runtime>) -> Result<PetSnapshot, String> {
+    runtime.snapshot(&id)
+}
+#[tauri::command]
+pub fn send_saved_pet_command(
+    id: String,
+    command: PetCommand,
+    app: tauri::AppHandle,
+) -> Result<PetSnapshot, String> {
+    dispatch_and_emit(&app, &id, command)
+}
+#[tauri::command]
+pub fn cage_saved_pet(id: String, app: tauri::AppHandle) -> Result<(), String> {
+    if let Some(window) = app.get_webview_window(&label(&id)?) {
+        window.close().map_err(|e| e.to_string())?;
+    }
+    app.state::<Runtime>().remove(&id);
+    Ok(())
+}
+#[cfg(test)]
+mod tests {
+    use super::*;
+    #[test]
+    fn day24_more_than_two_pet_states_are_independent_and_caging_is_local() {
+        let runtime = Runtime::default();
+        for id in ["one", "two", "three"] {
+            runtime
+                .machines
+                .lock()
+                .unwrap()
+                .insert(id.into(), PetMachine::default());
+        }
+        let before_two = runtime.snapshot("two").unwrap();
+        let before_three = runtime.snapshot("three").unwrap();
+        runtime.dispatch("one", PetCommand::Sleep).unwrap();
+        assert_eq!(runtime.snapshot("two").unwrap(), before_two);
+        assert_eq!(runtime.snapshot("three").unwrap(), before_three);
+        runtime.remove("one");
+        assert!(runtime.snapshot("one").is_err());
+        assert!(runtime.snapshot("two").is_ok());
+        assert!(runtime.snapshot("three").is_ok());
+    }
+    #[test]
+    fn day24_window_labels_are_derived_from_each_uuid_not_a_two_window_limit() {
+        let labels: std::collections::HashSet<_> = (0..12)
+            .map(|_| label(&uuid::Uuid::new_v4().to_string()).unwrap())
+            .collect();
+        assert_eq!(labels.len(), 12);
+        assert!(label("../invalid").is_err());
+    }
+}
diff --git a/apps/desktop_pet/tauri.conf.json b/apps/desktop_pet/tauri.conf.json
index 35798fa..e7b14e6 100644
--- a/apps/desktop_pet/tauri.conf.json
+++ b/apps/desktop_pet/tauri.conf.json
@@ -3,7 +3,7 @@
   "productName": "Rust AI Desktop Pet",
   "version": "0.2.16",
   "identifier": "dev.rustai.desktop-pet",
-  "build": { 
+  "build": {
     "devUrl": "http://localhost:1420",
     "frontendDist": "frontend/dist",
     "beforeDevCommand": {
@@ -26,22 +26,10 @@
         "minWidth": 360,
         "minHeight": 420,
         "resizable": true
-      },
-      {
-        "label": "pet",
-        "title": "Pet 生活",
-        "url": "index.html?view=pet",
-        "width": 200,
-        "height": 200,
-        "resizable": false,
-        "decorations": false,
-        "transparent": true,
-        "alwaysOnTop": true,
-        "shadow": false
       }
     ]
   },
-  "bundle": { 
-    "active": false 
+  "bundle": {
+    "active": false
   }
 }
diff --git a/apps/desktop_pet/tests/day24_active_pet.rs b/apps/desktop_pet/tests/day24_active_pet.rs
new file mode 100644
index 0000000..ce723b4
--- /dev/null
+++ b/apps/desktop_pet/tests/day24_active_pet.rs
@@ -0,0 +1,38 @@
+use desktop_pet::pet_store::PetStore;
+use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
+#[tokio::test]
+async fn day24_explicit_selection_survives_reopen_and_rejects_unknown_id() {
+    let path = std::env::temp_dir().join(format!("day24-pet-{}.sqlite", uuid::Uuid::new_v4()));
+    let options = SqliteConnectOptions::new()
+        .filename(&path)
+        .create_if_missing(true)
+        .foreign_keys(true);
+    let pool = SqlitePoolOptions::new()
+        .max_connections(1)
+        .connect_with(options.clone())
+        .await
+        .unwrap();
+    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
+    let store = PetStore { pool: pool.clone() };
+    assert!(store.active().await.unwrap().is_none());
+    store.create("小橘").await.unwrap();
+    store.create("小綠").await.unwrap();
+    assert!(store.active().await.unwrap().is_none());
+    let chosen = store.list().await.unwrap().remove(1);
+    assert_eq!(store.select(&chosen.id).await.unwrap(), chosen);
+    assert!(store.select("missing-id").await.is_err());
+    assert_eq!(store.active().await.unwrap(), Some(chosen.clone()));
+    pool.close().await;
+    let pool = SqlitePoolOptions::new()
+        .max_connections(1)
+        .connect_with(options)
+        .await
+        .unwrap();
+    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
+    assert_eq!(
+        (PetStore { pool: pool.clone() }).active().await.unwrap(),
+        Some(chosen)
+    );
+    pool.close().await;
+    std::fs::remove_file(path).unwrap();
+}
```

### Refactor：以 ID 作為視窗、狀態和事件的共同邊界

`Runtime` 用 HashMap 保存各自 `PetMachine`。開窗在鎖內序列化，同 ID 已有視窗便聚焦，不重設既有狀態；build失敗會移除剛加入的狀態。profile 只查本機主人、只回 ID／name／species，不回 AI key。UUID 先解析再形成安全 label／URL，不接受路徑片段。視窗並非直接由未驗證前端任意建立。

原生狀態事件是 ID＋snapshot；生活窗 URL 的 ID 在整個生命週期固定，不因控制台換選取就變成另一隻。控制台的互動目標以明確保存選取為準；選取變更重置本地 snapshot，避免不同寵物 revision 比較互相覆蓋。右鍵睡覺／醒來／互動使用 `pet:ID:action`；不發送舊共用指令。capability 對象改為 `main` 與 `pet-*`，不是只改 builder 卻讓動態窗沒有 IPC／事件權限。

原生直接關窗與回籠都只移除該隻 Runtime；不刪 profile，不取消啟動預設。重啟目前只自動出籠一隻明確預設，其他資料仍可從清單各自出籠。這個預設數量限制不同於同時出籠數量，後者沒有兩隻硬上限；多預設與逐寵物漫遊仍是未完成需求。原生 GUI 失敗紀錄中有一次測試腳本誤把控制窗所有 disabled 按鈕當作未完成，修正為檢查建立按鈕後重跑成功，沒有把 harness 錯誤當成產品 Red。

### 舊 Day 更正補充

不回寫 Day23、Day22；本次只修尚未封存的 Day24，保留 r1 原文與證據。makeup_queue 已完成項目不重開，不因 r2 再推 Day。r1 單窗限制如實留下歷史，本篇以 r2 為目前可套用版本。

## 3. 套件版本與平台界線

原 manifest／lock 不變。本日 SQLx 0.9.0、Tauri 2.12.4、Leptos 0.8.22、Serde 1.0.229、Tokio 1.54.0、UUID 1.28.0、wasm-bindgen 0.2.129 與本次官方 registry 查得 stable 一致。[官方 Tauri 套件資料](https://crates.io/api/v1/crates/tauri)；完整 lock／查詢對照在附件。沒有僅憑最新版而升級產品依賴。

本機 Rust/Cargo 1.99.1、Linux native／wasm32-unknown-unknown。Trunk 真正產生 Wasm 與 JS，native Tauri binary 編譯成功。最初離線 Trunk 因找不到 wasm-bindgen 失敗，正常模式成功，原始兩次 log 保留。最低 MSRV、Windows、macOS、安裝包與簽章未測；桌面空間與位置演算法尚未涵蓋所有螢幕配置。

## 4. 驗證項目、證據與未完成

- `cargo test --locked -p desktop-pet`：46 項通過、0 失敗；含三隻獨立狀態、移除一隻不影響另外兩隻、十二個不同 UUID label、選取持久／未知 ID／重新連線。
- r1 原生 GUI 行為 Red：三筆資料預期四窗，實際兩窗；原始 assertion log 保留。
- `cargo build --locked -p desktop-pet`、`trunk build --locked`：r2 通過，真實 Rust／WASM／JS 接線已編譯。
- r2 原生 Linux Xvfb＋WebKitWebDriver＋tauri-driver：控制窗建立三隻→各自出籠→同時三個不同 ID／名字／烏龜的200×200生活窗，位置分別 x=80、300、520，y=80；各窗按鈕在可見範圍。
- 重複出籠第一隻仍四窗，只聚焦；第一隻 sleep、另外兩隻仍 idle；實際按第一隻「回籠子」後只少一窗、另外兩隻仍 idle，三筆 SQLite profile 都保留；重新出籠可用。
- 關閉整個程序重啟：明確預設的同 ID 自動回到自己的生活窗；其餘兩筆仍在 DB，沒有把所有資料都誤當預設，也沒有殘留匿名生活窗。
- `git apply --cached --check candidate.patch`：精確來源4f0cd00通過。publication gate 13 項測試通過，含完整文章、截斷／literal code／索引拒絕、條件式冪等推進、修訂不得加 Day 與衝突拒絕。

精確來源4f0cd00的 CI 查詢快照如下，未包含本日隔離 patch；查詢時尚在排隊／執行，未把 Pending 當成功，另有本地真 GUI 結果：

- [Rust AI Chat](https://github.com/williechen/rust_ai_chat/actions/runs/39511377716)：queued/None；2026-10-20T22:54:04Z；jobs check-chat=None, build-chat=None
- [Persistence](https://github.com/williechen/rust_ai_chat/actions/runs/39511377839)：queued/None；2026-10-20T22:54:04Z；jobs test-persistence=None
- [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39511377875)：queued/None；2026-10-20T22:54:04Z；jobs test-chat-application=None, test-server=None, test-pet-domain=None, test-ui=None, test-shared=None, test-stats-server=None, test-ai-core=None, test-desktop-pet=None, test-chat-domain=None
- [Node](https://github.com/williechen/rust_ai_chat/actions/runs/39511377669)：completed/success；2026-10-20T22:54:14Z；jobs test-node=success
- [Rust Dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39511377730)：in_progress/None；2026-10-20T22:54:14Z；jobs build-dashboard=None, check-dashboard=None
- [Rust Desktop Pet](https://github.com/williechen/rust_ai_chat/actions/runs/39511377861)：in_progress/None；2026-10-20T22:54:07Z；jobs build-pet=None, check-pet=None

[完整 r2 patch、原生腳本、三窗截圖、逐 ID／rect 結果與原始測試 log](https://github.com/williechen/copilot_use/tree/teaching/rust-ai-chat-alignment/docs/teaching/day24/pet-validation)；[r1 原文與證據](https://github.com/williechen/copilot_use/tree/teaching/rust-ai-chat-alignment/docs/teaching/day24/revisions/pet-r1)。測試 DB 使用隔離路徑，完成後刪除本次 SQLite，不碰 copilot_use 資料。

本篇完整寫入同一正式頁後，四節全文與 literal code／來源／索引全部讀回，才條件式替換 Pet 的 Day24 r1→r2 交付證據；預期原body hash與state blob都要一致。Pet next_day固定25，不再加到26；其他案、makeup_queue、last_full_comparison保留。尚未完成多預設、持久每隻FSM、漫遊與跨平台，不能宣稱完整產品驗收。此次只做教學交付，正式排程未恢復，也未建立重複任務。
