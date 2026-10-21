# rust_ai_chat 最新 main 驗證與文件同步

2026-10-22（Asia/Taipei）。結論：主要 Day25 修改已存在且部分實測通過，完整驗收仍有缺口。

來源為 `williechen/rust_ai_chat/main a06c0386d892d28021d6327a5962b3ebbd7a2bb3`、workspace 0.2.25；教學基準為 `williechen/copilot_use/teaching/rust-ai-chat-alignment 149f526f90d62c41a1f255bce34846af0740ca0b`，規則 blob `54c734a07a53dc64bb2e873c3b851fc3a4f63668`。170 個 tracked 檔案與來源 Git blob 逐位元組一致。在隔離副本執行，沒有重套教學或修改產品遠端。升級探針是額外測試，與原始測試結果分開記錄。

## 實際驗證

| 項目 | 實際結果 |
|---|---|
| `DATABASE_URL=sqlite::memory: cargo test --locked --workspace` | 137 passed、0 failed、1 ignored；包含 Chat Day25 Session Gate 與 Pet capability 契約；預設 feature 不涵蓋 PG 整合測試 |
| `cargo test --locked -p stats-server --features postgres-integration -- --include-ignored` | 完整原始 targets 可編譯；正確配置兩個測試 DB URL 後，lib 23 與 Day23 2 項通過，Day24 2 項失敗，命令於該 target 停止 |
| 其餘 PG targets 分別執行 | Day25 再次改密碼 1 通過，idempotency_contract 2 通過，postgres_event_store 1 通過；合併不重複 target 計 PG 29 通過、2 失敗 |
| `cargo check --locked -p server --features ssr` | 通過 |
| `cargo check --locked -p ui --target wasm32-unknown-unknown --features hydrate` | 通過 |
| `cargo check --locked -p desktop-pet-frontend --target wasm32-unknown-unknown` | 通過 |
| `cargo fmt --all -- --check` | 失敗：Pet frontend/src/main.rs:135 trailing whitespace，rustfmt internal error |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 失敗：stats_server/src/admin.rs:321 collapsible_if；命令停止，不宣稱其餘 crate 全部檢查完成 |
| `node --test` | 0 passed、3 failed；VM fixture 缺 `URLSearchParams` |

Rust/Cargo 1.99.1、Linux、原生 PostgreSQL 18.6。PG 與 workspace 統計存在重疊，不能相加當不同測試總數。最初 PG 嘗試缺少 ignored test 所需 `ANALYTICS_TEST_DATABASE_URL`，補齊隔離環境後重跑，該測試通過；不把環境錯誤列為產品缺陷。另有外加升級探針初稿編譯／連線錯誤，修正後分開執行，保留原始嘗試紀錄。

## Chat

真 TCP HTTP、fresh SQLite 全部 migration 啟動成功。註冊完成303至獨立完成頁並清 Cookie，不自動導頁；大小寫帳號登入、/account、程序重啟後 Session 仍有效。有效 Session 的 `/room/validation-room` 回200，匿名房間頁回401；再次註冊撤銷目前 Session，大小寫重複帳號409。

仍有缺口：匿名 `GET /api/rooms` 回200（公開清單本身符合需求，但其他操作的身分／房間 ACL 尚未完成），`POST /logout` 回404；`/ws/{room_id}` 沒有接到房間頁的 Session middleware。room_auth 與 signin 仍各自解析 Cookie／雜湊／查 Session；共用登入、登出、Cookie、Session 實作要求尚未完成。未實測 WS 越權情境，不把靜態發現冒稱執行證據。

## Pet

最新 main 已有 `pet-*` capability、`core:event:default`、動態窗標籤測試，生活窗分支只剩一處 PetStatus。Rust 測試與 WASM check 通過。

`frontend/src/component/activity.rs` 仍使用 `🐢·`、`🐢♪`、`🐢♫`、`🐢💤`、`🐢z` 等複合字元，未達動畫每幀單一寵物圖像的既定要求。此 SHA 未重新執行原生三窗 GUI、事件同步、每窗 DOM 計數與200px裁切驗收；不沿用舊 SHA 的 GUI 成功結論。原生 GUI 操作不在本次可用控制介面內，不能以 JSON 字串斷言替代。

## Analytics

隔離 PG 透過真 TTY bootstrap_admin 建立管理員。真 HTTP 驗證受限登入、首次改密碼／Email、舊 Session401、重啟後舊密碼401、新密碼可登入、Dashboard200、再次 GET /change-password200。Day25 整合測試另驗證第二次提交成功、Email更新、全部 Session 撤銷、舊密碼拒絕與新密碼可登入，通過。

