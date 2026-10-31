# Day45 補正採用驗證｜a7a8267f

產品精確來源`a7a8267f8c2f9003dd0d91d9a8cd25daba2517e4`；文件基準`facd132c63ec38c197192f196bb706fd97faf642`。在未套任何patch的原樣產品隔離匯出，使用測試PostgreSQL執行完整Analytics postgres-integration：**68通過、0失敗、1既有legacy ignored，exit0**。[完整log](analytics.log)、[真實命令／環境／限制](review.json)。這是新產品SHA實跑，並非前次候選結果改標。

前次只剩的fixture讀入補正已採用：兩個歷史SQL fixture容許編輯器增加的單一檔尾LF，再以固定歷史SHA256核對真正原始位元。f6七→九份SQLx migration升級、重跑、原checksum與個人deny保留皆通過；已跑28改寫版ledger的DB仍會被拒絕，且不會偷偷改寫_sqlx_migrations。前次fixture補正patch現在已屬歷史，**不要再套用**；本回合不需新patch。

[來源檔blob／SHA256與子樹核對](source.json)：這次只改Analytics測試檔，Chat與Pet子樹未變，沿用明確標成d784的Chat44、PetUI11／DOM六pose結果，不重標為本次實跑。舊patch精確reverse-check因上下文註解不同而不相符，不拿它當未採用判斷；實際helper／call site及完整回歸證實採用。

本案PG Session Adapter與原有效斷言均承接，不新增功能／Refactor計功，不改跨案crate或全庫CI。測試DB已停止、隔離副本及臨時DB已清除，[清理紀錄](cleanup.json)保留，產品庫唯讀。

文件Gate1更新完成；**完整產品Gate2仍false**：1 legacy ignored、真實DB備份調整、Chat／Analytics真瀏覽器旅程及Tauri桌面驗收等範圍未完成。[CI快照](ci.json)有執行中工作，不代表七項全部通過。若真實DB曾跑28改寫版migration，仍需按備份明確處理，不能自動改ledger。

本次是同一Day45採用驗證，next_day三案仍46，FIFO與last_full_comparison不變。9ae3及更早失敗／候選證據全部保留，不抹除歷史。
