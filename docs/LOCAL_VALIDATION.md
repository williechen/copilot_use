# 主體程式測試與功能驗收

`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_ring` 是唯一程式、測試、CI 與功能驗收主體。 所有程式測試、CI 與功能驗收只對 rust_ai_ring 精確 SHA 記錄；文件更新不代表重新執行測試。

## 驗證方式

在隔離的 rust_ai_ring checkout，依當時 Cargo／lock／feature 與實際入口選擇 fmt、Clippy、unit／DB／HTTP／WS、SSR／WASM、GUI、重啟與跨層驗收。測試命令、預期／實際、來源 SHA、workflow／job、平台與未覆蓋部分分別揭露。必要工具或資料庫不可用時標示未執行，不以另一庫結果代替。

Analytics postgres-integration 必須涵蓋所有適用 targets 與專用測試資料庫；若當時有 ignored 的 DB 流程，核對其環境條件後用 --include-ignored 執行，不能把 CI 未執行項目算作通過。Chat／Pet SQLite 與 Analytics PostgreSQL 隔離，不操作正式 DB／秘密，不執行付費 AI。

CI 只採 [rust_ai_ring Actions](https://github.com/williechen/rust_ai_ring/actions) 的精確來源與覆蓋。本機測試、該庫 CI、UI／GUI、程式套用及完整功能驗收各自記錄；任何單項 success 不代表全部完成。copilot_use 只做文章／規則／JSON／連結與交付讀回一致性核對，不執行其舊程式驗收。

## 歷史與現行 Gate

舊 copilot_use 512 sockets、gRPC、跨平台安裝／更新與 Release 驗收[完整原文保留](teaching/history/evidence/2026-10-23-repository-roles/before/docs/LOCAL_VALIDATION.md)，只屬原來源，不作 rust_ai_ring 的已完成能力或測試門檻。最新本地正式驗證目標為 `f3f8255b0b701a1b57cb8c0a1737c3caa4319f2d`，結果未全部通過；本輪清理、空庫冷建置、命令與限制見[本輪報告](teaching/history/evidence/2026-10-24-formal-f3f8255/REPORT.md)。

[共同規則](COMMON_LEARNING_RULES.md) · [目前進度與驗證邊界](teaching/current/STATUS.md) · [既有流程尚未調整](REPOSITORY_ROLE_TRANSITION.md)

## 新目錄的驗證命令

命令以 rust_ai_ring 根目錄為基準；這是後續教學的命令入口，不是本次已執行的結果。

```bash
cargo metadata --locked --no-deps --format-version 1 --manifest-path chat_apps/Cargo.toml
cargo metadata --locked --no-deps --format-version 1 --manifest-path pet_apps/Cargo.toml
cargo metadata --locked --no-deps --format-version 1 --manifest-path analytics_apps/Cargo.toml
cargo metadata --locked --no-deps --format-version 1 --manifest-path crates/Cargo.toml
cargo test --locked --manifest-path chat_apps/Cargo.toml --workspace --features chat-server/ssr,chat-ui/ssr
cargo test --locked --manifest-path pet_apps/Cargo.toml --workspace --exclude pet-ui
cargo test --locked --manifest-path crates/Cargo.toml --workspace
cargo check --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --features hydrate --target wasm32-unknown-unknown
cargo check --locked --manifest-path pet_apps/Cargo.toml -p pet-ui --target wasm32-unknown-unknown
```

Analytics 先準備隔離 PostgreSQL 測試資料庫並設定 ANALYTICS_DATABASE_URL，再使用：

```bash
DATABASE_URL="$ANALYTICS_DATABASE_URL" cargo test --locked --manifest-path analytics_apps/Cargo.toml -p analytics-server --features postgres-integration -- --include-ignored
```

Pet 前端 Node tests 在 pet_apps/pet_ui 執行 `node --test`。CI 只引用 rust_ai_ring 的精確 commit / workflow / job；copilot_use 舊 workflows 不作產品驗收。
