# 目前系統架構與 DB 設定

> 本輪參考rust_ai_chat/main `fa0888ea1e1ea93415a4ba7f16972938969b1be3` 全面重測：workspace126通過／1失敗，候選129通過；WS七案、schema既有33＋protocol27案、Pet原生四案通過。main搜尋／SSR／Cookie及舊DB升級仍失敗，完整GUI／帳密等未驗證，完整產品未驗收。詳[證據](teaching/evidence/2026-10-18-full-fa0888e/REPORT.md)。下方舊SHA屬歷史，不自動代表copilot_use產品通過；P0 table／field可依既有功能需求增減或修改；原產品需求未增加。

> copilot_use P0 試作（本地工作分支codex/p0-table-trial，948685fe0692a9e56de94a4ae6a817fdf13fc0a2）：Chat註冊必填唯一Email、帳號不分英文大小寫、成功註冊撤銷目前Session，已接通UI／HTTP／SQLite／獨立完成頁及重啟保存。相關測試21、HTTP／schema16、升級2案、fmt／嚴格Clippy／前端建置通過。既有帳號Email補填、寄信重設、admin初始化、活動Session與其他Pet／Analytics P0仍待完成；完整P0未驗收。詳[試作證據](teaching/evidence/2026-10-18-copilot-p0-registration/REPORT.md)。

目錄／程式銜接以 rust_ai_chat 最新 main 為主；產品目標與前進項目以 copilot_use 為主，其餘按共同學習規則。以下是 copilot_use 當前程式架構；gRPC／SSE 新需求的驗證範圍見 [ANALYTICS_TRANSPORT.md](ANALYTICS_TRANSPORT.md)；兩個 repository 的 DB 目標統一，學習套用狀態另行核對。

| 元件 | 目錄與框架 | 職責／資料流程 | 資料／設定 |
|---|---|---|---|
| Chat Web/API | apps/server；Axum 0.8.9／Leptos 0.8.22 | HTTP／Session／ACL／WS／SSE；呼叫訊息 Port | CHAT_DATABASE_URL，預設 sqlite://data/chat.sqlite3；loopback 3001 |
| Chat UI | crates/ui、apps/server/frontend；SSR／WASM hydration | protected routes、HTTP／WS／reconnect／queue／串流畫面 | 不直連 DB |
| Chat Domain/Application | crates/chat_domain、chat_application | 輸入不變量、MessageRepository、durable commit | 無 DB／HTTP／GUI Driver |
| Chat Persistence | crates/persistence；SQLx 0.9.0 | SQLite 訊息／Outbox／topic／realtime 單一交易、租約／備份 | 私有 SQLite 檔／WAL；migrations 所屬 Adapter |
| Pet Native/UI | apps/desktop_pet、frontend；Tauri 2.12.3／Leptos 0.8.22 | GUI／200×200、IPC／確認、更新與冷啟動 | 本機桌面程式，不是獨立 HTTP 服務 |
| Pet Domain | crates/pet_domain | Profile、save_profile Port、Schedule 複習規則 | 無 DB／HTTP／OS／Tauri Driver |
| Pet Runtime | crates/pet_runtime，package day9-pet | SQLite／TOML／素材、AI／MCP、平台 FS、安全 receipt、Outbox／備份 | app_data 或 PET_DATA_DIR；獨立 SQLite 與每寵物檔案 |
| Analytics Web/API | apps/stats_server；Axum 0.8.9 | HTTP Dashboard／Query、gRPC Ingest、SSE 同服務；來源認證、冪等、明細、審核與回退收據 | ANALYTICS_DATABASE_URL；獨立 PostgreSQL；loopback 3100 |
| Analytics Application/Persistence | stats_server application／persistence 模組 | EventRepository → PostgreSQL transaction／dedup → gRPC ACK／HTTP 2xx | non-superuser role；migrations／權限獨立 |
| Shared integration | crates/shared | 四欄低敏 ProductEvent；Chat/Pet Outbox → authenticated Analytics | 不共享業務 DB，不含本文／路徑／憑證 |
| Analytics Transport | crates/analytics_transport；Tonic 0.14.6／Prost 0.14.4 | `.proto` 生成 gRPC client/server；Outbox 投遞、逐事件 ACK、deadline | 同一 Analytics 3100 port；HTTP JSON 保留相容入口 |
| AI／MCP Adapter | crates/ai_core、mcp_bridge | OpenAI Responses／embedding／生圖、有限 Agent 與 owner-scoped readonly tools | OpenAI／gpt-5.6-luna；OPENAI_API_KEY 留在本機 |
| Telemetry | crates/telemetry；OpenTelemetry 0.33.1 | traces／metrics／logs、W3C correlation；與產品事件分離 | 可選 numeric-loopback OTLP，剝除認證 headers |
| 建置／驗收 | Cargo workspace、獨立 frontend/native locks、原生環境／CI（Docker／Compose 僅歷史） | Rust／WASM／GUI／跨服務／三平台 installer/updater | 本地測試範圍，正式 tag／附件固定 |

