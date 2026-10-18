# Pet Day19 修正版：觀察到非預期偏差時停止漫遊
## 1. 學習大綱
來源 `rust_ai_chat/main=f692c2ebe7b414fe49858c1ab1c4a33ce7c3dfee`；教學分支 `44dabb3eebed12639cd949b93521cbdc47a401fb`。本篇為本地查核修正版，未寫回 Notion、未發布、未推進 Day。主體 Red 5 分鐘、Green 10 分鐘，Refactor／額外驗證另計；實際測試結果見 `驗證結果.md`。完整修改檔在 `source/`，patch 基底為上述來源 SHA。

目標：每次 set_position 後約200ms讀回位置，任一軸與 requested 偏差嚴格大於32 physical pixels，就 break 本次漫遊。入口 application/setup→PET_ENABLE_ROAM→roam_desktop→get_webview_window("pet")→outer_position/outer_size/current_monitor→bounded_next_position→set_position→after。tauri.conf.json 的 pet 生活窗與 main 控制窗標籤已核對。

這是單一觀察時點的安全停止。WM吸附或限制也可能觸發；並不能辨識人為拖曳，也不能涵蓋兩次觀察間所有位移。恰32不停止，(23,23)也不停止；採每軸判斷，並非歐氏距離。只要落在下一次set_position前但未被本次after觀察的外部位移，仍可能被後續移動覆寫。
## 2. 實作內容
### Red
先把 tests 模組中的 `day19_large_external_reposition_stops_roam` 加到現有 tests；helper不存在時預期E0425。名稱沿用原草稿供追溯，該測試實際只測位移判斷，不直接測loop停止。
### 完整最小 Green
檔案 `apps/desktop_pet/src/roam.rs`。保留檔案前方 imports、既有常數與 bounded_next_position，在其後加入以下 helper，並替換整個 roam_desktop。完整可替換檔見source同路徑。

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


