# Analytics Day32 r1｜管理後台真正依事件類型篩選已提交筆數

## ① 學習大綱：需求、資料流與來源

**本日可操作切片**：登入 Analytics 管理後台後，選 Chat／Pet 來源，輸入事件類型（例如 `chat.message.persisted`），按「篩選」，畫面只顯示該來源與**精確**事件類型的已提交數量；清除篩選後恢復該來源所有事件類型總數。空結果顯示 0，不把無資料當作 SQL 錯誤。未登入／首次改密碼限制仍由伺服器處理，查詢不接受任意來源、不拼接 SQL。

文件正本 `williechen/copilot_use/main`，程式來源唯讀 `williechen/rust_ai_chat/main@be0494a2a70be91eb74b2b726970a23fc3e5c044`；接續 `next_day.analytics=32`；規則 `daily-tdd-minimum-v1 + github-only-v1`。Day31 已把首次與一般修改密碼頁轉為 Leptos SSR，本日不重建登入。現行 `apps/stats_server/src/password_login.rs::dashboard` 只接受 `source`，一律呼叫 `pg_store::summary_for_source`；`dashboard_ui.rs::DashboardView` 只有來源導覽及總數，明寫「互動篩選尚待接通」。但 `pg_store::summary_for_source_and_type` 已有 SQLx PostgreSQL 參數化查詢、按 `source_app + event_type` 分組；不需要重建 Adapter、增加 migration 或另開平行 Query API。

真實鏈：瀏覽器 GET `/dashboard?source=web_chat&event_type=chat.message.persisted` → Axum `password_login::dashboard` → `session()` 重新核對 PostgreSQL 管理員與 Session／首次改密碼狀態 → `valid_event_type` → 既有 `pg_store::summary_for_source_and_type`（綁定參數）→ `summary_total` → Leptos SSR `DashboardView` → HTML 顯示數量、篩選條件與空狀態。事件攝取、去重與 commit 的既有程式不動；只有成功提交的 `product_events` 會被計數。**來源服務 token 不可當作人員後台 Cookie**，也不宣稱已有唯讀檢視者／逐來源 ACL。

## ② Red → 完整 Green → Refactor

### Red A：從 Leptos SSR 頁固定可用表單與跳脫

在 `apps/stats_server/src/dashboard_ui.rs` 末端新增此測試。Green 前 `render_dashboard_filtered` 尚不存在，預期是 missing-symbol **編譯失敗**；本次未執行，不能以來源舊測試成功當作 Red。

```rust
#[cfg(test)]
mod day32_filter_tests {
    #[test]
    fn dashboard_has_accessible_exact_kind_filter_and_escaped_value() {
        let html = super::render_dashboard_filtered(
            "Chat", 0, "web_chat", "chat.\" onfocus=\"bad"
        );
        assert_eq!(html.matches("<!doctype html>").count(), 1);
        assert!(html.contains("method=\"get\""));
        assert!(html.contains("action=\"/dashboard\""));
        assert!(html.contains("name=\"source\" value=\"web_chat\""));
        assert!(html.contains("for=\"event-type\""));
        assert!(html.contains("id=\"event-type\""));
        assert!(html.contains("name=\"event_type\""));
        assert!(html.contains("&quot;"));
        assert!(!html.contains("value=\"chat.\" onfocus="));
        assert!(html.contains("沒有符合條件的已提交事件"));
        assert!(html.contains("/dashboard?source=web_chat"));
    }
}
```

### Red B：正式 HTTP + PostgreSQL 行為

使用隔離 PostgreSQL fixture，登入有效管理員並完成首次改密碼；在 `product_events` 中先寫入同來源兩種 event_type，以及另一來源一筆。GET `/dashboard?source=web_chat&event_type=chat.message.persisted`：現有 handler **忽略 event_type**，預期顯示 Chat 所有類型的合計，而不是只算指定類型。測試還要覆蓋空類型、未知來源、未登入與必須改密碼狀態。這是由來源推導的預期失敗，**未實際發送 HTTP**。

