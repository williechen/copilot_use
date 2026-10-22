# 主體資料庫與 Adapter 邊界

`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_chat` 是唯一程式、測試、CI 與功能驗收主體。 Chat／Pet 使用隔離 SQLite，Analytics 使用獨立 PostgreSQL；SQLx 查詢、交易及 migration 在主體的持久化 Adapter，Domain／Application 不綁定 driver。

每次依 rust_ai_chat 最新來源核對 migration 位置、schema、ID／事件契約、使用者確認、備份／還原與重啟結果。舊庫 runtime、匯入、RAG 限額或備份命令不能推定主體已存在。執行中的 SQLite 不直接複製單一主檔，實際一致性與安全操作依主體程式及測試確認。

Analytics 歷史學習 DB 沿使用者已確認的刪除重建政策，不把保留資料原地升級當該項結案必要條件；本次不操作任何 DB。其他資料／固定 migration／歷史證據不因文件修訂刪除或重建。

[舊資料操作說明](teaching/history/evidence/2026-10-23-repository-roles/before/docs/SQLITE_TRANSITION.md) · [驗收](LOCAL_VALIDATION.md)
