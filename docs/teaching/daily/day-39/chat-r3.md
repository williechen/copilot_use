# Chat Day39 r3｜首頁與側欄首次自動載入：已實作版本與本地驗證

> 2026-10-26：按最新已實作程式及本地驗證修訂；此為前一輪本地修訂，隨本次Day40歷史勘誤提交；下文未push等敘述限原驗證時點。原[初版](chat.md)、[r1](chat-r1.md)、[r2](chat-r2.md)及舊 patch 保留。Day40、FIFO、last_full_comparison 均不變。

產品來源 `williechen/rust_ai_ring@bb259aaf76542554a3a704d9c4f4a3a219d65cbf`；文件編修基底 `d8d9f4ddf46d923c6abcc83ef0b86e5439eee4c4`。本文程式逐段取自此精確來源，[來源片段與 hashes](r3-source-excerpts.json)可核對。共用規則與需求依文件基底版本；本修訂整理既有 Day39，沒有新增一天的功能或重構計功。

## ① 目標、現況與前置條件

首次進入首頁或房間側欄時，自動走既有 `room_api::search_rooms("")`，不要求使用者先按重新整理。最新來源已含 WASM-only `Effect::new`、初載守衛與登入契約測試模組，不再描述成未套用候選。`HomePage`／`RoomPage` 重用 `RoomList`；SSR 不執行瀏覽器 fetch。

前置：Chat workspace、已安裝 `wasm32-unknown-unknown`，以及既有登入／房間 API。本文不改首頁 Session 管理入口、房間 ACL 或歷史訊息需求。

## Refactor

### 登入測試責任與正式模組接線

原本 Day28 的 POST／欄位契約與 Day34 的唯一表單／label 契約散在 `lib.rs`。最新程式把兩個完整測試移到 `signin_contract.rs`，仍測同一正式 `SignInPage`，不複製登入 HTML。與 r2 候選不同，新檔明確 `use crate::SignInPage`，沒有透過整個父模組 glob 匯入。

`lib.rs` 的 module/import 接線如下；`signin_contract` 只在 test＋ssr 載入。`should_auto_load` 的 cfg 使用來源原文，不藉改文把它換成另一個實作。

