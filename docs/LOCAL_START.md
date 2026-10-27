# rust_ai_ring 本地操作入口

本頁操作命令已於 2026-10-27 按本地更新後 rust_ai_ring `912e5e77d8e3d2e24f0e9d804f28d86ae768c1d4` 的 workspace manifest、server main／路由及 Tauri 設定核對。Chat／Pet／Analytics 的啟動位置、package、位址與 health 路由符合來源；本次未啟動服務、執行管理員初始化或 GUI 驗收。目錄／package／import 對照見 [SOURCE_LAYOUT](SOURCE_LAYOUT.md)。

本機程式目錄為 `/home/willie/Documents/rust_ai_ring`。在程式庫根目錄依 `.env.example` 設定本機 `.env`；Chat / Pet 使用 SQLite，Analytics 使用 PostgreSQL。資料庫 URL、tokens 與私有資料不放入文件或 Git，SQLite 採絕對路徑可避免因 workspace 切換而連到不同資料檔。

## Chat

```bash
cd /home/willie/Documents/rust_ai_ring/chat_apps
cargo leptos watch --cargo-locked
```

package 為 chat-server / chat-ui；Leptos 輸出名稱仍為 rust_ai_chat。服務位址依目前來源為 http://127.0.0.1:3000，health endpoint 為 /health。

## Pet

```bash
cd /home/willie/Documents/rust_ai_ring/pet_apps/pet_server
NO_COLOR=true cargo tauri dev
```

Cargo package 為 pet-server，Rust import 為 pet_server。Tauri 從 ../pet_ui 啟動 Trunk，前端目錄為 pet_apps/pet_ui；本機 Trunk 如繼承 NO_COLOR=1，改用布林字串 true。資料、IPC 與視窗流程需另作真正 GUI 驗收。

## Analytics

```bash
cd /home/willie/Documents/rust_ai_ring/analytics_apps
cargo run --locked -p analytics-server
```

管理員初始化工具為 `cargo run --locked -p analytics-server --bin bootstrap_admin`；帳密初始化與服務驗收依當時來源核對。服務位址為 http://127.0.0.1:3100，health endpoint 為 /healthz。

[驗證方式](LOCAL_VALIDATION.md) · [架構](SYSTEM_ARCHITECTURE.md) · [儲存庫分工](REPOSITORY_ROLE_TRANSITION.md)
