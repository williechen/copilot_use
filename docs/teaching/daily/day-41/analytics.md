# Analytics Day41｜管理後台顯示真實事件類型與已提交筆數明細

> 2026-10-27；產品唯讀 SHA `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`，文件預期 parent `f1e84763868d1ac51ed56ea4bcb4236dd55eac68`。Analytics next_day=41；[上一篇 Day40 r1](../day-40/analytics-r1.md) 已完成 CSV 精確類型匯出，這次不重教。**方案已寫、尚未套用／驗收。**

## ① 功能目標／現況／前置

正式 Leptos SSR Dashboard 目前只顯示來源／精確 kind 的**總數**與 CSV 下載連結；管理者不能在頁面上看到各事件類型各幾筆，需下載 CSV 才知道。本篇新增「已提交事件類型明細」表格：來源切換時只顯示該來源已提交的各 kind 與 count；精確 kind 篩選時只顯示相符 kind；空結果有原本的明確空狀態，表格不杜撰資料。繼續使用已存在的 PostgreSQL `summary_for_source`／`summary_for_source_and_type`，不增加查詢或平行資料源。

正式鏈：人員 `password_login::session` → `dashboard` 驗 Session／首次改密碼 → PG adapter `EventSummary` → `summary_total` → `dashboard_ui::render_dashboard_with_rows` → Leptos `DashboardView` SSR table。未登入仍導登入；機器來源 token 不可讀 Dashboard；目前只有管理者，檢視者角色／來源 ACL、SSE 仍待後續。

## Refactor｜Analytics 自身 Cookie parser 分離（與今日表格功能獨立）

**修改前**：`analytics_apps/analytics_server/src/password_login.rs` 混合登入、密碼修改、Dashboard、Session 驗證與低階 Cookie 逐 header 解析。Cookie 解析與 PG／HTTP handler 無須耦合；若往後加檢視者，也應共用**同一 Analytics** Cookie 語義，而不是複製 parser 或共用機器 Bearer。

**完整修改**：把原 `fn raw_cookie(headers: &HeaderMap) -> Option<String>` **逐行原封**移至新增 `analytics_apps/analytics_server/src/session_cookie.rs`，只改可見度為 `pub(crate)`，保留多 header、精確 `analytics_session` 名稱、64 字元 hex、重複 cookie 拒絕及無效 UTF-8 的原邏輯。新檔完整 imports `use axum::http::{HeaderMap, header};`，原 `password_login.rs` 移除 parser 並新增 `use crate::session_cookie::raw_cookie;`；`analytics_server/src/lib.rs` 增 `mod session_cookie;`。原 `login/session/logout/change_password` caller、SQL、Argon2、fingerprint 與 Cookie 設定均不變。

```rust
// session_cookie.rs 原行為：有且僅有一個有效 analytics_session 才返回 Some。
pub(crate) fn raw_cookie(headers: &HeaderMap) -> Option<String> {
    let mut session = None;
    for value in headers.get_all(header::COOKIE).iter() {
        let line = value.to_str().ok()?;
        for part in line.split(';').map(str::trim) {
            if let Some((name, raw)) = part.split_once('=') {
                if name != "analytics_session" { continue; }
                if session.is_some() || raw.len() != 64
                    || !raw.bytes().all(|c| c.is_ascii_hexdigit()) { return None; }
                session = Some(raw.to_owned());
            } else if part == "analytics_session" { return None; }
        }
    }
    session
}
```

**測試／斷言承接**：新檔 `parser_preserves_exact_name_and_rejects_duplicate_or_malformed_cookie` 覆蓋缺失、非法、合法、多 header 重複、錯誤 Cookie 名稱；原 Day23–40 人員登入、首次改密碼、停用／撤銷、PG Router 與 Cookie 續期測試全部保留，不刪任何有效斷言。[完整 Analytics Refactor patch](analytics-day41-refactor.patch) 包含新檔全文、原檔移除及 module/import 接線。下一個 Analytics Refactor 針對同案 PG fixture／授權 caller，與 Chat/Pet／全庫 CI 無關。

## ② Red → 完整最小 Green

### Red｜SSR 與真 PG Router 雙邊界

在 `analytics_server/src/dashboard_ui.rs` 新增 `day41_event_table_tests::committed_kind_rows_are_rendered_in_a_real_leptos_table_and_escaped`，斷言兩個 `EventSummary`（含 `<script>` payload）在真正 Leptos SSR table 顯示、中文 caption、精確總數、HTML escaping；新 `render_dashboard_with_rows` 在 Green 前缺符號，屬**編譯 Red**。在既有 `analytics_server/tests/day39_export_csv.rs` 真 PG Router 測試，保留原帳密/Session/CSV 斷言，新增 Dashboard 頁面的 Chat 兩 kind、Pet 不洩漏、精確篩選不顯示其他 kind，屬**行為 Red**。未跑 PostgreSQL 不得算 Red 通過；本次未實際執行。

