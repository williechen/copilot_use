# Chat Day31 r1｜在個人頁真正建立聊天室

## ① 學習大綱：目標、現況與前置條件

本篇從即時 `docs/learning-state.json` 的 `next_day.chat=31` 接續。文件正本 `williechen/copilot_use/main`，初讀 SHA `259a1e6f08a6522ef596fe1448af4ac2bed56167`；程式來源為唯讀的 `williechen/rust_ai_chat/main@1d1866dbbb30255fafa4c7f74b1868919662913c`。規則 `daily-tdd-minimum-v1 / github-only-v1`，規則 blob 與讀取時間另存同目錄交付證據。

最新 `apps/server/src/lib.rs` 已保護建房 POST，`signin.rs::login` 成功導向 `/account`，但 account 只有登出與回首頁，使用者仍無法在個人頁建房。本日新增「登入→個人頁輸入房名→提交→SQLite 寫入→可點擊進房」；不重教已套用的 API Session Gate。

呼叫鏈：Leptos `AccountView` 原生表單 → Axum `/account/rooms` → `active_member` 與 CSRF → 既有 `chat_application::create_room` → `SqliteRoomRepository` → rooms。Leptos 負責畫面，Axum 負責 Session、表單與 response；不另建帳號或房間表。這是可操作的 SSR 表單切片，不需要把 SQLx 或秘密放入 WASM，也不替換既有 Chat SSR/hydration/Router。

前置條件：使用者先在自己的 rust_ai_chat 工作副本套用本文；保留現有 migrations、Cargo.lock 與所有既有功能。核心教學預計 15 分鐘；三案各 15 分鐘加共用 15 分鐘，合計 60 分鐘；來源、版本、補教、發布與實測另計。本次未套用、未跑 Rust 測試；執行環境 `rustc --version` 原始錯誤為 `/bin/bash: line 1: rustc: command not found`（exit 127），不是 Red。

## ② 實作內容：Red → 完整 Green → Refactor

### Red：先固定畫面契約，再從登入後的真實入口驗證

先在 `crates/ui/src/lib.rs` 尾端新增測試；Green 前 `account` module 尚不存在，預期為 missing-symbol 編譯失敗，未實際執行。不能拿既有 API 測試的成功當作 Red。

```rust
#[cfg(test)]
mod day31_account_tests {
    #[test]
    fn account_has_operable_creation_form_and_escaped_feedback() {
        let html = crate::account::render_account("logout-token".into(), "create-token".into(), "<失敗>".into(), "<房名>".into(), None);
        assert!(html.contains("action=\"/account/rooms\""));
        assert!(html.contains("name=\"name\""));
        assert!(html.contains("name=\"csrf\" value=\"create-token\""));
        assert!(html.contains("&lt;失敗&gt;"));
        assert!(html.contains("&lt;房名&gt;"));
        assert!(!html.contains("<失敗>"));
    }
}
```

行為 Red：登入後開 `/account`，嘗試用鍵盤建立「中文房間」。來源頁沒有建房表單，無法完成流程；安全與成功矩陣見第④節。此行為失敗由來源推導，未宣稱已在瀏覽器重現。

### Green A：完整 Leptos 個人頁

在 `crates/ui/src/lib.rs` 頂層新增 `pub mod account;`。新建 `crates/ui/src/account.rs`，全文如下；它是既有 ui crate 的 SSR 元件，不新增另一個 Cargo 專案。

