# Chat Day19 修正版：刷新失敗保留已載入房間
## 1. 學習大綱
來源 `rust_ai_chat/main=f692c2ebe7b414fe49858c1ab1c4a33ce7c3dfee`；教學分支 `44dabb3eebed12639cd949b93521cbdc47a401fb`。本篇為 Day19 r1 完整教學修訂；原稿的占位、缺碼與契約說明已修正。教學測試在獨立候選副本執行，使用者套用、產品整合及完整功能驗收仍未完成。主體 Red 5 分鐘、Green 10 分鐘，Refactor／額外驗證另計；實際測試結果見[完整驗證紀錄](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-19-day19-review/驗證結果.md)。完整修改檔與原始日誌見[查核證據目錄](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-19-day19-review)；[修正 patch](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-19-day19-review/corrected.patch)的基底為上述來源 SHA。

目標：已成功載入房間後，刷新失敗時保留所有既有 (id,name)，顯示可重試提示；成功回傳空列表則清空。前置：Leptos 現有 RoomList、room_api API 及房間 ID 驗證。原程式在 HTTP 前清空 rows，失敗即遺失舊清單。

呼叫鏈：RoomList 刷新 callback → list_rooms → search_rooms("") → GET /api/rooms?q= → Axum list_rooms → 空 q 走 RoomRepository::list／SQLite → JSON(id,name,category_room_id) → (id,name) → filter_rooms → /room/{id} 連結。非空查詢走 search_by_name。這次保留既有 query 篩選，因此變更搜尋輸入仍可能隱藏部分已保存房間。
## 2. 實作內容
### Red
把下方 tests 模組中的 day19 測試先加入既有 `day16_room_search_tests`，尚未新增 helper 時，預期 E0425 缺少 refreshed_room_list。原草稿單一測試已實際重現此編譯失敗；它不是 UI 行為失敗。
### 完整最小 Green
檔案 `crates/ui/src/lib.rs`。helper 放在 `#[component] fn RoomList` 之前，不能插入 attribute 與函式之間；以以下整個 component 替換原 RoomList，保留原模組中其他函式。

```rust
fn refreshed_room_list(
    previous: Vec<(String, String)>,
    result: Result<Vec<(String, String)>, String>,
) -> (Vec<(String, String)>, String) {
    match result {
        Ok(value) => {
            let status = if value.is_empty() {
                "尚無房間"
            } else {
                "請選擇房間"
            };
            (value, status.to_owned())
        }
        Err(message) => (
            previous,
            format!("{message}，請重試"),
        ),
    }
}


#[component]
fn RoomList() -> impl IntoView {
    let params = use_params_map();
    let (query, set_query) = signal(String::new());
    let (rows, set_rows) = signal(Vec::<(String, String)>::new());
    let (loading, set_loading) = signal(false);
    let (status, set_status) = signal(String::from("請讀取房間列表"));

    let query_room = move || {
        if loading.get_untracked() {
            return;
        }
        set_loading.set(true);
        set_status.set("查詢中…".into());
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            let submitted = query.get_untracked();
            match room_api::search_rooms(&submitted).await {
                Ok(value) => {
                    set_status.set(
                        if value.is_empty() {
                            if submitted.trim().is_empty() {
                                "尚無房間"
                            } else {
                                "沒有符合條件的房間"
                            }
                        } else {
                            "查詢完成，請選擇房間"
                        }
                        .into(),
                    );
                    set_rows.set(value);
                }
                Err(message) => {
                    set_status.set(format!("查詢失敗：{message}，請重試"));
                }
            }
            set_loading.set(false);
        });
        #[cfg(not(target_arch = "wasm32"))]
        {
            set_loading.set(false);
        }
    };

    view! {
        <section>
            <h2>"房間列表"</h2>
            <input
                type="search"
                aria-label="搜尋房間"
                placeholder="房間名稱"
                prop:value=move || query.get()
                on:input=move |event| set_query.set(event_target_value(&event))
            />

            <button
                type="button"
                disabled=move || loading.get()
                on:click=move |_| { query_room() }
            >"查詢房間"</button>

            <button
                type="button"
                disabled=move || loading.get()
                on:click=move |_| {
                    if loading.get_untracked() { return; }
                    set_loading.set(true);
                    set_status.set("載入中…".into());

                    #[cfg(target_arch = "wasm32")]
                    wasm_bindgen_futures::spawn_local(async move {
                        let previous = rows.get_untracked();
                        let result = room_api::list_rooms().await;
                        let (next_rows, next_status) = refreshed_room_list(previous, result);
                        set_rows.set(next_rows);
                        set_status.set(next_status);
                        set_loading.set(false);
                    });

                    #[cfg(not(target_arch = "wasm32"))]
                    { set_loading.set(false); }
            }>"讀取／重新整理房間"</button>
            <p role="status">{move || status.get()}</p>
            <ul>{move || filter_rooms(&rows.get(), &query.get()).into_iter().map(|(id, name)| view! {
                <li>
                    <a
                        rel="external"
                        aria-current=if params.read().get("room_id").as_deref()==Some(id.as_str()) { Some("page") } else { None }
                        href=format!("/room/{id}")>{name}
                    </a>
                </li>
            }).collect_view()}</ul>
        </section>
    }
}
```

