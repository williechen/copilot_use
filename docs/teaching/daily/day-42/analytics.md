# Analytics Day42｜從真實事件類型明細一鍵套用精確篩選

> 2026-10-28；產品唯讀來源 `williechen/rust_ai_ring@0b2288e4996b6d80b00e7a77e2cd023835306528`；`next_day.analytics=42`。Day41 的真 PostgreSQL 事件類型表格已見於 `dashboard_ui::DashboardView`，本篇接續可操作的明細導覽，不重教表格。**方案已提交，產品尚未套用或驗收。**

## ① 目標／現況／正式呼叫鏈

管理後台目前可按來源顯示各 `event_type` 的真實已提交筆數，也可在表單手動輸入精確事件類型。使用者需要複製長字串才能查詢。Day42 讓表格每列事件類型成為可點擊連結，點選後透過既有 `/dashboard?source=...&event_type=...` 精確篩選；原「清除篩選」返回來源全量。不得把 HTML 中的事件類型直接拼成未編碼 URL；中文、空白、`&`、`<` 必須安全處理。

真實鏈：`analytics_server::dashboard_ui::DashboardView`（Leptos SSR）→ 原 `/dashboard` 路由 → `password_login` 有效管理員 Session／首次改密碼檢查 → `analytics_persistence` 真 PG 聚合 → Leptos 明細與計數。來源服務 ingest token 不能作為人員查詢 Session。本篇不建立平行 API／手寫 HTML Dashboard，不更改資料庫或角色 ACL。

## Refactor｜Analytics 自身查詢參數編碼責任

修改前 `analytics_server/src/dashboard_ui.rs` 同時持有 Leptos UI 與 `encode_query_component` 的 UTF-8 percent-encoding 純函式，CSV URL 直接呼叫它。修改後把原演算法完整移到新 `analytics_server/src/dashboard_url.rs`，在 `analytics_server/src/lib.rs` 宣告 `mod dashboard_url;`，`dashboard_ui.rs` 透過 `use crate::dashboard_url::encode_query_component;` 使用；保留原 CSV 呼叫與 Day40 測試，沒有引入另一套 URL encoder。

