# 正式版本與教學分支

正式產品固定於 [v0.1.0](https://github.com/williechen/copilot_use/releases/tag/v0.1.0)，程式 SHA `22b44f33b9e2c9edb0fb398fb9bff3efd57b00ee`。其六項 CI、四份本地報告與三平台安裝／真正簽章更新已通過，checksum 隨 Release 附件保存。正式 tag 不改動。本分支為 `teaching/rust-ai-chat-alignment`，從該 tag 建立；以下調整在正式版本固定後才執行，並已整合到 `main`，讓 GitHub 預設檢視也對應學習倉庫的產品目錄。後續每日教學仍在教學分支推進。

唯讀學習參考：`williechen/rust_ai_chat` 的 `fdc74c1ee244cf86df3fd1c427d893d2956f6211`。此 clone 與遠端均未修改；每日需另行唯讀核對最新 main，不能把 copilot_use 的產品驗收算成學習倉庫已套用。

## 已做的實際目錄與程式調整

| 正式版本原路徑 | 教學分支實際路徑 |
|---|---|
| `apps/chat` | `apps/server` |
| `apps/analytics` | `apps/stats_server` |
| `apps/pet_desktop` | `apps/desktop_pet` |
| `apps/pet_ui` | `apps/desktop_pet/frontend` |
| `apps/pet` | `crates/pet_domain` |
| `crates/chat_ui` | `crates/ui` |
| `crates/openai_adapter` | `crates/ai_core` |
| `crates/event_contract` | `crates/shared` |
| `apps/chat_frontend` | `apps/server/frontend` |

Chat 訊息資料與原有輸入不變量實際移至 `crates/chat_domain`；`crates/chat_application` 使用 `MessageRepository` Port，驗證後才取得可發布的 durable commit。`crates/persistence` 保存真正 SQLite schema、ChatWriter、交易 Outbox、租約／CAS、同意撤回與備份 Adapter；`apps/server` HTTP／WS 入口使用這些 crates，透過 re-export 保留既有 API 與 CLI 相容。這些不是空 crate。

SQLite migrations 為 `crates/persistence/migrations`；原 PostgreSQL migrations 保留在 `apps/server/postgres-migrations`，不在 SQLite 啟動時套用。Analytics 為 `apps/stats_server` 的 PostgreSQL。Pet 保持既有 SQLite；其 runtime／平台 Adapter 仍位於 pet_domain 所屬模組，純 Profile／Application 模組邊界保留，不能宣稱整個 crate 沒有 driver 依賴。

Rust 回歸測試保留在所屬 crate／app：例如 `crates/chat_application/src/tests.rs`、`crates/persistence/src/tests.rs`、`crates/persistence/src/database/tests.rs`、`apps/server/src/knowledge/tests.rs`、`apps/desktop_pet/tests`。私有模組測試不因外移暴露危險 API；根目錄沒有 tests workspace。

Docker、Compose、WASM 建置、Tauri 資產、Python／Node 驗收、CI與依賴相對路徑已跟實際程式調整。CI push 包含本教學分支；程式 package／binary 的 day9-* 名稱為既有命令相容性保留，不是教學日期。

## 驗收與每日教學規則

目錄／crate 重構的工作樹已通過完整 workspace 編譯、fmt／Clippy、SQLite／PG unit／HTTP／WS／512連線與故障回歸、Chat／Pet WASM 與 Tauri native 編譯。教學分支 `50bed562918659c3506b3e53101484bb5b6de380` 的六項 CI 已全部成功（run 38206372675），包含三平台安裝與更新。整合到 main 不修改此程式內容；新的 main CI 結果另行記錄，不能冒稱已完成。教學分支結果不得替代正式 tag 的既有證據。

每日 08:00 Asia/Taipei 先讀共同主頁、三專案主頁、上一 Day 與 rust_ai_chat 最新實際 source。只在本分支調整真正需要銜接教學的目錄／程式／所屬測試，執行相應驗證並提交；每份 Day 附精確來源、改動檔案、完整可套用程式及實際結果。未變更檔案標 N/A，不用空測試或只改規劃代替實作。沿用使用者指定 OpenAI gpt-5.6-luna，不把 key 寫入 Git／Notion；只有 AI 邏輯變更需要重跑對應付費測試。

正式 Day 五節為學習大綱、實作內容、套件版本、重構內容、驗證項目。查重與寫後讀回為必要步驟；沒有可核實程式／測試對應就不產生假的 Day。既有 Day 8 與未完成舊草稿保留其歷史狀態。需要新需求才另行調整整體產品與驗收，不覆寫 v0.1.0。

## 目錄對照與保留差異

```text
apps/
  server/             # Rust AI Chat HTTP／WS 入口
  desktop_pet/        # Tauri 入口；frontend/ 是 Pet Leptos 前端
  stats_server/       # PostgreSQL Analytics
crates/
  ai_core/
  chat_application/
  chat_domain/
  persistence/
  pet_domain/
  shared/
  telemetry/
  ui/
```

以上三個 app 與八個核心 crate 名稱對應唯讀 rust_ai_chat。`crates/mcp_bridge` 是本產品已使用的 MCP Adapter；`apps/server/frontend` 保留獨立 Chat hydration 建置入口。SQL migrations 保留所屬 Persistence／產品目錄，避免把 SQLite 與 PostgreSQL migrations 混合執行；不為複製參考外觀新增無用途的 deploy 或根 tests 目錄。部署與測試腳本、vendor 安全修補為本產品需要的附加內容。

`native-source-audit.yml` 是固定 checkout v0.1.0 SHA 的歷史來源診斷，因此刻意使用正式 tag 的舊路徑；日常 `verify.yml` 與 Docker／建置腳本使用目前對齊後的路徑。該 Windows 診斷 run 38242215542 已成功，Cargo.toml 的 LF／CRLF 正規化造成 raw dirty 標記，所有觀測 tracked diff 為空白。正式 tag 和原始報告保持不變。
