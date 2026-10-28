# 2026-10-28 本地正式驗證紀錄

產品來源 `e8a640978732651cbbca1483b07e771f876a439b`；Day42 patch 基準 `0b2288e4996b6d80b00e7a77e2cd023835306528`；文件預期 parent `8dde44d85c803040fad17a759394c370d950282b`。

兩庫原 main 乾淨，無未提交／未追蹤使用者檔；fetch 後均 ff-only。產品由 `912e5e77d8e3d2e24f0e9d804f28d86ae768c1d4` 快轉，文件由 `ecdd138a444cdd25ca3d959b979624c1bbb5b8fa` 快轉。產品不提交／不推送，測試在獨立 clone。原版文章／patch／交付證據不改寫。

七個 target 僅含空子目錄，清理前逐項確認無檔案／symlink／tracked path；釋放配置空間 188416 bytes。[實際清單](cleanup.json)。未清 Cargo registry、資料庫、.env 或 lock。

下表每行只代表該命令 exit code，不代表功能驗收；124 為限時，101 為 Cargo 失敗。完整 stdout／stderr、命令、cwd、耗時在 [commands.json](commands.json) 與 [extra-commands.json](extra-commands.json)。latest 是最新產品原樣；day42 是原文基準按各案 Refactor → Green 實際套用的候選。

| 副本 | 命令 | exit |
|---|---|---:|
| referenced-chatgpt-conversation-this-is-an | `cargo -V` | 0 |
| referenced-chatgpt-conversation-this-is-an | `node --version` | 0 |
| latest | `cargo metadata --locked --no-deps --format-version 1 --manifest-path crates/Cargo.toml` | 0 |
| latest | `cargo fmt --all --manifest-path crates/Cargo.toml -- --check` | 1 |
| latest | `cargo check --offline --locked --workspace --manifest-path crates/Cargo.toml` | 0 |
| latest | `cargo clippy --offline --locked --workspace --manifest-path crates/Cargo.toml` | 0 |
| latest | `cargo test --offline --locked --workspace --manifest-path crates/Cargo.toml` | 0 |
| latest | `cargo metadata --locked --no-deps --format-version 1 --manifest-path chat_apps/Cargo.toml` | 0 |
| latest | `cargo fmt --all --manifest-path chat_apps/Cargo.toml -- --check` | 1 |
| latest | `cargo check --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 101 |
| latest | `cargo clippy --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 101 |
| latest | `cargo test --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 101 |
| latest | `cargo metadata --locked --no-deps --format-version 1 --manifest-path pet_apps/Cargo.toml` | 0 |
| latest | `cargo fmt --all --manifest-path pet_apps/Cargo.toml -- --check` | 1 |
| latest | `cargo check --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 0 |
| latest | `cargo clippy --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 0 |
| latest | `cargo test --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 124 |
| latest | `cargo metadata --locked --no-deps --format-version 1 --manifest-path analytics_apps/Cargo.toml` | 0 |
| latest | `cargo fmt --all --manifest-path analytics_apps/Cargo.toml -- --check` | 1 |
| latest | `cargo check --offline --locked --workspace --manifest-path analytics_apps/Cargo.toml` | 0 |
| latest | `cargo clippy --offline --locked --workspace --manifest-path analytics_apps/Cargo.toml` | 0 |
| latest | `cargo test --offline --locked --workspace --manifest-path analytics_apps/Cargo.toml` | 0 |
| latest | `node --test pet_apps/pet_ui/tests/day33-open-controls-selects-origin.test.mjs pet_apps/pet_ui/tests/duplicate-preview-contract.test.mjs pet_apps/pet_ui/tests/scan-preview-contract.test.mjs pet_apps/pet_ui/tests/day41-cage-by-id.test.mjs pet_apps/pet_ui/tests/scan-preview-no-path.test.mjs pet_apps/pet_ui/tests/day42-cage-status.test.mjs` | 1 |
| latest | `cargo check --offline --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --target wasm32-unknown-unknown --features hydrate` | 0 |
| latest | `cargo check --offline --locked --manifest-path pet_apps/Cargo.toml -p pet-ui --target wasm32-unknown-unknown` | 0 |
| latest | `cargo test --offline --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --features ssr` | 101 |
| day42 | `git apply --check /home/willie/Documents/copilot_use/docs/teaching/daily/day-42/chat-day42-refactor.patch` | 0 |
| day42 | `git apply /home/willie/Documents/copilot_use/docs/teaching/daily/day-42/chat-day42-refactor.patch` | 0 |
| day42 | `git apply --check /home/willie/Documents/copilot_use/docs/teaching/daily/day-42/chat-day42-green.patch` | 0 |
| day42 | `git apply /home/willie/Documents/copilot_use/docs/teaching/daily/day-42/chat-day42-green.patch` | 0 |
| day42 | `git apply --check /home/willie/Documents/copilot_use/docs/teaching/daily/day-42/pet-day42-refactor.patch` | 0 |
| day42 | `git apply /home/willie/Documents/copilot_use/docs/teaching/daily/day-42/pet-day42-refactor.patch` | 0 |
| day42 | `git apply --check /home/willie/Documents/copilot_use/docs/teaching/daily/day-42/pet-day42-green.patch` | 0 |
| day42 | `git apply /home/willie/Documents/copilot_use/docs/teaching/daily/day-42/pet-day42-green.patch` | 0 |
| day42 | `git apply --check /home/willie/Documents/copilot_use/docs/teaching/daily/day-42/analytics-day42-refactor.patch` | 0 |
| day42 | `git apply /home/willie/Documents/copilot_use/docs/teaching/daily/day-42/analytics-day42-refactor.patch` | 0 |
| day42 | `git apply --check /home/willie/Documents/copilot_use/docs/teaching/daily/day-42/analytics-day42-green.patch` | 0 |
| day42 | `git apply /home/willie/Documents/copilot_use/docs/teaching/daily/day-42/analytics-day42-green.patch` | 0 |
| day42 | `cargo metadata --locked --no-deps --format-version 1 --manifest-path crates/Cargo.toml` | 0 |
| day42 | `cargo fmt --all --manifest-path crates/Cargo.toml -- --check` | 1 |
| day42 | `cargo check --offline --locked --workspace --manifest-path crates/Cargo.toml` | 0 |
| day42 | `cargo clippy --offline --locked --workspace --manifest-path crates/Cargo.toml` | 0 |
| day42 | `cargo test --offline --locked --workspace --manifest-path crates/Cargo.toml` | 0 |
| day42 | `cargo metadata --locked --no-deps --format-version 1 --manifest-path chat_apps/Cargo.toml` | 0 |
| day42 | `cargo fmt --all --manifest-path chat_apps/Cargo.toml -- --check` | 1 |
| day42 | `cargo check --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 101 |
| day42 | `cargo clippy --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 101 |
| day42 | `cargo test --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 101 |
| day42 | `cargo metadata --locked --no-deps --format-version 1 --manifest-path pet_apps/Cargo.toml` | 0 |
| day42 | `cargo fmt --all --manifest-path pet_apps/Cargo.toml -- --check` | 1 |
| day42 | `cargo check --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 0 |
| day42 | `cargo clippy --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 0 |
| day42 | `cargo test --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 0 |
| day42 | `cargo metadata --locked --no-deps --format-version 1 --manifest-path analytics_apps/Cargo.toml` | 0 |
| day42 | `cargo fmt --all --manifest-path analytics_apps/Cargo.toml -- --check` | 1 |
| day42 | `cargo check --offline --locked --workspace --manifest-path analytics_apps/Cargo.toml` | 0 |
| day42 | `cargo clippy --offline --locked --workspace --manifest-path analytics_apps/Cargo.toml` | 0 |
| day42 | `cargo test --offline --locked --workspace --manifest-path analytics_apps/Cargo.toml` | 0 |
| day42 | `node --test pet_apps/pet_ui/tests/day33-open-controls-selects-origin.test.mjs pet_apps/pet_ui/tests/duplicate-preview-contract.test.mjs pet_apps/pet_ui/tests/scan-preview-contract.test.mjs pet_apps/pet_ui/tests/day41-cage-by-id.test.mjs pet_apps/pet_ui/tests/scan-preview-no-path.test.mjs pet_apps/pet_ui/tests/day42-cage-status.test.mjs` | 1 |
| day42 | `cargo check --offline --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --target wasm32-unknown-unknown --features hydrate` | 101 |
| day42 | `cargo check --offline --locked --manifest-path pet_apps/Cargo.toml -p pet-ui --target wasm32-unknown-unknown` | 0 |
| day42 | `cargo test --offline --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --features ssr` | 101 |
| latest | `cargo test --offline --locked --manifest-path analytics_apps/Cargo.toml -p analytics-server -p analytics-persistence --features postgres-integration -- --include-ignored` | 0 |
| latest | `cargo test --offline --locked --manifest-path chat_apps/Cargo.toml -p chat-server -p chat-persistence -p chat-ui --features ssr` | 101 |
| day42 | `cargo test --offline --locked --manifest-path analytics_apps/Cargo.toml -p analytics-server -p analytics-persistence --features postgres-integration -- --include-ignored` | 0 |
| day42 | `cargo test --offline --locked --manifest-path chat_apps/Cargo.toml -p chat-server -p chat-persistence -p chat-ui --features ssr` | 101 |
| latest | `cargo check --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 0 |
| latest | `cargo clippy --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 0 |
| latest | `cargo test --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 0 |
| day42 | `cargo test --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 0 |

