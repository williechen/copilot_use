# 2026-10-15｜P0 Pet Day 12｜桌面漫遊明確啟用控制

來源文章：[2026-10-15｜P0 Pet Day 12｜桌面漫遊明確啟用控制](https://app.notion.com/p/3fae51d4509381d0a80ddbbaf251fb03?pvs=204)。

## 1. 學習大綱
- P0 Pet Day 12（約 15 分鐘）：將**桌面視窗漫遊**由啟動時無條件執行，改為明確啟用才執行的安全功能開關。這是實際桌面視窗行為的最小功能切片，不重做 Day 11 已存在的 SHA256 重複預覽接線。
- 來源：`williechen/rust_ai_chat@5a0a8cee0d1fb4d0cd248869d7b24ebb1d653399`，`apps/desktop_pet/src/lib.rs` 的 `application().setup` 目前直接 `spawn(roam_desktop)`，`apps/desktop_pet/src/roam.rs` 已有工作區邊界、負座標、stall 限制與測試。對照 `copilot_use/docs/FUTURE_WORK.md` 的桌面實際漫遊／安全性。
- 本日只新增**啟用控制**；不宣稱跨平台 GUI 已驗收或已有使用者設定 UI。
## 2. 實作內容
### Red：預設不自動移動
在 `apps/desktop_pet/src/lib.rs` 新增到現有私有測試模組（若尚無則在檔尾建立）：
```rust
#[cfg(test)]
mod roam_setting_tests {
    use super::roam_enabled_value;
    #[test]
    fn roam_requires_explicit_opt_in() {
        assert!(!roam_enabled_value(None));
        assert!(!roam_enabled_value(Some("")));
        assert!(!roam_enabled_value(Some("false")));
        assert!(!roam_enabled_value(Some("TRUE")));
        assert!(roam_enabled_value(Some("true")));
        assert!(roam_enabled_value(Some("1")));
    }
}
```
預期 Red：找不到 `roam_enabled_value`。
### 最小 Green：接上既有原生視窗漫遊
在 `apps/desktop_pet/src/lib.rs` 的 `application` 之前加入：
```rust
// 只接受明確的 true 或 1；其他值均維持不移動。
fn roam_enabled_value(value: Option<&str>) -> bool {
    matches!(value, Some("true" | "1"))
}
fn roam_enabled() -> bool {
    let value = std::env::var("PET_ENABLE_ROAM").ok();
    roam_enabled_value(value.as_deref())
}
```
在 `application().setup(|app| { ... })` 中，**只替換**原本無條件 spawn 的片段：
```rust
if roam_enabled() {
    let app_handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        roam_desktop(app_handle).await;
    });
}
```
不修改 `roam.rs` 的 `bounded_next_position`、`set_position` 或原有 stall 處理。啟用方式：啟動 Tauri 程序前設定 `PET_ENABLE_ROAM=1`；不設定則視窗保持由 OS／使用者控制。
### Refactor／限制
- `roam_enabled_value` 為純函式，測試不需修改程序環境變數，避免測試間競爭。
- 不允許 GUI 默認啟動就自行移動；目前僅啟動時讀取一次，執行中切換及每隻寵物獨立偏好尚未實作。
- 此變更不涉及掃描檔案、刪除、憑證或 SQLite；原有 FileOrganizer 授權根與唯讀預覽規則保持不變。
### 實際套用補充（2026-10-15）
本地已提交 `a57e532c125f5f18d7fd2a250401efde0c9d2612`。`roam_enabled_value`、`roam_enabled`、setup 的條件啟動與本日測試均已套用，未重寫既有漫遊演算法。實際檔案見[精確提交](https://github.com/williechen/rust_ai_chat/blob/a57e532c125f5f18d7fd2a250401efde0c9d2612/apps/desktop_pet/src/lib.rs)。
函式在未設定環境變數時預設關閉；但目前 tracked `.env.example` 已填 `PET_ENABLE_ROAM=true`，若複製並載入該範例就會明確啟用。範例的 `PET_AUTHORIZED_ROOT=/` 不是本次測試設定，不應作為授權掃描範例；本次沒有載入它或掃描根目錄。GUI 實際移動／多螢幕仍未測。
## 3. 套件版本
- 使用既有 `tauri`、`tokio`、Rust 標準函式庫；不新增依賴。
- Cargo.lock 精確版本、官方 stable／MSRV／三平台相容性本次**未另行查證**。
## 4. 驗證項目
- 建議 `cargo test -p desktop-pet roam_requires_explicit_opt_in`、`cargo test -p desktop-pet`、`cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets -- -D warnings`。
- GUI E2E：未設定時視窗不移動；設定 `PET_ENABLE_ROAM=1` 時確實移動且不超出 monitor work area；在負座標、多螢幕、視窗比工作區大及無法移動時檢查停止行為。
- 程式與新增測試已套用；漫遊 opt-in 邏輯與原生 Release 建置通過。GUI 實際視窗移動 E2E 未執行，不能以單元測試替代。
### 來源驗證最終判定（2026-10-16）
**目前狀態：test-workspace 的 Analytics 測試環境缺口已修正，最新 GitHub CI 全部通過。** `rust_ai_chat/main@d05438950b96e03c9c6c6f8f2c7e3573cc191a25` 的 6／6 workflows、18／18 checks 均為 Success；[Test All／test-workspace 成功紀錄](https://github.com/williechen/rust_ai_chat/actions/runs/38765474606/job/116527415647)。最新 test.yml 已提供獨立 PostgreSQL、`ANALYTICS_DATABASE_URL` 與兩個合成來源 token。「缺少 Analytics 資料庫與 token」是舊提交 `a57e532` 的失敗原因，已不再是當前阻礙。
本地正式驗證仍精確對應 `a57e532c125f5f18d7fd2a250401efde0c9d2612`：107 項 Rust、3 項 Node、fmt、嚴格 Clippy、SSR／兩個 WASM、Linux 原生／Chat Leptos／Pet 前端 Release，以及 14 項實際 HTTP／WS／DB 檢查通過。已唯讀比對 `a57e532..d054389`，只有 `test.yml` 與 `persistence.yml` 改動，應用程式／測試／依賴鎖均未變；因此保留原本地證據，不冒稱已對新 SHA 重跑全部本地驗證。[精確來源、CI 與可攜式證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/source-review/rust-ai-chat-validation.json)。
Day 12 已由使用者套用；原撰寫基底 `5a0a8ce` 與全部原教學程式區塊保留。Analytics HTTP 已接 PostgreSQL，Typing／來源認證／持久冪等／併發／重啟／DB 故障測試通過。Pet 實際 GUI 移動、跨平台安裝包、gRPC／SSE 銜接及正式憑證安全審查仍未由本次驗證完成；不將 CI 全綠當作全部 Day／P9／週六或學習階段完成。未讀取私人 .env／AI key，未執行付費 AI，未寫入 rust_ai_chat。
