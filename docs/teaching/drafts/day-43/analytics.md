# Analytics Day43 候選｜切換 Chat／Pet 來源時保留事件類型精確篩選

> 2026-10-29；唯讀產品 [rust_ai_ring@c6708c4](https://github.com/williechen/rust_ai_ring/tree/c6708c42f21ac919e57a379848b4977d27a1c3d6)；文件起點 `copilot_use/main@d6a701e`；`next_day.analytics=42`。**新 Day43 候選草稿，非 Gate1 正式發布；產品未套用。**

## ① 目標／現況／真實鏈

Day42 已將 Leptos SSR Dashboard 事件類型明細連到 `/dashboard?source=...&event_type=...`，後端沿管理員 Session → 原 PostgreSQL 聚合查詢；但是頂部「Chat 事件／Pet 事件」導覽使用固定 URL，切來源時丟失使用者的精確 `event_type`。新增來源切換保留當前篩選（含中文、空白、`&`）；無篩選仍維持原 URL，清除篩選仍返回該來源全量。只改已授權後台 Leptos URL，不新增 SQL／API、繞過 Session 或改機器 ingest token；角色／來源 ACL 仍是原未完成條件。

## Refactor｜Analytics 計數標籤純決策搬層（獨立）

修改前 `analytics_server/src/dashboard_ui.rs` 同時負責 Leptos 元件與 `summary_count_label` 純文案決策；修改後新增 `dashboard_summary.rs` 保存原函式，`lib.rs` 宣告 `mod dashboard_summary;`，`dashboard_ui.rs` import `crate::dashboard_summary::summary_count_label`。原 Day33 `count_label_distinguishes_full_and_exact_kind` 與 SSR HTML escaping 斷言保留；新增零筆、中文類型及來源標籤測試。無跨案、shared crates 或全庫 CI 變更。

```rust
pub(crate) fn summary_count_label(source_label: &str, event_type: &str, total: i64) -> String {
    if event_type.is_empty() {
        format!("{source_label} 全部事件類型：{total} 筆已提交事件")
    } else {
        format!("{source_label} 事件類型「{event_type}」：{total} 筆已提交事件")
    }
}
```

完整新增 module、原 caller 與測試在 [Analytics Refactor patch](analytics-day43-refactor.patch)。下一步獨立盤點後台 Session／CSRF 測試與資料邊界，不將全庫契約計入本案。

## ② Red → 完整 Green（核心預估 15 分鐘）

**Red：** 新增 `dashboard_ui::day43_source_switch_tests`，以真 `render_dashboard_filtered("Chat", 1, "web_chat", "chat.中文 &")` 產生 Leptos SSR，要求兩個來源連結均包含 `event_type=chat.%E4%B8%AD%E6%96%87%20%26`，HTML attribute 以 `&amp;` escape，原始字串不得直接進入 URL；無篩選時保留舊連結。原 main 來源切換丟失參數，為可見行為 Red；**未實際執行 Red**。原 Day29/32/33/40/41/42 的 SSR、CSV、PG、授權測試不刪。

**Green：** 沿 Day42 的 `dashboard_url::encode_query_component`，不複製 encoder：

```rust
pub(crate) fn source_switch_url(source: &str, event_type: &str) -> String {
    if event_type.is_empty() {
        format!("/dashboard?source={source}")
    } else {
        format!("/dashboard?source={source}&event_type={}", encode_query_component(event_type))
    }
}
```

在 `DashboardView` 先建立 `chat_source_url = source_switch_url("web_chat", &event_type)`、`pet_source_url = source_switch_url("desktop_pet", &event_type)`，將原兩個靜態 href 改成 Leptos `href=chat_source_url`／`href=pet_source_url`。既有 `aria-current`、CSV、清除篩選、表格及 `render_dashboard_with_rows` 保留。完整 [Analytics Green patch](analytics-day43-green.patch) 含 URL helper、兩組 Red 測試與正式 UI caller；無 Cargo、lock、migration 或新 crate。切來源後資料仍由原授權查詢返回，不能由 URL 連結存在推定來源 ACL 已通過。

## ③ 版本（2026-10-29）

Rust stable [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)、Leptos [0.8.22](https://docs.rs/crate/leptos/latest)、Axum [0.8.9](https://docs.rs/crate/axum/latest)、SQLx [0.9.0](https://docs.rs/crate/sqlx/latest)、Tokio [1.54.1](https://docs.rs/crate/tokio/latest)、UUID [1.29.0](https://docs.rs/crate/uuid/latest)、Argon2 [0.6.0](https://docs.rs/crate/argon2/latest)。產品 workspace `0.2.42`，Analytics lock resolved 依 [Day42 最新版本表](../../daily/day-42/versions-2026-10-29.md)。實際 PostgreSQL／本機 rustc／遞迴 MSRV 尚未核對；不宣稱版本 Gate 或相容性全通過，不升級產品。

## ④ 驗證／套用／驗收／下一步

已做：讀回精確產品的 `dashboard_ui.rs`、`dashboard_url.rs`、`lib.rs`，確認現行 caller／SSR 斷言與 patch 修改片段唯一。**未做**：真 `git apply --check`、Red、Cargo／SSR、隔離 PostgreSQL、HTTP、瀏覽器與 CI。Gate1／Gate2 均未通過；不追認 Day42 舊 SHA 測試。隔離 checkout `c6708c42f21ac919e57a379848b4977d27a1c3d6` 後先 Refactor 再 Green：

```bash
git apply --check analytics-day43-refactor.patch && git apply analytics-day43-refactor.patch
git apply --check analytics-day43-green.patch && git apply analytics-day43-green.patch
cargo fmt --all --manifest-path analytics_apps/Cargo.toml -- --check
cargo test --locked --manifest-path analytics_apps/Cargo.toml -p analytics-server
cargo test --locked --manifest-path analytics_apps/Cargo.toml -p analytics-server -p analytics-persistence --features postgres-integration -- --include-ignored
```

人工驗收：管理員登入→選「chat.中文 &」→切 Pet／切回 Chat 均保留精確類型→清除篩選回全量→核對真 PG 總數、CSV 與匿名／無權 401/403。一般檢視者來源 ACL／SSE 仍待獨立驗收。核心 15 分鐘僅 Red/Green，Refactor、版本、驗證與提交另計。方案／套用／驗收分列；不增加 Day、FIFO、`last_full_comparison`。
