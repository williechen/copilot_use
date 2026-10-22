## 目前進度

### 最新教學交付狀態（GitHub main）

現行教學正本為 `williechen/copilot_use/main`。本次核對 `docs/learning-state.json`、`docs/teaching/current/manifest.json` 與 Day27 文章；**已發布的教學不等於學習庫已套用、測試已跑或產品驗收成功**。

| 專案 | 最新已發布教學 | 下個 Day（本次讀回） | 文章提交 |
| --- | --- | --- | --- |
| Chat | [Day27](../daily/day-27/chat.md) | 28 | `c8d63b6764d9eae12320bebe01c58cbf2556f832` |
| Pet | [Day27](../daily/day-27/pet.md) | 28 | `9c29a657a8e93e34064b2d838ddfefeba9afe4e7` |
| Analytics | [Day27](../daily/day-27/analytics.md) | 28 | `adf829e0efe092a72538d0abef3b5402e3289097` |

之後仍須每次讀取 [即時學習狀態](../../learning-state.json)，不把 28 寫死；GitHub 規則最新架構要求為 Leptos UI → Axum HTTP/API／Tauri IPC → Application／Domain → SQLx Adapter。Chat／Pet Day25 FIFO 更正尚待完整驗證，沒有因同步索引而結案。Day27 三篇全文與 blob／SHA256 已保存在 [manifest](manifest.json)，原始文章和歷史資料均不更動。

以下為較早產品／功能驗證快照（2026-10-22），**非最新 Day27 原始碼測試結果**。當時核驗來源：rust_ai_chat/main `aaa9194a966aefae1ef4724f5d2b576b4289c550`。以下為已確認範圍，並非即時 CI 或完整產品驗收。

| 專案 | 已實作 | 待完成／補教 |
| --- | --- | --- |
| Chat | 註冊、帳密登入、SQLite 持久 Session、房間頁 Session Gate | 共用登入／登出／Cookie／Session；API／WebSocket 房間 ACL |
| Pet | SQLite 建立／載入、逐寵物生活窗、pet-* 事件權限、單一 PetStatus 元件 | 動畫每幀單一寵物圖像；三個不同 pet id 原生生活窗的狀態、事件、裁切與重啟驗收 |
| Analytics | 管理員初始化、帳密登入、持久 Session、首次與後續修改密碼／Email | 共用登入、角色／來源授權、服務配對換證與完整後台互動 |

- Analytics Day24 補教：已結案，無須重發。
- Chat／Pet Day25 補教：各自 pending，依各專案 FIFO 處理已列缺口；已實作內容不重教。
- 下一教學日只依即時 learning-state 判定；此處產品狀態快照不作 Day 推進依據。Day25（含）以前正文與程式封存，教學發布、程式套用及功能驗收分開記錄。
- Analytics 本次歷史學習庫採刪除重建政策，不要求保留資料原地升級；本次未操作資料庫。

### 驗證範圍

以下僅為前述較早來源 SHA 的歷史測試結果，不代表 Day27 Green 已套用：預設 stats-server 測試：20 通過、0 失敗、1 ignored；隔離 PostgreSQL 18.6 配置 DATABASE_URL 並加 `--include-ignored`：31 通過、0 失敗、0 ignored。CI 的 PG workflow 未加此旗標，不能視為覆蓋全部 31 項。

Chat 真瀏覽器完整流程與 API／WS 授權、Pet 三窗 GUI、Analytics 完整角色／來源授權仍須驗收；本次文章整理未重跑程式測試。copilot_use 產品整合與驗收另列。

[驗證證據](../history/evidence/2026-10-22-main-aaa9194/REPORT.md)

### 排程與通知

- 三專案每日 TDD 教學：每日 06:00，Asia/Taipei，啟用。
- 歷史提示詞的模型名稱不作實際引擎選用證據；此處未重新驗證排程模型設定。
- 帳戶「任務」通知：推播與電子郵件均已開啟；實際送達未測試。

### 教學交付

GitHub `williechen/copilot_use/main` 為唯一規則與教學文章正本；[每日教學入口](../daily/README.md)。每新增一個專案 Day 獨立 commit、推送及全文讀回，之後才條件同步狀態與 manifest。共同主頁與三案 Notion 教學原文搬遷至 [GitHub 匯出索引](../history/evidence/notion-export/2026-10-22/README.md)，核對後移至可還原垃圾桶。本次只同步目前教學入口與狀態摘要，不新增 Day、不修改補教佇列或完整比對時間。
