# Analytics Day30 r2｜帳密登入頁改用 Leptos SSR（含 Day29 未完成補教）

## ① 學習大綱與現況

文件正本 `williechen/copilot_use/main`；程式唯讀 `williechen/rust_ai_chat/main@9c2b7cf5a4c60a08254445911916c4ed9b02d7b3`；規則 `docs/COMMON_LEARNING_RULES.md`（daily-tdd-minimum-v1 / github-only-v1）。最新 `apps/stats_server/src/password_login.rs::login_page` 用 Axum `Html<&'static str>` 寫死登入表單，且 HTML 有重複的 `<!doctype html>`。既有 `POST /login` 已沿 Argon2 驗證 PostgreSQL `uac_user`，建立 `uac_login_session` 並設置 HttpOnly Cookie；改頁面時不能重建帳號與 Session。既有 `apps/stats_server/src/dashboard_ui.rs` 已有 Leptos SSR Dashboard，Day29 選中來源的 `aria-current` 已在當前程式中，不重教。

今天只完成可操作流程的前端入口切片：瀏覽器 `GET /login` → Axum route → Leptos SSR `LoginView` → 原生 HTML form `POST /login` → 現有驗證 Session → PostgreSQL；不改資料庫、不新增管理員或取消首次改密碼要求。Chat/Pet/Analytics 各 15 分鐘加共用 15 分鐘，合計 60 分鐘；額外 DB/瀏覽器驗收另計。

## ② 真 Red → 完整最小 Green → Refactor／舊 Day 補教

**Red（先新增失敗測試，未執行）**：在 `apps/stats_server/src/dashboard_ui.rs` 的既有測試 module 外新增：

```rust
#[cfg(test)]
mod day30_tests {
    #[test]
    fn login_page_is_server_rendered_with_accessible_fields() {
        let html = super::render_login();
        assert_eq!(html.matches("<!doctype html>").count(), 1);
        assert!(html.contains("action=\"/login\""));
        assert!(html.contains("name=\"username\""));
        assert!(html.contains("name=\"password\""));
        assert!(html.contains("autocomplete=\"current-password\""));
    }
}
```

Green 前因為 `render_login` 尚不存在，預期是編譯失敗（missing symbol），**不是本次已執行的測試失敗**。

**Green：在 `apps/stats_server/src/dashboard_ui.rs` 尾端加入完整可重用的 Leptos SSR 登入頁元件與 render function**：

```rust
#[component]
fn LoginView() -> impl IntoView {
    view! {
        <html lang="zh-Hant">
            <head>
                <meta charset="utf-8"/>
                <title>"Analytics 帳密登入"</title>
            </head>
            <body>
                <main>
                    <h1>"Analytics 帳密登入"</h1>
                    <form method="post" action="/login">
                        <div>
                            <label for="analytics-username">"帳號"</label>
                            <input id="analytics-username" name="username"
                                autocomplete="username" required/>
                        </div>
                        <div>
                            <label for="analytics-password">"密碼"</label>
                            <input id="analytics-password" name="password"
                                type="password" autocomplete="current-password" required/>
                        </div>
                        <button type="submit">"登入"</button>
                    </form>
                </main>
            </body>
        </html>
    }
}

pub fn render_login() -> String {
    format!("<!doctype html>{}", view! { <LoginView/> }.to_html())
}
```

在 `apps/stats_server/src/password_login.rs` **完整替換**既有 `login_page` 函式（保留函式以外所有認證、Session、變更密碼流程）：

```rust
async fn login_page() -> Html<String> {
    Html(crate::dashboard_ui::render_login())
}
```

本案 `apps/stats_server/src/lib.rs` 已宣告 `pub mod dashboard_ui;`；原 `password_login::build` 的 `.route("/login", get(login_page).post(login))` 保留不變。登入後的 Cookie、首次更改密碼、PostgreSQL SQL、Origin 與登出規則不變。**不要**以 Leptos SSR 元件中的假資料取代原 `POST /login` 行為。

**Refactor**：登入頁與 Dashboard SSR 都集中在 `dashboard_ui.rs`，Axum handler 僅組合 response；日後將改密碼頁逐步移往 Leptos，不能因此宣稱已完成 Router、hydrate 或全後台 UI。以上內容是原樣可套用的最小程式碼修改，但編譯和渲染 HTML 實測均未執行。

