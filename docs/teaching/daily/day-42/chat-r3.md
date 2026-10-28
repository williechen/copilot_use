# Chat Day42 r3｜2026-10-29 主分支接線讀回

> 本篇是已發布 [Day42 原文](chat.md) 的**不可變追加修訂**，不是 Day43，也不覆寫原文／patch。產品唯讀來源 [rust_ai_ring@c6708c4](https://github.com/williechen/rust_ai_ring/tree/c6708c42f21ac919e57a379848b4977d27a1c3d6)；文件規則依 copilot_use/main 2026-10-29 讀回。

## ① 目標與正式呼叫鏈

原 Day42 功能為進入房間後取得真實房名，失敗時顯示明確狀態。主分支已見 `chat_apps/chat_ui/src/room_api.rs::room_name`（GET /api/rooms/{id}），以及 `chat_apps/chat_ui/src/lib.rs` 的 ChatPage／room_heading／房間標題接線；既有 SQLite Repository、HTTP 路由、WS 授權仍由原流程負責。此為**程式接線已見**，非瀏覽器驗收。

## Refactor｜僅 Chat

原 [獨立 Refactor patch](chat-day42-refactor.patch) 將 `room_status_text` 從 Leptos 元件檔搬入 `chat_apps/chat_ui/src/room_list_state.rs`；最新來源該檔第47行可見函式，第153行起可見 `day42_status_refactor_tests`，覆蓋載入、錯誤、空清單、搜尋無結果及有結果。舊搜尋／首次載入斷言不得刪除。本輪未新增重構 patch，也不計入跨案 G-01／G-02。

## ② Red／Green 原文承接

[原文完整 Red、Green 與正式接線](chat.md#②-red--完整-green核心預估-15-分鐘)及 [Green patch](chat-day42-green.patch) 保留。Red 為 `day42_room_heading_tests` 的真實房名與讀取失敗兩個不同可見狀態；Green 是 `room_name` HTTP 查詢、`room_heading` 映射及 Leptos heading signal。原 patch 只對原來源 `0b2288e` 依 Refactor → Green 順序重現；**最新 main 已有相應程式，不得再正向套用**。

## ③ 版本

2026-10-29 官方 Rust 最新穩定版 [1.99.1（10月15日）](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)；產品四份 Cargo workspace 的 `workspace.package.version` 均為 `0.2.42`。主要套件及舊來源 lock 的逐項核對見 [Day42 版本表](versions.md)；最新四份 lock blob 與舊來源不同，**本輪未完成所有依賴最新穩定版／MSRV／WASM 相容 Gate**。

## ④ 驗證與下一步

本輪透過 GitHub 精確 commit 讀取相關來源，確認函式、module 與測試存在；**未執行** Red、Cargo、WASM、HTTP、瀏覽器或 CI。2026-10-28 [同一產品 SHA 的 Chat 57 項測試](../../../source-review/local-validation-2026-10-28-c6708c4/README.md)是既有獨立證據，非本輪重跑。下一步在隔離產品副本重跑 Chat SSR／WASM、真實房間 HTTP、登入／ACL、瀏覽器載入與錯誤提示；完整 Gate1／Gate2 尚未通過，next_day.chat 維持42。
