# Analytics Day40 r1｜CSV 匯出遵循後台目前精確事件類型篩選

> 2026-10-26修訂。最新產品SHA `b0e06b73f5ab37b899c00b2ba08fd4da780e0dbf`；原教學與patch基底 `bb259aaf76542554a3a704d9c4f4a3a219d65cbf`。本r1更正套用問題並保存實際本地命令，原[Day40文章](analytics.md)與patch原bytes保留。Day41、FIFO與last_full_comparison不變。

> 發布前刷新：最新來源 `b0e06b73f5ab37b899c00b2ba08fd4da780e0dbf`。Pet已在此SHA重新跑原生check及69項Rust測試；Chat／Analytics／crates tree與前輪`ff14ca6`一致，沿用該SHA的實測且不冒稱重跑。[完整補正](local-validation/source-refresh.json)。

## 最新核對與閱讀方式

最新程式已接CSV event_type Query、URL encode與csv.rs；實作與PG測試、真瀏覽器／試算表驗收分列。

下面①／Refactor／②保留「從bb259aa重現原始缺口」的完整TDD教學；它們描述教學前狀態，不表示最新來源仍全部未實作。當次Red未重新執行，預期Red不能冒充實際失敗證據。實測結果與當前未完成項集中於④。[Day39後續勘誤](../day-39/analytics-r3.md)及[歷史更正總表](historical-corrections.md)保留各SHA範圍。

## ① bb259aa 教學基底的目標與缺口｜ 功能目標、現況與前置

管理員於既有 Leptos SSR Dashboard 選 Chat/Pet 來源並輸入精確事件類型，點「匯出目前篩選 CSV」時，只匯出該來源、該事件類型的已提交 PostgreSQL 聚合；清除篩選則匯出該來源全部事件類型。現行 Day39 CSV 只按來源，Dashboard 已有 `event_type` 篩選，但匯出連結未攜帶它；下載內容與目前畫面不一致。新 `GET /api/v1/events/{source}/export.csv?event_type=...` 必須先驗人員 Session，再驗來源與精確事件類型；機器 Bearer token 不能匯出。空結果仍提供只有 header 的 CSV；公式防護、BOM、CRLF、來源隔離沿用 Day39。

真呼叫鏈：`dashboard_ui::DashboardView` → percent-encoded link → `export::export_summary` → `password_login::require_query_session` → `analytics_persistence::summary_for_source[_and_type]` → PG → 原 CSV serializer。查詢綁定參數，不將字串插入 SQL。來源與角色 ACL 尚未完成，本篇不宣稱已接通角色管理。核心60分鐘僅② Red／Green。

## Refactor

**獨立既有架構工作：將 Analytics CSV 編碼責任由 HTTP handler 模組移入專用同案 `csv.rs`**，與 Day40 新篩選功能無依賴。Day39 的 `export.rs` 同時負責 session／PG／HTTP headers 與 CSV BOM、欄位跳脫、公式防護；保留同案 server delivery 邊界，抽出純序列化函式以支援獨立 fixture 測試，而非錯誤搬入 SQLx persistence。

完整變更：
- 新增 `analytics_apps/analytics_server/src/csv.rs`，**完整原封**搬移 `fn csv_cell`、`pub fn render_csv` 與既有 `csv_escapes_quotes_and_spreadsheet_formulas` 測試；新增 `use analytics_persistence::EventSummary;`。
- `analytics_server/src/lib.rs` 在 `pub mod dashboard_ui;` 後加入 `mod csv;`；沒有空 crate。
- `analytics_server/src/export.rs` 刪除舊 serializer 與其測試的原位置，移除 `EventSummary` import，新增 `pub use crate::csv::render_csv;`，保留原 handler 的 `render_csv(&rows)` 呼叫與公開相容路徑。Session、查詢、MIME、檔名及 cookie 續期行為完全不變。

完整 [Analytics Refactor patch](analytics-day40-refactor-r1.patch) 包含新增檔全文、刪除舊實作與 `mod`／caller 接線。搬移前後以下原有斷言逐一保留，不能用新測試數量取代：BOM、CRLF、雙引號跳脫、`=1+1` 公式防護、中文與跨行資料。Day39 `day39_export_csv` 真 PG Router 401/403/200、來源隔離及登出後 401 仍必須執行；Day35 Persistence 來源聚合、Day37 Cookie、Day38 機器 Authorization 亦保留。原版交付時未跑 Cargo/PG；本r1實測詳見④；下一個 Analytics Refactor 應處理 Session/查詢 caller 與 fixture 的獨立責任，不加入 Chat/Pet workflow。

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

