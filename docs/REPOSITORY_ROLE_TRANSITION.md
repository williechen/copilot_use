# 儲存庫分工同步範圍

2026-10-23 使用者確認：`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_chat` 是唯一程式、測試、CI 與功能驗收主體。 正本規則見 [COMMON_LEARNING_RULES](COMMON_LEARNING_RULES.md)。

## 本次完成

首頁、共用規則、架構／銜接、目標／待辦、操作／驗收、教學現況及排程文件的現行角色已同步。程序來源本次讀取 rust_ai_chat/main `f61ea1c59667f3c92c78e576e3c48038a8ce030a`，只作靜態入口核對，未重新測試。改寫前原文與 blob／SHA256 [完整保留](teaching/history/evidence/2026-10-23-repository-roles/README.md)。

learning-state 與 manifest 原始 bytes、next_day、FIFO、last_full_comparison、所有 Day 正文與既有歷史證據保持原狀。舊程式與 Release 僅作歷史追溯；其相關工作已廢除。以上原始 bytes 不變的描述是先前分工整理的紀錄；目前狀態檔已另增現行政策，原歷史值與進度保留。

## 已廢除的程式工作與殘留執行設定

2026-10-23 依使用者確認，所有涉及 copilot_use 程式開發、移植、同步適配、重構、套用、建置、測試、CI、產品試驗、發布、安裝更新與功能驗收的工作要求均已廢除，不再列為待辦、選題或驗收門檻。copilot_use 僅維護需求、規則、教學文章、進度、索引與交付證據；文件內的教學程式碼用於說明 rust_ai_chat 的實作，不建立第二套產品。程式功能、測試、CI 與驗收只依 rust_ai_chat 精確 SHA。既有 copilot_use 程式、腳本、workflow、Release 及歷史欄位僅供追溯，不代表仍有維護或執行義務。

以下僅記錄尚未變更的設定檔，不是活動工作清單；本次未查核或停用實際執行中的排程。

| 設定 | 既有行為與本次界線 |
| --- | --- |
| .github/workflows/verify.yml | 仍測 copilot_use 程式；不作 rust_ai_chat CI 或主體驗收 |
| .github/workflows/weekly-sync.yml | 仍比較舊 product/reference 並呼叫本庫 Verify；不是已符合現行分工的週六執行證據 |
| .github/workflows/native-source-audit.yml | 保留舊固定 commit 原生 provenance；不當主體現況 |
| .github/workflows/release-approved.yml | 舊庫 Release 發布設定仍在；本次不觸發／發布／搬移資產 |
| scripts/run-weekly.sh、deploy/systemd/copilot-weekly.* | 舊範例的工作目錄與測試來源未改，本次不執行／安裝／停用 timer |
| scripts/github-pet-update.py、release-approved.py | 舊庫更新來源與簽章契約保留，不重定向主體安裝包 |
| 既有每日雲端任務 | 原 scheduler-instructions 文章已過時，僅保留歷史快照；新入口只指向最新規則與狀態，實際提示詞／模型／頻率／啟用狀態尚未讀回，未修改或觸發任務 |

既有排程不變的限制下，本次文件同步不宣稱上述執行設定已調整或 dry run 所有項目已結案。後續如調整流程，應保留任務身份／歷史證據，只驗證文件庫的文件一致性，主體程式測試／CI 歸 rust_ai_chat。

本次提交採 [skip ci] 避免純文件更新觸發舊庫程式測試；這只適用本次 push 的跳過標記，不是 workflow 或未來排程已停用的證據。
