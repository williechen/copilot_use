# Analytics 現行教學入口

[Day35 r2：事件類型精確統計 SQLx Adapter 與正式路由一併接通](../daily/day-35/analytics-r2.md)

**已套用／驗證結果**：bb95a60 已消除五個 E0425，全部 PG 測試目標可編譯；完整回歸 41 passed／3 failed／1 ignored。三敗為舊 Dashboard 文案，來源 token／人員 Session 查詢需求落差仍待修。下一篇 **Day36**。[完整結果與勘誤](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-compile-bb95a60.md)。

[Day35 r2 交付與讀回證據](../daily/day-35/analytics-r2-readback.json) · [上一篇 Day34](../daily/day-34/analytics.md)（首次 admin/admin 初始化與強制改密碼）。

[首頁](../analytics/README.md) · [目前功能與驗證現況](STATUS.md) · [進度](../../learning-state.json) · [manifest](manifest.json) · [CI 補充與來源證據](../daily/day-34/source-snapshot.json)

[文字勘誤：預設密碼字元數](../daily/day-34/analytics-corrections-r1.md) · [完整交付與同步證據](../daily/day-34/delivery-verification.json)

[前次程式重複套用核對](../../source-review/2026-10-25-duplicate-application-review.md)（來源 `3236b94e`；保留原教學與交付證據，進度不變）。

[全部後續調整與清理條件](../../FUTURE_WORK.md)：依本案順序選最小切片，不重做已接通內容；保留現行 Day 與原始證據。

## 舊內容與現存狀態

[全部文章逐篇核對](ARTICLE_STATUS.md) · [機器可讀狀態](article-states.json)

此案現行來源：精確查詢 persistence／Dashboard／路由已接通；測試改用新函式路徑與局部 fixture，五個編譯錯誤已修。未恢復舊 pg_store 公開匯出；三個舊斷言與查詢授權契約仍待修。r1／r2 正文及原 commit/blob/SHA256 保留，完整功能未驗收。
