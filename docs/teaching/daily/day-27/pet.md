# Pet Day 27｜建立第一隻寵物後自動選取、出籠並保留重啟身分

## ① 學習大綱／真實現況

- 學習庫來源：`rust_ai_chat/main@8cf82626c9c82939a3a56eab32e24a8a5b5b00ed`（唯讀）；教學正本：`copilot_use/main@998fe318e1b9cbae291a121274fd9bad3b94272f`，規則 blob `bb26b0f3b867a7d3da20fc408dd3ea987c9f41b0`，版本 `daily-tdd-minimum-v1`／`github-only-v1`。
- 本日最小使用者流程：在控制台輸入名稱 → 按「建立寵物」 → SQLite INSERT、提交後以 id 讀回 → 以同一 id 呼叫 `select_pet` → 寫入 `local_pet_selection` → Tauri `pet_windows::open` 顯示獨立生活窗；重啟時既有 `setup` 讀 `active()` 恢復最後選擇的寵物。
- 現有 `PetStore::create` 回 `"名稱（UUID）"` 字串，`frontend/src/bridge.rs::create_pet` 回 `String`，前端只更新列表。即使一隻已持久化，還須人工點「選為生活窗寵物」或「出籠」，建立按鈕未連到生活窗。Day26 的 `PetStatus` 已是單張 SVG `img`，不重做動畫。既有 `select_pet` 的選取／開窗行為可重用，不另起第二套 IPC。
- 主體切片估 15 分鐘；Refactor、原生 GUI、版本與 CI 核對另估 15–30 分鐘，不冒稱一小時皆為 Red/Green。

## ② 真 Red／完整 Green／Refactor／舊 Day FIFO

### Red（預期編譯失敗，非本次已跑測試）

新增 `apps/desktop_pet/tests/day27_create_and_select.rs`（完整）：

```rust
// apps/desktop_pet/tests/day27_create_and_select.rs
use desktop_pet::pet_store::PetStore;
use sqlx::sqlite::{SqliteConnectOptions,SqlitePoolOptions};

#[tokio::test]
async fn day27_new_pet_has_stable_identity_and_restart_selection() {
    let db_path=std::env::temp_dir()
        .join(format!("pet27-{}.sqlite",uuid::Uuid::new_v4()));
    let options=SqliteConnectOptions::new()
        .filename(&db_path).create_if_missing(true).foreign_keys(true);
    let pool=SqlitePoolOptions::new().max_connections(1)
        .connect_with(options.clone()).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    let store=PetStore{pool:pool.clone()};
    let created=store.create(" 阿龜 ").await.unwrap();
    assert_eq!(created.name,"阿龜");
    assert_eq!(created.species,"tortoise");
    assert!(uuid::Uuid::parse_str(&created.id).is_ok());
    assert_eq!(store.profile(&created.id).await.unwrap(),created);
    assert_eq!(store.select(&created.id).await.unwrap(),created);
    assert_eq!(store.active().await.unwrap(),Some(created.clone()));
    pool.close().await;
    let reopened=SqlitePoolOptions::new().max_connections(1)
        .connect_with(options).await.unwrap();
    let reloaded=PetStore{pool:reopened.clone()};
    assert_eq!(reloaded.active().await.unwrap(),Some(created));
    reopened.close().await;
    std::fs::remove_file(db_path).unwrap();
}
```

來源 `create()` 仍回 `String`，故測試對 `created.name`／`.id`／`.species` 的存取預期編譯失敗，**不是資料庫錯誤，也不是實測 Red**。同測試在 Green 後驗證提交後的資料身分及 SQLite 重新開啟仍保留選取。

### Green A：讓 SQLite 來源回傳結構化的已儲存身分

在 `apps/desktop_pet/src/pet_store.rs` **以以下完整函式取代原有 `pub async fn create`**；`SavedPet`、`profile`、`list`、`select` 及 migrations 不更動：

