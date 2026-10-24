# Chat 現行教學入口

[Day34 r2：重用 SignInPage，合併重複登入入口](../daily/day-34/chat-r2.md)

教學修訂已交付並完成精確 commit 全文讀回；產品去重尚未套用、測試未執行、功能未驗收。下一篇維持 Day35。

沿用既有 `/signin` 的 SignInPage；GET `/login` 改導向 `/signin`，POST `/login` 沿用原認證。r1 的新增 LoginView 方向已撤回；[原文及原始證據](../daily/day-34/chat.md) 保留。

[首頁](../chat/README.md) · [功能與驗證現況](STATUS.md) · [進度](../../learning-state.json) · [manifest](manifest.json) · [r2 交付證據](../daily/day-34/chat-r2-readback.json)

[最新程式重複套用核對](../../source-review/2026-10-25-duplicate-application-review.md)（來源 `3236b94e`；保留原教學與交付證據，進度不變）。