```rust
pub(crate) fn encode_query_component(value: &str) -> String {
    let mut result = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'`') {
            result.push(char::from(byte));
        } else {
            result.push_str(&format!("%{byte:02X}"));
        }
    }
    result
}
```

新增 `dashboard_url::tests::existing_csv_url_contract_keeps_utf8_and_reserved_characters` 驗證 ASCII 安全字元、空白、`&`、中文與 HTML 保留字；原 Day29/30/31/32/33/40/41 Leptos SSR、CSV、資料庫／授權斷言不刪。完整模組、caller 與測試在 [Analytics Refactor patch](analytics-day42-refactor.patch)。此項只屬 Analytics 專案，不改 shared crates、Chat/Pet 或全庫 workflow。下一個 Analytics Refactor 可獨立盤點 `password_login` 中的 Session/CSRF 與 Dashboard caller，先做測試映射再搬移。

## ② Red → 完整 Green（核心預估 15 分鐘）

### Red：真實 Leptos SSR 可點擊事件類型

在 `dashboard_ui.rs` 新增 `day42_kind_navigation_tests::kind_row_links_to_real_exact_filter_and_escapes_html`，用既有 `render_dashboard_with_rows` 提供真 `EventSummary`：`source_app=web_chat`、`event_type="chat.中文 &"`、`total=1`。預期 SSR HTML 包含指向 `event_type=chat.%E4%B8%AD%E6%96%87%20%26` 的可點擊連結，原始未編碼事件類型不得進入 URL；原來源表格只有純文字，這是可觀測行為 Red。**本次未實際執行 Red**。

```rust
#[test]
fn kind_row_links_to_real_exact_filter_and_escapes_html() {
    let html = render_dashboard_with_rows("Chat", 1, "web_chat", "", &[
        EventSummary { source_app: "web_chat".into(), event_type: "chat.中文 &".into(), total: 1 }
    ]);
    assert!(html.contains("event_type=chat.%E4%B8%AD%E6%96%87%20%26"));
    assert!(html.contains("href="));
    assert!(html.contains("chat.中文 &amp;"));
    assert!(!html.contains("event_type=chat.中文 &"));
}
```

### Green：使用既有來源與精確篩選路由

`analytics_apps/analytics_server/src/dashboard_ui.rs` 在 `DashboardView` 中複製 `source` 為 `filter_source`，不改既有表單／CSV 的所有權；原 `rows.into_iter().map` 改成下列完整單元。`event_type` 使用前述 percent encoder，`href` 與顯示文字由 Leptos view 做 HTML attribute／text escaping。

```rust
{rows.into_iter().map(|(kind, count)| {
    let detail_url = format!(
        "/dashboard?source={}&event_type={}",
        filter_source,
        encode_query_component(&kind)
    );
    view! { <tr><td><a href=detail_url>{kind}</a></td><td>{count}</td></tr> }
}).collect_view()}
```

[完整 Green patch](analytics-day42-green.patch) 包含 `filter_source` 宣告、表格修改與 SSR Red 測試；先套 Refactor，再套 Green。沿用 `analytics-server` 現有 `leptos`、`analytics-persistence`、`sqlx`、`serde`，無新 Cargo dependency、migration、lock、API 或 DB schema。原 CSV 匯出、來源切換、清除篩選、首次改密碼及 Session 授權不變。

## ③ 版本核對

2026-10-28 Rust 官方 stable **1.99.1**（2026-10-15，[官方 Rust release notes](https://doc.rust-lang.org/releases.html)）；workspace manifest 宣告 `leptos 0.8`、`axum 0.8`、`sqlx 0.9`、`tokio 1.54`、`uuid 1.29`，官方入口 [Leptos](https://docs.rs/crate/leptos/latest)、[Axum](https://docs.rs/crate/axum/latest)、[SQLx](https://docs.rs/crate/sqlx/latest)。精確 Analytics `Cargo.lock` resolved／本機 Rust、PostgreSQL 與 MSRV 相容核對**未完成**，不宣稱版本 Gate 全部通過；未修改產品 lock 或升級。

## ④ 實際驗證／未完成／下一步

**已做**：讀取最新產品 `dashboard_ui.rs`、`lib.rs`、workspace manifest，驗證原始替換位置唯一、生成兩份 unified patch，分別提交 GitHub 並以精確 commit 讀回。**未做**：`git apply --check`、Cargo、PostgreSQL、HTTP、Leptos SSR／瀏覽器及 CI；沒有真產品 Green 成功證據。

```bash
# 隔離副本 checkout 0b2288e4996b6d80b00e7a77e2cd023835306528
git apply --check analytics-day42-refactor.patch
git apply analytics-day42-refactor.patch
git apply --check analytics-day42-green.patch
git apply analytics-day42-green.patch
cargo fmt --all --manifest-path analytics_apps/Cargo.toml -- --check
cargo test --locked --manifest-path analytics_apps/Cargo.toml -p analytics-server
# 隔離 PG 並設定 DATABASE_URL 後：
cargo test --locked --manifest-path analytics_apps/Cargo.toml -p analytics-server --features postgres-integration -- --include-ignored
```

人工驗收：登入已完成改密碼的管理員，從 Chat 明細點中文／特殊字元事件類型→頁面只列精確事件→清除返回全部→切 Pet 來源；另測匿名 401／403、來源隔離、真 PG 總數與 CSV 一致。角色／來源 ACL、SSE 與一般帳號仍是後續需求，不能因本篇連結完成而視為已驗收。核心 Red／Green 預估 15 分鐘，Refactor、版本、驗證及提交另計；不改 FIFO、`last_full_comparison`。
