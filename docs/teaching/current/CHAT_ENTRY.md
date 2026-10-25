# Chat 現行教學入口

[問題文章與後續更正總表](ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

[Day35：建房名稱規則共用：拒絕控制字元且兩種入口一致](../daily/day-35/chat.md)

**已套用／驗證結果**：Chat Day35 已在 e753c36 套用；application／server／整合 35 項、UI SSR 14 項通過。後續 hydration、登入／建房／搜尋已局部實測，Session／ACL／歷史與完整功能仍未通過驗收。下一篇 **Day36**。 [完整結果與勘誤](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-update-e753c36.md)。

[Day35 交付與讀回證據](../daily/day-35/chat-readback.json) · [上一篇 Day34 r2](../daily/day-34/chat-r2.md)（登入入口去重已見來源，未當次驗收）。

[首頁](../chat/README.md) · [功能與驗證現況](STATUS.md) · [進度](../../learning-state.json) · [manifest](manifest.json) · [r2 交付證據](../daily/day-34/chat-r2-readback.json)

[前次程式重複套用核對](../../source-review/2026-10-25-duplicate-application-review.md)（來源 `3236b94e`；保留原教學與交付證據，進度不變）。

[全部後續調整與清理條件](../../FUTURE_WORK.md)：依本案順序選最小切片，不重做已接通內容；保留現行 Day 與原始證據。

## 舊內容與現存狀態

[全部文章逐篇核對](ARTICLE_STATUS.md) · [機器可讀狀態](article-states.json)

此案現行來源：Day35 共用房名規則已接回兩個 handler，Session／Origin／CSRF 及 JSON 400／HTML 422 保留；目前測試通過。共用認證 FIFO／ACL／歷史仍待完成。 教學正文與原 commit/blob/SHA256 保留；局部測試不代表整產品功能驗收。
