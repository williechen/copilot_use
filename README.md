# copilot_use — AI 對照實作

**權責：** [rust_ai_chat](https://github.com/williechen/rust_ai_chat) 是使用者控制的教學程式碼，AI 完全唯讀；本 Repository `copilot_use` 是 AI 控制的實作／驗證區，兩者的 SHA、CI 與 Gate 必須獨立核算。

唯讀參考基準：`rust_ai_chat` main `fdc74c1ee244cf86df3fd1c427d893d2956f6211`（2026-10-11 查核時）。

## 已寫入的三專案 Day 9 最小可靠事件切片

| Crate | 功能 | 目前限制 |
| --- | --- | --- |
| `apps/chat` | Room 建立、訊息入 DB、同交易 Outbox、租約/重試/ACK、HTTP 寫入及 Dispatcher | 尚未移植完整 Leptos、WebSocket、AI Persona、Session 等產品功能 |
| `apps/pet` | 授權根目錄一般檔案預覽、明確確認的隔離操作、成功 Receipt、唯讀 CLI | 隔離操作尚未具備敵對檔案系統的原子 TOCTOU 防護；未移植 Tauri GUI／AI 動畫 |
| `apps/analytics` | 獨立 PostgreSQL、Bearer 認證 Ingest、持久冪等、Count Query API | 尚缺正式 Dashboard、細部分析、備份/保留與使用者授權 |
| `crates/event_contract` | 四欄、版本化、無個資的跨系統事件 | Schema v1 僅涵蓋 Chat/Pet 最小事件 |

**本工作區尚不能被稱為三個最終完整產品，也不等於 P0 驗收完成。** 後續功能仍須逐項完成、測試及 CI 核驗。

## 啟動、API 與隔離測試

請參閱 [部署與測試操作說明](docs/DEPLOYMENT.md)。

```bash
# 需 Rust stable 與本機 PostgreSQL，請在隔離測試庫執行
export DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/postgres'
cargo test --workspace --all-targets
```

Chat 與 Analytics 需要各自獨立的 PostgreSQL 資料庫和使用者。啟動時僅對這兩套「AI 對照環境」執行各自的 migrations，不得觸及 rust_ai_chat 的既有 DB 或 migration。

## UUID 與 Adapter

跨系統契約以 Rust `Uuid` 表達 ID；本對照資料庫的實體欄位暫採 PostgreSQL 原生 `UUID`。改為 `VARCHAR(40)` 時，應將 SQL、SQLx bind/decode 與資料遷移限制在 Persistence Adapter；不改 Event Contract、Domain 與對外 JSON。雙模式的完整對照測試仍屬待完成項目。

## GitHub CI

`.github/workflows/verify.yml` 啟動 `postgres:17` 並執行 Workspace 測試。建立 Workflow 不代表已通過；請以 [Actions](https://github.com/williechen/copilot_use/actions) 真實結果為準。

本倉庫目前以三專案 **P0 Day 9 Vertical Slice** 為起點，不以尚未實作的功能假裝最終交付。
