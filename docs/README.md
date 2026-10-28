# 文件總入口

先依閱讀目的選擇入口；各頁只維護自己負責的資訊。

## 專案與最新文章

| 專案 | 專案首頁 | 最新文章 | 問題主線 |
|---|---|---|---|
| Chat | [專案大綱](teaching/chat/README.md) | [Day42 r2](teaching/daily/day-42/chat-r2.md) | [原完成條件](FUTURE_WORK.md#chat-auth) |
| Pet | [專案大綱](teaching/pet/README.md) | [Day42 r2](teaching/daily/day-42/pet-r2.md) | [原完成條件](FUTURE_WORK.md#pet-lifecycle) |
| Analytics | [專案大綱](teaching/analytics/README.md) | [Day42 r1](teaching/daily/day-42/analytics-r1.md) | [原完成條件](FUTURE_WORK.md#analytics-auth) |

## 目前驗證

固定產品來源 `c6708c42f21ac919e57a379848b4977d27a1c3d6`（2026-10-28）。Chat server／SQLite／SSR 57 項、Pet Node 10 項、四 workspace metadata／fmt 通過。GUI／CI／完整版本 Gate 尚未完成。[本輪原始命令與結果](source-review/local-validation-2026-10-28-c6708c4/README.md) · [完整功能與限制](teaching/current/STATUS.md)。

## 問題與文章追蹤

[功能問題與完成條件](FUTURE_WORK.md) · [文章勘誤／修訂](teaching/current/ARTICLE_CORRECTIONS.md) · [歷史逐篇對照](teaching/current/ARTICLE_STATUS.md)。問題沿既有 FIFO／D01–D09 追蹤；測試修復與功能驗收分開。

## 開發與工程

[來源目錄](SOURCE_LAYOUT.md) · [本地啟動](LOCAL_START.md) · [驗證命令](LOCAL_VALIDATION.md) · [共用教學規則](COMMON_LEARNING_RULES.md) · [各案 Refactor](teaching/functional-refactoring/PROGRESS.md) · [跨案工程](engineering/README.md)。

## 需求與歷史

[使用者流程](requirements/USER_FLOWS.md) · [人員驗證](requirements/AUTHENTICATION.md) · [架構](SYSTEM_ARCHITECTURE.md) · [學習狀態](learning-state.json) · [manifest](teaching/current/manifest.json) · [歷史原文](teaching/history/README.md) · [驗收封存](teaching/archived/README.md)。原文與舊證據保留各自 SHA，未將歷史測試追認新版本。