### Green｜完整檔案、caller 與測試

[Analytics Green patch](analytics-day41-green.patch)（Refactor 先套）完整修改 **3 個來源／測試檔**：

1. `analytics_apps/analytics_server/src/dashboard_ui.rs`：import `analytics_persistence::EventSummary`；`DashboardView` 增 `rows: Vec<(String,i64)>` 純呈現資料，既有來源 nav、kind filter、CSV link、空狀態與登出保留。在 summary-count 後加入真正 Leptos SSR table：

```rust
<table>
    <caption>"已提交事件類型明細"</caption>
    <thead><tr><th scope="col">"事件類型"</th><th scope="col">"已提交筆數"</th></tr></thead>
    <tbody>
        {rows.into_iter().map(|(kind, count)| view! {
            <tr><td>{kind}</td><td>{count}</td></tr>
        }).collect_view()}
    </tbody>
</table>
```

同檔新增 `render_dashboard_with_rows(source_label,total,source,event_type,rows:&[EventSummary])`，只把 adapter 結果投影為 `Vec<(String,i64)>`，交給原 Leptos `DashboardView`；舊 `render_dashboard_filtered` 的完整原簽名保留並轉呼叫新函式（空 rows），原 Day28/29／Day40 測試與 caller 不破壞。

2. `analytics_apps/analytics_server/src/password_login.rs`：正式 `dashboard` 在現有 PG `rows` 與 `summary_total` 成功後，將 `render_dashboard_filtered(...)` 換成 `render_dashboard_with_rows(source_label,total,source,kind,&rows)`；原 Session、來源白名單、kind 校驗、失敗狀態、Cookie 續期不變。
3. `analytics_apps/analytics_server/tests/day39_export_csv.rs`：保留全部原 PG 登入／CSV／隔離／登出斷言，追加 SSR 表格與來源／kind 隔離斷言；同一資料庫 fixture 的三種已提交事件仍直接查真 PostgreSQL。

Cargo/feature：`analytics-server` 已依賴 `analytics-persistence`、`leptos` SSR、`sqlx` PostgreSQL；整合測試須選 `postgres-integration`。沒有新依賴、空 library、migration、lock 或其他產品 Refactor。回復逆 Green 再逆 Refactor。

## ③ 版本核對（2026-10-27）

Rust stable **1.99.0**（2026-10-01，https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/）；Leptos **0.8.22**（https://docs.rs/crate/leptos/latest）；axum **0.8.9**（https://docs.rs/crate/axum/latest）；SQLx **0.9.0**（https://docs.rs/crate/sqlx/latest）；Tokio **1.54.1**（https://docs.rs/crate/tokio/1.54.1）；Argon2 **0.6.0**（https://docs.rs/crate/argon2/latest）；PostgreSQL **18.6**（2026-08-13，https://www.postgresql.org/docs/release/；19 RC1 排除）。manifest `leptos 0.8`、`axum 0.8`、`sqlx 0.9`、`tokio 1.54`；精確 Analytics Cargo.lock resolved `leptos 0.8.22`、`axum 0.8.9`、`sqlx 0.9.0`、`tokio 1.54.1`、`argon2 0.6.0`。未驗本機 PG server/CLI、MSRV、歷史資料庫升級，不自動升級。

## ④ 實際驗證／未完成／下一步

**已執行**：GitHub 精確來源 SHA／四檔全文與 manifest/lock 讀取、修改前字串匹配、Refactor → Green 的 preimage 產生；**未執行** git apply、Cargo、真 PG、瀏覽器、CI。

```bash
# cwd: rust_ai_ring 根；隔離副本 checkout 9c767978b20bd56e1c2182dac2f37e6ae9b7d988
git apply --check /path/to/analytics-day41-refactor.patch
git apply /path/to/analytics-day41-refactor.patch
git apply --check /path/to/analytics-day41-green.patch
git apply /path/to/analytics-day41-green.patch
cd analytics_apps
cargo fmt --all -- --check
cargo check --locked -p analytics-server
cargo test --locked -p analytics-server --lib day41
# DATABASE_URL 必須是專用可建立測試庫的 PG，不得指向正式庫
cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration
```

真瀏覽器驗收：管理員登入／改密碼、Chat/Pet 來源切換、兩 kind 的表格與 CSV 一致、精確篩選、空資料、HTML escaping、未登入／登出／機器 token 拒絕。**未完成**：檢視者角色與來源 ACL、SSE、操作稽核、真 PG/GUI/CI Gate2。方案／使用者套用／驗收分列。核心 Red/Green 15 分鐘；Refactor、版本、驗證、提交另計，不改 FIFO、`last_full_comparison`。
