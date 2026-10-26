# 三專案教學首頁

每日文章另設獨立 Refactor 段落，交付該篇專案的實際重構程式、測試承接與驗證；全專案重構另列獨立項目。核心60分鐘只計Red／Green；rust_ai_ring唯讀，使用者自行套用。

[共用規則](../COMMON_LEARNING_RULES.md) · [功能與驗證](current/STATUS.md) · [工程進度](functional-refactoring/PROGRESS.md) · [學習進度](../learning-state.json) · [manifest](current/manifest.json)

| 專案 | 現行教學 | 下一篇 | 驗證範圍 |
| --- | --- | --- | --- |
| Chat | [Day40 r1：首頁依有效 Session 顯示登入／管理](daily/day-40/chat-r1.md) | Day41 | 最新碼/account回歸，測試失敗；修復候選另驗；GUI／CI未驗 |
| Pet | [Day40 r1：生活窗直接關閉等同逐隻回籠](daily/day-40/pet-r1.md) | Day41 | 最新碼缺X回籠及3項舊測試；候選／原生／WASM另驗；GUI未驗 |
| Analytics | [Day40 r1：CSV 匯出遵循目前精確事件類型篩選](daily/day-40/analytics-r1.md) | Day41 | 真PG／Router、check通過；候選fmt失敗，最新fmt通過；GUI／CI未驗 |

[Chat首頁](chat/README.md) · [Pet首頁](pet/README.md) · [Analytics首頁](analytics/README.md)

最新程式 `ff14ca6224f7c5a65c799bf4a82a743fa79c880c`；原教學基底 `bb259aaf76542554a3a704d9c4f4a3a219d65cbf`。[真正本地命令與結果](daily/day-40/local-validation/README.md)：最新Chat測試失敗，Pet功能接線不完整；候選成功不抵消產品問題。

[全專案重構獨立項目](../../docs/engineering/README.md)：跨案契約、共用crates、跨workspace與全庫CI；不計入各案Refactor完成量。

## 使用與追溯

[使用者流程](../requirements/USER_FLOWS.md) · [人員驗證](../requirements/AUTHENTICATION.md) · [功能待辦](../FUTURE_WORK.md) · [架構](../SYSTEM_ARCHITECTURE.md) · [來源目錄](../SOURCE_LAYOUT.md) · [階段規劃](../ROADMAP_P0_P12.md) · [實際網頁排程](scheduler-instructions-v1.md)

[原文章與驗證總表](current/ARTICLE_STATUS.md) · [文章更正對照](current/ARTICLE_CORRECTIONS.md) · [歷史目錄](history/README.md) · [驗收封存](archived/README.md) · [三案本修訂讀回](daily/day-39/r2-delivery-verification.json)。保留各次來源、原文與證據；歷史測試不追認目前驗收。

> Day39前輪完整本地驗證與歷史勘誤已補入[對照](daily/day-40/historical-corrections.md)；限其原SHA，不追認Day40。

> Day40 r1保存[實際套用、Cargo、SQLite／PG與原始錯誤](daily/day-40/local-validation/README.md)。產品功能、修復候選、GUI與CI分列；next_day維持41。