### Green A：完整替換 Leptos Dashboard 區塊（保留舊呼叫者）

在 `apps/stats_server/src/dashboard_ui.rs` 中，**完整替換檔案開頭的 `DashboardView` 與 `render_dashboard` 兩個定義**（保留原本 `use leptos::prelude::*;`、下方 `LoginView`、`ChangePasswordView` 及所有測試）為：

```rust
#[component]
fn DashboardView(
    source_label: &'static str,
    total: i64,
    source: String,
    event_type: String,
) -> impl IntoView {
    let chat_current = (source_label == "Chat").then_some("page");
    let pet_current = (source_label == "Pet").then_some("page");
    let clear_url = format!("/dashboard?source={source}");
    let filter_description = if event_type.is_empty() {
        "全部事件類型".to_string()
    } else {
        format!("精確事件類型：{event_type}")
    };

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
                        <a href="/dashboard?source=web_chat" aria-current=chat_current>
                            "Chat 事件"
                        </a>
                        <a href="/dashboard?source=desktop_pet" aria-current=pet_current>
                            "Pet 事件"
                        </a>
                    </nav>
                    <a href="/change-password">"修改密碼與 Email"</a>
                    <form method="get" action="/dashboard">
                        <input type="hidden" name="source" value=source/>
                        <label for="event-type">"事件類型（精確比對）"</label>
                        <input
                            id="event-type"
                            name="event_type"
                            value=event_type
                            maxlength="128"
                            placeholder="例如 chat.message.persisted"
                        />
                        <button type="submit">"篩選"</button>
                        <a href=clear_url>"清除篩選"</a>
                    </form>
                    <p>{filter_description}</p>
                    <p>{format!("{source_label} 已提交事件總數 {total}")}</p>
                    <p role="status">
                        {if total == 0 {
                            "沒有符合條件的已提交事件"
                        } else {
                            "查詢完成"
                        }}
                    </p>
                    <p>"目前僅支援管理員；角色與來源 ACL 尚待接通。"</p>
                    <form method="post" action="/logout">
                        <button type="submit">"登出"</button>
                    </form>
                </main>
            </body>
        </html>
    }
}

pub fn render_dashboard_filtered(
    source_label: &'static str,
    total: i64,
    source: &str,
    event_type: &str,
) -> String {
    format!("<!doctype html>{}", view! {
        <DashboardView
            source_label
            total
            source=source.to_string()
            event_type=event_type.to_string()
        />
    }.to_html())
}

// 既有 Day28/29 測試與舊呼叫者仍可使用兩參數版本。
pub fn render_dashboard(source_label: &'static str, total: i64) -> String {
    let source = if source_label == "Pet" {
        "desktop_pet"
    } else {
        "web_chat"
    };
    render_dashboard_filtered(source_label, total, source, "")
}
```

`source` 僅由後端白名單決定；`event_type` 交給 Leptos text/attribute escaping，不能手寫 HTML `format!` 拼接使用者值。原 `render_dashboard("Pet", 2)` 呼叫保留，所以 Day28/29 的 SSR 回歸測試不用重寫。這是現有 Leptos SSR 而非新的靜態 HTML 或 WASM Dashboard。

### Green B：完整替換正式受保護 GET handler

在 `apps/stats_server/src/password_login.rs`，將 `DashboardSourceQuery` 與緊接的 `async fn dashboard` **整段替換**為下列完整程式。檔案原本已有 `Query`、`State`、`HeaderMap`、`StatusCode`、`Redirect`、`Html`、`pg_store` 等 imports；不更動 `build()` router、`session()`、`summary()`、`logout()` 或來源憑證入口：

