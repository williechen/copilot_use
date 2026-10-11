# 三專案對照實作：隔離部署與驗證

本倉庫為 **AI 控制的 copilot_use**。使用者持有的 rust_ai_chat 僅供唯讀比較。

## PostgreSQL
在**非正式環境**建立分開的 `chat_db`、`analytics_db` 與獨立帳號。不得對現有業務資料庫執行 migrations。兩個 app 啟動時各自執行 `sqlx::migrate!`，不共用資料表。

```sh
export CHAT_DATABASE_URL='postgres://chat_dev:secret@127.0.0.1:5432/chat_db'
export ANALYTICS_DATABASE_URL='postgres://analytics_dev:secret@127.0.0.1:5432/analytics_db'
export ANALYTICS_API_TOKEN='replace-with-independent-query-token'
export ANALYTICS_CHAT_TOKEN='replace-with-independent-chat-ingest-token'
export ANALYTICS_PET_TOKEN='replace-with-independent-pet-ingest-token'
export CHAT_API_TOKEN='replace-with-independent-chat-api-token'
export ANALYTICS_ENDPOINT='http://127.0.0.1:3100/api/v1/events'
cargo run -p day9-analytics
# 新終端
cargo run -p day9-chat
# 建立房間後送出訊息；事件將由 Outbox 發送至 Analytics
curl -X POST http://127.0.0.1:3001/api/v1/rooms -H "Authorization: Bearer $CHAT_API_TOKEN" -H 'Content-Type: application/json' -d '{"id":"room-1","name":"Rust"}'
curl -X POST http://127.0.0.1:3001/api/v1/messages -H "Authorization: Bearer $CHAT_API_TOKEN" -H 'Content-Type: application/json' -d '{"room_id":"room-1","content":"你好"}'
curl -H "Authorization: Bearer $ANALYTICS_API_TOKEN" http://127.0.0.1:3100/api/v1/events/count
```

Pet 目前是唯讀 CLI，不能將其視為 Tauri 桌面應用已完成：
```sh
PET_AUTHORIZED_ROOT=/tmp/test-pet cargo run -p day9-pet -- example.txt
```

## 靜態與整合驗證
```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
# SQLx 測試需提供有 CREATE DATABASE 權限的隔離 DATABASE_URL
DATABASE_URL='postgres://test:secret@127.0.0.1:5432/postgres' cargo test --workspace
```

## 未完成/不得宣稱驗收
- Chat 尚未承接 rust_ai_chat 原有 Leptos、完整 WS 產品協定、多使用者、Session、RAG/Agent。
- Pet 尚未有安全的 OS 原子檔案操作、Tauri GUI、動畫及 AI 完整功能；不對敵對目錄做正式隔離。
- Analytics 尚未提供完整 Dashboard、聚合、保留/刪除、備份還原、來源金鑰輪替。
- 雙 Repository 的 CI/Commit 不得互相代替；沒有實際執行的 E2E 一律標 Blocked。

最新本地測試與未完成項目見 [LOCAL_VALIDATION.md](LOCAL_VALIDATION.md)。
