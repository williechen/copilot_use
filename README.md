# copilot_use

Chat、Desktop Pet 與 Analytics 的規則、教學與交付證據庫。`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_chat` 是唯一程式、測試、CI 與功能驗收主體。

## 從這裡開始

| 入口 | 內容 |
| --- | --- |
| [共用文章與規則](https://github.com/williechen/copilot_use/blob/main/docs/COMMON_LEARNING_RULES.md) | 共用學習方式、文章格式、交付與驗證規則 |
| [Chat 首頁](https://github.com/williechen/copilot_use/blob/main/docs/teaching/chat/README.md) | 聊天系統目標、操作主線、目前教學與待辦入口 |
| [Pet 首頁](https://github.com/williechen/copilot_use/blob/main/docs/teaching/pet/README.md) | 桌面寵物目標、操作主線、目前教學與待辦入口 |
| [Analytics 首頁](https://github.com/williechen/copilot_use/blob/main/docs/teaching/analytics/README.md) | 統計後台目標、操作主線、目前教學與待辦入口 |

[目前進度與待完成項目](https://github.com/williechen/copilot_use/blob/main/docs/teaching/current/STATUS.md) · [最新學習進度](https://github.com/williechen/copilot_use/blob/main/docs/learning-state.json) · [文件總目錄](https://github.com/williechen/copilot_use/blob/main/docs/README.md)

## 最新教學（Day30）

| 專案 | 最新文章 | 下一篇 |
| --- | --- | --- |
| Chat | [Day30 r2](docs/teaching/daily/day-30/chat.md) | Day31 |
| Pet | [Day30](docs/teaching/daily/day-30/pet.md) | Day31 |
| Analytics | [Day30](docs/teaching/daily/day-30/analytics.md) | Day31 |

以 [即時學習進度](docs/learning-state.json) 為準；上述為教學交付，不代表主體程式已套用或驗收。

## 主體程式

程式來源為 [williechen/rust_ai_chat/main](https://github.com/williechen/rust_ai_chat)。[架構與程式入口](docs/SYSTEM_ARCHITECTURE.md)、[測試與驗收範圍](docs/LOCAL_VALIDATION.md) 依該庫來源 SHA 核對；本庫既有 apps／crates、工具與 Release 不作主體程式驗收。

## 開發與教學

規則、現行共用文章、三專案首頁、next_day／manifest、交付讀回與歷史資料統一維護於本庫 `main`；程式與程式 CI 以 `rust_ai_chat/main` 為主。從儲存庫首頁與文件目錄都能開啟同一份現行內容。

- [產品目標與工作清單](https://github.com/williechen/copilot_use/blob/main/docs/FUTURE_WORK.md)
- [系統架構](https://github.com/williechen/copilot_use/blob/main/docs/SYSTEM_ARCHITECTURE.md)
- [本地啟動](https://github.com/williechen/copilot_use/blob/main/docs/LOCAL_START.md) · [操作索引](https://github.com/williechen/copilot_use/blob/main/docs/DEPLOYMENT.md)
- [驗收方式](https://github.com/williechen/copilot_use/blob/main/docs/LOCAL_VALIDATION.md)

新教學文章存於 `docs/teaching/daily/day-NN/{chat,pet,analytics}.md`。每新增一個專案 Day，獨立 commit、推送並全文讀回，再條件同步進度與 manifest。下一 Day 以最新進度檔為準。

教學的主體程式以 `williechen/rust_ai_chat` 為唯讀來源；不因文章發布自動修改參考庫或宣稱產品驗收完成。必要 Gate 2 驗證完成後，正文原樣移至 [`docs/teaching/archived/day-NN/{project}.md`](docs/teaching/archived/README.md)，保留原 Git 交付證據。

<details>
<summary>歷史文章與原始封存證據</summary>

[統一歷史目錄](https://github.com/williechen/copilot_use/blob/main/docs/teaching/history/README.md) · [原始路徑對照](https://github.com/williechen/copilot_use/blob/main/docs/teaching/history/PATH_MAP.json) · [舊庫 Release 歷史（非主體程式交付）](https://github.com/williechen/copilot_use/releases)

同一 Day 的文章、修訂與原始封存證據集中保存；跨 Day 報告放在共用歷史證據區。歷史內容保留其來源版本與驗證範圍，現行待辦由目前進度頁追蹤。

</details>
