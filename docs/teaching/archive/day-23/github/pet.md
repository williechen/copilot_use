發布版 r1。來源 `rust_ai_chat/main f38360cd537daf6dc20e1d3868221bb56390cc7c`；教學基底 `5e9b343e7a220b7d57d1820761a935edd9f313b9`。規則 daily-tdd-minimum-v1，Notion共同頁讀取時最後修改2026-10-20T14:50:15.165Z。Day22（含）以前封存不改；新程式只在隔離副本驗證，由使用者另行套用，不修改產品原始碼。

## 1. 學習大綱／現況

目前控制台可呼叫 `create_pet`，由既有 `PetStore.create` 寫入 SQLite，但關閉再開控制台不會讀回名稱／ID；生活窗仍用 `PetMachine::default()`。本日先接通使用者可以操作的「啟動載入已儲存清單→建立後更新清單→手動重新載入」切片，而非只新增查詢helper。主體Red／Green估15分鐘；Refactor／Day22補教5分鐘，版本與證據閱讀另估8分鐘。

實際資料流：Leptos控制窗→Rust bridge→`window.listPets`→Tauri `list_pets`→同一個既有SQLite pool→`pet_profile`→三欄JSON→清單。建立仍沿用原交易與回應，成功後再讀清單；讀回失敗明示已建立，提示重新載入，避免直接重送建立。操作中停用按鈕。

清單只查本機 `local-owner` 並回傳 `id/name/species`，不回傳 `ai_api_key`；物種沿用既有 `tortoise`，不捏造新外觀。這不是多使用者授權模型。`copilot_use` 產品自己的 `apps/desktop_pet/src/main.rs::list_pets` 使用 `DesktopState/PetItem`、`crates/pet_runtime` 與registry契約；本篇不能直接視為產品整合。

前置：`PET_DATABASE_URL` 指向已建立、可遷移的測試SQLite檔。沿用既有啟動pool，不開第二個路徑的DB、不清空舊庫。歷史checksum不符即停下人工核對，不修checksum或刪庫。

## 2. 實作內容：Red → 完整最小 Green → Refactor

### Red：先加入持久清單的測試

先新增 `apps/desktop_pet/tests/day23_saved_pets.rs` 以下測試，再執行 `cargo test --locked -p desktop-pet --test day23_saved_pets`。實際得到E0599：`PetStore` 尚無 `list` 方法；這是預期缺符號Red，不是行為失敗或環境錯誤。檔案SQLite用獨立UUID路徑；關閉pool後重新連線讀回同一筆ID、名稱與物種，失敗的建立不得增加資料。

```rust
use desktop_pet::pet_store::PetStore;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
#[tokio::test]
async fn day23_control_list_survives_pool_reopen() {
    let path = std::env::temp_dir().join(format!("day23-pet-{}.sqlite", uuid::Uuid::new_v4()));
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
    assert!(store.list().await.unwrap().is_empty());
    store.create(" 小橘 ").await.unwrap();
    let before = store.list().await.unwrap();
    assert_eq!(before.len(), 1);
    assert_eq!(before[0].name, "小橘");
    assert_eq!(before[0].species, "tortoise");
    assert!(uuid::Uuid::parse_str(&before[0].id).is_ok());
    let encoded = serde_json::to_value(&before[0]).unwrap();
    assert_eq!(encoded.as_object().unwrap().len(), 3); // 不回傳主人或AI密鑰。
    assert!(store.create("  ").await.is_err());
    assert_eq!(store.list().await.unwrap(), before);
    pool.close().await;
    let reopened = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    let store = PetStore {
        pool: reopened.clone(),
    };
    assert_eq!(store.list().await.unwrap(), before);
    sqlx::migrate!("./migrations").run(&reopened).await.unwrap();
    assert_eq!(store.list().await.unwrap(), before);
    reopened.close().await;
    std::fs::remove_file(path).unwrap();
}
```

### Green：完整可套用修改

以下patch以精確main SHA產生，原始下載patch的 `git apply --check`已通過（保留來源換行）；Notion內diff用於閱讀，套用請使用下方驗證資料連結的原始patch。若已手動新增上方Red檔案，先保留其內容並只套其餘修改，或在乾淨隔離副本一次套完整patch；不要重複新增同一測試。patch完整包含store、Tauri command與註冊、JS、Rust bridge、Leptos控制窗、兩個測試與舊SQL唯讀fixture。沒有新增依賴、改Cargo.lock或改任何正式migration。

