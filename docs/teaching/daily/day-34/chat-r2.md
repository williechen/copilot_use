# Chat Day34 r2｜重用 SignInPage，合併重複登入入口

## ① 學習大綱：重複來源與修訂範圍

2026-10-25 使用者指出登入頁多處重複。發布前來源已更新，已重新核對 `williechen/rust_ai_ring/main@d241cb04765002df797c614049149c3a0a3f7630` 與文件基準 `williechen/copilot_use/main@66cf6fe0f5f3331502a1461572822d2fe92adeba`，確認 r1 漏讀了既有 `SignInPage`。本篇取代 r1 的實作指引；r1 正文、commit/blob/SHA256 保留為歷史，不再建議套用。仍屬 Day34 的修訂，下一篇維持 Day35。

| 位置 | 現況 | 修訂處理 |
| --- | --- | --- |
| `chat_apps/chat_ui/src/lib.rs::SignInPage` | 已有完整 Leptos 表單、label/input ID、autocomplete、maxlength、註冊及回首頁連結；由 `/signin` 路由真正呼叫 | 保留作唯一登入頁面元件 |
| `chat_apps/chat_server/src/signin.rs::login_page` | 最新來源已套用 r1，GET `/login` 呼叫另一份 Leptos LoginView（更早版本曾是 HTML 字串） | 改成導向 `/signin`，移除這份表單 |
| r1 提議的 `chat_ui/src/login.rs::LoginView` | 已在最新來源新增並由 GET `/login` 呼叫，與 SignInPage 重複 | 撤回這個新增方向；按下方銜接移除，保留 SignInPage |

教學歷史：登入頁入口於 Day21 建立；帳密 Session 於 Day23 接通；Day28 已把既有 `SignInPage` 接至 POST `/login`，現行來源保留其測試 `day28_signin_tests`。因此不能把「還有舊 HTML handler」誤判成「缺 Leptos 登入表單」，更不應重教已接通的 UI。

唯一呼叫鏈：首頁／註冊頁 → GET `/signin` → Axum `ssr::app_handler` → Leptos `App/SignInPage` → POST `/login` → 既有 Argon2／SQLite mbr_user、mbr_session → 303 `/account`。相容舊網址 GET `/login` 僅導向 `/signin`。GET 頁面與 POST 認證可使用不同路徑，不需為名稱一致重做登入 API。

Application／Domain、SQLx Adapter 及原認證契約沿用；本日不新增平行認證表、共用認證骨架或 UI crate。程式來源全庫唯讀，由使用者套用文件。

## ② Red → 完整最小 Green → Refactor

### Red：舊登入網址不再渲染第二份表單

在 `chat_apps/chat_server/src/signin.rs` 保留原 day23_tests，**以下列完整模組取代已存在的 `day34_login_view_tests`**，不要同時保留要求 GET `/login` 回傳表單的舊測試。

```rust
#[cfg(test)]
mod day34_login_redirect_tests {
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn legacy_login_get_redirects_without_rendering_another_form() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_lazy("sqlite::memory:")
            .unwrap();
        let response = super::routes(pool)
            .oneshot(Request::get("/login").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(response.headers()["location"], "/signin");
        let body = to_bytes(response.into_body(), 16384).await.unwrap();
        assert!(!std::str::from_utf8(&body).unwrap().contains("<form"));
    }
}
```

目前來源已套 r1，會回傳 200 與表單，預期此測試失敗於 303 斷言；本次未執行，不捏造 Red 結果。lazy pool 確認這條 GET 路由不需要先連 DB。

在 `chat_apps/chat_ui/src/lib.rs` 末端加入下列模組。它直接渲染已存在的 SignInPage，固定「單一表單＋既有認證契約」，不是新增另一個元件；在原來源預期已通過，屬回歸保護而非假造新功能 Red。

```rust
#[cfg(all(test, feature = "ssr"))]
mod day34_existing_signin_tests {
    use super::*;

    #[test]
    fn existing_signin_is_the_single_login_form() {
        let html = view! { <SignInPage/> }.to_html();
        assert_eq!(html.matches("<form").count(), 1);
        for marker in [
            "method=\"post\"", "action=\"/login\"",
            "for=\"login-username\"", "id=\"login-username\"",
            "name=\"username\"", "for=\"login-password\"",
            "id=\"login-password\"", "name=\"password\"",
            "type=\"password\"", "autocomplete=\"current-password\"",
            "href=\"/signup\"",
        ] {
            assert!(html.contains(marker), "missing {marker}");
        }
    }
}
```

### Green：只更換舊 GET handler，不新增登入元件

在 `chat_apps/chat_server/src/signin.rs` 原 imports：

```rust
response::{Html, IntoResponse, Response},
```

完整替換為：

```rust
response::{Html, IntoResponse, Redirect, Response},
```

Html 仍由 account 等既有處理函式使用，不刪除此 import。完整取代現有 `async fn login_page() -> Html<String>`（目前呼叫 chat_ui::login::render_login）為：

```rust
async fn login_page() -> Redirect {
    Redirect::to("/signin")
}
```

保留既有 `signin::routes`，其完整內容仍為：

```rust
pub fn routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/login", get(login_page).post(login))
        .route("/account", get(account))
        .route("/account/rooms", axum::routing::post(account_create_room))
        .layer(DefaultBodyLimit::max(4096))
        .with_state(pool)
}
```

`POST /login` 的 Credentials、Origin、帳號查詢、Argon2、Cookie、Session INSERT 及成功導向完全沿用原函式，不將 POST 重新導向 `/signin`。原有 day23_password_login 測試須繼續通過。

