# Rust AI Ring｜P0–P11 正式階段規劃

> 生效決策：2026-10-25。**此文件是需求／規劃，不是已完成聲明。** `copilot_use` 保存規則、文章、狀態與證據；**唯一程式碼、CI 與功能驗收主體是 [williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)**。不得要求在本文件庫重做程式、CI、發布或產品驗收。現有 Day、歷史 P9 文件與新 P9 階段不可混用。

## 規劃原則

1. 12 個正式階段固定為 **P0～P11**，每階段記錄 Scope、Non-goals、Prerequisites、Deliverables、Acceptance Criteria、Tests、Evidence、Status。
2. **P0 僅包含 Web Chat、Desktop Pet、Analytics 三產品的非 AI 基礎流程**；不把真實 AI、RAG、MCP、Agent、完整 OTel 當作 P0 Gate。
3. **P0 驗收後立即安排 P1.0 Common Logging Baseline**，作為 P1 Security & Identity 的首個共同工作包；**不等到 P10 才提供基本日誌**。
4. **P10** 專注 distributed tracing、metrics、context propagation、exporter/collector、alerting 與營運 Dashboard；各階段早期所需的基礎日誌、安全和診斷仍必須存在。
5. GitHub CI 全綠或原始碼存在 ≠ Runtime／GUI／E2E 完整驗收。以 rust_ai_ring 的精確 commit SHA、測試與驗收證據為準；新規劃不回填既有 Day 結論。
6. Domain / Application 透過 Port/Adapter 隔離框架、DB、OS 與模型 SDK；逐步實作，避免預設空架構。
7. 每階段狀態：Not Started → In Progress → Implemented → Verified；只有 Exit Gate 全部通過才能 Verified。
8. 原 [已廢除 P9 規則](P9_TARGET.md) 維持歷史停用狀態；下文「P9 Learning & Teaching Agent」是新階段名稱，不恢復舊 P9 規則或其驗收清單。

## 12 階段全覽

| 階段 | 正式名稱 | 核心交付 | Exit Gate |
| --- | --- | --- | --- |
| P0 | Foundation & MVP | Chat / Pet / Analytics 非 AI 核心流程 | 三產品使用者旅程與持久化 E2E、必要 CI / runtime |
| P1 | Security & Identity（含 P1.0 Logging） | 先建共用結構化 Logging，再強化身分、Session、ACL、CSRF、Rate Limit | 日誌與負向授權測試合格 |
| P2 | Reliability & Persistence | Transaction / Idempotency / Recovery / Backup | 重試、重啟、故障無錯誤成功或資料不一致 |
| P3 | Real-time Communication | Reconnect / Presence / Typing / History Sync | 漏訊可補回、連線生命週期受控 |
| P4 | AI Integration | Responses API、HTTP/SSE、Structured Output、Function Calling | 真實 Provider、streaming、逾時取消測試 |
| P5 | AI Persona & Conversation | Persona、Context、Turn-taking、Cooldown / Budget | 多角色發言受控、無無限循環 |
| P6 | Memory & RAG | Persistent Memory、Embeddings、Retriever abstraction | 跨重啟可取得授權記憶並可管理 |
| P7 | MCP & Tool Integration | MCP Client / Tool Registry / Tool permission / Audit | 工具權限限制、失敗處理可驗證 |
| P8 | Agent Runtime | State machine、Execution / Turn boundary、Recovery | 有界執行、取消、checkpoint / error recovery |
| P9 | Learning & Teaching Agent（新定義） | Learner state、Active Recall、Spaced Repetition | 學習狀態驅動可解釋教學行為 |
| P10 | Observability & Operations | OTel Traces / Metrics / Export / Alerting | 端到端觀測、安全無敏感值洩漏 |
| P11 | Production & Release | 部署、Installer、Updater、Rollback | 正式部署、升級、回滾與 release 驗收 |

## P0 — 非 AI 產品驗收

### Web Chat
- 註冊 → 登入 → 建房/查房 → 兩人聊天 → 訊息持久化 → 重載／重啟後讀取歷史 → 登出。
- HTTP、WebSocket、DB、Browser E2E；驗證登入及房間授權、Session 撤銷，拒絕無權存取。
### Desktop Pet
- 一般使用者啟動 Tauri → 建立／選取寵物 → 基本移動互動 → 結束／重啟後保留可恢復的寵物狀態。
- SQLite integration 與真 GUI/native runtime 操作；不把 FileOrganizer 破壞性操作與 AI 行為強行列入 P0。
### Analytics
- 管理員登入／密碼管理 → 授權事件寫入 → 同來源 event-id 去重 → Dashboard 正確統計／篩選 → 重啟後一致。
- PostgreSQL integration、HTTP/Browser E2E；機器事件來源 Token 與人員查詢 Session 權限分離。
### P0 Gate
- 三條使用者流程 Verified，必要 CI 通過、無 P0 blocker。
- 任何既有單元測試、局部測試或工作流成功，不自動表示完整 Gate 已通過。

