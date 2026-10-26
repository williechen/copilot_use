# 全專案重構：獨立項目

範圍是跨Chat／Pet／Analytics的契約、共用crates、跨workspace依賴、全庫測試與CI；各篇Refactor只處理所屬專案，不在此重複計功。本文是全專案項目正本，不新增教學Day或平行補教FIFO。

| 項目 | 範圍／交付 | 狀態／驗證 | 下一步 |
| --- | --- | --- | --- |
| G-01 [CI責任隔離](2026-10-26/ci-isolation.md) · [完整patch](2026-10-26/ci-isolation.patch) | 拆開Chat SQLite、Pet SQLite／native、Analytics PG的全庫workflow | 方案已交付；原精確來源套用檢查／YAML解析通過；產品未套用／Actions與GUI未驗收 | 使用者套用後核對job／features／services與required checks，故障隔離測試 |
| G-02 共用契約與依賴基線 | shared、ai-core等跨案契約、四workspace依賴、公開DTO／Port歸屬 | 待完整盤點；尚未提供變更patch，不能算已完成重構 | 核對正式caller與依賴方向後，再交付完整調整及相容／測試遷移 |

每項獨立保存來源SHA、相依、完整變更、舊斷言承接、實測／未執行、風險與回復方式。按全專案相依每日漸進，與三案自己的重構進度分開，不因三篇引用同一項目而算三次。產品唯讀，由使用者套用；不增加Day、不更新last_full_comparison。

[各專案Refactor](../teaching/functional-refactoring/README.md) · [各案進度](../teaching/functional-refactoring/PROGRESS.md) · [架構](../SYSTEM_ARCHITECTURE.md) · [共用規則](../COMMON_LEARNING_RULES.md)。
