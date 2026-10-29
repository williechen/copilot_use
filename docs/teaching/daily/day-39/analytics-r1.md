# Analytics Day39 r1｜管理後台依 Session 匯出指定來源事件統計 CSV

> **r1 勘誤**：保留 [Day39 初版](analytics.md) 原文與 commit/blob；將 CSV 首字元防公式檢查從可能不符合 `Pattern` trait 的陣列參數改為明確 `chars().next().is_some_and(matches!)`，防護集合與功能範圍不變。僅修正 Green，原 Refactor patch 不變。

> 2026-10-26；產品唯一來源 `williechen/rust_ai_ring/main@8fc4b277b6f6f77d97655130036fca47813de56e`（唯讀），文件 `williechen/copilot_use/main`。前篇 [Day38 r4](../day-38/analytics-r4.md) 已完成重複 Authorization 拒絕及 PG 測試 support；本篇以最新來源核對。**方案交付不等於套用／驗收**。

## ① 使用者功能與邊界

管理者在 Leptos 後台選擇 Chat 或 Pet 來源後，點「匯出目前來源 CSV」，下載該來源所有已提交事件類型與筆數；其他來源不得混入。現行 `analytics_server::password_login::dashboard` 已依人員 Session 查詢 `summary_for_source`，`dashboard_ui::DashboardView` 已顯示來源選擇與篩選，但沒有下載入口。新路由 `GET /api/v1/events/{source}/export.csv` **先驗有效、已完成首次改密碼的人員 Session**，再核對來源白名單、查 PostgreSQL 聚合；機器 Bearer token 不可用於下載。CSV 附 BOM、固定檔名、CRLF、完整引號跳脫與公式注入防護。既有精確事件類型篩選不在本次 CSV 匯出範圍（明確標示「目前來源」而非「目前篩選」）。

核心60分鐘僅② Red／Green；Refactor、版本、來源、文件與測試驗收另計。Analytics Leptos SSR 正式頁維持，不建立手寫 HTML 第二套前台。

## Refactor｜Analytics 自身 PostgreSQL 聚合 Adapter 歸位（獨立於 CSV Green）

前篇已將三組正式 PG Router 測試的 HTTP fixture 收斂到 `tests/support`。本篇沿 Analytics 既有分層，將原 `analytics_server/src/pg_store.rs::summary_for_source` 的 **完整參數化 SQL** 移到已實作 `analytics_persistence/src/lib.rs`，與 `summary_for_source_and_type` 並列。正式 `password_login::dashboard` 改直接 import `analytics_persistence::{summary_for_source,summary_for_source_and_type}`；`pg_store` 保留 `pub use analytics_persistence::summary_for_source` 相容既有 `source_summary` 與 Day36 測試，不刪有效斷言。只動 Analytics 的 PG Adapter 與正式 caller，不碰 shared、其他 workspace 或全庫 workflow；也不是本篇 CSV 完成後的小整理。

**完整 Refactor patch**：[analytics-day39-refactor.patch](analytics-day39-refactor.patch)。新增 Adapter `#[sqlx::test]`：同來源兩事件類型計數／排序，其他來源不混入；原 Day35 exact-kind、rollback、Day36/37 query Router、Day38 ingestion 測試保留。沒有新 crate、feature、migration 或 lock；`analytics-persistence` 已在 server Cargo dependencies，`postgres-integration` feature 已串到 persistence。這是有真 SQL caller 的遷移，不將空 library 冒充接通。

```rust
pub async fn summary_for_source(pool: &sqlx::PgPool, source: &str)
    -> Result<Vec<EventSummary>, sqlx::Error> {
    sqlx::query_as::<_, EventSummary>(
        "SELECT source_app, event_type, COUNT(*)::bigint AS total
         FROM product_events WHERE source_app = $1
         GROUP BY source_app, event_type ORDER BY event_type"
    ).bind(source).fetch_all(pool).await
}
```

**回復與下一步**：使用者在乾淨分支先 `git apply --check`；如相容 API 或 query 斷言不符，停止並三方比對；完整套用後可在候選分支 `git apply -R`。新 Adapter 的 PG 隔離測試及舊 Router 真 Session／Cookie／事件來源測試須全跑；本次未執行。下一包沿 Analytics 自身 Session、角色／來源 ACL 與 Leptos UI 逐步整理，不重複計 G-01 全庫 CI。

## ② 功能實作：Red → 完整 Green

### Red｜未授權／受限／跨來源的正式 Router 測試

完整新增 `analytics_apps/analytics_server/tests/day39_export_csv.rs` 見 [Green patch](analytics-day39-green-r1.patch)（測試先單獨加入並執行為 Red）。它使用已存在的 `tests/support::app`、`#[sqlx::test(migrations="../migrations")]` 真 PostgreSQL，先建立 admin，寫入 Chat 兩種及 Pet 一種事件；測匿名／機器憑證 401、首次改密碼前 403、改密碼後下載 200、Content-Type/Disposition、Chat/Pet 來源隔離、未知來源 400、登出後 401，並確認 Dashboard 真正顯示匯出連結。未加新路由時預期 **404**（但本次未執行 Red），不是把舊通過測試充作 Red。

### Green｜完整可套用功能 patch

