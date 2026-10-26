# Analytics Day40｜CSV 匯出遵循後台目前精確事件類型篩選

> 2026-10-26；產品來源 `williechen/rust_ai_ring/main@bb259aaf76542554a3a704d9c4f4a3a219d65cbf`，只向 `williechen/copilot_use/main` 交付文件與 patch。接續 [Day39 r2 人員 Session CSV](../day-39/analytics-r2.md)，不再建立平行 Dashboard／Query API。先套 Refactor，再套 Green；兩者獨立計功。

## ① 功能目標、現況與前置

管理員於既有 Leptos SSR Dashboard 選 Chat/Pet 來源並輸入精確事件類型，點「匯出目前篩選 CSV」時，只匯出該來源、該事件類型的已提交 PostgreSQL 聚合；清除篩選則匯出該來源全部事件類型。現行 Day39 CSV 只按來源，Dashboard 已有 `event_type` 篩選，但匯出連結未攜帶它；下載內容與目前畫面不一致。新 `GET /api/v1/events/{source}/export.csv?event_type=...` 必須先驗人員 Session，再驗來源與精確事件類型；機器 Bearer token 不能匯出。空結果仍提供只有 header 的 CSV；公式防護、BOM、CRLF、來源隔離沿用 Day39。

真呼叫鏈：`dashboard_ui::DashboardView` → percent-encoded link → `export::export_summary` → `password_login::require_query_session` → `analytics_persistence::summary_for_source[_and_type]` → PG → 原 CSV serializer。查詢綁定參數，不將字串插入 SQL。來源與角色 ACL 尚未完成，本篇不宣稱已接通角色管理。核心60分鐘僅② Red／Green。

## Refactor

**獨立既有架構工作：將 Analytics CSV 編碼責任由 HTTP handler 模組移入專用同案 `csv.rs`**，與 Day40 新篩選功能無依賴。Day39 的 `export.rs` 同時負責 session／PG／HTTP headers 與 CSV BOM、欄位跳脫、公式防護；保留同案 server delivery 邊界，抽出純序列化函式以支援獨立 fixture 測試，而非錯誤搬入 SQLx persistence。

完整變更：
- 新增 `analytics_apps/analytics_server/src/csv.rs`，**完整原封**搬移 `fn csv_cell`、`pub fn render_csv` 與既有 `csv_escapes_quotes_and_spreadsheet_formulas` 測試；新增 `use analytics_persistence::EventSummary;`。
- `analytics_server/src/lib.rs` 在 `pub mod dashboard_ui;` 後加入 `mod csv;`；沒有空 crate。
- `analytics_server/src/export.rs` 刪除舊 serializer 與其測試的原位置，移除 `EventSummary` import，新增 `pub use crate::csv::render_csv;`，保留原 handler 的 `render_csv(&rows)` 呼叫與公開相容路徑。Session、查詢、MIME、檔名及 cookie 續期行為完全不變。

完整 [Analytics Refactor patch](analytics-day40-refactor.patch) 包含新增檔全文、刪除舊實作與 `mod`／caller 接線。搬移前後以下原有斷言逐一保留，不能用新測試數量取代：BOM、CRLF、雙引號跳脫、`=1+1` 公式防護、中文與跨行資料。Day39 `day39_export_csv` 真 PG Router 401/403/200、來源隔離及登出後 401 仍必須執行；Day35 Persistence 來源聚合、Day37 Cookie、Day38 機器 Authorization 亦保留。**本次未跑 Cargo/PG**，此重構尚未經實測；下一個 Analytics Refactor 應處理 Session/查詢 caller 與 fixture 的獨立責任，不加入 Chat/Pet workflow。

## ② Red → 完整最小 Green

### Red｜真 Router／PostgreSQL 與 Leptos SSR 斷言

在既有 `analytics_server/tests/day39_export_csv.rs`，登入、完成首次改密碼並取得 `cookie` 後，登出前新增下列**完整**測試區塊。套 Green 前，`event_type` 被 Day39 匯出 handler 忽略，`chat.room.created` 仍會出現在精確 `chat.message.persisted` 的 CSV，故為**行為 Red**。同時在 Dashboard 模組新增 SSR URL encode 測試，新增符號前為**編譯 Red**；不能把環境無 PostgreSQL 當功能 Red。

