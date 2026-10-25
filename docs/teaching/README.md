# 三專案教學首頁

每日並行：[功能教學與全專案漸進工程](functional-refactoring/README.md)分開交付及驗收。核心60分鐘只計教學Red／Green，局部Refactor與工程另計；產品唯讀，由使用者套用工程patch。

**2026-10-26 現行摘要已核對。** 最新程式來源 `9b89656`（upgrade）；新CI快照為1成功／6執行中，尚未全綠。舊648b903為六成功／Persistence失敗，已配置的Analytics PostgreSQL步驟被略過。當日官方版本、manifest／lock與本機工具差異已補查；未升級產品、未重跑測試、未完成整體驗收。[版本與摘要核對](../source-review/2026-10-26-versions-and-summary-refresh.md)。

<!-- comparison-5c713553:start -->
[最新三專案程式／文章比對](../source-review/2026-10-25-full-comparison-5c713553.md)：105個閱讀版本、111份修訂、21項差異；來源 `5c713553`，局部測試與完整驗收分開記錄。
<!-- comparison-5c713553:end -->

[問題文章與後續更正總表](current/ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

<!-- article-status:start -->
[全部文章現存狀態（105個閱讀版本／111份修訂）](current/ARTICLE_STATUS.md)：原文、程式所見與歷史／目前驗證分開記錄；已由後續實作取代者勿原樣套回。
<!-- article-status:end -->

<!-- duplicate-review:start -->
[後續調整清單（13 項）](../FUTURE_WORK.md) · [原始核對報告](../source-review/2026-10-25-duplicate-application-review.md)。可清除、先遷測試與保留項目分別列明，產品套用／驗收另記。
<!-- duplicate-review:end -->

<!-- daily-delivery:start -->
## 最新交付與下一篇

| 專案 | 最新文章 | 下一篇 | 狀態 |
| --- | --- | --- | --- |
| Chat | [Day38：搜尋失敗顯示在真正的 Leptos 房間列表](daily/day-38/chat.md) | Day39 | 教學已交付／產品未套用／Red-Green與瀏覽器未執行／未驗收 |
| Pet | [Day38：多隻生活窗各自漫遊、逐 UUID 任務隔離](daily/day-38/pet.md) | Day39 | 教學已交付／產品未套用／原生測試與 GUI 未執行／未驗收 |
| Analytics | [Day38：機器上報拒絕重複 Authorization，獨立 PostgreSQL CI](daily/day-38/analytics.md) | Day39 | 教學已交付／產品未套用／PostgreSQL與CI未執行／未驗收 |

文案修復與最新 PG 結果見 [afdbb6e 驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-assertions-afdbb6e.md)；前次編譯修復見 [bb95a60 驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-compile-bb95a60.md)；前次套用／測試見 [e753c36 更新驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-update-e753c36.md)；原 Day35 正文與交付證據保留，完整產品驗收尚未完成。
<!-- daily-delivery:end -->

2026-10-24 目錄已同步至 [williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)：保留 `chat_apps`、`pet_apps`、`analytics_apps`、`crates` 四個 workspace；[新目錄與套件對照](../SOURCE_LAYOUT.md)。後續教學依當次來源 SHA 調整程式、Cargo、imports、呼叫端與測試。

[共用文章與規則](../COMMON_LEARNING_RULES.md) · [目前進度](current/STATUS.md) · [最新進度檔](../learning-state.json)

| 專案 | 首頁 |
| --- | --- |
| Chat | [目標、操作主線、待辦與全部 Day](chat/README.md) |
| Pet | [目標、操作主線、待辦與全部 Day](pet/README.md) |
| Analytics | [目標、操作主線、待辦與全部 Day](analytics/README.md) |


[新文章目錄](daily/README.md) · [驗收完成文章](archived/README.md) · [既有歷史文章與原始證據](history/README.md)

每新增一個專案 Day 就獨立 commit、推送與全文讀回，再條件同步狀態和 manifest。教學交付與產品驗收分開；資料整理不增加 Day。
