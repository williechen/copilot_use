# 當日版本查核與現行摘要更新

核對日期：2026-10-26（Asia/Taipei）；UTC完成時間 `2026-10-25T21:35:51.485190+00:00`。程式來源 `648b90323af7b381d6a79dc2fd08a53b63173a43`；文件讀取基準 `f1cdfdc3ce3e986e1bab381be54c783b75191dd0`。本次為文件維護，不交付新教學、不推進三案 next_day=38、不改產品／lock／migration／FIFO／last_full_comparison；沒有執行產品建置、測試、DB或GUI驗收。

## 官方穩定版本

重新取得 crates.io 官方發布 metadata，取數字版本最高且未 yanked、非 alpha／beta／RC 的穩定版。四workspace所有外部直接 normal／build／dev／target依賴均列入查核；path dependency屬本庫程式，不假造外部版本。逐manifest、section、宣告、resolved與官方MSRV及原始回應SHA256在 [機器證據](version-refresh-2026-10-26/evidence.json)。MSRV缺值表示官方未宣告，不推論為無限制。

Rust stable：[1.99.1 (58cf8f9c3 2026-10-13)](https://static.rust-lang.org/dist/channel-rust-stable.toml)，官方channel日期 2026-10-15；本機 `rustc 1.99.1 (58cf8f9c3 2026-10-13)`、`cargo 1.99.1 (5f94df478 2026-08-27)`。版本數字相符；沒有執行安裝或升級。

| 套件／官方來源 | 最新穩定版 | 官方rust-version | manifest宣告範圍 | Cargo.lock resolved（各workspace） |
| --- | --- | --- | --- | --- |
| [any_spawner](https://crates.io/api/v1/crates/any_spawner) | 0.3.0 | 官方 metadata 未宣告 | 0.3 | chat:0.3.0；pet:0.3.0；analytics:0.3.0 |
| [argon2](https://crates.io/api/v1/crates/argon2) | 0.6.0 | 1.85 | 0.6 | chat:0.6.0；analytics:0.6.0 |
| [async-trait](https://crates.io/api/v1/crates/async-trait) | 0.1.92 | 1.71 | 0.1 | chat:0.1.92；pet:0.1.92；analytics:0.1.92；crates:0.1.92 |
| [axum](https://crates.io/api/v1/crates/axum) | 0.8.9 | 1.80 | 0.8 | chat:0.8.9；analytics:0.8.9 |
| [cargo-leptos](https://crates.io/api/v1/crates/cargo-leptos) | 0.3.11 | 1.82.0 | CLI／間接依賴 | 不在產品 lock，見工具表 |
| [dotenv](https://crates.io/api/v1/crates/dotenv) | 0.15.0 | 官方 metadata 未宣告 | 0.15 | chat:0.15.0；pet:0.15.0；analytics:0.15.0 |
| [gloo-timers](https://crates.io/api/v1/crates/gloo-timers) | 0.4.0 | 1.82 | 0.4 | pet:0.4.0 |
| [js-sys](https://crates.io/api/v1/crates/js-sys) | 0.3.106 | 1.81 | 0.3 | chat:0.3.106；pet:0.3.106；analytics:0.3.106；crates:0.3.106 |
| [leptos](https://crates.io/api/v1/crates/leptos) | 0.8.22 | 1.88 | 0.8 | chat:0.8.22；pet:0.8.22；analytics:0.8.22 |
| [leptos_axum](https://crates.io/api/v1/crates/leptos_axum) | 0.8.10 | 1.88 | 0.8 | chat:0.8.10 |
| [leptos_config](https://crates.io/api/v1/crates/leptos_config) | 0.8.10 | 1.88 | 0.8 | chat:0.8.10；pet:0.8.10；analytics:0.8.10 |
| [leptos_router](https://crates.io/api/v1/crates/leptos_router) | 0.8.17 | 1.88 | 0.8 | chat:0.8.17 |
| [libc](https://crates.io/api/v1/crates/libc) | 0.2.191 | 1.65 | 0.2 | chat:0.2.191；pet:0.2.191；analytics:0.2.191；crates:0.2.191 |
| [libsqlite3-sys](https://crates.io/api/v1/crates/libsqlite3-sys) | 0.39.1 | 1.88.0 | CLI／間接依賴 | chat:0.37.0；pet:0.37.0；analytics:0.37.0 |
| [rpassword](https://crates.io/api/v1/crates/rpassword) | 7.5.4 | 1.85 | 7.5 | analytics:7.5.4 |
| [serde](https://crates.io/api/v1/crates/serde) | 1.0.229 | 1.56 | 1.0 | chat:1.0.229；pet:1.0.229；analytics:1.0.229；crates:1.0.229 |
| [serde-wasm-bindgen](https://crates.io/api/v1/crates/serde-wasm-bindgen) | 0.6.5 | 官方 metadata 未宣告 | 0.6 | pet:0.6.5 |
| [serde_json](https://crates.io/api/v1/crates/serde_json) | 1.0.154 | 1.71 | 1.0 | chat:1.0.154；pet:1.0.154；analytics:1.0.154；crates:1.0.154 |
| [sha2](https://crates.io/api/v1/crates/sha2) | 0.11.1 | 1.88 | 0.11 | chat:0.10.9,0.11.1；pet:0.10.9,0.11.1；analytics:0.10.9,0.11.1；crates:0.11.1 |
| [sqlx](https://crates.io/api/v1/crates/sqlx) | 0.9.0 | 1.94.0 | 0.9 | chat:0.9.0；pet:0.9.0；analytics:0.9.0 |
| [sqlx-cli](https://crates.io/api/v1/crates/sqlx-cli) | 0.9.0 | 1.94.0 | CLI／間接依賴 | 不在產品 lock，見工具表 |
| [tauri](https://crates.io/api/v1/crates/tauri) | 2.12.5 | 1.90 | 2.12 | pet:2.12.5 |
| [tauri-build](https://crates.io/api/v1/crates/tauri-build) | 2.7.4 | 1.90 | 2.7 | pet:2.7.4 |
| [tauri-cli](https://crates.io/api/v1/crates/tauri-cli) | 2.12.4 | 1.90 | CLI／間接依賴 | 不在產品 lock，見工具表 |
| [thiserror](https://crates.io/api/v1/crates/thiserror) | 2.0.21 | 1.77 | 2.0 | chat:1.0.69,2.0.21；pet:1.0.69,2.0.21；analytics:1.0.69,2.0.21；crates:2.0.21 |
| [tokio](https://crates.io/api/v1/crates/tokio) | 1.54.1 | 1.85 | 1.54 | chat:1.54.1；pet:1.54.1；analytics:1.54.1；crates:1.54.1 |
| [tower](https://crates.io/api/v1/crates/tower) | 0.5.3 | 1.64.0 | 0.5 | chat:0.5.3；pet:0.5.3；analytics:0.5.3 |
| [tracing](https://crates.io/api/v1/crates/tracing) | 0.1.44 | 1.65.0 | 0.1 | chat:0.1.44；pet:0.1.44；analytics:0.1.44 |
| [trunk](https://crates.io/api/v1/crates/trunk) | 0.22.0 | 1.96.1 | CLI／間接依賴 | 不在產品 lock，見工具表 |
| [uuid](https://crates.io/api/v1/crates/uuid) | 1.29.0 | 1.89.0 | 1.29 | chat:1.29.0；pet:1.29.0；analytics:1.29.0；crates:1.29.0 |
| [wasm-bindgen](https://crates.io/api/v1/crates/wasm-bindgen) | 0.2.129 | 1.81 | 0.2 | chat:0.2.129；pet:0.2.129；analytics:0.2.129；crates:0.2.129 |
| [wasm-bindgen-cli](https://crates.io/api/v1/crates/wasm-bindgen-cli) | 0.2.129 | 1.86 | CLI／間接依賴 | 不在產品 lock，見工具表 |
| [wasm-bindgen-futures](https://crates.io/api/v1/crates/wasm-bindgen-futures) | 0.4.79 | 1.81 | 0.4 | chat:0.4.79；pet:0.4.79；analytics:0.4.79 |
| [web-sys](https://crates.io/api/v1/crates/web-sys) | 0.3.106 | 1.81 | 0.3 | chat:0.3.106；pet:0.3.106；analytics:0.3.106 |

## CLI、資料庫與Linux原生環境

| 工具／官方來源 | 官方穩定版 | 本機觀察與採用判斷 |
| --- | --- | --- |
| [cargo-leptos](https://crates.io/api/v1/crates/cargo-leptos) | 0.3.11 | `cargo-leptos 0.3.11`，相符；保留現有SSR／hydration流程 |
| [Tauri CLI](https://crates.io/api/v1/crates/tauri-cli) | 2.12.4 | `tauri-cli 2.12.3`，較舊patch；建議另行升級後驗cargo tauri dev／bundle，這次未升級 |
| [Trunk](https://crates.io/api/v1/crates/trunk) | 0.22.0 | `trunk 0.21.14`；跨0.x minor，須核對設定／hooks／dist及Tauri dev再採用，不聲稱相容已實測 |
| [SQLx CLI](https://crates.io/api/v1/crates/sqlx-cli) | 0.9.0 | 本機cargo sqlx不存在（exit101）；不是產品測試失敗。CI配置安裝^0.9，本次不安裝、不跑migration |
| [wasm-bindgen CLI](https://crates.io/api/v1/crates/wasm-bindgen-cli) | 0.2.129 | 依Leptos build工具鏈選用並與crate精確配對；本次未新增CLI執行／建置證據 |
| [SQLite](https://www.sqlite.org/changes.html) | 3.54.1（2026-10-16） | 本機sqlite3 CLI不存在；不能從Python sqlite版本推定SQLx runtime。四lock的libsqlite3-sys為0.37.0，官方最新0.39.1；後者是binding crate版本，不能與SQLite版本混寫或直接越過SQLx約束升級 |
| [PostgreSQL](https://www.postgresql.org/support/versioning/) | 18.6 | `psql (PostgreSQL) 18.6 (Ubuntu 18.6-1.pgdg26.04+2)`僅證明client；未連線SHOW server_version，server實際版本本次未核實。19 RC1排除，CI postgres:latest非固定映像版本 |
| [WebKitGTK](https://webkitgtk.org/releases/) | 2.54.2 | pkg-config webkit2gtk-4.1為2.52.6；版本不同。Tauri使用4.1 API，不能把2.54.2直接當成已安裝或已修復空白視窗；應按發行版依賴／實機回歸再更新 |
| [GLib](https://download.gnome.org/sources/glib/cache.json) | 2.90.1 | 本機2.88.0；本機pkg-config成功不代表GitHub runner也具備開發套件 |
| [GTK3適用分支](https://download.gnome.org/sources/gtk/3.24/) | 3.24.52 | 本機3.24.52；這是Tauri Linux GTK3分支核對，不主張GTK3是GTK整體最新主版本，不自動遷至GTK4 |

原生安裝條件依 [Tauri官方Linux前置需求](https://v2.tauri.app/start/prerequisites/)；不根據本機版本替CI runner宣告修復。

## 相容性判斷與建議

- Rust 1.99.1高於本次已宣告的直接依賴／列示CLI MSRV（含SQLx 1.94.0、Trunk 1.96.1、Tauri 1.90）；這是metadata門檻檢查，不是所有transitive依賴或建置相容性驗證。未宣告MSRV的項目仍需真實build。
- 直接Rust依賴目前resolved與本次官方穩定版對照見表；thiserror 1.x／2.x、sha2 0.10／0.11等同時存在的舊間接版本保留依賴來源，不做全域替換。libsqlite3-sys與CLI差異分別處理，不能以cargo update掩蓋相容性問題。
- 後續UI切片沿Leptos 0.8 SSR／hydrate／CSR與Router既有features；WASM使用crate／CLI配對版本，檢查web-sys feature和目標依賴。沒有將native SQLx或Tauri依賴移入WASM內層。
- Pet用例重構維持Application／Domain無SQLx、Tauri；原生Adapter與SQLite分離，新的Port／DTO不反向依賴server。純內層編譯不等於IPC／WebKit／多窗驗收。
- Analytics保留PostgreSQL隔離、人員Session與兩正式查詢入口；不由client版本或crate版本表推論資料庫server已升級、ACL已完成或續期競態已驗收。
- 最新穩定版為後續教學優先目標；有工具版本差異時，須在最終Green列必要遷移與驗證後採用。本次只補完這個候選範圍的當日版本來源／宣告／lock／安裝／MSRV核對，並明列採用理由與限制。最終Green如新增依賴、工具或換日期，須重新核對；不能據此直接宣告Day38 Gate1完成。

## 現行摘要修正與歷史界線

1. 最新程式來源仍是648b903；三篇Day37最新閱讀版本為r3，各原commit／blob／SHA256保留。r2的「未套用／未測」只限原交付時點。
2. 查詢完成helper已在來源；Chat現在待修的是status未綁DOM的可見錯誤／重試，以及首次自動載入、Session管理入口、房間名標題等使用者流程。不得再沿舊來源將成功仍查詢中當最新診斷。
3. Pet先保存／建窗失敗補償與鎖已存在。控制窗／生活窗／同ID重複出籠有先前使用者局部驗證；空白視窗原因未定，不能持續表述為目前仍全部空白。固定漫遊label、直接關窗／重啟及native故障注入仍待驗收。
4. 本次重新讀取同SHA七個workflow與Persistence job步驟，仍為6成功／1失敗。`Test Pet Persistence`失敗，後續`Test analytics server with PostgreSQL`為skipped。workflow已選analytics-server與analytics-persistence並加postgres-integration，修正「未配置」的過時現行摘要。[失敗run](https://github.com/williechen/rust_ai_ring/actions/runs/40202563244)；GLib／GObject缺少開發套件原因依先前保存的精確SHA日誌，本次未重跑。
5. state／manifest新增本次文件維護觀察；舊通用source／comparison欄位保留其歷史用途並加解讀說明。更新現行manifest revision標記，不修改last_full_comparison、FIFO、Day37交付或驗收紀錄。

先前測試數字僅引用 [Day37精確來源驗證](2026-10-25-day37-verification-648b903.md)：Chat UI15、Pet62、Analytics PG45加略過另跑1；全部不是本次重新測試。本文不是新教學、完整Green或產品通過證據。


## 發布後偵測到新的程式來源

2026-10-25T21:38:21.400Z 再讀main已前進至 `9b89656f964b99e0cb89ba421855704a35c5ecb9`（upgrade）。精確compare共8檔：Persistence workflow新增Linux開發依賴，三案workspace package由0.2.35改0.2.37及四個lock內部package版本。逐檔讀回並解析新舊TOML／lock後，外部依賴宣告與resolved完整相同，沒有Rust／UI功能程式差異，所以本日官方外部版本表仍適用；不是重新測試新SHA。

新SHA CI讀取快照：Node成功，其餘6個執行中，包括 [Persistence 40236329539](https://github.com/williechen/rust_ai_ring/actions/runs/40236329539)。安裝步驟已加入不等於修復已驗證；舊648失敗／skipped記錄保留原SHA。上述測試數與GUI證據仍限648或各自原始來源，不改掛新SHA。

完整差異、8檔blob及CI快照見 [來源跟進紀錄](version-refresh-2026-10-26/source-followup.json)。本次沒有由代理修改rust_ai_ring或升級本機工具，三案next_day維持38。