完整 [Analytics Green patch](analytics-day40-green-r1.patch) 修改 `export.rs`、`dashboard_ui.rs`、真 PG 整合測試，不新增 Cargo／migration／手寫 HTML 第二套前台。

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
git apply --check /path/to/analytics-day40-refactor-r1.patch
git apply /path/to/analytics-day40-refactor-r1.patch
cd analytics_apps
cargo test --locked -p analytics-server --lib csv
cd ..
git apply --check /path/to/analytics-day40-green-r1.patch
git apply /path/to/analytics-day40-green-r1.patch
cd analytics_apps
cargo fmt --all -- --check
cargo test --locked -p analytics-server --lib day40_csv_filter_ui
cargo test --locked -p analytics-server --features postgres-integration --test day39_export_csv
cargo test --locked -p analytics-persistence --features postgres-integration
cargo test --locked -p analytics-server --features postgres-integration
cargo check --locked -p analytics-server
```

需專用有 CREATEDB 權限的 `DATABASE_URL`，不可用正式庫。瀏覽器驗收：管理員登入、完成首次改密碼，選來源＋kind → CSV 只有對應已提交事件；清除 kind → 全部；登出／停用 → 401；機器 token → 401；中文、`&`、雙引號與公式 payload 不破壞 CSV。舊事件／Session／PG assertions 不刪。回復時先逆套 Green，再逆套 Refactor。角色與來源 ACL、稽核、SSE、CSV 大量資料 streaming 仍是後續缺口。


### 完整現行CSV Refactor與handler

原始Refactor的export.rs也錯預期額外EOF空白行；修訂Refactor／Green已從精確bb基底按順序重新產生、標準檢查及實際套用。serializer原測試全文保留如下，沒有把HTTP責任搬進Persistence。

來源：[`analytics_apps/analytics_server/src/csv.rs`](https://github.com/williechen/rust_ai_ring/blob/ff14ca6224f7c5a65c799bf4a82a743fa79c880c/analytics_apps/analytics_server/src/csv.rs)。

```rust
use analytics_persistence::EventSummary;

fn csv_cell(value: &str) -> String {
    // Spreadsheet applications may interpret leading operators as formulas.
    let safe = if value
        .chars()
        .next()
        .is_some_and(|c| matches!(c, '=' | '+' | '-' | '@' | '\t' | '\r'))
    {
        format!("'{value}")
    } else {
        value.to_owned()
    };
    format!("\"{}\"", safe.replace('"', "\"\""))
}

pub fn render_csv(rows: &[EventSummary]) -> String {
    let mut csv = String::from("\u{feff}source_app,event_type,total\r\n");
    for row in rows {
        csv.push_str(&format!(
            "{},{},{}\r\n",
            csv_cell(&row.source_app),
            csv_cell(&row.event_type),
            row.total,
        ));
    }
    csv
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn csv_escapes_quotes_and_spreadsheet_formulas() {
        let csv = render_csv(&[
            EventSummary {
                source_app: "web_chat".into(),
                event_type: "=1+1".into(),
                total: 2,
            },
            EventSummary {
                source_app: "desktop_pet".into(),
                event_type: "say \"hi\",\nagain".into(),
                total: 1,
            },
        ]);
        assert!(csv.starts_with("\u{feff}source_app,event_type,total\r\n"));
        assert!(csv.contains("\"web_chat\",\"'=1+1\",2\r\n"));
        assert!(csv.contains("\"desktop_pet\",\"say \"\"hi\"\",\nagain\",1\r\n"));
    }
}
```

來源：[`analytics_apps/analytics_server/src/export.rs`](https://github.com/williechen/rust_ai_ring/blob/ff14ca6224f7c5a65c799bf4a82a743fa79c880c/analytics_apps/analytics_server/src/export.rs)。

```rust
use crate::IngestState;
pub use crate::csv::render_csv;
use analytics_persistence::{summary_for_source, summary_for_source_and_type};
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};

#[derive(Default, serde::Deserialize)]
pub struct ExportQuery {
    #[serde(default)]
    event_type: String,
}

