# Pet Day 28｜回籠偏好保存至 SQLite，重啟不再誤出籠

## ① 學習大綱與真實現況

程式唯讀來源：`williechen/rust_ai_chat/main@e62ed41e2122ef0979af8e2d453bafa2e05aa2e5`；文件正本：`williechen/copilot_use/main@b4e650ab5a51cbb872db61db9e8f2eb3e9f54bc8`；規則 `daily-tdd-minimum-v1 / github-only-v1`（blob `5df88ba6892c377e46b6a76a934b9f59b51694e7`）。

已核對 Day27 的 Leptos 建立按鈕 → `bridge::create_pet` → Tauri `create_pet` → SQLx `PetStore::create` → SQLite `pet_profile` → `select_pet` → `pet_windows::open`，現有來源已接通；不重教。問題是 `pet_windows::cage_saved_pet` 只關窗，`application().setup` 重啟仍透過 `PetStore::active` 開啟 `local_pet_selection.slot=1` 的寵物。既有 `pet_profile.is_default_closure` 可保存回籠偏好，無須修改歷史 migration。

本日最小流程：Leptos 生活窗「回籠子」→ `bridge::cage_saved_pet` → Tauri IPC → SQLx SQLite 旗標 → 重啟 `setup/active` 不重開；再次選取或出籠可恢復。主體 Pet Red+Green 15 分鐘；Chat 15、Analytics 15、共用彈性 15，合計 60 分鐘。Refactor 5、DB/GUI 驗證 20 分鐘另計。

## ② 真 Red／完整最小 Green／Refactor／FIFO

**Red：** 在 `apps/desktop_pet/src/pet_store.rs` 既有 `#[cfg(test)] mod day22_tests` 中新增下列測試；未實作 `set_caged` 時預期編譯失敗，不冒稱已執行：

```rust
#[sqlx::test(migrations = "./migrations")]
async fn day28_caged_pet_is_not_active(pool: sqlx::SqlitePool) {
    let store = PetStore { pool };
    let pet = store.create("阿龜").await.unwrap();
    store.select(&pet.id).await.unwrap();
    store.set_caged(&pet.id, true).await.unwrap();
    assert_eq!(store.active().await.unwrap(), None);
    store.select(&pet.id).await.unwrap();
    assert_eq!(store.active().await.unwrap(), Some(pet));
}
```

**Green A：** `apps/desktop_pet/src/pet_store.rs` 的 `impl PetStore` 新增完整方法：

```rust
pub async fn set_caged(&self, id: &str, caged: bool) -> Result<(), String> {
    let result = sqlx::query(
        "UPDATE pet_profile SET is_default_closure=?1,
         updated_at=CURRENT_TIMESTAMP
         WHERE id=?2 AND owner_id='local-owner'"
    )
    .bind(caged).bind(id).execute(&self.pool).await
    .map_err(|_| "無法保存回籠狀態".to_owned())?;
    if result.rows_affected() != 1 {
        return Err("找不到可回籠的寵物".into());
    }
    Ok(())
}
```

同檔 `active()` 既有 SQL `WHERE s.slot=1 AND p.owner_id='local-owner'` 的 WHERE 後加上 `AND COALESCE(p.is_default_closure,0)=0`；其餘函式內容保留。`select()` 在既有 `local_pet_selection` INSERT 成功、`tx.commit()` 前加入：

```rust
sqlx::query(
    "UPDATE pet_profile SET is_default_closure=FALSE,
     updated_at=CURRENT_TIMESTAMP
     WHERE id=?1 AND owner_id='local-owner'"
)
.bind(id).execute(&mut *tx).await
.map_err(|_| "無法恢復出籠狀態".to_owned())?;
```

**Green B：** `apps/desktop_pet/src/pet_windows.rs` 完整取代既有兩個 command（保留 `#[tauri::command]`、其他 `Runtime` 和 `open` 不動）：

```rust
#[tauri::command]
pub async fn open_saved_pet(
    id: String,
    app: tauri::AppHandle,
    store: State<'_, PetStore>,
) -> Result<(), String> {
    let pet = store.profile(&id).await?;
    open(&app, &pet)?;
    store.set_caged(&id, false).await
}

#[tauri::command]
pub async fn cage_saved_pet(
    id: String,
    app: tauri::AppHandle,
    store: State<'_, PetStore>,
) -> Result<(), String> {
    store.profile(&id).await?;
    if let Some(window) = app.get_webview_window(&label(&id)?) {
        window.close().map_err(|e| e.to_string())?;
    }
    app.state::<Runtime>().remove(&id);
    store.set_caged(&id, true).await
}
```

`frontend/src/main.rs` 已有真實 Leptos 出籠、回籠、選取按鈕；`frontend/index.html` 以每窗 `pet_id` 呼叫現有 IPC；`lib.rs::application` 已註冊上述 commands，`setup` 既有 `store.active().await` 自動受新旗標影響，不須再新增 UI／router。若關窗成功而 DB 寫入失敗，回傳錯誤，可能在重啟後再開；OS/DB 並非原子交易，須另驗。

**Refactor／FIFO：** `local_pet_selection` 是控制台最後選取，`is_default_closure` 是單寵物回籠偏好；SQLx 僅在 PetStore。`pet-day25-single-image-20261022` 保持 pending：Day26 單張圖已存在，但三個不同 UUID 原生 GUI、每幀單圖、事件與 200×200 裁切未以精確 SHA 驗證；本篇不結案、不回寫 Day25（含）以前封存內容。多生活窗重啟恢復仍是另一缺口。

## ③ 套件版本

`rust_ai_chat` workspace `edition 2024`、`version 0.2.27`；`tauri 2.12`、`sqlx 0.9`（SQLite）、`tokio 1.54`、`uuid 1.28`、`serde 1.0`、`leptos 0.8`。沿用既有 `pet_profile` 欄位，無新增 crate、無新 migration、無 lock 修改。未解析完整 `Cargo.lock` resolved versions／MSRV／native toolkit，未宣稱編譯成功。

## ④ 命令、預期／實際、CI/job/SHA、限制

| 命令 | 預期 | 實際 |
| --- | --- | --- |
| `cargo test --locked -p desktop-pet day28_caged_pet_is_not_active` | Red 缺方法失敗；Green 回籠/再選取正確 | **未執行** |
| `cargo test --locked -p desktop-pet` | 舊測試不退化 | **未執行** |
| `cargo check --locked -p desktop-pet-frontend --target wasm32-unknown-unknown` | Leptos/IPC 編譯 | **未執行** |
| `cd apps/desktop_pet && cargo tauri dev`，出籠→回籠→重啟 | 已回籠不自動開窗 | **未執行／GUI 未驗收** |
| 三個不同 pet id 的原生生活窗、事件、單幀單圖 | 各自獨立 | **未執行** |

來源 `e62ed41e2122ef0979af8e2d453bafa2e05aa2e5` 的 [Rust Desktop Pet run 39869399828](https://github.com/williechen/rust_ai_chat/actions/runs/39869399828)：`check-pet` job `120139710456`、`build-pet` job `120139710914` 讀取時為 success；[Test All run 39869399841](https://github.com/williechen/rust_ai_chat/actions/runs/39869399841) 的 `test-desktop-pet` job `120139710521` 當時 in_progress。來源 CI 不含未套用的 Day28 Green；`copilot_use` 文件 CI 不算程式驗收。本文只交付教學，沒有修改 `rust_ai_chat`、操作 DB、執行測試或產品驗收。
