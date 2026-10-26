# copilot_use

每日文章另設獨立 Refactor 段落，交付該篇專案的實際重構程式、測試承接與驗證；全專案重構另列獨立項目。核心60分鐘只計Red／Green；rust_ai_ring唯讀，使用者自行套用。

[共用規則](docs/COMMON_LEARNING_RULES.md) · [功能與驗證](docs/teaching/current/STATUS.md) · [工程進度](docs/teaching/functional-refactoring/PROGRESS.md) · [學習進度](docs/learning-state.json) · [manifest](docs/teaching/current/manifest.json)

| 專案 | 現行教學 | 下一篇 | 驗證範圍 |
| --- | --- | --- | --- |
| Chat | [Day41：公開房間選擇後登入並返回原房間](docs/teaching/daily/day-41/chat.md) | Day42 | Day41 方案已發布／靜態 hunk 通過；未套用、未編譯、未驗收 |
| Pet | [Day41：控制台逐隻回籠](docs/teaching/daily/day-41/pet.md) | Day42 | Day41 方案已發布／靜態 hunk 通過；未套用、未編譯、未驗收 |
| Analytics | [Day41：管理後台事件類型明細](docs/teaching/daily/day-41/analytics.md) | Day42 | Day41 方案已發布／靜態 hunk 通過；未套用、未編譯、未驗收 |

> **Day41 交付**：三篇文章與各自 Refactor／Green patch 已獨立提交、精確全文讀回；僅靜態 preimage 通過，未 git apply／Cargo／Node／PG／GUI／CI。下方既有 Day40 測試與人工 GUI 證據只屬原功能，**不能追認 Day41**。[Day41 SHA256 證據](docs/teaching/daily/day-41/delivery-verification.json) · [Rust stable 勘誤](docs/teaching/daily/day-41/version-correction.md) · [跨案工程獨立追蹤](docs/engineering/README.md)。

[Chat首頁](docs/teaching/chat/README.md) · [Pet首頁](docs/teaching/pet/README.md) · [Analytics首頁](docs/teaching/analytics/README.md)

最新程式 `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`；[本輪真正測試](docs/teaching/daily/day-40/rerun-9c76797/README.md)。Chat59、Pet72＋Node6、Analytics真PG53通過；Chat／Analytics指定瀏覽器流程通過。Pet原生指定流程已由使用者操作確認＋DB核對；CI重跑請求被403拒絕。

[全專案重構獨立項目](docs/engineering/README.md)：跨案契約、共用crates、跨workspace與全庫CI；不計入各案Refactor完成量。

## 使用與追溯

[使用者流程](docs/requirements/USER_FLOWS.md) · [人員驗證](docs/requirements/AUTHENTICATION.md) · [功能待辦](docs/FUTURE_WORK.md) · [架構](docs/SYSTEM_ARCHITECTURE.md) · [來源目錄](docs/SOURCE_LAYOUT.md) · [階段規劃](docs/ROADMAP_P0_P12.md) · [實際網頁排程](docs/teaching/scheduler-instructions-v1.md)

[原文章與驗證總表](docs/teaching/current/ARTICLE_STATUS.md) · [文章更正對照](docs/teaching/current/ARTICLE_CORRECTIONS.md) · [歷史目錄](docs/teaching/history/README.md) · [驗收封存](docs/teaching/archived/README.md) · [三案本修訂讀回](docs/teaching/daily/day-39/r2-delivery-verification.json)。保留各次來源、原文與證據；歷史測試不追認目前驗收。

產品程式、測試與CI以[williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)為準，本庫只維護文件、教學／工程方案與證據。[本地啟動](docs/LOCAL_START.md) · [驗收方式](docs/LOCAL_VALIDATION.md)。

> Day39前輪完整本地驗證與歷史勘誤已補入[對照](docs/teaching/daily/day-40/historical-corrections.md)；限其原SHA，不追認Day40。

> Day40 r1保存[實際套用、Cargo、SQLite／PG與原始錯誤](docs/teaching/daily/day-40/local-validation/README.md)。產品功能、修復候選、GUI與CI分列；next_day維持41。

[Pet原生GUI人工操作與資料庫交叉驗證](docs/teaching/daily/day-40/native-gui-9c76797/README.md)：指定多窗、原生關閉、重新出籠與兩種重啟流程通過，完整Gate2未通過。
