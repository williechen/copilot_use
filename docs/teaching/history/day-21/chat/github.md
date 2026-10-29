## 1. 學習大綱
**Day 21／P0：讓「登入」連結進入真實可載入的頁面；本日只接通路由與不冒充完成登入的 UI。** 這是帳號／Session 功能的第一個可操作入口，不代表可以登入。先完成入口，再逐日接入 SQLite 使用者、註冊與密碼驗證、Session、進房 ACL；仍保留匿名可查看公開房間清單。使用者曾指出「連登入都沒有怎麼進聊天室」，因此不得再優先處理歷史訊息。
來源 `rust_ai_chat/main=5654591f3002fc619c883af9b5197e2126cc6494`（唯讀），`copilot_use` 教學分支依實際最新 blob 更新；規則 daily-tdd-minimum-v1。現行 `crates/ui/src/lib.rs::HomePage` 有 `<a href="/signin" rel="external">`，但 `App` 只宣告 `/`、`/room/:room_id`；`apps/server/src/lib.rs::application` 僅配置根頁／房間 SSR，沒有 `/signin`。使用者從首頁按登入沒有有效登入頁。原始路由交付鏈：首頁連結 → Axum GET /signin → leptos_axum SSR app_handler → Leptos Router → SignInPage。必要前提：這一步**不接受密碼、不建立 Session**；顯示未接線提示，不應偽造成功。
## 2. 實作內容
### Red：先建立實際路由契約測試
在 `crates/ui/src/lib.rs` 的現有 `#[cfg(test)] mod day16_layout_tests` 中加入：
```rust
#[test]
fn day21_signin_page_renders_clear_unavailable_state() {
    let owner = leptos::prelude::Owner::new();
    owner.with(|| {
        let html = super::SignInPage().to_html();
        assert!(html.contains("使用者登入"));
        assert!(html.contains("尚未接通帳號驗證"));
    });
}
```
上面必須保有模組已有的 `use leptos::prelude::*;`；新增前 `SignInPage` 未定義，為預期的新符號 Red（E0425），**本輪未實測**，不可寫成已看到 E0425。另檢查 HTTP `/signin` 尚未路由；這是靜態證據而非已執行的 HTTP 測試。
### 完整最小 Green（兩個現有檔案）
一、`crates/ui/src/lib.rs` 中 `App` 的 `Routes` 內加入：
```rust
<Route path=path!("/signin") view=SignInPage />
```
同檔在 `HomePage` 前新增獨立的 SSR/CSR 共用 Leptos 元件，**刻意不偽裝可送出帳密**：
```rust
#[component]
fn SignInPage() -> impl IntoView {
    view! {
        <section class="sign-in">
            <h2>"使用者登入"</h2>
            <p role="status">"尚未接通帳號驗證；目前不可登入。"</p>
            <a href="/" rel="external">"返回公開房間清單"</a>
        </section>
    }
}
```
二、`apps/server/src/lib.rs::application` 原本的 Axum Router 新增（保持原有房間和 API 路由）：
```rust
.route("/signin", get(ssr::app_handler))
```
使用者點登入後確實能得到 SSR 頁面而非 404；不插入任何假 Session、不回傳認證成功；仍缺註冊表單／POST login／cookie／驗證。
### Refactor／明確續接
本次最小 Green 的「操作」只是開啟登入頁，**不是登入功能完成**。下一 Day 優先串真正使用者帳號的 SQLite schema 與持久化 Repository、密碼雜湊與註冊/登入介面，依依賴和已存在的 copilot_use 分支試作核對再選切片；避免製造第二套帳號模型。先把 `/signin` 根路由可達與 UI 錯誤狀態驗證，再做 POST；未經授權不得進受保護房間，公開列表例外。
## 3. 套件版本
`Cargo.toml` 工作區 `0.2.20`、edition 2024；`leptos=0.8`、`leptos_router=0.8`、`axum=0.8`。本切片不需新的 crate、資料庫 migration 或秘密設定。Cargo.lock 精確 patch 與本地 Rust toolchain 未另測。
## 4. 驗證項目
```bash
cargo test --locked -p ui --features ssr day21
cargo check --locked -p ui --features ssr
cargo check --locked -p server
# 實際執行應在本機啟動 server 後以 HTTP GET /signin 及瀏覽器驗證：
# 確認 200、頁面提示與返回公開列表，並確認不能用此頁面登入。
```
**預期：** UI Day21 測試通過、Axum GET /signin 進 Leptos SSR，不再 404，返回連結仍可開啟公開房間清單。**本輪實際：未修改 rust_ai_chat/main、未執行新測試／瀏覽器／HTTP；Red 和 Green 是可套用提案，不是已執行的結果。** 精確來源 SHA `5654591f3002fc619c883af9b5197e2126cc6494`；前次 `c92831e…` 的 CI 成功僅屬 Day20，不能替 Day21 提案作驗證。登入 POST、帳號 SQLite、Session、房間 ACL、產品整合、原生功能驗收全部未完成。
