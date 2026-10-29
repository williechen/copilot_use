# Pet Day39 r1｜控制窗逐 UUID「找回寵物」到主螢幕可用區

> **r1 勘誤**：保留 [Day39 初版](pet.md) 原文、commit 與 blob；將 Refactor 在 server 的新 Application 用例改為完整路徑呼叫，避免與 Green 同時編輯 `pet_windows.rs` 的相鄰 imports，兩 patch 可獨立套用且不新增 Cargo。原功能 Green patch 不變。

> 2026-10-26；程式唯一來源 `williechen/rust_ai_ring/main@8fc4b277b6f6f77d97655130036fca47813de56e`（唯讀）；文件 `williechen/copilot_use/main`。接續 [Day38 r4](../day-38/pet-r4.md) 已接通的逐 UUID roam registry／Application 出籠補償。**本篇方案未套用，未做原生 GUI／產品驗收**；先確認最新來源確有該程式再提出修改。

## ① 功能目標與真實呼叫鏈

需求明確要求控制窗能找回跑出可視區的某隻生活窗，不改其漫遊設定、不移動控制窗、不影響其他 UUID。現有 `pet_server::roam` 已逐 label 漫遊，`pet_windows::label` 用 `pet-<UUID.simple()>`，但 `pet_ui/src/main.rs` 的多寵物清單沒有找回按鈕。新增每隻的「找回寵物」：Leptos `For` → `bridge::find_saved_pet` → `index.html::findSavedPet` → Tauri `find_saved_pet` → SQLite `PetStore::profile` 存在檢查 → 指定原生生活窗 → 主螢幕 work_area 置中 → show/focus。既有出入籠、選取、SQLite 資料、主控制窗不重建。

核心60分鐘只計② Red／Green，Refactor／版本／驗證／文件提交另計。此功能不是「自動跑回」的全能力，也不處理螢幕拔除策略與拖曳恢復競態；原需求仍待驗。

## Refactor｜Pet 自身回籠補償移至 Application（獨立於找回功能）

前篇已將 `persist_then_open` 搬入 `pet_application::lifecycle`，真 `pet_windows::open_saved_pet` 會呼叫它。本篇沿相同 Pet 分層將 **原 `cage_saved_pet` 的「先寫回籠、關窗失敗恢復出籠、回復失敗報錯」** 抽成 `persist_then_close`，server 僅提供 SQLx `store.set_caged` 與 Tauri `window.close` closure，並維持 lifecycle lock 與 Runtime 移除時序。這不是② Green 的後續小整理，也沒有修改跨案 crates、全庫 CI 或 Analytics／Chat。

**完整可套用 Refactor patch**：[pet-day39-refactor-r1.patch](pet-day39-refactor-r1.patch)。`pet_application/src/lifecycle.rs` 新增完整用例函式及三組純測試：DB失敗不得關窗、原生失敗恢復與補償失敗回報、成功只寫一次並關一次。原 server cage caller 保留 `profile` 檢查、生命週期鎖、關窗後才移除 Runtime；`pet_persistence` 原逐 ID SQLite/失敗斷言不移除。既有回復 false 的語義保持不變；日後須分離「預設資格」與「目前回籠」欄位，不在此悄悄改 migration。

```rust
pub async fn persist_then_close<W, F, C>(mut write: W, close_window: C) -> Result<(), String>
where W: FnMut(bool) -> F,
      F: std::future::Future<Output = Result<(), String>>,
      C: FnOnce() -> Result<(), String>,
{
    write(true).await?;
    if let Err(error) = close_window() {
        return match write(false).await {
            Ok(()) => Err(format!("關閉寵物視窗失敗，已恢復出籠狀態：{error}")),
            Err(rollback) => Err(format!("關閉寵物視窗失敗：{error}；回復資料庫亦失敗：{rollback}")),
        };
    }
    Ok(())
}
```

**斷言承接**：Day35 PetStore 與 pet_persistence 的逐 ID、DB 失敗、缺 ID 斷言保留；新 Application fake 用例測 native close failure；Tauri lifecycle lock、Runtime 清除和 GUI 必須仍由原生測試驗證。無 Cargo/feature/lock 變更：pet-application 已是 pet-server 依賴，Rust crate 直接使用 `pet_application::lifecycle::persist_then_close` 完整路徑，既有 `persist_then_open` import 不動。本次未跑測試；不以純測試取代 native。風險為 close 事件可能觸發 Destroyed（原有 Runtime 清理），故套用後必測重入／重開；若 patch 不符最新來源停止，從乾淨候選以 `git apply -R` 回復。下一步處理直接關窗與預設資格分離，需新的持久化策略。

## ② 功能實作：Red → 完整 Green

### Red｜主螢幕置中及多寵物隔離

先於 `pet_apps/pet_server/src/pet_windows.rs::tests` 加入下列測試；缺少 `centered_position` 時屬**缺符號編譯 Red**，並非已跑失敗。另在原生三窗環境測「找回寵物」按鈕，目前 UI 不存在（尚未實測）。

```rust
#[test]
fn day39_find_position_respects_primary_work_area_and_negative_origins() {
    let area = PhysicalRect {
        position: PhysicalPosition { x: -1920, y: -100 },
        size: PhysicalSize { width: 1920, height: 1080 },
    };
    assert_eq!(centered_position(&area, PhysicalSize { width: 200, height: 200 }),
        PhysicalPosition { x: -1060, y: 340 });
    assert_eq!(centered_position(&area, PhysicalSize { width: 2500, height: 1200 }),
        PhysicalPosition { x: -1920, y: -100 });
}
```

