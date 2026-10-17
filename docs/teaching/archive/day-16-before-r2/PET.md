## 1. 學習大綱
來源：`williechen/rust_ai_chat/main` `fced2861bba21c33697051f685c50689f8052de8`；同 SHA 六個 workflows / 18 checks Success（[CI](https://github.com/williechen/rust_ai_chat/actions/runs/38992992860)）。
狀態：**教學待套用**；本篇程式與測試**未實際編譯、未執行**；不能把來源 CI 視為本篇驗收。時間：Red＋最小 Green 20 分鐘。
承接 Day 15 掃描→確認／取消的前台，修補真實使用者操作中的競態：**取消掃描後，較慢返回的舊 IPC 結果不可重新出現在畫面上**。此為 FUTURE_WORK Pet 安全預覽／確認鏈的必要阻塞修正，不實作刪除、Trash 或 restore。檔案：`apps/desktop_pet/frontend/src/main.rs`；保留 `bridge::scan_file_preview`、`bridge::scan_duplicate_preview` 呼叫端。
## 2. 實作內容
### Red（8 分鐘）
在檔案 `main` 前新增以下**純函式與測試**，先只貼測試可見編譯 Red：
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
### 最小 Green（12 分鐘）
在 `main` 前新增：
```rust
// 僅當 IPC 回傳屬於最新一輪使用者操作時，才能更新畫面。
fn accept_scan_result(request_generation: u64, current_generation: u64) -> bool {
    request_generation == current_generation
}
```
在 `App` 既有 signals 區塊中 `confirmed` 後新增：
```rust
let (scan_generation, set_scan_generation) = signal(0_u64);
```
在**兩個掃描按鈕**的 `on:click` 處理中，開始 `spawn_local` **之前**各新增：
```rust
// 新請求使舊的掃描結果失效；這個值必須在 await 前擷取。
let generation = scan_generation.get_untracked().wrapping_add(1);
set_scan_generation.set(generation);
```
第一個掃描按鈕原 `spawn_local` 區塊完整替換為：
```rust
spawn_local(async move {
    let result = bridge::scan_file_preview().await;
    if !accept_scan_result(generation, scan_generation.get_untracked()) {
        return;
    }
    match result {
        Ok(summary) => set_scan_status.set(Some(summary)),
        Err(_) => set_error.set(Some("掃描失敗，請確認已授權目錄後重試。".into())),
    }
    set_scan_loading.set(false);
});
```
第二個重複檔案掃描按鈕原 `spawn_local` 區塊完整替換為：
```rust
spawn_local(async move {
    let result = bridge::scan_duplicate_preview().await;
    if !accept_scan_result(generation, scan_generation.get_untracked()) {
        return;
    }
    match result {
        Ok(summary) => set_duplicate_status.set(Some(summary)),
        Err(_) => set_error.set(Some("重複檔案預覽失敗，請重試。".into())),
    }
    set_duplicate_loading.set(false);
});
```
將**取消／清除本次預覽**按鈕的 `on:click` 完整替換為：
```rust
on:click=move |_| {
    set_scan_generation.update(|generation| *generation = generation.wrapping_add(1));
    set_scan_loading.set(false);
    set_duplicate_loading.set(false);
    set_confirmed.set(false);
    set_scan_status.set(None);
    set_duplicate_status.set(None);
    set_error.set(None);
}
```
並移除取消按鈕原本的 `disabled=move || scan_loading.get() || duplicate_loading.get()`，使使用者能在 IPC 尚未返回時取消；掃描請求本身不被中斷，只忽略舊結果。流程：點掃描→generation 遞增→IPC await→僅最新 generation 更新摘要；取消→generation 遞增並清除→舊結果回來也不更新畫面。
### Refactor
### 根目錄 tests 遷移計畫
範圍僅為根目錄 `tests/pet/`：`pet_machine.rs`、`day5_codec_tests.rs`、`cleanup_event_contract.rs`、`profile_contract.rs` → `crates/pet_domain/tests/`，保留原檔名。這四份目前都使用 pet-domain 公開契約；不搬到 Tauri 原生或前台目錄。既有 `cleanup_contract.rs` 保留並比對案例，不能只因主題相近就刪除。
遷移是配合本日主功能的整理工作，**不拆分各原始檔內既有單元測試，不要求每個檔案另建 **[**tests.rs**](http://tests.rs)。
Red：以 `tests/Cargo.toml` 的九個明確 `[[test]]` 為基準，記錄測試名稱、數量、既有失敗與環境需求。Green：搬移上述檔案，將所需 dev-dependencies 加到目的 package，移除 workspace-tests 對應登記及不再使用的相依；維持原斷言。Refactor：將 `.github/workflows/test.yml` 的 `cargo test --locked -p workspace-tests` 改為目的 package 的測試命令，確保 CI 持續收集全部案例。新增功能測試與純搬移分開提交／驗證。
三專案搬完後再統一檢查：根目錄 `tests/common/mod.rs` 目前為空；確認無剩餘共用或跨專案測試及引用後，才移除 `tests/Cargo.toml`、工作區 `members` 的 `tests` 與空目錄，並更新 lockfile。若仍有真正跨專案測試則保留其 workspace-tests 容器與 CI，不強制清空。
驗證：`cargo test -p pet-domain --test pet_machine --test day5_codec_tests --test cleanup_event_contract --test profile_contract`；搬移前後比對案例，不能出現漏跑、重複、無故 ignored 或以刪測試取得通過。**目前僅更新教學計畫；尚未搬移、未執行上述驗證。**
原 Day 15 `confirmed` 是布林 UI 訊號，不是持久化安全憑證；本日只處理非同步畫面競態。測試歸屬 Pet frontend；是否可由 host `cargo test` 直接編譯需按 frontend target 確認。**安全注意：取消不會停止已送出的原生 IPC；目前只做唯讀預覽，後續若新增實際檔案操作，必須由 backend 驗證 opaque preview ID、root、symlink、檔案內容與一次性確認，不可只信前台 generation。**
### 舊 Day 更正補充
Day 15 已有確認／取消 UI 並有來源 CI Success，保持封存；補正「取消時不能取消進行中的掃描」造成的過期回覆顯示風險，不重寫 Day 15，也不聲稱已完成真實檔案操作。
## 3. 套件版本
只使用既有 `leptos`、`tauri` 橋接與 Rust `u64`，不新增套件／migration。官方 stable／MSRV／WASM build **未查證**。
## 4. 驗證項目
1. Red：新增測試先失敗；**未執行**。
2. Green：`cargo test -p desktop-pet day16_scan_generation_tests`（實際 frontend package 名稱需依 manifest 校正）、frontend WASM check；**未執行**。
3. 手動：掃描途中取消→舊結果不得復活；取消後立即重掃→僅最新摘要；重複掃描→清除確認；**未執行**。
4. 原生 OS 檔案操作、GUI 與 receipt／restore：**不在本 Day，未驗收**。
5. 來源 CI 成功僅涵蓋 `fced2861`，不涵蓋本篇新程式。
