# Analytics Day 28｜把受 Session 保護的事件後台改為 Leptos SSR 元件

## ① 學習大綱與真實現況

程式唯讀來源：`williechen/rust_ai_chat/main@e62ed41e2122ef0979af8e2d453bafa2e05aa2e5`；教學正本：`williechen/copilot_use/main@0cf3c78b1a14dc627b7767254bd4dcae2ba0eaf8`；規則 `daily-tdd-minimum-v1 / github-only-v1`，規則 blob `5df88ba6892c377e46b6a76a934b9f59b51694e7`。

最新 `apps/stats_server/src/password_login.rs` 已有管理員初始化、PostgreSQL 加鹽 Argon2 帳密、首次強制改密碼、`uac_login_session`、`/dashboard?source=web_chat|desktop_pet` 與 `pg_store::summary_for_source`；Day27 Green 已在程式 main，不重教。**架構缺口**：`dashboard()` 仍以 Axum `Html(format!(...))` 建立整張 HTML；而 `apps/stats_server/Cargo.toml` 已宣告 `leptos = { workspace = true, features = ["ssr"] }`，尚未用於該頁。依既有規則，Analytics 正式 UI 應逐步採 Leptos，不再延伸手寫 HTML 字串頁。

本日一個最小切片：已登入管理員開 `/dashboard?source=desktop_pet` → Axum `session()` 驗證與首次改密限制 → 來源白名單 → SQLx PostgreSQL `summary_for_source` → Leptos SSR `DashboardView` 產生真正的來源切換與統計頁。**不改 SQL、不碰歷史 DB、不新增平行 API**。這是 server-rendered Leptos 的第一步，尚無 hydration、前端非同步 Resources 或角色／來源 ACL，不宣稱完整 Dashboard 驗收。

Analytics Red + Green 15 分鐘；Chat 15、Pet 15、共用彈性 15，共 60 分鐘。Refactor 5、PostgreSQL/SSR 測試與 UI 驗收 20–30 分鐘另計。

## ② 真 Red → 完整最小 Green → Refactor／FIFO

**Red：** 先在 `apps/stats_server/src/lib.rs` 末尾新增測試，原始來源尚無 `dashboard_ui`，因此預期**編譯失敗（缺模組）**；此處未實跑：

```rust
#[cfg(test)]
mod day28_dashboard_leptos_tests {
    #[test]
    fn day28_leptos_dashboard_renders_source_and_links() {
        let html = crate::dashboard_ui::render_dashboard("Pet", 2);
        assert!(html.contains("Pet 已提交事件總數 2"));
        assert!(html.contains("/dashboard?source=web_chat"));
        assert!(html.contains("/dashboard?source=desktop_pet"));
        assert!(html.contains("修改密碼"));
        assert!(!html.contains("test-pet"));
    }
}
```

**Green A：** 新增 `apps/stats_server/src/dashboard_ui.rs` **完整檔案**。使用 Leptos `#[component]` 與 `view!`，由框架產生 HTML；來源標籤是 server-side 白名單結果，不接受未驗證的使用者任意字串：

```rust
use leptos::prelude::*;

#[component]
fn DashboardView(source_label: &'static str, total: i64) -> impl IntoView {
    view! {
        <html lang="zh-Hant">
            <head>
                <meta charset="utf-8"/>
                <title>"Analytics 管理後台"</title>
            </head>
            <body>
                <main>
                    <h1>"Analytics 管理後台"</h1>
                    <nav aria-label="事件來源">
                        <a href="/dashboard?source=web_chat">"Chat 事件"</a>
                        <a href="/dashboard?source=desktop_pet">"Pet 事件"</a>
                    </nav>
                    <a href="/change-password">"修改密碼與 Email"</a>
                    <p>{format!("{source_label} 已提交事件總數 {total}")}</p>
                    <p>"角色與來源 ACL、互動篩選尚待接通。"</p>
                    <form method="post" action="/logout">
                        <button type="submit">"登出"</button>
                    </form>
                </main>
            </body>
        </html>
    }
}

pub fn render_dashboard(source_label: &'static str, total: i64) -> String {
    format!(
        "<!doctype html>{}",
        leptos::ssr::render_to_string(
            move || view! { <DashboardView source_label total /> }
        )
    )
}
```

