# 三專案每日 TDD 教學排程

網頁任務：**三專案每日 TDD 教學**。2026-10-26已儲存、重新載入並全文比對一致；原排程與模型保留。

```text
每日依Chat→Pet→Analytics交付TDD文章。先讀copilot_use/main的共用規則、需求、架構、learning-state、manifest、入口與上一篇，核對兩庫精確SHA及當日官方穩定版本，依next_day選功能。

每篇保留目標、完整Red／Green、版本與驗證，另設與Red／Green並列的「Refactor」獨立段落，只處理該篇所屬專案：Chat只寫Chat、Pet只寫Pet、Analytics只寫Analytics。依各案累積進度逐步整理自身架構、測試、fixture與專案CI，不限定當日功能切片，也不作Green後續的小整理。交代完整變更／patch、有效斷言承接、實際驗證與下一步；未完成如實記錄。

全專案重構另列獨立項目，集中於docs/engineering/README.md：跨案契約、共用crates、跨workspace依賴與全庫CI。不得把全庫workflow或其他專案內容塞入各案Refactor，也不得重複計功。

每案獨立提交、精確commit全文讀回並存blob／SHA256後，條件同步進度、manifest、七首頁與入口。只寫copilot_use/main；rust_ai_ring唯讀，完整程式／patch及測試方案由使用者套用。核心60分鐘只計Red／Green，兩類Refactor及其餘另計。重構不增加Day；方案、套用、驗收與未執行分列。保留原文、斷言與FIFO，不建立第二套產品，不更新last_full_comparison；一項受阻仍做其餘可獨立項。
```

[全文讀回](current/day38-r4-schedule.json) · [各案Refactor](functional-refactoring/README.md) · [全專案獨立項目](../engineering/README.md) · [共用規則](../COMMON_LEARNING_RULES.md)

## 2026-10-29 執行判定補充

先核對當次工具能力，再分列 Gate1 文件缺口與 Gate2 產品未執行項。雲端沒有本地 checkout／Cargo 不能單獨作為停止教學或要求使用者提供本地資訊的理由；仍須完成可用 GitHub 來源、版本、完整 Red／Green／Refactor、逐案提交及精確讀回。真程式／patch／文件缺陷須版本化修訂，不以靜態比對假稱編譯通過。單案受阻繼續其他獨立案。

Gate1 條件齊備後依 expected next_day 與既有證據條件同步；Gate2 未跑如實列未執行，不重複加 Day 或誤阻文件發布。每次回報實際 commit、目前閱讀版本、next_day 和具體缺口。

本節是執行文件修訂，未宣稱網頁排程設定已修改；時間／模型／實際任務狀態須另讀即時設定。[本次原因與歷程](../source-review/local-validation-2026-10-29-day43/diagnosis.md)。

## 2026-10-29 網頁版已儲存之提示詞

既有雲端任務 `6acd967c15fc8191ade8407954c5b354` 已透過網頁修改提示詞，刷新後全文比對一致；仍啟用，每日時間／頻率未改，未建立新任務。[精確提示詞與SHA256](../source-review/acceptance-2026-10-29/schedule-readback.json)。

```text
每日依Chat→Pet→Analytics交付TDD文章。先讀copilot_use/main的共用規則、需求、架構、learning-state、manifest、入口與上一篇，核對兩庫精確SHA及當日官方穩定版本，依目前next_day選功能，不固定Day或重複發布。

每篇保留目標、完整Red／Green、版本與驗證，另設與Red／Green並列的「Refactor」獨立段落，只處理該篇所屬專案：Chat只寫Chat、Pet只寫Pet、Analytics只寫Analytics。依各案累積進度逐步整理自身架構、測試、fixture與專案CI，不限定當日功能切片，也不作Green後續的小整理。交代逐檔完整變更／patch、有效斷言承接、實際驗證與下一步；未完成如實記錄。

全專案重構獨立集中於docs/engineering/README.md：跨案契約、共用crates、跨workspace依賴與全庫CI。不得混入各案Refactor或重複計功。

開工先確認當次可用GitHub／網頁與執行工具。Gate1文件交付與Gate2產品驗收分開：沒有本地checkout、Rust/Cargo、DB、GUI或CI權限時，繼續用可用工具完成GitHub來源、官方穩定版本與相容限制、完整Red／Green／Refactor、patch接線及讀回，不要求使用者提供本地資訊，不單憑Gate2未跑阻止合格文件發布。真正程式／patch／文件缺陷仍須版本化修訂；無法驗證項明列未執行，不能把靜態hunk或舊SHA結果當實測。

每案獨立commit到copilot_use/main，從精確commit全文讀回並保存blob／SHA256；重查parent與目前state，Gate1條件齊備後才依expected next_day條件同步學習狀態、manifest、七首頁及ENTRY。已同步不再加一；單案受阻繼續其他獨立案，衝突不得強制覆寫。修訂保留原文／歷史證據與FIFO，不增加Day，不更新last_full_comparison。

只寫copilot_use/main；rust_ai_ring唯讀，完整產品變更由使用者套用。有執行工具才在隔離副本與測試DB驗證，保留真命令、exit、SHA與限制；不碰正式DB／秘密／付費AI。核心60分鐘只計Red／Green，Refactor與版本／驗證／交付另計。

每次完成回報三案實際文章連結、commit、讀回／同步狀態、目前next_day、Gate1／Gate2與具體剩餘缺口。不要只交計畫或因環境不足停在籠統候選；中斷時保留已提交成果與單一待續記錄，避免重複提交。
```
