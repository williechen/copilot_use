# Chat 現行教學入口

**2026-10-25 後續功能分層要求已確認。** 後續教學按功能提供用例／Port／Adapter／UI／正式caller與測試的完整必要重構，沿原功能項追蹤；本次僅更新規則，未重構產品、未新增Day或驗收。[架構正本](../../SYSTEM_ARCHITECTURE.md#functional-layering) · [來源評估與範圍](../../source-review/2026-10-25-functional-layering.md)。

[Day37 r3：驗證更新](../daily/day-37/chat-r3.md)

**程式已見／15項與WASM建置通過／瀏覽器錯誤提示未通過**；來源 `648b90323af7b381d6a79dc2fd08a53b63173a43`，下一篇仍為 **Day38**。[精確交付讀回](../daily/day-37/chat-r3-readback.json) · [本次完整驗證／錯誤與恢復](../../source-review/2026-10-25-day37-verification-648b903.md)。下方先前來源、交付及「未套用／未測試」摘要僅保留其當時範圍，不代表目前狀態。

**2026-10-25 首頁操作回報：尚未修復／未驗收。** 使用者已登入仍見「登入」，進入首頁仍須按讀取才有房間；最新來源 `dbe00814` 靜態確認固定登入入口與缺少初次自動載入。[完整證據與完成條件](../../source-review/2026-10-25-chat-homepage-feedback.md)。本來源已見 Day37 搜尋完成函式與 caller；下方舊來源的「未套用」只限原交付時點。本次產品唯讀、未新跑測試、不推進 Day。

[Day37 r2：房間搜尋完成時更新真實畫面狀態](../daily/day-37/chat-r2.md)

**教學已交付／程式未套用／新測試未執行／產品未驗收**；下一篇 **Day38**。[精確 commit 全文讀回、blob 與 SHA256](../daily/day-37/chat-r2-readback.json)。程式來源 `5c713553daee4055738239f047d6c774fcc7a7a1`；以下保留前次來源／驗證摘要，不能當成新 Green 成功。


<!-- comparison-5c713553:start -->
[最新三專案程式／文章比對](../../source-review/2026-10-25-full-comparison-5c713553.md)：105個閱讀版本、111份修訂、21項差異；來源 `5c713553`，局部測試與完整驗收分開記錄。

目前另見查詢成功仍留「查詢中…」及兩段房間status；refresh_result_view有WASM真caller，不能和舊helper一併當unused。
<!-- comparison-5c713553:end -->

[問題文章與後續更正總表](ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

[Day36 r2：既有 WebSocket 會話於登出後撤銷並停止後續收發](../daily/day-36/chat-r2.md)

**程式已存在／相關實測通過／完整產品未驗收**；實測來源 rust_ai_ring `5c713553daee4055738239f047d6c774fcc7a7a1`，原始教學基準 `61ed5930`。當時下一篇 **Day37**（現行進度見上方）；[交付全文讀回](../daily/day-36/chat-r2-readback.json)。

Server 31、UI 14 項測試與 WASM 通過；真 HTTP／WS 五情境通過。每秒重查不等於硬性 ≤1 秒；原子撤銷競態、ACL／歷史及瀏覽器待驗。 [完整實測報告](../../source-review/2026-10-25-day36-verification-5c713553.md)。原版文章及交付證據均保留。

[上一篇 Day35：建房名稱規則共用](../daily/day-35/chat.md) 已在後續來源套用並完成增量測試，但整體功能仍待驗收；[Day35 原始交付證據](../daily/day-35/chat-readback.json)。

[首頁](../chat/README.md) · [功能與驗證現況](STATUS.md) · [進度](../../learning-state.json) · [manifest](manifest.json) · [登入頁合併原交付證據](../daily/day-34/chat-r2-readback.json)

[前次程式重複套用核對](../../source-review/2026-10-25-duplicate-application-review.md)（來源 `3236b94e`；保留原教學與交付證據，進度不變）。

[全部後續調整與清理條件](../../FUTURE_WORK.md)：依本案順序選最小切片，不重做已接通內容；保留現行 Day 與原始證據。

## 舊內容與現存狀態

[全部文章逐篇核對](ARTICLE_STATUS.md) · [機器可讀狀態](article-states.json)

此案現行來源：Day35 共用房名規則已接回兩個 handler，Session／Origin／CSRF 及 JSON 400／HTML 422 保留；目前測試通過。共用認證 FIFO／ACL／歷史仍待完成。 教學正文與原 commit/blob/SHA256 保留；局部測試不代表整產品功能驗收。