在 `apps/stats_server/src/lib.rs` 的既有 `pub mod password_login;` 同層新增 `pub mod dashboard_ui;`；保留 `pg_store`、`bootstrap`、`admin` 與所有現有 HTTP routes。

**Green B：** `apps/stats_server/src/password_login.rs::dashboard` 保留整段 `session(&state,&headers)`、首次改密 Redirect、`source` 白名單、`pg_store::summary_for_source`、`crate::summary_total` 及所有錯誤分支，**只將最後原有 `renewed(Html(format!(...)), &raw)` 整段替換為**：

```rust
renewed(
    Html(crate::dashboard_ui::render_dashboard(source_label, total)),
    &raw,
)
```

這裡 Axum `Html` 僅負責 HTTP Content-Type，**HTML 本體由 Leptos SSR 元件渲染**；不是繼續用 `Html(format!(...))` 拼接畫面。原有 Day27 PostgreSQL 整合測試 `apps/stats_server/tests/day27_dashboard_source.rs` 仍可核對未登入 303、強制改密 303、登入後 Chat/Pet 真實聚合、未知來源 400；不要改動它或刪掉 server-side 授權。

**Refactor／舊 Day：** UI 元件只接收經 Axum Session/來源白名單及 PostgreSQL 查詢後的展示資料，不能拿 `PgPool`、Cookie 原文或服務 token；登入／改密頁仍是 Axum HTML，後續才逐步移至 Leptos。SSR 成功不等於 hydration/CSR 成功，角色與來源 ACL、CSRF/SSE 授權仍待完成。Analytics Day24 FIFO 更正已有讀回結案證據，本篇**不另補舊 Day、不回寫 Day25（含）以前封存檔**；也不操作使用者已選擇刪除重建政策下的歷史 PostgreSQL。

## ③ 套件版本

`rust_ai_chat@e62ed41e` 根 workspace `edition 2024`、`version 0.2.27`；`apps/stats_server/Cargo.toml` 已含 `leptos 0.8`（`ssr`）、`axum 0.8`、`sqlx 0.9`（PostgreSQL）、`argon2 0.6`、`tokio 1.54`、`serde 1.0`、`uuid 1.28`。無新增 crate、migration 或 `Cargo.lock` 修改。未核對 lock 全部 resolved versions、MSRV 與本機 stable rustc，不冒稱已建置。

## ④ 命令、預期／實際、CI/job/SHA、限制

| 命令／流程 | 預期 | 實際 |
| --- | --- | --- |
| `cargo test --locked -p stats-server day28_leptos_dashboard_renders_source_and_links` | Red 缺模組編譯失敗；Green 真實 Leptos SSR 標籤與連結 | **未執行** |
| `cargo check --locked -p stats-server` | Leptos SSR + Axum Handler 編譯 | **未執行** |
| `DATABASE_URL=<隔離測試 PostgreSQL> cargo test --locked -p stats-server --features postgres-integration -- --include-ignored` | 完整 PostgreSQL integration targets（包含 ignored） | **未執行／未建立測試 DB** |
| 瀏覽器：首次登入→強制改密→後台 Chat/Pet 切換→登出 | 真實 Session/PG、未授權不可見 | **未部署／未驗收** |
| `SELECT version,checksum,success FROM _sqlx_migrations ORDER BY version;` | 僅在已授權測試庫核對 migration | **未連線**，不操作歷史資料 |

精確來源 `rust_ai_chat/main@e62ed41e2122ef0979af8e2d453bafa2e05aa2e5`：[Rust Dashboard run 39869399796](https://github.com/williechen/rust_ai_chat/actions/runs/39869399796) `build-dashboard` job `120139709873` 與 `check-dashboard` job `120139710315` 已 success；[Test All run 39869399841](https://github.com/williechen/rust_ai_chat/actions/runs/39869399841) `test-stats-server` job `120139710475` 已 success。這些是**來源提交**的既有 CI，**不包含本篇未套用 Leptos Green**，也不證明完整 PostgreSQL integration／GUI／角色 ACL；`copilot_use` Actions 不作程式驗收。

本次僅交付教學，未寫入 `rust_ai_chat`、未執行測試、未操作 PostgreSQL、未部署／產品驗收。
