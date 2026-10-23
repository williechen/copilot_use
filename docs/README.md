# 文件總入口

先讀 [共用文章](COMMON_LEARNING_RULES.md) 的儲存庫分工，再進入各案首頁。`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_chat` 是唯一程式、測試、CI 與功能驗收主體。實作進度與驗證限制只維護在 [目前進度](teaching/current/STATUS.md)，最新下一 Day 以 [learning-state](learning-state.json) 為準。

| 用途 | 文件 |
| --- | --- |
| 共用規則與交付方式 | [共用文章](COMMON_LEARNING_RULES.md) |
| Chat 目標、待辦與教學 | [Chat 首頁](teaching/chat/README.md) |
| Pet 目標、待辦與教學 | [Pet 首頁](teaching/pet/README.md) |
| Analytics 目標、待辦與教學 | [Analytics 首頁](teaching/analytics/README.md) |
| 產品目標與開發清單 | [產品能力與完成條件](P9_TARGET.md) · [開發清單與驗收缺口](FUTURE_WORK.md) |
| 架構與程式銜接 | [系統架構](SYSTEM_ARCHITECTURE.md) · [目錄銜接](TEACHING_ALIGNMENT.md) · [模組對照](SEMANTIC_ARCHITECTURE_REVIEW.md) |
| 使用者與安全需求 | [使用者流程](requirements/USER_FLOWS.md) · [共用驗證](requirements/AUTHENTICATION.md) · [服務憑證](SERVICE_CREDENTIAL_REQUIREMENTS.md) · [UI](UI_ALIGNMENT_REQUIREMENTS.md) |
| 本地操作與驗收 | [本地啟動](LOCAL_START.md) · [操作索引](DEPLOYMENT.md) · [驗收](LOCAL_VALIDATION.md) · [資料移轉](SQLITE_TRANSITION.md) |
| 系統間通訊 | [Analytics Transport](ANALYTICS_TRANSPORT.md) |
| 每日文章與進度 | [新文章](teaching/daily/README.md) · [驗收完成封存](teaching/archived/README.md) · [manifest](teaching/current/manifest.json) |

<details>
<summary>歷史文章、原始封存與工程證據</summary>

[統一歷史資料](teaching/history/README.md) · [原始路徑對照](teaching/history/PATH_MAP.json) · [全文件清單](catalog/ALL_ARTICLES.md) · [來源核對](source-review/README.md) · [完整靜態核對](FULL_SOURCE_REVIEW.md)

</details>

現行架構、操作與驗收以 rust_ai_chat 為程式來源；copilot_use 舊產品腳本、workflow、Release 僅保留原用途，轉換限制見 [分工同步範圍](REPOSITORY_ROLE_TRANSITION.md)；[改寫前原文、來源 commit 與 SHA256](teaching/history/evidence/2026-10-22-current-docs/README.md) 集中保留，歷史驗證仍依原來源解讀。
