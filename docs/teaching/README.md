# 三專案教學

最新功能教學與各案獨立 Refactor 分別驗收。核心 60 分鐘只計 Red／Green。

## 專案與最新文章

| 專案 | 專案首頁 | 最新文章 | 問題主線 |
|---|---|---|---|
| Chat | [專案大綱](chat/README.md) | [Day42 r3](daily/day-42/chat-r3.md) | [原完成條件](../FUTURE_WORK.md#chat-auth) |
| Pet | [專案大綱](pet/README.md) | [Day42 r3](daily/day-42/pet-r3.md) | [原完成條件](../FUTURE_WORK.md#pet-lifecycle) |
| Analytics | [專案大綱](analytics/README.md) | [Day42 r2](daily/day-42/analytics-r2.md) | [原完成條件](../FUTURE_WORK.md#analytics-auth) |

## 目前驗證

固定產品來源 `c6708c42f21ac919e57a379848b4977d27a1c3d6`（2026-10-28）。Chat server／SQLite／SSR 57 項、Pet Node 10 項、四 workspace metadata／fmt 通過。GUI／CI／完整版本 Gate 尚未完成。[本輪原始命令與結果](../source-review/local-validation-2026-10-28-c6708c4/README.md) · [完整功能與限制](current/STATUS.md)。

## 問題與文章追蹤

[功能問題與完成條件](../FUTURE_WORK.md) · [文章勘誤／修訂](current/ARTICLE_CORRECTIONS.md) · [歷史逐篇對照](current/ARTICLE_STATUS.md)。問題沿既有 FIFO／D01–D09 追蹤；測試修復與功能驗收分開。

## 開發與工程

[來源目錄](../SOURCE_LAYOUT.md) · [本地啟動](../LOCAL_START.md) · [驗證命令](../LOCAL_VALIDATION.md) · [共用教學規則](../COMMON_LEARNING_RULES.md) · [各案 Refactor](functional-refactoring/PROGRESS.md) · [跨案工程](../engineering/README.md)。

## 需求與歷史

[使用者流程](../requirements/USER_FLOWS.md) · [人員驗證](../requirements/AUTHENTICATION.md) · [架構](../SYSTEM_ARCHITECTURE.md) · [學習狀態](../learning-state.json) · [manifest](current/manifest.json) · [歷史原文](history/README.md) · [驗收封存](archived/README.md)。原文與舊證據保留各自 SHA，未將歷史測試追認新版本。
