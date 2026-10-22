# copilot_use

以 Rust 開發的 Chat、Desktop Pet 與 Analytics 三專案工作區。

## 從這裡開始

| 入口 | 內容 |
| --- | --- |
| [共用文章與規則](https://github.com/williechen/copilot_use/blob/main/docs/COMMON_LEARNING_RULES.md) | 共用學習方式、文章格式、交付與驗證規則 |
| [Chat 首頁](https://github.com/williechen/copilot_use/blob/main/docs/teaching/chat/README.md) | 聊天系統目標、操作主線、目前教學與待辦入口 |
| [Pet 首頁](https://github.com/williechen/copilot_use/blob/main/docs/teaching/pet/README.md) | 桌面寵物目標、操作主線、目前教學與待辦入口 |
| [Analytics 首頁](https://github.com/williechen/copilot_use/blob/main/docs/teaching/analytics/README.md) | 統計後台目標、操作主線、目前教學與待辦入口 |

[目前進度與待完成項目](https://github.com/williechen/copilot_use/blob/main/docs/teaching/current/STATUS.md) · [最新學習進度](https://github.com/williechen/copilot_use/blob/main/docs/learning-state.json) · [文件總目錄](https://github.com/williechen/copilot_use/blob/main/docs/README.md)

## 三專案

| 專案 | 目標 | 主要程式位置 | 資料庫 |
| --- | --- | --- | --- |
| Chat | 多人、多房間聊天及使用者與房間管理 | `apps/server`、`crates/ui` | SQLite |
| Desktop Pet | 可互動、保存個別狀態的桌面寵物 | `apps/desktop_pet`、`crates/pet_domain`、`crates/pet_runtime` | 獨立 SQLite |
| Analytics | 事件統計、查詢與管理後台 | `apps/stats_server` | PostgreSQL |

各項實作與驗收結果見目前進度及對應來源證據。產品目標、教學發布、程式套用與功能驗收分別記錄。

## 開發與教學

產品程式、現行共用文章、三專案首頁、教學與歷史資料統一維護於 `main`。從儲存庫首頁與文件目錄都能開啟同一份現行內容。

- [產品目標與工作清單](https://github.com/williechen/copilot_use/blob/main/docs/FUTURE_WORK.md)
- [系統架構](https://github.com/williechen/copilot_use/blob/main/docs/SYSTEM_ARCHITECTURE.md)
- [本地啟動](https://github.com/williechen/copilot_use/blob/main/docs/LOCAL_START.md) · [操作索引](https://github.com/williechen/copilot_use/blob/main/docs/DEPLOYMENT.md)
- [驗收方式](https://github.com/williechen/copilot_use/blob/main/docs/LOCAL_VALIDATION.md)

新教學文章存於 `docs/teaching/daily/day-NN/{chat,pet,analytics}.md`。每新增一個專案 Day，獨立 commit、推送並全文讀回，再條件同步進度與 manifest。下一 Day 以最新進度檔為準。

教學與參考程式碼以 `williechen/rust_ai_chat` 為唯讀來源；不因文章發布自動修改參考庫或宣稱產品驗收完成。

<details>
<summary>歷史文章與原始封存證據</summary>

[統一歷史目錄](https://github.com/williechen/copilot_use/blob/main/docs/teaching/history/README.md) · [原始路徑對照](https://github.com/williechen/copilot_use/blob/main/docs/teaching/history/PATH_MAP.json) · [GitHub Releases](https://github.com/williechen/copilot_use/releases)

同一 Day 的文章、修訂與原始封存證據集中保存；跨 Day 報告放在共用歷史證據區。歷史內容保留其來源版本與驗證範圍，現行待辦由目前進度頁追蹤。

</details>
