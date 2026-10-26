# copilot_use

每日文章另設獨立 Refactor 段落，交付該篇專案的實際重構程式、測試承接與驗證；全專案重構另列獨立項目。核心60分鐘只計Red／Green；rust_ai_ring唯讀，使用者自行套用。

[共用規則](docs/COMMON_LEARNING_RULES.md) · [功能與驗證](docs/teaching/current/STATUS.md) · [工程進度](docs/teaching/functional-refactoring/PROGRESS.md) · [學習進度](docs/learning-state.json) · [manifest](docs/teaching/current/manifest.json)

| 專案 | 現行教學 | 下一篇 | 驗證範圍 |
| --- | --- | --- | --- |
| Chat | [Day40 r1：首頁依有效 Session 顯示登入／管理](docs/teaching/daily/day-40/chat-r1.md) | Day41 | 最新碼/account回歸，測試失敗；修復候選另驗；GUI／CI未驗 |
| Pet | [Day40 r1：生活窗直接關閉等同逐隻回籠](docs/teaching/daily/day-40/pet-r1.md) | Day41 | 最新碼缺X回籠及3項舊測試；候選／原生／WASM另驗；GUI未驗 |
| Analytics | [Day40 r1：CSV 匯出遵循目前精確事件類型篩選](docs/teaching/daily/day-40/analytics-r1.md) | Day41 | 真PG／Router、check通過；候選fmt失敗，最新fmt通過；GUI／CI未驗 |

[Chat首頁](docs/teaching/chat/README.md) · [Pet首頁](docs/teaching/pet/README.md) · [Analytics首頁](docs/teaching/analytics/README.md)

最新程式 `ff14ca6224f7c5a65c799bf4a82a743fa79c880c`；原教學基底 `bb259aaf76542554a3a704d9c4f4a3a219d65cbf`。[真正本地命令與結果](docs/teaching/daily/day-40/local-validation/README.md)：最新Chat測試失敗，Pet功能接線不完整；候選成功不抵消產品問題。

[全專案重構獨立項目](docs/engineering/README.md)：跨案契約、共用crates、跨workspace與全庫CI；不計入各案Refactor完成量。

## 使用與追溯

[使用者流程](docs/requirements/USER_FLOWS.md) · [人員驗證](docs/requirements/AUTHENTICATION.md) · [功能待辦](docs/FUTURE_WORK.md) · [架構](docs/SYSTEM_ARCHITECTURE.md) · [來源目錄](docs/SOURCE_LAYOUT.md) · [階段規劃](docs/ROADMAP_P0_P12.md) · [實際網頁排程](docs/teaching/scheduler-instructions-v1.md)

[原文章與驗證總表](docs/teaching/current/ARTICLE_STATUS.md) · [文章更正對照](docs/teaching/current/ARTICLE_CORRECTIONS.md) · [歷史目錄](docs/teaching/history/README.md) · [驗收封存](docs/teaching/archived/README.md) · [三案本修訂讀回](docs/teaching/daily/day-39/r2-delivery-verification.json)。保留各次來源、原文與證據；歷史測試不追認目前驗收。

產品程式、測試與CI以[williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)為準，本庫只維護文件、教學／工程方案與證據。[本地啟動](docs/LOCAL_START.md) · [驗收方式](docs/LOCAL_VALIDATION.md)。

> Day39前輪完整本地驗證與歷史勘誤已補入[對照](docs/teaching/daily/day-40/historical-corrections.md)；限其原SHA，不追認Day40。

> Day40 r1保存[實際套用、Cargo、SQLite／PG與原始錯誤](docs/teaching/daily/day-40/local-validation/README.md)。產品功能、修復候選、GUI與CI分列；next_day維持41。
