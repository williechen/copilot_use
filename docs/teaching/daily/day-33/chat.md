# Chat Day33 r1｜房間搜尋結果與「尚未載入」狀態正確區分

## ① 學習大綱：功能、現況與來源

**單一可操作切片：** 使用者進入 Leptos 房間列表後，初始畫面提示「請先讀取房間列表」；讀取成功但沒有房間顯示「尚無房間」；已有房間但輸入關鍵字沒有符合項目顯示「沒有符合條件的房間」；找到項目則顯示「請選擇房間」。搜尋失敗保留上一份成功快照及原有錯誤提示，不把錯誤誤判成「無房間」。

文件庫：`williechen/copilot_use/main`；唯一程式來源（唯讀）：`williechen/rust_ai_chat/main@f3f8255b0b701a1b57cb8c0a1737c3caa4319f2d`。接續 Chat Day32 的「歷史尚未載入」揭露；Day32 已在主體程式有部分驗證證據，但歷史查詢、作者、逐房 ACL、長連線撤銷仍是獨立缺口。此篇只處理**列表狀態語意**，不假稱歷史或 ACL 完成。

真實呼叫鏈：`HomePage/RoomPage → RoomList (crates/ui/src/lib.rs) → room_api::list_rooms/search_rooms → Axum 既有房間 API → Application/Domain → SQLx SQLite`；本篇重用 API、持久化與現有 `filter_rooms`，只在 Leptos signal 上辨識是否曾成功取得快照，不建立第二套資料來源。

## ② 實作：Red → 完整最小 Green → Refactor

### Red：先固定四種狀態（預期編譯失敗）

在 `crates/ui/src/lib.rs` 的檔案末端新增測試。現有來源尚無 `room_list_feedback`，因此套用 Red 後預期 **E0425 missing symbol**；不是宣稱已執行的測試失敗。

```rust
#[cfg(test)]
mod day33_room_list_feedback_tests {
    use super::room_list_feedback;

    #[test]
    fn distinguish_not_loaded_empty_filtered_and_found() {
        assert_eq!(room_list_feedback(false, 0, 0), "請先讀取房間列表");
        assert_eq!(room_list_feedback(true, 0, 0), "尚無房間");
        assert_eq!(room_list_feedback(true, 3, 0), "沒有符合條件的房間");
        assert_eq!(room_list_feedback(true, 3, 2), "請選擇房間");
    }
}
```

**瀏覽器 Red（尚未執行）：** 首頁載入後未讀取列表、成功載入三房、在搜尋欄輸入不存在的房名。現行 `RoomList` 的 `<ul>` 會變空，但其 `<p role="status">` 仍可能維持「請選擇房間」，無法辨識「尚未載入／零筆／本地篩選零筆」。讀取失敗須保留先前房間連結。

### Green A：新增純狀態投影函式

在 `crates/ui/src/lib.rs`、既有 `filter_rooms` 之後加入完整函式；不新增 crate。

```rust
fn room_list_feedback(loaded: bool, total: usize, visible: usize) -> &'static str {
    if !loaded {
        "請先讀取房間列表"
    } else if total == 0 {
        "尚無房間"
    } else if visible == 0 {
        "沒有符合條件的房間"
    } else {
        "請選擇房間"
    }
}
```

### Green B：接回真實 RoomList 的成功路徑

在 `fn RoomList()` 原有 `let (loading, set_loading) = signal(false);` 後新增：

```rust
let (has_snapshot, set_has_snapshot) = signal(false);
```

在 `query_room` 內 `match room_api::search_rooms(&submitted).await` 的 `Ok(value) => {` 分支，緊接分支開頭新增：

```rust
set_has_snapshot.set(true);
```

在「讀取／重新整理房間」按鈕的 `spawn_local` 中，將原本的以下三行：

```rust
let result = room_api::list_rooms().await;
let (next_rows, next_query, next_status) = refresh_result_view(previous, previous_query, result);
set_rows.set(next_rows);
```

完整替換為：

```rust
let result = room_api::list_rooms().await;
let succeeded = result.is_ok();
let (next_rows, next_query, next_status) =
    refresh_result_view(previous, previous_query, result);
if succeeded {
    set_has_snapshot.set(true);
}
set_rows.set(next_rows);
```

保留原本 `set_query`、`set_status`、`set_loading`，**失敗不把 has_snapshot 由 false 設為 true**；已有成功快照則仍顯示既有連結和原錯誤訊息。

