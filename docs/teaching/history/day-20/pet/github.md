## 1. 學習大綱
本日 **Day20** 單一切片：當寵物生活視窗在目前顯示器工作區內**兩軸都無法移動**時，仍先把生活窗定位到 `bounded_next_position` 計算的合法原點，之後結束本次自動漫遊，不再做三輪無效位置驗證。只有兩軸都不具空間才停止，單軸被限制時另一軸可以持續移動。
真實入口：`apps/desktop_pet/src/lib.rs::application() → setup → PET_ENABLE_ROAM → tauri::async_runtime::spawn → roam_desktop(app) → get_webview_window("pet") → outer_position/outer_size/current_monitor → bounded_next_position → window.set_position`。資料座標使用 `tauri::PhysicalPosition<i32>`、工作區 `PhysicalRect<i32,u32>`；原教學基底在兩軸工作區都不足時將 dx/dy 設為 0，卻仍重複等待和計次；目前 main 已在 set_position 之後使用 no_roaming_space(dx,dy) 立即停止無效漫遊。原教學基底 `rust_ai_chat/main=0cd9f7995c196a7359864a9395c0fe2372d09222`；現行主線 `rust_ai_chat/main=c92831e105fb0000860f995ea288e29631b5fed8`（唯讀）；教學規則 `daily-tdd-minimum-v1`。Red 5 分、Green 10 分（15 分主體），Refactor 與 GUI 驗證另計。
### 2026-10-19｜r2 真實程式與測試核對
現行 `c92831e105fb0000860f995ea288e29631b5fed8` 的 `apps/desktop_pet/src/roam.rs` blob `0de01ffd6e98a7fbc9388ed31a7ae7855c4c6749` 已包含 `no_roaming_space`，且由 `roam_desktop` 在成功 `set_position(requested)` 後、等待 `ROAM_VERIFY_INTERVAL` 前判斷停止；`day20_stops_only_when_both_axes_are_blocked`、`day20_oversized_window_is_pinned_and_stops`、`day20_single_axis_can_still_roam` 三個測試均存在。`apps/desktop_pet/src/lib.rs::application` 的 `setup` 僅在 `roam_enabled()` 時 spawn `roam_desktop`，標籤為 pet，主控視窗 main 不被移動。Day20 原文 Red／Green 保留作**從舊基底學習**；目前 main 不應重複貼入同名符號或測試。
[新 SHA Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39202728570) 的 `test-desktop-pet`（job `117898951153`）成功；[Rust Desktop Pet](https://github.com/williechen/rust_ai_chat/actions/runs/39202728360) 的 `check-pet`、`build-pet` 成功，已涵蓋本日合入原碼。但 CI 並未人工拖曳生活窗，也沒有在真實多螢幕、WM、縮放或視窗過大場景完成 GUI 驗收；單元測試對 helper 與幾何的驗證，不等於親自觀察漫遊 loop 在原生桌面停止。
## 2. 實作內容
### Red：先加測試
在 `apps/desktop_pet/src/roam.rs` 既有 `#[cfg(test)] mod tests` 中新增下列三個案例；尚未增加 helper 時預期 **E0425**，屬新符號缺失 Red，**本輪未實測**：
```rust
#[test]
fn day20_stops_only_when_both_axes_are_blocked() {
    assert!(super::no_roaming_space(0, 0));
    assert!(!super::no_roaming_space(0, 1));
    assert!(!super::no_roaming_space(2, 0));
    assert!(!super::no_roaming_space(-1, 2));
}

#[test]
fn day20_oversized_window_is_pinned_and_stops() {
    let mut dx = 2;
    let mut dy = 1;
    let next = super::bounded_next_position(
        PhysicalPosition { x: 20, y: 50 },
        PhysicalSize { width: 900, height: 1000 },
        &area(-100, -50, 800, 600),
        &mut dx,
        &mut dy,
    );
    assert_eq!(next, PhysicalPosition { x: -100, y: -50 });
    assert!(super::no_roaming_space(dx, dy));
}

#[test]
fn day20_single_axis_can_still_roam() {
    let mut dx = 2;
    let mut dy = 1;
    let next = super::bounded_next_position(
        PhysicalPosition { x: 10, y: 10 },
        PhysicalSize { width: 900, height: 100 },
        &area(0, 0, 800, 600),
        &mut dx,
        &mut dy,
    );
    assert_eq!(next.x, 0);
    assert_eq!(next.y, 11);
    assert_eq!(dx, 0);
    assert_eq!(dy, 1);
    assert!(!super::no_roaming_space(dx, dy));
}
```
### Green：完整最小修改
在 `apps/desktop_pet/src/roam.rs` 的 `bounded_next_position` 後新增：
```rust
fn no_roaming_space(dx: i32, dy: i32) -> bool {
    dx == 0 && dy == 0
}
```
在 **同檔已存在的** `pub async fn roam_desktop(app: tauri::AppHandle)` 中，把 `window.set_position(requested)` 的錯誤處理區段保持原樣，緊接其後（`tokio::time::sleep(ROAM_VERIFY_INTERVAL).await` 之前）加入：
```rust
// 先讓 WM 接受要求的位置，再停止沒有任何可移動軸的漫遊。
// 不把普通的單軸邊界反彈誤認為無空間。
if no_roaming_space(dx, dy) {
    eprintln!("desktop roaming stopped: pet window exceeds available work area");
    break;
}
```
整合順序為：`bounded_next_position` 改變 `dx/dy` → `set_position(requested)`（錯誤則 break）→ `no_roaming_space`（兩軸為零則 break）→ 原有 200ms 讀回與 Day19 異常位移驗證 → 原 50ms 漫遊循環。**不要**提前放在 `set_position` 前，否則無法完成 pin 到工作區原點。其餘 imports、Day19 檢查、stall 檢查和 Tauri 事件註冊不更動。這是可在單檔直接加入的完整 Green。
### Refactor／必要補教
可以為「工作區不足」使用不同訊息區分 WM 拒絕位置；現行設定仍屬單隻 pet 原生生活窗，並非多寵物配置系統。若寵物顯示器被拔除或工作區變更，仍需實際 WM 測試；`set_position` 可能被 WM 調整，這個切片不取代 Day19 的觀察及停止條件。
## 3. 套件版本
原教學基底工作區版本 `0.2.19`；現行 `Cargo.toml` 工作區版本 `0.2.20`、edition 2024、`tokio=1.53` workspace 宣告；Tauri 2.x（精確解析以本來源 SHA 的 Cargo.lock 為準），無新套件／migration。曾有 Linux GTK/WebKit 建置證據，但本輪未重跑原生 GUI，OS/WM/多螢幕未驗證。
## 4. 驗證項目
```bash
cargo test --locked -p desktop-pet day20
cargo test --locked -p desktop-pet
cargo build --locked -p desktop-pet
```
預期：Day20 3 tests 通過，雙軸不可移動時視窗 set_position 後停止；單軸仍可漫遊，既有 Day19 測試不退步。**本次未在本地重跑上述命令**。目前 main `c92831e105fb0000860f995ea288e29631b5fed8` 已合入三個 Day20 測試、helper 與循環接線；[新 SHA Test All/test-desktop-pet](https://github.com/williechen/rust_ai_chat/actions/runs/39202728570) 成功（job `117898951153`），[Rust Desktop Pet check/build](https://github.com/williechen/rust_ai_chat/actions/runs/39202728360) 成功。舊來源 SHA 的 CI 是歷史資料，不能套用到新原碼。待驗收：實際多螢幕工作區縮放、WM 拒絕/調整 set_position、單軸移動與真原生 GUI。Day20 程式已套用 `rust_ai_chat/main`；使用者原生桌面部署、其他產品整合與完整功能驗收均未完成。
