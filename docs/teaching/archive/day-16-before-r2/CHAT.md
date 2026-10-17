## 1. 學習大綱
來源：`williechen/rust_ai_chat/main` `fced2861bba21c33697051f685c50689f8052de8`；同 SHA 六個 workflows / 18 checks Success（[CI](https://github.com/williechen/rust_ai_chat/actions/runs/38992992860)）。
狀態：**教學待套用**；本篇程式與測試**未實際編譯、未執行**；不能把來源 CI 視為本篇驗收。時間：Red＋最小 Green 20 分鐘。
承接 Day 15 已實作 `GET /api/rooms → room_api::list_rooms → RoomList → /room/:room_id`，不重寫 API；本日讓使用者能從真實列表**輸入關鍵字、即時縮小可選房間**，並在空結果時看見清楚狀態。對應 FUTURE_WORK Chat 前台房間欄；UTMall 分類清單只借用高密度列表，不捏造 category／Hit。尚未有分類 API，不做假分類。檔案：`crates/ui/src/lib.rs`，現有 `RoomList` 及 `HomePage`。
## 2. 實作內容
### Red（8 分鐘）
在 `crates/ui/src/lib.rs` 末尾新增以下測試。這是現有 `RoomList` 即將呼叫的純函式，不額外建立無用 Port。
```rust
#[cfg(test)]
mod day16_room_search_tests {
    use super::filter_rooms;

    #[test]
    fn search_matches_chinese_names_and_preserves_ids() {
        let rooms = vec![
            ("room_1".to_owned(), "Rust 技術交流".to_owned()),
            ("room_2".to_owned(), "生活閒聊".to_owned()),
        ];
        assert_eq!(filter_rooms(&rooms, " 技術 "), vec![rooms[0].clone()]);
        assert_eq!(filter_rooms(&rooms, ""), rooms);
        assert!(filter_rooms(&rooms, "不存在").is_empty());
    }
}
```
Red 預期：`filter_rooms` 尚不存在，編譯失敗；**未實際執行**。
### 最小 Green（12 分鐘）
在 `RoomList` 之前新增完整函式：
```rust
// 搜尋只作用於已由現有 API 取得的房間；不猜測權限與分類。
fn filter_rooms(rows: &[(String, String)], query: &str) -> Vec<(String, String)> {
    let needle = query.trim().to_lowercase();
    rows.iter()
        .filter(|(_, name)| name.to_lowercase().contains(&needle))
        .cloned()
        .collect()
}
```
將現有 `RoomList` **完整替換**為：
```rust
#[component]
fn RoomList() -> impl IntoView {
    let (rows, set_rows) = signal(Vec::<(String, String)>::new());
    let (query, set_query) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (status, set_status) = signal(String::from("請讀取房間列表"));

    view! {
        <section aria-label="聊天室清單">
            <h2>"房間列表"</h2>
            <label for="room-search">"搜尋聊天室"</label>
            <input id="room-search" type="search" placeholder="輸入房間名稱"
                prop:value=move || query.get()
                on:input=move |event| set_query.set(event_target_value(&event)) />
            <button type="button" disabled=move || loading.get()
                on:click=move |_| {
                    if loading.get_untracked() { return; }
                    set_loading.set(true);
                    set_rows.set(Vec::new());
                    set_status.set("載入中…".into());
                    #[cfg(target_arch = "wasm32")]
                    wasm_bindgen_futures::spawn_local(async move {
                        match room_api::list_rooms().await {
                            Ok(value) => {
                                set_status.set(if value.is_empty() {
                                    "尚無房間"
                                } else { "請選擇房間" }.into());
                                set_rows.set(value);
                            }
                            Err(message) => set_status.set(message),
                        }
                        set_loading.set(false);
                    });
                    #[cfg(not(target_arch = "wasm32"))]
                    { set_loading.set(false); }
                }>"讀取／重新整理房間"</button>
            <p role="status">{move || status.get()}</p>
            {move || {
                let filtered = filter_rooms(&rows.get(), &query.get());
                let no_match = !rows.get().is_empty() && filtered.is_empty();
                view! {
                    <div>
                        {no_match.then(|| view! { <p>"沒有符合搜尋的聊天室"</p> })}
                        <ul>{filtered.into_iter().map(|(id, name)| view! {
                            <li><a href=format!("/room/{id}")>{name}</a></li>
                        }).collect_view()}</ul>
                    </div>
                }
            }}
        </section>
    }
}
```
呼叫鏈：既有按鈕觸發 `room_api::list_rooms`，成功後寫入 `rows`；`query` 每次輸入改變即執行 `filter_rooms`；點擊結果仍走原 `/room/{id}`。**搜尋不改後端、不能繞過既有房間授權。**
### Refactor
### 根目錄 tests 遷移計畫
範圍僅為根目錄 `tests/chat/`：`persist_publish.rs` → `crates/chat_application/tests/persist_publish.rs`；`product_event_contract.rs` → `crates/chat_application/tests/product_event_contract.rs`。這兩個檔案目前測的是 chat-application 的公開契約，不應因名稱含 Chat 就全搬到 server。既有 `apps/server/tests/root_hub_contract.rs` 保留。
遷移是配合本日主功能的整理工作，**不拆分各原始檔內既有單元測試，不要求每個檔案另建 **[**tests.rs**](http://tests.rs)。
Red：以 `tests/Cargo.toml` 的九個明確 `[[test]]` 為基準，記錄測試名稱、數量、既有失敗與環境需求。Green：搬移上述檔案，將所需 dev-dependencies 加到目的 package，移除 workspace-tests 對應登記及不再使用的相依；維持原斷言。Refactor：將 `.github/workflows/test.yml` 的 `cargo test --locked -p workspace-tests` 改為目的 package 的測試命令，確保 CI 持續收集全部案例。新增功能測試與純搬移分開提交／驗證。
三專案搬完後再統一檢查：根目錄 `tests/common/mod.rs` 目前為空；確認無剩餘共用或跨專案測試及引用後，才移除 `tests/Cargo.toml`、工作區 `members` 的 `tests` 與空目錄，並更新 lockfile。若仍有真正跨專案測試則保留其 workspace-tests 容器與 CI，不強制清空。
驗證：`cargo test -p chat-application --test persist_publish --test product_event_contract`；搬移前後比對案例，不能出現漏跑、重複、無故 ignored 或以刪測試取得通過。**目前僅更新教學計畫；尚未搬移、未執行上述驗證。**
本日不搬移既有 Day 15 RoomHub tests，已歸位於 `apps/server/tests/root_hub_contract.rs`。未更動 `Cargo.toml`／`Cargo.lock`；待使用者套用後以 UI 實測確認 Leptos closure 與 hydration。避免重複建置 room API。
### 舊 Day 更正補充
Day 15 的房間列表、選房已在 `fced2861` 出現並通過來源 CI，不再以 Day 15 舊快照宣稱缺少 Green；原文封存。UTMall 截圖中分類與 Hit 不是現有 DTO，先做**真實名稱搜尋**，分類需後續模型／API 及 ACL 證據；x5 截圖的在線名單屬另一功能，不冒充本日完成。
## 3. 套件版本
使用原 `leptos`、`leptos_router` 與 `web-sys`，無新增套件。Cargo 實際版本以本 SHA 工作區宣告／lock 為準；**本輪未核實官方最新 stable、MSRV 或 WASM 實測，不能宣稱已通過**。
## 4. 驗證項目
1. Red：`cargo test -p ui day16_room_search_tests`，預期新增函式前編譯失敗；**未執行**。
2. Green：`cargo test -p ui day16_room_search_tests`、`cargo check -p ui`；**未執行**。
3. 真實瀏覽器：載入列表、中文搜尋、空結果、清空搜尋、點選進房、建房後刷新、失敗重試、手機版；**未執行**。
4. 來源 CI：6/6 workflows、18/18 checks Success，僅證實**舊 SHA 原始碼**，不等於本篇修改已驗證。
5. 正式功能驗收：**待套用與 E2E**。
