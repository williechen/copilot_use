# 系統架構與現行程式入口

`copilot_use` 維護規則、教學、進度與證據；`rust_ai_ring` 為唯一程式、測試、CI 與功能驗收主體。現行目錄及名稱見 [SOURCE_LAYOUT](SOURCE_LAYOUT.md)，政策正本見 [共用規則](COMMON_LEARNING_RULES.md)。

本次來源 `0b3e58440bf4b36a398e3698e352e69faf53cfa2`，只核對靜態目錄、manifest 及入口。先前 f61ea1c / f3f8255 等驗證保留原始來源；本頁不宣稱在新 SHA 重跑功能、DB 或 GUI。

| 邊界 | 精確來源 | 實際位置與責任 |
| --- | --- | --- |
| Chat HTTP / WS / Session | [chat_apps/chat_server/src/lib.rs](https://github.com/williechen/rust_ai_ring/blob/0b3e58440bf4b36a398e3698e352e69faf53cfa2/chat_apps/chat_server/src/lib.rs) | Axum 路由、會員 Session、房間頁與 HTTP / WS 授權。 |
| Chat UI | [chat_apps/chat_ui/src/lib.rs](https://github.com/williechen/rust_ai_ring/blob/0b3e58440bf4b36a398e3698e352e69faf53cfa2/chat_apps/chat_ui/src/lib.rs) | Leptos SSR / hydration、Router 與互動。 |
| Chat application / domain | [chat_apps/chat_application/src/lib.rs](https://github.com/williechen/rust_ai_ring/blob/0b3e58440bf4b36a398e3698e352e69faf53cfa2/chat_apps/chat_application/src/lib.rs) | 使用案例與 ports；領域型別位於同 workspace 的 chat_domain。 |
| Chat persistence | [chat_apps/chat_persistence/src/lib.rs](https://github.com/williechen/rust_ai_ring/blob/0b3e58440bf4b36a398e3698e352e69faf53cfa2/chat_apps/chat_persistence/src/lib.rs) | SQLx SQLite adapter；migrations 在 chat_apps/migrations。 |
| Pet UI | [pet_apps/pet_ui/src/main.rs](https://github.com/williechen/rust_ai_ring/blob/0b3e58440bf4b36a398e3698e352e69faf53cfa2/pet_apps/pet_ui/src/main.rs) | Leptos CSR 與 Tauri IPC 呼叫端。 |
| Pet native / windows | [pet_apps/pet_server/src/lib.rs](https://github.com/williechen/rust_ai_ring/blob/0b3e58440bf4b36a398e3698e352e69faf53cfa2/pet_apps/pet_server/src/lib.rs) | Tauri commands、狀態、原生視窗與實際產品呼叫鏈。 |
| Pet storage | [pet_apps/pet_server/src/pet_store.rs](https://github.com/williechen/rust_ai_ring/blob/0b3e58440bf4b36a398e3698e352e69faf53cfa2/pet_apps/pet_server/src/pet_store.rs) | 目前 SQLx SQLite 實作仍在 server；migrations 在 pet_apps/migrations。 |
| Analytics HTTP / UI | [analytics_apps/analytics_server/src/lib.rs](https://github.com/williechen/rust_ai_ring/blob/0b3e58440bf4b36a398e3698e352e69faf53cfa2/analytics_apps/analytics_server/src/lib.rs) | Axum HTTP、Session、Dashboard 與事件入口。 |
| Analytics startup | [analytics_apps/analytics_server/src/main.rs](https://github.com/williechen/rust_ai_ring/blob/0b3e58440bf4b36a398e3698e352e69faf53cfa2/analytics_apps/analytics_server/src/main.rs) | PostgreSQL 與服務啟動；migrations 在 analytics_apps/migrations。 |

## 分層要求與後續教學

- Leptos 負責頁面與互動；Chat SSR / hydration、Pet CSR + Tauri、Analytics UI 依實際接線核對。
- Axum 負責 HTTP / WebSocket、routing / middleware、Cookie / Session 與服務端 ACL。
- Application / Domain 負責使用案例與規則；SQLx adapter 處理資料庫查詢、交易與 migrations。
- Pet 的 pet-application / pet-persistence，以及 analytics-persistence 目前是空 library，還沒有承接既有 server 內的程式。後續教學依最新來源進行必要的小切片搬移、介面調整與呼叫端／測試接線，不能以目錄存在代替實作完成。
- 共用 crates 的 shared 目前依賴 chat-domain，仍有跨 workspace 的 package path dependency；不得假設三案已完全解耦。

[使用者流程](requirements/USER_FLOWS.md) · [後續缺口](FUTURE_WORK.md) · [教學銜接](TEACHING_ALIGNMENT.md) · [本地操作](LOCAL_START.md) · [驗證](LOCAL_VALIDATION.md)
