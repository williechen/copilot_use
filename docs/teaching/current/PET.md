# 2026-10-17｜P0 Pet Day 15｜掃描後確認與取消流程

來源文章：[Notion 原頁](https://app.notion.com/p/3fbe51d4509381d68666d3031d1c8011)。

## 1. 學習大綱
主體 Red 5 分＋Green 10 分。接續既有「授權目錄 → 掃描／SHA256 → 摘要」，完成使用者可操作的確認／取消與重新掃描流程。確認的是目前預覽，不等於已執行檔案操作。
來源：rust_ai_chat/main `3e58e650bd63bcd21ba58e7aabf751542401ef80`；2026-10-17 重新讀取 main、相關 source／呼叫端及 CI，六個 workflow 均 success。這是既有來源 CI，不能驗證下列教學增量。copilot_use 教學分支基準 `adad4f26964b2518f47a9fde76544b544cd44334`；產品 `9f632c76ee432d77c1d20f46da9485d982d49c9e` 僅沿用舊靜態證據，本次未重新驗收產品。修訂版 Day15-r3（補齊舊 tests 搬移）：沿用原頁，原頁標記未套用且最新 main 缺少本次入口，故重整未實作內容。程式未套用、測試未執行、GUI／流程未驗收。
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
#### 本日舊 tests 搬移：4 個既有測試（待使用者套用）
本批與本日主功能一起做，不計入 Red＋Green 主體時數。確認預覽與完成真實操作必須分開，這批保留未確認不得完成、失敗不計數與完成只計一次的既有契約。CleanupOperation 是 domain 模型，不代表 OS 收據；不把 domain 測試當 IPC 成功證據。
來源固定為 rust_ai_chat/main `3e58e650bd63bcd21ba58e7aabf751542401ef80`，已核對原測試、owning package Cargo、公開呼叫端與 `.github/workflows/test.yml`；目的檔目前不存在。
- 來源：`tests/pet/cleanup_contract.rs`（舊 target `pet_cleanup_contract`）。
- 目的：`crates/pet_domain/tests/cleanup_contract.rs`（package `pet-domain`，Cargo 自動整合測試 target `cleanup_contract`）。
- 原測試 SHA256：`6f10b065cba9e81f5266d68948ae7a7310818e916925e6e2da2efad9e5ab508e`；移動後檔案內容、函式名稱及斷言逐位元保留，共 4 個 `#[test]`。
先在原位置建立基線（預期 4 個測試；實際未執行）：
```bash
cargo test --locked -p workspace-tests --test pet_cleanup_contract
```
使用者在學習庫套用時，先檢查目的不存在，再移動；本次助手沒有執行以下修改：
```bash
mkdir -p crates/pet_domain/tests
git mv tests/pet/cleanup_contract.rs crates/pet_domain/tests/cleanup_contract.rs
```
目的檔完整最終內容如下（與原檔相同，不另複製保留兩份）：
```rust
use pet_domain::cleanup::{CleanupError, CleanupOperation};
use pet_domain::{OperationState, count_as_cleanup};

#[test]
fn preview_cannot_be_counted_or_completed() {
    let mut operation = CleanupOperation::new();

    // 尚在預覽狀態不得記作清理成功。
    assert!(!operation.may_emit_success_event());
    // 沒有明確確認時，不能跳過確認直接標記完成。
    assert_eq!(
        operation.complete(true),
        Err(CleanupError::ConfirmationRequired)
    );
}

#[test]
fn confirmed_success_counts_only_after_completion() {
    let mut operation = CleanupOperation::new();
    operation.confirm().expect("預覽後應允許確認");

    // 即使已確認，只要未完成仍不能發送成功事件。
    assert!(!operation.may_emit_success_event());
    operation.complete(true).expect("已確認才能完成");
    assert!(operation.may_emit_success_event());

    // 已完成的操作不可再次完成或重複累計。
    assert_eq!(operation.complete(true), Err(CleanupError::AlreadyFinished));
}

#[test]
fn failed_operation_never_counts() {
    let mut operation = CleanupOperation::new();
    operation.confirm().expect("預覽後應允許確認");

    // 實際清理失敗時不得發送成功事件。
    operation.complete(false).expect("已確認後可記錄失敗");
    assert!(!operation.may_emit_success_event());
}

#[test]
fn four_legacy_operation_states_preserve_counting_rule() {
    // 將舊 tests/pet/operation_event_contract.rs 的唯一不同斷言合併到此。
    assert!(!count_as_cleanup(OperationState::Previewed));
    assert!(!count_as_cleanup(OperationState::Confirmed));
    assert!(!count_as_cleanup(OperationState::Failed));
    assert!(count_as_cleanup(OperationState::Succeeded));
}
```
在 `tests/Cargo.toml` **只刪除**本批的完整 target 區塊：
```toml
[[test]]
name = "pet_cleanup_contract"
path = "pet/cleanup_contract.rs"
```
owning package 的 `Cargo.toml`：N/A，原有依賴已涵蓋本檔 imports，且沒有 autotests=false，Cargo 自動發現目的 tests；不加重複 \[\[test\]\]，不加平行 workspace。根 `tests/Cargo.toml` 的其他 targets 與 dev-dependencies 保留；根 `Cargo.toml` 的 "tests" member 保留。依賴圖未改，本批 Cargo.lock 預期 N/A；若 --locked 指出 lock 不一致，先查明原因，不能改用非 locked 掩蓋。
CI 已核對：`.github/workflows/test.yml::test-pet-domain` 的 `cargo test --locked -p pet-domain` 自動收錄新位置；`test-workspace` 繼續測其餘根 targets。workflow 本批 N/A，不能刪除 workspace-tests job。保留 [cleanup.rs](http://cleanup.rs) 內嵌測試與根 pet_cleanup_event_contract 等未搬 targets；本批不重寫斷言、不把同名場景直接刪掉當去重。
搬完後執行（以下全部待執行）：
```bash
cargo test --locked -p pet-domain --test cleanup_contract
cargo test --locked -p pet-domain
cargo test --locked -p workspace-tests
cargo fmt --all -- --check
git diff --check
git diff --summary
```
核對 4 個測試名稱／斷言不減少，舊 target 已取消且來源不再留一份、新 target 只收錄一次。三篇全部套用後，根 manifest 從 12 個 target 減至 9 個；剩餘 9 個與 CI 保留，待後續相關功能分批搬完並驗證後才移除根 tests。不以「檔案已移動」或既有 CI 代替搬後測試通過。
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
本輪 r3 文件重建驗證：已靜態核對本批來源／目的、4 個測試及 Cargo／CI 收錄；實際搬移、搬前與搬後測試均未執行，rust_ai_chat 保持唯讀。既有主功能程式與 Pet 本機局部證據沿用 r2，不因本輪文件修訂視為已套用或重新驗收。
