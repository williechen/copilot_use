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
## 3. 套件版本
- 使用既有 `tauri`、`tokio`、Rust 標準函式庫；不新增依賴。
- Cargo.lock 精確版本、官方 stable／MSRV／三平台相容性本次**未另行查證**。
## 4. 驗證項目
- 建議 `cargo test -p desktop-pet roam_requires_explicit_opt_in`、`cargo test -p desktop-pet`、`cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets -- -D warnings`。
- GUI E2E：未設定時視窗不移動；設定 `PET_ENABLE_ROAM=1` 時確實移動且不超出 monitor work area；在負座標、多螢幕、視窗比工作區大及無法移動時檢查停止行為。
- **新測試及 GUI E2E 未執行，程式尚未套用**。既有 SHA CI Success 不代表本切片已驗收。
### 來源驗證同步（2026-10-15）
先完成本地來源驗證，再核對相同 SHA `5a0a8cee0d1fb4d0cd248869d7b24ebb1d653399` 的 GitHub CI。本地 102 項 Rust 測試（含真實隔離 PostgreSQL）及 3 項 Node 測試通過，SSR／WASM 檢查、Linux 原生／Chat Leptos／Pet 前端 release 建置通過；實際 HTTP／WS／SQLite 檢查通過。嚴格 Clippy **未通過**：`apps/desktop_pet/src/file_organizer.rs:309` 的 `expect(dead_code)` 已失效；GitHub 6／6 workflows、18／18 checks 成功，不包含這項嚴格 Clippy Gate。[精確來源、命令與限制](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/source-review/rust-ai-chat-validation.json)。
本次只驗證來源既有程式；本 Day 新增功能及測試仍尚待套用，未執行其 Green／GUI／功能 E2E，不將基線通過當成本 Day 功能驗收。未讀取 AI key，未執行付費 AI 呼叫。
