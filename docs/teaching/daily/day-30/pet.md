# Pet Day30 r2｜回籠關窗失敗時補償 SQLite 標記（附 Day29 未完成補教）

## ① 學習大綱、來源與真實現況

寫入文件正本：`williechen/copilot_use/main`；唯讀程式：`williechen/rust_ai_chat/main@9c2b7cf5a4c60a08254445911916c4ed9b02d7b3`；共同規則 `docs/COMMON_LEARNING_RULES.md`（daily-tdd-minimum-v1／github-only-v1）。已核對 `apps/desktop_pet/src/pet_windows.rs::cage_saved_pet`：Day29 的「先 `store.set_caged(&id,true)`、再 `window.close()`、再移除 Runtime」已進入現行程式；但如果 SQLite 寫入成功而 `window.close()` 回傳錯誤，持久狀態為回籠，生活視窗卻還開著。不可假裝 Day29 尚未套用。

本日最小切片：Leptos「回籠」→ `bridge::cage_saved_pet` → Tauri command → `PetStore::profile` → SQLite `set_caged(true)` → `window.close()` → （失敗時 `set_caged(false)` 補償並保留 Runtime；成功才 remove）。各寵物以 ID 個別處理，Chat／Pet SQLite 與 Analytics PostgreSQL 保持分離。教學時間 Chat／Pet／Analytics 各 15 分鐘，共用 15 分鐘，合計 60 分鐘；原生 GUI/DB 整合驗收額外計時。

## ② Red → 完整最小 Green → Refactor → Day29 未完成補教

**Red（由精確來源推導，未執行）：** 用測試用 SQLite 建立、開啟具 ID 的已儲存寵物；使 Tauri 關窗動作回傳錯誤。現行 `cage_saved_pet` 在 `window.close()?` 前已標記 `is_default_closure=true`，因此 DB 與仍開啟的生活窗不一致。預期應嘗試恢復 `false`，保留 Runtime；如果恢復也失敗，回傳包含兩個錯誤的訊息而不吞錯。測試需能注入關窗失敗，不能僅藉由 helper 單元測試宣稱原生 GUI 已驗收。

**Green（完整替換 `apps/desktop_pet/src/pet_windows.rs` 現有函式）：**

```rust
#[tauri::command]
pub async fn cage_saved_pet(
    id: String,
    app: tauri::AppHandle,
    store: State<'_, PetStore>,
) -> Result<(), String> {
    store.profile(&id).await?;
    let window = app.get_webview_window(&label(&id)?);

    // 儲存失敗不能先關窗，也不能清除 Runtime。
    store.set_caged(&id, true).await?;
    if let Some(window) = window {
        if let Err(close_error) = window.close() {
            // 關窗未成功，盡可能恢復 DB 的出籠標記。
            let original = close_error.to_string();
            return match store.set_caged(&id, false).await {
                Ok(()) => Err(format!("關閉寵物視窗失敗，已恢復出籠狀態：{original}")),
                Err(rollback) => Err(format!(
                    "關閉寵物視窗失敗：{original}；回復資料庫亦失敗：{rollback}"
                )),
            };
        }
    }
    app.state::<Runtime>().remove(&id);
    Ok(())
}
```

上述修改只適用**同步回報的關窗失敗**，不宣稱 SQLite/Tauri 具備跨系統原子交易。若無視窗仍成功標記回籠，是既有冪等行為；若視窗關閉成功後有非同步事件失敗，本切片不能處理。為使 Red/Green 真實可執行，測試需在隔離 App 測試基座以真實 Tauri Window、可注入的關窗失敗 adapter 跑完，或先把關窗結果與補償計畫抽為無 GUI 的可測 port 再跑整合；**本次未建立測試基座，未執行 Red/Green**。在已有 UI/IPC/SQLite 以外不得創造第二套資料表。

**Refactor：** 保留 `PetStore::set_caged` 單一寫入入口及 `Runtime::remove` 僅於成功路徑；不要直接從 Leptos 存取 SQLite。額外建議將關窗與 DB 補償封裝為 application workflow（以 port 注入）以驗證失敗順序；未套用前不宣稱 Refactor 完成。

### Day29 未完成項目補充教學

