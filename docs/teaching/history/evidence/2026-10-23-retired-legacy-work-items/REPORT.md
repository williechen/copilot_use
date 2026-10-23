# 舊工作項目廢除紀錄

2026-10-23 依使用者直接確認，以下兩項已廢除，不代表已實作或驗收完成：

- [ ] 現有測試歸位重構：自Day15起分批搬移11個非空測試檔，Chat兩檔歸chat-application、Server一檔歸server、Pet五檔歸pet-domain、Analytics三檔歸stats-server。整合步驟及文章位置見[教學總文章](COMMON_LEARNING_RULES.md)的Refactor；參考庫由使用者套用，尚未搬移或執行搬移後驗證。
- [ ] 依 rust_ai_chat 最新目錄、型別、Port、介面與所屬測試更新教學對應及理由；不在 copilot_use 建立平行程式。

已從 FUTURE_WORK 活動清單移除，並於共用規則明確排除其現行效力。第二項在原清單末句為「不在 copilot_use 建立平行程式」，使用者本次引用為「在 copilot_use 建立平行程式」；整個舊工作項目均撤除，不因此另行授權建立平行程式。

原文快照來源：`6e1a6960bb98a744fe3cedfcdb48d576035e8620`，保留 [FUTURE_WORK](before/docs/FUTURE_WORK.md) 與 [共用規則](before/docs/COMMON_LEARNING_RULES.md)。快照中的相對連結請依原 commit 解析。

本次僅更新文件；不搬移測試、不修改 rust_ai_chat、不更動 next_day、last_full_comparison、manifest 或排程設定。
