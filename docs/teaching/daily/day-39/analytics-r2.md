# Analytics Day39 r2｜管理後台依 Session 匯出指定來源事件統計 CSV

> **2026-10-26 r2 修訂**：Refactor 正文提供逐檔程式、完整測試與斷言承接；本地驗證僅對精確8fc來源＋三案Refactor，未套本篇功能Green。產品checkout保持唯讀。原[首版](analytics.md)與[r1](analytics-r1.md)保留。

共用規則精確commit `0ccd1a252b3e8fc28150242944b36929760b3cd3`，blob `ba9e6cbd6763d245e60800a02c60888ba6215bd5`。

產品來源 `williechen/rust_ai_ring/main@8fc4b277b6f6f77d97655130036fca47813de56e`；只發布 `williechen/copilot_use/main` 文件。候選Refactor的實測結果列在獨立段落，②功能Green的驗證狀態另列；兩者不互相替代。

## ① 使用者功能與邊界

管理者在 Leptos 後台選擇 Chat 或 Pet 來源後，點「匯出目前來源 CSV」，下載該來源所有已提交事件類型與筆數；其他來源不得混入。現行 `analytics_server::password_login::dashboard` 已依人員 Session 查詢 `summary_for_source`，`dashboard_ui::DashboardView` 已顯示來源選擇與篩選，但沒有下載入口。新路由 `GET /api/v1/events/{source}/export.csv` **先驗有效、已完成首次改密碼的人員 Session**，再核對來源白名單、查 PostgreSQL 聚合；機器 Bearer token 不可用於下載。CSV 附 BOM、固定檔名、CRLF、完整引號跳脫與公式注入防護。既有精確事件類型篩選不在本次 CSV 匯出範圍（明確標示「目前來源」而非「目前篩選」）。

核心60分鐘僅② Red／Green；Refactor、版本、來源、文件與測試驗收另計。Analytics Leptos SSR 正式頁維持，不建立手寫 HTML 第二套前台。

## Refactor

### 1. 實際分層缺口與呼叫鏈

精確8fc來源已把「來源＋事件類型」查詢放到 `analytics_persistence`，但「只按來源」聚合仍留在 server 的 `pg_store.rs`。Dashboard、Query API 與測試因此經不同模組取得相同 DTO。此次把來源聚合 SQL 移到既有 Persistence Adapter，保留薄的相容 re-export，讓正式 caller 與 SQL 責任分開。它不依賴②CSV匯出，也不改人員 Session 或來源授權政策。

| 檔案 | 完整變更單元 | 接線結果 |
| --- | --- | --- |
| `analytics_persistence/src/lib.rs` | 新增來源聚合函式及 PG 測試 | 和既有 exact-kind Adapter 共用 `EventSummary` |
| `analytics_server/src/pg_store.rs` | 移除原完整 SQL 函式，新增公開 re-export | 舊 `pg_store::summary_for_source` caller 與測試不用同時破壞 |
| `analytics_server/src/password_login.rs` | 更新正式 import | Dashboard／summary 直接呼叫 Persistence，Session 檢查保留 server |

表內路徑皆位於 `analytics_apps/`。沒有新增空 layer；原有 `analytics-persistence` crate 已具備真正 SQLx Adapter。

### 2. Persistence：完整新增函式

在 `analytics_persistence/src/lib.rs` 的 `EventSummary` 後、既有 `summary_for_source_and_type` 前加入：

```rust
pub async fn summary_for_source(
    pool: &sqlx::PgPool,
    source: &str,
) -> Result<Vec<EventSummary>, sqlx::Error> {
    sqlx::query_as::<_, EventSummary>(
        "SELECT source_app, event_type, COUNT(*)::bigint AS total
         FROM product_events
         WHERE source_app = $1
         GROUP BY source_app, event_type
         ORDER BY event_type",
    )
    .bind(source)
    .fetch_all(pool)
    .await
}
```

