# Analytics 現行教學入口

[Day35 r2：事件類型精確統計 SQLx Adapter 與正式路由一併接通](../daily/day-35/analytics-r2.md)

**教學已交付／功能未驗收**：精確 commit 全文讀回；產品未套用，PostgreSQL／Rust／HTTP／Leptos／瀏覽器未當次測試。下一篇 **Day36**。r2 修正 r1 漏掉的 source_kind_summary #[cfg(test)] 與正式 Router 註冊，完整接回真實 caller；[r1 原文](../daily/day-35/analytics.md) 僅保留歷史，不照原樣套用。

[Day35 r2 交付與讀回證據](../daily/day-35/analytics-r2-readback.json) · [上一篇 Day34](../daily/day-34/analytics.md)（首次 admin/admin 初始化與強制改密碼）。

[首頁](../analytics/README.md) · [目前功能與驗證現況](STATUS.md) · [進度](../../learning-state.json) · [manifest](manifest.json) · [CI 補充與來源證據](../daily/day-34/source-snapshot.json)

[文字勘誤：預設密碼字元數](../daily/day-34/analytics-corrections-r1.md) · [完整交付與同步證據](../daily/day-34/delivery-verification.json)

[前次程式重複套用核對](../../source-review/2026-10-25-duplicate-application-review.md)（來源 `3236b94e`；保留原教學與交付證據，進度不變）。

[全部後續調整與清理條件](../../FUTURE_WORK.md)：依本案順序選最小切片，不重做已接通內容；保留現行 Day 與原始證據。

## 舊內容與現存狀態

[全部文章逐篇核對](ARTICLE_STATUS.md) · [機器可讀狀態](article-states.json)

此案現行文章：ensure_default 已接主啟動與 CLI，login 允許預設 admin 抵達 Argon2 驗證，首次改密碼路由重用；本文「admin=4 bytes」已有獨立勘誤為5。本次新 PG／HTTP／瀏覽器未重跑。 教學編號、交付與歷史證據保持原值；當次完整驗收未重跑。
