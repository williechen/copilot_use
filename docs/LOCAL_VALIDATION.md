# 本地驗證

[目前結果與精確 SHA](teaching/current/STATUS.md) 是唯一狀態入口。本頁保留重現方式，執行前重新核對來源、lock、工具與環境；Day44本次已按精確SHA實跑，結果與未執行項見[本次證據](source-review/local-validation-2026-10-30-day44/README.md)。

每個 workspace 分別執行 `cargo metadata --locked --no-deps --format-version 1`、`cargo fmt --all -- --check`、`cargo check --locked --workspace`、`cargo clippy --locked --workspace`、`cargo test --locked --workspace`；或以 `--manifest-path <workspace>/Cargo.toml` 指定。Clippy exit 0 不代表零警告；0 tests 不當功能通過。

Chat 必須額外執行 `cargo test --locked -p chat-server -p chat-persistence -p chat-ui --features ssr` 與 `cargo check --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown`。Pet 執行 WASM check、pet_ui/tests 的 Node 契約測試及真正原生 GUI。Analytics 只對專用隔離 PostgreSQL 執行 `cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration -- --include-ignored`。

記錄每命令的 cwd、exit、來源 SHA、時間、工具版本、features、結果與阻塞。SQLite／PG 的保資料升級、重啟、HTTP／WS／IPC、SSR／瀏覽器、native GUI、CI 各自判定；fresh DB 或靜態 patch 不能代替舊庫驗收。正式 DB、秘密與付費 AI 不在驗證範圍。

[教學規則](COMMON_LEARNING_RULES.md) · [啟動](LOCAL_START.md) · [原始證據分包](archive/packages-2026-10-30/README.md)

Analytics務必`--no-fail-fast -- --include-ignored`揭露所有PG目標；最新main舊day37／39登出fixture失敗，僅隔離修補版63通過。若多clone共用target，切回不同local crate來源需強制重建並核對編譯來源；本次已保存錯用舊依賴的編譯失敗及修正重試，不將其當功能Red。
