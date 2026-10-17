# 2026-10-17｜P0 Pet Day 15｜掃描後確認與取消流程

來源文章：[Notion 原頁](https://app.notion.com/p/3fbe51d4509381d68666d3031d1c8011)。

## 1. 學習大綱
主體 Red 5 分＋Green 10 分。接續既有「授權目錄 → 掃描／SHA256 → 摘要」，完成使用者可操作的確認／取消與重新掃描流程。確認的是目前預覽，不等於已執行檔案操作。
來源：rust_ai_chat/main `3e58e650bd63bcd21ba58e7aabf751542401ef80`；2026-10-17 重新讀取 main、相關 source／呼叫端及 CI，六個 workflow 均 success。這是既有來源 CI，不能驗證下列教學增量。copilot_use 教學分支基準 `adad4f26964b2518f47a9fde76544b544cd44334`；產品 `9f632c76ee432d77c1d20f46da9485d982d49c9e` 僅沿用舊靜態證據，本次未重新驗收產品。修訂版 Day15-r2：沿用原頁，原頁標記未套用且最新 main 缺少本次入口，故重整未實作內容。程式未套用、測試未執行、GUI／流程未驗收。
已讀 `apps/desktop_pet/frontend/src/main.rs`、bridge、Tauri `scan_file_preview`／`scan_duplicate_preview` 與 `file_organizer.rs`：目前只有預覽 IPC，沒有檔案移動／Trash／restore command；frontend 摘要丟棄 opaque ID。不能把既有 CleanupOperation domain 狀態當真實 filesystem receipt。
## 2. 實作內容
### Red
手動驗收腳本：啟動 → 掃描 → 顯示摘要 → 確認 → 顯示確認狀態 → 取消 → 重掃後回到未確認。目前缺少確認／取消入口，預期 UI Red，尚未執行；不把已有摘要單測冒稱新功能 Red。
### 最小 Green
只修改 `apps/desktop_pet/frontend/src/main.rs::App`，沿用 scan_status／duplicate_status 與 loading signals，bridge／Tauri／Domain／DB N/A。signals 區塊新增：
```rust
// 放在 App 的 signals 區塊；確認只針對目前顯示的掃描結果。
let (confirmed, set_confirmed) = signal(false);
```
兩個既有掃描按鈕 on:click 通過 loading guard 後，均先加入以下完整重設片段，再執行原有非同步掃描；其餘 IPC 保留：
```rust
// 新掃描一律使前一份確認失效，也不保留另一類舊摘要。
set_confirmed.set(false);
set_scan_status.set(None);
set_duplicate_status.set(None);
```
在兩個預覽 Show 後、ErrorMessage 前新增：
```rust
<button type="button"
    disabled=move || scan_loading.get() || duplicate_loading.get()
        || (scan_status.get().is_none() && duplicate_status.get().is_none())
    on:click=move |_| set_confirmed.set(true)
>"確認本次預覽"</button>
<button type="button"
    disabled=move || scan_loading.get() || duplicate_loading.get()
    on:click=move |_| {
        set_confirmed.set(false);
        set_scan_status.set(None);
        set_duplicate_status.set(None);
    }
>"取消／清除本次預覽"</button>
<Show when=move || confirmed.get() fallback=|| ()>
    <p role="status">"已確認本次預覽；尚未執行移動、刪除或還原。"</p>
</Show>
```
呼叫鏈：主動掃描按鈕 → bridge → Tauri → authorized_root → scan／SHA256 → 純文字摘要 → 使用者確認／取消 → Leptos 畫面狀態。loading 時禁用確認／取消，兩個 scan guard 保留，避免對未完成掃描確認；新掃描先失效舊確認。
### Refactor
本日完成可操作的「掃描 → 確認／取消」段落。真實檔案操作仍是同一功能的未完成下一段：後端保存 scan session／opaque ID、明確選擇項目、確認綁定不可重放的計畫、操作前 root／symlink／TOCTOU／權限重驗、journal／private Trash／receipt／restore，最後才上報真實成功事件。不得直接將目前摘要字串送進刪除函式，也不得由確認按鈕發出成功 Analytics。
此處阻礙是真實 OS adapter／IPC 尚不存在，不因加上確認 UI 宣稱完整清理功能完成。先以專用測試檔案設計後端與還原，再在後續 Day 提案；不在本輪修改參考庫原始碼。
### 舊 Day 更正補充
已實作 Day14 保持封存；原未套用 Day15 的清除按鈕併入取消流程，同一頁修訂，不新增 Day。若使用者已在本機套用某段但尚未推到 main，該段應保留封存，修改改由下一 Day 提出；本次靜態判定只涵蓋遠端 main。
### 本機產品 IPC 局部驗證（2026-10-17）
已用本機 copilot_use 的既有原生 Release 執行 `--acceptance-gui`，設定全新測試資料目錄／授權測試目錄與 `PET_GUI_TEST_FUNCTIONAL_ONLY=true`，不執行 AI。測試腳本實際經 WebKit → Tauri invoke 呼叫 `scan_duplicate_preview`、`scan_file_preview`、`confirm_cleanup`、`restore_file`；專用 SQLite 讀回兩筆 cleanup_receipts、一筆 restore_receipts，sample.txt 內容還原正確，second.txt 留在專用隔離區。
既有 binary SHA256：`24073b42a8717b355ec7d906340b0eb257c8571bfd1e25bff889f64be5f1762f`；本機 source HEAD `6256adec302b5f633568a89f4a4e48047922765d`，未重建，binary 與此 source 的精確對應尚未核實。整套驗收未產出最終 GUI 報告，取得目標收據後已停止程序；只記局部 IPC／檔案證據，不記整套通過。
產品 IPC 的逐檔 opaque ID／confirmed 與 receipt 可作學習流程參照；rust_ai_chat 最新 main 沒有上述操作 IPC，不能只加前台 invoke 就當已接通，仍須由使用者在後續教學實作後端。本次 Day15 新增確認 UI 的編譯與 GUI 測試仍未執行。證據另存 GitHub `docs/teaching/evidence/2026-10-17-pet-local-ipc.json`。
## 3. 套件版本
已讀 frontend Cargo：Leptos 0.8 CSR、serde 1.0、serde-wasm-bindgen 0.6、wasm-bindgen 0.2、web-sys 0.3；本日不新增依賴。lock／官方 stable／MSRV／安全公告未重新核實；native／WASM／Tauri GUI 未執行。
## 4. 驗證項目
待執行：`cargo test --manifest-path apps/desktop_pet/frontend/Cargo.toml`、`cargo check --manifest-path apps/desktop_pet/frontend/Cargo.toml --target wasm32-unknown-unknown`。
GUI：無摘要不能確認；一般／SHA256 掃描均可確認；掃描中禁用；取消清掉兩類摘要；重掃／失敗使前次確認失效；顯示繁中固定文案且無私人路徑。確認／取消前後測試目錄內容完全一致，不出現檔案成功收據或 Analytics 成功事件。命令、GUI、filesystem 回歸全部未執行；真實操作／還原／跨服務仍未接通，不能標正式驗收。
