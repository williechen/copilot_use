## 目前進度

### 最新教學交付狀態（GitHub main）

現行教學正本為 `williechen/copilot_use/main`。本次核對 `docs/learning-state.json`、`docs/teaching/current/manifest.json` 與 Day27 文章；**已發布的教學不等於學習庫已套用、測試已跑或產品驗收成功**。

| 專案 | 最新已發布教學 | 下個 Day（本次讀回） | 文章提交 |
| --- | --- | --- | --- |
| Chat | [Day27](../daily/day-27/chat.md) | 28 | `c8d63b6764d9eae12320bebe01c58cbf2556f832` |
| Pet | [Day27](../daily/day-27/pet.md) | 28 | `9c29a657a8e93e34064b2d838ddfefeba9afe4e7` |
| Analytics | [Day27](../daily/day-27/analytics.md) | 28 | `adf829e0efe092a72538d0abef3b5402e3289097` |

之後仍須每次讀取 [即時學習狀態](../../learning-state.json)，不把 28 寫死；GitHub 規則最新架構要求為 Leptos UI → Axum HTTP/API／Tauri IPC → Application／Domain → SQLx Adapter。Chat／Pet Day25 FIFO 更正尚待完整驗證，沒有因同步索引而結案。Day27 三篇全文與 blob／SHA256 已保存在 [manifest](manifest.json)，原始文章和歷史資料均不更動。

### 程式來源與驗證邊界

唯一主體為 rust_ai_chat/main。本次文件同步讀取到 `f61ea1c59667f3c92c78e576e3c48038a8ce030a`，只靜態核對入口，未對此 SHA 執行新測試或功能驗收；較早 SHA 的測試不可當作最新 main 結果。

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

GitHub `williechen/copilot_use/main` 為唯一規則與教學文章正本；[每日教學入口](../daily/README.md)。每新增一個專案 Day 獨立 commit、推送及全文讀回，之後才條件同步狀態與 manifest。共同主頁與三案 Notion 教學原文搬遷至 [GitHub 匯出索引](../history/evidence/notion-export/2026-10-22/README.md)，核對後移至可還原垃圾桶。本次只同步目前教學入口與狀態摘要，不新增 Day、不修改補教佇列或完整比對時間。
