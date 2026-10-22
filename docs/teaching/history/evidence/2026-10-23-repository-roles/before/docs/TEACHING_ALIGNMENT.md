# 目錄與程式銜接

產品來源為 copilot_use/main；目錄／程式適配依 rust_ai_chat/main 的實際來源及當日差異核對。兩庫各記精確 SHA，不因檔名相同宣稱功能、資料或 wire contract 等同。本次只核對產品現有接線，未重新完整比對參考庫。

| 邊界 | 產品位置 | 接線規則 |
| --- | --- | --- |
| Chat HTTP／WS | `apps/server` | 入口轉換與授權後呼叫 Application／Adapter |
| Chat 前端 | `crates/ui`、`apps/server/frontend` | SSR／hydration；只透過服務介面 |
| Chat 純核心 | `crates/chat_domain`、`crates/chat_application` | 輸入不變量與 MessageRepository Port |
| Chat SQLite | `crates/persistence` | 實作 Port，訊息／Outbox／realtime 同交易 |
| Pet UI／原生 | `apps/desktop_pet`、其 `frontend` | IPC、桌面視窗、確認與結果呈現 |
| Pet 純核心 | `crates/pet_domain` | Profile／Application Port／Schedule |
| Pet Adapter | `crates/pet_runtime` | SQLite／OS／HTTP／AI，package 名稱保留 day9-pet |
| Analytics | `apps/stats_server` | 內部分 application／persistence／HTTP／gRPC／SSE |
| 共用傳輸 | `crates/shared`、`crates/analytics_transport` | 事件契約與實際 gRPC Driver |
| 其他 Adapter | `ai_core`、`mcp_bridge`、`telemetry` | 不回流純 Domain／Application |

`day9-*` 是既有 package／命令相容名稱，不是目前教學 Day。Chat／Pet／Analytics migration 分別歸 `crates/persistence/migrations`、`crates/pet_runtime/migrations`、`apps/stats_server/migrations`；舊 PostgreSQL 匯入 schema 在 `apps/server/postgres-migrations`，不套用至 Chat SQLite。

## 修改時核對

1. 同時找出型別／函式、實際呼叫端與 UI／API／IPC，再核對測試斷言及持久化結果。
2. 明列 ID、作者／房間、category 與 topic、JSON／TOML、WS DTO、事件名稱及 DB Adapter 的映射；合理差異保留，不新增無呼叫端的同名骨架。
3. 測試放所屬 app／crate／模組。學習庫測試歸位的剩餘工作依 [開發清單](FUTURE_WORK.md) 追蹤，不能把歷史根 tests 遷移計畫誤認為產品目前 workspace 結構。
4. 修改純層依賴時執行 `python3 scripts/architecture-boundaries.py --offline`；它核對實際 Cargo normal／build 依賴，不把 dev-only harness 當執行邊界。
5. 文件與教學發布、參考庫實際套用、產品功能驗收分開記錄。每日依 [共用規則](COMMON_LEARNING_RULES.md) 交付，每案每 Day 獨立 commit／推送／全文讀回後才條件同步狀態。

[關鍵模組及呼叫鏈](SEMANTIC_ARCHITECTURE_REVIEW.md) · [產品目標](P9_TARGET.md) · [既有完整比對及原來源](FULL_SOURCE_REVIEW.md) · [本次原文封存](teaching/history/evidence/2026-10-22-current-docs/README.md)
