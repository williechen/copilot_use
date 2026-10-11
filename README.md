# copilot_use — P9 最終產品實作（AI 控制）

> **正式目標：完成可部署的 Rust AI Chat、Desktop Pet、Product Analytics 三個完整產品，而非 Day 9 教學示範。**
> `rust_ai_chat` 是使用者控制、AI 永遠唯讀的教學 Repository；本 Repo 由 AI 獨立朝 P9 開發，不等待教學 Day 或 P0–P5 進度。
> **目前狀態：P9 尚未完成。** 已存在的 Chat Outbox／Pet Quarantine／Analytics Ingest 只是早期切片，尚缺完整 UI、AI 能力、安全驗證、完整部署與跨產品 E2E。不可把早期切片宣稱為可交付最終產品。

## P9 三產品交付範圍

- **Chat**：Axum/Leptos SSR+Hydration、Router、多使用者/主題/AI Personas、Session 與授權、WebSocket presence/typing/private/reconnect、history、Transactional Outbox、Streaming、Structured Output、Function Calling、RAG/pgvector、MCP、Agent Runtime、OTel、部署/測試。
- **Desktop Pet**：Tauri/Leptos 桌面 GUI、五項設定與可選進階設定、多寵物、AI 角色/動畫生成、200×200 畫框、學習/記憶/複習/RAG/MCP/Agent、安全 FileOrganizer（授權、明確確認、TOCTOU 防護、Receipt）、跨平台打包與測試。
- **Product Analytics**：獨立資料庫，同一個服務內的 Dashboard+Ingest/Query API、來源認證、持久冪等、聚合報表、事件最小化、資料保留/刪除/備份、可追溯 AI 建議及 OTel、部署/測試。
- **整合驗收**：三產品可各自執行/部署、隔離資料庫，跨系統事件驗證、可靠重試、E2E、CI/CD、故障恢復；所有規格和測試皆有可查證證據後才可稱 P9 完成。

## 2026-10-11｜已提交的 P9 產品程式增量（非最終驗收）

- Chat 已提供 Bearer-authenticated Room/Message REST API、持久 history 查詢和瀏覽器聊天畫面，包含 Message+Outbox 同交易及 Dispatcher。現在的 UI 是 polling，**尚未實作真正的 WebSocket/Leptos 完整產品**。
- Pet 已增加五項繁體中文 pet.toml 設定解析、可選外觀/動畫覆寫、多寵物目錄隔離、200×200 Display Manifest、可操作 CLI；**Tauri/Leptos GUI、AI 圖像生成及安全 OS 原子操作仍未完成**。
- Analytics 已增加 Dashboard 和依 source_app/event_type 聚合的查詢 API，與 ingestion 共用同一 Axum 服務。**完整權限／留存／AI 建議仍未完成**。
- Dockerfile、compose.yaml、PostgreSQL 雙資料庫及 scripts/smoke.sh 已提交；GitHub Actions 含 Rust integration 與 Compose Chat→Outbox→Analytics E2E。**CI/E2E 未核實前不得稱測試成功；P9 仍未完成**。
- 可用命令：先在隔離環境複製 `.env.example` 為 `.env` 並設定隨機密碼／Token，再執行 `docker compose --env-file .env up --build -d`。Chat 位於 http://127.0.0.1:3001、Analytics Dashboard 位於 http://127.0.0.1:3100；僅限本機開發環境。真正公開部署還需要 TLS、正式 Session/Auth、資安稽核及備份還原。

## 開發軌道

本 Repo 可直接實作最終功能，不受 `rust_ai_chat` 教學 Day 制約。每次更新必須說明具體新增的產品功能、仍缺的功能、真正的測試/CI 及部署證據。**工作中狀態不能與完成驗收混淆。**

---

## 舊有 Day 9 切片（歷史基準，不代表 P9 目標）

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
