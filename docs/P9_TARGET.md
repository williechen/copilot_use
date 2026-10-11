# P9 最終三產品實作目標（獨立於教學進度）

## 最高優先權的產品定位
- 本 Repo 是 AI 控制的最終產品程式碼；**直接以 P9 為交付目標**。允許依相依性拆成多次 Commit/測試，但不按 rust_ai_chat 的 Day 編號或教學 P0–P5 進度等待。
- rust_ai_chat 僅作使用者控制的唯讀學習/原始需求參考，兩套代碼與 CI、Gate、進度各自驗收。
- 現在已有的 Day9 Chat/Pet/Analytics 範例只是 P9 起始實作，不能代表 P9 或完整產品完成。
- 以下是依既有需求整理的完整能力範圍；**Gate 以原始需求及實際證據逐項核對**，本文件不捏造已完成狀態。

## P9 必備能力

- Chat：Axum/Leptos SSR、Hydration、Nested/Protected Router、Session/ACL、多房間與使用者、history、WS presence/typing/private/reconnect/send queue、交易 Outbox、多 Persona/AI-to-AI、Streaming/Structured Output/tools、RAG/pgvector/MCP/Agent、OTel。
- Desktop Pet：Tauri/Leptos、繁體中文五項設定與進階覆寫、多寵物、角色一致性生圖/動畫與固定 200×200 顯示、記憶/學習/active recall/spaced repetition/adaptation/proactivity、RAG/MCP/Agent、安全 FileOrganizer 的授權/預覽/確認/TOCTOU/receipt/Trash/restore、跨平台 installer/updater。
- Analytics：獨立 DB、同服務 Dashboard/Ingest/Query、來源認證、冪等、完整來源事件、Taipei 趨勢、資料治理/備份、可追溯 1–3 句 AI 洞察及可審核建議、OTel。
- 整合：獨立執行、隔離資料庫、跨服務事件/重試/故障恢復，Observe→Analyze→Act→Evaluate→Observe 閉環；Chat/Pet 採用必須授權且能回退。

當前狀態、已測證據與逐項缺口統一維護於 [LOCAL_VALIDATION.md](LOCAL_VALIDATION.md)，避免多份矩陣互相矛盾。使用者已取消正式部署，保留本地/Docker/CI/跨平台測試與固定成品目標。

## 開發與驗收政策
1. 優先補齊**完整產品可啟動的實際入口**，不是只開新 trait 或只留 CLI；每次提交以可測試的真實跨層功能為單位。
2. Chat/Pet/Analytics 的 Domain、Application、Persistence/Platform Adapter、UI 與 Integration Contract 必須各自維持邊界。
3. PostgreSQL 原生 UUID 或 VARCHAR(40) 均允許；Domain/Application 不依賴儲存型別，SQL/SQLx/Migration 限定在 Adapter 並做雙模式測試。
4. 對寫入敏感操作使用安全預設（確認、權限、路徑重新驗證、不可跟隨 symlink、原子性），未達安全證據不得啟用正式清理。
5. 驗證須提供 Rust fmt、clippy、unit/DB/HTTP/WS/GUI/E2E、跨平台、部署/回滾、資安、監控的真實結果。缺證據標 Blocked；不可把 scaffold/README/CI 配置當測試通過。
6. 對每個功能記錄規格→檔案/提交→測試/CI→未完成事項，持續前進直至全部 P9 要求被驗證。

**目標是 P9 完整三產品，不是照著 Notion 教學 Day N 一天寫一小節。**
