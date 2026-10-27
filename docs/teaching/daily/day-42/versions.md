# Day42 版本與 Cargo.lock 追加核對（2026-10-28）

這是三篇已提交 Day42 文章的**追加版本證據**，不覆寫已發布原文，也不代表編譯／MSRV／native／DB／GUI 驗收。產品來源固定為 [rust_ai_ring@0b2288e](https://github.com/williechen/rust_ai_ring/tree/0b2288e4996b6d80b00e7a77e2cd023835306528)；四份 workspace manifest 的 `workspace.package.version` 均為 `0.2.41`，根目錄沒有 Cargo workspace。

| Workspace | Cargo.lock blob SHA | 已讀回 resolved 版本 |
| --- | --- | --- |
| Chat | `916dc5dbc118f7af1f7fe94f32ba699b7a0c5680` | axum 0.8.9、Leptos 0.8.22、leptos_router 0.8.17、SQLx 0.9.0、Tokio 1.54.1、UUID 1.29.0、Argon2 0.6.0 |
| Pet | `436666203bfe523433b2d01b5489106f8c5457f2` | Tauri 2.12.5、tauri-build 2.7.4、Leptos 0.8.22、SQLx 0.9.0、Tokio 1.54.1、UUID 1.29.0 |
| Analytics | `050f912fcbe05b1c93ee845fb780bd84246c90da` | axum 0.8.9、Leptos 0.8.22、SQLx 0.9.0、Tokio 1.54.1、UUID 1.29.0 |
| 共用 crates | `7360b9c3a7a783fbea531eea30dfe9516166c52a` | Tokio 1.54.1、UUID 1.29.0、Serde 1.0.229 |

2026-10-28 官方穩定版核對：[Rust 1.99.1（2026-10-15）](https://doc.rust-lang.org/releases.html)、[Tauri 2.12.5](https://docs.rs/crate/tauri/latest)、[Leptos 0.8.22](https://docs.rs/crate/leptos/latest)、[Axum 0.8.9](https://docs.rs/crate/axum/latest)、[SQLx 0.9.0](https://docs.rs/crate/sqlx/latest)、[Tokio 1.54.1](https://docs.rs/crate/tokio/latest)、[UUID 1.29.0](https://docs.rs/crate/uuid/latest)。均與所列 resolved 版本一致。Rust 1.99.1 是對上一篇 1.99.0 的正式更正；沒有變更 lock、安裝或產品原始碼。

**仍待確認**：全部直接／遞迴套件 MSRV、實際 `rustc --version`、WASM target、Tauri CLI／WebKitGTK/OS native library、隔離 PostgreSQL／SQLite、真實 Cargo build/test。此時只能判定「官方版本與四份 lock 的主要套件已讀回」，不能判定完整版本 Gate／Gate1／Gate2 通過。

[完整 Day42 交付 blob/SHA256 與靜態 patch 驗證](delivery-verification.json)。
