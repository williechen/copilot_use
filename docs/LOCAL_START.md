# 本地啟動

產品來源 `2c7ff8b861d32ab6d879bf80c9f2dce50de38097`。依產品 .env.example 在本機設定環境；不把秘密放入文件。Chat／Pet 使用各自 SQLite，Analytics 使用 PostgreSQL；SQLite URL 採明確絕對路徑，避免從不同 workspace 啟動連到另一份 DB。以下啟動會依程式執行 migration，驗收請用獨立資料副本。

| 專案 | 工作目錄 | 命令 | 入口 |
| --- | --- | --- | --- |
| Chat | `/home/willie/Documents/rust_ai_ring/chat_apps` | `cargo leptos watch --cargo-locked` | `127.0.0.1:3000`，`/health` |
| Pet | `/home/willie/Documents/rust_ai_ring/pet_apps/pet_server` | `NO_COLOR=true cargo tauri dev` | 控制台＋逐隻生活窗；前端 1420 |
| Analytics | `/home/willie/Documents/rust_ai_ring/analytics_apps` | `cargo run --locked -p analytics-server` | `127.0.0.1:3100`，`/healthz` |

Chat 使用 CHAT_DATABASE_URL，Pet 使用 PET_DATABASE_URL，Analytics 使用 ANALYTICS_DATABASE_URL。Pet 的 Tauri beforeDevCommand 在 ../pet_ui 執行 trunk serve；需 Rust、WASM target、Trunk 及對應 native 依賴。Analytics 管理員初始化工具是 `cargo run --locked -p analytics-server --bin bootstrap_admin`，請先閱讀該程式要求，不在既有正式庫盲目初始化。

啟動成功不代表完整驗收。[驗證與限制](LOCAL_VALIDATION.md) · [現況](teaching/current/STATUS.md)

Day44活動已在main。舊中文活動值的forward已在main；驗收migration仍只在SQLite備份副本執行，不修改已部署migration checksum；散步仍受既有PET_ENABLE_ROAM opt-in及runtime暫停條件限制。Trunk建置須匹配lock的wasm-bindgen；2026-10-31已建置前端release與原生debug，25項隔離原生功能檢查通過；實體桌面與原生release／安裝包仍未驗。見[真實命令](source-review/local-validation-2026-10-30-day44-r2/README.md)。

[Pet 2c7ff8b採用後最新驗證](source-review/pet-day44-adopted-2026-10-30/README.md)；既有Rust單元結果保留4b1008b來源；本輪2c7ff8b原生／瀏覽器新證據另列，不追認舊測試。

[2026-10-31 Gate2原生／HTTP實測與已確認阻擋](source-review/gate2-2026-10-31/README.md)；目前完整Gate2=false，不以CI全綠替代逐房／角色授權或缺失的舊庫／實機證據。
