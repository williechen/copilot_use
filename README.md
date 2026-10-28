# copilot_use｜三專案教學與工程文件

維護 Chat、Desktop Pet、Analytics 的需求、TDD 教學、重構方案、問題追蹤與驗證證據。產品程式與 CI 位於 [williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)。

## 專案與最新文章

| 專案 | 專案首頁 | 最新文章 | 問題主線 |
|---|---|---|---|
| Chat | [專案大綱](docs/teaching/chat/README.md) | [Day42 r3](docs/teaching/daily/day-42/chat-r3.md) | [原完成條件](docs/FUTURE_WORK.md#chat-auth) |
| Pet | [專案大綱](docs/teaching/pet/README.md) | [Day42 r3](docs/teaching/daily/day-42/pet-r3.md) | [原完成條件](docs/FUTURE_WORK.md#pet-lifecycle) |
| Analytics | [專案大綱](docs/teaching/analytics/README.md) | [Day42 r2](docs/teaching/daily/day-42/analytics-r2.md) | [原完成條件](docs/FUTURE_WORK.md#analytics-auth) |

## 目前驗證

固定產品來源 `c6708c42f21ac919e57a379848b4977d27a1c3d6`（2026-10-28）。Chat server／SQLite／SSR 57 項、Pet Node 10 項、四 workspace metadata／fmt 通過。GUI／CI／完整版本 Gate 尚未完成。[本輪原始命令與結果](docs/source-review/local-validation-2026-10-28-c6708c4/README.md) · [完整功能與限制](docs/teaching/current/STATUS.md)。

## 問題與文章追蹤

[功能問題與完成條件](docs/FUTURE_WORK.md) · [文章勘誤／修訂](docs/teaching/current/ARTICLE_CORRECTIONS.md) · [歷史逐篇對照](docs/teaching/current/ARTICLE_STATUS.md)。問題沿既有 FIFO／D01–D09 追蹤；測試修復與功能驗收分開。

## 開發與工程

[來源目錄](docs/SOURCE_LAYOUT.md) · [本地啟動](docs/LOCAL_START.md) · [驗證命令](docs/LOCAL_VALIDATION.md) · [共用教學規則](docs/COMMON_LEARNING_RULES.md) · [各案 Refactor](docs/teaching/functional-refactoring/PROGRESS.md) · [跨案工程](docs/engineering/README.md)。

## 需求與歷史

[使用者流程](docs/requirements/USER_FLOWS.md) · [人員驗證](docs/requirements/AUTHENTICATION.md) · [架構](docs/SYSTEM_ARCHITECTURE.md) · [學習狀態](docs/learning-state.json) · [manifest](docs/teaching/current/manifest.json) · [歷史原文](docs/teaching/history/README.md) · [驗收封存](docs/teaching/archived/README.md)。原文與舊證據保留各自 SHA，未將歷史測試追認新版本。
