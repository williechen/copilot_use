# 可攜式資料與 Adapter

Chat／Pet 使用可攜式 SQLite；Analytics 使用獨立 PostgreSQL。Domain/Application 不綁定 Driver。SQLite migrations 在各自 Persistence／runtime，Analytics migrations 在 stats_server。舊 Chat PG migration 與 import 只作唯讀移轉，不套用到 SQLite；原 DB／volume 不覆寫或刪除。

SQLite RAG 每個 owner 最多 1000 份文件；同主機多程序使用 WAL，不支援跨主機網路檔案系統。執行中的備份用 VACUUM INTO，不單獨複製主 DB。Pet 備份包含 SQLite、設定與素材。具體設定、備份／還原／搬移／唯讀匯入命令見 [DEPLOYMENT.md](DEPLOYMENT.md)，必要回歸見 [LOCAL_VALIDATION.md](LOCAL_VALIDATION.md)。
