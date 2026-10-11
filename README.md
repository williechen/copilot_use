# copilot_use — AI 對照程式碼

這是由 AI 控制的獨立對照 Repository。使用者控制的 [rust_ai_chat](https://github.com/williechen/rust_ai_chat) **永遠唯讀**，不在此自動覆寫。基準 SHA: `fdc74c1ee244cf86df3fd1c427d893d2956f6211`。

## Day 9 — 三專案可靠事件 Vertical Slice

- `apps/chat`：獨立 PostgreSQL Chat 訊息 + Transactional Outbox；commit 後呼叫現有應用的 WS broadcast（本對照專案不包含既有完整 WS/Leptos UI）；lease / fencing / retry / worker。
- `apps/pet`：授權根下的一般檔案預覽、明確確認、quarantine 操作 receipt；不代表已通過高安全等級 TOCTOU / GUI E2E。
- `apps/analytics`：獨立 PostgreSQL DB、Bearer 認證、HTTP Ingest、`(source_app,event_id)` 唯一約束、持久冪等、count Query API。
- `crates/event_contract`：跨專案一致、只有四欄且不帶私人內容的事件。

## 執行與測試

需要 Rust stable (edition 2024) 與 PostgreSQL 17 測試環境。兩個資料庫必須獨立建立、使用不同角色。

```sh
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres  # 僅測試 CI，勿用生產 DB
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

export ANALYTICS_DATABASE_URL='postgres://analytics_user:***@127.0.0.1:5432/analytics_db'
export ANALYTICS_API_TOKEN='replace-with-strong-secret'
cargo run -p day9-analytics

export CHAT_DATABASE_URL='postgres://chat_user:***@127.0.0.1:5432/chat_db'
export ANALYTICS_ENDPOINT='http://127.0.0.1:3100/api/v1/events'
cargo run -p day9-chat
```

**安全與未完成界線：** 本 Repository 是 Day9 的可驗證對照切片，**不是**三個最終產品的完整功能集合；未整合 rust_ai_chat 的 Leptos/WebSocket、Tauri 真實 GUI、Pet 的原子 OS 檔案操作、跨平台測試、全量 E2E、追蹤/部署。必須通過 CI 與後續整合才可宣稱 P0 驗收。任何 Copilot/AI 提交均只屬於 copilot_use。

PostgreSQL 原生 UUID 及 VARCHAR(40) 都符合領域契約；本對照實作選 native UUID，SQLx 與 Migration 差異封裝在各自 persistence 層；切換不得重寫 domain/event contract。
