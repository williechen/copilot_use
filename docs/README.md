# 文件總入口

> 2026-10-29 驗收增量：[網頁排程已儲存、GUI／CI／舊庫實測](source-review/acceptance-2026-10-29/README.md)。Chat 真瀏覽器另發現清除搜尋回歸，修補候選已通過；CI attempt #2 仍失敗，Pet 原生人工 GUI／真實舊 PG 備份待驗。Gate2 未結案。

> 2026-10-29 Day43 本地實測：固定產品 `8b882696c5548accb9d98a32b27db35e11109c74`；[測試矩陣與原始證據](source-review/local-validation-2026-10-29-day43/README.md) · [今日交付延遲原因](source-review/local-validation-2026-10-29-day43/diagnosis.md)。下列舊 SHA 結果保留原範圍；next_day 維持 44，完整 GUI／CI／舊 DB 驗收未完成。

**2026-10-29：已正式發布 [Day43 三案教學](teaching/daily/day-43/README.md)（Gate1 文件交付）；最新產品已含 Day43 實作；完整 Gate2 未驗收。三案下一待發布篇為 Day44。**

先依閱讀目的選擇入口；各頁只維護自己負責的資訊。

## 專案與最新文章

| 專案 | 專案首頁 | 最新文章 | 問題主線 |
|---|---|---|---|
| Chat | [專案大綱](teaching/chat/README.md) | [Day43](teaching/daily/day-43/chat-r2.md) | [原完成條件](FUTURE_WORK.md#chat-auth) |
| Pet | [專案大綱](teaching/pet/README.md) | [Day43](teaching/daily/day-43/pet-r1.md) | [原完成條件](FUTURE_WORK.md#pet-lifecycle) |
| Analytics | [專案大綱](teaching/analytics/README.md) | [Day43](teaching/daily/day-43/analytics-r1.md) | [原完成條件](FUTURE_WORK.md#analytics-auth) |

## 目前驗證

固定產品 `8b882696c5548accb9d98a32b27db35e11109c74`（2026-10-29）。四 workspace metadata／check／clippy 通過；Chat SSR 原樣 1 項 fixture 失敗，補修候選 UI 21 項通過；Pet fmt 原樣失敗、workspace 清理後重試通過；兩案 WASM／Node／Analytics 隔離 PG 測試已執行。[詳細矩陣](source-review/local-validation-2026-10-29-day43/README.md)。

## 問題與文章追蹤

[功能問題與完成條件](FUTURE_WORK.md) · [文章勘誤／修訂](teaching/current/ARTICLE_CORRECTIONS.md) · [歷史逐篇對照](teaching/current/ARTICLE_STATUS.md)。問題沿既有 FIFO／D01–D09 追蹤；測試修復與功能驗收分開。

## 開發與工程

[來源目錄](SOURCE_LAYOUT.md) · [本地啟動](LOCAL_START.md) · [驗證命令](LOCAL_VALIDATION.md) · [共用教學規則](COMMON_LEARNING_RULES.md) · [各案 Refactor](teaching/functional-refactoring/PROGRESS.md) · [跨案工程](engineering/README.md)。

## 需求與歷史

[使用者流程](requirements/USER_FLOWS.md) · [人員驗證](requirements/AUTHENTICATION.md) · [架構](SYSTEM_ARCHITECTURE.md) · [學習狀態](learning-state.json) · [manifest](teaching/current/manifest.json) · [歷史原文](teaching/history/README.md) · [驗收封存](teaching/archived/README.md)。原文與舊證據保留各自 SHA，未將歷史測試追認新版本。
