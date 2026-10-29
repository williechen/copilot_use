# 過時排程文章修正

2026-10-23 使用者指出現行排程文章已過時。原文完整快照保留於 [scheduler-instructions-v1.before.md](scheduler-instructions-v1.before.md)，來源 commit `d575053e1d1770bdfacbb000c7bd386d659be9ae`。

現行入口改為讀取最新共用規則、進度、manifest、需求與功能證據；移除歷史提示詞作現行規格的效力。共用規則與週六操作不再斷言實際任務已啟用或沿用舊時間。狀態檔與 manifest 新增 schedule_document_policy，既有欄位原值保留為歷史觀察。

本機 automation.toml 清單未找到對應紀錄；未從實際雲端排程服務讀回任務，不能宣稱其存在、停用、時間或提示詞已更新。本次未修改任務、workflow、腳本或程式。每週至少一次完整比對、僅真正完整成功才更新 last_full_comparison 的要求保留；next_day、last_full_comparison、補教與驗收結果未更動。