```rust
use leptos::prelude::*;

#[component]
fn AccountView(logout_csrf: String, create_csrf: String, message: String,
               name: String, room_id: Option<String>) -> impl IntoView {
    view! {
        <html lang="zh-Hant">
            <head><meta charset="utf-8"/><title>"個人管理"</title></head>
            <body><main>
                <h1>"個人管理"</h1>
                <p role="status">{message}</p>
                {room_id.map(|id| view! {
                    <p><a href=format!("/room/{id}")>"進入剛建立的房間"</a></p>
                })}
                <form method="post" action="/account/rooms">
                    <input type="hidden" name="csrf" value=create_csrf/>
                    <label for="room-name">"聊天室名稱"</label>
                    <input id="room-name" name="name" value=name required maxlength="200"/>
                    <button type="submit">"建立聊天室"</button>
                </form>
                <form method="post" action="/logout">
                    <input type="hidden" name="csrf" value=logout_csrf/>
                    <button type="submit">"登出"</button>
                </form>
                <a href="/">"回公開房間清單"</a>
            </main></body>
        </html>
    }
}

pub fn render_account(logout_csrf: String, create_csrf: String, message: String,
                      name: String, room_id: Option<String>) -> String {
    format!("<!doctype html>{}", view! {
        <AccountView logout_csrf create_csrf message name room_id/>
    }.to_html())
}
```

### Green B：重用 Session 與 Application／SQLite

在 `apps/server/src/auth_session.rs` 尾端新增：

```rust
pub fn csrf_for_create_room(headers: &HeaderMap) -> Result<String, StatusCode> {
    let raw = headers.get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| cookie_token(v, "chat_session"))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(token_digest(&format!("create-room:{raw}")))
}
```

在 `apps/server/src/signin.rs::routes` 的 `.route("/account", get(account))` 後插入一行 `.route("/account/rooms", axum::routing::post(account_create_room))`。仍使用原本的 pool、4096 bytes body limit。完整替換舊 `account` 函式，並在同檔新增下列其餘定義；原 login、login_page、測試全部保留。

```rust
fn account_html(headers: &HeaderMap, message: String, name: String,
                room_id: Option<String>) -> Result<Html<String>, StatusCode> {
    Ok(Html(ui::account::render_account(
        crate::auth_session::csrf_for_logout(headers)?,
        crate::auth_session::csrf_for_create_room(headers)?,
        message, name, room_id,
    )))
}

async fn account(State(pool): State<SqlitePool>, headers: HeaderMap)
    -> Result<Html<String>, StatusCode>
{
    crate::auth_session::active_member(&pool, &headers).await?;
    account_html(&headers, String::new(), String::new(), None)
}

#[derive(Deserialize)]
struct AccountRoomForm { name: String, csrf: String }

async fn account_create_room(
    State(pool): State<SqlitePool>, headers: HeaderMap, Form(form): Form<AccountRoomForm>,
) -> Result<Response, StatusCode> {
    crate::auth_session::active_member(&pool, &headers).await?;
    if headers.get(header::ORIGIN).and_then(|h| h.to_str().ok())
        != Some("http://127.0.0.1:3000")
        || form.csrf != crate::auth_session::csrf_for_create_room(&headers)?
    { return Err(StatusCode::FORBIDDEN); }

    if form.name.trim().is_empty() || form.name.len() > 200 || form.name.contains('\0') {
        return Ok((StatusCode::UNPROCESSABLE_ENTITY,
            account_html(&headers, "名稱須非空白、不得含 NUL，且最多 200 UTF-8 bytes".into(), form.name, None)?
        ).into_response());
    }
    let repo = persistence::SqliteRoomRepository::new(pool);
    let created = chat_application::create_room(&repo, chat_application::CreateRoom {
        id: uuid::Uuid::new_v4().to_string(),
        category_room_id: "default-category".into(),
        name: form.name.clone(),
    }).await;
    match created {
        Ok(room) => Ok((StatusCode::CREATED,
            account_html(&headers, "已建立；可進房或回公開清單".into(), String::new(), Some(room.id))?
        ).into_response()),
        Err(_) => Ok((StatusCode::SERVICE_UNAVAILABLE,
            account_html(&headers, "未確認建立成功；請先查清單再決定是否重送".into(), form.name, None)?
        ).into_response()),
    }
}
```