最後在 `RoomList` 的 `view!` 內，**只替換**原本從 `<ul>{move || filter_rooms(...)` 至對應 `</ul>` 的整個區塊為：

```rust
<div class="room-results" aria-live="polite">
    {move || {
        let all = rows.get();
        let visible = filter_rooms(&all, &query.get());
        let feedback = room_list_feedback(
            has_snapshot.get(), all.len(), visible.len()
        );
        view! {
            <p class="room-result-feedback" role="status">{feedback}</p>
            <ul>
                {visible.into_iter().map(|(id, name)| view! {
                    <li>
                        <a
                            rel="external"
                            aria-current=if params.read().get("room_id").as_deref()
                                == Some(id.as_str()) { Some("page") } else { None }
                            href=format!("/room/{id}")
                        >{name}</a>
                    </li>
                }).collect_view()}
            </ul>
        }
    }}
</div>
```

這個區塊沿用 `params`、`rows`、`query`，連結路徑、房間 ID、SSR/Hydration 及原有按鈕完全不改。原先請求層的 `<p role="status">{move || status.get()}</p>` 保留，用於錯誤／載入中／重試訊息；新 `room-result-feedback` 專門說明目前顯示的結果。沒有成功快照前，使用者輸入文字不會被誤判成「尚無房間」。

### Refactor：職責分離與非目標

`room_list_feedback` 只投影已取得的列表快照，不查 DB；`filter_rooms` 仍負責中文大小寫不敏感的本地搜尋；`room_api` 負責 HTTP，後端授權及 SQLx 不變。後續可將結果提示獨立成 Leptos 元件，但本日不抽象化。沒有實際新建 `rust_ai_chat` commit。

### 上一 Day 與 FIFO 補教狀態

Chat Day32 的歷史載入未接通提示已在來源程式；真正 SQLite 歷史讀回、訊息作者、逐房 ACL、SSR/瀏覽器完整驗收仍保留。更早 Day25 共用認證 FIFO pending，本篇不聲稱結案，也不回寫歷史原文。

## ③ 套件與軟體版本（2026-10-24 官方核對）

| 項目 | rust_ai_chat manifest／lock | 當日官方最新穩定 | 選版與相容性 |
| --- | --- | --- | --- |
| Rust toolchain | `rust-toolchain.toml: stable`；本輪實際安裝版未重新量測 | **1.99.1**（2026-10-15） | edition 2024 可用；保留 stable，套用前執行 `rustc -V`；[官方公告](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) |
| Leptos | `0.8`／lock **0.8.22** | **0.8.22**（2026-10-05；0.9.0-rc0 排除） | 沿用 0.8 SSR/Hydration API；[官方 releases](https://github.com/leptos-rs/leptos/releases/tag/v0.8.22) |
| Leptos Router | `0.8`／lock **0.8.17** | 本輪僅核對 lock 的 0.8.17，獨立最新 patch 尚未從官方 release 證實 | 本篇未改 Router API；維持 lock；[官方專案](https://github.com/leptos-rs/leptos) |

`Cargo.lock` 精確來源 blob `85d844f618f0fd31a81775aa25f4c0af1a3754cd`。此篇未增依賴、未升級、未在 WASM/native 環境重新安裝；Leptos Router 最新 patch 尚需正式核實，故**第③節 Gate 1 尚待補核**，不以已核對 Leptos 主 crate 冒稱所有版本皆最新。

## ④ 驗證命令、預期／實際與未完成

| 驗證 | 命令／操作 | 預期 | 本次實際 |
| --- | --- | --- | --- |
| Red | `cargo test --locked -p ui --features ssr day33_room_list_feedback -- --nocapture` | 新函式不存在，編譯失敗 | 未執行 |
| Green | 同上（套用 Green 後） | 4 種狀態測試通過 | 未執行 |
| SSR | `cargo test --locked -p ui --features ssr` | 原有 SSR 測試不退化 | 未執行 |
| WASM | `cargo check --locked -p ui --features hydrate --target wasm32-unknown-unknown` | Leptos Hydration 可編譯 | 未執行 |
| 瀏覽器 | 登入／讀取房間→搜尋命中／未命中→故障注入→重新整理 | 不誤導、保留連結及錯誤 | 未執行 |
| DB／ACL | 依來源真實 API/SQLite 測試與瀏覽器授權 | 不改資料／權限 | 未執行 |

**交付狀態：** 僅教學文件候選；沒有在 rust_ai_chat 套用，沒有 CI/DB/瀏覽器新驗收。Gate 2 未通過；Day32 與 Day25 未完成事項保留。