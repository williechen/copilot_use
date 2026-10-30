# 跨案工程現況

| 原項目 | 目前狀態 | 下一步 |
| --- | --- | --- |
| G-01 CI 責任隔離 | 舊方案已交付，未計為套用完成；6b34的Persistence仍失敗；舊基準 patch 已封存 | 按目前 workflow 重核 package、features、services、required checks，重新交付可套用版本與故障隔離驗證 |
| G-02 共用契約與相依 | 尚待完整盤點與變更方案；shared 仍依賴 chat-domain | 核對四 workspace caller／DTO／Port／依賴，保留相容性與斷言再調整 |

本頁只處理跨案契約、共用 crates、跨 workspace 依賴及全庫 CI；不塞入各案 Refactor、不重複計功、不增加 Day。舊方案與證據見 [既有分包](../archive/packages-2026-10-30/README.md)。[各案重構](../teaching/functional-refactoring/PROGRESS.md) · [規則](../COMMON_LEARNING_RULES.md)

本次四workspace metadata／fmt／check／clippy與各套件測試已跑，但不代表G-01拆job或G-02解耦完成。Analytics fixture問題與修補只在該案文章；[原始CI／矩陣](../source-review/local-validation-2026-10-30-day44/README.md)。