```diff
diff --git a/apps/desktop_pet/src/pet_store.rs b/apps/desktop_pet/src/pet_store.rs
index 52bd56b..7f66eae 100644
--- a/apps/desktop_pet/src/pet_store.rs
+++ b/apps/desktop_pet/src/pet_store.rs
@@ -1,9 +1,29 @@
+#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
+pub struct SavedPet {
+    pub id: String,
+    pub name: String,
+    pub species: String,
+}
+
 #[derive(Clone)]
 pub struct PetStore {
     pub pool: sqlx::SqlitePool,
 }
 
 impl PetStore {
+    pub async fn list(&self) -> Result<Vec<SavedPet>, String> {
+        let rows: Vec<(String, String, String)> = sqlx::query_as(
+            "SELECT id,name,species FROM pet_profile WHERE owner_id='local-owner' ORDER BY name,id",
+        )
+        .fetch_all(&self.pool)
+        .await
+        .map_err(|_| "無法讀取已儲存寵物".to_owned())?;
+        Ok(rows
+            .into_iter()
+            .map(|(id, name, species)| SavedPet { id, name, species })
+            .collect())
+    }
+
     pub async fn create(&self, input: &str) -> Result<String, String> {
         let name = crate::normalize_new_pet_name(input)?;
         let id = uuid::Uuid::new_v4().to_string();
diff --git a/apps/desktop_pet/src/lib.rs b/apps/desktop_pet/src/lib.rs
index 72a6c4f..6bc11d7 100644
--- a/apps/desktop_pet/src/lib.rs
+++ b/apps/desktop_pet/src/lib.rs
@@ -121,6 +121,11 @@ async fn create_pet(name: String, store: State<'_, pet_store::PetStore>) -> Resu
     store.create(&name).await
 }
 
+#[tauri::command]
+async fn list_pets(store: State<'_, pet_store::PetStore>) -> Result<Vec<pet_store::SavedPet>, String> {
+    store.list().await
+}
+
 pub fn application(db: sqlx::SqlitePool) {
     tauri::Builder::default()
         .on_window_event(|window, event| {
@@ -183,6 +188,7 @@ pub fn application(db: sqlx::SqlitePool) {
             quit_pet,
             validate_new_pet_name,
             create_pet,
+            list_pets,
         ])
         .run(tauri::generate_context!())
         .expect("failed to run Rust AI Desktop Pet");
diff --git a/apps/desktop_pet/frontend/index.html b/apps/desktop_pet/frontend/index.html
index 0a8d8df..6b300d3 100644
--- a/apps/desktop_pet/frontend/index.html
+++ b/apps/desktop_pet/frontend/index.html
@@ -104,7 +104,9 @@
       return invoke("scan_duplicate_preview");
     };
 
-    window.validateNewPetName = (name) => {
+    window.listPets = () => invoke("list_pets");
+
+    window.createPet = (name) => {
       return invoke("create_pet", { name });
     };
 
diff --git a/apps/desktop_pet/frontend/src/bridge.rs b/apps/desktop_pet/frontend/src/bridge.rs
index 0993949..83e3469 100644
--- a/apps/desktop_pet/frontend/src/bridge.rs
+++ b/apps/desktop_pet/frontend/src/bridge.rs
@@ -63,8 +63,11 @@ extern "C" {
     #[wasm_bindgen(js_namespace = window, js_name = scanDuplicatePreview)]
     fn scan_duplicate_preview_js() -> Promise;
 
-    #[wasm_bindgen(js_namespace = window, js_name = validateNewPetName)]
-    fn validate_new_pet_name_js(name: &str) -> Promise;
+    #[wasm_bindgen(js_namespace = window, js_name = listPets)]
+    fn list_pets_js() -> Promise;
+
+    #[wasm_bindgen(js_namespace = window, js_name = createPet)]
+    fn create_pet_js(name: &str) -> Promise;
 }
 
 // 將 JavaScript 的錯誤轉換為 Rust 的字串
@@ -160,9 +163,21 @@ pub async fn quit_pet() -> Result<(), String> {
     Ok(())
 }
 
-pub async fn validate_new_pet_name(name: String) -> Result<String, String> {
-    let value = JsFuture::from(validate_new_pet_name_js(&name))
+pub async fn create_pet(name: String) -> Result<String, String> {
+    let value = JsFuture::from(create_pet_js(&name))
         .await
         .map_err(js_error)?;
     value.as_string().ok_or_else(|| "名稱回應格式錯誤".into())
 }
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
diff --git a/apps/desktop_pet/frontend/src/main.rs b/apps/desktop_pet/frontend/src/main.rs
index 462606f..e90d5e8 100644
--- a/apps/desktop_pet/frontend/src/main.rs
+++ b/apps/desktop_pet/frontend/src/main.rs
@@ -51,6 +51,22 @@ fn App() -> impl IntoView {
     let (new_pet_name, set_new_pet_name) = signal(String::new());
     let (new_pet_feedback, set_new_pet_feedback) = signal(String::new());
 
+    let (saved_pets, set_saved_pets) = signal(Vec::<bridge::SavedPet>::new());
+    let (pets_busy, set_pets_busy) = signal(false);
+    if !bridge::is_pet_window() {
+        set_pets_busy.set(true);
+        spawn_local(async move {
+            match bridge::list_pets().await {
+                Ok(rows) => {
+                    set_saved_pets.set(rows);
+                    set_new_pet_feedback.set("已載入儲存清單".into());
+                }
+                Err(error) => set_new_pet_feedback.set(error),
+            }
+            set_pets_busy.set(false);
+        });
+    }
+
     {
         let snapshot_reader = snapshot;
         let snapshot_writer = set_snapshot;
@@ -116,19 +132,46 @@ fn App() -> impl IntoView {
                         on:input=move |ev| set_new_pet_name.set(event_target_value(&ev)) />
                 </label>
                 <button
-                    type="button" 
+                    type="button"
+                    disabled=move || pets_busy.get()
                     on:click=move |_| {
+                        if pets_busy.get_untracked() { return; }
+                        set_pets_busy.set(true);
                         let proposed = new_pet_name.get_untracked();
                         spawn_local(async move {
-                            let response = bridge::validate_new_pet_name(proposed).await;
-                            set_new_pet_feedback.set(match response {
-                                Ok(name) => format!("已儲存名稱：{name}"),
-                                Err(error) => error,
-                            });
+                            match bridge::create_pet(proposed).await {
+                                Ok(saved) => match bridge::list_pets().await {
+                                    Ok(rows) => {
+                                        set_saved_pets.set(rows);
+                                        set_new_pet_feedback.set(format!("已儲存：{saved}"));
+                                    }
+                                    Err(_) => set_new_pet_feedback.set(format!("已建立：{saved}；清單讀回失敗，請重新載入，勿重送建立。")),
+                                },
+                                Err(error) => set_new_pet_feedback.set(error),
+                            }
+                            set_pets_busy.set(false);
                         });
                     }
                 >"建立寵物"</button>
                 <p role="status">{move || new_pet_feedback.get()}</p>
+                <h3>"已儲存寵物"</h3>
+                <button type="button" disabled=move || pets_busy.get()
+                    on:click=move |_| {
+                        if pets_busy.get_untracked() { return; }
+                        set_pets_busy.set(true);
+                        spawn_local(async move {
+                            match bridge::list_pets().await {
+                                Ok(rows) => { set_saved_pets.set(rows); set_new_pet_feedback.set("已重新載入清單".into()); }
+                                Err(error) => set_new_pet_feedback.set(error),
+                            }
+                            set_pets_busy.set(false);
+                        });
+                    }>"重新載入清單"</button>
+                <Show when=move || pets_busy.get() fallback=|| ()><p>"處理中…"</p></Show>
+                <ul>
+                    <For each=move || saved_pets.get() key=|pet| pet.id.clone()
+                        children=|pet| view! { <li>{format!("{}／{}（{}）", pet.name, pet.species, pet.id)}</li> } />
+                </ul>
             </section>
 
             <h2>"互動與聊天"</h2>
diff --git a/apps/desktop_pet/tests/day23_saved_pets.rs b/apps/desktop_pet/tests/day23_saved_pets.rs
new file mode 100644
index 0000000..ebfcd5f
--- /dev/null
+++ b/apps/desktop_pet/tests/day23_saved_pets.rs
@@ -0,0 +1,75 @@
+use desktop_pet::pet_store::PetStore;
+use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
+#[tokio::test]
+async fn day23_control_list_survives_pool_reopen() {
+    let path = std::env::temp_dir().join(format!("day23-pet-{}.sqlite", uuid::Uuid::new_v4()));
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
+    assert!(store.list().await.unwrap().is_empty());
+    store.create(" 小橘 ").await.unwrap();
+    let before = store.list().await.unwrap();
+    assert_eq!(before.len(), 1);
+    assert_eq!(before[0].name, "小橘");
+    assert_eq!(before[0].species, "tortoise");
+    assert!(uuid::Uuid::parse_str(&before[0].id).is_ok());
+    let encoded = serde_json::to_value(&before[0]).unwrap();
+    assert_eq!(encoded.as_object().unwrap().len(), 3); // 不回傳主人或AI密鑰。
+    assert!(store.create("  ").await.is_err());
+    assert_eq!(store.list().await.unwrap(), before);
+    pool.close().await;
+    let reopened = SqlitePoolOptions::new()
+        .max_connections(1)
+        .connect_with(options)
+        .await
+        .unwrap();
+    let store = PetStore {
+        pool: reopened.clone(),
+    };
+    assert_eq!(store.list().await.unwrap(), before);
+    sqlx::migrate!("./migrations").run(&reopened).await.unwrap();
+    assert_eq!(store.list().await.unwrap(), before);
+    reopened.close().await;
+    std::fs::remove_file(path).unwrap();
+}
+
+#[tokio::test]
+async fn day23_old_pet_checksum_rejects_without_deleting_saved_pet() {
+    let pool = SqlitePoolOptions::new()
+        .max_connections(1)
+        .connect("sqlite::memory:")
+        .await
+        .unwrap();
+    let old = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pet-day22");
+    sqlx::migrate::Migrator::new(old.as_path())
+        .await
+        .unwrap()
+        .run(&pool)
+        .await
+        .unwrap();
+    sqlx::query("INSERT INTO pet_owner(id,name) VALUES ('local-owner','舊主人')")
+        .execute(&pool)
+        .await
+        .unwrap();
+    sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id) VALUES ('sentinel','舊寵物','tortoise','local-owner')").execute(&pool).await.unwrap();
+    let result = sqlx::migrate!("./migrations").run(&pool).await;
+    assert!(
+        result
+            .unwrap_err()
+            .to_string()
+            .contains("previously applied but has been modified")
+    );
+    let saved: String = sqlx::query_scalar("SELECT name FROM pet_profile WHERE id='sentinel'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(saved, "舊寵物");
+}
diff --git a/apps/desktop_pet/tests/fixtures/pet-day22/20261018095626_create_pet_owner.sql b/apps/desktop_pet/tests/fixtures/pet-day22/20261018095626_create_pet_owner.sql
new file mode 100644
index 0000000..8c72040
--- /dev/null
+++ b/apps/desktop_pet/tests/fixtures/pet-day22/20261018095626_create_pet_owner.sql
@@ -0,0 +1,7 @@
+-- Add migration script here
+CREATE TABLE pet_owner (
+    id TEXT NOT NULL PRIMARY KEY, -- PKNO
+    name VARCHAR(255) NOT NULL, -- 主人名稱
+    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 建立時間
+    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新時間
+);
\ No newline at end of file
diff --git a/apps/desktop_pet/tests/fixtures/pet-day22/20261018091733_create_pet.sql b/apps/desktop_pet/tests/fixtures/pet-day22/20261018091733_create_pet.sql
new file mode 100644
index 0000000..2b96297
--- /dev/null
+++ b/apps/desktop_pet/tests/fixtures/pet-day22/20261018091733_create_pet.sql
@@ -0,0 +1,65 @@
+-- Add migration script here
+CREATE TABLE pet_profile (
+    id TEXT NOT NULL PRIMARY KEY, -- PKNO
+    name VARCHAR(255) NOT NULL, -- 寵物名稱
+    ai_api_key VARCHAR(255), -- AI API 金鑰
+    species VARCHAR(255) NOT NULL, -- 物種
+    age INT, -- 年齡
+    sexy VARCHAR(255), -- 性別
+    image_path VARCHAR(255), -- 圖片路徑
+    is_default_closure BOOLEAN DEFAULT FALSE, -- 是否預設關籠 
+    owner_id TEXT NOT NULL REFERENCES pet_owner(id), -- 主人ID
+    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 建立時間
+    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新時間
+);
+
+CREATE TABLE pet_extra (
+    id TEXT NOT NULL PRIMARY KEY, -- PKNO
+    pet_id TEXT NOT NULL REFERENCES pet_profile(id), -- 寵物ID
+    owner_id TEXT NOT NULL REFERENCES pet_owner(id), -- 主人ID
+    category VARCHAR(255), -- 類別(例如: 興趣、個性、喜好、其他)
+    content VARCHAR(255), -- 內容
+    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 建立時間
+    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新時間
+);
+
+CREATE TABLE pet_activity (
+    id TEXT NOT NULL PRIMARY KEY, -- PKNO
+    pet_id TEXT NOT NULL REFERENCES pet_profile(id), -- 寵物ID
+    owner_id TEXT NOT NULL REFERENCES pet_owner(id), -- 主人ID
+    activity_type VARCHAR(255), -- 活動類型(例如: 散步、玩耍、訓練)
+    is_activity BOOLEAN DEFAULT FALSE, -- 是否進行中
+    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 建立時間
+    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新時間
+);
+
+CREATE TABLE pet_study(
+    id TEXT NOT NULL PRIMARY KEY, -- PKNO
+    pet_id TEXT NOT NULL REFERENCES pet_profile(id), -- 寵物ID
+    owner_id TEXT NOT NULL REFERENCES pet_owner(id), -- 主人ID
+    study_item VARCHAR(255), -- 學習項目(例如: 尋找、刪除、整理)
+    is_study BOOLEAN DEFAULT FALSE, -- 是否進行中
+    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 建立時間
+    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新時間
+);
+
+CREATE TABLE pet_life_log (
+    id TEXT NOT NULL PRIMARY KEY, -- PKNO
+    pet_id TEXT NOT NULL REFERENCES pet_profile(id), -- 寵物ID
+    owner_id TEXT NOT NULL REFERENCES pet_owner(id), -- 主人ID
+    do_item VARCHAR(255), -- 執行項目(例如: 散步、玩耍、訓練)
+    do_title TEXT, -- 標題(例如: 今天有散步，找到球)
+    content TEXT, -- 內容(例如: 今天的散步情況．找到球的過程)
+    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 建立時間
+    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新時間
+);
+
+CREATE TABLE codemap (
+    id TEXT NOT NULL PRIMARY KEY, -- PKNO
+    category VARCHAR(255), -- 類別(例如: 活動類型, 寵物屬性)
+    code VARCHAR(255), -- 代碼(例如: WALK, PLAY, TRAIN)
+    value VARCHAR(255), -- 值(例如: 散步、玩耍、訓練)
+    description VARCHAR(255), -- 描述
+    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 建立時間
+    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新時間
+);
\ No newline at end of file
diff --git a/apps/desktop_pet/tests/fixtures/pet-day22/20261018114025_creat_protocol.sql b/apps/desktop_pet/tests/fixtures/pet-day22/20261018114025_creat_protocol.sql
new file mode 100644
index 0000000..6ebb517
--- /dev/null
+++ b/apps/desktop_pet/tests/fixtures/pet-day22/20261018114025_creat_protocol.sql
@@ -0,0 +1,26 @@
+-- Add migration script here
+CREATE TABLE pet_token_log(
+    id TEXT NOT NULL PRIMARY KEY,
+    token_src TEXT NOT NULL, -- 憑證來源(pet)
+    token_type TEXT NOT NULL,  -- 憑證類型(jwt, sha256, md5)
+    token_context TEXT NOT NULL,  -- 憑證內容
+    
+    ref_token_src TEXT,  -- 參考的憑證來源(analytic)
+    ref_token_type TEXT,  -- 參考的憑證類型(jwt, sha256, md5)
+    ref_token_context TEXT,  -- 參考的憑證內容
+
+    expired_at TIMESTAMP,  -- 憑證過期時間
+    token_status TEXT,  -- 憑證狀態
+    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
+);
+
+CREATE TABLE pet_send_log(
+    id TEXT NOT NULL PRIMARY KEY,
+    local_token TEXT NOT NULL,  -- 本地的憑證
+    remote_token TEXT,  -- 遠端的憑證
+    json_data TEXT NOT NULL,  -- 本地的JSON數據
+    send_status TEXT,  -- 發送狀態
+    send_at TIMESTAMP,  -- 發送時間
+    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
+);
+
```

