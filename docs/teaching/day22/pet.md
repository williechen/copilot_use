## 1. 學習大綱
Day22 r2 重用原草稿：控制台「建立寵物」→ 現有 Leptos bridge → JavaScript invoke → Tauri create_pet → SQLite transaction → 提交後 SELECT → 控制台回應名稱及 ID。主體 Red／Green 估 15 分鐘，Refactor 3 分鐘、來源／版本／驗證閱讀另估 7 分鐘。
來源 rust_ai_chat/main `da5155265bf66ef9d495fd38913ad6cb93e1bac8`，copilot_use 教學基底 `b3db92fcc1ff488b758eee5702f320d623d603b8`，規則 daily-tdd-minimum-v1；共同頁最後修改 2026-10-19T08:19:19.122Z。2026-10-20 Asia/Taipei 重讀共同規則、三專案入口、Day21、目標與狀態。入口 next_day=19/17 為過去觀察，learning-state 的 22 為本次條件更新基準。
目前 main.rs 已開 PET_DATABASE_URL 並 migrate，但沒有把 pool 傳給 Tauri；表單只驗證名稱。沿用這份 pool，避免另外開 app_data_dir DB 造成寫入與讀取不同檔案。DB 須由使用者事先建立；不自動清空或重建。既有 pet_profile 的 species 預設本日寫入 tortoise，owner 使用明確本機 ID。sexy／image_path 可空，尚未接外觀渲染。
本日只完成建立／資料儲存與回應切片；生活窗仍使用 PetMachine::default()，逐寵物身分、初始外觀畫面、重啟載入及多寵物 UI 尚未接通。copilot_use 使用 crates/pet_runtime 的 PetStore／registry 與不同 Tauri state，不能把本教學直接當產品實作或已驗收。
前置：既有 migrations 可執行。原 pet migration 被改過 sexy／image_path；舊 DB 若 checksum mismatch，停止並保留 DB 交人工處理，不改 migration、不刪庫。
## 2. 實作內容
### Red：持久化與拒絕空白
先在 apps/desktop_pet/src/lib.rs 加 `mod pet_store;`，新增 apps/desktop_pet/src/pet_store.rs，只放以下測試。執行 `cargo test -p desktop-pet day22_create_pet`，PetStore 尚不存在，預期編譯失敗。本輪未實測此 Red；匯入或原生依賴失敗不算測到業務行為。

```rust
#[cfg(test)]
mod day22_tests {
    use super::*;
    #[sqlx::test(migrations = "./migrations")]
    async fn day22_create_pet_persists(pool: sqlx::SqlitePool) {
        let store = PetStore { pool: pool.clone() };
        assert!(store.create(" 小橘 ").await.unwrap().contains("小橘"));
        let row: (String, String, String) = sqlx::query_as("SELECT name,species,owner_id FROM pet_profile")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row, ("小橘".into(), "tortoise".into(), "local-owner".into()));
        assert!(store.create("  ").await.is_err());
        let (count,): (i64,) = sqlx::query_as("SELECT count(*) FROM pet_profile").fetch_one(&pool).await.unwrap();
        assert_eq!(count, 1);
        let (violations,): (i64,) = sqlx::query_as("SELECT count(*) FROM pragma_foreign_key_check")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(violations, 0);
    }
}
```

### Green：完整 Store 與原生命令
在 apps/desktop_pet/src/pet_store.rs 的測試前加入完整 Store。沿用父 module 已有 normalize_new_pet_name。只在 transaction 內確保本機 owner、建立具 UUID 的 profile，失敗回滾；提交後獨立讀回。若提交成功但讀回失敗，訊息要求先核對，避免盲目重送建立另一隻。

```rust
#[derive(Clone)]
pub struct PetStore { pub pool: sqlx::SqlitePool }

impl PetStore {
    pub async fn create(&self, input: &str) -> Result<String, String> {
        let name = crate::normalize_new_pet_name(input)?;
        let id = uuid::Uuid::new_v4().to_string();
        let mut tx = self.pool.begin().await.map_err(|_| "無法開始儲存")?;
        sqlx::query("INSERT INTO pet_owner(id,name) VALUES ('local-owner','本機使用者') ON CONFLICT(id) DO NOTHING")
            .execute(&mut *tx).await.map_err(|_| "無法儲存主人")?;
        sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id) VALUES (?1,?2,'tortoise','local-owner')")
            .bind(&id).bind(&name).execute(&mut *tx).await.map_err(|_| "無法儲存寵物")?;
        tx.commit().await.map_err(|_| "寵物提交失敗")?;
        let (saved_id, saved_name): (String, String) = sqlx::query_as("SELECT id,name FROM pet_profile WHERE id=?1")
            .bind(&id).fetch_one(&self.pool).await.map_err(|_| "已提交但讀回失敗，請先查核資料，勿立即重送")?;
        Ok(format!("{saved_name}（{saved_id}）"))
    }
}
```

apps/desktop_pet/src/main.rs 全文改為：

```rust
use std::str::FromStr;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("PET_DATABASE_URL")?;
    let options = sqlx::sqlite::SqliteConnectOptions::from_str(&database_url)?.foreign_keys(true);
    let db = sqlx::sqlite::SqlitePoolOptions::new().connect_with(options).await?;
    sqlx::migrate!("./migrations").run(&db).await?;
    desktop_pet::application(db);
    Ok(())
}
```