來源：[`chat_apps/chat_ui/src/lib.rs`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/chat_apps/chat_ui/src/lib.rs#L1)。

```rust
mod chat_socket;

pub mod account;
#[cfg(target_arch = "wasm32")]
mod room_api;
mod room_list_state;

#[cfg(all(test, feature = "ssr"))]
mod signin_contract;

use std::rc::Rc;

use chat_domain::{ChatMessage, MessageAuthor};
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_params_map,
    path,
};

use room_list_state::{filter_rooms, room_list_feedback};

#[cfg(any(target_arch = "wasm32", test))]
use room_list_state::refresh_result_view;

#[cfg(any(target_arch = "wasm32"))]
use room_list_state::should_auto_load;
```

完整移轉後測試新檔：

來源：[`chat_apps/chat_ui/src/signin_contract.rs`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/chat_apps/chat_ui/src/signin_contract.rs#L1)。

```rust
use crate::SignInPage;
use leptos::prelude::*;

#[test]
fn day28_leptos_signin_posts_to_existing_login_handler() {
    let html = view! { <SignInPage /> }.to_html();
    assert!(html.contains("<form"));
    assert!(html.contains("method=\"post\""));
    assert!(html.contains("action=\"/login\""));
    assert!(html.contains("name=\"username\""));
    assert!(html.contains("name=\"password\""));
    assert!(html.contains("type=\"password\""));
    assert!(html.contains("autocomplete=\"current-password\""));
    assert!(html.contains("/signup"));
}

#[test]
fn existing_signin_is_the_single_login_form() {
    let html = view! { <SignInPage/> }.to_html();
    assert_eq!(html.matches("<form").count(), 1);
    for marker in [
        "method=\"post\"",
        "action=\"/login\"",
        "for=\"login-username\"",
        "id=\"login-username\"",
        "name=\"username\"",
        "for=\"login-password\"",
        "id=\"login-password\"",
        "name=\"password\"",
        "type=\"password\"",
        "autocomplete=\"current-password\"",
        "href=\"/signup\"",
    ] {
        assert!(html.contains(marker), "missing {marker}");
    }
}
```

兩組斷言都保留：第一組保護既有 `/login` handler 與帳密欄位，第二組額外保護單一 form、label/id 配對與 `/signup` 連結。測試搬檔不代表登入／Session 用例或房間權限已完成分層。這項 Refactor 與初次載入功能分開計；下一步仍沿真實登入／Session、進房 caller 整理責任。

## ② Red → 現行 Green 與斷言承接

### Red 的原始目的與本次證據邊界

缺 `should_auto_load` 時，四種布林組合測試會是缺符號編譯 Red；舊 UI 還需觀察首次 GET 次數。這兩個 Red 本次沒有在移除實作後重跑。下列測試在最新程式實際為 Green，不能重新標成已執行 Red。

來源：[`chat_apps/chat_ui/src/room_list_state.rs::should_auto_load`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/chat_apps/chat_ui/src/room_list_state.rs#L41)。

```rust
#[cfg(any(target_arch = "wasm32", test))]
pub(super) fn should_auto_load(has_snapshot: bool, loading: bool) -> bool {
    !has_snapshot && !loading
}
```

來源：[`chat_apps/chat_ui/src/room_list_state.rs::day39_initial_load_is_once_and_does_not_interrupt_pending_request`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/chat_apps/chat_ui/src/room_list_state.rs#L135)。

```rust
    #[test]
    fn day39_initial_load_is_once_and_does_not_interrupt_pending_request() {
        assert!(should_auto_load(false, false));
        assert!(!should_auto_load(false, true));
        assert!(!should_auto_load(true, false));
        assert!(!should_auto_load(true, true));
    }
```

### Green：完整 RoomList caller 與狀態回饋

以下完整元件保留搜尋、手動刷新、loading 防重入、快照與狀態 DOM。Effect 只讀 untracked signal，不因 loading／資料變更重跑；失敗後的重試仍走既有手動操作。這是來源實作說明，首次只發一次 GET 仍待真瀏覽器驗證。

來源：[`chat_apps/chat_ui/src/lib.rs::RoomList`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/chat_apps/chat_ui/src/lib.rs#L305)。

```rust
#[component]
fn RoomList() -> impl IntoView {
    let params = use_params_map();
    let (query, set_query) = signal(String::new());
    let (rows, set_rows) = signal(Vec::<(String, String)>::new());
    let (loading, set_loading) = signal(false);
    let (has_snapshot, set_has_snapshot) = signal(false);
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
            let previous = rows.get_untracked();
            let result = room_api::search_rooms(&submitted).await;
            apply_room_search(
                previous,
                submitted,
                result,
                set_rows,
                set_has_snapshot,
                set_status,
                set_loading,
            );
        });
        #[cfg(not(target_arch = "wasm32"))]
        {
            set_loading.set(false);
            set_rows.set(Vec::new());
            set_has_snapshot.set(false);
        }
    };

    // chat_apps/chat_ui/src/lib.rs::RoomList，query_room closure 宣告之後、view! 之前
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if should_auto_load(has_snapshot.get_untracked(), loading.get_untracked()) {
            query_room(); // existing real search_rooms("") -> GET /api/rooms?q=
        }
    });

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
                        let previous_query = query.get_untracked();
                        let result = room_api::list_rooms().await;
                        let succeeded = result.is_ok();
                        let (next_rows, next_query, next_status) =
                            refresh_result_view(previous, previous_query, result);
                        if succeeded {
                            set_has_snapshot.set(true);
                        }
                        set_rows.set(next_rows);
                        set_query.set(next_query);
                        set_status.set(next_status);
                        set_loading.set(false);
                    });

                    #[cfg(not(target_arch = "wasm32"))]
                    { set_loading.set(false); }
            }>"讀取／重新整理房間"</button>
            <div class="room-results" aria-live="polite">
            {move || {
                let all = rows.get();
                let visible = filter_rooms(&all, &query.get());
                let feedback = room_status_text(
                    &status.get(),
                    has_snapshot.get(),
                    all.len(),
                    visible.len(),
                );
                view! {
                    <p class="room-result-feedback" role="status">{feedback}</p>
                    <ul>
                        {visible.into_iter().map(|(id, name)| view! {
                        <li>
                            <a
                                rel="external"
                                aria-current=if params.read().get("room_id").as_deref() == Some(id.as_str()) { Some("page") } else { None }
                                href=format!("/room/{id}")
                            >{name}</a>
                        </li>
                        }).collect_view()}
                    </ul>
                }
            }}
            </div>
        </section>
    }
}
```

搜尋回應的正式狀態處理仍由以下函式承接：

來源：[`chat_apps/chat_ui/src/lib.rs::apply_room_search`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/chat_apps/chat_ui/src/lib.rs#L53)。

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
    let (next_rows, _retry_query, next_status) = refresh_result_view(previous, submitted, result);
    if succeeded {
        set_has_snapshot.set(true);
    }
    set_rows.set(next_rows);
    set_status.set(next_status);
    set_loading.set(false);
}
```

Cargo 接線維持 `chat-ui` 的 ssr/hydrate features，workspace metadata 指向 `chat-server`／`chat-ui`。沒有新增 dependency、migration 或 lock 變更；Leptos UI 透過 WASM `room_api` 呼叫既有 API，沒有在 SSR 中加入瀏覽器 fetch。Day37 成功／錯誤與 Day38 DOM 回饋斷言均承接在 workspace SSR 測試中。

## ③ 來源版本與執行環境

| 項目 | 精確來源或已執行環境 |
| --- | --- |
| Rust / Cargo | 實測 1.99.1；edition 2024 |
| axum | Cargo.lock resolved `0.8.9` |
| leptos | Cargo.lock resolved `0.8.22` |
| sqlx | Cargo.lock resolved `0.9.0` |
| tokio | Cargo.lock resolved `1.54.1` |
| uuid | Cargo.lock resolved `1.29.0` |

四個 workspace 的 manifest／lock 已重新讀取；沒有改版本、依賴或 features。本文列的是來源 resolved 及當次實測版本，不宣稱本輪另做官方「最新穩定版」查核或升級；r2 的官方查核表保留其原日期與範圍。

## ④ 實測命令、證據與未完成

最新精確 SHA 實測：**57 項 Rust 測試通過、0 失敗；SSR server／WASM hydrate check 通過**；`cargo fmt --all -- --check` exit 0。這些已於前一輪驗證實際執行，本輪編修只重核來源與保存證據，沒有重跑或重複計功。

| 命令 | exit code | 原始 log |
| --- | --- | --- |
| `cargo fmt --all -- --check` | 0 | [log](r3-local-validation/logs/latest-chat-fmt.log) |
| `cargo check --offline --locked -p chat-server --features ssr` | 0 | [log](r3-local-validation/logs/latest-chat-check-server.log) |
| `cargo test --offline --locked --workspace --features ssr` | 0 | [log](r3-local-validation/logs/latest-chat-test.log) |
| `cargo check --offline --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown` | 0 | [log](r3-local-validation/logs/latest-chat-check-wasm.log) |

當次工作目錄：`/home/willie/Documents/Codex/2026-10-26/referenced-chatgpt-conversation-this-is-an/work/latest/chat_apps`。完整工作目錄、環境覆寫、耗時及 log SHA256 見[逐命令紀錄](r3-local-validation/commands.jsonl)。表中 Cargo 以 `--offline --locked` 執行；快取命中不代表跳過測試，log 保留實際測試名稱與結果。

教學舊基底 `8fc4b27` 另有隔離候選：Refactor → Green 的兩份 patch 均先 `git apply --check` exit 0，再 `git apply` exit 0，之後實際 Cargo check/test 通過；但三案候選 fmt 均 exit 1，Chat/Pet 的 diff whitespace 檢查亦失敗。這些舊 patch 不適合重貼最新來源；最新來源正向套用檢查全部失敗，且不等同舊 patch 逐位元結果。新程式直接以最新 SHA 驗證。

尚未執行：真瀏覽器首頁／進房側欄首次各 1 次 GET、SSR hydration 重複請求、500 後錯誤與舊清單可見、快速切頁／舊回應競態、窄螢幕與完整 Session／房間 ACL 驗收。

[完整兩組驗證摘要](r3-local-validation/README.md)保留初次工作目錄被刪除與空間處理中斷；中斷不計成功。`rust_ai_ring` 本輪文件編修保持唯讀；未 push、未執行新 CI、未封存／宣稱 Gate 2 全部完成。