Day29 已提示 SQLite 成功但 `window.close()` 失敗造成不一致，本篇提供上述**可套用補償方案**，但未實跑，不結案。仍須逐一在測試用 DB/原生 GUI 驗證：① DB 失敗前視窗不關、Runtime 不清；② 關窗失敗後恢復 `is_default_closure=false`；③ 補償也失敗會明確回報雙重錯誤；④ 正常回籠、App 重啟載入；⑤ 三個不同 pet id 的視窗與事件互不干涉。預期見上述路徑與第④節；本次實際皆未執行。更早 FIFO `pet-day25-single-image-20261022` 仍 pending：動畫每幀單一圖像與三個不同 ID 的 GUI 本次沒有完成，**不關閉 makeup_queue**，不回寫 Day25 文章。


### 新共同規則 12 項一致性補充（r2）

**適用政策：** 文件 Gate 1 與程式 Gate 2 分離；`copilot_use/main` 是唯一教學寫入位置，`rust_ai_chat/main` 在本教學操作中唯讀。此篇僅修訂 Day30，不產生 Day31、不得以文章讀回聲稱程式已套用。原始版本 commit、blob 與 SHA256 在 GitHub 歷史及 manifest 持續可追溯。

**增量式呼叫鏈：** 只新增本篇最小差異，沿用精確來源 SHA 已存在的 Leptos UI、Axum／Tauri IPC、Application／Domain 與 SQLx；完整鏈路用以確認整合上下文，不要求每天各層均重新編寫。UI 以 Leptos 元件為準，Axum 可以包裝 Leptos SSR HTTP Response，但不得手寫正式頁面取代。

**雙軌補教追蹤：** 先接續 Day29 第④節未驗收內容，核對具體路徑／測試／預期與實際；同時保持更早 makeup_queue FIFO，最多結案一筆舊 Day，且須具備對應的 Gate 2 證據。追蹤、文件更正、補教討論均不自動結案。當日未完成部分下一 Day 第②節再補充，不跳號或重覆課題。

**估時與封存：** 核心 60 分鐘包括每案 Red→Green→Refactor 各 15 分鐘及共用 15 分鐘；補教、版本核對、文件 ①③④、GitHub 提交／讀回與 Gate 2 測試額外計時。Gate 2 尚未通過，不得搬至 `docs/teaching/archived/day-30/pet.md`；依新規則只有必要測試與驗收成立後才將**正文原樣**移至 archived 並同步 manifest／連結。週六完整比對目前不執行、`last_full_comparison` 不更新；這不代表已停用任何實際排程。

**本次修訂驗證結果：** 此次是 GitHub 教學文本重交付；尚未修改 `rust_ai_chat`、尚未實際執行 Day30 提案的 Red／Green、HTTP、資料庫或 GUI 驗收，功能狀態仍為 pending。

## ③ 套件版本

沿用程式既有 `tauri` 2.x、`sqlx`、`leptos` 與 SQLite adapter，不增加相依、不修改 Cargo.lock 或 migration。實際 resolved patch 版本、MSRV 與原生 GUI 作業系統版本本次未查明；不能以 workspace manifest 當成 lockfile 版本。

## ④ 驗證命令、預期／實際、CI、證據與限制

| 命令或測試流程 | 預期 | 本次實際 |
| --- | --- | --- |
| `cargo check --locked -p desktop_pet` | command 可編譯 | 未執行 |
| `cargo test --locked -p desktop_pet` | 原有資料／多寵測試不退化 | 未執行 |
| SQLite `set_caged(true)` 注入失敗 | 視窗／Runtime 保留 | 未執行 |
| 模擬 `window.close()` 失敗 | 嘗試 `set_caged(false)`、回傳錯誤 | 未執行 |
| DB 補償失敗 | 同時揭露關窗／DB 錯誤 | 未執行 |
| 三個不同 UUID 寵物原生視窗／每幀單一圖像 | 分別顯示／事件隔離 | 未執行 |
| 重啟 Tauri App 載入 | DB 與實際 GUI 一致 | 未執行 |

精確來源 SHA `9c2b7cf5a4c60a08254445911916c4ed9b02d7b3`：Rust Desktop Pet workflow run `39891849805` 為 **success**，Test All `39891849769` 為 success，Node `39891849761` 為 failure；無法據此宣稱本 Day 的補償程式已被 CI 執行或 GUI 驗收。未操作任何使用者 DB、未部署、未寫入 `rust_ai_chat`；文件交付與產品驗收分開認定。
