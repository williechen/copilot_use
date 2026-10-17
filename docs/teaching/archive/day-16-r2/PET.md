# 2026-10-17｜P0 Pet Day 16｜寵物生活與控制雙視窗

## 1. 學習大綱
修訂 r2（2026-10-17）。來源：`rust_ai_chat/main` `b2733b7f78dce1da1aa07555122384c27109e4e8`；本機工作目錄仍為 `fced2861`，本輪僅 fetch／靜態讀取，未 checkout、未改原始碼。
**狀態：舊版 Day16 的 Red 已部分套用；本次主流程為待套用教學計畫，未提供已驗證的完整實作。** 最新 SHA 的 18 個 checks 為 16 success／2 failure（`test-ui`、`test-stats-server`）；不沿用 Day15 CI 作為本篇通過證據。三篇主體只優先交付可操作的基本流程，搜尋／總數／競態是相關附加工作。

交付流程：開啟寵物生活視窗 → 開啟控制視窗 → 操作既有互動／睡眠／喚醒並看到雙窗一致狀態 → 使用既有聊天 → 在控制窗掃描、閱讀摘要並確認或取消。控制窗的按鈕與長內容必須可見、可捲動，不再擠進寵物小窗。
本機 Tauri IPC 可使用；不要求新增遠端服務。確認目前只是預覽的 UI 確認，不能宣稱實際整理、刪除、Trash 或 restore 已完成。
## 2. 實作內容
### Red：以雙視窗操作驗收
保留已套用的 `day16_scan_generation_tests`（見本節末）。先記錄現有單窗無法完整顯示控制項；新增雙窗開啟、共享 revision、控制窗關閉後可再開、漫遊只移動寵物窗、掃描確認／取消的驗收案例。
### Green：分開生活與控制
1. `apps/desktop_pet/tauri.conf.json` 定義寵物生活窗與控制窗。寵物窗維持輕量呈現；控制窗可調整大小、可捲動。用明確 window label 與前台入口參數判定用途，不能只靠 CSS 隱藏整套控制項。
2. `apps/desktop_pet/src/lib.rs` 保留同一份 `PetMachine`，兩窗使用既有 IPC 讀狀態及送命令，狀態變更向兩窗同步。增加開啟／聚焦控制窗的入口，明確定義關閉及退出行為。
3. `apps/desktop_pet/src/roam.rs` 將漫遊目標指定為寵物窗，避免控制窗跟著走。前台 `main.rs`、`bridge.rs`、`index.html` 與樣式依用途呈現：生活窗顯示寵物與控制入口；控制窗放互動、既有聊天及檔案預覽。
4. 接通現有 `scan_file_preview`、`scan_duplicate_preview`。長摘要換行，確認與取消可操作。將已套用 Red 的 generation 判斷接回兩個 IPC 完成處；取消清除預覽並讓舊結果失效。原生唯讀掃描不會因此中斷，busy 必須在舊請求完成後解除，不能提前宣稱已停止。
5. 狀態與錯誤顯示要讓使用者知道掃描／預覽／確認各階段；不以勾選布林值冒充原生安全授權。
### Refactor：根目錄 tests 的 Pet 批次
`tests/pet/pet_machine.rs`、`day5_codec_tests.rs`、`cleanup_event_contract.rs`、`profile_contract.rs` → `crates/pet_domain/tests/`，保留原檔名；既有 `cleanup_contract.rs` 保留。這些是 domain 契約，不搬進前台或 Tauri crate。

搬移前，以 `tests/Cargo.toml` 的 target、測試名稱與數量建立基準；搬移時保留原斷言，調整目的 package 的 dev-dependencies，再移除原 `[[test]]` 與不再使用的相依。不要拆分原始碼內嵌單元測試。同步 `.github/workflows/test.yml`：目的測試須在具備原 DB／環境設定的 job 執行；不得將需要 DB 的案例直接丟給未配置 DB 的 job。其他根目錄批次尚未完成時保留 `workspace-tests` 與 CI；全部驗證後才移除根目錄測試容器、workspace member 與空目錄，更新 lockfile。測試名稱／數量不得無故減少或重複，既有失敗與功能變更另記，不在純搬移中偷改斷言。

### 舊 Day 更正補充與已套用紀錄
Day15 原文封存。取消競態配合掃描流程處理，但不能取代雙視窗主體。以下 Red 已在來源 SHA 套用，原樣保留；函式與 IPC 接線尚待實作：
```rust
#[cfg(test)]
mod day16_scan_generation_tests {
    use super::accept_scan_result;

    #[test]
    fn canceled_or_superseded_scan_cannot_restore_preview() {
        assert!(accept_scan_result(3, 3));
        assert!(!accept_scan_result(2, 3));
        assert!(!accept_scan_result(3, 4));
    }
}
```
## 3. 套件版本
沿用來源 workspace 0.2.15／edition 2024；Tauri 宣告 2.12、tauri-build 2.7、Leptos CSR 0.8。精確版本依來源 Cargo.lock。雙視窗不要求新增遠端 IPC 套件；未核實官方最新 stable 或 MSRV。
## 4. 驗證項目
`cargo test -p desktop-pet --lib`；`cargo test -p desktop-pet-frontend day16_scan_generation_tests`；`cargo check -p desktop-pet-frontend --target wasm32-unknown-unknown`；搬移後 `cargo test -p pet-domain`。前台 host 測試若受 WASM 限制須改用可執行 target 並記錄，不能虛報通過。原生驗收：雙窗、狀態同步、控制項可見、聊天、掃描／確認／取消、控制窗重開、漫遊目標。
本輪只核對來源、已套用 Red 與遠端 CI，**上述新主流程、Green、tests 搬移及驗收命令均未執行**。來源 Red 的預期失敗與非預期錯誤要分開記錄；修正後重新檢查同一 SHA 的結果，不能刪除測試讓 CI 變綠。完成條件是主流程可操作及相關驗證通過，不是只有頁面發布或 helper 測試通過。
