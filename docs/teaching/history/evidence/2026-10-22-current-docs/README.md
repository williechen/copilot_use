# 現行操作、產品目標與架構文件核對

核對日期：2026-10-22；copilot_use/main 來源 `06113718d79e26307dd06ad3a7c45be7fb4f50ad`。

本次修正本地操作、產品目標／開發清單、架構及程式銜接，並更新兩個文件入口。改寫前的每一份文件原文放在 `before/`，與 [來源 commit、原路徑及 SHA256 manifest](manifest.json) 集中保存。既有 Notion 匯出、Day 歷史正文與工程證據沒有改寫或刪除。

核對內容：服務管理器與建置／Pet 安裝腳本、Cargo workspace、Chat／Pet／Analytics 入口、純核心 Port 與持久化／傳輸 Adapter。完整兩庫比較報告仍沿用原來源，未將本次產品文件核對算為新的完整比較。

驗證項目與結果見 [verification.json](verification.json)。本次只有靜態及 CLI／語法檢查，沒有 build、服務啟動、資料庫、GUI、付費 AI 或跨平台測試；未推進產品完成狀態、learning-state 或 teaching manifest。
