## 目前進度

### 最新教學交付狀態（GitHub main）

教學文件正本：`williechen/copilot_use/main`。已按 `learning-state.json`、`manifest.json` 與 Day30 文章讀回核對。**文章交付不代表已套用主體程式、測試通過或產品功能驗收完成。**

| 專案 | 最新已發布教學 | 下一 Day | 文章 Commit |
| --- | --- | --- | --- |
| Chat | [Day30 r3](../daily/day-30/chat.md) | **31** | `0548392623eed068ce9b29cd4c188cd3bb8668ba` |
| Pet | [Day30 r2](../daily/day-30/pet.md) | **31** | `ed0fcdd51341fd9866cb78e08573463be7e06d71` |
| Analytics | [Day30 r2](../daily/day-30/analytics.md) | **31** | `d6175ee83aad7cd3a32433826ccb20b01f0211de` |

本表是 Day30 已交付快照；往後實際下一 Day 仍以 [即時學習狀態](../../learning-state.json) 為準。Day30 三篇已全文讀回，manifest 已同步；Chat／Pet Day25 FIFO 更正仍 pending，不能因教學文件發布而結案。文章 Commit／Blob 只證明 GitHub 文件，不代表產品驗收。

### 程式來源與驗證邊界

唯一主體為 rust_ai_chat/main。本輪驗證 `9c2b7cf5a4c60a08254445911916c4ed9b02d7b3`、文件基準 `fad5db13a6711ac45a9cfece8198b612d7ce4607`；[全部84篇逐篇結果](../history/evidence/2026-10-23-all-articles-9c2b7cf/ALL_ARTICLES.md)與[命令、真入口、缺陷、CI及勘誤](../history/evidence/2026-10-23-all-articles-9c2b7cf/REPORT.md)已保存。不是三案全通過。

Chat後端25通過、UI SSR9通過／1失敗；Pet原生49與前端9通過、Node0通過／3失敗；Analytics隔離PG完整35通過。另核心套件32通過、WASM檢查與fmt通過；Chat嚴格Clippy失敗。workspace整體命令因磁碟空間不足未完成，各套件已分開補驗，不能稱單一workspace命令成功。

真HTTP/WS與DB驗證支持Chat Day26/27/29、Analytics Day26/27/29共6篇完成切片封存；原文按blob／SHA256保留。[封存清單](../history/evidence/2026-10-23-all-articles-9c2b7cf/archive-manifest.json)。原文舊命令更正獨立記錄，未將原樣錯誤命令當成功。其他活動篇保留：Day28原文SSR API須勘誤、Pet控制窗恢復／動態窗漫遊／故障及三窗GUI未驗收；Analytics舊類型summary正式路由404、帳密入口缺舊限流政策。現存GitHub索引Day2–29為84篇；Day1正文未找到，另列來源缺口。
- 程式、測試、CI／job、UI／API／IPC／DB、功能及重啟驗收只認 rust_ai_chat 的精確來源。
- copilot_use 的 commit／blob／SHA256 只證明規則、文章、next_day／manifest 與全文讀回；舊程式、Release、CI 與產品試驗不作主體驗收。
- learning-state／manifest 內舊 validated_program_sha、daily_product_sha、fixed_tag、acceptance 與試驗紀錄完整保留原用途，不能用來挑選本次主體 checkout 或推進 Day。
- Analytics Day24 補教既有結案證據保留；Chat／Pet Day25 FIFO 仍依狀態與匹配證據逐項判定，不因本次規則整理結案。

原產品狀態、較早測試數字及證據連結[完整保留](../history/evidence/2026-10-23-repository-roles/before/docs/teaching/current/STATUS.md)。本次只在 learning-state／manifest 增加最新驗證與完成封存資料；next_day、補教佇列與 last_full_comparison 不變。現行目標見 [FUTURE_WORK](../../FUTURE_WORK.md)；流程設定的剩餘差異見 [同步範圍](../../REPOSITORY_ROLE_TRANSITION.md)。

### Chat瀏覽器／SQLite後續驗收（2026-10-23）

程式仍為`9c2b7cf5a4c60a08254445911916c4ed9b02d7b3`；[本輪報告與畫面](../history/evidence/2026-10-23-browser-sqlite-9c2b7cf/REPORT.md)。Chat Day17／19／20已用真正hydrated瀏覽器驗證中文及特殊字元搜尋、真DB故障保留rows、恢復重試與刷新清query，改為本篇切片通過；3篇沿既有封存原文記錄完成，累計9篇完成切片（本次沒有新搬移）。Day15列表／選房通過，但保留建房功能的UI入口缺失，只部分通過，不列完成。Day18目前schema正常，但完整fa歷史庫先遇rooms migration checksum mismatch，單獨22554歷史也拒絕升級且資料保留；任意actor可寫、真操作後稽核0筆，改列實測失敗，不能標完成。其他前輪測試／失敗與未驗收範圍保留；[更新後84篇矩陣](../history/evidence/2026-10-23-browser-sqlite-9c2b7cf/ALL_ARTICLES.md)。

### 排程與通知

- 既有每日教學紀錄為每日 06:00，Asia/Taipei、啟用；本次未重新讀回雲端任務狀態，亦未變更任務。
- 歷史提示詞的模型名稱不作實際引擎選用證據；此處未重新驗證排程模型設定。
- 歷史通知紀錄為推播與電子郵件已開啟；本次未重新核驗設定或實際送達。

### 教學交付

GitHub `williechen/copilot_use/main` 為唯一規則與教學文章正本；[每日教學入口](../daily/README.md)。每新增一個專案 Day 獨立 commit、推送及全文讀回，之後才條件同步狀態與 manifest。`williechen/rust_ai_chat/main` 是程式、測試、CI 與功能驗收的唯一來源。本次文件整理不新增 Day、不修改補教佇列或完整比對時間。