```rust
// apps/desktop_pet/src/pet_store.rs
// 只將既有 create() 簽名與尾端的返回值更換，
// INSERT/transaction/提交後 SELECT id,name/錯誤分支完整保留。
pub async fn create(&self, input: &str) -> Result<SavedPet, String> {
    let name = crate::normalize_new_pet_name(input)?;
    let id = uuid::Uuid::new_v4().to_string();
    let mut tx = self.pool.begin().await.map_err(|_| "無法開始儲存")?;
    sqlx::query(
        "INSERT INTO pet_owner(id,name) VALUES('local-owner','本機使用者') ON CONFLICT(id) DO NOTHING"
    ).execute(&mut *tx).await.map_err(|_| "無法儲存主人")?;
    sqlx::query(
        "INSERT INTO pet_profile(id,name,species,owner_id) VALUES(?1,?2,'tortoise','local-owner')"
    ).bind(&id).bind(&name)
      .execute(&mut *tx).await.map_err(|_| "無法儲存寵物")?;
    tx.commit().await.map_err(|_| "寵物提交失敗")?;
    let (saved_id,saved_name,species):(String,String,String) =
        sqlx::query_as(
            "SELECT id,name,species FROM pet_profile WHERE id=?1 AND owner_id='local-owner'"
        ).bind(&id).fetch_one(&self.pool).await
          .map_err(|_| "已提交但讀回失敗，請先查核資料，勿立即重送")?;
    Ok(SavedPet { id:saved_id, name:saved_name, species })
}
```

既有 `day22_create_pet_persists` 測試內 `assert!(store.create(" 小橘 ").await.unwrap().contains("小橘"));` 改為 `assert_eq!(store.create(" 小橘 ").await.unwrap().name, "小橘");`，以維持既有資料持久測試語意。其他 `store.create(...).await.unwrap();` 不需要修改。

### Green B：Tauri command 與 WASM bridge 改用相同 `SavedPet` 結構

在 `apps/desktop_pet/src/lib.rs` 將原有 `create_pet` 函式替換為：

```rust
#[tauri::command]
async fn create_pet(
    name: String,
    store: State<'_, pet_store::PetStore>,
) -> Result<pet_store::SavedPet, String> {
    store.create(&name).await
}
```

在 `apps/desktop_pet/frontend/src/bridge.rs` 將原有 `pub async fn create_pet(name:String) -> Result<String,String>` 函式整段替換（保留既有 `SavedPet` Deserialize 定義）：

```rust
pub async fn create_pet(name: String) -> Result<SavedPet, String> {
    let value=JsFuture::from(create_pet_js(&name)).await.map_err(js_error)?;
    serde_wasm_bindgen::from_value(value)
        .map_err(|_| "建立寵物回應格式錯誤；請重新整理清單確認是否已建立".into())
}
```

既有 `frontend/index.html` 的 `window.createPet=(name)=>invoke("create_pet",{name})` 不必改；JSON 由 Tauri 直接序列化。沒有透過顯示名稱反推 ID，避免相同名字時選錯寵物。

### Green C：建立按鈕完成真實「建立 → 生活窗」

在 `apps/desktop_pet/frontend/src/main.rs`「建立寵物」按鈕既有 `on:click` 中只替換其 `spawn_local(async move { ... })`，其餘 busy gate、欄位與已儲存清單完整保留：

