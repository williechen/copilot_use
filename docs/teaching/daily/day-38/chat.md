# Chat Day38｜搜尋失敗顯示在真正的 Leptos 房間列表，保留既有連結

> **交付日：2026-10-26（Asia/Taipei）**。程式唯讀來源 [rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9](https://github.com/williechen/rust_ai_ring/tree/9b89656f964b99e0cb89ba421855704a35c5ecb9)；文件規則：`daily-tdd-minimum-v1`、`functional-layering-v1`、`test-refactor-v1`、`github-only-v1`。本文是**可由使用者套用的完整最小修補教學**，不是已套用的產品程式；Red、Green、真瀏覽器與 CI **本次均未執行**，Gate 2 未驗收。

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

## ③ 交付日官方穩定版本（2026-10-26）

核對來源：[當日官方 crates.io API／四 workspace lock 證據](../../../source-review/version-refresh-2026-10-26/evidence.json)，核對時間 2026-10-26 05:35（台北）；最新 `rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9` 直接依賴未改。官方 [Rust 1.99.1（2026-10-15）](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)；[leptos 0.8.22](https://crates.io/crates/leptos/0.8.22)（MSRV 1.88）、[leptos_router 0.8.17](https://crates.io/crates/leptos_router/0.8.17)（1.88）、[wasm-bindgen 0.2.129](https://crates.io/crates/wasm-bindgen/0.2.129)（1.81）、[axum 0.8.9](https://crates.io/crates/axum/0.8.9)（1.80）、[sqlx 0.9.0](https://crates.io/crates/sqlx/0.9.0)（1.94）。此處皆為已查最高非 yanked 穩定版，Chat lock 同版；本篇不升級依賴，沒有以 MSRV 推定建置通過。

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