### Green｜真實 UI→IPC→native caller，完整 patch

[pet-day39-green.patch](pet-day39-green.patch) 含 **六個完整檔案變更**：`pet_server/src/pet_windows.rs`（置中演算法＋真 Tauri command＋測試）、`pet_server/src/roam.rs`（只取消目標 UUID 任務＋測試）、`pet_server/src/lib.rs`（generate_handler）、`pet_ui/index.html`（JS invoke）、`pet_ui/src/bridge.rs`（wasm_bindgen import＋async wrapper）、`pet_ui/src/main.rs`（逐 ID 找回按鈕、loading、結果回饋）。程式路徑均位於 `pet_apps`；Cargo package `pet-server`、`pet-ui`，Rust crate `pet_server`、`pet_ui`，無新增依賴或 features。

```rust
#[tauri::command]
pub async fn find_saved_pet(id: String, app: tauri::AppHandle,
    store: State<'_, PetStore>) -> Result<(), String> {
    let _operation = lifecycle_guard().await;
    store.profile(&id).await?;
    let name = label(&id)?;
    let window = app.get_webview_window(&name).ok_or("寵物尚未出籠")?;
    let monitor = app.primary_monitor().map_err(|e| e.to_string())?.ok_or("找不到主螢幕")?;
    let size = window.outer_size().map_err(|e| e.to_string())?;
    let position = centered_position(monitor.work_area(), size);
    crate::roam::stop_for_pet(&name);
    let result = window.set_position(position).and_then(|_| window.show()).and_then(|_| window.set_focus());
    crate::roam::spawn_for_pet(app.clone(), name);
    result.map_err(|e| format!("找回寵物失敗：{e}"))
}
```

只操作現存生活窗，未出籠拒絕，不偷偷開新窗；移動前停止該 label 的 roam task，移動後按原設定重新啟動（現行來源仍由全域 `PET_ENABLE_ROAM` 決定，**逐寵物漫遊持久化尚未完成**，不得冒稱滿足完整政策）。主螢幕取得失敗時保留原窗，不動資料。需真 GUI 驗證舊任務取消與新任務競態。

```bash
# rust_ai_ring 根目錄，僅在使用者自己的工作樹
 git apply --check /path/to/pet-day39-refactor-r1.patch
 git apply /path/to/pet-day39-refactor-r1.patch
 git apply --check /path/to/pet-day39-green.patch
 git apply /path/to/pet-day39-green.patch
 cd pet_apps
 cargo test --locked -p pet-application
 cargo test --locked -p pet-persistence
 cargo test --locked -p pet-server day39
 cargo check --locked -p pet-server
 cargo check --locked -p pet-ui --target wasm32-unknown-unknown
 node --test pet_ui/tests/*.test.mjs
```

### 有效斷言、手動驗收與下一步

Day24/35/36/37/38 的逐 ID 狀態、關窗補償、恢復清單、roam task registry 斷言全保留。原生三窗驗證：A、B、C 同時出籠，移 A 到副螢幕外後只找回 A，B/C 位置及狀態不變；重複找回不開新窗；回籠 A 後找回報錯；控制窗不移動；禁用漫遊時找回不重啟；主螢幕不存在時錯誤可見；負座標及高 DPI 正確。直接關窗尚未持久化，需下一 Day 獨立功能切片處理。

## ③ 2026-10-26 官方穩定版核對

| 工具 | 來源 manifest／lock | 官方穩定版與日期 | 相容處置 |
| --- | --- | --- | --- |
| Rust | edition 2024，執行環境未讀 | 1.99.1（10/15） | 機器上另驗 rustc、native/WASM |
| Tauri | `2.12`／`2.12.5` | 2.12.5（10/21）；3 alpha 不採 | API `primary_monitor`、`set_position` 官方文件核對；Linux WebKit 另驗 |
| Leptos | `0.8`／`0.8.22` | 0.8.22（10/05） | CSR 不改 |
| SQLx | `0.9`／`0.9.0` | 0.9.0（05/21） | SQLite schema 未改 |
| Tokio | `1.54`／`1.54.1` | 1.54.1（10/21） | 無新 feature |
| UUID | `1.29`／`1.29.0` | 1.29.0（10/23） | 逐 UUID label 保持 |

官方來源：[Rust](https://blog.rust-lang.org/releases/) · [Tauri](https://docs.rs/crate/tauri/latest) · [Tauri API](https://docs.rs/tauri/latest/tauri/struct.AppHandle.html) · [Leptos](https://docs.rs/crate/leptos/latest) · [SQLx](https://docs.rs/crate/sqlx/latest) · [Tokio](https://docs.rs/crate/tokio/latest) · [UUID](https://docs.rs/crate/uuid/latest)。Pet lock blob `d607731dc9af032fe7a6998b806d95b325a4f589`，四 workspace lock 已讀。

## ④ 交付／實測／未完成

**已實際**：精確原始碼／四 workspace lock、正式 JS/Tauri/Leptos 呼叫鏈及官方版本查閱。**未執行**：候選 patch `git apply --check`、Rust/Node tests、native/Linux WebKit、三窗 GUI、正式 CI；Red 是預期編譯失敗，沒有虛構測試結果。Gate1 需獨立 commit 精確讀回及進度同步，Gate2 需使用者套用後以新產品 SHA 驗收。舊 Day 及 FIFO 保留，不新增重構 Day。下一篇優先「直接關生活窗等同回籠」與逐寵物漫遊持久化的最早可交付切片。
