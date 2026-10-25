# P0–P12 十三階段本地文章掃描（2026-10-25）

已依使用者更正的十三階段規劃建立逐篇階段分類、重複／過時候選與證據矩陣。此交付為文件掃描，不是產品 Gate 驗收。

## 本地可用性與更新

- 初始本地副本：`532e739f1ef4e5920de300eba0f90adeeaf96187`，取自 `new-chat-5/work/copilot_use`；當時缺少正式 Roadmap。
- 依使用者「請先更新到最新」，已從 GitHub 抓取 `main`，本次凍結於 `4a3c43187e1ffd7cf4ba8d44cd4f8529f0d9d6d4`。後續查核全以此更新後的本地內容為第一來源。
- Roadmap：`docs/ROADMAP_P0_P11.md`，Git blob `fc67de29a53841c2edd0899609611c21d58f7fe3`；SHA256 `55007b1e62a86f56d9a0ee743184d5adb94b251460ca6ec40ee026550d8b78ee`。先前網路讀取版本與更新後本地檔內容會核對。
- 本輪掃描 554 份 docs Markdown，其中 teaching 505 份；重新讀取／UTF-8 解碼／雜湊，未載入舊索引的判定來產生新文章結論。
- 檔案系統發現 157 份文章原文／修訂／鏡像，對應 105 個閱讀版本（Day2–Day36 × 三產品）；類別：{"reading_version": 105, "mirror_or_archive_variant": 45, "earlier_revision": 7}。
- Day1 的 Chat／Pet／Analytics 原文仍缺，不重建。離線庫外 Notion 最新內文、外部測試／CI 連結及壓縮證據內文未全部重新取得。
- 另有本地 rust_ai_ring checkout 可供後續唯讀查核；已在獨立副本同步最新 main 並有限唯讀取證；不編譯、不修改該庫，不宣稱產品驗收。

## 交付

- [當前文件衝突](DOCUMENT_CONFLICTS.md)：本地 curriculum 的 P12／13 階段正確；前版 Roadmap 已更正，daily 尾段過時的 Day35 已同步為37。
- [掃描方法與保存口徑](METHOD.md)。
- [逐篇矩陣](ARTICLE_MATRIX.md)：105 個閱讀版本，加上全部發現的同日修訂與鏡像。
- [正式階段覆蓋／缺口／重複判讀](PHASE_GAPS.md)：P1.0 屬 P1 首工作包；P0 基本授權、去重、重啟不得推延至後期。
- [逐篇完整機器資料](ARTICLE_MATRIX.json)：每篇主題、階段／相關階段、原文與實作行號、風險候選、SHA256／blob、歷史 commit 引用、重複關係與原功能追蹤。
- [全部 Markdown 清單](MARKDOWN_INVENTORY.json)、[完全重複群組](EXACT_DUPLICATES.json)、[數量與界線](SUMMARY.json)。

## 證據界線

本次「重新掃描」指完整檔案機器讀取與雜湊、對每份發現文章擷取大綱與實作證據、閱讀版本逐篇主題對照、全部同日檔逐行相似度比較。這不等於所有程式碼區塊逐行人工審查或功能測試；自動命中舊路徑、501、SSR API 不直接判錯。JSON 已分列文件、實作、CI、驗收狀態；后三者均未於本輪驗證。

舊文章索引只用作最後數量對照與既有追蹤入口。本輪沒有沿用它的「已讀／已通過」欄位。所有原文章、歷史版本、進度／next_day／FIFO 保持原樣；只修改現行規劃／導航／引用欄位與新增報告，歷史文章及進度證據保持。未將新文章規劃當實作排程，也GitHub 發布以合併報告的發布證據為準。

## 十三階段更正與合併

[詳細 Roadmap](../../ROADMAP_P0_P12.md) · [合併來源與界線](../phase-plan-merge-2026-10-25/README.md)。本資料夾名稱為首次交付識別，保留原路徑避免斷鏈；現行內容採 P0–P12。原 Markdown inventory 的 hash 仍指最初 `4a3c431` 掃描快照，不冒充合併後各文件的新 hash。
