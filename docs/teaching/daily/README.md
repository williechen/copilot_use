# 每日教學與歷史文章

> 文件保存規則：已驗證的 Day43 教學正文依指定 [只留既有 ZIP](day-43/README.md)；未驗證原稿、規則、入口、狀態及證據保留原檔。Day44 尚無原稿，未補造；其後未驗證原稿不刪除。

> **2026-10-30 SSR 修補追驗：** 產品已更新至 `7ce39f0166a2120539e5908b33c88322a930c35a`，正式 main 已套用 Router imports。隔離原樣測試 UI SSR 21 項、server／SQLite／UI SSR 組合 60 項及 metadata／fmt 通過。[新 SHA 命令與 CI 快照](../../source-review/day43-7ce39f0-2026-10-30/README.md)；遠端 CI 仍執行中，未稱全綠。其他 workspace 與 GUI／WASM／DB 保留各自原 SHA 的證據；next_day 三案 44、FIFO、last_full_comparison 不變，完整 Gate2 未結案。下方較早日期與 SHA 為歷史快照。

> 2026-10-29 驗收增量：[網頁排程已儲存、GUI／CI／舊庫實測](../../source-review/acceptance-2026-10-29/README.md)。Chat 真瀏覽器另發現清除搜尋回歸，修補候選已通過；CI attempt #2 仍失敗，Pet 隔離 A/B 原生回籠生命週期已由人工操作及 SQLite 核對通過，其他原生情境／真實舊 PG 備份待驗。Gate2 未結案。

> 2026-10-29 Day43 本地實測：固定產品 `8b882696c5548accb9d98a32b27db35e11109c74`；[測試矩陣與原始證據](../../source-review/local-validation-2026-10-29-day43/README.md) · [今日交付延遲原因](../../source-review/local-validation-2026-10-29-day43/diagnosis.md)。下列舊 SHA 結果保留原範圍；next_day 維持 44，完整 GUI／CI／舊 DB 驗收未完成。

目前閱讀 Day43；Day42 文件交付掛起狀態已依原證據結案、Day43 已正式文件發布，三案下一待發布篇為 Day44。文件發布不代表產品套用或 Gate2 驗收。

## 專案與最新文章

| 專案 | 專案首頁 | 最新文章 | 問題主線 |
|---|---|---|---|
| Chat | [專案大綱](../chat/README.md) | [Day43](day-43/chat-archive.md) | [原完成條件](../../FUTURE_WORK.md#chat-auth) |
| Pet | [專案大綱](../pet/README.md) | [Day43](day-43/pet-archive.md) | [原完成條件](../../FUTURE_WORK.md#pet-lifecycle) |
| Analytics | [專案大綱](../analytics/README.md) | [Day43](day-43/analytics-archive.md) | [原完成條件](../../FUTURE_WORK.md#analytics-auth) |

## 目前驗證

固定產品 `8b882696c5548accb9d98a32b27db35e11109c74`（2026-10-29）。四 workspace metadata／check／clippy 通過；Chat SSR 原樣 1 項 fixture 失敗，補修候選 UI 21 項通過；Pet fmt 原樣失敗、workspace 清理後重試通過；兩案 WASM／Node／Analytics 隔離 PG 測試已執行。[詳細矩陣](../../source-review/local-validation-2026-10-29-day43/README.md)。

## 問題與文章追蹤

[功能問題與完成條件](../../FUTURE_WORK.md) · [文章勘誤／修訂](../current/ARTICLE_CORRECTIONS.md) · [歷史逐篇對照](../current/ARTICLE_STATUS.md)。問題沿既有 FIFO／D01–D09 追蹤；測試修復與功能驗收分開。

## 開發與工程

[來源目錄](../../SOURCE_LAYOUT.md) · [本地啟動](../../LOCAL_START.md) · [驗證命令](../../LOCAL_VALIDATION.md) · [共用教學規則](../../COMMON_LEARNING_RULES.md) · [各案 Refactor](../functional-refactoring/PROGRESS.md) · [跨案工程](../../engineering/README.md)。

## 需求與歷史

[使用者流程](../../requirements/USER_FLOWS.md) · [人員驗證](../../requirements/AUTHENTICATION.md) · [架構](../../SYSTEM_ARCHITECTURE.md) · [學習狀態](../../learning-state.json) · [manifest](../current/manifest.json) · [歷史原文](../history/README.md) · [驗收封存](../archived/README.md)。原文與舊證據保留各自 SHA，未將歷史測試追認新版本。

## 近期交付

| Day | 文章／附件／證據 |
|---|---|
| 43 | [Day43 三案正式教學](day-43/README.md) · [交付證據](day-43/delivery-verification.json) |
| 42 | [Day42 索引](day-42/README.md) |
| 41 | [Chat](day-41/chat.md) · [Pet](day-41/pet.md) · [Analytics](day-41/analytics.md) |
| 40 | [Chat](day-40/chat-r2.md) · [Pet](day-40/pet-r2.md) · [Analytics](day-40/analytics-r1.md) |
| 39 | [Chat](day-39/chat-r3.md) · [Pet](day-39/pet-r3.md) · [Analytics](day-39/analytics-r3.md) |
