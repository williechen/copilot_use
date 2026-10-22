# GitHub 教學交付與 Notion 搬遷完成

- 唯一教學交付：williechen/copilot_use，teaching/rust-ai-chat-alignment。
- 原文搬遷 commit：`fb50aaf16761e45733b88a8e81961dc495f0930e`；86 頁（共同主頁、三案入口、所有遞迴教學與封存子頁）。
- 187 個變更檔案以精確 GitHub commit SHA 全文讀回相等；86 頁正文另以原始 fetch 內容比對並記錄 SHA256、blob SHA，程式碼未改寫。[證據](readback.json)。
- 讀回證據 commit：`3195ab2c518481327216d432646a5fedee21aeb4`，證據本身亦全文讀回通過後才開始清理。
- 使用者確認後移到可還原 Notion 垃圾桶；86/86 頁再讀回均有 deleted 標記。[清理紀錄](cleanup.json)。未永久刪除，未變更其他 Notion 頁。
- 規則、learning-state 必要 policy、current manifest、三案入口、README、TEACHING_FORMAT、FUTURE_WORK 已同步；新文章目錄 docs/teaching/daily/day-NN/{project}.md。
- 每新增一個專案 Day 就獨立 commit／推送，不累積多天或多案。精確 commit 全文讀回及證據保存後，才以最新 SHA 條件同步該案進度與 manifest，再讀回；狀態可另 commit 並引用文章 commit。
- next_day 保持 Chat27／Pet27／Analytics27；makeup_queue、last_full_comparison 與既有 Day25 以前 GitHub 封存正文完全不變。此次沒有新 Day、程式套用或功能驗收。
- 既有 06:00 Asia/Taipei 任務提示詞已原地更新、重新載入伺服器後全文核對一致，維持 ACTIVE，未新增排程。[排程證據](scheduler.json)。
- rust_ai_chat 全庫未寫入；此為文件與原文搬遷驗證，未重跑產品測試。