尚未執行：原生 GUI 人工操作、瀏覽器完整情境、GitHub CI rerun、正式舊 DB 升級、完整 MSRV matrix、官方最新版本全面查核。Node mock 不代表 native／SQLite 已驗收；metadata 不代表 build。測試缺依賴先記失敗後 locked fetch，後續重試獨立列示，未修改 lock。

產品已見三案 Day42 caller 與各案 Refactor，不能再說最新產品全部未套用；也不能由靜態 caller 判定 Gate2。Chat 切房競態、Pet 預設資格與回籠旗標混用、Analytics 角色／來源 ACL 原完成條件保留。FIFO、next_day 三案 42、last_full_comparison 不變。版本 Gate 未全面通過，不推進 Day43。

原文 Chat patch 雖能 apply，但 component attribute 錯掛到 room_heading，候選 SSR／WASM 編譯失敗。原 Refactor → Green 後再套 chat-day42-component-fix.patch，WASM 通過；再套 chat-signin-fixture-fix.patch，SSR UI 18 項通過。最新產品已修 attribute，僅需最新基準修正附件。這些候選結果不追認原樣通過。

Chat 原樣 SSR 兩項登入測試因 Router context 缺失失敗；獨立修正候選補 Owner、Router、RequestUrl，18 項 UI 測試通過。候選四 workspace fmt 全通過（fmt-fix-exits.txt），原樣 fmt 仍記失敗。

最新 Pet Node 原樣 6 通過／4 失敗；fixture 修正候選 10 通過。保留所有斷言，補 location 並 structuredClone 跨 VM args；[修正 patch](pet-day42-fixture-fix.patch)。[Chat 格式修正](chat-format-fix.patch) 是最新產品的獨立候選，不改原文 Green。