## P1.0 — 三產品共用 Logging Baseline（優先工作包）

### 已知差距與取證位置
- 以 rust_ai_ring/main 2026-10-25 觀測到 `crates/telemetry/src/lib.rs` 仍為空，三個進程未統一初始化 `tracing-subscriber`，部分操作仍直接 `println!` / `eprintln!`。
- **勿將本文件庫內歷史 `crates/telemetry` 程式當作 rust_ai_ring 已實作的證據**；實作前務必重新讀取最新 source SHA。

### 最小交付
1. 在 rust_ai_ring 共用 `crates/telemetry` 建立 `tracing` / `tracing-subscriber` 初始化，Chat/Pet/Analytics 逐進程接入；避免 Domain/Application 依賴特定輸出技術。
2. `RUST_LOG` 與 ERROR / WARN / INFO / DEBUG / TRACE；開發期易讀文字，正式環境 JSON 可切換；stdout/stderr 作為預設 sink。
3. 必要欄位：timestamp、level、target、service.name、event_name；HTTP context 使用 request_id，必要時用 connection_id（request_id **不等於** distributed trace ID）。
4. Chat：啟停、HTTP 與 WebSocket connect/disconnect、auth failure、room/message persistence error。
5. Pet：啟停、建立/選取寵物、DB 錯誤、唯讀預覽與安全拒絕；不記錄完整私密路徑。
6. Analytics：啟停、event stored/duplicate/conflict、DB 錯誤、管理員登入成功/失敗。
7. 以明確 allowlist / safe formatting 防止 password、API key、cookie、Session/token、聊天室本文、私密檔案內容及敏感 path 寫入；禁止未審核的 request/header debug dump。
8. 部署文件描述 stdout/stderr 收集、rotate、retention、查詢方法；留存期依環境設定，不在程式無界累積。
9. 測試/CI：JSON 可解析、level filter、必要欄位、secret-redaction、失敗事件、三程序 runtime smoke；以新 CI run 與真實取證結果判定 Verified。

### P1.0 不包含
OpenTelemetry Collector / exporter、跨服務 distributed traces、metrics dashboard、alerting、AI 成本追蹤；以上由 **P10** 負責。

## P1 後續與 P2～P11 依賴

- P1.0 Logging → P1 Session / ACL / resource authorization / rate-limit 與 security-hardening。
- P2 Persistence reliability、P3 即時通訊可按實際依賴拆分；基礎安全與日誌持續要求。
- P4 AI → P5 Persona → P6 Memory/RAG → P7 MCP → P8 Runtime → P9 Learning（依功能邊界增量交付，可局部並行）。
- P10 擴展既有 Logging 為完整 distributed observability；P11 交付可維運的正式部署及安裝。
- 若階段只涉及某產品，其他產品標記 N/A，不能視作未完成。

## 工作與證據治理

| 層級 | 位置 | 責任 |
| --- | --- | --- |
| 正式階段與教學 | copilot_use（本庫） | 需求定義、優先序、學習文章與歷史證據 |
| 產品程式與測試 | rust_ai_ring | 程式碼、Migration、CI、runtime、功能驗收 |
| Phase Gate | 連結雙庫來源與精確 SHA | Requirement → code → tests → CI → E2E → acceptance |
| Daily Day | copilot_use 的 next_day / manifest | 小型教學切片，不能取代 Phase Verified |

狀態不由本 Roadmap 的發布自動更新；與 [FUTURE_WORK](FUTURE_WORK.md)、[現行 STATUS](teaching/current/STATUS.md)、[現行需求](requirements/USER_FLOWS.md) 發生差異時，保留既有證據，依最新確認的需求和 rust_ai_ring 真實驗收重新調整。

## 參考
- [rust_ai_ring（唯一產品主體）](https://github.com/williechen/rust_ai_ring)
- [儲存庫分工](REPOSITORY_ROLE_TRANSITION.md)
- [共用規則](COMMON_LEARNING_RULES.md)
- [目前完成度](teaching/current/STATUS.md)
- [Notion 規劃草稿](https://app.notion.com/p/404e51d4509381158084f17be04caf8c?pvs=204)
