# 目錄與程式銜接

以 rust_ai_chat 最新 main 的真實 source 為目錄／程式微調基準，按 [共同學習規則](COMMON_LEARNING_RULES.md) 更新 copilot_use。產品目標及待前進項目以 [P9_TARGET.md](P9_TARGET.md) 與 [FUTURE_WORK.md](FUTURE_WORK.md) 為準。

```text
apps/
  server/                 # Chat HTTP／WS Adapter
    frontend/             # Chat hydration 建置入口
  desktop_pet/            # Tauri IPC／updater
    frontend/             # Pet Leptos UI
  stats_server/           # 同服務 Analytics HTTP／gRPC／SSE
crates/
  chat_domain/
  chat_application/
  persistence/            # Chat SQLite Adapter／migrations
  pet_domain/             # Profile／Port／Schedule
  pet_runtime/            # SQLite／HTTP／AI／OS／檔案 Adapter
  ai_core/                # OpenAI Provider／Agent Adapter
  mcp_bridge/
  shared/                 # 跨產品事件契約與 proto/analytics.proto
  analytics_transport/    # Tonic／Prost Driver；純層不匯入
  telemetry/
  ui/
```

三個 app 與八個核心 crate 名稱對應 rust_ai_chat；pet_runtime、mcp_bridge、analytics_transport 與 Chat frontend 是實際使用的產品 Adapter／建置入口。runtime 保留 day9-pet package／CLI 與 profile/application re-export，已有 Tauri IPC 格式保持。day9-* package 名稱為命令相容性，不是教學編號。

SQLite Chat migration 在 crates/persistence/migrations；Pet 在 crates/pet_runtime/migrations；Analytics PostgreSQL 在 apps/stats_server/migrations。舊 Chat PG import 的 migration 原文在 apps/server/postgres-migrations，不套用到 SQLite。測試留在 owning app／crate／私有模組，不建立根 tests workspace。

每次銜接先核對型別／函式／呼叫端、資料流及斷言。不同 ID 型別、codec、WS／事件協定與 DB Adapter 必須說明映射，不以檔名一致推論相同語意；詳見 [模組與介面對照](SEMANTIC_ARCHITECTURE_REVIEW.md) 與 [完整靜態核對](FULL_SOURCE_REVIEW.md)。視窗漫遊與 SHA256 重複檔案分組已納入產品目標，實作進度另見未來清單，不以目錄對齊代表功能完成。python3 scripts/architecture-boundaries.py 驗證實際 Cargo 的純層正常／build 依賴。

教學只在 teaching/rust-ai-chat-alignment 推進；匹配驗收後整合 main。固定正式 tag／附件不覆寫。每日 06:00 依確認缺口與當日差異產生四節教學，適用 CI 通過即可。原教學保留，必要實作補在第 2 節末尾；不設獨立重構章。週六 08:00 的整合／功能測試／提交與驗收另存工程證據，不放入教學，也不驗證 AI 項目。P0–P12 共 13 階段、26 個月，2026-10-12 起始，每天合計一小時；階段切換、學習完成與功能通過分開。Dry Run 只修正使用者提出問題涉及的文件，不預排 Day。
