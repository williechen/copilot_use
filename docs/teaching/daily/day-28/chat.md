# Chat Day 28｜以既有 Leptos 登入頁直接提交 SQLite 帳密登入

## ① 學習大綱與真實現況

- **程式唯讀來源**：`williechen/rust_ai_chat/main@e62ed41e2122ef0979af8e2d453bafa2e05aa2e5`；**教學規則正本**：`williechen/copilot_use/main@b7f7b4b646b18e09e9026ac5cea735e12425f968`，`docs/COMMON_LEARNING_RULES.md` blob `5df88ba6892c377e46b6a76a934b9f59b51694e7`；規則 `daily-tdd-minimum-v1 / github-only-v1`。兩庫不同用途，本文不修改 rust_ai_chat。
- **今天的一個最小使用者切片**：使用者進入 `/signin`（既有 Leptos SSR + hydration）→ 在該頁直接輸入帳號／密碼並提交表單 → 既有 Axum `POST /login` → SQLite `mbr_user` + Argon2 驗證 → SQLite `mbr_session` 寫入 → 303 `/account`。目前 `crates/ui/src/lib.rs::SignInPage` 只有指向另一張 Axum HTML `/login` 的連結，沒有可提交的 Leptos 登入表單；`apps/server/src/signin.rs::login` 已具備真實帳密、Session、Origin 驗證，不重做。
- **Day27 已套用部分**：`apps/server/src/auth_session.rs::active_member`、`room_auth::require_member` 和 `signin::account` 的 Session adapter 已存在於這次來源；不得再把 Day27 的「尚未套用」當作目前程式事實。`/account` 仍是 Axum HTML，`/api/rooms` 建房與 `/ws/{room_id}` 尚未完成會員／房間 ACL，今天不宣稱它們已完成。
- **實際呼叫鏈**：Leptos `App → SignInPage`（SSR `apps/server/src/ssr.rs`）→ 瀏覽器同源原生 POST → Axum `signin::routes → login` → SQLx SQLite `mbr_user/mbr_session` → HTTP 303/Set-Cookie → `/account`（`auth_session::active_member`）。沒有新資料表、沒有假 API、沒有付費 AI。
- **學習時間**：Chat 主體 Red + Green 15 分鐘；Pet 15、Analytics 15、共用彈性 15，合計 60 分鐘。本文 Refactor 5 分鐘、版本／CI 5 分鐘、瀏覽器及安全性驗證 10–20 分鐘另列，不計入主體 60 分鐘。

## ② 真 Red → 完整最小 Green → Refactor／FIFO

### Red：先驗證 Leptos 真正有可提交的登入表單

在 `crates/ui/src/lib.rs` 末尾**新增**下列測試模組；測試從實際 Leptos SSR 元件渲染 HTML，而不是檢查一個不會被路由呼叫的 helper。原始 `SignInPage` 沒有 `<form>`、`username`／`password` input，因此下列 assertion 預期**失敗**（不是預期編譯錯誤）。本次沒有實跑，Red 僅為依來源推導的預期。

```rust
// crates/ui/src/lib.rs：檔尾新增；先跑 Red，後貼 Green。
#[cfg(all(test, feature = "ssr"))]
mod day28_signin_tests {
    use super::*;

    #[test]
    fn day28_leptos_signin_posts_to_existing_login_handler() {
        let html = leptos::ssr::render_to_string(|| view! { <SignInPage /> });
        assert!(html.contains("<form"));
        assert!(html.contains("method=\"post\""));
        assert!(html.contains("action=\"/login\""));
        assert!(html.contains("name=\"username\""));
        assert!(html.contains("name=\"password\""));
        assert!(html.contains("type=\"password\""));
        assert!(html.contains("autocomplete=\"current-password\""));
        assert!(html.contains("/signup"));
    }
}
```

### Green：只替換既有 Leptos `SignInPage` 的完整函式

在 `crates/ui/src/lib.rs` 將原有 `#[component] fn SignInPage()` **整段**替換為以下版本；`App` 的 `Route path=path!("/signin") view=SignInPage`、`apps/server/src/lib.rs` 的 `GET /signin` SSR 路由、`signin.rs::login` 及 `signin.rs::routes` 全部保留。使用瀏覽器原生表單，故不需要在 WASM 放置帳密狀態或複製驗證規則。`name` 精確對應既有 `Credentials { username, password }`，`action` 精確對應既有 `POST /login`。

```rust
// crates/ui/src/lib.rs：完整取代原 SignInPage 函式。
#[component]
fn SignInPage() -> impl IntoView {
    view! {
        <section class="sign-in">
            <h2>"使用者登入"</h2>
            <form method="post" action="/login">
                <div>
                    <label for="login-username">"帳號"</label>
                    <input
                        id="login-username"
                        name="username"
                        type="text"
                        autocomplete="username"
                        required
                        maxlength="32"
                    />
                </div>
                <div>
                    <label for="login-password">"密碼"</label>
                    <input
                        id="login-password"
                        name="password"
                        type="password"
                        autocomplete="current-password"
                        required
                        maxlength="128"
                    />
                </div>
                <button type="submit">"登入"</button>
            </form>
            <p>
                <a href="/signup" rel="external">"建立會員"</a>
            </p>
            <p>
                <a href="/" rel="external">"回房間清單"</a>
            </p>
        </section>
    }
}
```