### Refactor與操作契約

把有建立副作用的 `validateNewPetName/validate_new_pet_name` bridge呼叫同步更名為 `createPet/create_pet`，跨HTML、extern/wrapper與呼叫者一起改。既有純名稱驗證command保留，不拿它當儲存。清單按名稱及ID排序，UI以ID為key，兩隻同名寵物不會被合併。

本日不建立逐寵物生活窗、不將清單任何一筆偷偷指定為生活窗主角；預設出籠、多寵物狀態、初始外觀、重啟自動出籠尚未接通。清單載入失敗顯示錯誤並保留舊清單，不冒稱重新讀回成功；建立成功但後續讀回失敗時請只按重新載入。

### 舊 Day 更正補充（程式／套件差異）：Pet Day22

對應FIFO `pet-day22-postfreeze-20261020`，本篇只加入這一個舊Day單元。main已有 `PetStore/create_pet` 與同pool接線，不照舊文重新新增。上方完整patch已同步修正bridge名稱，並將啟動讀取接回真實控制窗。workspace為0.2.22，Tokio1.54.0／SQLx0.9.0／Tauri2.12.4，不為補教執行cargo update。

上游曾修改 `20261018091733_create_pet.sql`，增加三個AI使用欄位；本日未再修改舊migration。補測用舊main `da5155265bf66ef9d495fd38913ad6cb93e1bac8` 的全套Pet migrations建隔離庫、插入sentinel寵物，再跑最新runner，確實因checksum拒絕且舊寵物保留。fresh最新庫可用不代表正式舊庫安全升級；需備份及讀實際 `_sqlx_migrations` 後另訂前向方案，禁止刪庫、repair或改checksum掩蓋。