```rust
// apps/desktop_pet/frontend/src/main.rs
// 僅替換「建立寵物」按鈕原有的 spawn_local(async move { ... }) 閉包。
spawn_local(async move {
    match bridge::create_pet(proposed).await {
        Ok(created) => {
            // 先更新清單，不因後續開窗失敗重送 INSERT。
            match bridge::list_pets().await {
                Ok(rows) => set_saved_pets.set(rows),
                Err(_) => set_new_pet_feedback.set(
                    "寵物已儲存，但清單讀回失敗；請重新載入，勿重送建立。".into()
                ),
            }
            // select_pet Tauri command 既有實作會先儲存
            // local_pet_selection，再呼叫 pet_windows::open。
            match bridge::select_pet(&created.id).await {
                Ok(selected) => {
                    set_active_pet.set(Some(selected));
                    set_new_pet_name.set(String::new());
                    set_new_pet_feedback.set(
                        format!("已建立並出籠：{}（{}）",created.name,created.id)
                    );
                }
                Err(error) => set_new_pet_feedback.set(format!(
                    "寵物已儲存：{}（{}）；選取／出籠未完成：{}。請從清單重試，不要再建立。",
                    created.name, created.id, error
                )),
            }
        }
        Err(error) => set_new_pet_feedback.set(format!("建立寵物失敗：{error}")),
    }
    set_pets_busy.set(false);
});
```

既有 `select_pet` Tauri command 會執行 `store.select(&id).await?`、`pet_windows::open(&app,&saved)?`，並 emit `pet-profile-changed`，故正常路徑可直接顯示生活窗。若資料已提交但「選取／開窗」失敗，只記錄真實已儲存的 id 及重試指示，不重新 INSERT 或謊稱回滾；SQLite 資料保持原狀。若 IPC 成功但清單讀取失敗，仍可繼續出籠，清單另行手動重試。

### Refactor／前置契約

使用 `SavedPet{id,name,species}` 作為 DB → command → JS → Rust/WASM UI 的唯一結構化契約，不增加第二個 pet ID 格式或複製 SQLite 查詢。原有 `pet_windows::Runtime` 的逐 ID FSM、`pet-* `視窗命名、事件 payload id 篩選與 `PetStatus` 單一 img 保留。

**Day25 FIFO 補教 `pet-day25-single-image-20261022`：仍為 pending。** Day26 的單圖片程式已出現在最新 main，但三個不同 UUID 的原生 GUI、事件、裁切、回籠與重啟仍未現場驗證；本篇僅新增真正的一鍵操作流，不以單元測試結案 Day25，更不能回寫封存正文。

## ③ Cargo／版本

來源 workspace edition 2024、version 0.2.25；`sqlx 0.9` SQLite、`tokio 1.54`、`uuid 1.28`、`serde 1.0`；frontend `leptos 0.8`、`serde-wasm-bindgen 0.6`、`wasm-bindgen 0.2`、`gloo-timers 0.4`。不新增 Cargo dependency 或 migration；版本為 manifest requirement，不冒稱已核對 lock 的每個精確解析版本及本機 stable／MSRV。

## ④ 驗證、CI/job/SHA、限制

| 驗證 | 預期 | 實際 |
| --- | --- | --- |
| `cargo test -p desktop-pet --test day27_create_and_select` | 產生 UUID、提交後回傳 SavedPet、選取及重開 SQLite 成功 | 未執行 |
| `cargo test -p desktop-pet` | 舊 Day22/23/24 測試與新契約兼容 | 未執行 |
| `cargo check -p desktop-pet-frontend --target wasm32-unknown-unknown` | Frontend bridge 新型別編譯 | 未執行 |
| `cd apps/desktop_pet && cargo tauri dev` | 建立→自動出籠、三 ID 不混線、同 ID 不重複、重啟回讀 | 未執行／GUI 未驗收 |

唯一 CI 範圍：`rust_ai_chat/main@8cf82626c9c82939a3a56eab32e24a8a5b5b00ed` 的 [Rust Desktop Pet Actions Run 39785099304](https://github.com/williechen/rust_ai_chat/actions/runs/39785099304) completed/success、[Test All Run 39785099220](https://github.com/williechen/rust_ai_chat/actions/runs/39785099220) completed/success；屬**原始來源 commit**，不含本文 Green；`copilot_use` Actions 不作教學 CI 證據。新生寵物的原生 GUI、透明視窗裁切、不同 id 的 3 窗事件隔離、狀態變更持久化（目前 Runtime FSM 為記憶體狀態）、重啟自動恢復最後選擇與完整產品驗收仍須現場測試，沒有宣稱已通過。
