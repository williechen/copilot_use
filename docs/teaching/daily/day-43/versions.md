# Day43｜2026-10-29 版本與相容性核對（Gate1 文件基線）

資料基準：唯讀 `williechen/rust_ai_ring/main@c6708c42f21ac919e57a379848b4977d27a1c3d6`；四份 workspace 的 `workspace.package.version=0.2.42`、edition 2024。此 Day43 僅增加現有 Leptos／Tauri 模組、signal、測試，**不新增相依、不修改 Cargo.toml／Cargo.lock／migration**。以下主要套件 resolved 與 lock blob 沿用原 [Day42 最新四份 lock 逐項來源](../day-42/versions-2026-10-29.md)，絕不宣稱升級已執行。

| 依賴／工具 | 2026-10-29 官方穩定版 | 現有來源版本 | MSRV／相容性或處理 |
| --- | --- | --- | --- |
| Rust 編譯器 | [1.99.1（2026-10-15）](https://blog.rust-lang.org/releases/latest/) | 未取得使用者本機 `rustc --version` | edition 2024；本輪僅查官方發布，非實際編譯 |
| Leptos | [0.8.22](https://docs.rs/crate/leptos/latest) | Chat／Pet／Analytics 0.8.22 | 本次使用既有 signals、SSR、CSR；SSR／hydrate 真正相容性待產品 Gate2 |
| Leptos Router | [0.8.17](https://docs.rs/crate/leptos_router/latest) | Chat 0.8.17 | SSR 測試保留 Router／RequestUrl fixture；未執行新 patch |
| Axum | [0.8.9](https://docs.rs/crate/axum/latest) | Chat／Analytics 0.8.9 | 與新 URL／SSR 呼叫鏈相接，但本次無 server API 變更 |
| SQLx | [0.9.0](https://docs.rs/crate/sqlx/latest/source/Cargo.toml.orig) | Chat／Pet／Analytics 0.9.0 | 上游 manifest `rust-version=1.94`；保留 Chat／Pet SQLite、Analytics PostgreSQL 隔離 |
| Tokio | [1.54.1（2026-10-21）](https://docs.rs/crate/tokio/latest) | 1.54.1 | 上游 MSRV 1.85；無新的 async runtime 接線 |
| UUID | [1.29.0](https://docs.rs/crate/uuid/latest) | 1.29.0 | Pet 原生選單依既有 `Uuid::parse_str`；既有 UUID feature 保留 |
| Tauri | [2.12.5](https://docs.rs/crate/tauri/latest/source/Cargo.toml) | Pet 2.12.5 | 上游 `rust-version=1.90`；Native WebKitGTK、平台 DLL／IPC 未實測 |
| Tauri CLI | [2.12.4（2026-10-21）](https://docs.rs/crate/tauri-cli/latest) | 使用者本機 CLI 未取得 | 3.x alpha 為預發行、非 stable；需核對平台工具鏈 |
| Argon2 | [0.6.0](https://docs.rs/crate/argon2/latest) | Chat／Analytics 0.6.0 | 既有登入／密碼路徑不改變 |
| tauri-build | [目前 crate 來源](https://docs.rs/crate/tauri-build/latest) | Pet lock 2.7.4 | 無變更；版本與平台 native build 須單獨檢查 |

四份 `Cargo.lock` 源 blob：Chat `c66ae41859b129ed88cb4cf2b4b845f6c6c7a19d`；Pet `d47380ec6b39cec7f2818dfba73464c722e2a208`；Analytics `997b563d093ca2accfc37c2eea169a9ebacffb32`；共用 crates `cd4ff4c9c2c1291b93ad4908b4c2d3824071862d`。宣告版號見各 workspace 的精確 Cargo.toml；三案 Green、Refactor 均不改 manifest／lock，故本次**不需要新增採用、版本遷移或 lock 重算**。變更應先以使用者套用時的真實 compiler、native／WASM target 與 DB 基線重驗。

## Gate 範圍

- **Gate1 文件交付：** 已核對以上 Day43 所直接涉及的 Rust／框架／資料與 CLI 主依賴官方 stable、現有 lock resolved、已知 MSRV 與未核對的執行限制；不以 `docs.rs` 建置代替本機編譯。
- **Gate2 產品驗收待執行：** 四 workspace Cargo metadata／fmt/check/clippy/test、Chat SSR／WASM、Pet Node／SQLite／Tauri 原生 GUI、Analytics SSR／隔離 PostgreSQL／HTTP、真實 CI，以及完整遞迴 MSRV／OS Native 前置需求與舊 DB 升級相容性；每項按套用後精確 SHA 紀錄。
- **安全性：** 無密碼／憑證更動、不執行正式 DB migration、不自動安裝或升級、不修改 `rust_ai_ring`。本次是對已鎖定套件的文件判定，未知的 CLI／遞迴 MSRV 明示未知，不表示相容性通過。

[三案文章](README.md) · [前 Day 來源驗證](../day-42/README.md) · [共用規則](../../../COMMON_LEARNING_RULES.md)
