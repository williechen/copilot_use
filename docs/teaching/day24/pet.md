## 1. 學習大綱：把儲存身分接到真正生活窗

本日一個最小切片是「控制台明確選取已儲存寵物→SQLite 保存選取→生活窗立即顯示名字與初始烏龜外觀→程序重啟載入同一身分」。來源 `rust_ai_chat/main` 為 `4f0cd00b8d571bdaf120298efebf0d62b795498f`，workspace 0.2.23。原始碼雖可建立與列出三欄 `SavedPet`，生活窗仍只使用 `PetMachine::default()`，沒有已儲存 ID／名稱；動畫還是貓咪 emoji，與資料中的 tortoise 不一致。本篇不再優先加漫遊細節。

真實呼叫鏈：Leptos 清單按鈕→Rust bridge→`window.selectPet`→Tauri `select_pet`→同一 `PetStore` pool 交易寫入新增的 `local_pet_selection`→提交後 emit `pet-profile-changed`→生活窗顯示名字；啟動時 `active_pet` join 回本機寵物三欄。原生視窗標籤確定是 `main` 控制窗、`pet` 生活窗（URL `index.html?view=pet`），沒有假設不存在的多寵物建窗 API。

本篇只支援既有單生活窗的一隻明確選取寵物；建立不自動指定第一筆、尚未選取就顯示引導，不把預設 FSM 當作新寵物。ID 放在名字的 title 屬性，外觀依現有唯一建立物種 tortoise 提供烏龜 emoji；不是自訂素材、逐物種素材引擎或多隻獨立狀態。FSM 與漫遊仍是既有單一模型，切換身分不重設成多份；這些未完成處須保留。

規則 daily-tdd-minimum-v1；共同 Notion 更新 `2026-10-20T16:32:31.684Z`，規則 blob `ebebad6b3d671b7cdfa48a5f121a9958010b15b4`，教學初始基準 `f3b32b6a27d06db72e40f7d92a1530f52d549342`。Chat Day24 已完整交付，依序才處理 Pet。Day23（含）以前不改。主體 Red／Green 估 15 分鐘，Refactor／migration 閱讀 8 分鐘，版本與驗證 8 分鐘另列。`copilot_use` 產品的 `DesktopState/PetItem`、registry 與 `pet_runtime` 契約不同，本篇不冒稱產品整合。

前置是 `PET_DATABASE_URL` 指向可遷移的 SQLite。新增 migration 僅建選取表，保留所有舊 SQL/checksum；遇舊庫 checksum 不符就停下人工核對，不刪舊庫、不修 checksum。任何 DB、IPC 讀取失敗顯示錯誤，不任意選寵物。

發布前 main 由 c965681 更新到 4f0cd00，差異只有 root 的 AI migration；Pet 的所有程式、manifest／lock 與 migration 都未改。已在更新後隔離基準重新執行 44 項測試、WASM check、native build 與真雙視窗／程序重啟驗證。Red log 仍保留 c965681 的原始缺符號失敗，經差異核對對 4f0cd00 仍適用。Chat 已完成發布的歷史來源不回寫。

## 2. 實作內容：Red → 完整最小 Green → Refactor

### Red：先要求明確選取能重開資料庫

新增 `apps/desktop_pet/tests/day24_active_pet.rs` 後，對未修改 store 的來源執行 `cargo test --locked -p desktop-pet --test day24_active_pet`。實際 E0599：缺少 `active`／`select`，屬預期新增符號 Red，不是環境或匯入錯誤。測試不用第一筆當預設，建立兩筆後仍未選取；明確選第二筆，未知 ID 不可覆寫舊選取，關 pool 重開並再跑 migration，仍是同一完整身分。關閉 pool 後刪除本測試自己的暫存 SQLite。

