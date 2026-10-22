# 2026-10-19 Pet Day19 — 非預期大幅位移停止漫遊（未正式發布）
## 1. 學習大綱
來源 rust_ai_chat/main f692c2ebe7b414fe49858c1ab1c4a33ce7c3dfee；規則 daily-tdd-minimum-v1。入口 apps/desktop_pet/src/lib.rs::application → setup → roam_desktop → pet WebviewWindow → bounded_next_position → set_position → outer_position。主體 Red 5 分鐘／Green 10 分鐘，Refactor 5 分鐘／GUI 10 分鐘另計。現有程式只比較 after == before，未檢查 after 與 requested 的顯著偏差。
## 2. 實作內容
Red：在 roam.rs::tests 加 day19_large_external_reposition_stops_roam，斷言 expected(100,100)／observed(400,100) 為 true、observed(102,99) 為 false；helper 尚未存在，預期先編譯失敗。
Green：在 bounded_next_position 後加 const UNEXPECTED_MOVE_TOLERANCE:i64=32; fn unexpected_displacement(requested:PhysicalPosition<i32>,observed:PhysicalPosition<i32>)->bool { (i64::from(observed.x)-i64::from(requested.x)).abs()>32 || (i64::from(observed.y)-i64::from(requested.y)).abs()>32 }。在 roam_desktop 讀取 after 成功後、if after==before 之前加 if unexpected_displacement(requested,after) { eprintln!("desktop roaming stopped: unexpected window displacement"); break; }。Refactor：集中差異判定、使用 i64 防溢位，保留 main 控制窗不動。只處理單次驗證窗的顯著位移，不宣稱完整拖曳恢復。
## 3. 套件版本
apps/desktop_pet/Cargo.toml 已讀：tauri 2.12、tauri-build 2.7、Tokio workspace、edition 2024；無新增依賴／migration，MSRV 未查。
## 4. 驗證項目
cargo test --locked -p desktop-pet day19_large_external_reposition_stops_roam；cargo test --locked -p desktop-pet；cargo build --locked -p desktop-pet；實際原生 GUI 驗證 main 不移動、位移>32 停止，均未執行。來源 Pet CI https://github.com/williechen/rust_ai_chat/actions/runs/39148141526 success，未覆蓋 Green。Notion 正式發布阻擋、next_day 仍19，產品整合／功能驗收未通過。