POST 的權限不依賴使用者先 GET 頁面；所有輸入仍在伺服器驗證，錯誤頁保留名稱且由 Leptos escape。DB 錯誤不顯示成功。成功回 201 搭配新房連結；重新整理 POST 仍可能重送，請先回清單，冪等 key／PRG 是尚待完成的改善，不假稱已去重。

### Refactor

原本 account 的完整字串頁已移入 Leptos；Axum 只授權及傳入資料。建房直接重用現有 application/repository，不複製 SQL、不新增平行資料表。表單與既有 JSON API 的名稱限制暫時相同；後續可把驗證提到同一 application value object，但這不是已完成的改動。首頁保持公開清單，建立功能留在個人頁。

### 上一 Day 未完成事項補充

來源為 Day30 r3 第④節。最新 SHA 已有 guarded POST，舊版「尚未套用」是當時交付狀態，不代表現在缺少 API。公開 GET、匿名／有效／撤銷 Session POST 的歷史驗證可從 `docs/teaching/current/STATUS.md` 連至 `2026-10-23-day30-validation-1d1866d/REPORT.md` 追溯；本次未重跑。此篇補齊原文所缺的實際 UI，逐房 ACL、ownership、訊息作者及 WS 存續撤銷仍待實作／驗證，不能一起結案。

### 舊 Day 更正補充：FIFO 第一項

`chat-day25-shared-auth-20261022` 保持 pending。本篇只重用既有 `active_member` 與 shared cookie/hash primitives，沒有把 Chat／Analytics 所有登入、登出、Session 政策完整共用並驗證；不得因新增 CSRF helper 宣稱完成該補教。Day25 以前原文與證據不修改，本次不結案任何 makeup_queue。

## ③ 套件與軟體版本（2026-10-23 核對）

來源均為官方公告或維護者發佈的 crate 文件；表中「最新」排除預發行。下列 lock 值由程式來源精確 SHA 的 Cargo.lock 全文解析，**不是本機已安裝或已編譯聲明**。

