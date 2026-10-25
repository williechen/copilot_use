# Analytics 現行教學入口

**2026-10-26 現行摘要已核對。** 最新程式來源 `9b89656`（upgrade）；新CI快照為1成功／6執行中，尚未全綠。舊648b903為六成功／Persistence失敗，已配置的Analytics PostgreSQL步驟被略過。當日官方版本、manifest／lock與本機工具差異已補查；未升級產品、未重跑測試、未完成整體驗收。[版本與摘要核對](../../source-review/2026-10-26-versions-and-summary-refresh.md)。

**2026-10-25 後續功能分層要求已確認。** 後續教學按功能提供用例／Port／Adapter／UI／正式caller與測試的完整必要重構，沿原功能項追蹤；本次僅更新規則，未重構產品、未新增Day或驗收。[架構正本](../../SYSTEM_ARCHITECTURE.md#functional-layering) · [來源評估與範圍](../../source-review/2026-10-25-functional-layering.md)。

[Day37 r3：驗證更新](../daily/day-37/analytics-r3.md)

**程式已見／PG45項加略過另1項通過／CI受阻／完整驗收待完成**；來源 `648b90323af7b381d6a79dc2fd08a53b63173a43`，下一篇仍為 **Day38**。[精確交付讀回](../daily/day-37/analytics-r3-readback.json) · [本次完整驗證／錯誤與恢復](../../source-review/2026-10-25-day37-verification-648b903.md)。下方先前來源、交付及「未套用／未測試」摘要僅保留其當時範圍，不代表目前狀態。

[Day37 r2：拒絕歧義 Session Cookie，兩種查詢共用 Guard](../daily/day-37/analytics-r2.md)

**教學已交付／程式未套用／新測試未執行／產品未驗收**；下一篇 **Day38**。[精確 commit 全文讀回、blob 與 SHA256](../daily/day-37/analytics-r2-readback.json)。程式來源 `5c713553daee4055738239f047d6c774fcc7a7a1`；以下保留前次來源／驗證摘要，不能當成新 Green 成功。


<!-- comparison-5c713553:start -->
[最新三專案程式／文章比對](../../source-review/2026-10-25-full-comparison-5c713553.md)：105個閱讀版本、111份修訂、21項差異；來源 `5c713553`，局部測試與完整驗收分開記錄。

精確查詢Session修正已在來源；一般帳號／角色／來源ACL仍缺。本段5c713553的CI摘要已過時；648b903已選analytics-server＋analytics-persistence並加postgres-integration，但前段Pet失敗使該步驟skipped，仍未獲CI通過證據。
<!-- comparison-5c713553:end -->

[問題文章與後續更正總表](ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

[Day36 r2：精確事件統計 API 改用人員 Session，不再接受來源 token 查詢](../daily/day-36/analytics-r2.md)

**程式已存在／相關實測通過／完整產品未驗收**；實測來源 rust_ai_ring `5c713553daee4055738239f047d6c774fcc7a7a1`，原始教學基準 `61ed5930`。當時下一篇 **Day37**（現行進度見上方）；[交付全文讀回](../daily/day-36/analytics-r2-readback.json)。

PostgreSQL 全套 42 項通過、原略過項另跑 1 項通過；真 HTTP Session、首次改密碼、登出、過期／停用／fingerprint 及攝取去重通過。角色／來源 ACL、SSE、瀏覽器待驗。 [完整實測報告](../../source-review/2026-10-25-day36-verification-5c713553.md)。原版文章及交付證據均保留。

[上一篇 Day35 r2：精確事件統計 SQLx Adapter](../daily/day-35/analytics-r2.md) 已在後續來源套用並通過 PostgreSQL 增量回歸，但原來源 token 查詢授權不符人員 Session 需求；[Day35 原始交付證據](../daily/day-35/analytics-r2-readback.json)。

[首頁](../analytics/README.md) · [目前功能與驗證現況](STATUS.md) · [進度](../../learning-state.json) · [manifest](manifest.json) · [CI 補充與來源證據](../daily/day-34/source-snapshot.json)

[文字勘誤：預設密碼字元數](../daily/day-34/analytics-corrections-r1.md) · [完整交付與同步證據](../daily/day-34/delivery-verification.json)

[前次程式重複套用核對](../../source-review/2026-10-25-duplicate-application-review.md)（來源 `3236b94e`；保留原教學與交付證據，進度不變）。

[全部後續調整與清理條件](../../FUTURE_WORK.md)：依本案順序選最小切片，不重做已接通內容；保留現行 Day 與原始證據。

## 舊內容與現存狀態

[全部文章逐篇核對](ARTICLE_STATUS.md) · [機器可讀狀態](article-states.json)

此案現行來源：精確查詢 persistence／Dashboard／路由已接通；bb95a60 修編譯、afdbb6e 修四處文字斷言，PostgreSQL 回歸通過。原pg_store公開路徑相容性、角色／來源ACL與server PG CI實際通過仍追蹤（配置已補，執行被Pet失敗阻擋）；精確查詢人員Session已接；r1／r2 正文及原 commit/blob/SHA256 保留，完整功能未驗收。
