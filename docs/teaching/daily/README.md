# 每日新文章

<!-- duplicate-review:start -->
[後續調整清單（13 項）](../../FUTURE_WORK.md) · [原始核對報告](../../source-review/2026-10-25-duplicate-application-review.md)。可清除、先遷測試與保留項目分別列明，產品套用／驗收另記。
<!-- duplicate-review:end -->

<!-- daily-delivery:start -->
## 最新交付與下一篇

| 專案 | 最新文章 | 下一篇 | 狀態 |
| --- | --- | --- | --- |
| Chat | [Day34 r2：重用 SignInPage，合併重複登入入口](day-34/chat-r2.md) | Day35 | 教學已交付／功能未驗收 |
| Pet | [Day34：新增寵物名稱規則接入 application 並拒絕控制字元](day-34/pet.md) | Day35 | 教學已交付／功能未驗收 |
| Analytics | [Day34：首次 admin/admin 初始化及強制改密碼銜接](day-34/analytics.md) | Day35 | 教學已交付／功能未驗收 |

來源、套用及測試證據各自記錄；文章發布不代表產品驗收。
<!-- daily-delivery:end -->

2026-10-24 目錄已同步至 [williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)：保留 `chat_apps`、`pet_apps`、`analytics_apps`、`crates` 四個 workspace；[新目錄與套件對照](../../SOURCE_LAYOUT.md)。後續教學依當次來源 SHA 調整程式、Cargo、imports、呼叫端與測試。

[共用文章](../../COMMON_LEARNING_RULES.md) · [Chat 首頁](../chat/README.md) · [Pet 首頁](../pet/README.md) · [Analytics 首頁](../analytics/README.md) · [目前進度](../current/STATUS.md)

**必要 Gate 2 驗證通過後，正文原樣移至 `../archived/day-NN/{project}.md`。** 本目錄只追蹤尚待驗收或勘誤的教學；舊有 `history` 已封存文章與證據保留原位，不批次重寫。

## 先前交付快照（三專案 Day33）

[Analytics Day33：Dashboard 數量標示區分全部與精確篩選](day-33/analytics.md)（教學已發布；程式未套用、測試未執行、功能未驗收；下一篇 Day34）。

[Pet Day33：生活窗「控制台」按鈕選取來源寵物](day-33/pet.md)（教學已發布；程式未套用、測試未執行、功能未驗收；下一篇 Day34）。

[Chat Day33：房間搜尋結果與尚未載入狀態](day-33/chat.md)（教學已發布、版本補核見 [附件](day-33/chat-version-verification.md)；程式未套用、測試未執行、功能未驗收；下一篇 Day34）。

## Day32 教學

| 專案 | 文章 | 狀態 |
| --- | --- | --- |
| Chat | [Day32：聊天室歷史載入狀態](day-32/chat.md) | 教學已交付；後續主體程式驗證另見目前進度，完整功能仍未驗收 |
| Pet | [Day32：右鍵開啟控制窗並選取對應寵物](day-32/pet.md) | 教學已交付；後續主體程式驗證另見目前進度，完整功能仍未驗收 |
| Analytics | [Day32：Dashboard 事件類型篩選](day-32/analytics.md) | 教學已交付；後續主體程式驗證另見目前進度，完整功能仍未驗收 |

本表保留 Day32 發布時的狀態；最新交付與下一篇以本頁頂端各案清單為準；以 [learning-state](../../learning-state.json) 為準。

## 尚待完成

| Day | 專案／文章 | 尚待完成的必要項目 |
| --- | --- | --- |
| 26 | [Pet](day-26/pet.md) | 三個不同 pet id 的每幀單一圖像、事件與真實 GUI 驗收 |
| 27 | [Pet](day-27/pet.md) | 建立／選取後的真實 Tauri 出籠與 App 重啟載入 |
| 28 | [Chat](day-28/chat.md) | 原文 Leptos SSR API 勘誤；完整 SSR 回歸失敗另列追蹤 |
| 28 | [Pet](day-28/pet.md) | 回籠後真實 App 重啟及 GUI 一致性 |
| 28 | [Analytics](day-28/analytics.md) | 原文 Leptos SSR API 的版本化勘誤；目前程式已修正 |
| 29 | [Pet](day-29/pet.md) | 真實 GUI 的 DB 失敗／關窗失敗注入與狀態一致性 |
| 30 | [Chat r2](day-30/chat.md) | POST 建房授權、WS/ACL 實測尚未完成 |
| 30 | [Pet](day-30/pet.md) | SQLite 失敗補償與三窗原生 GUI 驗收尚未完成 |
| 30 | [Analytics](day-30/analytics.md) | Leptos 登入頁尚未套用；PG／路由整合尚待驗證 |

| 31 | [chat：個人頁建房](day-31/chat.md) | 表單、CSRF、HTTP／SQLite 與瀏覽器流程待驗證；逐房 ACL 等缺口保留；程式未套用、測試未執行 |

| 31 | [pet：控制窗關閉後重開](day-31/pet.md) | 原生 close／hide／重開與三窗 GUI 未驗證；回籠補償及單圖 FIFO 保留；程式未套用、測試未執行 |

| 31 | [analytics：Leptos 改密碼頁](day-31/analytics.md) | SSR／正式路由、隔離 PostgreSQL 全 targets 與瀏覽器驗收未執行；初始化及 ACL 差異保留；程式未套用、測試未執行 |

上述結果以 [逐篇驗證矩陣](../history/evidence/2026-10-23-all-articles-9c2b7cf/ALL_ARTICLES.md) 與 [最新補驗報告](../history/evidence/2026-10-23-browser-sqlite-9c2b7cf/REPORT.md) 的精確來源及覆蓋範圍為準。文章發布、程式已套用及單元測試通過，不直接等於本篇必要驗證完成。

## 完成後封存

1. 核對文章要求、`rust_ai_ring` 精確 SHA、實際呼叫鏈與本篇必要測試／DB／HTTP／瀏覽器或 GUI 證據；未驗收或失敗項不得標為完成。
2. 將完成文章正文原樣移至 `archived/day-NN/{project}.md`，保留原 commit、blob、全文 SHA256、驗證結果與路徑對照；已在 history 的原文只補完成證據，不重複搬移或改寫。
3. 同步本清單、專案首頁、目前進度、manifest 與歷史索引，再從 GitHub 精確提交讀回核對。必要勘誤另存，保留原發布內容。

[已驗收文章](../archived/README.md) · [既有歷史與證據](../history/README.md) · [已驗證完成切片清單](../history/evidence/2026-10-23-browser-sqlite-9c2b7cf/archive-manifest.json)

第③節依共同規則核對本篇涉及套件及軟體**交付當日官方最新穩定版**，附官方來源、核對日、現有 manifest／lock 或安裝版、相容性與選版理由；未查明不得宣稱最新或通過 Gate 1。最新穩定版優先作為教學目標，`rust_ai_ring` 維持唯讀，不自動升級。

新文章按 `day-NN/{chat,pet,analytics}.md` 存放。每新增一個專案 Day 立即獨立 commit／推送，全文讀回後條件同步狀態與 manifest，再讀回；不能累積多天或多案一起提交。實際下一 Day 取 [learning-state.json](../../learning-state.json)，不依目錄推算。封存、勘誤及驗證更新均不增加 Day，也不改變完整產品驗收的判定。
