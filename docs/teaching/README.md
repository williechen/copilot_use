# 三專案教學首頁

[問題文章與後續更正總表](current/ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

<!-- article-status:start -->
[全部文章現存狀態（99 篇＋舊修訂）](current/ARTICLE_STATUS.md)：原文、程式所見與歷史／目前驗證分開記錄；已由後續實作取代者勿原樣套回。
<!-- article-status:end -->

<!-- duplicate-review:start -->
[後續調整清單（13 項）](../FUTURE_WORK.md) · [原始核對報告](../source-review/2026-10-25-duplicate-application-review.md)。可清除、先遷測試與保留項目分別列明，產品套用／驗收另記。
<!-- duplicate-review:end -->

<!-- daily-delivery:start -->
## 最新交付與下一篇

| 專案 | 最新文章 | 下一篇 | 狀態 |
| --- | --- | --- | --- |
| Chat | [Day35：建房名稱規則共用：拒絕控制字元且兩種入口一致](daily/day-35/chat.md) | Day36 | e753c36 已套用；增量測試通過／完整驗收待做 |
| Pet | [Day35：逐寵物回籠狀態搬入 pet-persistence，接回 Tauri 真實呼叫鏈](daily/day-35/pet.md) | Day36 | e753c36 已套用；增量測試通過／GUI 待驗 |
| Analytics | [Day35 r2：事件類型精確統計 SQLx Adapter 與正式路由一併接通](daily/day-35/analytics-r2.md) | Day36 | afdbb6e PG 回歸通過；授權契約／完整驗收待做 |

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