```rust
    let filtered = get(
        app.clone(),
        "/api/v1/events/web_chat/export.csv?event_type=chat.message.persisted",
        Some(&cookie),
    ).await;
    assert_eq!(filtered.status(), StatusCode::OK);
    let body = body_text(filtered).await;
    assert!(body.contains("chat.message.persisted"));
    assert!(!body.contains("chat.room.created"));
    assert!(!body.contains("pet.sleep.started"));
    let no_kind = get(
        app.clone(),
        "/api/v1/events/web_chat/export.csv?event_type=missing.kind",
        Some(&cookie),
    ).await;
    assert_eq!(no_kind.status(), StatusCode::OK);
    assert!(!body_text(no_kind).await.contains("chat.message.persisted"));
    let invalid = get(
        app.clone(),
        "/api/v1/events/web_chat/export.csv?event_type=%20invalid",
        Some(&cookie),
    ).await;
    assert_eq!(invalid.status(), StatusCode::BAD_REQUEST);
    let page = get(
        app.clone(),
        "/dashboard?source=web_chat&event_type=chat.message.persisted",
        Some(&cookie),
    ).await;
    assert_eq!(page.status(), StatusCode::OK);
    assert!(body_text(page).await.contains(
        "/api/v1/events/web_chat/export.csv?event_type=chat.message.persisted"
    ));

```

```rust
#[cfg(test)]
mod day40_csv_filter_ui_tests {
    use super::*;
    #[test]
    fn dashboard_csv_link_encodes_exact_kind_and_escapes_markup() {
        assert_eq!(encode_query_component("a &中文"), "a%20%26%E4%B8%AD%E6%96%87");
        let html = render_dashboard_filtered("Chat", 1, "web_chat", "x&y");
        assert!(html.contains("/api/v1/events/web_chat/export.csv?event_type=x%26y"));
        assert!(!html.contains("event_type=x&y"));
        let empty = render_dashboard_filtered("Chat", 0, "web_chat", "");
        assert!(empty.contains("/api/v1/events/web_chat/export.csv"));
    }
}
```

### Green｜完整最小實作與正式 caller

完整 [Analytics Green patch](analytics-day40-green.patch) 修改 `export.rs`、`dashboard_ui.rs`、真 PG 整合測試，不新增 Cargo／migration／手寫 HTML 第二套前台。

1. `export.rs` 的 Axum extractor import 由 `extract::{Path, State}` 擴為 `extract::{Path, Query, State}`，Persistence import 改為 `use analytics_persistence::{summary_for_source, summary_for_source_and_type};`，加入完整 Query DTO：

```rust
#[derive(Default, serde::Deserialize)]
pub struct ExportQuery {
    #[serde(default)]
    event_type: String,
}

```

2. `export_summary` 新增 `Query(query): Query<ExportQuery>`（置於 Path 與 HeaderMap 間）；**完整替換**原本 `let rows = summary_for_source...` 的查詢單元：

```rust
if !query.event_type.is_empty() && !crate::valid_event_type(&query.event_type) {
    return Err(StatusCode::BAD_REQUEST);
}
let rows = if query.event_type.is_empty() {
    summary_for_source(&state.pool, &source).await
} else {
    summary_for_source_and_type(&state.pool, &source, &query.event_type).await
}
.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
```

前面的 `require_query_session`、來源白名單及後面的 `render_csv`／MIME／Content-Disposition／renewed 均保留；精確事件類型與 ingest 同一 128-byte／trim 契約，未知合法 kind 回空 CSV。不能因新查詢跳過人員 Session。

3. `dashboard_ui.rs` 保留 Leptos `DashboardView`、來源切換、篩選表單與 SSR escaping；在 `DashboardView` 前新增**完整**UTF-8 query component encoder：

```rust
fn encode_query_component(value: &str) -> String {
    let mut result = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            result.push(char::from(byte));
        } else {
            result.push_str(&format!("%{byte:02X}"));
        }
    }
    result
}
```