// 在桌面上漫遊寵物的主循環
pub async fn roam_desktop(app: tauri::AppHandle) {
    let mut dx = ROAM_INITIAL_DX;
    let mut dy = ROAM_INITIAL_DY;
    let mut stalled_attempts = 0_u8;

    loop {
        let window = match app.get_webview_window(roaming_window_label()) {
            Some(window) => window,
            None => {
                eprintln!("desktop roaming unavailable: pet window not found");
                break;
            }
        };

        let before = match window.outer_position() {
            Ok(position) => position,
            Err(error) => {
                eprintln!("desktop roaming unavailable: {error}");
                break;
            }
        };

        let window_size = match window.outer_size() {
            Ok(size) => size,
            Err(error) => {
                eprintln!("desktop roaming unavailable: {error}");
                break;
            }
        };

        let monitor = match window.current_monitor() {
            Ok(Some(monitor)) => monitor,
            Ok(None) => {
                eprintln!("desktop roaming unavailable: no monitor found");
                break;
            }
            Err(error) => {
                eprintln!("desktop roaming unavailable: {error}");
                break;
            }
        };

        let requested =
            bounded_next_position(before, window_size, monitor.work_area(), &mut dx, &mut dy);

        if let Err(error) = window.set_position(requested) {
            eprintln!("desktop roaming unavailable: {error}");
            break;
        }

        tokio::time::sleep(ROAM_VERIFY_INTERVAL).await;

        let after = match window.outer_position() {
            Ok(position) => position,
            Err(error) => {
                eprintln!("desktop roaming verification unavailable: {error}");
                break;
            }
        };

        if unexpected_displacement(requested, after) {
            eprintln!("desktop roaming stopped: unexpected window displacement");
            break;
        }

        if after == before {
            stalled_attempts += 1;

            if stalled_attempts >= ROAM_STALL_LIMIT {
                eprintln!(
                    "desktop roaming unavailable: \
                     window position did not change after \
                     {ROAM_STALL_LIMIT} attempts"
                );
                break;
            }
        } else {
            stalled_attempts = 0;
        }

        tokio::time::sleep(ROAM_INTERVAL).await;
    }
}
```

以下為完整測試模組，含既有測試及新增邊界測試；取代既有同名模組。

```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day19_displacement_threshold_axes_and_extremes() {
        let p = |x, y| PhysicalPosition { x, y };
        assert!(!unexpected_displacement(p(-100, -100), p(-68, -132)));
        assert!(unexpected_displacement(p(-100, -100), p(-67, -100)));
        assert!(unexpected_displacement(p(-100, -100), p(-100, -133)));
        assert!(!unexpected_displacement(p(0, 0), p(23, 23)));
        assert!(unexpected_displacement(p(i32::MIN, 0), p(i32::MAX, 0)));
        assert!(unexpected_displacement(p(0, i32::MAX), p(0, i32::MIN)));
    }

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


    fn area(x: i32, y: i32, width: u32, height: u32) -> PhysicalRect<i32, u32> {
        PhysicalRect {
            position: PhysicalPosition { x, y },
            size: PhysicalSize { width, height },
        }
    }

    #[test]
    fn moves_inside_work_area() {
        let mut dx = 2;
        let mut dy = 1;
        let next = bounded_next_position(
            PhysicalPosition { x: 100, y: 100 },
            PhysicalSize {
                width: 320,
                height: 360,
            },
            &area(0, 0, 1920, 1080),
            &mut dx,
            &mut dy,
        );
        assert_eq!(next, PhysicalPosition { x: 102, y: 101 });
        assert_eq!((dx, dy), (2, 1));
    }

    #[test]
    fn bounces_at_right_and_bottom_edges() {
        let mut dx = 2;
        let mut dy = 1;
        let next = bounded_next_position(
            PhysicalPosition { x: 1599, y: 719 },
            PhysicalSize {
                width: 320,
                height: 360,
            },
            &area(0, 0, 1920, 1080),
            &mut dx,
            &mut dy,
        );
        assert_eq!(next, PhysicalPosition { x: 1600, y: 720 });
        assert_eq!((dx, dy), (-2, 1));
    }

    #[test]
    fn supports_negative_monitor_origin() {
        let mut dx = -2;
        let mut dy = -1;
        let next = bounded_next_position(
            PhysicalPosition { x: -1600, y: 0 },
            PhysicalSize {
                width: 320,
                height: 360,
            },
            &area(-1920, 0, 1920, 1080),
            &mut dx,
            &mut dy,
        );
        assert_eq!(next, PhysicalPosition { x: -1602, y: 0 });
        assert_eq!((dx, dy), (-2, 1));
    }

    #[test]
    fn oversized_window_is_pinned_without_panicking() {
        let mut dx = 2;
        let mut dy = 1;
        let next = bounded_next_position(
            PhysicalPosition { x: 10, y: 20 },
            PhysicalSize {
                width: 1000,
                height: 900,
            },
            &area(-100, -50, 800, 600),
            &mut dx,
            &mut dy,
        );
        assert_eq!(next, PhysicalPosition { x: -100, y: -50 });
        assert_eq!((dx, dy), (0, 0));
    }

    #[test]
    fn day18_roaming_selects_pet_life_window() {
        assert_eq!(roaming_window_label(), "pet");
        assert_ne!(roaming_window_label(), "main");
    }
}

```

### Refactor
兩軸統一使用容許值常數，先轉i64再相減與abs，避免i32極值溢位。保留原work_area、負原點、stall三次、關窗退出及只選pet的行為。helper通過不能證明原生WM、GUI拖曳與loop break；這些需獨立驗證。
## 3. 套件版本
Cargo.lock：tauri 2.12.4、tauri-build 2.7.3、tokio 1.53.3。Linux GTK 3.24.52、WebKitGTK 2.52.6（pkg-config）。未量測實際WM／多螢幕與縮放行為。
本輪 Linux x86_64，rustc 1.99.1／cargo 1.99.1、stable 工具鏈，WASM target 已安裝。專案未宣告 rust-version，最低 MSRV 未驗證；其他作業系統未驗證。無新增套件或 migration，Cargo.lock 未改。

## 4. 驗證項目
```sh
cargo test --locked -p desktop-pet day19
cargo test --locked -p desktop-pet
cargo build --locked -p desktop-pet
```
本輪實際命令與結果以驗證結果表為準；未列為通過的建置不得推定執行過。單元涵蓋大小位移、32/33、負軸與極值；原生GUI未執行，尚需正常漫遊→在驗證窗口位移→停止、main不移動、小位移及WM調整、多螢幕、關窗不回退。來源Pet CI不含本次Green；未完成完整產品驗收。
