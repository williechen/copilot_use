## 目前進度

### 最新教學交付狀態（GitHub main）

教學文件正本：`williechen/copilot_use/main`。已按 `learning-state.json`、`manifest.json` 與 Day29 文章讀回核對。**文章交付不代表已套用主體程式、測試通過或產品功能驗收完成。**

| 專案 | 最新已發布教學 | 下一 Day | 文章 Commit |
| --- | --- | --- | --- |
| Chat | [Day29](../daily/day-29/chat.md) | **30** | `d8ef21fd2f0eca49df445244a805f22dd64eb5e4` |
| Pet | [Day29](../daily/day-29/pet.md) | **30** | `ef58906758a744d98b46c6c62c1e886343717ebd` |
| Analytics | [Day29](../daily/day-29/analytics.md) | **30** | `ae5001c214bd5c571f3d2d150ba1e097ce984bbb` |

本表是 Day29 文件交付快照；往後實際下一 Day 仍以 [即時學習狀態](../../learning-state.json) 為準。Day29 三篇已全文讀回、manifest 與每日索引已同步；Chat／Pet Day25 FIFO 更正仍 pending，不能因教學文件發布而結案。文章 Commit／Blob 只證明 GitHub 文件，不代表產品驗收。

### 程式來源與驗證邊界

唯一主體為 rust_ai_chat/main。Day29 教學核對使用 `143d6b3f5180b669860b1300286b89464c669434`，只靜態核對入口，未對此 SHA 執行新測試或功能驗收；較早 SHA 的測試不可當作最新 main 結果。

- 程式、測試、CI／job、UI／API／IPC／DB、功能及重啟驗收只認 rust_ai_chat 的精確來源。
- copilot_use 的 commit／blob／SHA256 只證明規則、文章、next_day／manifest 與全文讀回；舊程式、Release、CI 與產品試驗不作主體驗收。
- learning-state／manifest 內舊 validated_program_sha、daily_product_sha、fixed_tag、acceptance 與試驗紀錄完整保留原用途，不能用來挑選本次主體 checkout 或推進 Day。
- Analytics Day24 補教既有結案證據保留；Chat／Pet Day25 FIFO 仍依狀態與匹配證據逐項判定，不因本次規則整理結案。

原產品狀態、較早測試數字及證據連結[完整保留](../history/evidence/2026-10-23-repository-roles/before/docs/teaching/current/STATUS.md)。本次不改 learning-state、manifest、補教佇列或 last_full_comparison。現行目標見 [FUTURE_WORK](../../FUTURE_WORK.md)；流程設定的剩餘差異見 [同步範圍](../../REPOSITORY_ROLE_TRANSITION.md)。

### 排程與通知

- 既有每日教學紀錄為每日 06:00，Asia/Taipei、啟用；本次未重新讀回雲端任務狀態，亦未變更任務。
- 歷史提示詞的模型名稱不作實際引擎選用證據；此處未重新驗證排程模型設定。
- 歷史通知紀錄為推播與電子郵件已開啟；本次未重新核驗設定或實際送達。

### 教學交付

GitHub `williechen/copilot_use/main` 為唯一規則與教學文章正本；[每日教學入口](../daily/README.md)。每新增一個專案 Day 獨立 commit、推送及全文讀回，之後才條件同步狀態與 manifest。共同主頁與三案 Notion 教學原文搬遷至 [GitHub 匯出索引](../history/evidence/notion-export/2026-10-22/README.md)，核對後移至可還原垃圾桶。本次同步 Day29 首頁與狀態摘要，不新增 Day、不修改補教佇列或完整比對時間。
