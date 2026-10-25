# copilot_use

<!-- comparison-5c713553:start -->
[最新三專案程式／文章比對](docs/source-review/2026-10-25-full-comparison-5c713553.md)：105個閱讀版本、111份修訂、21項差異；來源 `5c713553`，局部測試與完整驗收分開記錄。
<!-- comparison-5c713553:end -->

[問題文章與後續更正總表](docs/teaching/current/ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

<!-- article-status:start -->
[全部文章現存狀態（105個閱讀版本／111份修訂）](docs/teaching/current/ARTICLE_STATUS.md)：原文、程式所見與歷史／目前驗證分開記錄；已由後續實作取代者勿原樣套回。
<!-- article-status:end -->

<!-- duplicate-review:start -->
[後續調整清單（13 項）](docs/FUTURE_WORK.md) · [原始核對報告](docs/source-review/2026-10-25-duplicate-application-review.md)。可清除、先遷測試與保留項目分別列明，產品套用／驗收另記。
<!-- duplicate-review:end -->

<!-- daily-delivery:start -->
## 最新交付與下一篇

| 專案 | 最新文章 | 下一篇 | 狀態 |
| --- | --- | --- | --- |
| Chat | [Day36 r2：既有 WebSocket 會話於登出後撤銷並停止後續收發](docs/teaching/daily/day-36/chat-r2.md) | Day37 | 程式已存在／相關實測通過／完整產品未驗收 |
| Pet | [Day36 r3：重啟恢復全部未回籠寵物，而非只恢復最後選取者](docs/teaching/daily/day-36/pet-r3.md) | Day37 | 程式已存在／相關實測通過／完整產品未驗收 |
| Analytics | [Day36 r2：精確事件統計 API 改用人員 Session，不再接受來源 token 查詢](docs/teaching/daily/day-36/analytics-r2.md) | Day37 | 程式已存在／相關實測通過／完整產品未驗收 |

文案修復與最新 PG 結果見 [afdbb6e 驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-assertions-afdbb6e.md)；前次編譯修復見 [bb95a60 驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-compile-bb95a60.md)；前次套用／測試見 [e753c36 更新驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-update-e753c36.md)；原 Day35 正文與交付證據保留，完整產品驗收尚未完成。
<!-- daily-delivery:end -->

2026-10-24 目錄已同步至 [williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)：保留 `chat_apps`、`pet_apps`、`analytics_apps`、`crates` 四個 workspace；[新目錄與套件對照](docs/SOURCE_LAYOUT.md)。後續教學依當次來源 SHA 調整程式、Cargo、imports、呼叫端與測試。

2026-10-23 依使用者確認，所有涉及 copilot_use 程式開發、移植、同步適配、重構、套用、建置、測試、CI、產品試驗、發布、安裝更新與功能驗收的工作要求均已廢除，不再列為待辦、選題或驗收門檻。copilot_use 僅維護需求、規則、教學文章、進度、索引與交付證據；文件內的教學程式碼用於說明 rust_ai_ring 的實作，不建立第二套產品。程式功能、測試、CI 與驗收只依 rust_ai_ring 精確 SHA。既有 copilot_use 程式、腳本、workflow、Release 及歷史欄位僅供追溯，不代表仍有維護或執行義務。

Chat、Desktop Pet 與 Analytics 的規則、教學與交付證據庫。`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_ring` 是唯一程式、測試、CI 與功能驗收主體。

## 從這裡開始

| 入口 | 內容 |
| --- | --- |
| [共用文章與規則](https://github.com/williechen/copilot_use/blob/main/docs/COMMON_LEARNING_RULES.md) | 共用學習方式、文章格式、交付與驗證規則 |
| [Chat 首頁](https://github.com/williechen/copilot_use/blob/main/docs/teaching/chat/README.md) | 聊天系統目標、操作主線、目前教學與待辦入口 |
| [Pet 首頁](https://github.com/williechen/copilot_use/blob/main/docs/teaching/pet/README.md) | 桌面寵物目標、操作主線、目前教學與待辦入口 |
| [Analytics 首頁](https://github.com/williechen/copilot_use/blob/main/docs/teaching/analytics/README.md) | 統計後台目標、操作主線、目前教學與待辦入口 |

[目前進度與待完成項目](https://github.com/williechen/copilot_use/blob/main/docs/teaching/current/STATUS.md) · [最新學習進度](https://github.com/williechen/copilot_use/blob/main/docs/learning-state.json) · [文件總目錄](https://github.com/williechen/copilot_use/blob/main/docs/README.md)


## 目前目錄與每日排程

| Workspace | 程式位置 |
| --- | --- |
| Chat | `chat_apps/chat_server`、`chat_ui`、`chat_domain`、`chat_application`、`chat_persistence` |
| Pet | `pet_apps/pet_server`、`pet_ui`、`pet_domain`、`pet_application`、`pet_persistence` |
| Analytics | `analytics_apps/analytics_server`、`analytics_persistence` |
| 共用 | `crates/ai_core`、`shared`、`telemetry` |

[目錄及 Cargo package 對照](docs/SOURCE_LAYOUT.md) · [已更新的每日排程指令與讀回證據](docs/teaching/scheduler-instructions-v1.md)。每日交付後同步本頁、文件／教學／每日目錄及三案首頁的最新文章連結與狀態。

## 主體程式

程式來源為 [williechen/rust_ai_ring/main](https://github.com/williechen/rust_ai_ring)。[架構與程式入口](docs/SYSTEM_ARCHITECTURE.md)、[測試與驗收範圍](docs/LOCAL_VALIDATION.md) 依該庫來源 SHA 核對；本庫既有 apps／crates、工具與 Release 不作主體程式驗收。

## 開發與教學

規則、現行共用文章、三專案首頁、next_day／manifest、交付讀回與歷史資料統一維護於本庫 `main`；程式與程式 CI 以 `rust_ai_ring/main` 為主。從儲存庫首頁與文件目錄都能開啟同一份現行內容。

- [產品目標與工作清單](https://github.com/williechen/copilot_use/blob/main/docs/FUTURE_WORK.md)
- [系統架構](https://github.com/williechen/copilot_use/blob/main/docs/SYSTEM_ARCHITECTURE.md)
- [本地啟動](https://github.com/williechen/copilot_use/blob/main/docs/LOCAL_START.md) · [操作索引](https://github.com/williechen/copilot_use/blob/main/docs/DEPLOYMENT.md)
- [驗收方式](https://github.com/williechen/copilot_use/blob/main/docs/LOCAL_VALIDATION.md)

新教學文章存於 `docs/teaching/daily/day-NN/{chat,pet,analytics}.md`。每新增一個專案 Day，獨立 commit、推送並全文讀回，再條件同步進度與 manifest。下一 Day 以最新進度檔為準。

教學的主體程式以 `williechen/rust_ai_ring` 為唯讀來源；不因文章發布自動修改參考庫或宣稱產品驗收完成。必要 Gate 2 驗證完成後，正文原樣移至 [`docs/teaching/archived/day-NN/{project}.md`](docs/teaching/archived/README.md)，保留原 Git 交付證據。

<details>
<summary>歷史文章與原始封存證據</summary>

[統一歷史目錄](https://github.com/williechen/copilot_use/blob/main/docs/teaching/history/README.md) · [原始路徑對照](https://github.com/williechen/copilot_use/blob/main/docs/teaching/history/PATH_MAP.json) · [舊庫 Release 歷史（非主體程式交付）](https://github.com/williechen/copilot_use/releases)

同一 Day 的文章、修訂與原始封存證據集中保存；跨 Day 報告放在共用歷史證據區。歷史內容保留其來源版本與驗證範圍，現行待辦由目前進度頁追蹤。

</details>
