# 每日新文章

[共用文章](../../COMMON_LEARNING_RULES.md) · [Chat 首頁](../chat/README.md) · [Pet 首頁](../pet/README.md) · [Analytics 首頁](../analytics/README.md) · [目前進度](../current/STATUS.md)

**必要 Gate 2 驗證通過後，正文原樣移至 `../archived/day-NN/{project}.md`。** 本目錄只追蹤尚待驗收或勘誤的教學；舊有 `history` 已封存文章與證據保留原位，不批次重寫。

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

上述結果以 [逐篇驗證矩陣](../history/evidence/2026-10-23-all-articles-9c2b7cf/ALL_ARTICLES.md) 與 [最新補驗報告](../history/evidence/2026-10-23-browser-sqlite-9c2b7cf/REPORT.md) 的精確來源及覆蓋範圍為準。文章發布、程式已套用及單元測試通過，不直接等於本篇必要驗證完成。

## 完成後封存

1. 核對文章要求、`rust_ai_chat` 精確 SHA、實際呼叫鏈與本篇必要測試／DB／HTTP／瀏覽器或 GUI 證據；未驗收或失敗項不得標為完成。
2. 將完成文章正文原樣移至 `archived/day-NN/{project}.md`，保留原 commit、blob、全文 SHA256、驗證結果與路徑對照；已在 history 的原文只補完成證據，不重複搬移或改寫。
3. 同步本清單、專案首頁、目前進度、manifest 與歷史索引，再從 GitHub 精確提交讀回核對。必要勘誤另存，保留原發布內容。

[已驗收文章](../archived/README.md) · [既有歷史與證據](../history/README.md) · [已驗證完成切片清單](../history/evidence/2026-10-23-browser-sqlite-9c2b7cf/archive-manifest.json)

第③節依共同規則核對本篇涉及套件及軟體**交付當日官方最新穩定版**，附官方來源、核對日、現有 manifest／lock 或安裝版、相容性與選版理由；未查明不得宣稱最新或通過 Gate 1。最新穩定版優先作為教學目標，`rust_ai_chat` 維持唯讀，不自動升級。

新文章按 `day-NN/{chat,pet,analytics}.md` 存放。每新增一個專案 Day 立即獨立 commit／推送，全文讀回後條件同步狀態與 manifest，再讀回；不能累積多天或多案一起提交。實際下一 Day 取 [learning-state.json](../../learning-state.json)，不依目錄推算。封存、勘誤及驗證更新均不增加 Day，也不改變完整產品驗收的判定。
