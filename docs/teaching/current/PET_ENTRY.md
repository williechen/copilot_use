# Pet 現行教學入口

[現行Day41：控制台逐隻回籠](../daily/day-41/pet.md) · [現況](STATUS.md) · [本輪真正測試](../daily/day-40/rerun-9c76797/README.md)

來源 `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`。Rust72＋Node6通過；原生多窗／關閉／出籠／重啟由使用者操作確認＋DB核對通過。本輪確實在此SHA重跑；歷史失敗仍保留原SHA。CI重跑被403拒絕，完整Gate2未通過。

[歷史r1](../daily/day-40/pet-r1.md) · [歷史更正](../daily/day-40/historical-corrections.md)。下一篇 Day42；原 FIFO 次序不變。

[Pet原生GUI人工操作與資料庫交叉驗證](../daily/day-40/native-gui-9c76797/README.md)：指定多窗、原生關閉、重新出籠與兩種重啟流程通過，完整Gate2未通過。

> **Day41 文章已交付／產品未套用、未執行本篇 Red/Green 或 Refactor 驗證、未驗收。** [Refactor patch](../daily/day-41/pet-day41-refactor.patch) · [Green patch](../daily/day-41/pet-day41-green.patch) · [全文讀回與 SHA256](../daily/day-41/delivery-verification.json) · [Rust stable 更正](../daily/day-41/version-correction.md)。上方 Day40 驗證不追認 Day41。
