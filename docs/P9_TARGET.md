# P9 最終三產品實作目標（獨立於教學進度）

> 實作進度、精確來源與驗證結果統一見[目前進度](teaching/current/STATUS.md)。本文件保留功能需求／完成條件或架構；歷次測試摘要已移入歷史紀錄，不作現行完成聲明。

## 需求、實作與驗收

本頁列出產品最終需求，不是已完成功能清單。當前原始碼位置見 [系統架構](SYSTEM_ARCHITECTURE.md)，待做及待驗收項目見 [開發清單](FUTURE_WORK.md)。程式、測試、CI 與功能驗收全部以 rust_ai_chat 為主；copilot_use 保存需求、規則、教學及交付證據。需求存在不表示程式已實作或驗收。

本次只整理文件；保留所有原需求，不新增產品完成聲明。改寫前版本及來源雜湊見 [封存紀錄](teaching/history/evidence/2026-10-22-current-docs/README.md)。

## 最高優先權的產品定位
- 本 Repo 是需求、規則與教學文件庫；主體程式為 rust_ai_chat。**P9 產品能力是最終需求目標（不等於 P0～P12 中的 P9 學習階段）**。允許依相依性拆成多次 Commit/測試，但不按 rust_ai_chat 的 Day 編號或教學 P0–P12 進度等待。
- 需求正文維護在本 repo；實作現況、選題缺口、目錄／呼叫鏈、測試／CI／驗收只以 rust_ai_chat 最新 main 為主，依 [共同學習規則](COMMON_LEARNING_RULES.md) 分開記錄教學交付與主體程式的套用／驗收。
- 以下是依既有需求整理的完整能力範圍；**Gate 以原始需求及實際證據逐項核對**，本文件不捏造已完成狀態。

## 已確認的產品需求補充

Chat 入口／註冊／帳號與 Session、Pet 多寵物出入籠／習性／右鍵坐下及多螢幕例外、Analytics 帳號／角色／來源授權／稽核，依 [現行使用者流程](requirements/USER_FLOWS.md)與 [詳細驗收條件](UI_ALIGNMENT_REQUIREMENTS.md)。這些已確認需求須逐項實作及驗收，本次僅同步文件，未新增功能完成聲明。

## 實作依賴 Gate：先能建立身分／實體，才擴充能力（2026-10-19）

這是對下列 **P9 必備能力** 的施作順序補充，**沒有刪除、延後或降低最終 P9 產品功能條件**，也不把已發佈教學視為成品驗收。以 `FUTURE_WORK.md` 當前功能清單追蹤，並以 rust_ai_chat 最新入口、資料庫、UI、實際測試重新判斷。

1. **Chat：** 首先確保使用者能完成註冊或正確登入 → 持久化帳號／Session → 公開清單顯示與進房時的身分／ACL 核對 → 加入／建立房間，才接訊息歷史、多角色聊天、AI、RAG/Agent 等上層能力；未經授權的房間或訊息不得開放。若其他分支已有 P0 註冊試作，先查重用與合併狀態。
2. **Desktop Pet：** 首先能以 UI 建立第一隻有身分、名稱及初始外觀的寵物 → Tauri IPC → SQLite 持久化 → 生活視窗顯示及重啟載入，然後才發展出籠/回籠、基本互動、漫遊、手動找回、多寵物、AI/記憶。預設建立的 `PetMachine` 不是使用者建立寵物的驗收證據。
3. **Analytics：** 首先是可安全初始化、持久化、登入與首次強制改密碼的管理員身分以及 Session/後台權限，再加其他角色、帳號級等待、來源授權、事件/成本統計與 gRPC/SSE。固定登入碼與 429 header 不是正式登入驗收證據。

產品端要有 **UI→API/IPC→Domain/Application→DB→UI 回饋及重啟讀回** 的最低限度可操作驗收；教學端則可每天完成該功能的一個 Red→最小 Green 切片，但須明列整條功能尚未接通的缺口。不得讓既有的低階測試、CI success 或文章 Day 遞增掩蓋基本功能未完成。正式部署仍非固定交付目標，保留本地／CI／原生操作驗收。

