# Chat Day37 r3｜房間搜尋完成時更新真實畫面狀態，失敗保留快照（驗證更新）

> **2026-10-25 驗證更新**；精確來源 [`rust_ai_ring@648b90323af7b381d6a79dc2fd08a53b63173a43`](https://github.com/williechen/rust_ai_ring/tree/648b90323af7b381d6a79dc2fd08a53b63173a43)。本次產品快照唯讀，未推送產品。**本次結果：15 項 UI 測試、WASM check、Leptos SSR／hydrated build 通過；真瀏覽器成功／空結果／恢復通過，但 HTTP500 後沒有可見錯誤／重試，Day37 顯示契約未完整通過。** 來源已存在 apply_room_search／WASM caller；RoomList 未綁定 status.get()，不可只憑 Signal 測試宣稱畫面驗收。首頁自動讀取與登入／管理仍另列缺口；本地房名修正不在648b903快照中。
>
> 本修訂保留 [Day37 r2 原文](chat-r2.md) 與其 commit/blob/SHA256；只更新驗證狀態，不新增 Day。三案 next_day 維持38，Gate2未完整通過、不封存。[完整驗證與失敗恢復](../../../source-review/2026-10-25-day37-verification-648b903.md) · [原始 logs／AX／版本證據包](../../../source-review/day37-verification-648b903/evidence.zip)。本篇獨立提交與精確讀回結果見 `chat-r3-readback.json`。

## ① 學習大綱

基本登入／建房／房間清單已有接線；本日修正使用者可操作搜尋的回饋。來源 `chat_apps/chat_ui/src/lib.rs::RoomList::query_room` 成功分支只更新 rows／has_snapshot，沒有清除「查詢中…」。既有 `refresh_result_view` 已正確處理成功／空／失敗且被重新整理按鈕呼叫，因此不能把它的既有通過測試當新 Red。

真鏈：Leptos `RoomList` → `room_api::search_rooms` 的 fetch → Chat Axum 搜尋路由 → 既有 application／SQLite SQLx 查詢 → JSON → Signals → `role="status"`／房間連結。後端查詢、帳號與 Session、SQL 及資料契約不改。本篇只接通搜尋完成回饋，不宣稱 ACL／歷史或整個聊天驗收。

## ② Red → 完整 Green → Refactor

以下完整 Red／Green／Refactor 保留 r2 的 `5c713553` 教材基準；其中「來源尚無」「預期失敗」只限原教材時點。當前648b903已含Green，不要重複套用；本次跑的是已實作版本的測試，沒有重演原始Red。程式與產品判定以下方第④節及驗證報告為準。

### Red：新增真實 Signal 提交端測試

將以下完整模組加入 `chat_apps/chat_ui/src/lib.rs` 尾端。名稱從 Red 到 Green 完全一致。來源尚無 `apply_room_search`，第一步預期為 E0425 缺少待新增符號的編譯 Red，**尚未執行**；不是把已存在 reducer 的通過結果冒稱 Red。使用者也可先在瀏覽器成功搜尋，觀察第一個 status 仍為「查詢中…」作為既有行為缺陷；本次未操作瀏覽器。

```rust
#[cfg(test)]
mod day37_search_tests {
    use super::*;

    #[test]
    fn real_search_completion_updates_the_signals_used_by_the_view() {
        let owner = Owner::new();
        owner.with(|| {
            for result in [
                Ok(vec![("new".to_owned(), "貓房".to_owned())]),
                Ok(vec![]),
                Err("離線".to_owned()),
            ] {
                let old = vec![("old".to_owned(), "原房間".to_owned())];
                let (rows, set_rows) = signal(old.clone());
                let (snapshot, set_snapshot) = signal(true);
                let (status, set_status) = signal("查詢中…".to_owned());
                let (loading, set_loading) = signal(true);
                let expected = result.clone();
                apply_room_search(
                    old.clone(), "貓".to_owned(), result,
                    set_rows, set_snapshot, set_status, set_loading,
                );
                assert!(!loading.get_untracked());
                assert!(snapshot.get_untracked());
                match expected {
                    Ok(value) => {
                        assert_eq!(rows.get_untracked(), value);
                        assert_eq!(status.get_untracked(),
                            if value.is_empty() { "尚無房間" } else { "請選擇房間" });
                    }
                    Err(_) => {
                        assert_eq!(rows.get_untracked(), old);
                        assert_eq!(status.get_untracked(), "離線，請重試");
                    }
                }
            }
            let (rows, set_rows) = signal(Vec::new());
            let (snapshot, set_snapshot) = signal(false);
            let (status, set_status) = signal("查詢中…".to_owned());
            let (loading, set_loading) = signal(true);
            apply_room_search(Vec::new(), String::new(), Err("離線".into()),
                set_rows, set_snapshot, set_status, set_loading);
            assert!(rows.get_untracked().is_empty());
            assert!(!snapshot.get_untracked());
            assert!(!loading.get_untracked());
        });
    }
}
```

### Green：重用 reducer，集中真實 caller 所需 Signal 更新

在同檔 `refresh_result_view` 後、`shell` 前新增以下完整函式；`leptos::prelude::*` 已匯入 `WriteSignal`、`Set`，無新 imports／dependencies。

```rust
fn apply_room_search(
    previous: Vec<(String, String)>,
    submitted: String,
    result: Result<Vec<(String, String)>, String>,
    set_rows: WriteSignal<Vec<(String, String)>>,
    set_has_snapshot: WriteSignal<bool>,
    set_status: WriteSignal<String>,
    set_loading: WriteSignal<bool>,
) {
    let succeeded = result.is_ok();
    let (next_rows, _retry_query, next_status) =
        refresh_result_view(previous, submitted, result);
    if succeeded {
        set_has_snapshot.set(true);
    }
    set_rows.set(next_rows);
    set_status.set(next_status);
    set_loading.set(false);
}
```

接著**只替換 `RoomList` 的 `query_room` closure 中原 WASM spawn_local 區塊**為以下完整內容；載入守衛、set_loading(true)、status「查詢中…」、非 WASM 分支與讀取／重新整理按鈕全部保留。

```rust
#[cfg(target_arch = "wasm32")]
wasm_bindgen_futures::spawn_local(async move {
    let submitted = query.get_untracked();
    let previous = rows.get_untracked();
    let result = room_api::search_rooms(&submitted).await;
    apply_room_search(previous, submitted, result,
        set_rows, set_has_snapshot, set_status, set_loading);
});
```

搜尋輸入 query 不在完成時清除或覆寫，因此使用者請求期間另輸入的文字仍保留；`_retry_query` 刻意不用，與重新整理按鈕成功後清除 query 的原契約不同。後續錯誤保留上次 rows 與 snapshot，成功（包括空結果）才建立 snapshot。畫面仍用既有兩段 status；本切片不宣稱已合併為單一狀態模型。

### Refactor、上一 Day 未完成事項與舊稿更正

不要新增 `finish_room_query` 或複製 reducer；新 `apply_room_search` 負責 Signal 提交，既有 `refresh_result_view` 負責資料結果，兩者責任不同且均有真 caller。舊 `refreshed_room_list` 的清理沿 FUTURE_WORK 既有項目，不能因只見測試就刪。Day36 的 WS 原子撤銷競態、ACL／歷史與瀏覽器缺口仍未完成，引用其原始實測報告，不重新教已套用 WS。FIFO 本次不結案。

Cargo：工作目錄 `chat_apps`；package `chat-ui`、crate `chat_ui`、features `ssr`／`hydrate` 原樣；`chat_apps/Cargo.toml` 的 Leptos bin-package `chat-server`、lib-package `chat-ui`、輸出名 `rust_ai_chat` 不改。既有 wasm-bindgen／futures／web-sys target dependencies 原樣。四個 workspace 的 Cargo.toml／Cargo.lock、migrations、後端與其它專案均無必要修改。文件中的程式碼由使用者在產品庫套用。

## ③ 套件與軟體版本

交付日 2026-10-25，直接讀官方 crates.io API 的 versions，排除預發行與 yanked；不是以 Cargo.lock 猜測最新版。逐項官方網址、發行日期與 MSRV 原始回應見 [本次官方版本重新查核](../../../source-review/day37-verification-648b903/official-versions.json)。四個 workspace 的 manifest／lock 已讀；下表為本篇使用範圍。

| 項目 | manifest 宣告 | lock resolved | 當日官方最新穩定 | MSRV |
| --- | --- | --- | --- | --- |
| leptos | `0.8` | 0.8.22 | [0.8.22](https://crates.io/crates/leptos/0.8.22) | 1.88 |
| leptos_router | `0.8` | 0.8.17 | [0.8.17](https://crates.io/crates/leptos_router/0.8.17) | 1.88 |
| wasm-bindgen | `0.2` | 0.2.129 | [0.2.129](https://crates.io/crates/wasm-bindgen/0.2.129) | 1.81 |
| wasm-bindgen-futures | `0.4` | 0.4.79 | [0.4.79](https://crates.io/crates/wasm-bindgen-futures/0.4.79) | 1.81 |
| web-sys | `0.3` | 0.3.106 | [0.3.106](https://crates.io/crates/web-sys/0.3.106) | 1.81 |
| serde_json | `1.0` | 1.0.154 | [1.0.154](https://crates.io/crates/serde_json/1.0.154) | 1.71 |
| axum | `0.8` | 0.8.9 | [0.8.9](https://crates.io/crates/axum/0.8.9) | 1.80 |
| sqlx | `0.9` | 0.9.0 | [0.9.0](https://crates.io/crates/sqlx/0.9.0) | 1.94.0 |
| tokio | `1.54` | 1.54.1 | [1.54.1](https://crates.io/crates/tokio/1.54.1) | 1.85 |
| cargo-leptos | `CI 0.3.11` | 工具，不在產品 lock | [0.3.11](https://crates.io/crates/cargo-leptos/0.3.11) | 1.82.0 |

Rust 官方 stable manifest 為 **1.99.1**（2026-10-15），[官方公告](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)；本機 `rustc 1.99.1`、`cargo 1.99.1`，Edition 2024，workspace 未固定 rust-version。所有上列已宣告 MSRV 均不高於本機 toolchain。建議採表列當日穩定版本；產品 lock 已相符者保留，無需 cargo update；相同 lock 的 native／WASM 相容仍須第④節實測，不能從 MSRV 推定成功。sha2 0.10.9 若見於 lock 屬並存的傳遞依賴，不任意強制升成 0.11。

本機 cargo-leptos 0.3.11 與官方相符；WASM glue 的 wasm-bindgen／web-sys／futures 保持配套 lock。SQLite 最新穩定 [3.54.1（10-16）](https://www.sqlite.org/chronology.html)；產品 runtime SQLite 版本本次未查詢，Python 的 3.46.1 不是產品版本。本篇沒有新 SQL／migration，不為 UI 修補強行替換 SQLx 綁定 SQLite；升級需另驗 SQLx native linkage、DB 與 WAL。
當日版本資料已於本次重新查核（見連結的 checked_at）；本次測試快照的四個 workspace 來源雜湊記於 summary.json。原 r2 版本檔與查核時間不覆寫。

## ④ 本次實際驗證、失敗與界線

**本次結果：15 項 UI 測試、WASM check、Leptos SSR／hydrated build 通過；真瀏覽器成功／空結果／恢復通過，但 HTTP500 後沒有可見錯誤／重試，Day37 顯示契約未完整通過。** 來源已存在 apply_room_search／WASM caller；RoomList 未綁定 status.get()，不可只憑 Signal 測試宣稱畫面驗收。首頁自動讀取與登入／管理仍另列缺口；本地房名修正不在648b903快照中。

本次在隔離 Git 快照的對應 workspace 執行（Cargo cache 在快照外，不變更來源；完整退出碼與 log見證據包）：

```bash
cargo test --locked -p chat-ui --features ssr
cargo check --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown
cargo leptos build --cargo-locked
```

真瀏覽器使用648b903新建置資產與 task-only SQLite；成功顯示中文房名、空結果清單清空、資料庫恢復後可重試。暫時改名 task-only rooms 表產生 HTTP500，失敗後保留舊清單但畫面仍顯示「請選擇房間」，**錯誤／重試提示未通過**。status沒有DOM讀取，需後續最小修補與瀏覽器回歸；本篇不捏造已完成修補。

同648b903共有6個workflow成功、1個Persistence失敗。失敗job121196631294在Test Pet Persistence執行包含pet-server的測試，因缺glib-2.0.pc／gobject-2.0.pc而exit101；後續已配置的完整Analytics PG步驟被skipped。本機測試成功不能替代CI，也沒有在本次修正或重跑workflow。[CI快照](../../../source-review/day37-verification-648b903/ci.json)。

保留產品實作、局部測試、CI與功能驗收的獨立認定；不修改last_full_comparison、FIFO、舊文章或封存位置。
