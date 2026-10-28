# 教學、各案Refactor與全專案重構

> 2026-10-28 最新固定來源 `c801cb227e4862b8f49b5071faece70ba0b7e9c9`；[最後來源刷新與獨立重跑](source-review/local-validation-2026-10-28-c801cb2/README.md)。下列 e8a6409 與其他舊來源結果保留原 SHA；未完成 GUI／CI／完整版本 Gate，next_day 三案維持 42。


功能Red／Green只處理當日可操作功能。各篇在①後設與②並列的獨立「Refactor」段落，只處理該篇Chat、Pet或Analytics自己的架構、測試、fixture與專案CI，按該案累積相依每日接續，不作Green接續的局部整理。

Refactor必須有完整可套用程式／patch、正式caller／module／Cargo／features接線、舊測試與有效斷言去向、實際／未執行驗證、風險／回復與下一步；不能只有說明或目錄。既有行為重構不假造新功能Red，純內層、DB、HTTP／IPC與UI的獨有斷言保留。

[各案入口](teaching/functional-refactoring/README.md)與[各案進度](teaching/functional-refactoring/PROGRESS.md)只追蹤各專案。[全專案重構](engineering/README.md)另列跨案契約、共用crates、跨workspace及全庫CI等獨立項目；其完整變更與證據不複製到各案段落，亦不抵算各案進度。相依時明列，不把未套用項目當主分支已存在。

核心60分鐘只計Red／Green，兩類Refactor及其他作業另計。copilot_use只維護文件、程式／patch方案與證據；rust_ai_ring唯讀，由使用者套用。保留已發布原文、有效斷言與FIFO，修訂不增減Day，不更新last_full_comparison。精確來源、當日版本、逐案獨立提交與全文讀回依[共用規則](COMMON_LEARNING_RULES.md)，路徑依[SOURCE_LAYOUT](SOURCE_LAYOUT.md)。