## P9 必備能力

- Chat：Axum/Leptos SSR、Hydration、Nested/Protected Router、Session/ACL、多房間與使用者、history、WS presence/typing/private/reconnect/send queue、交易 Outbox、多 Persona/AI-to-AI、Streaming/Structured Output/tools、RAG/SQLite vectors/MCP/Agent、OTel。
- Desktop Pet：Tauri/Leptos、繁體中文五項設定與進階覆寫、多寵物、角色一致性生圖/動畫與固定 200×200 顯示、記憶/學習/active recall/spaced repetition/adaptation/proactivity、RAG/MCP/Agent、實際桌面視窗漫遊（監視器工作區邊界、移動及停滯處理）、安全 FileOrganizer 的授權/預覽/SHA256 重複檔案分組/確認/TOCTOU/receipt/Trash/restore、跨平台 installer/updater。
- Analytics：獨立 PostgreSQL、gRPC／Protobuf 事件上報、HTTP Dashboard／Query＋SSE 即時更新、同服務 Dashboard/Ingest/Query、來源認證、冪等、完整來源事件、Taipei 趨勢、資料治理/備份、可追溯 1–3 句 AI 洞察及可審核建議、OTel；按 Provider／Model／來源產品／功能／請求 ID／時間統計 input、output、cached input、reasoning、total tokens、價格版本與估算／實際費用，日／月趨勢、預算警示及供應商實際提供的 quota／remaining／reset。缺失額度標未知，不以 Token 推算配額；串流、重試、失敗與缺失 usage 須去重／補帳，不記錄密鑰、Prompt 或回應本文。驗證採既有紀錄與模擬資料，不讀密鑰、不執行付費呼叫。
- 整合：獨立執行、隔離資料庫、跨服務事件/重試/故障恢復，Observe→Analyze→Act→Evaluate→Observe 閉環；Chat/Pet 採用必須授權且能回退。

舊庫視窗漫遊與 SHA256 分組的 [Linux 紀錄](weekly/2026-10-16/acceptance.json) 只保留其原來源與平台，不作 rust_ai_chat 已實作／已驗收的證據；主體現況需匹配其精確 SHA 另行核對。

產品缺口、安全修正、參考適配及教學清單維護於 [FUTURE_WORK.md](FUTURE_WORK.md)；驗收方式與有效證據入口見 [LOCAL_VALIDATION.md](LOCAL_VALIDATION.md)。使用者已取消正式部署，保留原生本地／CI／跨平台測試；Docker 僅為歷史資產，不作現行開發或驗證 runtime與固定成品目標。

## 開發與驗收政策
1. 優先補齊**完整產品可啟動的實際入口**，不是只開新 trait 或只留 CLI；每次提交以可測試的真實跨層功能為單位。
2. Chat/Pet/Analytics 的 Domain、Application、Persistence/Platform Adapter、UI 與 Integration Contract 必須各自維持邊界。
3. PostgreSQL 原生 UUID 或 VARCHAR(40) 均允許；Domain/Application 不依賴儲存型別，SQL/SQLx/Migration 限定在 Adapter 並做雙模式測試。
4. 對寫入敏感操作使用安全預設（確認、權限、路徑重新驗證、不可跟隨 symlink、原子性），未達安全證據不得啟用正式清理。
5. 驗證須提供 Rust fmt、clippy、unit/DB/HTTP/WS/GUI/E2E、跨平台、部署/回滾、資安、監控的真實結果。缺證據標 Blocked；不可把 scaffold/README/CI 配置當測試通過。
6. 對每個功能記錄規格→檔案/提交→測試/CI→未完成事項，持續前進直至全部 P9 要求被驗證。

**產品交付以完整三產品能力及逐項驗收為準。每日教學依 GitHub 現行共用規則獨立交付。**

Chat／Pet 使用可攜式 SQLite；Analytics 維持 PostgreSQL。根據變更完成必要驗收；測試保留在所屬專案／模組，不放最外層。