在 `let clear_url = format!("/dashboard?source={source}");` 後新增：

```rust
let export_url = if event_type.is_empty() {
    format!("/api/v1/events/{source}/export.csv")
} else {
    format!(
        "/api/v1/events/{source}/export.csv?event_type={}",
        encode_query_component(&event_type)
    )
};
```

將舊 `<a href=format!("/api/v1/events/{source}/export.csv")>"匯出目前來源 CSV"</a>` 替換為 `<a href=export_url>"匯出目前篩選 CSV"</a>`。UI 不持有 PgPool；URL percent-encode 保留 `&`、中文等精確字串，Leptos 照常 escape HTML 屬性。

### 斷言承接、驗證與下一步

```bash
# rust_ai_ring 根目錄；使用者自己的隔離工作樹
git apply --check /path/to/analytics-day40-refactor.patch
git apply /path/to/analytics-day40-refactor.patch
cd analytics_apps
cargo test --locked -p analytics-server --lib csv
cd ..
git apply --check /path/to/analytics-day40-green.patch
git apply /path/to/analytics-day40-green.patch
cd analytics_apps
cargo fmt --all -- --check
cargo test --locked -p analytics-server --lib day40_csv_filter_ui
cargo test --locked -p analytics-server --features postgres-integration --test day39_export_csv
cargo test --locked -p analytics-persistence --features postgres-integration
cargo test --locked -p analytics-server --features postgres-integration
cargo check --locked -p analytics-server
```

需專用有 CREATEDB 權限的 `DATABASE_URL`，不可用正式庫。瀏覽器驗收：管理員登入、完成首次改密碼，選來源＋kind → CSV 只有對應已提交事件；清除 kind → 全部；登出／停用 → 401；機器 token → 401；中文、`&`、雙引號與公式 payload 不破壞 CSV。舊事件／Session／PG assertions 不刪。回復時先逆套 Green，再逆套 Refactor。角色與來源 ACL、稽核、SSE、CSV 大量資料 streaming 仍是後續缺口。

## ③ 2026-10-26 官方穩定版核對

| 技術 | Manifest／Lock | 官方最新穩定 | 判斷 |
| --- | --- | --- | --- |
| Rust | edition 2024／本機未讀 | 1.99.1（10/15） | 真機核對 MSRV |
| Axum | 0.8／0.8.9 | 0.8.9（04/14） | Query extractor 沿用 |
| Leptos | 0.8／0.8.22 | 0.8.22（10/05） | SSR 元件不變 |
| SQLx | 0.9／0.9.0 | 0.9.0（05/21） | PG parameter binding |
| Tokio | 1.54／1.54.1 | 1.54.1（10/21） | 測試 runtime 不變 |
| UUID | 1.29／1.29.0 | 1.29.0（10/23） | PG fixture 仍使用 v4 |
| PostgreSQL | 測試 DB／runner 未查 | 18.6（08/13）；19 RC 非穩定 | 不宣稱已升級 DB |

官方：[Rust](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) · [Axum](https://docs.rs/crate/axum/latest) · [Leptos](https://docs.rs/crate/leptos/latest) · [SQLx](https://docs.rs/crate/sqlx/latest) · [Tokio](https://docs.rs/crate/tokio/latest) · [UUID](https://docs.rs/crate/uuid/latest) · [PostgreSQL](https://www.postgresql.org/docs/release/18.6/)。沒有新增依賴／feature／lock；本機工具與 PostgreSQL 實際版仍待驗。

## ④ 實際驗證／未完成

已核對來源檔案、正式 Router／Leptos caller、PG Adapter、既有測試與 lock；完整 Refactor、Red/Green 及測試方案已編入文章與 patch。**未實際 git apply、Cargo、PostgreSQL、瀏覽器或 GitHub Actions**；不能宣稱產品 Green、匯出正確或 Gate2。文件方案、產品套用、產品驗收分列，原 Day39 r2、有效斷言、FIFO、`last_full_comparison` 保留。跨案共用契約／crates／全庫 CI 只在 [全專案工程首頁](../../../engineering/README.md) 計功。
