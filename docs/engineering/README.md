# 全專案重構：獨立項目

> 2026-10-29 Day43 本地實測：固定產品 `8b882696c5548accb9d98a32b27db35e11109c74`；[測試矩陣與原始證據](../source-review/local-validation-2026-10-29-day43/README.md) · [今日交付延遲原因](../source-review/local-validation-2026-10-29-day43/diagnosis.md)。下列舊 SHA 結果保留原範圍；next_day 維持 44，完整 GUI／CI／舊 DB 驗收未完成。

> **2026-10-29 Day43 跨案工程狀態：** 新教學僅為三案**專案內部** Refactor；[Day43 原文與 patch](../teaching/daily/day-43/README.md) 不構成全專案 G-01/G-02 交付、不得重複計功。本次未新增跨案契約或共用 crates patch，無全庫 CI 新驗收；G-01／G-02 原相依與狀態不變。產品 `rust_ai_ring` 保持唯讀，不改 FIFO／`last_full_comparison`。


> 2026-10-28 固定產品來源 `c6708c42f21ac919e57a379848b4977d27a1c3d6`；[本輪原始命令與結果](../source-review/local-validation-2026-10-28-c6708c4/README.md)。本輪只增量重測 fixture、metadata／fmt；其他來源實測維持原 SHA，完整驗收與版本 Gate 尚未完成。


範圍是跨Chat／Pet／Analytics的契約、共用crates、跨workspace依賴、全庫測試與CI；各篇Refactor只處理所屬專案，不在此重複計功。本文是全專案項目正本，不新增教學Day或平行補教FIFO。

| 項目 | 範圍／交付 | 狀態／驗證 | 下一步 |
| --- | --- | --- | --- |
| G-01 [CI責任隔離](2026-10-26/ci-isolation.md) · [完整patch](2026-10-26/ci-isolation.patch) | 拆開Chat SQLite、Pet SQLite／native、Analytics PG的全庫workflow | 方案已交付；原精確來源套用檢查／YAML解析通過；產品未套用／Actions與GUI未驗收 | 使用者套用後核對job／features／services與required checks，故障隔離測試 |
| G-02 共用契約與依賴基線 | shared、ai-core等跨案契約、四workspace依賴、公開DTO／Port歸屬 | 待完整盤點；尚未提供變更patch，不能算已完成重構 | 核對正式caller與依賴方向後，再交付完整調整及相容／測試遷移 |

每項獨立保存來源SHA、相依、完整變更、舊斷言承接、實測／未執行、風險與回復方式。按全專案相依每日漸進，與三案自己的重構進度分開，不因三篇引用同一項目而算三次。產品唯讀，由使用者套用；不增加Day、不更新last_full_comparison。

[各專案Refactor](../teaching/functional-refactoring/README.md) · [各案進度](../teaching/functional-refactoring/PROGRESS.md) · [架構](../SYSTEM_ARCHITECTURE.md) · [共用規則](../COMMON_LEARNING_RULES.md)。

2026-10-26 Day39：未新增跨案 Refactor 或全庫 workflow；G-01 仍為未套用／未驗收方案，G-02 待盤點。三篇 Day39 的各案 Refactor 不計為 G-01／G-02 進度。

2026-10-26 Day40：三案各自 Refactor（Chat 登入雜湊、Pet Runtime／roam、Analytics CSV serializer）不增加 G-01／G-02 完成量；G-01 仍未套用／未驗收，G-02 尚待完整跨 workspace caller 與依賴比對。本日無跨案 patch 或全庫 CI 實測。

2026-10-28 Day42：三篇文章的 Refactor 各自僅涉及 Chat、Pet、Analytics 的專案內部程式，**不構成跨案重構交付**。G-01 仍為待使用者套用及全庫 CI 驗收的獨立方案，G-02 尚待完整 caller／契約盤點。本日沒有新增全專案 patch、沒有全庫 CI 實測、不增加 G-01／G-02 計功。[Day42 各案證據](../teaching/daily/day-42/delivery-verification.json)。
