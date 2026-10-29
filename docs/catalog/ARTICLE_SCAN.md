# 全文章掃描與儲存庫首頁修正

掃描來源：main `5a8516b40176b0d88e71a74a05251d3e4ddecf2f`。舊教學分支已由外部合併進 main 並刪除，本輪不重建。

逐一完整讀取 436 個已追蹤 Markdown 檔，檢查現行／歷史用途、相對連結、已刪分支引用與合併衝突標記；逐檔 SHA256、發現與處理見 [完整清單](ARTICLE_SCAN.json)。這是文件掃描，不是功能或安全程式碼驗收。

- 儲存庫根 README 改為現行專案介紹、共用文章、三個首頁及進度入口；移除舊 Day／版本更新流水帳。
- 修正現行首頁與發布政策到 main；歷史原文及固定 SHA 報告內的舊分支名稱屬當時來源，不改寫。
- 過期測試快照、Pet 舊診斷及發行狀態移到 [歷史來源註記](../teaching/history/evidence/2026-10-22-article-scan/source-notes.md)，現行文章改指統一進度或即時 Releases。
- 原始封存正文及既有證據不修改；next_day、makeup_queue、last_full_comparison 不變。
- 原06:00任務沿用原ID，只更新已刪分支及歷史路徑引用；不新增任務。

## 封存完整性修復

合併時三篇 Day13 Notion 原文與既存 SHA256 不符：Chat、Pet、Analytics 各一篇。合併後版本另存同目錄 merge-5a8516b-*.md，原檔從先前已驗證 commit 7fddb89 還原，兩份均保留；修復後原始證據雜湊全數一致。
