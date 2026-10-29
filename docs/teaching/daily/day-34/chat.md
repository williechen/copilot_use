# Chat Day34 r1｜帳密登入頁採 Leptos 並補齊可存取表單

## ① 學習大綱

交付日期：2026-10-25（Asia/Taipei）。文件基準 `williechen/copilot_use/main@fd426c68facdfdfd97616cf04a8bbc6badfe02cc`；唯一程式來源 `williechen/rust_ai_ring/main@54f5d80a5678e68e164d56e6d9f11c57cdff1f36`，全程唯讀，由使用者套用本文程式。舊名稱 rust_ai_chat 僅供歷史追溯；資源輸出名稱與外部契約依來源保留。

已重新讀取共同規則、SOURCE_LAYOUT、SYSTEM_ARCHITECTURE、TEACHING_ALIGNMENT、FUTURE_WORK、USER_FLOWS、AUTHENTICATION、UI_ALIGNMENT_REQUIREMENTS、learning-state、manifest、三案 ENTRY 及上一篇。即時 next_day 為 34；文章發布、程式套用、功能驗收分開記錄。

本日功能：匿名使用者開啟既有 `/login`，取得有明確 label/input 關聯、帳密自動填入提示及註冊入口的 Leptos SSR 表單。送出仍呼叫既有 POST `/login`，成功建立 SQLite Session 並前往個人管理頁。來源的 `signin.rs::login_page` 仍是手寫 HTML 字串且 label 無 for/id；不需要重建登入協定。

實際呼叫鏈：Leptos HomePage `/signin` → 既有 `/login` → Axum `signin::routes/login` → Argon2 驗密 → SQLx `mbr_user/mbr_session` → `/account` 的 `chat_ui::account`。Application/Domain 房間用例及 chat-persistence 重用；本切片不改房間規則，也不將 UI 直接接 DB。帳密驗證目前仍在 server，並未宣稱共用認證 application 已完成。

Day33 `room_list_feedback` 已存在於 `chat_apps/chat_ui/src/lib.rs`，不重貼。名稱／歷史來源銜接在本篇補充，上一篇無新 GUI 驗收證據。

## ② Red → 完整最小 Green → Refactor

### Red：先測真實 HTTP 頁面

將以下完整模組加到 `chat_apps/chat_server/src/signin.rs` 末端，保留既有 day23_tests。原始 GET 頁面缺少 for/id 與註冊入口，預期 marker 斷言失敗，並非缺符號編譯失敗。
```rust
#[cfg(test)]
mod day34_login_view_tests {
    use axum::{body::{Body, to_bytes}, http::{Request, StatusCode}};
    use tower::ServiceExt;

    #[tokio::test]
    async fn real_login_route_has_labelled_form_without_database_access() {
        // GET 頁面不應查帳號或建立 Session，lazy pool 不連線。
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_lazy("sqlite::memory:").unwrap();
        let response = super::routes(pool).oneshot(
            Request::get("/login").body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.headers()["content-type"].to_str().unwrap()
            .starts_with("text/html"));
        let body = to_bytes(response.into_body(), 16384).await.unwrap();
        let html = std::str::from_utf8(&body).unwrap();
        for marker in ["<!doctype html>", "action=\"/login\"",
            "method=\"post\"", "for=\"login-username\"",
            "id=\"login-username\"", "name=\"username\"",
            "for=\"login-password\"", "id=\"login-password\"",
            "type=\"password\"", "autocomplete=\"current-password\"",
            "href=\"/signup\""] {
            assert!(html.contains(marker), "missing {marker}: {html}");
        }
        assert!(!html.contains("value=\"admin\""));
    }
}
```

### Green：兩個必要檔案與一個模組宣告

