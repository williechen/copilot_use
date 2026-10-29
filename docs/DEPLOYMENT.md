# 主體程式操作與驗收索引

現行程式來源與目錄：[rust_ai_ring 四個 workspace 對照](SOURCE_LAYOUT.md)（來源 `0b3e5844`，2026-10-24 靜態核對）。

`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_ring` 是唯一程式、測試、CI 與功能驗收主體。 使用者已取消公開部署，本次不新增部署或安裝要求。

| 工作 | 現行來源 |
| --- | --- |
| 來源／架構 | [系統架構](SYSTEM_ARCHITECTURE.md)、[rust_ai_ring/main](https://github.com/williechen/rust_ai_ring) |
| 啟動與依賴 | [本地入口](LOCAL_START.md)、主體 README／Cargo／app 配置 |
| 測試與功能驗收 | [驗收範圍](LOCAL_VALIDATION.md) |
| 週六核對 | [週六流程與設定限制](SATURDAY_OPERATIONS.md) |
| DB／migration | [資料邊界](SQLITE_TRANSITION.md)、主體的 Adapter 與 migration |
| 舊庫發行／更新 | [歷史適用範圍](GITHUB_PET_UPDATES.md)，不作主體交付 |

原本操作索引[原文保留](teaching/history/evidence/2026-10-23-repository-roles/before/docs/DEPLOYMENT.md)。現行流程設定尚未變更的部分見 [分工同步範圍](REPOSITORY_ROLE_TRANSITION.md)。