```rs
use desktop_pet::pet_store::PetStore;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
#[tokio::test]
async fn day24_explicit_selection_survives_reopen_and_rejects_unknown_id() {
    let path = std::env::temp_dir().join(format!("day24-pet-{}.sqlite", uuid::Uuid::new_v4()));
    let options = SqliteConnectOptions::new()
        .filename(&path)
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    let store = PetStore { pool: pool.clone() };
    assert!(store.active().await.unwrap().is_none());
    store.create("小橘").await.unwrap();
    store.create("小綠").await.unwrap();
    assert!(store.active().await.unwrap().is_none());
    let chosen = store.list().await.unwrap().remove(1);
    assert_eq!(store.select(&chosen.id).await.unwrap(), chosen);
    assert!(store.select("missing-id").await.is_err());
    assert_eq!(store.active().await.unwrap(), Some(chosen.clone()));
    pool.close().await;
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    assert_eq!(
        (PetStore { pool: pool.clone() }).active().await.unwrap(),
        Some(chosen)
    );
    pool.close().await;
    std::fs::remove_file(path).unwrap();
}
```

### Green：完整可套用修改

以下完整 patch 包含 additive migration、store、Tauri commands 與註冊、JS、Rust bridge、Leptos 清單／生活窗與初始外觀、測試。若先手動加上 Red 檔案，套其餘修改即可；或在乾淨隔離副本一次套全部，不能重複新增檔案。完整原始 patch 在第4節附件，精確基準 apply check 通過。無新增依賴或修改 lock。

選取表主鍵 slot 限制為 1，外鍵連到現有 profile，刪除該 profile 才 cascade 刪選取；不刪寵物資料。查詢限制 `local-owner`，只回 `id/name/species`，不把 AI key 或 owner 資料傳前端。這是單機身分篩選，不是遠端多使用者授權模型。

