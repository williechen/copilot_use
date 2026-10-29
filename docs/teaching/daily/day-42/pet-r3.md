# Pet Day42 r3｜2026-10-29 主分支接線讀回

> 本篇是 [Day42 原文](pet.md) 的不可變追加修訂，保留原 patch 與 r1／r2；不是 Day43。產品唯讀來源 [rust_ai_ring@c6708c4](https://github.com/williechen/rust_ai_ring/tree/c6708c42f21ac919e57a379848b4977d27a1c3d6)。

## ① 目標與正式呼叫鏈

原 Day42 以控制台逐隻 UUID 查詢 SQLite 回籠狀態。最新 `pet_apps/pet_server/src/lib.rs` 已見 `pet_cage_status_by_id`（第125行）及 Tauri `generate_handler!` 註冊（第316行）；`pet_apps/pet_ui/tests/day42-cage-status.test.mjs` 已見兩個不同 UUID 的回傳及 SQLite 讀取失敗傳遞斷言。此為**正式 IPC 接線與測試來源已見**，非原生視窗驗收。

## Refactor｜僅 Pet

[獨立 Refactor patch](pet-day42-refactor.patch) 將 `roam_enabled_value` 移至 `pet_apps/pet_server/src/roam.rs`；最新第221行已見函式，第226行起已見 `day42_roam_policy_refactor_tests`，保留 true／1 才啟用、其它輸入 fail-closed 的斷言。server 的正式 caller 已改用 `crate::roam::roam_enabled_value`。此項只屬 Pet，不計全庫重構。

## ② Red／Green 原文承接

[原文完整 Red／Green 與前置順序](pet.md)及 [Green patch](pet-day42-green.patch) 保留。Red 鎖定逐 UUID 的回籠查詢及原生失敗回傳；Green 由 PetStore SQLite read_caged → Tauri command → `window.petCageStatusById` → Leptos 控制台狀態構成。原 patch 僅適用 `0b2288e`，須 Refactor → Green；最新 main 已含對應接線，不重複套用。

## ③ 版本

2026-10-29 Rust 官方穩定版為 [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)；產品四 workspace version `0.2.42`。舊來源的 Tauri／Leptos／SQLx resolved 版本見 [Day42 版本表](versions.md)；最新 lock blob 已變，native／CLI／WASM／MSRV 的完整相容驗證仍未完成。

## ④ 驗證與下一步

本輪 GitHub 精確 SHA 讀回確認 IPC command、module 與 Node 測試存在；**未執行** Node、Cargo、SQLite、Tauri GUI 或 CI。2026-10-28 [同 SHA Node 10 項通過](../../../source-review/local-validation-2026-10-28-c6708c4/README.md)是前輪證據，不是本輪重跑。下一步測真 SQLite 錯誤、逐 UUID IPC、原生回籠／關窗／重啟及 WebKit，保留既有有效斷言。Gate1／Gate2 未完整通過，next_day.pet 維持42。
