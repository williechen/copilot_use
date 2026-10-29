# 2026-10-27 正式本地驗證：原始快照 912e5e7

本輪已完成預定的隔離自動化驗證，**結果未全通過**。依使用者「不修正」指示，沒有套用任何產品、測試或教學 patch；四項已知失敗仍重現。

來源固定為 rust_ai_ring `912e5e77d8e3d2e24f0e9d804f28d86ae768c1d4`。文件更新前 parent 為 copilot_use `45b46d6ca20799072f67f71a4753c4cac7e66a6d`。以該程式 commit 的 `git archive` 建立隔離副本，排除本機 .env、資料庫與未知檔案；沿用外部 Cargo 建置快取，並非冷建置。日期依 Asia/Taipei。

## 結果

| 驗證項目 | 通過 | 失敗 | ignored | exit code |
|---|---:|---:|---:|---:|
| Chat workspace（SSR，含 doc-tests） | 59 | 2 | 0 | 101 |
| Pet workspace（排除 pet-ui，含 doc-tests） | 86 | 0 | 0 | 0 |
| Pet Node | 6 | 2 | 0 | 1 |
| Analytics 預設 workspace | 36 | 0 | 1 | 0 |
| Analytics 專用 PostgreSQL（server＋persistence，含 ignored） | 55 | 0 | 0 | 0 |
| 共用 crates | 8 | 0 | 0 | 0 |

四組 `cargo metadata --locked --no-deps --format-version 1` 皆 exit 0。Chat hydrate 與 Pet UI 的 `wasm32-unknown-unknown` 編譯檢查皆 exit 0。metadata 的 no-deps 結果不代表完整依賴圖或產品功能驗收。各列是該次命令結果；Analytics 預設與 PG 測試存在重疊，不能相加為獨立測試總數。

## 四項失敗

- Chat：`signin_contract::day28_leptos_signin_posts_to_existing_login_handler` 與 `signin_contract::existing_signin_is_the_single_login_form`。`SignInPage` 使用 `use_query_map()`，測試直接渲染頁面而未提供 Router context；panic 為 `Tried to access reactive URL outside a <Router> component.`。正式 App 有 Router，這兩項失敗不能單獨判定正式登入流程壞掉，也不能視為登入驗收通過。
- Pet Node：`control can cage one explicit UUID without changing selection` 與 `native cage error reaches control caller`。Day41 測試 VM 只有 `window.location`，未提供腳本需要的全域 `location`，初始化即發生 `ReferenceError: location is not defined`，尚未進入回籠功能斷言。

本輪保留上述失敗，沒有修改測試、吞掉錯誤或忽略失敗項目。

## PostgreSQL 隔離驗證

本輪另以本機 PostgreSQL 18.6 建立全新、任務專用 cluster。只開啟私人暫存 Unix socket，停用 TCP 監聽；沒有連接或遷移任何既有應用資料庫。測試角色 `codex_validation` 只用於此 cluster；server 與 persistence 均選入並啟用 postgres-integration，執行 `--include-ignored`。55 項通過、0 failed、0 ignored。測試後 `pg_ctl stop` exit 0；資料目錄留在任務 work/ 供稽核，不屬於產品資料。

## 工具、命令與證據

Rust／Cargo 1.99.1、Node v22.22.1；WASM target 已安裝。Node 與產品 CI 的 Node 24 不同，本輪不宣稱與 CI 環境完全相同。所有 Cargo 指令使用 `--locked`；未更動任何 Cargo.lock。Rust 測試使用 `--no-fail-fast`，讓其他 targets 與 doc-tests 繼續執行。每個命令上限 600 秒，本輪無超時。

[逐項命令與摘要](commands.json) 保存 cwd、環境覆寫、exit code、耗時、stdout／stderr 檔名與摘要；[完整證據包](evidence.zip) 保存原始 stdout／stderr、PG 日誌、環境、結果、保護檢查與執行腳本。資料庫 URL 僅指向本輪臨時 cluster，不含既有帳密或 .env 內容。

