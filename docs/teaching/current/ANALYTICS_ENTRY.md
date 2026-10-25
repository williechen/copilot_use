# Analytics 現行教學入口

[問題文章與後續更正總表](ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

[Day36 r2：精確事件統計 API 改用人員 Session，不再接受來源 token 查詢](../daily/day-36/analytics-r2.md)

**程式已存在／相關實測通過／完整產品未驗收**；實測來源 rust_ai_ring `5c713553daee4055738239f047d6c774fcc7a7a1`，原始教學基準 `61ed5930`。下一篇 **Day37**；[交付全文讀回](../daily/day-36/analytics-r2-readback.json)。

PostgreSQL 全套 42 項通過、原略過項另跑 1 項通過；真 HTTP Session、首次改密碼、登出、過期／停用／fingerprint 及攝取去重通過。角色／來源 ACL、SSE、瀏覽器待驗。 [完整實測報告](../../source-review/2026-10-25-day36-verification-5c713553.md)。原版文章及交付證據均保留。

[上一篇 Day35 r2：精確事件統計 SQLx Adapter](../daily/day-35/analytics-r2.md) 已在後續來源套用並通過 PostgreSQL 增量回歸，但原來源 token 查詢授權不符人員 Session 需求；[Day35 原始交付證據](../daily/day-35/analytics-r2-readback.json)。

[首頁](../analytics/README.md) · [目前功能與驗證現況](STATUS.md) · [進度](../../learning-state.json) · [manifest](manifest.json) · [CI 補充與來源證據](../daily/day-34/source-snapshot.json)

[文字勘誤：預設密碼字元數](../daily/day-34/analytics-corrections-r1.md) · [完整交付與同步證據](../daily/day-34/delivery-verification.json)

[前次程式重複套用核對](../../source-review/2026-10-25-duplicate-application-review.md)（來源 `3236b94e`；保留原教學與交付證據，進度不變）。

[全部後續調整與清理條件](../../FUTURE_WORK.md)：依本案順序選最小切片，不重做已接通內容；保留現行 Day 與原始證據。

## 舊內容與現存狀態

[全部文章逐篇核對](ARTICLE_STATUS.md) · [機器可讀狀態](article-states.json)

此案現行來源：精確查詢 persistence／Dashboard／路由已接通；bb95a60 修編譯、afdbb6e 修四處文字斷言，PostgreSQL 回歸通過。原 pg_store 公開路徑變更與查詢授權契約仍追蹤；r1／r2 正文及原 commit/blob/SHA256 保留，完整功能未驗收。