```diff
diff --git a/apps/desktop_pet/frontend/index.html b/apps/desktop_pet/frontend/index.html
index 7242117..05072ca 100644
--- a/apps/desktop_pet/frontend/index.html
+++ b/apps/desktop_pet/frontend/index.html
@@ -104,6 +104,11 @@
       return invoke("scan_duplicate_preview");
     };
 
+    window.activePet = () => invoke("active_pet");
+    window.selectPet = (id) => invoke("select_pet", { id });
+    window.listenPetProfile = (callback) => window.__TAURI__.event.listen(
+      "pet-profile-changed", (event) => callback(event.payload));
+
     window.listPets = () => {
       return invoke("list_pets");
     };
diff --git a/apps/desktop_pet/frontend/src/bridge.rs b/apps/desktop_pet/frontend/src/bridge.rs
index 0a6e104..9693a66 100644
--- a/apps/desktop_pet/frontend/src/bridge.rs
+++ b/apps/desktop_pet/frontend/src/bridge.rs
@@ -5,6 +5,12 @@ use wasm_bindgen_futures::JsFuture;
 
 #[wasm_bindgen]
 extern "C" {
+    #[wasm_bindgen(js_namespace=window, js_name=activePet)]
+    fn active_pet_js() -> Promise;
+    #[wasm_bindgen(js_namespace=window, js_name=selectPet)]
+    fn select_pet_js(id: &str) -> Promise;
+    #[wasm_bindgen(js_namespace=window, js_name=listenPetProfile)]
+    fn listen_pet_profile_js(callback: &Function) -> Promise;
 
     #[wasm_bindgen(
         js_namespace=window,
@@ -184,3 +190,26 @@ pub async fn list_pets() -> Result<Vec<SavedPet>, String> {
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
index 080f06b..63c1070 100644
--- a/apps/desktop_pet/frontend/src/main.rs
+++ b/apps/desktop_pet/frontend/src/main.rs
@@ -54,6 +54,25 @@ fn App() -> impl IntoView {
     let (saved_pets, set_saved_pets) = signal(Vec::<bridge::SavedPet>::new());
     let (pets_busy, set_pets_busy) = signal(false);
 
+    let (active_pet, set_active_pet) = signal::<Option<bridge::SavedPet>>(None);
+    let profile_events = RwSignal::new(0_u64);
+    spawn_local(async move {
+        if let Err(message) = bridge::listen_pet_profile(move |pet| {
+            profile_events.update(|n| *n += 1);
+            set_active_pet.set(Some(pet));
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
@@ -106,7 +125,13 @@ fn App() -> impl IntoView {
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
+                <p role="alert">{move || error.get().unwrap_or_default()}</p>
                 <button
                     type="button"
                     on:click=move |_| spawn_local(async move{
@@ -173,7 +198,23 @@ fn App() -> impl IntoView {
                 <Show when=move || pets_busy.get() fallback=|| ()><p>"處理中…"</p></Show>
                 <ul>
                     <For each=move || saved_pets.get() key=|pet| pet.id.clone()
-                        children=|pet| view! { <li>{format!("{}／{}（{}）", pet.name, pet.species, pet.id)}</li> } />
+                        children=move |pet| {
+                            let id=pet.id.clone();
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
+                            </li> }
+                        } />
                 </ul>
             </section>
 
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
index ad65ded..6feafdb 100644
--- a/apps/desktop_pet/src/lib.rs
+++ b/apps/desktop_pet/src/lib.rs
@@ -128,6 +128,17 @@ async fn list_pets(
     store.list().await
 }
 
+#[tauri::command]
+async fn active_pet(store: State<'_, pet_store::PetStore>) -> Result<Option<pet_store::SavedPet>, String> {
+    store.active().await
+}
+#[tauri::command]
+async fn select_pet(id: String, app: tauri::AppHandle, store: State<'_, pet_store::PetStore>) -> Result<pet_store::SavedPet, String> {
+    let saved=store.select(&id).await?;
+    app.emit("pet-profile-changed", &saved).map_err(|_| "選取已儲存，通知失敗，請重新載入生活窗".to_owned())?;
+    Ok(saved)
+}
+
 pub fn application(db: sqlx::SqlitePool) {
     tauri::Builder::default()
         .on_window_event(|window, event| {
@@ -191,6 +202,8 @@ pub fn application(db: sqlx::SqlitePool) {
             validate_new_pet_name,
             create_pet,
             list_pets,
+            active_pet,
+            select_pet,
         ])
         .run(tauri::generate_context!())
         .expect("failed to run Rust AI Desktop Pet");
diff --git a/apps/desktop_pet/src/pet_store.rs b/apps/desktop_pet/src/pet_store.rs
index 7f66eae..c9955eb 100644
--- a/apps/desktop_pet/src/pet_store.rs
+++ b/apps/desktop_pet/src/pet_store.rs
@@ -11,6 +11,36 @@ pub struct PetStore {
 }
 
 impl PetStore {
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

### Refactor：先提交，再通知；啟動讀取不蓋過新事件

`select` 於同交易確認 ID 與本機主人並寫設定；查無資料直接 rollback。提交後才通知其他視窗，通知失敗明示「已儲存」，不把 DB 已提交冒稱完全失敗；重啟仍能讀取持久選取。前端先註冊 listener，再讀啟動值，事件計數避免慢啟動結果覆蓋較新選取。按鈕處理中停用；錯誤保留原身分，不顯示假成功。listener 隨整個 App 視窗生存，沿用現有 bridge 的 closure lifetime 模式。

### 舊 Day 更正補充

本篇未修改封存 Day23；先前 Day22 補教已結案，不重複新增。最新 main 的建立物種本來是 tortoise，因此本日生活窗接線一併把貓咪 emoji 改為烏龜狀態，這是新切片所需的實際畫面差異，不宣稱所有素材／個性／多寵物需求已完成。

## 3. 套件版本與平台界線

保留原始 manifest／lock；本日直接涉及 SQLx 0.9.0、Tauri 2.12.4、Leptos 0.8.22、Serde 1.0.229、Tokio 1.54.0、UUID 1.28.0、wasm-bindgen 0.2.129，本次官方 registry 查得 stable 與這些 lock 相同。查詢原始資料與本日 lock 對照在附件，[官方 Tauri 套件資料](https://crates.io/api/v1/crates/tauri)。不因最新版本而更新來源依賴。

實測 Rust/Cargo 1.99.1、Linux native、wasm32-unknown-unknown。`cargo check` WASM 與 Trunk 真建置通過；最初離線 Trunk 找不到 wasm-bindgen 執行檔而失敗，改正常下載模式後成功，保留兩次 log。不是 Rust 型別錯誤。Windows、macOS、最低 MSRV、正式安裝包與簽章未測。

## 4. 驗證項目、證據與未完成

- Red `cargo test --locked -p desktop-pet --test day24_active_pet`：實際 E0599，缺少本日方法。
- Green `cargo test --locked -p desktop-pet`：44 項通過、0 失敗，含新增選取／重開／未知 ID 測試及全部 Pet 現有測試。
- `cargo check --locked -p desktop-pet-frontend --target wasm32-unknown-unknown`：通過，真實 UI/bridge 型別與 Wasm import 編譯成功。
- `trunk build --locked`：成功；離線版本下載失敗另記，未冒稱離線可用。
- `cargo build --locked -p desktop-pet`：通過。
- 原生 Linux Xvfb + WebKitWebDriver／tauri-driver：真控制窗建立寵物→SQLite 查到同一 ID→按「選為生活窗寵物」→生活窗即時顯示名字、title ID 與烏龜→關掉整個程序重開→同生活窗載入同一身分，五項流程斷言全部通過。使用隔離測試 DB，無付費 AI；腳本與截圖在附件。
- 原始 patch 對精確來源基準 `git apply --cached --check` 通過。三份原有 migration 保留；新 migration 初次與再次執行都通過。真實歷史 DB checksum 升級、跨平台、多生活窗、每隻 FSM／漫遊及自訂外觀仍未驗收。

精確來源 4f0cd00 的 CI（未包含本日隔離 patch）如下；查詢時尚在排隊／執行，未把 Pending 當成成功。先前 c965681 的 CI 結果保留附件，不冒充新 SHA 的結果。

- [Rust AI Chat](https://github.com/williechen/rust_ai_chat/actions/runs/39511377716)：queued/None；2026-10-20T22:54:04Z；jobs check-chat=None, build-chat=None
- [Persistence](https://github.com/williechen/rust_ai_chat/actions/runs/39511377839)：queued/None；2026-10-20T22:54:04Z；jobs test-persistence=None
- [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39511377875)：queued/None；2026-10-20T22:54:04Z；jobs test-chat-application=None, test-server=None, test-pet-domain=None, test-ui=None, test-shared=None, test-stats-server=None, test-ai-core=None, test-desktop-pet=None, test-chat-domain=None
- [Node](https://github.com/williechen/rust_ai_chat/actions/runs/39511377669)：completed/success；2026-10-20T22:54:14Z；jobs test-node=success
- [Rust Dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39511377730)：in_progress/None；2026-10-20T22:54:14Z；jobs build-dashboard=None, check-dashboard=None
- [Rust Desktop Pet](https://github.com/williechen/rust_ai_chat/actions/runs/39511377861)：in_progress/None；2026-10-20T22:54:07Z；jobs build-pet=None, check-pet=None

[完整修改、GUI 腳本與截圖、測試／CI／套件原始證據](https://github.com/williechen/copilot_use/tree/teaching/rust-ai-chat-alignment/docs/teaching/day24/pet-validation)。教學發布、使用者套用、產品整合、完整功能驗收分別記錄；本篇未寫產品原始碼。本次測試 SQLite 在驗證後清除，保留程式與 log，不碰 copilot_use 資料。

本篇完整四節、完整程式與索引實際寫入並全文讀回，literal code 一致，才條件式更新 Pet next_day 24→25；保留 Chat、Analytics、makeup_queue 與 last_full_comparison。短測試內容寫入成功不足以推進；同證據重試不重複遞增，衝突先重讀，不強制覆寫。正式排程本次不恢復、不新增重複任務。
