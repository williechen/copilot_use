# 系統架構與執行邊界

`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_chat` 是唯一程式、測試、CI 與功能驗收主體。 規則正本見 [共用規則](COMMON_LEARNING_RULES.md)，需求及未完成項見[現行使用者流程](requirements/USER_FLOWS.md)、[人員驗證需求](requirements/AUTHENTICATION.md)與[FUTURE_WORK](FUTURE_WORK.md)。

## 程式來源與驗證範圍

本次僅依 rust_ai_chat/main `f61ea1c59667f3c92c78e576e3c48038a8ce030a` 靜態核對目錄及啟動入口，未對此 SHA 重跑測試、服務、DB 或 GUI。每次教學仍須重取最新 main；本頁定點路徑不能代替即時來源。先前 copilot_use 程式架構[原文保留](teaching/history/evidence/2026-10-23-repository-roles/before/docs/SYSTEM_ARCHITECTURE.md)，不作主體實作現況。

| 邊界 | rust_ai_chat 來源 | 職責與核對 |
| --- | --- | --- |
| Chat HTTP／WS／Session | [apps/server/src/lib.rs](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/apps/server/src/lib.rs) | Axum 路由、登入／登出、房間頁授權與 HTTP／WS；各入口 ACL 需分別驗證 |
| Chat UI | [crates/ui](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/crates/ui/src/lib.rs) | Leptos SSR／hydration、Router、元件與互動；不直接查 DB |
| Chat 純層 | [chat_application](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/crates/chat_application/src/lib.rs)、[chat_domain](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/crates/chat_domain/src/lib.rs) | 使用案例、Port、領域規則 |
| Chat 儲存 | [persistence](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/crates/persistence/src/lib.rs) | SQLx SQLite Adapter；資料表與 migration 依此庫原始碼核對 |
| Pet UI／原生橋接 | [frontend](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/apps/desktop_pet/frontend/src/main.rs)、[desktop_pet](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/apps/desktop_pet/src/lib.rs) | Leptos CSR → Tauri IPC → 使用案例／狀態／原生視窗；逐 pet id 與重啟結果分開驗 |
| Pet 核心／持久化 | [pet_domain](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/crates/pet_domain/src/lib.rs)、[pet_store](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/apps/desktop_pet/src/pet_store.rs) | 核心與 SQLx SQLite／平台 Adapter 邊界；不從舊庫 pet_runtime 推定接線 |
| Analytics | [stats_server main](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/apps/stats_server/src/main.rs)、[stats_server lib](https://github.com/williechen/rust_ai_chat/blob/f61ea1c59667f3c92c78e576e3c48038a8ce030a/apps/stats_server/src/lib.rs) | Axum HTTP、帳密／Session、PostgreSQL；操作、角色及來源權限逐項核對 |

## 既定架構要求

- **Leptos** 負責頁面、元件與互動：Chat SSR／hydration／Router、Pet CSR + Tauri、Analytics 正式登入／修改密碼／管理 UI。不得以 Axum `Html(...)`／`Html(format!(...))` 完整字串頁面取代 Leptos UI 或冒稱新 UI Green。
- **Axum** 負責 HTTP/API、WebSocket、Routing／Middleware、Cookie／Session、安全性及服務端 ACL；UI route guard 不能代替後端授權。
- **Application／Domain** 負責使用案例、流程與業務規則；**SQLx Adapter** 負責查詢、交易與 migration。Chat／Pet SQLite 與 Analytics PostgreSQL 隔離。
- **Tauri IPC** 配合 Pet Leptos CSR，負責原生橋接與視窗操作；業務及資料存取沿核心／Adapter 邊界處理。

每 Day 核對真實 `Leptos UI → Axum HTTP/API 或 Tauri IPC → Application/Domain → SQLx Adapter` 的最小可操作切片。要求與現行實作是否達成分開：只有 API/helper 或 Axum HTML 成功，不代表 Leptos UI 完成。gRPC／Outbox／SSE 等目標只在 rust_ai_chat 實際接線與匹配測試存在時才可記為已實作，不借用舊 copilot_use 能力。

## Day 與證據

先逐案讀回已發布 Day27，再依 copilot_use 的 [learning-state](learning-state.json) 即時 next_day 接續，不寫死、倒退、跳號或重教已完成內容。必要更正在後續對應 Day 的 Red／完整 Green／Refactor 後，或採保留原 commit／blob／SHA256 的版本化勘誤；原 Day 與歷史證據不回寫。文件同步不增加 Day，也不代表程式套用或產品驗收。

[程式銜接](TEACHING_ALIGNMENT.md) · [呼叫鏈核對](SEMANTIC_ARCHITECTURE_REVIEW.md) · [驗收方式](LOCAL_VALIDATION.md) · [分工同步範圍](REPOSITORY_ROLE_TRANSITION.md)
