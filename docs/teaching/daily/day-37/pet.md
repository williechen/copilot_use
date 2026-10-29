# Pet Day37｜出籠建窗失敗時不留下錯誤 SQLite 出籠標記

> 2026-10-25；產品來源 `williechen/rust_ai_ring/main@5c713553daee4055738239f047d6c774fcc7a7a1`（唯讀），文件存放 `williechen/copilot_use/main`。接續 [Day36 r3](../day-36/pet-r3.md)。本篇**尚未套用、尚未測試、未驗收**，版本 Gate 完整查核仍待完成。

## ① 學習大綱

現行 `pet_apps/pet_server/src/pet_windows.rs::open_saved_pet` 的順序為 `store.profile` → `open(&app,&pet)` → `store.set_caged(id,false)`。因此 DB 更新失敗時，生活窗可能已開啟；上次出籠標記值未知時也不能把所有失敗均回寫 true。真呼叫鏈：`pet_ui` Leptos CSR → Tauri `open_saved_pet` → `pet_store::PetStore` → `pet_persistence` SQLite；窗口由 `pet_windows::open` 管理，`Runtime` 按 UUID 分流。Pet Day36 已完成多隻恢復局部測試，真 native 重啟尚待驗。

**今日切片**：先以儲存層讀取舊籠狀態、開窗、寫入新狀態；若寫入失敗且是本次新建窗，盡力關閉剛建立的窗／清理 Runtime，同時明確回傳補償失敗。本教學不改「啟動時只有預設寵物自動出籠」尚待資料模型分離的需求。

## ② Red → Green → Refactor

### Red：先要求 adapter 能準確讀取原標記

在 `pet_apps/pet_persistence/src/lib.rs` 測試模組加入：

```rust
#[cfg(test)]
mod day37_open_state_tests {
    use super::*;
    #[sqlx::test(migrations = "../migrations")]
    async fn read_caged_flag_and_fail_closed_when_unknown(pool: sqlx::SqlitePool) {
        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','本機')")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id,is_default_closure) VALUES('a','A','tortoise','local-owner',1)")
            .execute(&pool).await.unwrap();
        assert_eq!(read_caged(&pool, "a").await.unwrap(), true);
        assert!(read_caged(&pool, "missing").await.is_err());
        set_caged(&pool,"a",false).await.unwrap();
        assert!(!read_caged(&pool,"a").await.unwrap());
    }
}
```

預期 Red：缺少 `read_caged` 導致編譯失敗；這不是 DB 環境錯誤。接著對 `open_saved_pet` 加入有 Tauri runner 時才能執行的真實建窗失敗注入，不把純 SQLite 測試冒稱 native GUI 驗收。

### Green A：補上精確最小 SQLx 查詢

在 `pet_apps/pet_persistence/src/lib.rs` 的 `set_caged` 後加入：

```rust
pub async fn read_caged(pool: &sqlx::SqlitePool, id: &str) -> Result<bool, String> {
    let value: Option<i64> = sqlx::query_scalar(
        "SELECT COALESCE(is_default_closure,0) FROM pet_profile
         WHERE id=?1 AND owner_id='local-owner'"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|_| "無法讀取回籠狀態".to_owned())?;
    match value {
        Some(0) => Ok(false),
        Some(1) => Ok(true),
        Some(_) => Err("回籠狀態資料不合法".into()),
        None => Err("找不到寵物".into()),
    }
}
```

在 `pet_apps/pet_server/src/pet_store.rs` 的 `impl PetStore` 內新增真 caller wrapper：

```rust
pub async fn read_caged(&self, id: &str) -> Result<bool, String> {
    pet_persistence::read_caged(&self.pool, id).await
}
```

### Green B：新建窗可補償、已存在窗不能誤關

在 `pet_apps/pet_server/src/pet_windows.rs` **完整替換**原 `open_saved_pet`：