## 3. 套件版本

workspace0.2.22／edition2024，rustc／cargo1.99.1。精確lock與[crates.io](https://crates.io)官方穩定版核對：SQLx0.9.0、Tokio1.54.0、Tauri2.12.4、tauri-build2.7.3、Leptos0.8.22、Serde1.0.229、UUID1.28.0、wasm-bindgen0.2.129、serde-wasm-bindgen0.6.5。本日不新增套件、不升級lock；native與wasm32 check已執行，MSRV、Windows／macOS未驗。

## 4. 驗證項目與實際結果

- 原main `cargo test --locked -p desktop-pet`：42 passed、0 failed，涵蓋原有Rust測試，不能當新切片Red。
- Red：同一新測試編譯E0599缺 `list`，紀錄 `red.log`。
- Green `cargo test --locked -p desktop-pet`：44 passed、0 failed，包含兩個新檔案DB／checksum測試。零測試target不計入。
- `cargo check --locked -p desktop-pet-frontend --target wasm32-unknown-unknown`：通過，涵蓋新增UI與bridge的型別／WASM編譯。
- Node執行實際HTML script，mock `window.__TAURI__`：確認 `createPet(name)` 呼叫 `create_pet` 帶正確name，`listPets()` 呼叫 `list_pets`；舊副作用bridge名稱已移除。這是mock IPC接線測試，不是原生GUI。
- 完整patch對精確main archive執行 `git apply --check`：通過。

[完整patch、測試logs、版本與CI證據](https://github.com/williechen/copilot_use/tree/teaching/rust-ai-chat-alignment/docs/teaching/day23/pet-validation)。只在隔離副本實測，不提交rust_ai_chat產品修改。

### 同SHA CI快照

- [Rust Desktop Pet / 39428806744](https://github.com/williechen/rust_ai_chat/actions/runs/39428806744)：success，UTC 2026-10-20T13:26:13Z。
- [Rust AI Chat / 39428806840](https://github.com/williechen/rust_ai_chat/actions/runs/39428806840)：success，UTC 2026-10-20T13:26:52Z。
- [Persistence / 39428806698](https://github.com/williechen/rust_ai_chat/actions/runs/39428806698)：failure，UTC 2026-10-20T13:18:09Z。
- [Test All / 39428806748](https://github.com/williechen/rust_ai_chat/actions/runs/39428806748)：failure，UTC 2026-10-20T13:19:11Z。
- [Rust Dashboard / 39428806718](https://github.com/williechen/rust_ai_chat/actions/runs/39428806718)：success，UTC 2026-10-20T13:17:59Z。
- [Node / 39428806798](https://github.com/williechen/rust_ai_chat/actions/runs/39428806798)：success，UTC 2026-10-20T13:15:52Z。

所有workflow SHA均為本篇main。CI不包含新增教學程式；jobs未逐項重新驗收，不能把workflow成功當新GUI通過。

### 限制與下一步

已驗證檔案SQLite關閉pool再開的資料持久性，尚未做OS程序重啟、真Tauri按鈕／視窗GUI、逐寵物生活窗、外觀載入、預設出籠、多寵物與正式使用者舊庫升級驗收。本日交付為可載入的控制台清單，不宣稱整隻寵物建立→生活窗主線已完成。下一步延續第一個仍缺失的身分／外觀／生活窗接線。教學發布、使用者套用、產品整合與功能驗收分開記錄。
