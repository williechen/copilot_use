# 可攜式資料庫調整

2026-10-12 使用者變更：Chat／Pet 使用可攜式 SQLite，Analytics 儀表板維持 PostgreSQL。本地測試仍為交付範圍，不公開部署；正式版本及其來源 SHA 以 GitHub Release 與附件證據為準。

Chat 的既有 PostgreSQL migration 原文保留於 `apps/chat/postgres-migrations`，不再由 Chat 啟動執行。SQLite 使用獨立 migration history。舊 Docker `chat-data` volume 不會被刪除或覆寫；新 Chat 使用 `chat-sqlite-data`。Pet 保留既有 SQLite 與備份／檔案 projection。Analytics 的原生 UUID／VARCHAR(40) adapter 與 PostgreSQL migrations 不變。

已實跑 SQLite 訊息／Outbox 原子交易、失敗回滾、併發去重、CAS 租約、Session／ACL、多人 WS、五程序512連線／64筆並行fan-out、同意撤回、備份逐表核對／搬移重啟、PostgreSQL 停機時 Chat 持續寫入／恢復投遞。舊 PostgreSQL 的唯讀匯入已核對 UUID、bytea、timestamp、JSONB、UUID[]，拒絕覆寫；原 DB 不修改。真實 OpenAI embedding／SQLite cosine RAG／串流／MCP／Persona 已實跑；上述功能已在乾淨來源 `22b44f33b9e2c9edb0fb398fb9bff3efd57b00ee` 完成本地／Docker／release GUI 驗收，且同 SHA 六項 CI 全部成功（包括三平台真正簽章更新與冷啟動）；正式發布以 GitHub Release 工作流程及附件為準。

SQLite 精確檢索每位 owner 最多1000份知識文件；WAL 支援同一主機多程序，不支援跨主機網路共享檔案。備份使用 VACUUM INTO，不可單獨複製仍在使用的主 DB 檔。所用 libsqlite3-sys 0.37.0 bundled SQLite 為3.51.3，包含 WAL-reset 修正。設定／搬移／唯讀匯入命令見 DEPLOYMENT.md。

先前 `78006cb` 的 Linux 桌面、Docker、Rust、故障恢復 CI 通過，macOS／Windows GUI 未通過，不能算成完成。macOS 固定250ms等待已改成有上限的實際DOM條件等待；Windows 測試暫存資料夾已明確設置私有 DACL，保留產品對不安全目錄的拒絕。是否通過須讀取正式版本 SHA 的實際 runner 結果。

正式 tag 完成後才建立教學對齊分支，依唯讀 rust_ai_chat 的實際目錄／內容產生對應教學與程式調整；tests 保留在所屬專案／模組，不搬到最外層。正式 tag 固定前不新增 Day。
