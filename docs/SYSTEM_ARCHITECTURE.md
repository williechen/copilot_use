# 目前系統架構與 DB 設定

目錄／程式銜接以 rust_ai_chat 最新 main 為主；產品目標與前進項目以 copilot_use 為主，其餘按共同學習規則。以下是 copilot_use 當前程式架構；gRPC／SSE 新需求的驗證範圍見 [ANALYTICS_TRANSPORT.md](ANALYTICS_TRANSPORT.md)；兩個 repository 的 DB 目標統一，學習套用狀態另行核對。

| 元件 | 目錄與框架 | 職責／資料流程 | 資料／設定 |
|---|---|---|---|
| Chat Web/API | apps/server；Axum 0.8.9／Leptos 0.8.22 | HTTP／Session／ACL／WS／SSE；呼叫訊息 Port | CHAT_DATABASE_URL，預設 sqlite://data/chat.sqlite3；loopback 3001 |
| Chat UI | crates/ui、apps/server/frontend；SSR／WASM hydration | protected routes、HTTP／WS／reconnect／queue／串流畫面 | 不直連 DB |
| Chat Domain/Application | crates/chat_domain、chat_application | 輸入不變量、MessageRepository、durable commit | 無 DB／HTTP／GUI Driver |
| Chat Persistence | crates/persistence；SQLx 0.9.0 | SQLite 訊息／Outbox／topic／realtime 單一交易、租約／備份 | 私有 SQLite 檔／WAL；migrations 所屬 Adapter |
| Pet Native/UI | apps/desktop_pet、frontend；Tauri 2.12.4／Leptos 0.8.22 | GUI／200×200、IPC／確認、更新與冷啟動 | 本機桌面程式，不是獨立 HTTP 服務 |
| Pet Domain | crates/pet_domain | Profile、save_profile Port、Schedule 複習規則 | 無 DB／HTTP／OS／Tauri Driver |
| Pet Runtime | crates/pet_runtime，package day9-pet | SQLite／TOML／素材、AI／MCP、平台 FS、安全 receipt、Outbox／備份 | app_data 或 PET_DATA_DIR；獨立 SQLite 與每寵物檔案 |
| Analytics Web/API | apps/stats_server；Axum 0.8.9 | HTTP Dashboard／Query、gRPC Ingest、SSE 同服務；來源認證、冪等、明細、審核與回退收據 | ANALYTICS_DATABASE_URL；獨立 PostgreSQL；loopback 3100 |
| Analytics Application/Persistence | stats_server application／persistence 模組 | EventRepository → PostgreSQL transaction／dedup → gRPC ACK／HTTP 2xx | non-superuser role；migrations／權限獨立 |
| Shared integration | crates/shared | 四欄低敏 ProductEvent；Chat/Pet Outbox → authenticated Analytics | 不共享業務 DB，不含本文／路徑／憑證 |
| Analytics Transport | crates/analytics_transport；Tonic 0.14.6／Prost 0.14.4 | `.proto` 生成 gRPC client/server；Outbox 投遞、逐事件 ACK、deadline | 同一 Analytics 3100 port；HTTP JSON 保留相容入口 |
| AI／MCP Adapter | crates/ai_core、mcp_bridge | OpenAI Responses／embedding／生圖、有限 Agent 與 owner-scoped readonly tools | OpenAI／gpt-5.6-luna；OPENAI_API_KEY 留在本機 |
| Telemetry | crates/telemetry；OpenTelemetry 0.33.1 | traces／metrics／logs、W3C correlation；與產品事件分離 | 可選 numeric-loopback OTLP，剝除認證 headers |
| 建置／驗收 | Cargo workspace、獨立 frontend/native locks、Docker／Compose／CI | Rust／WASM／GUI／跨服務／三平台 installer/updater | 本地測試範圍，正式 tag／附件固定 |

本表為產品架構及當時觀察，不是最新學習倉庫的自動驗收結果；逐功能分別核對產品實作狀態、rust_ai_chat 學習套用狀態、對應 source／CI SHA 與正式驗收狀態。既有「已驗證」只代表其原始證據範圍，不因新 SHA 自動失效，也不能延伸宣稱新版本已驗證。

## DB 目標與實際套用

| 產品 | 兩個 repository 的統一目標 | copilot_use 實際 | rust_ai_chat 唯讀核對 |
|---|---|---|---|
| Chat | 可攜式 SQLite | SQLite URI／schema／WAL／backup，已驗證 | 本輪唯讀 source SHA `16261e0ebeef1a888997cc0ebc42b08ceaf8db24` 已使用 SqlitePool／SqliteRoomRepository／SqliteMessageRepository；schema／ID／Outbox 仍與成品不同，source 觀察不等於 DB 執行驗收 |
| Pet | 可攜式 SQLite | PetStore SQLite／獨立檔案／snapshot，已驗證 | 本輪 source 為 Profile／FSM／唯讀檔案預覽，尚無 PetStore SQLite 等同實作；教學依統一目標及當日差異銜接 |
| Analytics | PostgreSQL | 獨立 PostgreSQL／migration／來源角色，已驗證 | `a57e532` 已接上來源認證 → HTTP → PostgreSQL，實際寫入／冪等／衝突／故障測試通過；gRPC／SSE 尚未銜接，不等於產品驗收 |

本輪逐模組、介面與資料流程靜態核對已完成，範圍、差異與可攜式證據見 [完整核對報告](FULL_SOURCE_REVIEW.md)。產品待補項目與安全修正見 [未來工作](FUTURE_WORK.md)；尚未修正或重新驗收，不宣稱兩庫語意等同。

若使用者本地設定尚未推送，GitHub 唯讀核對不代表該本機狀態。rust_ai_chat 不由助手寫入。

## 學習與重構並行

每日依 rust_ai_chat 現有架構、確認缺口與當日差異產生四節教學，原文保留，必要實作補在第 2 節末尾；不預排 Day，不設獨立重構章。週六工程按 Red → 最小 Green → Refactor 完成必要整合、功能測試與提交，證據獨立記錄，不納入教學，也不驗證 AI 項目；功能 Gate 通過即為功能通過，不為文字不同重寫已驗證功能。

Chat／Pet 各自持久化訊息或真實操作結果及 Outbox → gRPC／Protobuf → 來源認證 Analytics → PostgreSQL 去重／查詢／洞察 → 人工審核 → Chat／Pet 確認採用／收據／可回退。

Dashboard 以 HTTP 查詢最新聚合；SSE `/api/v1/events/stream` 通知資料版本改變。每次連線／重連都觸發重新查詢，PostgreSQL 為唯一資料來源。
