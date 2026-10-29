# 本地啟動

產品來源 `7ce39f0166a2120539e5908b33c88322a930c35a`。依產品 .env.example 在本機設定環境；不把秘密放入文件。Chat／Pet 使用各自 SQLite，Analytics 使用 PostgreSQL；SQLite URL 採明確絕對路徑，避免從不同 workspace 啟動連到另一份 DB。以下啟動會依程式執行 migration，驗收請用獨立資料副本。

| 專案 | 工作目錄 | 命令 | 入口 |
| --- | --- | --- | --- |
| Chat | `/home/willie/Documents/rust_ai_ring/chat_apps` | `cargo leptos watch --cargo-locked` | `127.0.0.1:3000`，`/health` |
| Pet | `/home/willie/Documents/rust_ai_ring/pet_apps/pet_server` | `NO_COLOR=true cargo tauri dev` | 控制台＋逐隻生活窗；前端 1420 |
| Analytics | `/home/willie/Documents/rust_ai_ring/analytics_apps` | `cargo run --locked -p analytics-server` | `127.0.0.1:3100`，`/healthz` |

Chat 使用 CHAT_DATABASE_URL，Pet 使用 PET_DATABASE_URL，Analytics 使用 ANALYTICS_DATABASE_URL。Pet 的 Tauri beforeDevCommand 在 ../pet_ui 執行 trunk serve；需 Rust、WASM target、Trunk 及對應 native 依賴。Analytics 管理員初始化工具是 `cargo run --locked -p analytics-server --bin bootstrap_admin`，請先閱讀該程式要求，不在既有正式庫盲目初始化。

啟動成功不代表完整驗收。[驗證與限制](LOCAL_VALIDATION.md) · [現況](teaching/current/STATUS.md)
