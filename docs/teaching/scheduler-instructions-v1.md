# 三專案每日 TDD 教學排程

現有網頁版任務：**三專案每日 TDD 教學**。2026-10-26已將Refactor改為專案累積工程接續，儲存及重新載入後全文一致；原排程與模型保留。

```text
每日並行交付功能教學與專案重構，分開計時、記錄與驗收。

教學：依Chat→Pet→Analytics，各交付一個可操作功能的Red／完整Green與直接相關測試。先讀copilot_use/main的共用規則、需求、架構、learning-state、manifest、現行入口與上一篇；依next_day選題，核對兩庫精確SHA及當日官方穩定版本。每案獨立提交、全文讀回並保存blob／SHA256後，才條件同步進度、manifest、七首頁與入口。

Refactor以各專案整體架構、共用依賴、測試與CI為範圍，接續既有工程進度，每日按相依完成一個完整小工作包，不隨當日教學切片重新選題。各篇Refactor只列所屬專案的累積進度、本次工作包／完整patch連結、驗證與下一步；未推進如實記錄。當日切片的小整理併Green說明，不能抵算專案重構。跨專案工作包共用同一份證據，不重複計功。

只寫copilot_use/main文件與證據；rust_ai_ring唯讀，完整程式／patch及測試方案由使用者套用。重構不作教學主題、不增加Day；核心60分鐘只計Red／Green，其餘另計。方案交付、已套用、已驗收及未執行分列。保留原文與有效斷言，不建立第二套產品；FIFO依共用規則，不更新last_full_comparison。一項受阻仍完成其餘可獨立工作。
```

[全文核對紀錄](current/project-refactor-scope-2026-10-26-schedule.json) · [共用規則](../COMMON_LEARNING_RULES.md)