新增完整 `chat_apps/chat_ui/src/login.rs`：
```rust
use leptos::prelude::*;

#[component]
fn LoginView() -> impl IntoView {
    view! {
        <html lang="zh-Hant">
            <head><meta charset="utf-8"/><title>"帳密登入"</title></head>
            <body><main>
                <h1>"帳密登入"</h1>
                <form method="post" action="/login">
                    <label for="login-username">"帳號"</label>
                    <input id="login-username" name="username"
                        autocomplete="username" required maxlength="32"/>
                    <label for="login-password">"密碼"</label>
                    <input id="login-password" name="password" type="password"
                        autocomplete="current-password" required maxlength="128"/>
                    <button type="submit">"登入"</button>
                </form>
                <a href="/signup">"註冊新帳號"</a>
                <a href="/">"回公開房間清單"</a>
            </main></body>
        </html>
    }
}

pub fn render_login() -> String {
    format!("<!doctype html>{}", view! { <LoginView/> }.to_html())
}
```

在 `chat_apps/chat_ui/src/lib.rs` 的 `pub mod account;` 後新增：
```rust
pub mod login;
```

在 `chat_apps/chat_server/src/signin.rs` **完整取代**從 `async fn login_page()` 至該函式結束的舊 HTML handler，其他 login/account/建房與測試原樣保留：
```rust
async fn login_page() -> Html<String> {
    Html(chat_ui::login::render_login())
}
```

Cargo 不增依賴：chat-server 已 path 依賴 `chat-ui = { path = "../chat_ui" }`；Leptos、Axum、SQLx、Tokio、tower/util 均已存在。server 的 `ssr = ["chat-ui/ssr"]`、ui 的 `ssr = ["leptos/ssr", "leptos_router/ssr"]` 保留。workspace Leptos metadata 仍為 bin-package `chat-server`、lib-package `chat-ui`、bin-features ssr、lib-features hydrate；輸出名稱 `rust_ai_chat` 不因 repo 更名改動。Rust import 用 chat_ui，檔案用 chat_ui，`-p` 用 chat-ui。沒有搬移或刪除舊產品檔案。

### Refactor

頁面責任移入既有 UI crate；Axum 只傳遞 `Html<String>` 回應、驗證與建立 Session。普通原生 POST 表單不需另寫 JavaScript 或新的 hydration route；避免把密碼回填 HTML。既有 Origin 檢查、加鹽雜湊、HttpOnly Cookie 與錯誤狀態碼保留；本篇未補登入錯誤畫面、帳號等待或逐房 ACL，不宣稱其已完成。

### 上一 Day 未完成事項及舊 Day 更正補充

Day33 功能已在來源，測試／瀏覽器仍須對現行 SHA 重跑。舊 `crates/ui`／`-p ui` 改用 `chat_apps/chat_ui`／`-p chat-ui`，且命令從 chat_apps 執行；源程式 stable toolchain 位於各 workspace，根目錄沒有 rust-toolchain.toml。上一篇 Router 最新版補核已完成，本日重新查到 0.8.17；不覆寫其原 commit/blob/SHA256。Day25 共用認證 FIFO 仍 pending；本文不以單頁遷移結案。

## ③ 套件與軟體版本

2026-10-25 重新核對官方發布頁／套件原始文件；最新穩定與 lock 分開，未以 nightly 的 docs.rs 建置工具鏈當 MSRV，也不把 RC 當正式版。本日保留來源 lock，不做無關升級。