### Day29 未完成項目補充教學

原 Day29 `docs/teaching/history/day-29/analytics/github.md` 的來源篩選 `aria-current` 目前已套入 `dashboard_ui.rs`，但以下仍缺操作證據：`cargo test --locked -p stats-server active_source_is_marked_for_accessibility`、`cargo check --locked -p stats-server`、登入後 Chat/Pet 來源切換與 `aria-current` 互斥、PostgreSQL `summary_for_source` 數字、未知來源 HTTP 400、未登入轉向登入頁。需在隔離 PostgreSQL 的真實路由測試及 SSR 頁面讀回核對，不以字串測試當端對端結果。**本次都未執行**，Day29 未完成部分保留待驗收；不重發或改寫 Day29 原 commit，無證據不更動 makeup_queue。

### 新共同規則一致性補充（r2）

此篇僅修訂 Day30 的可套用教學，並非 Day31：文件 Gate 1 與程式 Gate 2 分開，僅寫入 copilot_use/main，rust_ai_chat/main 保持唯讀。原文的 Git commit、blob SHA、SHA256 繼續保留以供追溯。

**增量式完整呼叫鏈：** 登入的 UI 使用 Leptos SSR，Axum 僅包裝其 HTTP Response，沿用既有認證與 PostgreSQL；無須為單一切片重寫每一層。上一 Day29 未完成事項應放在本篇第②節補充，下一 Day 繼續追蹤仍未驗收項目；歷史 makeup_queue 保持 FIFO，不因文件交付而結案。

**時間與封存：** 每案 Red→Green→Refactor 核心 15 分鐘，加共用 15 分鐘共 60 分鐘。舊 Day 補教、來源／版本、文件交付與 Gate 2 實際驗證額外計時。只有本篇必要 Gate 2 驗收通過才能將正文原樣移至 docs/teaching/archived/day-30/analytics.md；本次未套用提案程式，未執行測試，仍不可封存。

週六完整比對目前不執行，不更新 last_full_comparison，且不代表雲端排程已實際停用。修訂不新增 Day，next_day 維持 31。

## ③ 套件版本

沿用 `apps/stats_server/Cargo.toml`：`leptos = { workspace = true, features = ["ssr"] }`、`axum.workspace = true`、`sqlx.workspace = true`，dev `tower = { version = "0.5", features = ["util"] }`。既有 `postgres-integration` feature 保留。本日不新增 crate 或 migration，不更動 Cargo.lock；精確 lock 版本、MSRV 與本地工具鏈未核實，不冒稱最新穩定版本相容性已驗證。

## ④ 命令、預期／實際與 CI 限制

| Gate / 命令 | 預期 | 本次實際 |
| --- | --- | --- |
| `cargo test --locked -p stats-server login_page_is_server_rendered_with_accessible_fields` | 一個 doctype、具備登入欄位與 action | 未執行 |
| `cargo check --locked -p stats-server` | Leptos SSR / Axum Handler 編譯通過 | 未執行 |
| `cargo test --locked -p stats-server --features postgres-integration -- --include-ignored` | 全部 PG integration target 可跑 | 未執行 |
| 未登入 `GET /login` | 真實 SSR 頁可見、表單可送往原 POST /login | 未執行 |
| 真實測試庫帳密登入、強制改密碼、Session 重驗 | 原 Argon2、Cookie、DB 與跳轉行為不退化 | 未執行 |
| Day29 來源切換與未知來源驗證 | Chat/Pet 互斥標記、未知 400、登入 Gate 不退化 | 未執行 |

來源精確 SHA `9c2b7cf5a4c60a08254445911916c4ed9b02d7b3`：`Rust Dashboard` workflow run 39891849841、`Persistence` 39891849758、`Test All` 39891849769 結論 success；`Node` 39891849761 failure。僅為**來源既有** workflow 結論；job 覆蓋未逐一核實，非本 Day Green/PG integration/產品驗收的證明。本次只交付 GitHub 文件，不連線或重建任何資料庫、不部署、不修改 `rust_ai_chat`，角色/來源 ACL、原管理員登入 HTML 以外的 Leptos 頁與完整測試仍未完成。
