# copilot_use

每日文章另設獨立 Refactor 段落，交付該篇專案的實際重構程式、測試承接與驗證；全專案重構另列獨立項目。核心60分鐘只計Red／Green；rust_ai_ring唯讀，使用者自行套用。

[共用規則](docs/COMMON_LEARNING_RULES.md) · [功能與驗證](docs/teaching/current/STATUS.md) · [工程進度](docs/teaching/functional-refactoring/PROGRESS.md) · [學習進度](docs/learning-state.json) · [manifest](docs/teaching/current/manifest.json)

| 專案 | 現行教學 | 下一篇 | 驗證範圍 |
| --- | --- | --- | --- |
| Chat | [Day38 r4：搜尋失敗可見狀態與既有房間連結](docs/teaching/daily/day-38/chat-r4.md) | Day39 | 候選 SSR 14項通過＋WASM check；產品未套用，GUI／候選CI未驗 |
| Pet | [Day38 r4：逐 UUID 生活窗漫遊任務](docs/teaching/daily/day-38/pet-r4.md) | Day39 | 候選 Application 4項＋Server／SQLite 60項通過；產品未套用，GUI／候選CI未驗 |
| Analytics | [Day38 r4：上報拒絕重複 Authorization](docs/teaching/daily/day-38/analytics-r4.md) | Day39 | 候選 Router／PG 48項＋另跑ignored 1項通過；產品未套用，GUI／候選CI未驗 |

[Chat首頁](docs/teaching/chat/README.md) · [Pet首頁](docs/teaching/pet/README.md) · [Analytics首頁](docs/teaching/analytics/README.md)

目前產品來源 `9b89656`；既有七個workflow全success，[CI快照](docs/teaching/daily/day-38/r2-product-ci-observation.json)。這些既有結果不驗證尚未套用的候選patch。[當日版本與原始證據](docs/teaching/daily/day-38/r2-versions.json)。

[全專案重構獨立項目](docs/engineering/README.md)：跨案契約、共用crates、跨workspace與全庫CI；不計入各案Refactor完成量。

## 使用與追溯

[使用者流程](docs/requirements/USER_FLOWS.md) · [人員驗證](docs/requirements/AUTHENTICATION.md) · [功能待辦](docs/FUTURE_WORK.md) · [架構](docs/SYSTEM_ARCHITECTURE.md) · [來源目錄](docs/SOURCE_LAYOUT.md) · [階段規劃](docs/ROADMAP_P0_P12.md) · [實際網頁排程](docs/teaching/scheduler-instructions-v1.md)

[原文章與驗證總表](docs/teaching/current/ARTICLE_STATUS.md) · [文章更正對照](docs/teaching/current/ARTICLE_CORRECTIONS.md) · [歷史目錄](docs/teaching/history/README.md) · [驗收封存](docs/teaching/archived/README.md) · [三案本修訂讀回](docs/teaching/daily/day-38/r4-delivery-verification.json)。保留各次來源、原文與證據；歷史測試不追認目前驗收。

產品程式、測試與CI以[williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)為準，本庫只維護文件、教學／工程方案與證據。[本地啟動](docs/LOCAL_START.md) · [驗收方式](docs/LOCAL_VALIDATION.md)。
