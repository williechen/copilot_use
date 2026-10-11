# P9 最終三產品實作目標（獨立於教學進度）

## 最高優先權的產品定位
- 本 Repo 是 AI 控制的最終產品程式碼；**直接以 P9 為交付目標**。允許依相依性拆成多次 Commit/測試，但不按 rust_ai_chat 的 Day 編號或教學 P0–P5 進度等待。
- rust_ai_chat 僅作使用者控制的唯讀學習/原始需求參考，兩套代碼與 CI、Gate、進度各自驗收。
- 現在已有的 Day9 Chat/Pet/Analytics 範例只是 P9 起始實作，不能代表 P9 或完整產品完成。
- 以下是依既有需求整理的完整能力範圍；**P9 各正式 Gate 名稱與驗收門檻尚須從原始需求再核對**，本文件不捏造已完成狀態。

## P9 必備能力矩陣

| 系統 | P9 最終功能 | 現況 | 缺口 |
|---|---|---|---|
| Chat | Axum+Leptos Router、Nested/Protected Route、SSR/Hydration | 未完成 | UI/Route/Session |
| Chat | 多主題房間、多使用者、私人聊天室、AI Personas、持久訊息/History | 部分 | Session/房間權限/History |
| Chat | WS Presence/Typing/重連、AI streaming、可靠 Outbox | 部分 | 真實 WS + E2E |
| Chat | Structured Output、Tool calling、RAG/pgvector、MCP、Agent Runtime | 未完成 | 端到端 AI 核心 |
| Chat | 全鏈路 OTel/追蹤、安全、部署與回滾 | 未完成 | 觀測/維運 |
| Pet | Tauri+Leptos 桌面 GUI、多寵物、五項 Profile、進階覆寫 | 未完成 | GUI/設定儲存 |
| Pet | AI 生圖/角色一致性/動作與動畫、200×200 顯示 | 未完成 | Asset pipeline/Renderer |
| Pet | 學習/記憶/複習、AI/RAG/MCP/Agent | 未完成 | AI 功能 |
| Pet | 授權 FileOrganizer 預覽/明確確認/OS 安全操作/可信 receipt | 部分 | 安全 TOCTOU 防護/GUI |
| Analytics | 單一服務 Dashboard + 認證 Ingest/Query API | 部分 | Dashboard/完整 Query |
| Analytics | 獨立 PostgreSQL、持久冪等、重啟/併發、來源事件目錄 | 部分 | 全事件/E2E |
| Analytics | 查詢聚合、視覺化、1~3句可追溯 AI 洞察/可審核建議 | 未完成 | 指標/洞察/審核 |
| Analytics | 存取控制、留存/刪除、備份還原、OTel 與正式部署 | 未完成 | 資料治理/維運 |
| Cross | Chat/Pet 各自資料庫獨立、驗證跨系統事件、冪等、故障重試 | 部分 | 集成 E2E |
| Cross | Observe→Analyze→Act→Evaluate→Observe 閉環，建議需 Chat/Pet 授權採用並回滾 | 未完成 | 閉環驗收 |

## 開發與驗收政策
1. 優先補齊**完整產品可啟動的實際入口**，不是只開新 trait 或只留 CLI；每次提交以可測試的真實跨層功能為單位。
2. Chat/Pet/Analytics 的 Domain、Application、Persistence/Platform Adapter、UI 與 Integration Contract 必須各自維持邊界。
3. PostgreSQL 原生 UUID 或 VARCHAR(40) 均允許；Domain/Application 不依賴儲存型別，SQL/SQLx/Migration 限定在 Adapter 並做雙模式測試。
4. 對寫入敏感操作使用安全預設（確認、權限、路徑重新驗證、不可跟隨 symlink、原子性），未達安全證據不得啟用正式清理。
5. 驗證須提供 Rust fmt、clippy、unit/DB/HTTP/WS/GUI/E2E、跨平台、部署/回滾、資安、監控的真實結果。缺證據標 Blocked；不可把 scaffold/README/CI 配置當測試通過。
6. 對每個功能記錄規格→檔案/提交→測試/CI→未完成事項，持續前進直至全部 P9 要求被驗證。

**目標是 P9 完整三產品，不是照著 Notion 教學 Day N 一天寫一小節。**