以下測試模組含既有測試及本日新增項；取代同名既有模組，勿重複宣告。

```rust
#[cfg(test)]
mod day16_room_search_tests {
    use super::filter_rooms;
    #[test]
    fn day19_empty_success_replaces_previous_rows() {
        let (rows, status) = super::refreshed_room_list(
            vec![("old".into(), "舊房間".into())], Ok(vec![]));
        assert!(rows.is_empty());
        assert_eq!(status, "尚無房間");
    }
    #[test]
    fn day19_failure_preserves_complete_rows_and_message() {
        let old = vec![("one".into(), "同名房間".into()), ("two".into(), "同名房間".into())];
        let (rows, status) = super::refreshed_room_list(old.clone(), Err("讀取失敗：503".into()));
        assert_eq!(rows, old);
        assert_eq!(status, "讀取失敗：503，請重試");
    }

#[test]
fn day19_refresh_failure_keeps_previous_room_links() {
    let old = vec![("room_1".into(), "Rust 技術交流".into())];
    let (rows, status) =
        super::refreshed_room_list(old.clone(), Err("503".into()));
    assert_eq!(rows, old);
    assert!(status.contains("請重試"));

    let (rows, status) = super::refreshed_room_list(
        old,
        Ok(vec![("room_2".into(), "生活閒聊".into())]),
    );
    assert_eq!(rows[0].0, "room_2");
    assert_eq!(status, "請選擇房間");
}


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

    #[test]
    fn day18_query_filter_preserves_room_ids() {
        let rows = vec![
            ("room_1".to_string(), "Rust 技術交流".to_string()),
            ("room_2".to_string(), "生活閒聊".to_string()),
        ];
        assert_eq!(super::filter_rooms(&rows, " 技術 "), vec![rows[0].clone()]);
        assert_eq!(super::filter_rooms(&rows, " "), rows);
    }
}
```

### Refactor
用 helper 集中成功／失敗的資料與提示轉換，不改 HTTP JSON、房間 ID、搜尋及網址契約。錯誤 message 已由 room_api 加上「讀取失敗」，helper 只加重試文字，避免原建議的重複前綴。loading 防重入及 await 後復原保留。helper 單元測試不會證明 callback 沒有清空 rows，必須另驗 WASM 及瀏覽器。
## 3. 套件版本
Cargo.lock：leptos 0.8.22、leptos_router 0.8.17、web-sys 0.3.106、wasm-bindgen 0.2.129、wasm-bindgen-futures 0.4.79。
本輪 Linux x86_64，rustc 1.99.1／cargo 1.99.1、stable 工具鏈，WASM target 已安裝。專案未宣告 rust-version，最低 MSRV 未驗證；其他作業系統未驗證。無新增套件或 migration，Cargo.lock 未改。

## 4. 驗證項目
```sh
cargo test --locked -p ui --features ssr day19
cargo test --locked -p ui --features ssr
cargo check --locked -p ui --features hydrate --target wasm32-unknown-unknown
```
單元涵蓋失敗保留同名不同 ID、成功替換、空成功清空與完整提示。SSR 既有「歷史載入尚未接通」斷言不在本日修正範圍，結果單獨列示。未執行 cargo leptos 完整建置或瀏覽器 200→503→200；瀏覽器需確認 loading 時舊連結仍存在、失敗保留 href、重試成功替換。後端 repository 錯誤原生為500，503可由測試代理注入。解析／fetch 失敗也應保留。來源 CI 成功不包含本次候選，產品整合及完整驗收未完成。

本輪實際驗證：Day19 Green 3 passed；WASM check exit0；套件回歸6 passed／1 failed（既有SSR提示斷言，原始來源同樣失敗）。Red已實測E0425缺helper。瀏覽器未測。

來源與證據：規則 daily-tdd-minimum-v1，GitHub規則blob `d33e42581be48d495ff7a2e32ac97821ab08df6e`；Notion規則頁最後編輯 `2026-10-18T16:17:59.565Z`。[逐項差異與更正](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-19-day19-review/核對報告.md)、[驗證日誌及CI精確SHA／workflow／job](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-19-day19-review)。本修訂從Notion原文與真實GitHub來源重建；未取得原ZIP，不聲稱原ZIP逐字核驗。
