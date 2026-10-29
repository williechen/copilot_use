# Pet Day29｜回籠失敗不能先關視窗：持久化與視窗狀態一致性

## ① 學習大綱與現況

唯讀程式：`williechen/rust_ai_chat/main@143d6b3f5180b669860b1300286b89464c669434`。已核對 `apps/desktop_pet/src/pet_store.rs::set_caged` 將 `pet_profile.is_default_closure` 寫入 SQLite，`apps/desktop_pet/src/pet_windows.rs::cage_saved_pet` 目前先 `window.close()`、移除 `Runtime`，**最後**才 `store.set_caged(&id,true).await`。若 SQLite 更新失敗，視窗與執行狀態已消失但資料庫仍表示出籠，重啟可能不一致。Day28 已完成基本保存，不重教。當日最小切片只修復**回籠的操作順序**，不新增 schema、檔案掃描或多寵同步。

真實鏈：Leptos「回籠子」按鈕 → `bridge::cage_saved_pet` → Tauri `cage_saved_pet` → `PetStore::profile` → `set_caged`（SQLite）→ `window.close` → `Runtime::remove`。Chat／Pet／Analytics 各 15 分鐘、彈性 15 分鐘，總計 60 分鐘；整合驗收另計。

## ② Red → Green → Refactor 與 FIFO

**Red：** 建立隔離 SQLite Pet 資料、開啟已存寵物視窗，模擬 `UPDATE pet_profile` 失敗（例如測試專用資料庫連線故障）。現有 `cage_saved_pet` 在資料庫回傳錯誤前已關閉視窗、清除 runtime；預期需求是失敗時視窗仍可操作且 runtime 不被清除。這是靜態程式推導的 Red，**未實際執行**。不得在真實使用者 DB 注入故障。

**Green：** 僅調整 `apps/desktop_pet/src/pet_windows.rs` 的現有命令順序。沿用現有 `profile`、`label`、`State<PetStore>` 與 `Runtime`：

```rust
#[tauri::command]
pub async fn cage_saved_pet(
    id: String,
    app: tauri::AppHandle,
    store: State<'_, PetStore>,
) -> Result<(), String> {
    store.profile(&id).await?;
    let window = app.get_webview_window(&label(&id)?);
    // 先確認 SQLite 已成功保存；失敗則不關閉視窗、不清 runtime。
    store.set_caged(&id, true).await?;
    if let Some(window) = window {
        window.close().map_err(|e| e.to_string())?;
    }
    app.state::<Runtime>().remove(&id);
    Ok(())
}
```

**重要限制：** 這只保證「SQLite 失敗不會先破壞視窗」。若 SQLite 成功但 `window.close()` 失敗，仍會有資料庫標記回籠、視窗尚在的狀態；需要另設補償策略、可重試操作或 UI 提示，不能稱為跨 SQLite/Tauri 的原子交易。進一步可將可注入失敗的 `set_caged` 呼叫抽象為 adapter，讓失敗路徑可以做不依賴 GUI 的單元測試；不要僅測試私有 helper 就宣稱真實視窗已驗收。

**Refactor／FIFO：** 將保存成功作為移除視窗與 runtime 的先決條件，避免重複狀態；保留既有多寵互不影響測試。Day25 以前不修改，未完成的既有補教項照原佇列 FIFO，不用本篇結案。

## ③ 套件版本

`rust_ai_chat` 根 `Cargo.toml` workspace `0.2.28`、edition `2024`；沿用既有 Tauri、Leptos、SQLx 與 SQLite，不新增 crate、migration、DB schema 或 lockfile 變更。精確 resolved 版本、MSRV 及本機安裝版本本次未核對。

## ④ 驗證、預期與實際

| 操作／命令 | 預期 | 本次實際 |
| --- | --- | --- |
| `cargo check --locked -p desktop_pet` | Tauri 命令編譯 | 未執行 |
| `cargo test --locked -p desktop_pet` | 既有多寵與資料存取測試不退化 | 未執行 |
| 隔離 DB 注入 `set_caged` 失敗 | 回傳錯誤、視窗保持、runtime 不清除 | 未執行 |
| SQLite 成功、正常關閉視窗 | 持久化回籠、runtime 移除 | 未執行 |
| SQLite 成功但關窗失敗 | 必須提示不一致並提供補償；**本切片尚未解決** | 未執行 |
| 重啟 App 後重新載入寵物 | 回籠偏好與 UI 一致 | 未執行 |

CI：未取得來源 SHA `143d6b3f5180b669860b1300286b89464c669434` 對應 workflow/run/job 成功證據；不得宣稱通過。未寫入 `rust_ai_chat`、未執行資料庫操作、未完成產品功能驗收。本文只代表 GitHub 教學交付。
