# 每日新文章

每日功能教學與全專案漸進工程分開交付及驗收。核心60分鐘只計Red／Green，局部Refactor、工程與其他交付另計；rust_ai_ring唯讀，使用者自行套用。

[共用規則](../../COMMON_LEARNING_RULES.md) · [功能與驗證](../current/STATUS.md) · [工程進度](../functional-refactoring/PROGRESS.md) · [學習進度](../../learning-state.json) · [manifest](../current/manifest.json)

| 專案 | 現行教學 | 下一篇 | 驗證範圍 |
| --- | --- | --- | --- |
| Chat | [Day38 r2：搜尋失敗可見狀態與既有房間連結](day-38/chat-r2.md) | Day39 | 完整patch與精確讀回；套用檢查／語法解析通過；產品未套用、Red／Green／DB／GUI未執行 |
| Pet | [Day38 r2：逐 UUID 生活窗漫遊任務](day-38/pet-r2.md) | Day39 | 完整patch與精確讀回；套用檢查／語法解析通過；產品未套用、Red／Green／DB／GUI未執行 |
| Analytics | [Day38 r2：上報拒絕重複 Authorization](day-38/analytics-r2.md) | Day39 | 完整patch與精確讀回；套用檢查／語法解析通過；產品未套用、Red／Green／DB／GUI未執行 |

[Chat首頁](../chat/README.md) · [Pet首頁](../pet/README.md) · [Analytics首頁](../analytics/README.md)

目前產品來源 `9b89656`；既有七個workflow全success，[CI快照](day-38/r2-product-ci-observation.json)。這些既有結果不驗證尚未套用的候選patch。[當日版本與原始證據](day-38/r2-versions.json)。

[每日工程：CI責任隔離完整工作包](../../engineering/2026-10-26/ci-isolation.md)；方案已交付，未套用／未驗收。其他分層與測試遷移沿工程進度接續。

## 使用與追溯

[使用者流程](../../requirements/USER_FLOWS.md) · [人員驗證](../../requirements/AUTHENTICATION.md) · [功能待辦](../../FUTURE_WORK.md) · [架構](../../SYSTEM_ARCHITECTURE.md) · [來源目錄](../../SOURCE_LAYOUT.md) · [階段規劃](../../ROADMAP_P0_P12.md) · [實際網頁排程](../scheduler-instructions-v1.md)

[原文章與驗證總表](../current/ARTICLE_STATUS.md) · [文章更正對照](../current/ARTICLE_CORRECTIONS.md) · [歷史目錄](../history/README.md) · [驗收封存](../archived/README.md) · [三案本修訂讀回](day-38/r2-delivery-verification.json)。保留各次來源、原文與證據；歷史測試不追認目前驗收。
