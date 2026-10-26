# 各專案Refactor進度

最新來源 `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`。本輪真正重跑測試與可行瀏覽器流程，沒有新增功能或重構計功；歷史證據保持原SHA。

| 專案 | 來源現況 | 本輪／後續 |
| --- | --- | --- |
| [Chat](../daily/day-40/chat-r2.md#refactor) | 共用token_digest保留，/account回歸已補回 | Rust59及fmt/check/WASM通過；指定登入／管理／登出瀏覽器流程通過 |
| [Pet](../daily/day-40/pet-r2.md#refactor) | Runtime.remove清理與native分支保留；3個舊斷言恢復 | Rust72＋Node6及fmt/check/WASM通過；指定native多窗／關閉／出籠／重啟由使用者操作確認＋DB核對通過；故障路徑待驗 |
| [Analytics](../daily/day-40/analytics-r1.md#refactor) | csv.rs與原聚合邊界不變 | 本輪真PG53及fmt/check通過；精確篩選／CSV下載瀏覽器通過 |

[增量來源／命令](../daily/day-40/rerun-9c76797/README.md) · [歷史勘誤](../daily/day-40/historical-corrections.md) · [全專案G-01／G-02](../../engineering/README.md)。Day41、FIFO、last_full_comparison不變。
