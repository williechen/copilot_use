# Chat 現行教學入口

[問題文章與後續更正總表](ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

[Day36 r2：既有 WebSocket 會話於登出後撤銷並停止後續收發](../daily/day-36/chat-r2.md)

**程式已存在／相關實測通過／完整產品未驗收**；實測來源 rust_ai_ring `5c713553daee4055738239f047d6c774fcc7a7a1`，原始教學基準 `61ed5930`。下一篇 **Day37**；[交付全文讀回](../daily/day-36/chat-r2-readback.json)。

Server 31、UI 14 項測試與 WASM 通過；真 HTTP／WS 五情境通過。每秒重查不等於硬性 ≤1 秒；原子撤銷競態、ACL／歷史及瀏覽器待驗。 [完整實測報告](../../source-review/2026-10-25-day36-verification-5c713553.md)。原版文章及交付證據均保留。

[上一篇 Day35：建房名稱規則共用](../daily/day-35/chat.md) 已在後續來源套用並完成增量測試，但整體功能仍待驗收；[Day35 原始交付證據](../daily/day-35/chat-readback.json)。

[首頁](../chat/README.md) · [功能與驗證現況](STATUS.md) · [進度](../../learning-state.json) · [manifest](manifest.json) · [r2 交付證據](../daily/day-34/chat-r2-readback.json)

[前次程式重複套用核對](../../source-review/2026-10-25-duplicate-application-review.md)（來源 `3236b94e`；保留原教學與交付證據，進度不變）。

[全部後續調整與清理條件](../../FUTURE_WORK.md)：依本案順序選最小切片，不重做已接通內容；保留現行 Day 與原始證據。

## 舊內容與現存狀態

[全部文章逐篇核對](ARTICLE_STATUS.md) · [機器可讀狀態](article-states.json)

此案現行來源：Day35 共用房名規則已接回兩個 handler，Session／Origin／CSRF 及 JSON 400／HTML 422 保留；目前測試通過。共用認證 FIFO／ACL／歷史仍待完成。 教學正文與原 commit/blob/SHA256 保留；局部測試不代表整產品功能驗收。
