# Analytics Day33 r1｜Dashboard 數量標示區分「全部事件」與「精確篩選」

## ① 學習大綱：需求、來源與實際呼叫鏈

**本日最小可操作切片：** 管理員登入後在 Analytics Dashboard 選 Chat 來源，未設定事件類型時顯示「Chat 全部事件類型：12 筆已提交事件」；指定 `chat.message.persisted` 並篩選後，顯示「Chat 事件類型「chat.message.persisted」：3 筆已提交事件」。空結果顯示 0，文字不會將「已篩選的數量」誤寫成「事件總數」。切換 Pet 來源後必須顯示 Pet，不使用前一個來源的標籤。

唯一程式來源唯讀：`williechen/rust_ai_chat/main@f3f8255b0b701a1b57cb8c0a1737c3caa4319f2d`；文件正本：`williechen/copilot_use/main`；接續 Day32 已套用的「來源＋精確事件類型篩選」切片。本日**只改 Leptos SSR 呈現的結果語意**，不改 SQL、HTTP 契約、管理員 Session 或事件去重。

真實鏈：瀏覽器 GET `/dashboard?source=web_chat&event_type=chat.message.persisted` → Axum `apps/stats_server/src/password_login.rs::dashboard` → `session()`/首次改密碼檢查 → `valid_event_type` → SQLx PostgreSQL `pg_store::summary_for_source_and_type` → `summary_total` → `dashboard_ui::render_dashboard_filtered` → Leptos `DashboardView`。現有 `DashboardView` 已在篩選說明列印「精確事件類型」，但數量段落仍固定為 `{source_label} 已提交事件總數 {total}`，容易讓人把過濾後的 3 筆誤認為整體 12 筆。

## ② 實作：Red → 完整 Green → Refactor

### Red A：結果語意純函式契約

在 `apps/stats_server/src/dashboard_ui.rs` 末端新增以下測試。Green 前 `summary_count_label` 不存在，預期是 **missing function 編譯失敗**；不是本輪已實際執行的 Red。

```rust
#[cfg(test)]
mod day33_summary_caption_tests {
    #[test]
    fn count_label_distinguishes_full_and_exact_kind() {
        assert_eq!(
            super::summary_count_label("Chat", "", 12),
            "Chat 全部事件類型：12 筆已提交事件"
        );
        assert_eq!(
            super::summary_count_label("Chat", "chat.message.persisted", 3),
            "Chat 事件類型「chat.message.persisted」：3 筆已提交事件"
        );
        assert_eq!(
            super::summary_count_label("Pet", "pet.sleep", 0),
            "Pet 事件類型「pet.sleep」：0 筆已提交事件"
        );
    }

    #[test]
    fn real_leptos_ssr_escapes_filtered_type_in_count_label() {
        let html = super::render_dashboard_filtered(
            "Chat", 3, "web_chat", "<img src=x onerror=alert(1)>"
        );
        assert!(html.contains("3 筆已提交事件"));
        assert!(html.contains("class=\"summary-count\""));
        assert!(!html.contains("<img src=x onerror=alert(1)>"));
        assert!(html.contains("&lt;img"));
    }
}
```

**Red B（真實頁面，尚未執行）：** 使用隔離 PostgreSQL fixture 先寫入 Chat 兩種類型、Pet 一種類型，完成登入與首次改密碼，分別查全部／單一事件／零筆／切來源。現有頁面會在已篩選結果仍稱「已提交事件總數」，屬誤導。這是由來源程式推導，不能當成已執行的 DB 或瀏覽器失敗證據。

### Green A：新增單一結果語意函式

在 `apps/stats_server/src/dashboard_ui.rs` 的 `#[component] fn DashboardView` **之前**新增：

```rust
fn summary_count_label(source_label: &str, event_type: &str, total: i64) -> String {
    if event_type.is_empty() {
        format!("{source_label} 全部事件類型：{total} 筆已提交事件")
    } else {
        format!("{source_label} 事件類型「{event_type}」：{total} 筆已提交事件")
    }
}
```

此函式只產生文字，不手動拼 HTML、不解讀來源授權、不重算 SQL count。輸入 `event_type` 由既有 handler 的 `valid_event_type` 驗證；測試直接傳 HTML 字元以確認 Leptos SSR 會做跳脫。

### Green B：在現有 Leptos DashboardView 真正使用

在 `fn DashboardView` 中、`let filter_description = ...;` 區塊**結束之後、`view! {` 之前**新增：

```rust
let count_label = summary_count_label(source_label, &event_type, total);
```

在同一 `view!` 內，**只將原本**：

```rust
<p>{format!("{source_label} 已提交事件總數 {total}")}</p>
```

