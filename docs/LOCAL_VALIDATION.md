# 主體程式測試與功能驗收

> 2026-10-28 最新固定來源 `c801cb227e4862b8f49b5071faece70ba0b7e9c9`；[最後來源刷新與獨立重跑](source-review/local-validation-2026-10-28-c801cb2/README.md)。下列 e8a6409 與其他舊來源結果保留原 SHA；未完成 GUI／CI／完整版本 Gate，next_day 三案維持 42。


`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_ring` 是唯一程式、測試、CI 與功能驗收主體。 所有程式測試、CI 與功能驗收只對 rust_ai_ring 精確 SHA 記錄；文件更新不代表重新執行測試。

## 驗證方式

在隔離的 rust_ai_ring checkout，依當時 Cargo／lock／feature 與實際入口選擇 fmt、Clippy、unit／DB／HTTP／WS、SSR／WASM、GUI、重啟與跨層驗收。測試命令、預期／實際、來源 SHA、workflow／job、平台與未覆蓋部分分別揭露。必要工具或資料庫不可用時標示未執行，不以另一庫結果代替。

Analytics postgres-integration 必須涵蓋所有適用 targets 與專用測試資料庫；若當時有 ignored 的 DB 流程，核對其環境條件後用 --include-ignored 執行，不能把 CI 未執行項目算作通過。Chat／Pet SQLite 與 Analytics PostgreSQL 隔離，不操作正式 DB／秘密，不執行付費 AI。

CI 只採 [rust_ai_ring Actions](https://github.com/williechen/rust_ai_ring/actions) 的精確來源與覆蓋。本機測試、該庫 CI、UI／GUI、程式套用及完整功能驗收各自記錄；任何單項 success 不代表全部完成。copilot_use 只做文章／規則／JSON／連結與交付讀回一致性核對，不執行其舊程式驗收。

## 本次正式本地驗證（2026-10-27）

[原始快照 912e5e7 的隔離驗證](source-review/local-validation-2026-10-27-912e5e7/README.md)：依使用者指示不套用修正。Chat 59 通過／2 失敗；Pet Rust 86 通過、Node 6 通過／2 失敗；Analytics 專用 PostgreSQL 55 通過（含 ignored）；共用 crates 8 通過。四組 metadata 與兩個 WASM 檢查通過。命令、exit code、stdout／stderr、失敗及未執行原因另存證據；產品驗收與進度狀態保留。

## 歷史與現行 Gate

舊 copilot_use 512 sockets、gRPC、跨平台安裝／更新與 Release 驗收[完整原文保留](teaching/history/evidence/2026-10-23-repository-roles/before/docs/LOCAL_VALIDATION.md)，只屬原來源，不作 rust_ai_ring 的已完成能力或測試門檻。2026-10-24 的歷史正式驗證目標為 `f3f8255b0b701a1b57cb8c0a1737c3caa4319f2d`，當時結果未全部通過；該輪清理、空庫冷建置、命令與限制見[歷史報告](teaching/history/evidence/2026-10-24-formal-f3f8255/REPORT.md)。

[共同規則](COMMON_LEARNING_RULES.md) · [目前進度與驗證邊界](teaching/current/STATUS.md) · [既有流程尚未調整](REPOSITORY_ROLE_TRANSITION.md)

## 四個 workspace 的驗證命令（方案，非本次執行）

2026-10-27 已按 rust_ai_ring `cf18d6f073b3709352f53f5531f4448a492dd3e3` 核對四份 Cargo manifest 的成員目錄；下列 features、測試目標及資料庫條件仍需依當次實際 Cargo package 與測試來源確認，不能僅憑目錄樹視為通過。

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

Analytics 先準備隔離 PostgreSQL 測試資料庫，設定專用 `ANALYTICS_TEST_DATABASE_URL`（不得指向應用資料庫）。server 與 persistence 都須選入，啟用 feature 並不會自動執行依賴套件的 unit tests；ignored 測試亦需要專用資料庫的 CREATE SCHEMA 權限。再使用：

```bash
DATABASE_URL="$ANALYTICS_TEST_DATABASE_URL" cargo test --locked --manifest-path analytics_apps/Cargo.toml -p analytics-server -p analytics-persistence --features analytics-server/postgres-integration,analytics-persistence/postgres-integration -- --include-ignored
```

Pet 前端 Node tests 在 pet_apps/pet_ui 執行 `node --test`。CI 只引用 rust_ai_ring 的精確 commit / workflow / job；copilot_use 舊 workflows 不作產品驗收。