**同篇 Green 的完整接線**：`GET /signin` 已在 `apps/server/src/lib.rs` 指向 `ssr::app_handler`，SSR 渲染 `App/SignInPage`；既有 `POST /login` 由 `signin::routes` 合併至同一 Axum Router，並執行 `Credentials` 驗證、`Argon2` 比對、`mbr_session` 寫入、`Set-Cookie` 及 303。`GET /account` 透過 `auth_session::active_member` 讀取同一 SQLite Session。這些既有實作**不需要重新貼上或重建**，請勿建立第二組身分表或替換原本登入政策。原生 POST 依瀏覽器提供 Origin；後端目前僅允許 `http://127.0.0.1:3000`，其他 host 的正式設定尚未設計。

### Refactor 與必要的舊 Day 更正

- Leptos 專責輸入表單與導覽；Axum 專責 `POST /login`、Origin、Cookie／Session；SQLx 專責 SQLite 存取。`/login` 舊 HTML GET 暫留相容入口，不能因本篇就聲稱所有帳號頁已改成 Leptos。
- **Day27 架構偏差**：當時 `account` 以 Axum `Html(format!(...))` 產生整頁，且未覆蓋 `/api/rooms` 與 WebSocket ACL；這些是後續 Leptos 帳號頁與 server-side ACL 的待修，不回寫 Day27。
- **FIFO：`chat-day25-shared-auth-20261022`** 仍為 `pending`。本篇沿用 Day27 的 `active_member`，但 `signin.rs`、`logout.rs`、`signup.rs` 仍有不同的 Cookie/hash/Session 呼叫邊界；尚無完整共用登入／登出與政策驗證的實測證據，不能結案。Day25（含）以前文章／程式／證據不回寫。

## ③ Cargo、lock 與版本

核對 `rust_ai_chat@e62ed41e` 的根 `Cargo.toml`：workspace `edition = "2024"`、`version = "0.2.27"`；`leptos = "0.8"`、`leptos_router = "0.8"`、`leptos_axum = "0.8"`、`axum = "0.8"`、`sqlx = "0.9"`（SQLite）、`argon2 = "0.6"`、`tokio = "1.54"`。`crates/ui/Cargo.toml` 的 `ssr` feature 已存在，不新增套件、migration 或 `Cargo.lock` 變更。本次**未完整解析 lock 中每個精確版本、MSRV／本機 rustc stable**；上述是 manifest 約束，不冒充安裝版本或建置成功。

## ④ 命令、預期／實際、CI/job/SHA 與限制

| 命令或操作 | 預期 | 本次實際 |
| --- | --- | --- |
| `cargo test --locked -p ui --features ssr day28_leptos_signin_posts_to_existing_login_handler`（先 Red 再 Green） | Red 斷言失敗；貼 Green 後渲染真實 form 與欄位 | **未執行**，只靜態檢查 |
| `cargo check --locked -p server --features ssr` | Axum + Leptos SSR 能編譯 | **未執行** |
| `cargo check --locked -p ui --features hydrate --target wasm32-unknown-unknown` | hydration 仍可編譯 | **未執行** |
| 啟動獨立測試 SQLite，瀏覽器開 `/signin` → POST `/login` → `/account` | 帳密成功後 Set-Cookie/303，錯密拒絕，登出後帳號頁拒絕 | **未啟動／未驗收** |
| 匿名開 `/room/{id}`、`/api/rooms` POST、`/ws/{id}` | 房間頁受 Session 保護；API/WS 需另補 ACL | **未執行**，不得宣稱 ACL 已通過 |

來源 SHA **`e62ed41e2122ef0979af8e2d453bafa2e05aa2e5`** 的 GitHub Actions（2026-10-22 23:04 UTC 建立、此次讀取時）：
- `Rust AI Chat` [run 39869399923](https://github.com/williechen/rust_ai_chat/actions/runs/39869399923)，`build-chat` job `120139710339`、`check-chat` job `120139710686` 均 `in_progress`。
- `Test All` [run 39869399841](https://github.com/williechen/rust_ai_chat/actions/runs/39869399841)，`test-ui` job `120139710437`、`test-server` job `120139710446` 已 `success`；其他 job 未在此宣稱全數完成。
- `Node` [run 39869399825](https://github.com/williechen/rust_ai_chat/actions/runs/39869399825) `failure`。此為來源程式 CI，**不包含本文未套用的 Day28 Green**；`copilot_use` 的文件 commit 不是程式 CI。

**交付界線**：本篇是可供使用者套用的教學；未修改 `rust_ai_chat`、未執行 Red/Green、未操作 DB／部署、未驗收瀏覽器與 ACL。GitHub 文件 commit、全文讀回、狀態同步只證明教學交付。
