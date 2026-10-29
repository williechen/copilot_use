# rust_ai_ring 本地操作入口

> **2026-10-30 SSR 修補追驗：** 產品已更新至 `7ce39f0166a2120539e5908b33c88322a930c35a`，正式 main 已套用 Router imports。隔離原樣測試 UI SSR 21 項、server／SQLite／UI SSR 組合 60 項及 metadata／fmt 通過。[新 SHA 命令與 CI 快照](source-review/day43-7ce39f0-2026-10-30/README.md)；遠端 CI 仍執行中，未稱全綠。其他 workspace 與 GUI／WASM／DB 保留各自原 SHA 的證據；next_day 三案 44、FIFO、last_full_comparison 不變，完整 Gate2 未結案。下方較早日期與 SHA 為歷史快照。

> 2026-10-28 固定產品來源 `c6708c42f21ac919e57a379848b4977d27a1c3d6`；[本輪原始命令與結果](source-review/local-validation-2026-10-28-c6708c4/README.md)。本輪只增量重測 fixture、metadata／fmt；其他來源實測維持原 SHA，完整驗收與版本 Gate 尚未完成。


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
