# 目前閱讀入口

> **2026-10-30 SSR 修補追驗：** 產品已更新至 `7ce39f0166a2120539e5908b33c88322a930c35a`，正式 main 已套用 Router imports。隔離原樣測試 UI SSR 21 項、server／SQLite／UI SSR 組合 60 項及 metadata／fmt 通過。[新 SHA 命令與 CI 快照](../../source-review/day43-7ce39f0-2026-10-30/README.md)；遠端 CI 仍執行中，未稱全綠。其他 workspace 與 GUI／WASM／DB 保留各自原 SHA 的證據；next_day 三案 44、FIFO、last_full_comparison 不變，完整 Gate2 未結案。下方較早日期與 SHA 為歷史快照。

> 2026-10-29 驗收增量：[網頁排程已儲存、GUI／CI／舊庫實測](../../source-review/acceptance-2026-10-29/README.md)。Chat 真瀏覽器另發現清除搜尋回歸，修補候選已通過；CI attempt #2 仍失敗，Pet 隔離 A/B 原生回籠生命週期已由人工操作及 SQLite 核對通過，其他原生情境／真實舊 PG 備份待驗。Gate2 未結案。

> 2026-10-29 Day43 本地實測：固定產品 `8b882696c5548accb9d98a32b27db35e11109c74`；[測試矩陣與原始證據](../../source-review/local-validation-2026-10-29-day43/README.md) · [今日交付延遲原因](../../source-review/local-validation-2026-10-29-day43/diagnosis.md)。下列舊 SHA 結果保留原範圍；next_day 維持 44，完整 GUI／CI／舊 DB 驗收未完成。

| 專案 | 專案首頁 | 最新文章 | 問題主線 |
|---|---|---|---|
| Chat | [專案大綱](../chat/README.md) | [Day43](../daily/day-43/chat-archive.md) | [原完成條件](../../FUTURE_WORK.md#chat-auth) |
| Pet | [專案大綱](../pet/README.md) | [Day43](../daily/day-43/pet-archive.md) | [原完成條件](../../FUTURE_WORK.md#pet-lifecycle) |
| Analytics | [專案大綱](../analytics/README.md) | [Day43](../daily/day-43/analytics-archive.md) | [原完成條件](../../FUTURE_WORK.md#analytics-auth) |

[功能與測試狀態](STATUS.md) · [問題追蹤](../../FUTURE_WORK.md) · [文章勘誤](ARTICLE_CORRECTIONS.md) · [歷史逐篇對照](ARTICLE_STATUS.md)。

**Gate1 正式發布：** [Day43 三案文章／各自 Refactor 與 Green patch](../daily/day-43/README.md) 已完成精確提交讀回，三案下一待發布為 Day44；**Gate2 產品尚未套用／驗收**。[證據](../daily/day-43/delivery-verification.json) · [版本](../daily/day-43/versions.md)。
