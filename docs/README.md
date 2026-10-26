# 文件總入口

每日文章另設獨立 Refactor 段落，交付該篇專案的實際重構程式、測試承接與驗證；全專案重構另列獨立項目。核心60分鐘只計Red／Green；rust_ai_ring唯讀，使用者自行套用。

[共用規則](COMMON_LEARNING_RULES.md) · [功能與驗證](teaching/current/STATUS.md) · [工程進度](teaching/functional-refactoring/PROGRESS.md) · [學習進度](learning-state.json) · [manifest](teaching/current/manifest.json)

| 專案 | 現行教學 | 下一篇 | 驗證範圍 |
| --- | --- | --- | --- |
| Chat | [Day40 r2：首頁依有效 Session 顯示登入／管理](teaching/daily/day-40/chat-r2.md) | Day41 | 本輪Rust59、fmt/check/WASM通過；登入→管理→登出瀏覽器通過 |
| Pet | [Day40 r2：生活窗直接關閉等同逐隻回籠](teaching/daily/day-40/pet-r2.md) | Day41 | Rust72＋Node6通過；原生多窗／關閉／出籠／重啟由使用者操作確認＋DB核對通過 |
| Analytics | [Day40 r1：CSV 匯出遵循目前精確事件類型篩選](teaching/daily/day-40/analytics-r1.md) | Day41 | 本輪真PG53、fmt/check通過；瀏覽器精確篩選／CSV下載通過 |

[Chat首頁](teaching/chat/README.md) · [Pet首頁](teaching/pet/README.md) · [Analytics首頁](teaching/analytics/README.md)

最新程式 `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`；[本輪真正測試](teaching/daily/day-40/rerun-9c76797/README.md)。Chat59、Pet72＋Node6、Analytics真PG53通過；Chat／Analytics指定瀏覽器流程通過。Pet原生指定流程已由使用者操作確認＋DB核對；CI重跑請求被403拒絕。

[全專案重構獨立項目](../docs/engineering/README.md)：跨案契約、共用crates、跨workspace與全庫CI；不計入各案Refactor完成量。

## 使用與追溯

[使用者流程](requirements/USER_FLOWS.md) · [人員驗證](requirements/AUTHENTICATION.md) · [功能待辦](FUTURE_WORK.md) · [架構](SYSTEM_ARCHITECTURE.md) · [來源目錄](SOURCE_LAYOUT.md) · [階段規劃](ROADMAP_P0_P12.md) · [實際網頁排程](teaching/scheduler-instructions-v1.md)

[原文章與驗證總表](teaching/current/ARTICLE_STATUS.md) · [文章更正對照](teaching/current/ARTICLE_CORRECTIONS.md) · [歷史目錄](teaching/history/README.md) · [驗收封存](teaching/archived/README.md) · [三案本修訂讀回](teaching/daily/day-39/r2-delivery-verification.json)。保留各次來源、原文與證據；歷史測試不追認目前驗收。

> Day39前輪完整本地驗證與歷史勘誤已補入[對照](teaching/daily/day-40/historical-corrections.md)；限其原SHA，不追認Day40。

> Day40 r1保存[實際套用、Cargo、SQLite／PG與原始錯誤](teaching/daily/day-40/local-validation/README.md)。產品功能、修復候選、GUI與CI分列；next_day維持41。

[Pet原生GUI人工操作與資料庫交叉驗證](teaching/daily/day-40/native-gui-9c76797/README.md)：指定多窗、原生關閉、重新出籠與兩種重啟流程通過，完整Gate2未通過。
