# 系統架構與執行邊界

本頁以 copilot_use/main 原始碼核對，來源 commit 見 [本次紀錄](teaching/history/evidence/2026-10-22-current-docs/README.md)。它描述產品接線；[學習庫目前進度](teaching/current/STATUS.md)、[最終產品需求](P9_TARGET.md) 與功能驗收分開。依賴版本以各 Cargo.lock 為準。

## 產品組成

| 元件 | 入口與職責 | 資料／執行 |
| --- | --- | --- |
| Chat Web/API | [server main](../apps/server/src/main.rs)：Axum 路由、帳號／Session、房間授權、HTTP／WS／SSE | Chat 獨立 SQLite；本地 3001 |
| Chat UI | [ui](../crates/ui/src/lib.rs) 與 [前端入口](../apps/server/frontend/src/main.rs)：Leptos SSR／hydration，呼叫 HTTP／WS | 不直接連 DB；資產 `target/site/pkg` |
| Chat 核心 | [chat_domain](../crates/chat_domain/src/lib.rs) → [chat_application](../crates/chat_application/src/lib.rs) → MessageRepository | 不匯入 DB／HTTP driver |
| Chat 儲存 | [persistence](../crates/persistence/src/lib.rs)：ChatWriter、訊息／Outbox／realtime 交易 | migration 在 `crates/persistence/migrations` |
| Pet Native/UI | [Tauri 入口](../apps/desktop_pet/src/main.rs)、[前端](../apps/desktop_pet/frontend/src/main.rs)：生活窗、控制窗與 IPC | 獨立桌面程序；不由 local-services 管理 |
| Pet 核心 | [pet_domain](../crates/pet_domain/src/lib.rs)：Profile、儲存 Port、複習規則 | 純核心，不依賴 Tauri／SQLx |
| Pet Runtime | [pet_runtime](../crates/pet_runtime/src/lib.rs)：PetStore、SQLite、檔案、OS 操作、Outbox、AI Adapter | 自有 app data／`PET_DATA_DIR`；migration 在 runtime |
| Analytics | [main](../apps/stats_server/src/main.rs)、[路由](../apps/stats_server/src/lib.rs)：Dashboard／Query／管理、HTTP 與 gRPC 攝取、SSE | 獨立 PostgreSQL；本地 3100 |
| Analytics 核心與儲存 | `apps/stats_server/src/application`／`persistence` 模組：EventRepository、驗證、交易去重 | migration 在 `apps/stats_server/migrations` |
| 跨產品契約 | [shared](../crates/shared/src/lib.rs)、[analytics_transport](../crates/analytics_transport/src/lib.rs) | ProductEvent／Protobuf、gRPC client/server、逐事件 ACK |
| 外部能力 | `ai_core`、`mcp_bridge`、`telemetry` | Provider／MCP／觀測 Adapter；不作純領域依賴 |

## 資料流與一致性

```mermaid
flowchart LR
  C[Chat UI] --> H[Chat HTTP / WS]
  H --> A[Application Port]
  A --> D[(Chat SQLite + Outbox)]
  P[Pet UI] --> T[Tauri IPC]
  T --> R[Pet Runtime / Domain]
  R --> S[(Pet SQLite + Outbox)]
  D --> G[gRPC / Protobuf]
  S --> G
  G --> E[Analytics source authentication]
  E --> Q[(PostgreSQL dedup / events)]
  Q --> V[HTTP Query / SSE]
```

Chat 先完成持久交易，再對外通知；Pet 的真實檔案操作需 preview／明確確認／receipt，不以純狀態成功取代 OS 結果。Chat／Pet Outbox 保留投遞重試；Analytics 完成資料庫交易後才 ACK，同 ID 去重與衝突分開處理。HTTP JSON 相容攝取入口仍存在。詳細契約及原驗證來源見 [通訊文件](ANALYTICS_TRANSPORT.md)。

三產品不共享業務 DB。Analytics 的查詢 token、管理 token、Chat／Pet 來源 token 分離。**目前 copilot_use Dashboard 仍使用 Bearer token**；人員帳密、Session、角色與來源資料範圍是 [待完成需求](requirements/AUTHENTICATION.md)，不能由學習庫完成紀錄推定此產品已具備。

## 執行及後續接線

本地服務管理器使用 `.local/runtime/chat.sqlite3` 與 `.local/runtime/postgres`，Chat／Analytics 只聽 loopback；Pet 另行建置與啟動。具體步驟及憑證使用見 [本地啟動](LOCAL_START.md)。啟動／healthz 與完整業務驗收分開。

目錄與純層規則見 [程式銜接](TEACHING_ALIGNMENT.md)，關鍵符號與呼叫鏈見 [模組對照](SEMANTIC_ARCHITECTURE_REVIEW.md)，開發缺口見 [FUTURE_WORK](FUTURE_WORK.md)。既有完整兩庫靜態報告保存原來源範圍於 [FULL_SOURCE_REVIEW](FULL_SOURCE_REVIEW.md)，本次未重新做兩庫完整比對或產品測試。

已確認的架構責任維持：Chat 由服務端執行帳號／Session／房間授權；Pet 以 pet id 隔離持久狀態與生活窗；Analytics 將人員 Session／角色與機器配對憑證分離。需求完成須提供匹配 SHA 的跨層及重啟驗收。原文及歷史證據完整保留於 [文件封存](teaching/history/evidence/2026-10-22-current-docs/README.md)。
