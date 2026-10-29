# 各專案Refactor進度

> **2026-10-29 Day43 各案獨立 Refactor 文件交付（僅方案，不計已實作）：** [Chat：Session 入口決策搬層](../daily/day-43/chat.md#refactorchat-session-入口狀態搬層與功能-green-獨立)、[Pet：原生選單 UUID 解析搬層](../daily/day-43/pet.md)、[Analytics：Dashboard 計數標籤搬層](../daily/day-43/analytics.md) 分別有完整模組／正式 caller／舊斷言承接及 [六份可套用 patch](../daily/day-43/README.md)。來源 `rust_ai_ring@c6708c42f21ac919e57a379848b4977d27a1c3d6`；本輪僅嚴格來源 diff 核對、無使用者產品套用或真 Rust/DB/GUI/CI 測試。方案已發表≠重構已完成，均不計 G-01／G-02，也不增加 Day；三案下一待發布篇為 Day44。[提交證據](../daily/day-43/delivery-verification.json)。


> **2026-10-29 主分支來源勘誤（優先於下方歷史快照）：** 產品唯讀來源 `c6708c42f21ac919e57a379848b4977d27a1c3d6` 已讀回 Day42 三案 Refactor 的正式函式／caller／測試：Chat `chat_ui/src/room_list_state.rs::room_status_text` 與五種狀態測試；Pet `pet_server/src/roam.rs::roam_enabled_value` 與正式 server caller；Analytics `analytics_server/src/dashboard_url.rs::encode_query_component` 與 Dashboard caller。故下方「Day42 僅方案／未套用」**只描述 0b2288e 撰文時的歷史狀態，不能代表最新產品**。現況為**產品 main 已見實作及測試來源／本輪未重新執行／完整 Gate2 未驗收**；不得重複套用原 patch 或把本項計入 G-01/G-02。[精確來源與三篇修訂](../daily/day-42/source-readback-2026-10-29.json) · [Chat r3](../daily/day-42/chat-r3.md) · [Pet r3](../daily/day-42/pet-r3.md) · [Analytics r2](../daily/day-42/analytics-r2.md)。

> 2026-10-28 固定產品來源 `c6708c42f21ac919e57a379848b4977d27a1c3d6`；[本輪原始命令與結果](../../source-review/local-validation-2026-10-28-c6708c4/README.md)。本輪只增量重測 fixture、metadata／fmt；其他來源實測維持原 SHA，完整驗收與版本 Gate 尚未完成。


最新來源 `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`。本輪真正重跑測試與可行瀏覽器流程，沒有新增功能或重構計功；歷史證據保持原SHA。

| 專案 | 來源現況 | 本輪／後續 |
| --- | --- | --- |
| [Chat](../daily/day-40/chat-r2.md#refactor) | 共用token_digest保留，/account回歸已補回 | Rust59及fmt/check/WASM通過；指定登入／管理／登出瀏覽器流程通過 |
| [Pet](../daily/day-40/pet-r2.md#refactor) | Runtime.remove清理與native分支保留；3個舊斷言恢復 | Rust72＋Node6及fmt/check/WASM通過；指定native多窗／關閉／出籠／重啟由使用者操作確認＋DB核對通過；故障路徑待驗 |
| [Analytics](../daily/day-40/analytics-r1.md#refactor) | csv.rs與原聚合邊界不變 | 本輪真PG53及fmt/check通過；精確篩選／CSV下載瀏覽器通過 |

[增量來源／命令](../daily/day-40/rerun-9c76797/README.md) · [歷史勘誤](../daily/day-40/historical-corrections.md) · [全專案G-01／G-02](../../engineering/README.md)。Day41、FIFO、last_full_comparison不變。

2026-10-28 Day42 候選（產品來源 `0b2288e4996b6d80b00e7a77e2cd023835306528`）：[Chat 狀態顯示決策搬層](../daily/day-42/chat-r1.md)、[Pet 漫遊啟用政策搬層](../daily/day-42/pet-r1.md)、[Analytics URL encoder 搬層](../daily/day-42/analytics-r1.md) 各有獨立 patch 與舊斷言承接；[靜態套用與 SHA256 證據](../daily/day-42/delivery-verification.json) 通過。**僅方案，未套用／未編譯／未驗收；不計為已完成工程。** 三案 next_day 仍42；不更動舊測試或 FIFO。
