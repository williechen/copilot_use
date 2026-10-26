# copilot_use

每日文章另設獨立 Refactor 段落，交付該篇專案的實際重構程式、測試承接與驗證；全專案重構另列獨立項目。核心60分鐘只計Red／Green；rust_ai_ring唯讀，使用者自行套用。

[共用規則](docs/COMMON_LEARNING_RULES.md) · [功能與驗證](docs/teaching/current/STATUS.md) · [工程進度](docs/teaching/functional-refactoring/PROGRESS.md) · [學習進度](docs/learning-state.json) · [manifest](docs/teaching/current/manifest.json)

| 專案 | 現行教學 | 下一篇 | 驗證範圍 |
| --- | --- | --- | --- |
| Chat | [Day40：首頁依有效 Session 顯示登入／管理](docs/teaching/daily/day-40/chat.md) | Day41 | Day40 方案已交付；Refactor／功能 Green／GUI／候選 CI 未驗 |
| Pet | [Day40：生活窗直接關閉等同逐隻回籠](docs/teaching/daily/day-40/pet.md) | Day41 | Day40 方案已交付；Refactor／功能 Green／Tauri／GUI 未驗 |
| Analytics | [Day40：CSV 匯出遵循目前精確事件類型篩選](docs/teaching/daily/day-40/analytics.md) | Day41 | Day40 方案已交付；Refactor／CSV Green／PG／GUI 未驗 |

[Chat首頁](docs/teaching/chat/README.md) · [Pet首頁](docs/teaching/pet/README.md) · [Analytics首頁](docs/teaching/analytics/README.md)

本篇來源 `8fc4b277b6f6f77d97655130036fca47813de56e`；前次來源 `9b89656` 的既有七個workflow全success，[CI快照](docs/teaching/daily/day-38/r2-product-ci-observation.json)。這些既有結果不驗證尚未套用的候選patch。[當日版本與原始證據](docs/teaching/daily/day-38/r2-versions.json)。

[全專案重構獨立項目](docs/engineering/README.md)：跨案契約、共用crates、跨workspace與全庫CI；不計入各案Refactor完成量。

## 使用與追溯

[使用者流程](docs/requirements/USER_FLOWS.md) · [人員驗證](docs/requirements/AUTHENTICATION.md) · [功能待辦](docs/FUTURE_WORK.md) · [架構](docs/SYSTEM_ARCHITECTURE.md) · [來源目錄](docs/SOURCE_LAYOUT.md) · [階段規劃](docs/ROADMAP_P0_P12.md) · [實際網頁排程](docs/teaching/scheduler-instructions-v1.md)

[原文章與驗證總表](docs/teaching/current/ARTICLE_STATUS.md) · [文章更正對照](docs/teaching/current/ARTICLE_CORRECTIONS.md) · [歷史目錄](docs/teaching/history/README.md) · [驗收封存](docs/teaching/archived/README.md) · [三案本修訂讀回](docs/teaching/daily/day-39/r2-delivery-verification.json)。保留各次來源、原文與證據；歷史測試不追認目前驗收。

產品程式、測試與CI以[williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)為準，本庫只維護文件、教學／工程方案與證據。[本地啟動](docs/LOCAL_START.md) · [驗收方式](docs/LOCAL_VALIDATION.md)。

> Day39 三案 Refactor 只計各案；全庫 G-01／G-02 保留在 [工程首頁](docs/engineering/README.md)，未重複計功。正文已補完整重構程式與測試；隔離Refactor驗證通過，產品未套用，功能Green與GUI仍待驗。