```rust
#[derive(Default, Deserialize)]
struct DashboardSourceQuery {
    source: Option<String>,
    event_type: Option<String>,
}

async fn dashboard(
    State(state): State<IngestState>,
    headers: HeaderMap,
    Query(query): Query<DashboardSourceQuery>,
) -> Response {
    let (restricted, raw) = match session(&state, &headers).await {
        Ok(v) => v,
        Err(StatusCode::UNAUTHORIZED) => return Redirect::to("/login").into_response(),
        Err(e) => return e.into_response(),
    };
    if restricted {
        return renewed(Redirect::to("/change-password"), &raw);
    }

    let source = query.source.as_deref().unwrap_or("web_chat");
    let source_label = match source {
        "web_chat" => "Chat",
        "desktop_pet" => "Pet",
        _ => return StatusCode::BAD_REQUEST.into_response(),
    };
    let kind = query.event_type.as_deref().unwrap_or("");
    if !kind.is_empty() && (!crate::valid_event_type(kind) || kind.contains('\0')) {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let rows = if kind.is_empty() {
        pg_store::summary_for_source(&state.pool, source).await
    } else {
        pg_store::summary_for_source_and_type(&state.pool, source, kind).await
    };
    let rows = match rows {
        Ok(v) => v,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    let Some(total) = crate::summary_total(&rows) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    renewed(
        Html(crate::dashboard_ui::render_dashboard_filtered(
            source_label, total, source, kind,
        )),
        &raw,
    )
}
```

此處 `valid_event_type` 已存在，拒絕前後空白、空字串（空字串作「全部」分支）、超過 128 UTF-8 bytes；NUL 亦拒絕。`summary_for_source_and_type` 用 SQLx `.bind(source).bind(event_type)`，不以 URL 插值 SQL。Session 在讀取 query 結果前驗證，restricted 先導向改密碼；資料庫查詢失敗回 503 而非空資料。原 `renewed` 持續更新既有 Cookie。**未提供**逐來源檢視者授權；本日只維持現有管理員保護。

### Refactor：重用現有查詢 Port 邊界與 UI 契約

`pg_store` 是唯一 PostgreSQL 查詢 Adapter；`dashboard_ui` 不引入 PgPool、SQLx、服務 token 或新事件類型表；`password_login` 只負責認證、來源白名單、輸入驗證與 query dispatch。UI 的篩選值由 GET URL 可重現，切換 Chat／Pet 導覽會清除先前 event_type，避免舊來源條件誤用。現階段同步 SSR GET 沒有 client-side stale response 問題；未來改為 CSR/AJAX 時必須再測晚回應競態。後續補上來源 ACL、角色、明細表格與 SSE，不能以本篇「管理員單一數字」冒充完整 Analytics Dashboard。

### 上一 Day 未完成事項補充

Day31 改密碼 Leptos SSR 在最新來源中已存在，但首次初始化 `admin/admin` 與現有 bootstrap 密碼長度仍有差異；獨立 PostgreSQL 真登入／改密碼／Session 撤銷／帳號管理仍須在精確 SHA 驗證。此處只使用 Day31 既有登入主線，不修改歷史 Day31。

### 舊 Day 更正補充（FIFO）

`makeup_queue` 中 Analytics Day22／Day24 的教學更正已有完成紀錄，**本次沒有新的未結案 Analytics 舊 Day 項目可關閉**。本日不回填 Day25 以前歷史，不改舊文章正文，也不將已廢除的 copilot_use 程式工作重新列為補教。

## ③ 套件與軟體版本（2026-10-24 官方穩定版）

來源 `Cargo.toml` 宣告 Rust edition 2024、Leptos 0.8、Axum 0.8、SQLx 0.9、Tokio 1.54、serde 1.0、argon2 0.6；來源 `Cargo.lock@1b637de69d62d6413631b11b3e1af2bc85caa40d`。官方最新穩定版與 lock 不混淆：