真正頁面路由已存在於 `chat_apps/chat_server/src/lib.rs` 的 `.route("/signin", get(ssr::app_handler))`，Leptos `App` 已有 `<Route path=path!("/signin") view=SignInPage />`。兩者保留，不再 merge 第二組 `/signin` handler。首頁、註冊頁已指向 `/signin`；既有 SignInPage 表單只 POST `/login`，因此不會產生重新導向迴圈。

### 必要檔案移除與呼叫者銜接

最新來源已套用 r1，因此本次完整 Green 還必須移除 r1 新增的 `chat_apps/chat_ui/src/login.rs` 與根模組中的 `pub mod login;`，將 server 的 render_login 呼叫改為上述 Redirect，並替換 r1 測試模組。移除前先核對該檔是否含使用者額外修改；有額外修改時保留必要差異，再清除已無呼叫者的 LoginView。不要刪除 `SignInPage`、`account.rs` 或其他原有產品功能。這些是教學套用步驟，本次未操作產品檔案。

### Cargo、metadata 與 Refactor

無新增 dependency、feature、Cargo.lock 或 Leptos metadata；chat-server 已依賴 Axum 0.8，Redirect 使用既有 crate。chat-ui 的 ssr/hydrate、package `chat-ui`、server 的 ssr feature，以及 workspace 的 bin-package `chat-server`、lib-package `chat-ui`、輸出名稱 `rust_ai_chat` 保留。Rust imports 使用底線，命令 package 使用連字號。測試依賴原有 tower/util、SQLx、Tokio，不新建無呼叫者檔案。

Refactor 的結果是刪除第二份頁面內容，重用既有元件與正式呼叫鏈。未修改的認證邏輯不再貼成另一套 Green。

### 上一 Day 與舊 Day 更正補充

Day33 房間搜尋結果狀態已在來源；原驗收缺口保留。Day28 原文測試使用過時 `leptos::ssr::render_to_string` 的勘誤，現行來源已用 `view! { <SignInPage/> }.to_html()`；本篇測試亦沿此 API，不回寫原文章。Day34 r1 以新 LoginView 重複已存在功能的提議撤回，由此 r2 取代，原 commit `3353e7a67ec440489e11545f5ae64e0ab327fc95`、blob `b43a2718c7bdb7066c9c66f1e907786dc9634c48`、SHA256 `793dc5ac604b3db36f67d1646421760856d2b08735da4fc9308de8c238660691` 保留。Day25 共用認證 FIFO 仍未結案，進度不再遞增。

## ③ 套件與軟體版本

本修訂與 r1 同於 2026-10-25 交付，沿用本日已核對的官方版本資料；重新讀取四個 workspace manifests/lock。來源 workspace 版本已升為 0.2.34（crates workspace 仍依當次 manifest），Chat 直接依賴版本未變，本修訂沒有新增或升級依賴。官方來源、四份 lock blob／SHA256、相容性與環境限制見 [原版本核對](chat.md#③-套件與軟體版本) 與 [來源快照](source-snapshot.json)。r1 的版本資料保留，頁面重複的實作提議由本篇取代。

本切片直接涉及：Axum 宣告 0.8／lock 0.8.9、Leptos 0.8／0.8.22、Leptos Router 0.8／0.8.17、tower 0.5/util／0.5.3、SQLx 0.9／0.9.0、Tokio 1.54／1.54.1；當日核對最新穩定與 lock 一致。Rust/Cargo 建議 stable 1.99.1，安裝版未量測；不把 nightly/RC 當穩定版本。edition 2024 與 Tokio 1.54 至少需要 Rust 1.85，本修訂未做最低 MSRV 矩陣驗證。既有 SQLite／Argon2／Serde／UUID／SHA-2 沒有修改，資料庫 runtime 未量測，不將 CI 的編譯成功當成 GUI/DB 驗收。

## ④ 驗證命令、預期／實際與缺口

工作目錄從 `rust_ai_ring` 根目錄進入 chat_apps：

```bash
cd chat_apps
cargo test --locked -p chat-server --features ssr day34_login_redirect_tests
cargo test --locked -p chat-ui --features ssr day34_existing_signin_tests
cargo test --locked -p chat-ui --features ssr day28_signin_tests
cargo test --locked -p chat-server --features ssr day23_password_login
cargo test --locked -p chat-server --features ssr
cargo check --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown
cargo fmt --all -- --check
```

只套 Red：舊 GET `/login` 回 200，預期失敗。套 Green：GET `/login` 回 303 `/signin` 且本身不含登入表單；既有 SignInPage 只有一個表單，POST `/login` 的帳密／SQLite Session 回歸不變。

實機另測：首頁登入 → `/signin`；直接輸入 `/login` → `/signin` 且無迴圈；錯誤密碼不得建立 Session；成功登入 → `/account`；註冊回登入仍到相同頁；鍵盤、label 聚焦、中文輸入、SSR/hydration 與登出重登。原有錯誤回饋、選房登入導向、帳號等待、逐房 ACL／歷史及共用認證仍有缺口，本篇不宣稱完成。

本次只修訂教學文件，未套用產品，Rust／DB／HTTP／瀏覽器測試均未執行，Gate 2 未通過。[原 CI 補充](CI_SUPPLEMENT.md) 僅屬舊來源 54f5d80a，不轉移到此次 d241cb04；此次來源 CI 尚未取得可核實的 workflow/job 結果，標記未知，不拿舊 check/build 成功或失敗替代驗收。文件 commit、全文讀回、blob／SHA256 與入口同步另存交付證據；next_day 保持三案各35，last_full_comparison 不變。