Day24 原始測試仍失敗：

- `day24_password_change.rs:72` 預期403、實際422。測試 form 使用 `mail`，handler 要求 `email`，在進入 CSRF 驗證前即表單解碼失敗。
- `day24_password_change.rs:242` 的 SQL 有 `Other'` 缺少左引號，實際 PostgreSQL 回42601。另第134行 `active'` 也缺左引號，前面斷言失敗時尚未執行到它。

缺失的 `20261021071000_unique_account_email.sql` 已在 main 補上，因此不再以「缺 SQL 檔不能編譯」描述最新版本。沒有為讓測試通過而改寫原始碼或歷史 migration。

## 舊資料庫升級

以精確歷史 migration 建立隔離庫，插入一筆寵物／管理員，再用目前 Migrator 升級：

| 系統／來源歷史 | 結果 | 原資料 |
|---|---|---|
| Pet `4f0cd00b8d571bdaf120298efebf0d62b795498f` | VersionMismatch(20261018095626) | 1筆保留 |
| Pet Day24 `08b891a22c1934a2bf0b1edbb103c9b16bcf12de` | 升級成功 | 1筆保留 |
| Analytics `4f0cd00b8d571bdaf120298efebf0d62b795498f` | 升級成功 | 1筆保留 |
| Analytics Day24 `08b891a22c1934a2bf0b1edbb103c9b16bcf12de` | VersionMismatch(20261018073932) | 1筆保留 |

這證明升級相容性依真實 DB 歷史而異，不能以 fresh DB 或一條升級路徑的成功宣稱全部修復。未讀取或修改使用者既有資料庫，未重寫 checksum。

## 套件與 CI

本輪只核對精確 Cargo.lock：Axum0.8.9、SQLx0.9.0、Tokio1.54.0、Argon2 0.6.0、SHA2 0.11.1（另有傳遞依賴0.10.9）、Tauri2.12.4、tauri-build2.7.3、Leptos0.8.22、UUID1.28.0。未宣稱上述均為官方最新穩定版；沒有更新 lock、MSRV 或跨平台驗收。

同一精確 main SHA：

- [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39636430617)：success，9個 Rust test jobs成功。
- [Rust AI Chat](https://github.com/williechen/rust_ai_chat/actions/runs/39636430046)、[Rust Desktop Pet](https://github.com/williechen/rust_ai_chat/actions/runs/39636430301)、[Rust Dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39636430030)、[Persistence](https://github.com/williechen/rust_ai_chat/actions/runs/39636430406)：success。
- [Node](https://github.com/williechen/rust_ai_chat/actions/runs/39636430708)：failure。

workflow/job狀態、完整 SHA、UTC起訖時間與連結保存在 `ci-final.json`。CI 不涵蓋本輪全部 PG、舊庫、GUI或共用登入要求。

## Notion、進度與排程

三篇 Day25 Notion 全文與 GitHub 封存 SHA256逐字一致；封存正文未改，Chat／Pet／Analytics next_day 均維持26。更新共同主頁與三案入口的目前進度、缺口及證據入口；保留封存文章、既有補教、last_full_comparison。沒有發布新 Day或將測試成功當作教學進度遞增。

既有 [三專案每日 TDD 教學](https://chatgpt.com/scheduled?automationId=6acd967c15fc8191ade8407954c5b354&automationSource=cloud) 提示詞已原地更新，重新載入後完整文字一致；保留啟用狀態、每日06:00、Asia/Taipei。已寫入要求模型 `gpt-6.1-sol`、推理 `medium`，以及每次實讀共同主頁、封存到Day25、動態讀進度、共用登入、三窗／動畫及完整PG測試要求。沒有建立重複任務，沒有觸發另一輪教學發布。

**實際模型切換尚未完成：** 任務原對話選單目前只有 GPT-6 與 GPT-5.6 Sol，沒有 GPT-6.1 Sol；原推理強度為「中」。不能以提示詞保存宣稱引擎已切換，也不能以此保證下一次排程結果。[OpenAI Docs 的 GPT-6.1 Sol 模型頁](https://developers.openai.com/api/docs/models/gpt-6.1-sol)確認模型識別與medium推理支援，但不證明此雲端任務介面可選到該模型。

專用 PostgreSQL55449、SQLite與測試憑證僅供本輪使用；測試完成後停止專用服務並清理隔離資料。既有5432、產品資料、使用者未提交文件及來源庫程式保留。未執行付費AI、跨平台或完整產品驗收。
