# 各專案Refactor進度

最新來源 `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`。本輪來源／fmt核對沒有新增功能或重構計功，歷史Runtime／PG證據保持原SHA。

| 專案 | 來源現況 | 本輪／後續 |
| --- | --- | --- |
| [Chat](../daily/day-40/chat-r2.md#refactor) | 共用token_digest保留，/account回歸已補回 | fmt通過；新SHA的workspace與瀏覽器尚未重跑 |
| [Pet](../daily/day-40/pet-r2.md#refactor) | Runtime.remove清理與native分支保留；3個舊斷言恢復 | fmt通過；舊候選server60不當新SHA重跑，真native待驗 |
| [Analytics](../daily/day-40/analytics-r1.md#refactor) | csv.rs與原聚合邊界不變 | 前輪PG53沿原SHA；本輪fmt通過 |

[增量來源／命令](../daily/day-40/source-update-9c76797/README.md) · [歷史勘誤](../daily/day-40/historical-corrections.md) · [全專案G-01／G-02](../../engineering/README.md)。Day41、FIFO、last_full_comparison不變。
