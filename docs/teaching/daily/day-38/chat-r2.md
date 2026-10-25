# Chat Day38 r2｜搜尋失敗顯示在真正的 Leptos 房間列表，保留既有連結

> **交付日：2026-10-26（Asia/Taipei）**。程式唯讀來源 [rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9](https://github.com/williechen/rust_ai_ring/tree/9b89656f964b99e0cb89ba421855704a35c5ecb9)；文件規則：[合併後共用規則](../../../COMMON_LEARNING_RULES.md)。本文是**可由使用者套用的完整最小修補教學**，不是已套用的產品程式；Red、Green、真瀏覽器與 CI **本次均未執行**，Gate 2 未驗收。

本修訂保留[原版](chat.md)，以當前精確來源提供[完整可套用patch](chat-r2.patch)。核心60分鐘只計②主Red／Green；局部Refactor、工程與其他交付另計。這是同Day修訂，next_day維持39。

從未改過的來源使用完整patch；若先手動加入Red測試，依Green正文補程式，不重複套入已含測試的patch。來源有本地變更先審查差異，不覆寫。完整patch的SHA256：`fecb9f84591031f0173a598de5ad30857161ce2415b63e9326c38efb95fec745`。

規則精確來源：copilot_use `43b0c4f4eb5d6b65d6d3155a473b2a673d944046`，共用規則 blob `420147c5202bc2dd44fe44a4bb83da2c1d91d5cf`。

## ① 學習大綱：承接 Day37 的真實 UI 缺口

[上一篇 Day37 r3](../day-37/chat-r3.md) 在精確來源 648b903 已確認 `apply_room_search` 更新 `status` Signal，卻沒有把 `status.get()` 接到 RoomList DOM；真瀏覽器 HTTP 500 後保留房間列，但顯示「請選擇房間」，而不是錯誤／重試。最新 9b89656f964b99e0cb89ba421855704a35c5ecb9 的 `chat_apps/chat_ui/src/lib.rs` 第 344 行附近仍是 `room_list_feedback(has_snapshot, all.len(), visible.len())`，因此不是重做 Day37 的 reducer。選一個能被使用者看到的 UI 邊界修補：**載入／搜尋中顯示進度，失敗顯示可重試原因，成功顯示空／無符合／請選房；保留先前房間連結**。

正式路徑：`RoomList`（Leptos SSR + hydrate）→ `room_api::search_rooms/list_rooms`（WASM fetch）→ 現有 `chat-server` Axum 房間 API → `chat-application`／`chat-persistence` SQLite → `apply_room_search`／`refresh_result_view` → 真實 `role="status"` DOM。**不新增 API、SQL、Session 或新 crate**；登入／管理文字、首頁首次自動讀取、匿名選房登入回跳、作者／歷史／ACL 是獨立缺口，不能因本篇通過而結案。

## ② Red → 完整最小 Green → Refactor

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

### Refactor：測試與分層對照

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

本次新增靜態驗證：在隔離來源快照，`git apply --check`通過；完整修改檔以rustfmt解析通過。[逐檔來源blob與結果](r2-static-checks.json)。這兩項不代表編譯、Red失敗、Green通過、DB、native、GUI或CI；上述仍未執行。產品倉庫未套用本候選內容。
