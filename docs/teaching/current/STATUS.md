## 目前進度

資料核驗日：2026-10-22。最近已核驗來源：rust_ai_chat/main `aaa9194a966aefae1ef4724f5d2b576b4289c550`。以下為已確認範圍，並非即時 CI 或完整產品驗收。

| 專案 | 已實作 | 待完成／補教 |
| --- | --- | --- |
| Chat | 註冊、帳密登入、SQLite 持久 Session、房間頁 Session Gate | 共用登入／登出／Cookie／Session；API／WebSocket 房間 ACL |
| Pet | SQLite 建立／載入、逐寵物生活窗、pet-* 事件權限、單一 PetStatus 元件 | 動畫每幀單一寵物圖像；三個不同 pet id 原生生活窗的狀態、事件、裁切與重啟驗收 |
| Analytics | 管理員初始化、帳密登入、持久 Session、首次與後續修改密碼／Email | 共用登入、角色／來源授權、服務配對換證與完整後台互動 |

- Analytics Day24 補教：已結案，無須重發。
- Chat／Pet Day25 補教：各自 pending，依各專案 FIFO 處理已列缺口；已實作內容不重教。
- 三案 next_day 均為 27；Day25（含）以前正文與程式封存。教學發布、程式套用及功能驗收分開記錄。
- Analytics 本次歷史學習庫採刪除重建政策，不要求保留資料原地升級；本次未操作資料庫。

### 驗證範圍

上述來源的預設 stats-server 測試：20 通過、0 失敗、1 ignored；隔離 PostgreSQL 18.6 配置 DATABASE_URL 並加 `--include-ignored`：31 通過、0 失敗、0 ignored。CI 的 PG workflow 未加此旗標，不能視為覆蓋全部 31 項。

Chat 真瀏覽器完整流程與 API／WS 授權、Pet 三窗 GUI、Analytics 完整角色／來源授權仍須驗收；本次文章整理未重跑程式測試。copilot_use 產品整合與驗收另列。

[驗證證據](../history/evidence/2026-10-22-main-aaa9194/REPORT.md)

### 排程與通知

- 三專案每日 TDD 教學：每日 06:00，Asia/Taipei，啟用。
- 提示詞指定 `gpt-6.1-sol`、推理強度中；實際引擎選用尚未核實。
- 帳戶「任務」通知：推播與電子郵件均已開啟；實際送達未測試。

### 教學交付

GitHub 教學分支為唯一正本；[每日教學入口](../daily/README.md)。每新增一個專案 Day 獨立 commit、推送及全文讀回，之後才條件同步狀態與 manifest。共同主頁與三案 Notion 教學原文搬遷至 [GitHub 匯出索引](../history/evidence/notion-export/2026-10-22/README.md)，核對後移至可還原垃圾桶。本次未新增 Day，未改補教佇列或完整比對時間。
