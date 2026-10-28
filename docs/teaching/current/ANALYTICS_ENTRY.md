# Analytics 現行教學入口

> 2026-10-28 現行驗證修訂：產品 `e8a640978732651cbbca1483b07e771f876a439b` 已含三案 Day42 實作／各案 Refactor；[本地實測矩陣與限制](../../source-review/local-validation-2026-10-28-e8a6409/README.md) 優先於下列舊來源現況摘要。歷史結果維持原 SHA，不追認本次通過；GUI／CI／完整版本 Gate 未完成，next_day 三案維持 42。


[現行Day41：管理後台事件類型明細](../daily/day-41/analytics.md) · [現況](STATUS.md) · [本輪真正測試](../daily/day-40/rerun-9c76797/README.md)

來源 `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`。本輪真PG53、fmt/check通過；瀏覽器精確篩選／CSV下載通過。本輪確實在此SHA重跑；歷史失敗仍保留原SHA。CI重跑被403拒絕，完整Gate2未通過。

[歷史r1](../daily/day-40/analytics-r1.md) · [歷史更正](../daily/day-40/historical-corrections.md)。下一篇 Day42；原 FIFO 次序不變。

> **Day41 文章已交付／產品未套用、未執行本篇 Red/Green 或 Refactor 驗證、未驗收。** [Refactor patch](../daily/day-41/analytics-day41-refactor.patch) · [Green patch](../daily/day-41/analytics-day41-green.patch) · [全文讀回與 SHA256](../daily/day-41/delivery-verification.json) · [Rust stable 更正](../daily/day-41/version-correction.md)。上方 Day40 驗證不追認 Day41。
