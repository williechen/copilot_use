# Chat 現行教學入口

[Day34 r2：重用 SignInPage，合併重複登入入口](../daily/day-34/chat-r2.md)

教學修訂已交付並完成精確 commit 全文讀回；最新來源已見 r2 去重與相應測試；本次 Rust／HTTP／瀏覽器未執行，功能未驗收。下一篇維持 Day35。

沿用既有 `/signin` 的 SignInPage；GET `/login` 改導向 `/signin`，POST `/login` 沿用原認證。r1 的新增 LoginView 方向已撤回；[原文及原始證據](../daily/day-34/chat.md) 保留。

[首頁](../chat/README.md) · [功能與驗證現況](STATUS.md) · [進度](../../learning-state.json) · [manifest](manifest.json) · [r2 交付證據](../daily/day-34/chat-r2-readback.json)

[前次程式重複套用核對](../../source-review/2026-10-25-duplicate-application-review.md)（來源 `3236b94e`；保留原教學與交付證據，進度不變）。

[全部後續調整與清理條件](../../FUTURE_WORK.md)：依本案順序選最小切片，不重做已接通內容；保留現行 Day 與原始證據。

## 舊內容與現存狀態

[全部文章逐篇核對](ARTICLE_STATUS.md) · [機器可讀狀態](article-states.json)

此案現行文章：現行閱讀版本為 r2；最新來源已移除 LoginView／模組宣告，GET /login redirect /signin 與唯一表單測試已見，POST /login 保留。本次尚未執行 Rust／HTTP／瀏覽器，不追認 Gate 2。 教學編號、交付與歷史證據保持原值。