[analytics-day39-green-r1.patch](analytics-day39-green-r1.patch) 含：`analytics_server/src/export.rs`（**完整** CSV 序列化、Header 與授權 handler、單元測試）、`analytics_server/src/lib.rs`（模組）、`analytics_server/src/password_login.rs`（正式 Router 路由）、`analytics_server/src/dashboard_ui.rs`（正式 Leptos SSR 可點下載入口）、`analytics_server/tests/day39_export_csv.rs`（完整 PG Router 測試）。CSV 用 `EventSummary` 的現有 DTO，不重新寫聚合 SQL，不改人員 Session／機器攝取分工，不增加 Cargo dependencies／features／lock／migration。

```rust
pub async fn export_summary(State(state): State<IngestState>,
    Path(source): Path<String>, headers: HeaderMap) -> Result<Response, StatusCode> {
    let raw = crate::password_login::require_query_session(&state, &headers).await?;
    if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
        return Err(StatusCode::BAD_REQUEST);
    }
    let rows = crate::pg_store::summary_for_source(&state.pool, &source)
        .await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let mut response = render_csv(&rows).into_response();
    response.headers_mut().insert(header::CONTENT_TYPE,
        "text/csv; charset=utf-8".parse().unwrap());
    response.headers_mut().insert(header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"analytics-{source}-summary.csv\"").parse().unwrap());
    Ok(crate::password_login::renewed(response, &raw))
}
```

修訂後 `csv_cell` 以 `value.chars().next().is_some_and(|c| matches!(c, '=' | '+' | '-' | '@' | '\\t' | '\\r'))` 判斷首字元。CSV `csv_cell` 會對 `= + - @ TAB CR` 開頭的欄位加前導單引號，再雙引號包覆、內部 `"` 加倍；這是試算表匯入防護，須用 Excel/LibreOffice 真實匯入驗證。未提供逐事件明細、個資或敏感憑證；來源白名單也用於安全檔名。

```bash
# rust_ai_ring 根目錄；使用者自己的工作樹，先檢查再套用
 git apply --check /path/to/analytics-day39-refactor.patch
 git apply /path/to/analytics-day39-refactor.patch
 git apply --check /path/to/analytics-day39-green-r1.patch
 git apply /path/to/analytics-day39-green-r1.patch
 cd analytics_apps
 cargo test --locked -p analytics-server --lib export::tests
 cargo test --locked -p analytics-persistence --features postgres-integration
 cargo test --locked -p analytics-server --features postgres-integration --test day39_export_csv
 cargo test --locked -p analytics-server --features postgres-integration
 cargo check --locked -p analytics-server
```

### 斷言承接與驗收界線

原 Day23/24/27 的 admin、首次改密碼、Session、來源切換、CSRF、PG 資料計數斷言不刪；Day37 Cookie 多 Header、Day38 機器 Authorization 拒絕仍獨立。新 Day39 專驗「人員下載」而非重複機器上報。真瀏覽器：登入→首次改密碼→選 Chat/Pet→CSV 下載、UTF-8 中文／引號／試算表公式、來源隔離、登出後直接 URL 401；瀏覽器 cache 不應保存（既有全路由 no-store middleware）。角色／來源 ACL、SSE、下載稽核尚未接通，不冒稱完成完整管理權限。

## ③ 2026-10-26 官方穩定版本核對

| 依賴／工具 | Manifest／Lock | 官方穩定與日期 | 決策 |
| --- | --- | --- | --- |
| Rust | edition 2024；本機未查 | 1.99.1（10/15） | 使用者機器另驗 MSRV |
| Axum | 0.8／0.8.9 | 0.8.9（04/14） | 現有 Router／HeaderMap API |
| Leptos | 0.8／0.8.22 | 0.8.22（10/05） | 保留現有 SSR |
| SQLx | 0.9／0.9.0 | 0.9.0（05/21） | PG Adapter 不改 migration |
| Tokio | 1.54／1.54.1 | 1.54.1（10/21） | 保持現有 runtime |
| UUID | 1.29／1.29.0 | 1.29.0（10/23） | 測試 ID 不變 |
| PostgreSQL | 原CI版本須當次 runner 核對；本機未查 | 18.6（08/13；19 RC 非穩定） | 測試需真 PG；不假稱已升級 |

官方來源：[Rust](https://blog.rust-lang.org/releases/) · [Axum](https://docs.rs/crate/axum/latest) · [Leptos](https://docs.rs/crate/leptos/latest) · [SQLx](https://docs.rs/crate/sqlx/latest) · [Tokio](https://docs.rs/crate/tokio/latest) · [UUID](https://docs.rs/crate/uuid/latest) · [PostgreSQL](https://www.postgresql.org/docs/release/18.6/)。Analytics lock blob `d1bdb79c505f085872a2fa7f95bd3fcd0b67b23b`；四 workspace lock 已讀。

## ④ 實際驗證、Gate 與下一步

**實際已做**：精確來源／四 Cargo.lock／正式 UI→Router→Session→PG caller 靜態核對、官方版本頁查詢。**未執行**：Red 404、patch apply check、候選 Rust/PG 測試、瀏覽器 CSV 匯入、CI；產品未套用本篇。Gate1 以獨立 commit 全文讀回與同步證據為準；Gate2 由使用者套用後的產品 SHA 與真 PG／GUI／CI 驗證。保留原文、既有斷言及 FIFO，不將本次重構算新 Day。下一 Day 優先 Analytics 角色／來源授權與 Session 撤銷的最早完整可交付切片。