本表為產品架構及當時觀察，不是最新學習倉庫的自動驗收結果；逐功能分別核對產品實作狀態、rust_ai_chat 學習套用狀態、對應 source／CI SHA 與正式驗收狀態。既有「已驗證」只代表其原始證據範圍，不因新 SHA 自動失效，也不能延伸宣稱新版本已驗證。

## 已確認需求的架構責任（尚未實作／驗收）

- Chat：HTTP／UI 區分房名登入回房及登入／管理回個人頁；伺服器負責帳號唯一性、房間權限、登入等待、活動 Session、密碼生命週期與撤銷。不得只以前台倒數或路由守衛代替。
- Pet：控制窗以寵物識別管理獨立狀態、持久設定及每隻生活窗；runtime 負責出入籠生命週期、依種類／個性的活動與互動、漫遊拖曳及多螢幕政策。既有單 pet 來源觀察保留歷史身分。
- Analytics：人員帳密／Session／角色及來源資料範圍與服務配對憑證分開；伺服器對頁面、API、即時連線執行授權及立即撤銷，持久化登入等待與不含秘密的管理稽核。

完整條件引用 [共同規則](COMMON_LEARNING_RULES.md)及 [UI 需求](UI_ALIGNMENT_REQUIREMENTS.md)。此節為架構需求，不改寫上表既有驗證範圍。

## DB 目標與實際套用

| 產品 | 兩個 repository 的統一目標 | copilot_use 實際 | rust_ai_chat 唯讀核對 |
|---|---|---|---|
| Chat | 可攜式 SQLite | SQLite URI／schema／WAL／backup，已驗證 | 本輪唯讀 source SHA `16261e0ebeef1a888997cc0ebc42b08ceaf8db24` 已使用 SqlitePool／SqliteRoomRepository／SqliteMessageRepository；schema／ID／Outbox 仍與成品不同，source 觀察不等於 DB 執行驗收 |
| Pet | 可攜式 SQLite | PetStore SQLite／獨立檔案／snapshot，已驗證 | 本輪 source 為 Profile／FSM／唯讀檔案預覽，尚無 PetStore SQLite 等同實作；教學依統一目標及當日差異銜接 |
| Analytics | PostgreSQL | 獨立 PostgreSQL／migration／來源角色，已驗證 | `a57e532` 已接上來源認證 → HTTP → PostgreSQL，實際寫入／冪等／衝突／故障測試通過；gRPC／SSE 尚未銜接，不等於產品驗收 |

本輪逐模組、介面與資料流程靜態核對已完成，範圍、差異與可攜式證據見 [完整核對報告](FULL_SOURCE_REVIEW.md)。產品待補項目與安全修正見 [未來工作](FUTURE_WORK.md)；尚未修正或重新驗收，不宣稱兩庫語意等同。

若使用者本地設定尚未推送，GitHub 唯讀核對不代表該本機狀態。rust_ai_chat 不由助手寫入。

## 學習與重構並行

每日依 rust_ai_chat 現有架構、確認缺口與當日差異產生四節教學，原文保留，舊 Day 更正於下一個 Day 的 Refactor 後獨立補加，不回填舊 Day；不預排 Day，不設獨立重構章。週六工程按 Red → 最小 Green → Refactor 完成必要整合、功能測試與提交，證據獨立記錄，不納入教學，也不驗證 AI 項目；功能 Gate 通過即為功能通過，不為文字不同重寫已驗證功能。

Chat／Pet 各自持久化訊息或真實操作結果及 Outbox → gRPC／Protobuf → 來源認證 Analytics → PostgreSQL 去重／查詢／洞察 → 人工審核 → Chat／Pet 確認採用／收據／可回退。

Dashboard 以 HTTP 查詢最新聚合；SSE `/api/v1/events/stream` 通知資料版本改變。每次連線／重連都觸發重新查詢，PostgreSQL 為唯一資料來源。

本輪參考庫新增的chat_action_log尚無runtime writer，NULL主鍵及任意操作者可插入；候選僅修主鍵，完整身分／稽核仍待接線。P0資料模型按功能調整，並以實際操作、授權、保存及升級驗證。
