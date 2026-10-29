# Chat Day38 r3｜搜尋失敗顯示在真正的 Leptos 房間列表，保留既有連結

> **交付日：2026-10-26（Asia/Taipei）**。程式唯讀來源 [rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9](https://github.com/williechen/rust_ai_ring/tree/9b89656f964b99e0cb89ba421855704a35c5ecb9)；文件規則：[合併後共用規則](../../../COMMON_LEARNING_RULES.md)。本文是**可由使用者套用的完整最小修補教學**，不是已套用的產品程式；Red、Green、真瀏覽器與 CI **本次均未執行**，Gate 2 未驗收。

本修訂保留[原版](chat.md)，以當前精確來源提供[完整可套用patch](chat-r2.patch)。核心60分鐘只計②主Red／Green；同篇專案Refactor獨立段落與其餘作業另計。這是同Day修訂，next_day維持39。

從未改過的來源使用完整patch；若先手動加入Red測試，依Green正文補程式，不重複套入已含測試的patch。來源有本地變更先審查差異，不覆寫。完整patch的SHA256：`fecb9f84591031f0173a598de5ad30857161ce2415b63e9326c38efb95fec745`。

規則精確來源：copilot_use `92913e7aae67b317fe25dab75f6ec56721042d35`，共用規則blob `492b50b2e16f1d8997ab55028bfc3c4d072af363`；專案Refactor與②Red／Green並列，功能程式與patch沿用r2精確來源。

## ① 學習大綱：承接 Day37 的真實 UI 缺口

[上一篇 Day37 r3](../day-37/chat-r3.md) 在精確來源 648b903 已確認 `apply_room_search` 更新 `status` Signal，卻沒有把 `status.get()` 接到 RoomList DOM；真瀏覽器 HTTP 500 後保留房間列，但顯示「請選擇房間」，而不是錯誤／重試。最新 9b89656f964b99e0cb89ba421855704a35c5ecb9 的 `chat_apps/chat_ui/src/lib.rs` 第 344 行附近仍是 `room_list_feedback(has_snapshot, all.len(), visible.len())`，因此不是重做 Day37 的 reducer。選一個能被使用者看到的 UI 邊界修補：**載入／搜尋中顯示進度，失敗顯示可重試原因，成功顯示空／無符合／請選房；保留先前房間連結**。

正式路徑：`RoomList`（Leptos SSR + hydrate）→ `room_api::search_rooms/list_rooms`（WASM fetch）→ 現有 `chat-server` Axum 房間 API → `chat-application`／`chat-persistence` SQLite → `apply_room_search`／`refresh_result_view` → 真實 `role="status"` DOM。**不新增 API、SQL、Session 或新 crate**；登入／管理文字、首頁首次自動讀取、匿名選房登入回跳、作者／歷史／ACL 是獨立缺口，不能因本篇通過而結案。

## 專案 Refactor

本節在同篇與②Red／Green並列，按Chat整體架構、共用依賴、測試與CI的累積進度每日調整；不是Green完成後才開始的步驟，也不限定於當日功能切片。另計時間，不增加Day。

### 專案目標與累積現況

| 工程責任 | 需銜接的邊界 |
| --- | --- |
| 認證與Session | Application用例／Session與身份Port；HTTP／WS adapter及組裝，登入／登出／續期契約 |
| 房間與成員 | 房間／成員／加入與ACL用例、SQLite repository，公開清單與授權進房分責任 |
| 訊息與即時傳輸 | 訊息／作者／歷史與WS生命週期，HTTP／WS共用授權邊界與真caller |
| Leptos UI | 房間列表、帳號／管理與進房流程；UI狀態與API client邊界，SSR／hydrate保持一致 |
| 共用依賴／組裝 | 檢視四workspace公開契約、shared、Cargo normal／build／dev／target依賴、features與正式caller；先承接再清舊碼 |

先完成專案caller／Port／Adapter與斷言基線，再依相依選完整包。新功能與未完成需求依FUTURE_WORK另追蹤，不能把表內規劃寫成已存在或已重構。

### 既有測試遷移基線（專案清單的一部分）

| 舊測試／群組 | 判斷 | 處理與必留斷言 | 目標責任 |
| --- | --- | --- | --- |
| search_matches_chinese_names_and_preserves_ids + day18_query_filter_preserves_room_ids | 同層部分重複 | 合併參數化：中文搜尋／前後空白／空字串／純空白／無結果；保留ID | filter_rooms 的功能測試 |
| day19_failure_preserves_complete_rows_and_message + day19_refresh_failure_keeps_previous_room_links | 同層失敗快照斷言重疊 | 遷移舊helper有效斷言，保留同名不同ID／完整順序／錯誤文字／成功替換 | 正式刷新結果處理；再驗UI錯誤可見 |
| day20_refresh_success_reveals_full_snapshot + day20_refresh_failure_keeps_query_and_rows | 不同結果分支 | 保留成功清搜尋與失敗保留搜尋；可表格化但不刪情境 | refresh_result_view 或後續真狀態用例 |
| real_search_completion_updates_the_signals_used_by_the_view | Signal整合；不是DOM驗證 | 保留Signal接線；另補真元件／瀏覽器可見錯誤與自動載入 | UI層與瀏覽器 |

來源 `rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9`。此表是來源核對後的擬議映射，非已遷移清單；完整跨案對照見 [測試覆蓋表](test-coverage-map.csv)。同層同契約才整併；跨層保留接線、SQL、HTTP／IPC或DOM獨有的驗證責任。fixture抽共用準備，不複製業務實作或將情境斷言藏入共用helper。



### 本次完整調整：CI責任隔離

原Persistence workflow將Chat→Pet server/native→Analytics PG串在同一job，前段故障會使後段不執行。Chat的SQLite Adapter改由chat-sqlite獨立執行；本workflow不新增Chat純內層、HTTP／WS或UI的執行入口，既有其它workflow保留，完整覆蓋仍需核對。

下面是取代產品來源 `.github/workflows/persistence.yml` 的完整YAML，三案共用同一份內容及證據。本r3把先前已交付的方案納入同篇獨立段落，不重複計為新增工程成果；其餘Application／Domain／Adapter的完整重構仍列待辦。

```yaml
name: Persistence
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
permissions:
  contents: read
concurrency:
  group: persistence-${{ github.ref }}
  cancel-in-progress: true
env:
  CARGO_TERM_COLOR: always
jobs:
  chat-sqlite:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Chat SQLite adapter tests
        working-directory: chat_apps
        env:
          DATABASE_URL: "sqlite::memory:"
        run: cargo test --locked -p chat-persistence

  pet-sqlite:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Pet SQLite adapter tests (no Tauri)
        working-directory: pet_apps
        env:
          DATABASE_URL: "sqlite::memory:"
        run: cargo test --locked -p pet-persistence

  pet-native:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Tauri native dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y build-essential pkg-config curl wget file \
            libssl-dev libxdo-dev librsvg2-dev libwebkit2gtk-4.1-dev \
            libayatana-appindicator3-dev
      - uses: dtolnay/rust-toolchain@stable
      - name: Pet native/server tests (not GUI acceptance)
        working-directory: pet_apps
        env:
          DATABASE_URL: "sqlite::memory:"
        run: cargo test --locked -p pet-server

  analytics-postgres:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:18
        env:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: analytics_ci
        ports: ["5432:5432"]
        options: >-
          --health-cmd "pg_isready -U postgres -d analytics_ci"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Analytics SQLx adapter + real Router integration
        working-directory: analytics_apps
        env:
          DATABASE_URL: postgres://postgres:postgres@localhost:5432/analytics_ci
          ANALYTICS_DATABASE_URL: postgres://postgres:postgres@localhost:5432/analytics_ci
          ANALYTICS_CHAT_TOKEN: local-ci-chat-token
          ANALYTICS_PET_TOKEN: local-ci-pet-token
        run: cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration
```

### 測試／CI承接與套用條件

| 原選取 | 新責任入口 | 必留範圍 |
| --- | --- | --- |
| chat-persistence | chat-sqlite | SQLite Adapter原斷言全部保留 |
| pet-persistence | pet-sqlite | SQLite Adapter原斷言全部保留，不帶Tauri |
| pet-server | pet-native | 原server／IPC與native測試保留，GUI另驗 |
| analytics-server＋analytics-persistence | analytics-postgres | postgres-integration、真PG與正式Router／資料層測試 |

四job無互相needs、無continue-on-error，不刪測試換全綠。若required checks仍指定舊test-persistence，使用者同步調整新job名稱；本次未改分支規則。PG18是擬議服務major，實際minor及DB結果須套用後讀回。其它workflow保留；純內層／HTTP／WS／IPC／UI覆蓋仍須完整基線核對，不能從此workflow推論全專案覆蓋完整。

### 實際驗證、回復與下一步

方案來源為 `9b89656f964b99e0cb89ba421855704a35c5ecb9`。原方案已通過精確來源的patch套用檢查與YAML解析；產品未套用，Actions、DB、native與GUI未執行。本r3未新增執行或套用結果。

使用者套用後，以隔離分支的Pet native可控失敗確認其餘job仍執行且整體失敗，恢復後重跑全部原斷言。回復時還原該workflow及相應required checks，不回退其它產品變更。

下一步補齊認證／Session、房間／成員、訊息／WS、Leptos與shared的caller、Port／Adapter、有效斷言及CI選取基線，再依真相依提供完整用例與測試遷移；前案未套用不得假設已存在。這些未完成項不能由本篇Green或CI方案代替，也不新增FIFO或更新last_full_comparison。

[原完整差異patch](../../../engineering/2026-10-26/ci-isolation.patch) · [原驗證設計](../../../engineering/2026-10-26/ci-isolation.md) · [專案盤點](../../functional-refactoring/chat.md) · [跨日累積進度](../../functional-refactoring/PROGRESS.md) · [r2原文](chat-r2.md)。

## ② 功能實作：Red → 完整 Green

### Red：先讓「訊號正確但畫面錯誤」有可失敗的契約

在 `chat_apps/chat_ui/src/lib.rs` 檔尾新增測試。現有 Day37 Signal 測試是**已通過的回歸基準**，不能冒充本篇新 Red。以下先因 `room_status_text` 尚不存在而編譯失敗；真正的行為 Red 則是前篇 HTTP 500 瀏覽器實證，必須在套用前後重新操作驗證。測試不假裝已跑。

```rust
#[cfg(test)]
mod day38_room_status_tests {
    use super::room_status_text;

    #[test]
    fn request_progress_and_failure_override_stale_room_feedback() {
        assert_eq!(room_status_text("查詢中…", true, 2, 1), "查詢中…");
        assert_eq!(room_status_text("載入中…", true, 2, 1), "載入中…");
        assert_eq!(room_status_text("伺服器：500，請重試", true, 2, 1),
                   "伺服器：500，請重試");
        assert_eq!(room_status_text("離線，請重試", false, 0, 0),
                   "離線，請重試");
    }

    #[test]
    fn success_and_local_filter_show_one_consistent_status() {
        assert_eq!(room_status_text("請選擇房間", false, 0, 0),
                   "請先讀取房間列表");
        assert_eq!(room_status_text("請選擇房間", true, 0, 0),
                   "尚無房間");
        assert_eq!(room_status_text("請選擇房間", true, 2, 0),
                   "沒有符合條件的房間");
        assert_eq!(room_status_text("請選擇房間", true, 2, 1),
                   "請選擇房間");
    }
}
```

### Green A：唯一的可見狀態決策（同檔）

在既有 `room_list_feedback` 函式**之後**加入完整函式；直接重用原本的成功／本地篩選規則，不複製另一份房間演算法。只有目前 reducer 產生的兩種進度與 `，請重試` 結尾的錯誤需要覆蓋舊快照回饋。錯誤內容仍由 API 產生並以 Leptos 文字節點 escape，不以 `inner_html` 注入。

```rust
fn room_status_text(
    status: &str,
    loaded: bool,
    total: usize,
    visible: usize,
) -> String {
    if matches!(status, "查詢中…" | "載入中…")
        || status.ends_with("，請重試")
    {
        return status.to_owned();
    }
    room_list_feedback(loaded, total, visible).to_owned()
}
```

### Green B：修改正式 RoomList 元件，而非只修改 helper

在同檔 `#[component] fn RoomList()` 的 `<div class="room-results" aria-live="polite">` 中，保留原本 `rows`、`filter_rooms`、`<ul>`、`href`、`aria-current`、按鈕與兩條非同步 caller，**只替換**原本三行：

```rust
let feedback = room_list_feedback(
    has_snapshot.get(), all.len(), visible.len()
);
```

為以下完整替換（在原有 `let visible = filter_rooms(&all, &query.get());` 後）：

```rust
let feedback = room_status_text(
    &status.get(),
    has_snapshot.get(),
    all.len(),
    visible.len(),
);
```

原本緊接的 `view! { <p class="room-result-feedback" role="status">{feedback}</p> <ul>...</ul> }` **保留不變**。這一個 `status.get()` 是關鍵的 Leptos reactive dependency：`set_status` 更新後，正式 DOM 必須重新計算；不另建第二個提示、不要把舊 `room_list_feedback` 整段刪掉。失敗時 `apply_room_search`／`refresh_result_view` 已保留舊 rows，無需重建 reducer 或 API。按鈕 `查詢房間`、`讀取／重新整理房間` 本身即重試入口；失敗提示不得宣稱有新的「重試」按鈕。

### Green 回歸檢查：測試與既有邊界

| 舊測試／入口 | 處置 | 保留的獨立斷言 |
| --- | --- | --- |
| `day37_search_tests::real_search_completion_updates_the_signals_used_by_the_view` | 保留回歸，不複製 | Signal 更新、失敗保留 rows、loading 釋放 |
| `day16_room_search_tests` | 保留 | 中文搜尋、ID、清單失敗與刷新 |
| 新 `day38_room_status_tests` | 新增兩個可失敗情境 | UI 顯示選擇規則、錯誤優先於舊快照 |
| `RoomList` 真瀏覽器 HTTP500／恢復 | 必須重新驗證，尚未執行 | DOM 可見錯誤、連結保留、按鈕重試、恢復後清除錯誤 |

本切片只有 UI presentation responsibility，Axum delivery、Application／Domain 與 SQLite Adapter **原樣重用**，不把 `chat-persistence` 改成空殼或建立無 caller 的用例。Leptos `RoomList` 是正式呼叫者；本篇測試函式不能代替真瀏覽器／hydrate 驗證。既有測試按功能保留，不因歷史 Day 名稱直接刪除。

### Cargo／imports／metadata／工作目錄

四個 workspace 根為 `chat_apps`、`pet_apps`、`analytics_apps`、`crates`，各自有 Cargo.toml/Cargo.lock，repo 根無 Cargo.toml。此切片只改 `chat_apps/chat_ui/src/lib.rs`：`chat-ui` Cargo package、`chat_ui` Rust crate；既有 `use leptos::prelude::*`、`chat-domain`、`shared`、`leptos_router` 皆保留。**Cargo.toml、Cargo.lock、normal/build/dev/target dependencies、features、Rust imports、migrations、CI YAML 均無需新增或變更**。Leptos workspace metadata 保留 `bin-package="chat-server"`、`lib-package="chat-ui"`、`name="rust_ai_chat"`（資源名稱不是舊 repo）；SSR 與 hydrate 不改。不要在 copilot_use 新增產品程式。

## ③ 當日官方穩定版本與採用判斷

2026-10-26 台北日期，crates/Rust於 `2026-10-25T23:01:56.157211+00:00` 重查；排除預發行與 yanked。[完整宣告／lock／官方網址／原始回應SHA](r2-versions.json)。來源9b與648外部宣告及resolved一致；9b內部package為0.2.37。

| 套件／工具 | manifest 宣告 | lock resolved／已安裝 | 官方穩定版／MSRV |
| --- | --- | --- | --- |
| leptos | `0.8` | 0.8.22 | [0.8.22](https://crates.io/crates/leptos/0.8.22)／1.88 |
| leptos_router | `0.8` | 0.8.17 | [0.8.17](https://crates.io/crates/leptos_router/0.8.17)／1.88 |
| wasm-bindgen | `見package/target表` | 0.2.129 | [0.2.129](https://crates.io/crates/wasm-bindgen/0.2.129)／1.81 |
| wasm-bindgen-futures | `見package/target表` | 0.4.79 | [0.4.79](https://crates.io/crates/wasm-bindgen-futures/0.4.79)／1.81 |
| web-sys | `見package/target表` | 0.3.106 | [0.3.106](https://crates.io/crates/web-sys/0.3.106)／1.81 |
| axum | `0.8` | 0.8.9 | [0.8.9](https://crates.io/crates/axum/0.8.9)／1.80 |
| sqlx | `0.9` | 0.9.0 | [0.9.0](https://crates.io/crates/sqlx/0.9.0)／1.94.0 |
| tokio | `1.54` | 1.54.1 | [1.54.1](https://crates.io/crates/tokio/1.54.1)／1.85 |
| uuid | `1.29` | 1.29.0 | [1.29.0](https://crates.io/crates/uuid/1.29.0)／1.89.0 |
| serde | `1.0` | 1.0.229 | [1.0.229](https://crates.io/crates/serde/1.0.229)／1.56 |
| serde_json | `1.0` | 1.0.154 | [1.0.154](https://crates.io/crates/serde_json/1.0.154)／1.71 |

Rust官方stable 1.99.1，[官方channel](https://static.rust-lang.org/dist/channel-rust-stable.toml)，本機rustc/cargo同版。最高所列MSRV不高於本機Rust，但不等於SSR／WASM／native／DB建置通過。相符resolved維持；若有差異優先以最新穩定版驗證遷移，本包不改lock、安裝工具或升級。

工具：cargo-leptos已安裝0.3.11＝官方0.3.11；SQLx CLI未安裝（官方0.9.0），WASM工具須與lock一致。SQLite CLI未安裝，官方SQLite3.54.1；libsqlite3-sys lock0.37.0對官方0.39.1，bundled實際引擎不能從CLI或registry推定，升級須SQLx相容／保資料測試。本UI修補不動DB依賴。

資料庫／native當日官方來源、已安裝探測與採用理由見[版本詳細核對](../../../source-review/2026-10-26-versions-and-summary-refresh.md)，不能以metadata宣稱功能已驗收。
## ④ 驗證步驟、結果與舊 Day 更正

```bash
cd rust_ai_ring/chat_apps
cargo test --locked -p chat-ui --features ssr day38_room_status_tests
cargo test --locked -p chat-ui --features ssr
cargo check --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown
cargo leptos build --cargo-locked
# 使用隔離 SQLite、測試帳號啟動 Chat；真瀏覽器輸入搜尋，
# 模擬房間 API HTTP 500 → status 顯示錯誤、舊房間連結仍可見，
# 恢復 API → 點原有查詢／重新整理 → status 清除且房間列更新。
```

| Gate | 預期 | 本次實際 |
| --- | --- | --- |
| Red（先新增測試） | `room_status_text` 缺符號；舊 DOM HTTP500 錯誤不可見 | 未執行；後者為 Day37 已記錄的舊來源瀏覽器缺陷 |
| Green unit／SSR | 新兩項與既有 Day37／Day16 回歸通過 | 未執行 |
| WASM／Leptos build | hydrate + SSR 正確編譯 | 未執行 |
| 真瀏覽器 | 載入、500、舊連結、重試、成功／空結果可見 | 未執行 |
| CI／產品 Gate2 | rust_ai_ring 正式來源套用後重新跑適用 CI／瀏覽器 | 未套用／未驗收 |

**舊 Day37 更正補充：** reducer 更新 Signal 不是可見 UI；舊 r3 已明示這個缺口，本篇在下一 Day 修正，不回寫 r2／r3。Day36 WebSocket 登出、作者與歷史、房間 ACL、首頁登入管理與自動讀取仍分別追蹤。此篇只表示**文件已交付**；產品原始碼仍唯讀，`last_full_comparison` 不變。

沿用r2的靜態驗證：在相同精確來源快照，`git apply --check`通過；完整修改檔以rustfmt解析通過。本r3重組正文，未重跑以上檢查或任何產品測試。[逐檔來源blob與結果](r2-static-checks.json)。這兩項不代表編譯、Red失敗、Green通過、DB、native、GUI或CI；上述仍未執行。產品倉庫未套用本候選內容。