| 元件 | 官方最新穩定 | 來源宣告／lock | 官方來源 |
| --- | --- | --- | --- |
| Rust／Cargo | **1.99.1**（2026-10-15） | toolchain=`stable`；實際安裝未核 | [Rust 官方發行](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) |
| Leptos SSR | **0.8.22** | `0.8`／0.8.22 | [Leptos crate](https://docs.rs/crate/leptos/latest)，不採 0.9 RC |
| Axum | **0.8.9** | `0.8`／0.8.9 | [Axum crate](https://docs.rs/crate/axum/latest) |
| SQLx | **0.9.0** | `0.9`／0.9.0 | [SQLx crate](https://docs.rs/crate/sqlx/latest) |
| Tokio | **1.54.1** | `1.54`／1.54.1 | [Tokio crate](https://docs.rs/crate/tokio/latest) |
| serde／serde_json | **1.0.229／1.0.154** | `1.0`／1.0.229、1.0.154 | [serde](https://docs.rs/crate/serde/latest)、[serde_json](https://docs.rs/crate/serde_json/latest) |
| Argon2 | **0.6.0** | `0.6`／0.6.0 | [Argon2 crate](https://docs.rs/crate/argon2/latest)；本篇不更動密碼 |
| PostgreSQL server | **18.6**（2026-08-13） | 實際 server 未核 | [PostgreSQL 官方公告](https://www.postgresql.org/about/news/postgresql-186-1711-1615-1519-1424-and-19-beta-3-released-3365/)；19 RC1 不是穩定版 |

**不新增依賴、不更動 lock**。PostgreSQL 版本需以測試 DB `SHOW server_version;` 實測；不能從 SQLx crate 推斷伺服器版本。保留現有 migration 與測試 fixture；不可因歷史 migration 差異自行修改 checksum 或刪除使用者 DB。

## ④ 驗證：命令、預期、實際、缺口

1. **Red**：在 Green 前加入 `day32_filter_tests`，執行 `cargo test --locked -p stats-server day32_filter_tests -- --nocapture`，預期找不到 `render_dashboard_filtered`；**實際未執行**。
2. **Green**：貼入 A/B 後，執行 `cargo test --locked -p stats-server day32_filter_tests -- --nocapture`、`cargo test --locked -p stats-server day29_tests`、`cargo test --locked -p stats-server day28_dashboard_leptos_tests`、`cargo clippy --locked -p stats-server --all-targets -- -D warnings`；預期 SSR 跳脫、舊簽名與所有 targets 回歸；**實際未執行**。
3. **PostgreSQL**：在**隔離測試庫**設 `DATABASE_URL`，執行 `cargo test --locked -p stats-server --features postgres-integration -- --include-ignored`（視 fixture 要求設定），確認事件去重、source/kind 聚合、rollback 不計數；**實際未執行**。
4. **真實瀏覽器**：管理員登入並完成首次改密碼；分別新增已提交 Chat `chat.message.persisted` 2 筆、`chat.room.created` 1 筆、Pet 同名 1 筆；開 `/dashboard?source=web_chat` 預期 3；加 `&event_type=chat.message.persisted` 預期 2；`&event_type=missing` 預期 0 與空狀態；切到 Pet 預期該來源的數量；未知來源／NUL／前後空白應 400；未登入轉 `/login`、restricted 轉 `/change-password`；SQL injection 字串只能當資料。**實際未執行**。
5. **CI 參考**：`rust_ai_chat@be0494a2a70be91eb74b2b726970a23fc3e5c044` 的 [Rust Dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39977648560) 與 [Persistence](https://github.com/williechen/rust_ai_chat/actions/runs/39977648818) 於 2026-10-23 結論 success；不包含本篇尚未套用的 Green，不能當本篇驗收。

**交付與驗收分離**：文章發布到 copilot_use 文件庫，不向 rust_ai_chat 寫入。Green 是可套用教學，**未套用／未執行 Red、Green、PostgreSQL、瀏覽器與正式功能驗收**。未完成：管理員初始化差異、登入失敗限制／共用驗證、角色與來源 ACL、明細、SSE、真正的 gRPC 來源認證與端到端資料可見性；不因 Day32 增加便標記產品功能完成。
