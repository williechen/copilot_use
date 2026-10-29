# 2026-10-28 最後來源刷新與獨立重跑

精確產品來源 `c801cb227e4862b8f49b5071faece70ba0b7e9c9`；測試期間遠端由 e8a6409 前進後，重新 fetch、乾淨 main ff-only，另開 clone 實測；不將舊 SHA 結果追認。版本 0.2.42；四 workspace manifest／lock 與完整 tracked tree 保存於本目錄。

[前輪原文 patch／修正候選／工具版本／清理與完整證據](../local-validation-2026-10-28-e8a6409/README.md) 保留其原 SHA。原六份 patch 的適用基準仍為 0b2288e，不對最新 main 重複套用。以下為新 SHA 原樣結果，exit 0 只限該命令。

| 命令 | exit |
|---|---:|
| `cargo fetch --locked --manifest-path crates/Cargo.toml` | 0 |
| `cargo metadata --no-deps --format-version 1 --locked --manifest-path crates/Cargo.toml` | 0 |
| `cargo fmt --all --manifest-path crates/Cargo.toml -- --check` | 0 |
| `cargo check --workspace --locked --manifest-path crates/Cargo.toml` | 0 |
| `cargo clippy --workspace --locked --manifest-path crates/Cargo.toml` | 0 |
| `cargo test --workspace --locked --manifest-path crates/Cargo.toml` | 0 |
| `cargo fetch --locked --manifest-path chat_apps/Cargo.toml` | 0 |
| `cargo metadata --no-deps --format-version 1 --locked --manifest-path chat_apps/Cargo.toml` | 0 |
| `cargo fmt --all --manifest-path chat_apps/Cargo.toml -- --check` | 0 |
| `cargo check --workspace --locked --manifest-path chat_apps/Cargo.toml` | 0 |
| `cargo clippy --workspace --locked --manifest-path chat_apps/Cargo.toml` | 0 |
| `cargo test --workspace --locked --manifest-path chat_apps/Cargo.toml` | 0 |
| `cargo fetch --locked --manifest-path pet_apps/Cargo.toml` | 0 |
| `cargo metadata --no-deps --format-version 1 --locked --manifest-path pet_apps/Cargo.toml` | 0 |
| `cargo fmt --all --manifest-path pet_apps/Cargo.toml -- --check` | 0 |
| `cargo check --workspace --locked --manifest-path pet_apps/Cargo.toml` | 0 |
| `cargo clippy --workspace --locked --manifest-path pet_apps/Cargo.toml` | 0 |
| `cargo test --workspace --locked --manifest-path pet_apps/Cargo.toml` | 0 |
| `cargo fetch --locked --manifest-path analytics_apps/Cargo.toml` | 0 |
| `cargo metadata --no-deps --format-version 1 --locked --manifest-path analytics_apps/Cargo.toml` | 0 |
| `cargo fmt --all --manifest-path analytics_apps/Cargo.toml -- --check` | 0 |
| `cargo check --workspace --locked --manifest-path analytics_apps/Cargo.toml` | 0 |
| `cargo clippy --workspace --locked --manifest-path analytics_apps/Cargo.toml` | 0 |
| `cargo test --workspace --locked --manifest-path analytics_apps/Cargo.toml` | 0 |
| `node --test pet_apps/pet_ui/tests/day33-open-controls-selects-origin.test.mjs pet_apps/pet_ui/tests/duplicate-preview-contract.test.mjs pet_apps/pet_ui/tests/scan-preview-contract.test.mjs pet_apps/pet_ui/tests/day41-cage-by-id.test.mjs pet_apps/pet_ui/tests/scan-preview-no-path.test.mjs pet_apps/pet_ui/tests/day42-cage-status.test.mjs` | 1 |
| `cargo check --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --target wasm32-unknown-unknown --features hydrate` | 0 |
| `cargo check --locked --manifest-path pet_apps/Cargo.toml -p pet-ui --target wasm32-unknown-unknown` | 0 |
| `cargo test --locked --manifest-path chat_apps/Cargo.toml -p chat-server -p chat-persistence -p chat-ui --features ssr` | 101 |
| `cargo test --locked --manifest-path analytics_apps/Cargo.toml -p analytics-server -p analytics-persistence --features postgres-integration -- --include-ignored` | 0 |
| `/usr/lib/postgresql/18/bin/pg_ctl -D /home/willie/Documents/Codex/2026-10-28/referenced-chatgpt-conversation-this-is-an/work/pg stop` | 0 |

四 workspace metadata／fmt／check／clippy／預設 test、兩案 WASM、Chat SSR 與隔離 PG 分開看，不以預設 test 代替 SSR 或 PG feature。Chat SSR 原登入測試仍缺 Router fixture；Pet Node 仍有 location fixture 問題。前輪候選修正的成功只屬前輪來源；未自動套入產品。

原生 GUI、瀏覽器、CI rerun、舊 DB 升級與完整 MSRV／官方版本 Gate 未執行；Gate2 未通過。next_day 三案 42、FIFO、last_full_comparison 不變。產品不提交／推送。Pet SQLx 與 Analytics Router 測試僅觸及隔離資料庫；自建 PG 已停止。

最新 SHA 的隔離副本再實際 check／apply chat-signin-fixture-fix.patch 與 pet-final-fixture.patch，Chat SSR UI 18 項及 Pet Node 10 項全通過；完整命令與輸出在 final-candidate-fixes.json。這是新 SHA 加候選 fixture 的結果，原樣 main 的失敗仍保留。未提交或推送產品。另清理本輪隔離副本的兩個 WASM target 子目錄，釋放 260141056 bytes，見 own-wasm-cleanup.json。
