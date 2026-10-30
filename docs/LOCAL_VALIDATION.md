# 本地驗證

[目前結果與精確 SHA](teaching/current/STATUS.md) 是唯一狀態入口。本頁保留重現方式，執行前重新核對來源、lock、工具與環境；最新Gate2功能步驟與阻擋見[2026-10-31證據](source-review/gate2-2026-10-31/README.md)；以下命令是重現方式，不表示本輪全部重跑。

每個 workspace 分別執行 `cargo metadata --locked --no-deps --format-version 1`、`cargo fmt --all -- --check`、`cargo check --locked --workspace`、`cargo clippy --locked --workspace`、`cargo test --locked --workspace`；或以 `--manifest-path <workspace>/Cargo.toml` 指定。Clippy exit 0 不代表零警告；0 tests 不當功能通過。

Chat 必須額外執行 `cargo test --locked -p chat-server -p chat-persistence -p chat-ui --features ssr` 與 `cargo check --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown`。Pet 執行 WASM check、pet_ui/tests 的 Node 契約測試及真正原生 GUI。Analytics 只對專用隔離 PostgreSQL 執行 `cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration -- --include-ignored`。

記錄每命令的 cwd、exit、來源 SHA、時間、工具版本、features、結果與阻塞。SQLite／PG 的保資料升級、重啟、HTTP／WS／IPC、SSR／瀏覽器、native GUI、CI 各自判定；fresh DB 或靜態 patch 不能代替舊庫驗收。正式 DB、秘密與付費 AI 不在驗證範圍。

[教學規則](COMMON_LEARNING_RULES.md) · [啟動](LOCAL_START.md) · [原始證據分包](archive/packages-2026-10-30/README.md)

Analytics務必`--no-fail-fast -- --include-ignored`揭露所有PG目標；最後4b1008b已採用day37／39登出fixture修正，主分支PG63通過。若多clone共用target，切回不同local crate來源需強制重建並核對編譯來源；本次已保存錯用舊依賴的編譯失敗及修正重試，不將其當功能Red。

[Pet 2c7ff8b採用後最新驗證](source-review/pet-day44-adopted-2026-10-30/README.md)；既有Rust單元結果保留4b1008b來源；本輪2c7ff8b原生／瀏覽器新證據另列，不追認舊測試。

[2026-10-31 Gate2原生／HTTP實測與已確認阻擋](source-review/gate2-2026-10-31/README.md)；目前完整Gate2=false，不以CI全綠替代逐房／角色授權或缺失的舊庫／實機證據。

[64eff52 SQL更新後最新回歸](source-review/gate2-sql-2026-10-31/README.md)：Pet112／Node12／WASM及四workspace metadata通過；舊native／browser證據仍屬2c7ff8b，完整Gate2未通過。