SQL 與原 server 實作一致：`$1` 綁定來源、同一 statement 取得聚合快照、COUNT轉bigint對應 i64、依 event_type 排序。沒有字串拼 SQL，也沒有在 HTTP 層另算統計。未知來源在 Adapter 回空集合；HTTP 路由是否拒絕來源仍由 server 負責。

### 3. Server：相容 API 與正式 import

`analytics_server/src/pg_store.rs` 開頭改成下面兩行；刪除檔尾原本完整 `pub async fn summary_for_source(...)` 函式，保留 `persist_event`、transaction、冪等與衝突處理全部原碼。不要留下第二份 SQL。

```rust
pub use analytics_persistence::summary_for_source;
use sqlx::PgPool;
```

`analytics_server/src/password_login.rs` 開頭使用：

```rust
use crate::IngestState;
use analytics_persistence::{summary_for_source, summary_for_source_and_type};
```

既有 Dashboard 在 kind 為空時呼叫 `summary_for_source(&state.pool, source).await`；有 kind 時呼叫 `summary_for_source_and_type`。此處不改 handler 或查詢分支，import 的符號現在直接解析至 Persistence。`lib.rs::source_summary` 及既有測試仍可經 `pg_store::summary_for_source` 相容路徑抵達同一函式。人員登入／Session 檢查仍在 HTTP handler，不能搬到 Persistence 或以 source token 取代。

### 4. Persistence：完整新增真 PG 測試

放入既有 `#[cfg(all(test, feature="postgres-integration"))] mod day35_exact_kind_adapter_tests`。測試沿用正式 migrations，每個 SQLx 測試獨立資料庫；不共用已污染的測試資料，也不讀正式庫。

```rust
#[sqlx::test(migrations = "../migrations")]
    async fn all_kinds_are_sorted_and_scoped_to_one_source(pool: sqlx::PgPool) {
        for (source, kind) in [
            ("web_chat", "z.last"),
            ("web_chat", "a.first"),
            ("web_chat", "a.first"),
            ("desktop_pet", "pet.only"),
        ] {
            sqlx::query("INSERT INTO product_events(source_app,event_id,schema_version,event_type) VALUES($1,$2,1,$3)")
                .bind(source).bind(uuid::Uuid::new_v4().to_string()).bind(kind)
                .execute(&pool).await.unwrap();
        }
        let rows = super::summary_for_source(&pool, "web_chat").await.unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(
            (&rows[0].event_type, rows[0].total),
            (&"a.first".to_owned(), 2)
        );
        assert_eq!(
            (&rows[1].event_type, rows[1].total),
            (&"z.last".to_owned(), 1)
        );
        assert!(rows.iter().all(|r| r.source_app == "web_chat"));
        for source in ["not.recorded", "' OR TRUE --"] {
            assert!(
                super::summary_for_source(&pool, source)
                    .await
                    .unwrap()
                    .is_empty()
            );
        }
    }
```

這個 fixture 刻意逆序插入 z.last／a.first，才能驗證 ORDER BY；Chat兩筆a.first、一本z.last驗證分組計數；Pet獨立事件驗證來源隔離；未知及SQL形狀輸入驗證空集合與綁定。它不代替舊的 transaction rollback 或 Session Router 測試。

### 5. 依賴、舊測試承接與 CI 入口

| 原契約／位置 | 此次處置 | 原因 |
| --- | --- | --- |
| `EventSummary` DTO | 原 Persistence 定義保持 | 不建立 server/persistence 各一份 DTO |
| exact-kind counts／`rolled_back_event_is_not_counted` | 原 Persistence PG 測試保留 | exact-kind與transaction不是本次全類型聚合測試可替代 |
| Day36 真 Router Session、source token拒絕 | 原 server 測試保留 | SQL 搬移不能繞過 HTTP 授權 |
| Day37 多 Cookie、登出／限制 Session | 原 Router 整合測試保留 | 相容 re-export 不等於人員權限已驗收 |
| Day38 重複 Authorization／寫入冪等 | 原 ingest 整合測試保留 | 另一個機器通道，不與查詢測試合併 |
| 新 all-kinds Adapter test | 直接呼叫新 Persistence 函式 | 證明真正SQL所在位置，非測試重寫相同行為 |