| 命令 | exit code |
|---|---:|
| `rustc --version` | 0 |
| `cargo --version` | 0 |
| `node --version` | 0 |
| `rustup target list --installed` | 0 |
| `cargo metadata --locked --no-deps --format-version 1 --manifest-path chat_apps/Cargo.toml` | 0 |
| `cargo metadata --locked --no-deps --format-version 1 --manifest-path pet_apps/Cargo.toml` | 0 |
| `cargo metadata --locked --no-deps --format-version 1 --manifest-path analytics_apps/Cargo.toml` | 0 |
| `cargo metadata --locked --no-deps --format-version 1 --manifest-path crates/Cargo.toml` | 0 |
| `node --test` | 1 |
| `cargo test --locked --manifest-path chat_apps/Cargo.toml --workspace --features chat-server/ssr,chat-ui/ssr --no-fail-fast` | 101 |
| `cargo test --locked --manifest-path pet_apps/Cargo.toml --workspace --exclude pet-ui --no-fail-fast` | 0 |
| `cargo test --locked --manifest-path analytics_apps/Cargo.toml --workspace --no-fail-fast` | 0 |
| `cargo test --locked --manifest-path crates/Cargo.toml --workspace --no-fail-fast` | 0 |
| `/usr/lib/postgresql/18/bin/initdb -D /home/willie/Documents/Codex/2026-10-27/referenced-chatgpt-conversation-this-is-an/work/formal-validation-912e5e7/pgdata -U codex_validation -A trust --no-locale --encoding=UTF8` | 0 |
| `/usr/lib/postgresql/18/bin/pg_ctl -D /home/willie/Documents/Codex/2026-10-27/referenced-chatgpt-conversation-this-is-an/work/formal-validation-912e5e7/pgdata -l /home/willie/Documents/Codex/2026-10-27/referenced-chatgpt-conversation-this-is-an/outputs/formal-validation-912e5e7/postgres.log -o '-F -k /tmp/codex-verify-pg-e7hn96tg -h '"'"''"'"' -p 55437' -w start` | 0 |
| `psql 'postgresql://codex_validation@localhost:55437/postgres?host=/tmp/codex-verify-pg-e7hn96tg' -X -v ON_ERROR_STOP=1 -c 'SELECT version(), current_user, current_database();'` | 0 |
| `cargo test --locked --manifest-path analytics_apps/Cargo.toml -p analytics-server -p analytics-persistence --features analytics-server/postgres-integration,analytics-persistence/postgres-integration --no-fail-fast -- --include-ignored` | 0 |
| `/usr/lib/postgresql/18/bin/pg_ctl -D /home/willie/Documents/Codex/2026-10-27/referenced-chatgpt-conversation-this-is-an/work/formal-validation-912e5e7/pgdata -m fast -w stop` | 0 |
| `cargo check --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --features hydrate --target wasm32-unknown-unknown` | 0 |
| `cargo check --locked --manifest-path pet_apps/Cargo.toml -p pet-ui --target wasm32-unknown-unknown` | 0 |

## 未執行與保護範圍

- Chat／Analytics 實際瀏覽器、Pet GUI／重啟／跨層完整產品驗收：本次限隔離自動化測試；未啟動使用者服務或操作使用者資料。PG Router integration 通過亦不等同瀏覽器驗收。 exit code 為 null（未執行）。
- GitHub CI：本次沒有觸發或讀取產品 CI，不引用既有 CI 為本輪通過。 exit code 為 null（未執行）。
- 產品／測試修正、教學 patch 套用：使用者指定不修正；原始 SHA 原樣驗證。 exit code 為 null（未執行）。

原始 rust_ai_ring 工作樹仍乾淨；所有 tracked 檔案、.env 與既有資料庫的記錄 hash 保持不變。隔離副本的既有來源檔也保持原樣；Tauri 編譯自動產生四份 `pet_server/gen/schemas/*.json`，僅位於隔離副本。

驗證驅動程式在所有測試完成後，最初因新增的 schema 檔而使保存檢查 exit 1；已逐一分類四份生成檔，重新確認既有檔案未變，保存檢查重跑 exit 0。此檢查錯誤與上述測試命令的 exit code 分別記錄，未把失敗改記為成功。

本次只更新驗證文章及其導覽，不更動 next_day、manifest、last_full_comparison 或既有產品驗收狀態；不以本輪結果改寫歷史測試、教學交付或完整產品驗收結論。
