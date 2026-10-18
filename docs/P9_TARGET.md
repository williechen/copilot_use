# P9 最終三產品實作目標（獨立於教學進度）

> 2026-10-18 正式複驗：參考 main `794da6e57a6490d580a4c49789519998f941b4c2`；Chat SSR/WASM 建置通過但名稱查詢漏列與 SSR 斷言失敗、Pet 原生幾何 4 案通過、Analytics Cookie 續期與 migration 升級失敗。Day18 已發布，next_day 各 19，本次未遞增；詳細命令及剩餘缺口見 [複驗證據](teaching/evidence/2026-10-18-functional-revalidation/REPORT.md)。下方舊 SHA／next_day 屬歷史觀察，copilot_use 產品實作未由參考庫結果自動驗收。

## 最高優先權的產品定位
- 本 Repo 是 AI 控制的最終產品程式碼；**直接以 P9 產品能力為交付目標（不等於 P0～P12 中的 P9 學習階段）**。允許依相依性拆成多次 Commit/測試，但不按 rust_ai_chat 的 Day 編號或教學 P0–P12 進度等待。
- 產品目標與前進項目以本 repo 為主；目錄／程式銜接以 rust_ai_chat 最新 main 為主，其他遵循 [共同學習規則](COMMON_LEARNING_RULES.md)，兩邊程式與學習進度分別驗收。
- 以下是依既有需求整理的完整能力範圍；**Gate 以原始需求及實際證據逐項核對**，本文件不捏造已完成狀態。

## 已確認的產品需求補充

Chat 入口／註冊／帳號與 Session、Pet 多寵物出入籠／習性／右鍵坐下及多螢幕例外、Analytics 帳號／角色／來源授權／稽核，依 [現行共同規則](COMMON_LEARNING_RULES.md)「已逐項確認的現行需求（2026-10-17）」與 [詳細驗收條件](UI_ALIGNMENT_REQUIREMENTS.md)。這些已確認需求須逐項實作及驗收，本次僅同步文件，未新增功能完成聲明。

## P9 必備能力

- Chat：Axum/Leptos SSR、Hydration、Nested/Protected Router、Session/ACL、多房間與使用者、history、WS presence/typing/private/reconnect/send queue、交易 Outbox、多 Persona/AI-to-AI、Streaming/Structured Output/tools、RAG/SQLite vectors/MCP/Agent、OTel。
- Desktop Pet：Tauri/Leptos、繁體中文五項設定與進階覆寫、多寵物、角色一致性生圖/動畫與固定 200×200 顯示、記憶/學習/active recall/spaced repetition/adaptation/proactivity、RAG/MCP/Agent、實際桌面視窗漫遊（監視器工作區邊界、移動及停滯處理）、安全 FileOrganizer 的授權/預覽/SHA256 重複檔案分組/確認/TOCTOU/receipt/Trash/restore、跨平台 installer/updater。
- Analytics：獨立 PostgreSQL、gRPC／Protobuf 事件上報、HTTP Dashboard／Query＋SSE 即時更新、同服務 Dashboard/Ingest/Query、來源認證、冪等、完整來源事件、Taipei 趨勢、資料治理/備份、可追溯 1–3 句 AI 洞察及可審核建議、OTel；按 Provider／Model／來源產品／功能／請求 ID／時間統計 input、output、cached input、reasoning、total tokens、價格版本與估算／實際費用，日／月趨勢、預算警示及供應商實際提供的 quota／remaining／reset。缺失額度標未知，不以 Token 推算配額；串流、重試、失敗與缺失 usage 須去重／補帳，不記錄密鑰、Prompt 或回應本文。驗證採既有紀錄與模擬資料，不讀密鑰、不執行付費呼叫。
- 整合：獨立執行、隔離資料庫、跨服務事件/重試/故障恢復，Observe→Analyze→Act→Evaluate→Observe 閉環；Chat/Pet 採用必須授權且能回退。

視窗漫遊與 SHA256 重複檔案分組已納入 copilot_use 產品目標；依既有成品來源核對仍尚未實作／驗收，rust_ai_chat 的對應核心或測試不等於 copilot_use 已完成；既有視窗內角色動畫或一般掃描預覽不代表這兩項完成。

產品缺口、安全修正、參考適配及教學清單維護於 [FUTURE_WORK.md](FUTURE_WORK.md)；驗收方式與有效證據入口見 [LOCAL_VALIDATION.md](LOCAL_VALIDATION.md)。使用者已取消正式部署，保留原生本地／CI／跨平台測試；Docker 僅為歷史資產，不作現行開發或驗證 runtime與固定成品目標。

## 開發與驗收政策
1. 優先補齊**完整產品可啟動的實際入口**，不是只開新 trait 或只留 CLI；每次提交以可測試的真實跨層功能為單位。
2. Chat/Pet/Analytics 的 Domain、Application、Persistence/Platform Adapter、UI 與 Integration Contract 必須各自維持邊界。
3. PostgreSQL 原生 UUID 或 VARCHAR(40) 均允許；Domain/Application 不依賴儲存型別，SQL/SQLx/Migration 限定在 Adapter 並做雙模式測試。
4. 對寫入敏感操作使用安全預設（確認、權限、路徑重新驗證、不可跟隨 symlink、原子性），未達安全證據不得啟用正式清理。
5. 驗證須提供 Rust fmt、clippy、unit/DB/HTTP/WS/GUI/E2E、跨平台、部署/回滾、資安、監控的真實結果。缺證據標 Blocked；不可把 scaffold/README/CI 配置當測試通過。
6. 對每個功能記錄規格→檔案/提交→測試/CI→未完成事項，持續前進直至全部 P9 要求被驗證。

**目標是 P9 完整三產品，不是照著 Notion 教學 Day N 一天寫一小節。**

Chat／Pet 使用可攜式 SQLite；Analytics 維持 PostgreSQL。根據變更完成必要驗收；測試保留在所屬專案／模組，不放最外層。
