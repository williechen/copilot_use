## 1. 學習大綱
目標：修正桌面寵物「漫遊」只移動生活窗 pet、不移動控制窗 main，且沿用既有螢幕邊界與 stall 偵測。這是 Day17 教學候選的真實建窗契約補齊；來源 main 尚未套用，Day18 提供經 tauri.conf.json 交叉核對的完整最小 Green。主體 Red/Green 15 分鐘。
來源 rust_ai_chat/main 98f30d8b3593d80103b1a4bdbb9f0cb8cb08069e：apps/desktop_pet/tauri.conf.json:20-41 確認 windows 標籤 main（控制台）與 pet（生活窗），apps/desktop_pet/src/[lib.rs:94](http://lib.rs:94)-98 的 open_control_window 僅操作 main，:148-152 在 roam_enabled 時 spawn roam_desktop，apps/desktop_pet/src/[roam.rs:60](http://roam.rs:60)-137 目前錯誤地 get_webview_window("main") → outer_position/outer_size/current_monitor → bounded_next_position → set_position → 驗證位移。應保留既有邊界演算法，只修正目標窗。
## 2. 實作內容
### Red（先寫，尚未執行）
在 apps/desktop_pet/src/[roam.rs](http://roam.rs) 既有 #\[cfg(test)\] mod tests 內增加：
```rust
#[test]
fn day18_roaming_selects_pet_life_window() {
    assert_eq!(roaming_window_label(), "pet");
    assert_ne!(roaming_window_label(), "main");
}
```
現有來源沒有 roaming_window_label，故預期先編譯失敗；屬預期 Red，未實際執行。手動整合 Red：啟用漫遊時目前 get_webview_window("main") 將移動控制窗，與 tauri.conf.json 標籤契約不符；未實際啟動 GUI。
### 完整最小 Green
在 apps/desktop_pet/src/[roam.rs](http://roam.rs) 頂部常數區加入：
```rust
// 與 tauri.conf.json 的生活窗 label 一致，main 僅供控制台。
const PET_WINDOW_LABEL: &str = "pet";
fn roaming_window_label() -> &'static str {
    PET_WINDOW_LABEL
}
```
將 roam_desktop 內原本的 window 選取區塊完整替換為：
```rust
let window = match app.get_webview_window(roaming_window_label()) {
    Some(window) => window,
    None => {
        eprintln!("desktop roaming unavailable: pet window not found");
        break;
    }
};
```
保留 loop、dx/dy、outer_position、outer_size、current_monitor、bounded_next_position、set_position、200ms 位移確認與 stall limit。此最小切片不動控制窗 show/focus、寵物事件處理與多寵物邏輯。生活窗不存在時安全停止，不 fallback 到 main；啟動鏈 setup → roam_enabled → tokio spawn → roam_desktop → pet WebviewWindow → bounded_next_position → set_position。Green 為完整可套用的函式選窗替換，不宣稱 GUI 已驗收。
### Refactor／補教
未來抽取共用窗標籤定義，避免 tauri.conf.json 與 Rust 字串漂移；加入雙窗 GUI 自動化驗證（生活窗位移、控制窗原位、負座標螢幕、關閉生活窗停止）。Day17 的標籤待核對缺口已由本次實際 tauri.conf.json:20-41 核對補充；Day17 原文不改。
## 3. 套件版本
apps/desktop_pet/Cargo.toml：tauri 2.12、tauri-build 2.7、tokio workspace；tauri.conf.json product version 0.2.16；不新增套件。Cargo.lock/MSRV 本次未核實。
## 4. 驗證項目
來源 SHA rust_ai_chat/main 98f30d8b3593d80103b1a4bdbb9f0cb8cb08069e（唯讀）；教學分支發布前 SHA 4fbc8103d1d1b83ac5970f7367172172a051fb6b；規則 daily-tdd-minimum-v1。
`cargo test -p desktop-pet day18_roaming_selects_pet_life_window`：預期新增測試通過，實際未執行。`cargo test -p desktop-pet roam::tests`：預期邊界測試回歸，實際未執行。GUI：啟用漫遊，確認 pet 移動、main 不移動、關閉 pet 停止、不同螢幕 work area 正確；未執行。CI：未取得符合此候選 patch 的 workflow/job 成功證據，不標記通過。程式套用／產品整合／功能驗收均未核實。待完成：實際套用 patch、cargo 測試、GUI 及多寵物整合證據。
