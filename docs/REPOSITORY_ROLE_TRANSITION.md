# 儲存庫分工同步範圍

2026-10-23 使用者確認：`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_chat` 是唯一程式、測試、CI 與功能驗收主體。 正本規則見 [COMMON_LEARNING_RULES](COMMON_LEARNING_RULES.md)。

## 本次完成

首頁、共用規則、架構／銜接、目標／待辦、操作／驗收、教學現況及排程文件的現行角色已同步。程序來源本次讀取 rust_ai_chat/main `f61ea1c59667f3c92c78e576e3c48038a8ce030a`，只作靜態入口核對，未重新測試。改寫前原文與 blob／SHA256 [完整保留](teaching/history/evidence/2026-10-23-repository-roles/README.md)。

learning-state 與 manifest 原始 bytes、next_day、FIFO、last_full_comparison、所有 Day 正文與既有歷史證據保持原狀。舊程式與 Release 保留原適用範圍；不再作主體驗收。

## 尚未變更的執行設定

| 設定 | 既有行為與本次界線 |
| --- | --- |
| .github/workflows/verify.yml | 仍測 copilot_use 程式；不作 rust_ai_chat CI 或主體驗收 |
| .github/workflows/weekly-sync.yml | 仍比較舊 product/reference 並呼叫本庫 Verify；不是已符合現行分工的週六執行證據 |
| .github/workflows/native-source-audit.yml | 保留舊固定 commit 原生 provenance；不當主體現況 |
| .github/workflows/release-approved.yml | 舊庫 Release 發布設定仍在；本次不觸發／發布／搬移資產 |
| scripts/run-weekly.sh、deploy/systemd/copilot-weekly.* | 舊範例的工作目錄與測試來源未改，本次不執行／安裝／停用 timer |
| scripts/github-pet-update.py、release-approved.py | 舊庫更新來源與簽章契約保留，不重定向主體安裝包 |
| 既有每日雲端任務 | 本次只更新 scheduler-instructions 文件，未修改、觸發或建立任務，也未讀回即時提示詞／模型／頻率／狀態 |

既有排程不變的限制下，本次文件同步不宣稱上述執行設定已調整或 dry run 所有項目已結案。後續如調整流程，應保留任務身份／歷史證據，只驗證文件庫的文件一致性，主體程式測試／CI 歸 rust_ai_chat。

本次提交採 [skip ci] 避免純文件更新觸發舊庫程式測試；這只適用本次 push 的跳過標記，不是 workflow 或未來排程已停用的證據。