替換為：

```rust
<p class="summary-count" aria-live="polite">{count_label}</p>
```

保留原本 `<p>{filter_description}</p>`、`<p role="status">` 的空結果／查詢完成提示、`<form method="get" action="/dashboard">`、來源導覽、清除篩選、`render_dashboard_filtered` 和舊兩參數 `render_dashboard`。Leptos 自動跳脫 `event_type`，**不可**改成手寫 `Html(format!(...))`。沒有任何新 crate 或 migration，真實 Axum/SQLx 呼叫鏈保持不變。

### Refactor：顯示、計數與授權各司其職

`summary_count_label` 是 UI presentation function；`summary_total` 仍處理數值加總及 overflow；`pg_store::summary_for_source_and_type` 使用既有綁定參數；`session()` 與首次改密碼強制檢查仍在 Axum。測試區分「顯示標籤正確」與「DB 計數正確」，不得用 SSR 字串斷言取代 PostgreSQL 查詢驗證。

### 上一 Day 未完成與 FIFO

Day32 已有本輪 PG／HTTP／瀏覽器篩選證據，但本篇新結果標示尚未套用或驗證；管理員初始化、角色／來源 ACL、SSE 與明細頁仍有缺口。先前 Day 的未完成事項與 FIFO 依既有 manifest 保留，未重新計數、不回寫舊正文。

## ③ 套件與軟體版本（2026-10-24 官方核對）

| 項目 | rust_ai_chat manifest／lock 或實測 | 當日官方最新穩定 | 相容性及選版理由 |
| --- | --- | --- | --- |
| Rust | `rust-toolchain.toml: stable`；本輪未執行 `rustc -V` | **1.99.1**（10/15） | edition 2024；[官方公告](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) |
| Leptos | workspace `0.8`／lock **0.8.22** | **0.8.22**（10/05；0.9 RC 排除） | 沿用 SSR `view!`；[官方 release](https://github.com/leptos-rs/leptos/releases/tag/v0.8.22) |
| Axum | workspace `0.8`／lock **0.8.9** | **0.8.9**（04/14） | HTTP handler 不變；[官方 release](https://github.com/tokio-rs/axum/releases/tag/axum-v0.8.9) |
| SQLx | workspace `0.9`／lock **0.9.0** | **0.9.0**（05/06） | 重用既有 PG adapter，無 migration；[官方 changelog](https://github.com/transact-rs/sqlx/blob/main/CHANGELOG.md) |
| Tokio | workspace `1.54`／lock **1.54.1** | **1.54.1**（10/21） | 既有 async handler，不增 runtime；[官方 release](https://github.com/tokio-rs/tokio/releases/tag/tokio-1.54.1) |
| PostgreSQL | 本輪不啟動資料庫；上一輪報告為 18.6 | **18.6**（08/13；19 RC 排除） | 18.x 相容，無 schema 變更；[官方公告](https://www.postgresql.org/about/news/postgresql-186-1711-1615-1519-1424-and-19-beta-3-released-3365/) |

來源 `Cargo.lock` blob `85d844f618f0fd31a81775aa25f4c0af1a3754cd`。此篇沒有依賴升級或安裝；若正式環境 PG 版本不同，須先執行 `SHOW server_version;`，不能拿上輪 fixture 當本輪結果。Leptos SSR native 與 browser hydration 分開驗證。

## ④ 驗證命令、預期／實際、證據與缺口

| 類別 | 命令／操作 | 預期 | 本輪實際 |
| --- | --- | --- | --- |
| Red | `cargo test --locked -p stats-server day33_summary_caption -- --nocapture`（僅 Red） | 找不到 `summary_count_label`，編譯失敗 | 未執行 |
| Green | 同上（完整 Green 後） | 3 個語意樣本與 SSR escaping 通過 | 未執行 |
| SSR regression | `cargo test --locked -p stats-server` | 既有登入、改密碼、Dashboard 測試不退化 | 未執行 |
| PG integration | `DATABASE_URL=<isolated_pg_url> cargo test --locked -p stats-server --features postgres-integration` | 真 PG 各來源與事件類型計數正確 | 未執行 |
| HTTP/Browser | 登入→篩選→清除→切來源→零筆→登出 | 文字、數量、Session 與權限一致 | 未執行 |
| 安全 | 事件類型跳脫、未登入與首次改密碼限制 | 不輸出 HTML injection，不洩漏資料 | 未執行 |

**交付與驗收分離：** 僅文件教學；`rust_ai_chat` 未修改、未執行本篇測試，Gate 2 未通過；上一 Day 的程式驗證不能冒充本篇結果。