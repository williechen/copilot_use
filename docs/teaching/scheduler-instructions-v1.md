# 三專案每日 TDD 教學排程

網頁任務：**三專案每日 TDD 教學**。2026-10-26已儲存、重新載入並全文比對一致；原排程與模型保留。

```text
每日依Chat→Pet→Analytics交付TDD文章。先讀copilot_use/main的共用規則、需求、架構、learning-state、manifest、入口與上一篇，核對兩庫精確SHA及當日官方穩定版本，依next_day選功能。

每篇保留目標、完整Red／Green、版本與驗證，另設與Red／Green並列的「Refactor」獨立段落，只處理該篇所屬專案：Chat只寫Chat、Pet只寫Pet、Analytics只寫Analytics。依各案累積進度逐步整理自身架構、測試、fixture與專案CI，不限定當日功能切片，也不作Green後續的小整理。交代完整變更／patch、有效斷言承接、實際驗證與下一步；未完成如實記錄。

全專案重構另列獨立項目，集中於docs/engineering/README.md：跨案契約、共用crates、跨workspace依賴與全庫CI。不得把全庫workflow或其他專案內容塞入各案Refactor，也不得重複計功。

每案獨立提交、精確commit全文讀回並存blob／SHA256後，條件同步進度、manifest、七首頁與入口。只寫copilot_use/main；rust_ai_ring唯讀，完整程式／patch及測試方案由使用者套用。核心60分鐘只計Red／Green，兩類Refactor及其餘另計。重構不增加Day；方案、套用、驗收與未執行分列。保留原文、斷言與FIFO，不建立第二套產品，不更新last_full_comparison；一項受阻仍做其餘可獨立項。
```

[全文讀回](current/day38-r4-schedule.json) · [各案Refactor](functional-refactoring/README.md) · [全專案獨立項目](../engineering/README.md) · [共用規則](../COMMON_LEARNING_RULES.md)
