# 2026-10-16｜P0 Pet Day 13｜重複預覽顯示群組大小與檔案數

來源文章：[2026-10-16｜P0 Pet Day 13｜重複預覽顯示群組大小與檔案數](https://app.notion.com/p/3fae51d45093814dbdabee6deac48554?pvs=204)。

## 1. 學習大綱
P0／Pet Day 13。主體實作 15 分鐘；Refactor／必要補加不計入學習時數。
來源：rust_ai_chat/main `2aecb0f02f9176287e848b7e4897f28ef3929072`；copilot_use/main `49e1d946dd3c22ffcc4b1aa0e7715cb5b1686724`；產品教學分支 `1d1668b97e65ccfafc073829f131c93133c8bb7e`。已讀共同規則、P9_TARGET、FUTURE_WORK、SYSTEM_ARCHITECTURE、learning-state 與 Notion 共同／專案／Day 12；共同規則為準。當前 P0，最晚切點 2026-12-12，未達切點也無提前完成證據，不切換階段。首次週六完整比對尚未成功，last_full_comparison 保持 null；本日只核對選題相關差異，不冒稱全量完成。
對應 FUTURE_WORK 原「FileOrganizer SHA256 重複檔案分組／IPC／UI 接線」及「使用者授權、型別化 preview／confirmation／receipt」項目。最新 file_[organizer.rs](http://organizer.rs) 已回 DuplicatePreview\{groups,warnings\}，每組含 sizeBytes／itemIds；Day 11 IPC、Day 12 漫遊 opt-in 都已套用。本日不重做掃描／SHA256，而是讓 UI 顯示每組的檔案大小及數量，並避免舊結果誤作本次結果。
成品 pet_runtime 的 private directory／no-follow／receipt 是行為及安全目標；成品 SHA256 分組仍列未完成，不能聲稱已有成品等價。保留學習庫既有 Tauri → JS → WASM 路線與無路徑參數命令，不把成品檔名硬搬過來。
## 2. 實作內容
### Red：加入顯示契約測試
新增 `apps/desktop_pet/frontend/src/duplicate_preview.rs`。先只加入下方 tests 模組，執行第 4 節第一條命令，預期缺 DuplicatePreviewView 而編譯失敗；再加入上方型別及 impl 完成 Green。以下是最終完整檔案，避免散落重複實作：
```rust
use serde::Deserialize;

// 僅保留顯示所需資料，不持有 hash、警告本文或私人路徑。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateGroupView {
    size_bytes: u64,
    item_ids: Vec<serde::de::IgnoredAny>,
}

#[derive(Debug, Deserialize)]
pub struct DuplicatePreviewView {
    groups: Vec<DuplicateGroupView>,
    warnings: Vec<serde::de::IgnoredAny>,
}

impl DuplicatePreviewView {
    pub fn summary(&self) -> String {
        let mut text = format!(
            "SHA256 重複群組：{}；掃描警告：{}。僅供預覽，尚未移動或刪除檔案。",
            self.groups.len(), self.warnings.len()
        );
        for (index, group) in self.groups.iter().enumerate() {
            text.push_str(&format!(
                "\n群組 {}：每檔 {} 位元組，{} 個檔案。",
                index + 1, group.size_bytes, group.item_ids.len()
            ));
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_shows_group_sizes_without_private_paths() {
        let value: DuplicatePreviewView = serde_json::from_str(r#"{
            "groups": [
                {"sizeBytes":4,"itemIds":["item-000000","item-000001"],
                 "digestSha256":"private-hash"},
                {"sizeBytes":0,"itemIds":["item-000002","item-000003","item-000004"]}
            ],
            "warnings":[{"relativePath":"private.txt","message":"private-error"}]
        }"#).unwrap();
        let text = value.summary();
        assert!(text.contains("SHA256 重複群組：2；掃描警告：1"));
        assert!(text.contains("群組 1：每檔 4 位元組，2 個檔案"));
        assert!(text.contains("群組 2：每檔 0 位元組，3 個檔案"));
        assert!(text.contains("尚未移動或刪除"));
        for private in ["private.txt", "private-error", "private-hash", "item-000000"] {
            assert!(!text.contains(private));
        }
    }

    #[test]
    fn empty_preview_is_explicit_and_missing_contract_is_rejected() {
        let value: DuplicatePreviewView =
            serde_json::from_str(r#"{"groups":[],"warnings":[]}"#).unwrap();
        assert_eq!(value.summary(),
            "SHA256 重複群組：0；掃描警告：0。僅供預覽，尚未移動或刪除檔案。");
        assert!(serde_json::from_str::<DuplicatePreviewView>(
            r#"{"groups":[{"itemIds":[]}],"warnings":[]}"#
        ).is_err());
    }
}
```
在 `apps/desktop_pet/frontend/Cargo.toml` 檔尾新增（若已有 dev-dependencies，合併一次）：
```toml
[dev-dependencies]
serde_json.workspace = true
```
此套件已在 workspace／lock，沒有引入新版本。[main.rs](http://main.rs) 頂端既有 mod bridge／mod component 後加入：
```rust
mod duplicate_preview;
```
### 最小 Green：將資料接到真實按鈕
`apps/desktop_pet/frontend/src/bridge.rs` 刪除舊 DuplicatePreviewCounts 型別，將既有 scan_duplicate_preview 完整替換為：
```rust
pub async fn scan_duplicate_preview() -> Result<String, String> {
    let value = JsFuture::from(scan_duplicate_preview_js())
        .await.map_err(js_error)?;
    let result: crate::duplicate_preview::DuplicatePreviewView =
        serde_wasm_bindgen::from_value(value).map_err(|e| e.to_string())?;
    Ok(result.summary())
}
```
保留 extern 中 scan_duplicate_preview_js 及其他 bridge 函式。`apps/desktop_pet/frontend/src/main.rs`::App 在 duplicate_status signal 後新增：
```rust
let (duplicate_loading, set_duplicate_loading) = signal(false);
```
將原「檢查重複檔案」按鈕及緊接其後顯示 duplicate_status 的 Show 整段替換為以下內容；保留後面的 ErrorMessage：
```rust
<button
    type="button"
    disabled=move || duplicate_loading.get()
    on:click=move |_| {
        // 清除過時預覽；同時只允許一次使用者請求。
        if duplicate_loading.get_untracked() { return; }
        set_duplicate_loading.set(true);
        set_duplicate_status.set(None);
        set_error.set(None);
        spawn_local(async move {
            match bridge::scan_duplicate_preview().await {
                Ok(summary) => set_duplicate_status.set(Some(summary)),
                Err(message) => set_error.set(Some(message)),
            }
            set_duplicate_loading.set(false);
        });
    }
>"檢查重複檔案"</button>
<Show when=move || duplicate_loading.get() fallback=|| ()>
    <p role="status">"正在檢查重複檔案…"</p>
</Show>
<Show when=move || duplicate_status.get().is_some() fallback=|| ()>
    <pre class="duplicate-preview" role="status">
        {move || duplicate_status.get().unwrap_or_default()}
    </pre>
</Show>
```
`apps/desktop_pet/frontend/style.css` 檔尾追加（相同 selector 若已存在則合併）：
```css
.duplicate-preview {
    max-width: 100%;
    max-height: 5rem;
    overflow: auto;
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    font: inherit;
    margin: 0;
}
```
### Refactor
將預覽顯示整理為三個職責：`duplicate_preview.rs` 的 `summary` 產生純文字摘要；`bridge.rs` 反序列化 IPC 結果；`App` 管理載入、錯誤與畫面。顯示模型僅保留大小與數量，避免 hash、opaque ID 及警告路徑進入畫面。
#### 整合現有測試：移除 workspace 根目錄 tests
本項是現有測試的歸屬整合與重構，加入既有 Day 13 第 2 節 Refactor；不計為新功能、不推進 Day，也不占三專案主體合計 60 分鐘。以下依唯讀參考庫 `3e58e650bd63bcd21ba58e7aabf751542401ef80` 盤點，由你在 `/home/willie/Documents/rust_ai_chat` 套用。原有測試內容、斷言與產品行為保持；依序完成 Chat、Pet、Analytics 的搬移後，才移除共同測試套件。
先在任何搬移前，於 workspace 根目錄保存原測試清單：
```bash
cargo test --locked -p workspace-tests -- --list
```
本篇負責下列檔案，執行位置為 workspace 根目錄：
```bash
mkdir -p crates/pet_domain/tests
git mv tests/pet/cleanup_contract.rs crates/pet_domain/tests/cleanup_contract.rs
git mv tests/pet/cleanup_event_contract.rs crates/pet_domain/tests/cleanup_event_contract.rs
git mv tests/pet/day5_codec_tests.rs crates/pet_domain/tests/day5_codec_tests.rs
git mv tests/pet/pet_machine.rs crates/pet_domain/tests/pet_machine.rs
git mv tests/pet/profile_contract.rs crates/pet_domain/tests/profile_contract.rs
```
五個檔案都驗證 pet-domain 的既有行為，歸入該 crate 的整合測試；沿用 serde_json、uuid 等既有依賴。frontend 與 desktop-pet 各自已有的測試留在原處，不集中搬入另一個共用 tests 套件。
三篇搬移全部完成後，在同一次重構中收尾：
1. 根 `Cargo.toml` 的 `workspace.members` 移除 `"tests"`；移除 `tests/Cargo.toml` 的 `workspace-tests` 套件。新位置採 Cargo 自動探索整合測試，不要搬回原本 `autotests = false` 或舊 `[[test]]` 清單。
2. `.github/workflows/test.yml` 原 `cargo test --locked -p workspace-tests` 改為下列四個擁有測試的套件；保留既有測試環境設定與其他工作，不新增本機 Docker 需求。
```bash
cargo test --locked -p chat-application -p pet-domain -p server -p stats-server --tests
```
1. 確認 `tests/common/mod.rs` 仍為空檔、`tests/analytics/postgres_ingest.rs` 仍只有空白後，以 `git rm` 移除這兩個空檔及 `tests/Cargo.toml`，再以 `rmdir` 移除殘留空目錄。若檔案已有內容，先整合到對應套件，不可直接刪除；所有有斷言的檔案必須先完成搬移。
2. 執行 `cargo metadata --offline --format-version 1 --no-deps > /dev/null` 讓 Cargo 同步套件關係與 lock；若需要鎖檔解析，再執行 `cargo generate-lockfile --offline`，檢查 `Cargo.lock` 僅移除 `workspace-tests` 及重接 `stats-server` 的既有 tower 依賴。若有無關版本變動，先還原該次 lock 變動並排除解析原因；不使用 `cargo update` 升級套件。
3. 在原生本機測試環境先用 `cargo test --locked -p workspace-tests -- --list` 保存搬移前清單；搬移後以四套件的 `--tests -- --list` 比對函式名稱、數量與 ignored 狀態，再執行測試。這條搬移前命令必須在第 1 步之前執行。
```bash
cargo test --locked -p chat-application -p pet-domain -p server -p stats-server --tests -- --list
cargo test --locked -p chat-application -p pet-domain -p server -p stats-server --tests
cargo fmt --all -- --check
test ! -d tests
rg -n 'workspace-tests|tests/(chat|pet|analytics|server)/' Cargo.toml Cargo.lock .github
```
最後一條預期沒有舊套件／路徑命中（`rg` 無命中退出碼為 1）；不能只以整體綠燈認定測試未遺漏。四套件可能同時列出既有其他測試，應逐一確認這次搬移的 11 個非空測試檔都有被探索，斷言逐位元保留。Analytics HTTP 契約測試仍需原有 PostgreSQL 與 `ANALYTICS_DATABASE_URL`、`ANALYTICS_CHAT_TOKEN`、`ANALYTICS_PET_TOKEN`；使用本機測試資料庫與合成 Token，不指向正式資料或讀取私人 `.env`。原有 `postgres_event_store` 測試及其 feature gate 保留，不因空白的 `postgres_ingest.rs` 被移除而刪除。
本次只盤點並提供重構步驟；參考庫維持唯讀，尚未套用搬移，也未執行搬移後測試。原文的版本與 CI 證據屬原教學基準，不能當作這次重構已通過的結果。
## 3. 套件版本
workspace 0.2.12。frontend 既有宣告 leptos 0.8、serde 1.0、serde-wasm-bindgen 0.6、wasm-bindgen 0.2；同 SHA lock 分別 0.8.22、1.0.229、0.6.5、0.2.129；Tauri lock 2.12.4（不可沿用舊 Day 的 2.12.3 當今日版本）。測試沿用 workspace serde_json 1.0／lock 1.0.154，不升級依賴。官方最新 stable／MSRV／安全公告與新功能 native/WASM 相容性本次未查證。
## 4. 驗證項目
### 實作邊界與未完成項目
顯示用純函式獨立在 owning frontend 私有模組，測試不呼叫 Tauri、不掃真實檔案。serde 只保留群組大小、成員數及警告數，不把 hash／opaque ID／警告路徑帶進畫面；預覽不估算「已釋放空間」。錯誤清除舊預覽，請求中停用按鈕，Leptos 以文字呈現結果。後端、JS、Domain、SQLite、migration 本日 N/A。
沿用原限制：目前 hash 的 open/read 仍有 TOCTOU 風險；本日僅資訊性唯讀預覽，不是可刪除清單，也沒有新增刪除、Trash、restore 或成功事件。root 授權／non-root／symlink 回歸保留，不修改正式 tag。200×200 的實際佈局、鍵盤操作與不同縮放必須 GUI 驗證；CSS 上限不能代替實測。SQLite PetStore 仍在原 FUTURE_WORK，未因 UI 完成。
### 驗證命令與結果
本次重新查核最新來源 SHA 的六份 workflow 定義與實際紀錄：Rust AI Chat、Test All、Persistence、Node、Rust Dashboard、Rust Desktop Pet，6/6 workflows、18/18 checks 均 completed/success，沒有 skipped/cancelled/pending check。main 未設 branch protection 必要 contexts；另無 legacy commit-status context（status API 的 pending/total_count=0 只表示沒有 legacy status，並非一個待完成工作），此庫適用 Gate 為上述 Actions/check-runs，已逐一核對，沒有用歷史 CI 替代。[匹配 SHA 的 CI](https://github.com/williechen/rust_ai_chat/actions?query=branch%3Amain)。基準 CI 通過只允許教學發布，不代表以下新增程式已套用或測試通過。
```bash
cargo test --locked -p desktop-pet-frontend duplicate_preview::tests
cargo check --locked -p desktop-pet-frontend --target wasm32-unknown-unknown
node --test apps/desktop_pet/frontend/tests/duplicate-preview-contract.test.mjs
cargo test --locked -p desktop-pet
cargo fmt --all -- --check
```
若新增 dev-dependency 使既有 lock package dependency 清單需更新，由使用者在自己的套用工作區正常更新 Cargo.lock 後再執行 --locked；本助手不寫學習庫或 lock。Red／Green 預期依測試斷言；JS 無任意路徑參數與後端原掃描安全回歸須維持。
GUI 使用專用、已授權非 root 測試目錄：同內容兩檔顯示 1 組及正確 size/count；不同內容同大小不成組；空結果明示 0；失敗不留舊成功；連按只送一次；預覽不改檔案；200×200 可捲動讀完。不得載入私人環境檔或用 / 作測試 root。
本次未套用程式、未執行新 Rust／WASM／GUI 測試，未宣稱 Red／Green 已實測。狀態：程式尚待套用、功能尚未由本日實作證明、正式驗收未通過；Notion 發布及讀回另依實際交易紀錄。未讀密鑰、未執行付費 AI。
