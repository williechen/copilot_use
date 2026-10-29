# 教學與重構分工

功能教學依 Chat→Pet→Analytics 交付，每案以正式 UI／HTTP／IPC→用例→Adapter 的最小完整 Red／Green 切片推進。每篇在目標後設獨立「Refactor」，只處理該專案累積的架構、測試、fixture 與專案 CI，完整列出程式、caller、模組、相依、斷言去向及驗證；不能只交摘要或 patch 連結。

[各案重構](teaching/functional-refactoring/PROGRESS.md) 與 [跨案工程](engineering/README.md) 分開；共用契約、跨 workspace 依賴與全庫 CI 不塞入三案 Refactor，也不重複計功。

核心 60 分鐘只計 Red／Green；Refactor、版本查核、驗證與交付另計。文件 Gate1 與產品 Gate2 分列；產品唯讀，由使用者套用。未執行、失敗、部分通過與完整驗收分開；不刪有效斷言、不增加 Day、不重開 FIFO。

[完整共用規則](COMMON_LEARNING_RULES.md) · [目前進度](teaching/current/STATUS.md)
