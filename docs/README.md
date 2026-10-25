# 文件總入口

[問題文章與後續更正總表](teaching/current/ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

<!-- article-status:start -->
[全部文章現存狀態（99 篇＋舊修訂）](teaching/current/ARTICLE_STATUS.md)：原文、程式所見與歷史／目前驗證分開記錄；已由後續實作取代者勿原樣套回。
<!-- article-status:end -->

<!-- duplicate-review:start -->
[後續調整清單（13 項）](FUTURE_WORK.md) · [原始核對報告](source-review/2026-10-25-duplicate-application-review.md)。可清除、先遷測試與保留項目分別列明，產品套用／驗收另記。
<!-- duplicate-review:end -->

<!-- daily-delivery:start -->
## 最新交付與下一篇

| 專案 | 最新文章 | 下一篇 | 狀態 |
| --- | --- | --- | --- |
| Chat | [Day36：既有 WebSocket 會話於登出後撤銷並停止後續收發](teaching/daily/day-36/chat.md) | Day37 | 教學已交付／程式未套用／未測試／未驗收 |
| Pet | [Day36 r2：重啟恢復全部未回籠寵物，而非只恢復最後選取者](teaching/daily/day-36/pet-r2.md) | Day37 | 教學已交付／程式未套用／未測試／未驗收 |
| Analytics | [Day36：精確事件統計 API 改用人員 Session，不再接受來源 token 查詢](teaching/daily/day-36/analytics.md) | Day37 | 教學已交付／程式未套用／未測試／未驗收 |

文案修復與最新 PG 結果見 [afdbb6e 驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-assertions-afdbb6e.md)；前次編譯修復見 [bb95a60 驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-compile-bb95a60.md)；前次套用／測試見 [e753c36 更新驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-update-e753c36.md)；原 Day35 正文與交付證據保留，完整產品驗收尚未完成。
<!-- daily-delivery:end -->

2026-10-24 目錄已同步至 [williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)：保留 `chat_apps`、`pet_apps`、`analytics_apps`、`crates` 四個 workspace；[新目錄與套件對照](SOURCE_LAYOUT.md)。後續教學依當次來源 SHA 調整程式、Cargo、imports、呼叫端與測試。

2026-10-23 依使用者確認，所有涉及 copilot_use 程式開發、移植、同步適配、重構、套用、建置、測試、CI、產品試驗、發布、安裝更新與功能驗收的工作要求均已廢除，不再列為待辦、選題或驗收門檻。copilot_use 僅維護需求、規則、教學文章、進度、索引與交付證據；文件內的教學程式碼用於說明 rust_ai_ring 的實作，不建立第二套產品。程式功能、測試、CI 與驗收只依 rust_ai_ring 精確 SHA。既有 copilot_use 程式、腳本、workflow、Release 及歷史欄位僅供追溯，不代表仍有維護或執行義務。

先讀 [共用文章](COMMON_LEARNING_RULES.md) 的儲存庫分工，再進入各案首頁。`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_ring` 是唯一程式、測試、CI 與功能驗收主體。實作進度與驗證限制只維護在 [目前進度](teaching/current/STATUS.md)，最新下一 Day 以 [learning-state](learning-state.json) 為準。

| 用途 | 文件 |
| --- | --- |
| 共用規則與交付方式 | [共用文章](COMMON_LEARNING_RULES.md) |
| Chat 目標、待辦與教學 | [Chat 首頁](teaching/chat/README.md) |
| Pet 目標、待辦與教學 | [Pet 首頁](teaching/pet/README.md) |
| Analytics 目標、待辦與教學 | [Analytics 首頁](teaching/analytics/README.md) |
| 現行需求與開發清單 | [使用者流程](requirements/USER_FLOWS.md) · [人員驗證](requirements/AUTHENTICATION.md) · [開發清單與驗收缺口](FUTURE_WORK.md) |
| 架構與程式銜接 | [系統架構](SYSTEM_ARCHITECTURE.md) · [目錄銜接](TEACHING_ALIGNMENT.md) · [模組對照](SEMANTIC_ARCHITECTURE_REVIEW.md) |
| 使用者與安全需求 | [使用者流程](requirements/USER_FLOWS.md) · [共用驗證](requirements/AUTHENTICATION.md) · [服務憑證](SERVICE_CREDENTIAL_REQUIREMENTS.md) · [UI](UI_ALIGNMENT_REQUIREMENTS.md) |
| 本地操作與驗收 | [本地啟動](LOCAL_START.md) · [操作索引](DEPLOYMENT.md) · [驗收](LOCAL_VALIDATION.md) · [資料移轉](SQLITE_TRANSITION.md) |
| 系統間通訊 | [Analytics Transport](ANALYTICS_TRANSPORT.md) |
| 排程狀態與規則入口 | [排程文章狀態](teaching/scheduler-instructions-v1.md)（原任務已更新並重新載入全文讀回確認） |
| 每日文章與進度 | [新文章](teaching/daily/README.md) · [驗收完成封存](teaching/archived/README.md) · [manifest](teaching/current/manifest.json) |

<details>
<summary>歷史文章、原始封存與工程證據</summary>

[統一歷史資料](teaching/history/README.md) · [原始路徑對照](teaching/history/PATH_MAP.json) · [全文件清單](catalog/ALL_ARTICLES.md) · [來源核對](source-review/README.md) · [完整靜態核對](FULL_SOURCE_REVIEW.md)

</details>

現行架構、操作與驗收以 rust_ai_ring 為程式來源；copilot_use 舊產品腳本、workflow、Release 僅供歷史追溯，不再要求維護或執行，轉換限制見 [分工同步範圍](REPOSITORY_ROLE_TRANSITION.md)；[改寫前原文、來源 commit 與 SHA256](teaching/history/evidence/2026-10-22-current-docs/README.md) 集中保留，歷史驗證仍依原來源解讀。