| 項目／官方來源 | manifest | lock／工具現況 | 官方最新穩定／建議 |
| --- | --- | --- | --- |
| [Rust／Cargo](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) | stable、edition 2024、resolver 3 | 本機無 rustc；工具鏈未安裝驗證 | 1.99.1，建議驗證使用此版本 |
| [Leptos](https://docs.rs/crate/leptos/latest) | 0.8 | 0.8.22 | 0.8.22，沿用 |
| [leptos_router](https://docs.rs/crate/leptos_router/latest) | 0.8 | 0.8.17 | 0.8.17，沿用 |
| [leptos_axum](https://docs.rs/crate/leptos_axum/latest)／[config](https://docs.rs/crate/leptos_config/latest) | 各 0.8 | 各 0.8.10 | 各 0.8.10，沿用 |
| [Axum](https://docs.rs/axum/latest/axum/) | 0.8 + ws | 0.8.9 | 0.8.9，沿用 |
| [SQLx](https://docs.rs/sqlx/latest/sqlx/) | 0.9 + runtime-tokio/sqlite/postgres/macros | 0.9.0 | 0.9.0，沿用 |
| [Tokio](https://docs.rs/crate/tokio/latest) | 1.54/full | 1.54.1 | 1.54.1，沿用 |
| [Serde](https://docs.rs/crate/serde/latest)／[JSON](https://docs.rs/crate/serde_json/latest) | 1.0 / 1.0 | 1.0.229 / 1.0.154 | 相同，沿用 |
| [UUID](https://docs.rs/crate/uuid/latest) | 1.28 + v4/serde/js | 1.28.0 | 1.28.0，沿用 |
| [Argon2](https://docs.rs/crate/argon2/latest)／[SHA2](https://docs.rs/crate/sha2/latest) | 0.6 / 0.11 | 0.6.0 / 0.11.1 | 相同，沿用；lock 另有傳遞 SHA2 0.10.9 |
| [Tower](https://docs.rs/crate/tower/latest) | dev 0.5/util | 0.5.3 | 0.5.3，沿用 |
| [SQLite](https://www.sqlite.org/)／[bindings](https://docs.rs/crate/libsqlite3-sys/latest) | SQLx 間接依賴 | libsqlite3-sys 0.37.0；執行期 sqlite_version 未執行 | engine 3.54.1；bindings 0.38.2 |

相容性：官方發佈 manifest 的 [Leptos MSRV](https://docs.rs/crate/leptos/latest/source/Cargo.toml) 為 1.88；[SQLx MSRV](https://docs.rs/crate/sqlx/latest/source/Cargo.toml) 為 1.94.0，故不可只依 Leptos 的較低門檻選 compiler。建議使用最新穩定 Rust 1.99.1，但完整 dependency graph 的最低工具鏈仍須實測。新頁純 SSR；原有 ui hydration 路由與 WASM 不增加依賴，仍應回歸編譯。

SQLite 最新穩定引擎優先作後續驗證目標，但目前 SQLx lock 使用舊 bindings；不能把綁定 crate 版本當 engine 版號，也不能只將 0.38.2 強塞入 lock（`links=sqlite3`、SQLx semver 與 native ABI 都須核對）。本文先提供相容既有鎖定圖的切片，SQLite 升級需要獨立驗證及使用者套用；本次不 cargo update、不安裝工具、不動 migration，不宣稱引擎已升級。

## ④ 驗證項目、實際結果與缺口

由使用者套用後，在該工作副本設定 `CARGO_TARGET_DIR` 指向全新外部暫存目錄，執行：

```sh
cargo test --locked -p ui --features ssr day31_account_tests
cargo check --locked -p server --features ssr
cargo test --locked -p server
cargo check --locked -p ui --features hydrate --target wasm32-unknown-unknown
```

| 操作／測試 | 預期 | 本次實際 |
| --- | --- | --- |
| 新 UI 契約測試 | 有 form、CSRF、名稱與 escaped feedback | 未執行；Red/Green 均未實跑 |
| 全新隔離 SQLite，依既有 migrations 啟動，登入後 GET /account | Leptos 表單，無額外 DB 寫入 | 未執行 |
| 原生 form 提交中文名稱、200 bytes 邊界 | 201、恰一列、可點進房，公開清單可找到 | 未執行 |
| 無／偽造／過期／撤銷 Session POST | 401、rooms 不增加 | 未執行 |
| 有效 Session 但錯誤 csrf 或跨 Origin | 403、不寫 DB | 未執行 |
| 空白、NUL、201 bytes 名稱 | 422，保留名稱與錯誤；UI maxlength 非 bytes 安全界線 | 未執行 |
| DB 寫入失敗、服務重啟讀回 | 失敗不回成功；已成功房間重啟保留 | 未執行 |
| 鍵盤與窄螢幕，登出、進房、原 Chat hydration | 操作可達，既有功能不退化 | 未執行 |

CI 只引用唯讀程式 SHA `1d1866dbbb30255fafa4c7f74b1868919662913c`：`Test All` run [39932989041](https://github.com/williechen/rust_ai_chat/actions/runs/39932989041)，2026-10-23T09:38:46Z completed/success；job `test-server` 120345579246、`test-ui` 120345579353 均 success。已讀 workflow，命令分別為 `cargo test --locked -p server`、`cargo test --locked -p ui`，不代表本文 SSR 表單、CSRF 或瀏覽器新流程已驗證。

文件發布、程式套用、功能驗收各自判定。只有全文讀回及條件同步成功才能推進文章 next_day；Gate 2 未完成，本文留 daily，不封存。`last_full_comparison` 保持原值。本次不修改 rust_ai_chat、不碰既有 DB、不讀正式秘密、不用付費 AI。下一個切片仍由最新來源與未完成事項選取，無固定預排。