```rust
#[tauri::command]
pub async fn open_saved_pet(
    id: String,
    app: tauri::AppHandle,
    store: State<'_, PetStore>,
) -> Result<(), String> {
    let pet = store.profile(&id).await?;
    let old_caged = store.read_caged(&id).await?;
    let name = label(&id)?;
    let existed = app.get_webview_window(&name).is_some();

    open(&app, &pet)?;
    if let Err(error) = store.set_caged(&id, false).await {
        // 若原本已有視窗，絕對不可因這次儲存失敗而關掉其他呼叫者的視窗。
        if !existed {
            let close_result = match app.get_webview_window(&name) {
                Some(window) => window.close().map_err(|e| e.to_string()),
                None => Ok(()),
            };
            app.state::<Runtime>().remove(&id);
            if let Err(close_error) = close_result {
                return Err(format!(
                    "出籠狀態保存失敗：{error}；關窗補償也失敗：{close_error}"
                ));
            }
        }
        return Err(format!("出籠狀態保存失敗：{error}；原回籠值：{old_caged}"));
    }
    Ok(())
}
```

本切片只涵蓋**單呼叫、沒有同 ID 併行交錯**的補償。從 `open` 回傳到 `set_caged` 中間，第二個並行請求仍可能觸及同一窗；正式完整 Green 還必須將該流程序列化，並以視窗建立識別而非單次預先 `existed` 判斷所有權。因此此處是**待補足的中間稿，不可當成完整可套用 Green 或通過文件 Gate**。不要在未處理併行前將其投入正式產品。

### Refactor 與上篇補充

`pet_persistence` 只實作 SQLx adapter，不依賴 Tauri；`PetStore` 依舊是 server 的 adapter 呼叫橋；不建立新空庫。由真實 `pet_ui` 的 `open_saved_pet` IPC 保持原有名稱與參數，不更新 Leptos CSR metadata。需要再加入 per-pet async coordination 或持久化 state machine 才能正確處理同 ID 同時出籠／回籠、關窗事件再入及失敗補償競態；直接關生活窗等同回籠、預設資格與出籠分離亦未完成。Day36 原始正文與來源證據保留。Rust crate 用 `pet_server`/`pet_persistence`，Cargo -p 用 `pet-server`/`pet-persistence`。`pet_apps/Cargo.toml`、`pet_server/Cargo.toml` 已有 `tauri`、`sqlx`、`pet-persistence`，無新增 dependencies/features/imports/migrations；`pet_apps/Cargo.lock` 不修改。四個 workspace 無須全部改動。

## ③ 官方版本與相容性

交付日 2026-10-25；`pet_apps/Cargo.lock`：Tauri **2.12.5**、SQLx **0.9.0**、Tokio **1.54.1**、Leptos **0.8.22**；`Cargo.toml`：tauri `2.12`、sqlx `0.9`、tokio `1.54`、leptos `0.8`、Rust Edition 2024。Rust 官方穩定 **1.99.1**（[官方發行](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)）。各 crate 的**當日最新官方穩定發布精確版仍未驗證**（[tauri](https://crates.io/crates/tauri)、[sqlx](https://crates.io/crates/sqlx)、[tokio](https://crates.io/crates/tokio)、[leptos](https://crates.io/crates/leptos)），不得以 lock 版冒稱最新；native WebKitGTK 系統依賴及 MSRV 待實機檢查，不升級 lock。**版本 Gate 1 未完成**。

## ④ 驗證步驟、預期與實際

```bash
cd /home/willie/Documents/rust_ai_ring/pet_apps
cargo test -p pet-persistence day37_open_state_tests
cargo test -p pet-server
cargo check -p pet-server
cargo check -p pet-ui --target wasm32-unknown-unknown
```

預期：Red 缺符號、SQLx Green 正確讀值；完整實作仍需 Tauri native 實測一窗與三窗、DB 寫入失敗、窗口關閉失敗、同 UUID 雙呼叫、真正重啟及預設資格。實際：**尚未套用、未執行 Red/Green、無 CI 或 GUI 驗收**。本文章只是保留問題及補償設計的草稿，不得推進 Day。
