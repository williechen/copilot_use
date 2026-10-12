# 目錄與程式銜接

以 rust_ai_chat 最新 main 的真實 source 為目錄／程式微調基準，按 [共同學習規則](COMMON_LEARNING_RULES.md) 更新 copilot_use。產品目標及待前進項目以 [P9_TARGET.md](P9_TARGET.md) 與 [FUTURE_WORK.md](FUTURE_WORK.md) 為準。

```text
apps/
  server/                 # Chat HTTP／WS Adapter
    frontend/             # Chat hydration 建置入口
  desktop_pet/            # Tauri IPC／updater
    frontend/             # Pet Leptos UI
  stats_server/           # 同服務 Analytics Web／API
crates/
  chat_domain/
  chat_application/
  persistence/            # Chat SQLite Adapter／migrations
  pet_domain/             # Profile／Port／Schedule
  pet_runtime/            # SQLite／HTTP／AI／OS／檔案 Adapter
  ai_core/                # OpenAI Provider／Agent Adapter
  mcp_bridge/
  shared/                 # 跨產品事件契約
  telemetry/
  ui/
```

三個 app 與八個核心 crate 名稱對應 rust_ai_chat；pet_runtime、mcp_bridge 與 Chat frontend 是實際使用的產品 Adapter／建置入口。runtime 保留 day9-pet package／CLI 與 profile/application re-export，已有 Tauri IPC 格式保持。day9-* package 名稱為命令相容性，不是教學編號。

SQLite Chat migration 在 crates/persistence/migrations；Pet 在 crates/pet_runtime/migrations；Analytics PostgreSQL 在 apps/stats_server/migrations。舊 Chat PG import 的 migration 原文在 apps/server/postgres-migrations，不套用到 SQLite。測試留在 owning app／crate／私有模組，不建立根 tests workspace。

每次銜接先核對型別／函式／呼叫端、資料流及斷言。不同 ID 型別、codec、WS／事件協定與 DB Adapter 必須說明映射，不以檔名一致推論相同語意；詳見 [模組與介面對照](SEMANTIC_ARCHITECTURE_REVIEW.md)。python3 scripts/architecture-boundaries.py 驗證實際 Cargo 的純層正常／build 依賴。

教學只在 teaching/rust-ai-chat-alignment 推進；匹配驗收後整合 main。固定正式 tag／附件不覆寫。每日先處理參考的微調與必要切片；每週六按共同規則完整比對。