| 項目 | 宣告／解析或環境 | 官方最新穩定與來源 |
| --- | --- | --- |
| Rust/Cargo | 各 workspace rust-toolchain.toml = stable；未量測安裝版 | [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) |
| Leptos | 0.8 / 0.8.22 | [0.8.22（排除 0.9 RC）](https://github.com/leptos-rs/leptos/releases) |
| SQLx | 0.9 / 0.9.0 | [0.9.0](https://github.com/transact-rs/sqlx/discussions/4271) |
| Tokio | 1.54 / 1.54.1 | [1.54.1](https://github.com/tokio-rs/tokio/releases) |
| Serde | 1.0 + derive / 1.0.229 | [1.0.229](https://docs.rs/crate/serde/latest) |
| UUID | 1.29 + serde,v4,js / 1.29.0 | [1.29.0](https://docs.rs/crate/uuid/latest) |
| Leptos Router | 0.8 / 0.8.17 | [0.8.17](https://docs.rs/crate/leptos_router/latest) |
| Axum | 0.8 + ws / 0.8.9 | [0.8.9](https://github.com/tokio-rs/axum/releases) |
| Argon2 | 0.6 / 0.6.0 | [0.6.0](https://docs.rs/crate/argon2/latest) |
| SHA-2 | 0.11 / 0.11.1 | [0.11.1](https://docs.rs/crate/sha2/latest) |
| tower（測試） | 0.5 + util / 0.5.3 | [0.5.3](https://docs.rs/crate/tower/latest) |
| SQLite | libsqlite3-sys 0.37.0；runtime 未量測 | [3.54.1](https://www.sqlite.org/changes.html) |

相容性：各 workspace edition 2024；Tokio 1.54 系列官方 MSRV 1.85，edition 2024 亦至少需要 1.85。本文建議已核對的 stable 1.99.1，未宣稱在每個依賴的最低 MSRV 測試。Leptos 0.8 API 沿用 `view!`／`IntoView`／`.to_html()`；native SSR 與 WASM 各自驗證。SQLx 0.9 既有 runtime-tokio/macros 保留；Chat/Pet 使用 sqlite，Analytics 使用 postgres。實際 SQLite 以 `SELECT sqlite_version()`、PostgreSQL 以 `SHOW server_version` 讀取，未在此環境測量 runtime。CLI 不增安裝步驟；使用隨 Rust toolchain 的 Cargo/rustfmt/clippy。沒有執行付費 AI 或讀取正式秘密。

crates.io API 的瀏覽讀取回報「not accessible via this tool」，已改用上述官方 release／套件文件；該錯誤不當成版本證據。其餘未變更依賴仍由四份原始 lock 固定；完整 manifests/lock blob 證據見本目錄 `source-snapshot.json`。

## ④ 驗證步驟與結果

以下命令從使用者的 `rust_ai_ring` 根目錄進入所屬 workspace，先只套 Red，再套 Green 後重跑。不得在 copilot_use 執行產品建置。

```bash
cd chat_apps
rustc -Vv
cargo -V
cargo test --locked -p chat-server --features ssr day34_login_view_tests
cargo test --locked -p chat-server --features ssr day23_password_login
cargo test --locked -p chat-ui --features ssr
cargo check --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown
cargo test --locked -p chat-server --features ssr
cargo fmt --all -- --check
```

Red／Green 的預期見第②節；本次只交付文件，**未套用產品、未執行上述 Rust 測試／DB／瀏覽器或 GUI**，沒有冒稱 Red 已失敗或 Green 已通過。文件的 JSON、相對連結、精確 commit 全文與雜湊另做交付驗證。

實機流程：鍵盤逐欄進入 /login→點 label 聚焦正確欄位→註冊入口→以測試帳號登入→/account→登出；確認密碼未回填、錯誤密碼不產生 Session、未登入仍可看公開房間清單。POST/SQLite 使用既有 day23 測試，不能只測 SSR marker 就宣稱全流程成功。

CI 來源限定 `54f5d80a5678e68e164d56e6d9f11c57cdff1f36`。[該 SHA Actions](https://github.com/williechen/rust_ai_ring/actions?query=branch%3Amain) 查詢時，connector 的 PR-only workflow_runs 回空，combined statuses 也回空；這不能證明 push workflows 不存在或全部成功。workflow/job、涵蓋範圍與測試結果本次**未取得可確認結果**，故 CI 為未知，不沿用舊 SHA 成功結論；待修項包含舊報告的 Clippy／migration checksum 及本次切片待執行驗證。不得清空既有資料庫或改舊 migration checksum 迴避問題。

尚待完成：選房登入導向保留、逐房 ACL、作者／歷史、首次管理員改密碼、共用認證 FIFO 與真瀏覽器驗收。本篇未結案 FIFO、未封存、未更動 last_full_comparison；每日局部核對不等於全面比對。文章 commit 全文讀回通過後才可同步 next_day 34→35；Gate 2 仍未通過。