/// Human Session is required; machine Bearer tokens never authorize CSV export.
pub async fn export_summary(
    State(state): State<IngestState>,
    Path(source): Path<String>,
    Query(query): Query<ExportQuery>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    let raw = crate::password_login::require_query_session(&state, &headers).await?;
    if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
        return Err(StatusCode::BAD_REQUEST);
    }
    if !query.event_type.is_empty() && !crate::valid_event_type(&query.event_type) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let rows = if query.event_type.is_empty() {
        summary_for_source(&state.pool, &source).await
    } else {
        summary_for_source_and_type(&state.pool, &source, &query.event_type).await
    }
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let mut response = render_csv(&rows).into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        "text/csv; charset=utf-8".parse().expect("static MIME"),
    );
    response.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"analytics-{source}-summary.csv\"")
            .parse()
            .expect("source allowlist"),
    );
    Ok(crate::password_login::renewed(response, &raw))
}
```

## ③ 當日版本與實機核對

Rust／Cargo實測1.99.1，WASM target已安裝；PostgreSQL實測18.6，GTK／WebKit依賴實測見[工具log](local-validation/commands.jsonl)。以下為2026-10-26重新核對的官方穩定頁與當次來源lock；不升級依賴或改Cargo.lock。

| 技術 | 官方核對 |
| --- | --- |
| Rust | [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) |
| Axum | [0.8.9](https://docs.rs/crate/axum/latest) |
| Leptos | [0.8.22](https://docs.rs/crate/leptos/latest) |
| SQLx | [0.9.0](https://docs.rs/crate/sqlx/latest) |
| Tokio | [1.54.1](https://docs.rs/crate/tokio/latest) |
| UUID | [1.29.0](https://docs.rs/crate/uuid/latest) |
| PostgreSQL | [18.6](https://www.postgresql.org/docs/release/18.6/) |

原版的「本機未讀」已由本次工具實測補正；未做release產物、完整MSRV矩陣或DB升級，不能由穩定版metadata推定它們通過。

## ④ 真正本地驗證與未完成

本次結果分為下列來源；**最新碼失敗與未接通項不由候選成功抵消**。

| 範圍／命令 | exit | 結果／log |
| --- | --- | --- |
| candidate-analytics-fmt：`cargo fmt --all -- --check` | 1 |  [log](local-validation/day40-logs/candidate-analytics-fmt.log) |
| candidate-analytics-check：`cargo check --offline --locked -p analytics-server` | 0 |  [log](local-validation/day40-logs/candidate-analytics-check.log) |
| candidate-analytics-pg：`cargo test --offline --locked -p analytics-server -p analytics-persistence --features postgres-integration` | 0 | 3passed/0failed/0ignored；34passed/0failed/1ignored；0passed/0failed/0ignored；0passed/0failed/0ignored；2passed/0failed/0ignored；2passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；2passed/0failed/0ignored；1passed/0failed/0ignored；2passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored [log](local-validation/day40-logs/candidate-analytics-pg.log) |
| candidate-analytics-ignored：`cargo test --offline --locked -p analytics-server --features postgres-integration --lib admin::tests::login_query_postgres_and_logout_flow -- --ignored --exact` | 0 | 1passed/0failed/0ignored [log](local-validation/day40-logs/candidate-analytics-ignored.log) |
| latest-analytics-fmt：`cargo fmt --all -- --check` | 0 |  [log](local-validation/day40-logs/latest-analytics-fmt.log) |
| latest-analytics-check：`cargo check --offline --locked -p analytics-server` | 0 |  [log](local-validation/day40-logs/latest-analytics-check.log) |
| latest-analytics-pg：`cargo test --offline --locked -p analytics-server -p analytics-persistence --features postgres-integration` | 0 | 3passed/0failed/0ignored；34passed/0failed/1ignored；0passed/0failed/0ignored；0passed/0failed/0ignored；2passed/0failed/0ignored；2passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；1passed/0failed/0ignored；2passed/0failed/0ignored；1passed/0failed/0ignored；2passed/0failed/0ignored；0passed/0failed/0ignored；0passed/0failed/0ignored [log](local-validation/day40-logs/latest-analytics-pg.log) |
| latest-analytics-ignored：`cargo test --offline --locked -p analytics-server --features postgres-integration --lib admin::tests::login_query_postgres_and_logout_flow -- --ignored --exact` | 0 | 1passed/0failed/0ignored [log](local-validation/day40-logs/latest-analytics-ignored.log) |

每條命令的精確cwd、時間、環境覆寫與完整原始錯誤見[JSONL](local-validation/commands.jsonl)；[兩組範圍及結果](local-validation/README.md)。Cargo使用--offline --locked，沒有改lock或升級依賴。

真PG／Router的精確kind匯出、來源隔離、Session／首次改密碼、escaping斷言實際執行；預設ignored已另補跑，未替代瀏覽器／試算表／角色ACL／舊DB升級。

GitHub文件提交與原始程式套用分開；本輪僅更新產品checkout至遠端SHA，驗證皆在隔離worktree，沒有提交／推送產品程式。原始文章與patch保留，靜態hunk檢查不追認為當時編譯成功。