server 原本已依賴 `analytics-persistence`；`postgres-integration` feature 已傳入該 crate，Persistence 的 `sqlx` 與 `uuid` 依賴也已存在。故不改 Cargo、features、lock、migration 或全庫workflow；既有 PG CI 入口需繼續選 server＋persistence 及此 feature。只跑不帶feature的library不會執行這些PG測試。

### 6. 完整 patch、命令、實測與回復

[完整 Refactor patch](analytics-day39-refactor-r2.patch) 包含全部三檔修改；可以單獨套在8fc，不用先套CSV Green。前後兩種順序皆已 `git apply --check` 確認，原 SQL 沒有複製成平行實作。

```bash
# DATABASE_URL 指向有 CREATEDB 權限的專用本地測試庫，不用正式庫
git apply --check /path/to/analytics-day39-refactor-r2.patch
git apply /path/to/analytics-day39-refactor-r2.patch
cd analytics_apps
cargo test --offline --locked -p analytics-server -p analytics-persistence --features postgres-integration
```

**本次實測**：隔離精確8fc來源＋Refactor；`cargo test --offline --locked -p analytics-server -p analytics-persistence --features postgres-integration` exit 0，49項通過、0項失敗、1項ignored。[原始log](r2-validation/analytics-pg.log)；此結果不包含未套用的Day39功能Green。

相容 re-export 的代價是暫時保留舊模組 API；等所有 caller 遷移並有測試證據後再移除，本次不一口氣破壞。乾淨候選可先 `git apply -R --check` 再反向回復；本地有後續變更先三方比對。下一步沿 Analytics Session／角色與查詢 caller 接續，各案進度與全專案G-01分開計。

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
 git apply --check /path/to/analytics-day39-refactor-r2.patch
 git apply /path/to/analytics-day39-refactor-r2.patch
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

## ④ 驗證、證據與未完成

本r2新增的實測限於獨立Refactor：完整來源8fc的隔離副本套入三案Refactor，沒有套②功能Green。各案精確命令、通過／ignored數與原始log見本案Refactor段落及[驗證紀錄](r2-refactor-validation.json)。產品checkout與遠端main均未套用。

三案Refactor各自可直接套在原始8fc；也已實際檢查與各自Green前後兩種套用順序。原Chat Refactor及Chat／Pet Green存在換行格式不符的git apply失敗，本r2只修正Green patch換行格式，功能設計及斷言不變；原始失敗與修正後檢查都保留在[套用紀錄](r2-composition-checks.json)及[修正前紀錄](r2-composition-checks-before-eol-fix.json)。這些套用檢查不是Green功能測試。

**未執行**：②新功能Red／Green的Rust／WASM／Node／原生／瀏覽器測試與候選GitHub Actions；Pet Tauri server編譯及真生活窗事件亦未驗。Chat初次自動讀取、Pet找回視窗、Analytics CSV下載／試算表匯入各自依②的真入口驗收步驟執行，不能由Refactor回歸測試替代。

本次版本表保留同日r1的官方查核與來源lock紀錄，沒有改依賴或聲稱重新升級。Gate1依[本案精確commit全文讀回與blob／SHA256](analytics-r2-readback.json)記錄；Gate2仍是使用者套用後，以新產品SHA跑功能／DB／GUI與CI。獨立段落提供的是可套用重構程式與證據，不是產品main已重構或已驗收。

既有原文、FIFO與有效斷言保留；Refactor不新增Day，next_day維持40，last_full_comparison不變。各案後續責任見本案Refactor最後一段；全專案G-01／G-02另列[工程項目](../../../engineering/README.md)，不以本案測試通過抵算。
