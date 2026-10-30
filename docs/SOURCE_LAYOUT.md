# 現行來源與目錄

產品 `rust_ai_ring@6b34c42f74fc1585b1b864fb7dd6969318270e0f`；本地 `/home/willie/Documents/rust_ai_ring`。根目錄沒有 Cargo.toml，必須使用各 workspace manifest。

| Workspace | package 版本 | 成員／package |
| --- | --- | --- |
| `chat_apps` | `0.2.43` | `chat-server`, `chat-application`, `chat-domain`, `chat-persistence`, `chat-ui` |
| `pet_apps` | `0.2.43` | `pet-server`, `pet-ui`, `pet-domain`, `pet-application`, `pet-persistence` |
| `analytics_apps` | `0.2.43` | `analytics-server`, `analytics-persistence`, `analytics-ui`, `analytics-domain`, `analytics-application` |
| `crates` | `0.2.43` | `ai-core`, `shared`, `telemetry` |

Chat：chat_server/src/main.rs／lib.rs、chat_ui/src/lib.rs；Leptos SSR＋hydrate，Axum HTTP／WS，SQLite。
Pet：pet_server/src/main.rs／lib.rs、pet_ui/src/main.rs／index.html；Leptos CSR＋Tauri，SQLite。
Analytics：analytics_server/src/main.rs／lib.rs；Leptos Dashboard、Axum HTTP，PostgreSQL。
每案 migration 位於各自 workspace 的 migrations/，四 workspace 各有 Cargo.lock；不要在教學裡重建舊 apps/ 目錄或第二套產品。

`shared` 仍以 path 相依 `chat_apps/chat_domain`。Pet Tauri frontendDist 是 `../pet_ui/dist`，devUrl 是 `http://localhost:1420`。Leptos 輸出名仍為 `rust_ai_chat`，不因 repo 更名改外部契約。

[啟動](LOCAL_START.md) · [驗證](LOCAL_VALIDATION.md) · [架構](SYSTEM_ARCHITECTURE.md)

最新main的Day44 caller：Chat `chat_server/src/room_auth.rs`呼叫`room_patch.rs`（教學原附件名room_path）；Pet `window_position.rs`供pet_windows找回視窗；Analytics `dashboard_url.rs`供Leptos Dashboard，password_login驗logout CSRF。Pet活動Green r3未在main，新增activity模組／資料migration僅見隔離候選。精確blob見[來源清單](source-review/local-validation-2026-10-30-day44/source-inventory.json)。
