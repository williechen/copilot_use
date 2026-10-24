# Pet 現行教學入口

[Day34：新增寵物名稱規則接入 application 並拒絕控制字元](../daily/day-34/pet.md)

教學已交付並完成精確 commit 全文讀回；最新來源已見名稱規則與 caller 接線；本次未執行該功能的 Rust／DB／GUI 測試，功能未驗收。下一篇 Day35。

[首頁](../pet/README.md) · [目前功能與驗證現況](STATUS.md) · [進度](../../learning-state.json) · [manifest](manifest.json) · [CI 補充與來源證據](../daily/day-34/source-snapshot.json)

[前次程式重複套用核對](../../source-review/2026-10-25-duplicate-application-review.md)（來源 `3236b94e`；保留原教學與交付證據，進度不變）。

[全部後續調整與清理條件](../../FUTURE_WORK.md)：依本案順序選最小切片，不重做已接通內容；保留現行 Day 與原始證據。

## 舊內容與現存狀態

[全部文章逐篇核對](ARTICLE_STATUS.md) · [機器可讀狀態](article-states.json)

此案現行文章：名稱規則已移到 pet-application，Tauri 及 PetStore 使用同一函式，含控制字元拒絕與測試；pet-persistence 仍空。本次 Rust／SQLite／GUI 未執行。 教學編號、交付與歷史證據保持原值；當次完整驗收未重跑。
