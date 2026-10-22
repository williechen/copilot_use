Here is the result of "fetch" for the Page with URL https://app.notion.com/p/3fde51d450938194ae7fdcc55bce44e2 as of 2026-10-18T22:13:45.043Z:
<page url="https://app.notion.com/p/3fde51d450938194ae7fdcc55bce44e2">
<ancestor-path></ancestor-path>
<properties>
{"title":"2026-10-19 Pet Day 19 — 非預期位移"}
</properties>
<iconMetadata>null</iconMetadata>
<content>
## 1. 學習大綱
Pet 漫遊外部位移安全切片，未實測。
## 1. 學習大綱（完整）
目標：當寵物生活窗在一次 set_position 後被外部大幅移位，停止本次漫遊以免覆寫使用者或視窗管理器的位置。Red 5 分鐘、Green 10 分鐘，Refactor 5 分鐘、原生 GUI 10 分鐘另計。只保證「驗證窗口內顯著位移」；不是完整拖曳暫停／恢復。
來源 rust_ai_chat/main `f692c2ebe7b414fe49858c1ab1c4a33ce7c3dfee`、規則 daily-tdd-minimum-v1。呼叫鏈 `apps/desktop_pet/src/lib.rs::application` → setup／PET_ENABLE_ROAM → roam_desktop → get_webview_window("pet") → outer_position／outer_size／current_monitor → bounded_next_position → set_position → 200ms 後 outer_position。現況只用 after==before 判斷停滯，未檢查 after 與 requested 的大幅差異。
## 2. 實作內容
**Red**：在 `apps/desktop_pet/src/roam.rs` 既有 tests 模組新增；helper 尚未存在時預期編譯失敗，**未實測**。
```rust
#[test]
fn day19_large_external_reposition_stops_roam() {
    assert!(unexpected_displacement(
        PhysicalPosition { x: 100, y: 100 },
        PhysicalPosition { x: 400, y: 100 },
    ));
    assert!(!unexpected_displacement(
        PhysicalPosition { x: 100, y: 100 },
        PhysicalPosition { x: 102, y: 99 },
    ));
}
```
**Green**：在 bounded_next_position 後新增純函式：
```rust
const UNEXPECTED_MOVE_TOLERANCE: i64 = 32;

fn unexpected_displacement(
    requested: PhysicalPosition<i32>,
    observed: PhysicalPosition<i32>,
) -> bool {
    let dx = (i64::from(observed.x) - i64::from(requested.x)).abs();
    let dy = (i64::from(observed.y) - i64::from(requested.y)).abs();
    dx > UNEXPECTED_MOVE_TOLERANCE || dy > UNEXPECTED_MOVE_TOLERANCE
}
```
在 `roam_desktop` 的 `let after = match window.outer_position() { ... };` 成功讀回後、原本 `if after == before` 前新增：
```rust
if unexpected_displacement(requested, after) {
    eprintln!("desktop roaming stopped: unexpected window displacement");
    break;
}
```
**Refactor**：用 i64 比較避免 i32 溢位，沿用 work_area 邊界及負座標；main 控制窗不動。32 physical pixels 是示範閾值，WM 吸附也可能觸發安全停止；需實際 GUI 驗證。Day18 native probe 歷史結果不能直接代替 Day19。
## 3. 套件版本
已讀 `apps/desktop_pet/Cargo.toml`：Tauri 2.12、tauri-build 2.7、Tokio workspace、edition 2024；無新增套件或 migration。Cargo.lock 精確解析版／MSRV／原生 WM 版本本輪未重查。
## 4. 驗證項目
- `cargo test --locked -p desktop-pet day19_large_external_reposition_stops_roam`：預期先缺 helper 編譯失敗、套 Green 後 1 passed；**實際未執行**。
- `cargo test --locked -p desktop-pet`、`cargo build --locked -p desktop-pet`：**未執行**。
- 真實 GUI：正常漫遊→驗證窗內外部位移\>32px→停止；小位移繼續；main 不移動，邊界／關窗不回退 main。**未執行**。
- 來源 [Pet CI](https://github.com/williechen/rust_ai_chat/actions/runs/39148141526) 與 [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39148141573) Success，但未包含尚未套用 Green。
- Notion 私人草稿已保存；父頁正式發布／讀回未完成；next_day 仍19。程式套用、產品整合、完整功能驗收均否。
</content>
</page>