apps/desktop_pet/src/lib.rs 依下列 diff 加入 module、pool state 與命令，保留原 menu／事件／漫遊等程式：

```diff
--- a/apps/desktop_pet/src/lib.rs
+++ b/apps/desktop_pet/src/lib.rs
@@ -1,4 +1,5 @@
 mod chat;
+mod pet_store;
 mod file_organizer;
 mod roam;
 
@@ -115,8 +116,17 @@
     normalize_new_pet_name(&name)
 }
 
-pub fn application() {
+#[tauri::command]
+async fn create_pet(
+    name: String,
+    store: State<'_, pet_store::PetStore>,
+) -> Result<String, String> {
+    store.create(&name).await
+}
+
+pub fn application(pool: sqlx::SqlitePool) {
     tauri::Builder::default()
+        .manage(pet_store::PetStore { pool })
         .on_window_event(|window, event| {
             if window.label() == "main" {
                 if let tauri::WindowEvent::CloseRequested { api, .. } = event {
@@ -175,6 +185,7 @@
             open_control_window,
             quit_pet,
             validate_new_pet_name,
+            create_pet,
         ])
         .run(tauri::generate_context!())
         .expect("failed to run Rust AI Desktop Pet");
```

### Green：真實控制台接線
下列兩份 diff 使既有 frontend/src/bridge.rs 的 validate_new_pet_name → window.validateNewPetName 呼叫真正 create_pet。這個舊 bridge 名稱暫留以縮小改動；按鈕改稱建立寵物，成功訊息只在 DB 提交且讀回後顯示。原驗證 IPC 保留供 Day21 與既有測試使用。

```diff
--- a/apps/desktop_pet/frontend/index.html
+++ b/apps/desktop_pet/frontend/index.html
@@ -105,7 +105,7 @@
     };
 
     window.validateNewPetName = (name) => {
-      return invoke("validate_new_pet_name", { name });
+      return invoke("create_pet", { name });
     };
 
   </script>
```

```diff
--- a/apps/desktop_pet/frontend/src/main.rs
+++ b/apps/desktop_pet/frontend/src/main.rs
@@ -122,12 +122,12 @@
                         spawn_local(async move {
                             let response = bridge::validate_new_pet_name(proposed).await;
                             set_new_pet_feedback.set(match response {
-                                Ok(name) => format!("名稱「{name}」有效；尚未儲存寵物"),
+                                Ok(name) => format!("已儲存寵物：{name}"),
                                 Err(error) => error,
                             });
                         });
                     }
-                >"檢查名稱"</button>
+                >"建立寵物"</button>
                 <p role="status">{move || new_pet_feedback.get()}</p>
             </section>
 
```

不改 public/pet.js，那是生活窗狀態流程，並非本日控制台建立表單的真實呼叫者。不假設新建生活視窗標籤，也不把 profile 寫入當成生活窗已綁定。
### Refactor 與必要補充
main 已驗 migrations，Store 不再重開另一份 DB；資料／OS Adapter 保持在 native，frontend 只送名稱與顯示結果。後續可統一 bridge 名稱，但需同步 Rust extern、wrapper、JS、呼叫者四處。Day21 的名稱有效不是建立成功；本頁更正原草稿預計改用 app_data_dir 的做法，與現有 PET_DATABASE_URL 保持一致。未改 Day21／封存 Day，未新增補教佇列。
## 3. 套件版本
來源 workspace 0.2.21、edition 2024；Tauri 2.12、tauri-build 2.7、SQLx 0.9（SQLite／macros／migrate 已開）、Tokio 1.53、UUID 1.28。沒有新增 Cargo 依賴，不需因本切片改 lock；精確解析版本見來源 Cargo.lock。native rustc／cargo 1.99.1；MSRV 未宣告／未驗證，跨平台／WASM 建置尚未執行。原生 Linux 的 GTK/WebKit 依賴與 GUI 顯示須另核對，不把 native 環境錯誤當 Red。
## 4. 驗證項目
命令 `cargo test -p desktop-pet day22_create_pet`：預期獨立 SQLite fixture 中名稱正規化、species=tortoise、owner 有效、空白不插入、foreign_key_check=0。本輪候選執行結果另附，未套用 rust_ai_chat，Red 未執行。
來源 [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39302157333) 在精確 main SHA 的 test-desktop-pet 成功，2026-10-19T18:24:00Z–18:26:32Z，命令 cargo test --locked -p desktop-pet。只覆蓋來源原有程式，沒有 Day22 Store／IPC，不代表資料庫／GUI／端到端驗收。
套用後應以專用 PET_DATABASE_URL 啟動控制台，按建立、看到名稱與 ID，獨立查 pet_profile／pet_owner；無效名稱不新增，DB 失敗不顯示成功。關閉再啟動只能先驗 DB row 還在，不能聲稱 UI 已自動載入。多次點擊會建立多隻、尚無冪等 key 與按鈕 busy guard，先單次提交；多寵物與完整 UI 流程留後續。產品整合、生活窗、外觀／重啟載入與完整驗收未完成。

本輪實際結果：隔離候選 cargo test -p desktop-pet day22_create_pet 仍在原生編譯，尚無測試結果；本頁完整程式碼區塊讀回一致。未執行 GUI／WASM／重啟載入驗收。
