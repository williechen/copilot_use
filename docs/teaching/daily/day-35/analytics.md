# Analytics Day35 r1｜事件類型精確統計 SQLx Adapter 接入 analytics-persistence

## ① 學習大綱：真實 Dashboard 與資料層的最小搬移

交付日期：2026-10-25（Asia/Taipei）；文件只寫 williechen/copilot_use/main，唯一產品來源 williechen/rust_ai_ring/main@891ff32ae29e1588dc40027cd5c9c99ca91d2378（唯讀）。即時 next_day=35，接續 [Analytics Day34](../day-34/analytics.md)。上一篇 ensure_default 已在 analytics_server/src/main.rs 的 migration 後呼叫，預設 admin 改密碼流程由 Leptos/Axum 接通；本日不重做登入、初始化或空的 UI skeleton。

**本日唯一切片：** 將已存在的「依 source_app + event_type 精確統計已提交事件」SQLx SELECT 及其 EventSummary Row DTO 從 analytics_server/src/pg_store.rs 移入**目前空的** analytics_persistence/src/lib.rs；更新正式 Axum API 與受管理者 Session 保護的 Dashboard 呼叫者，並讓舊 pg_store 路徑以 re-export 保留測試／外部使用相容。使用三種事件、兩個來源及 SQL injection 字串驗證隔離、聚合和 bind，不新增另一套 summary API 或未使用的 repository trait。

真實鏈 A：Leptos DashboardView 的 GET /dashboard?source=web_chat&event_type=... → analytics_server/src/password_login.rs::dashboard → 人員 Session／restricted 檢查 → analytics_persistence::summary_for_source_and_type → PostgreSQL product_events → render_dashboard_filtered。鏈 B：GET /api/v1/events/{source}/summary/{event_type}（來源 Bearer token 授權）→ analytics_server/src/lib.rs::source_kind_summary → **同一** SQLx Adapter → JSON Vec<EventSummary>。機器 ingestion POST /api/v1/events 仍由 pg_store::persist_event 去重後提交，不改其錯誤碼、schema、憑證或 idempotency；未提交／rollback 不計數。

來源四 workspace：chat_apps、pet_apps、analytics_apps package version 0.2.34，crates 0.2.32；目錄 analytics_server/analytics_persistence，Cargo package analytics-server/analytics-persistence，Rust crate analytics_server/analytics_persistence。現行 analytics_ui 仍未接業務，Leptos SSR 在 analytics_server/src/dashboard_ui.rs；不新增 Chat cargo-leptos metadata 或未使用的 hydration 元件。角色／來源 ACL、SSE、明細、gRPC 仍是獨立需求，單一 SQL adapter 不宣稱已完成。

## ② Red → 完整最小 Green → Refactor

### Red：先在新的資料層測真實 PostgreSQL

在目前空的 analytics_apps/analytics_persistence/src/lib.rs **先只放**以下測試模組（不新增 EventSummary/summary_for_source_and_type），預期找不到符號而編譯失敗；此為 Red，不是要在正式 PostgreSQL 執行破壞性 SQL。測試需隔離、可由 sqlx::test 建立的測試 DB。

~~~rust
#[cfg(all(test, feature = "postgres-integration"))]
mod day35_exact_kind_adapter_tests {
    use super::{EventSummary, summary_for_source_and_type};

    #[sqlx::test(migrations = "../migrations")]
    async fn exact_kind_counts_committed_rows_only_for_requested_source(pool: sqlx::PgPool) {
        for (source, kind) in [
            ("web_chat", "chat.message.persisted"),
            ("web_chat", "chat.message.persisted"),
            ("web_chat", "chat.room.created"),
            ("desktop_pet", "chat.message.persisted"),
        ] {
            sqlx::query(
                "INSERT INTO product_events (source_app,event_id,schema_version,event_type)
                 VALUES ($1,$2,1,$3)"
            )
            .bind(source)
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(kind)
            .execute(&pool).await.unwrap();
        }
        let chat = summary_for_source_and_type(
            &pool, "web_chat", "chat.message.persisted"
        ).await.unwrap();
        assert_eq!(chat, vec![EventSummary {
            source_app: "web_chat".into(),
            event_type: "chat.message.persisted".into(),
            total: 2,
        }]);
        let pet = summary_for_source_and_type(
            &pool, "desktop_pet", "chat.message.persisted"
        ).await.unwrap();
        assert_eq!(pet[0].total, 1);
        assert_eq!(pet.len(), 1);
        assert!(summary_for_source_and_type(
            &pool, "web_chat", "' OR TRUE --"
        ).await.unwrap().is_empty());
        assert!(summary_for_source_and_type(
            &pool, "web_chat", "not.recorded"
        ).await.unwrap().is_empty());
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn rolled_back_event_is_not_counted(pool: sqlx::PgPool) {
        let mut tx = pool.begin().await.unwrap();
        sqlx::query(
            "INSERT INTO product_events (source_app,event_id,schema_version,event_type)
             VALUES ('web_chat',$1,1,'chat.message.persisted')"
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .execute(&mut *tx).await.unwrap();
        tx.rollback().await.unwrap();
        assert!(summary_for_source_and_type(
            &pool, "web_chat", "chat.message.persisted"
        ).await.unwrap().is_empty());
    }
}
~~~

這裡測 SQLx adapter 的 DB 隔離、精確 bind、聚合與交易，不用 fake Vec 或僅測字串格式冒充 persistence。server 原有 source_kind_summary_tests、day27_dashboard_source 仍須回歸，尤其匿名 Dashboard 不能因搬層而繞過 Session。未實跑 Red；DB 未提供、migration checksum 失敗或 SQLx 建測試庫權限不足是**環境／既有 CI 缺口**，與預期缺符號 Red 分開記錄。

### Green A：analytics_persistence/src/lib.rs 完整實作

把下列程式放在上述測試模組**前面**，與 Red 測試合成新檔完整內容（不要重複測試模組）：

~~~rust
/// PostgreSQL 聚合查詢結果，供 JSON API 與 Leptos SSR 共用。
#[derive(Debug, serde::Serialize, sqlx::FromRow, PartialEq, Eq)]
pub struct EventSummary {
    pub source_app: String,
    pub event_type: String,
    pub total: i64,
}

/// 僅聚合已提交的精確來源與事件類型；輸入不拼入 SQL。
pub async fn summary_for_source_and_type(
    pool: &sqlx::PgPool,
    source: &str,
    event_type: &str,
) -> Result<Vec<EventSummary>, sqlx::Error> {
    sqlx::query_as::<_, EventSummary>(
        "SELECT source_app, event_type, COUNT(*)::bigint AS total
         FROM product_events
         WHERE source_app = $1 AND event_type = $2
         GROUP BY source_app, event_type"
    )
    .bind(source)
    .bind(event_type)
    .fetch_all(pool)
    .await
}
~~~

完全沿用原 pg_store 的 SQL、欄位、ORDER 語意（精確類型只會有零或一筆）與 Result 型別，不把來源／類型插入字串。persistence 只用 SQLx、Serde；原 analytics_persistence/Cargo.toml 已有 sqlx.workspace=true、serde.workspace=true、uuid.workspace=true，既有 [features] postgres-integration=[]，**不需要**新加這三項或創建空 application/domain 層。

### Green B：舊型別移除、相容 re-export 與兩個正式 caller

在 analytics_apps/analytics_server/src/pg_store.rs，保留 use sqlx::PgPool、EventInput/valid_event、PersistOutcome/PersistError、lookup_existing_event、persist_event 以及 summary_for_source。**刪除原本整段 EventSummary struct 與 summary_for_source_and_type 函式**（原檔第 71–109 行），改在 imports 之後加入：

~~~rust
// 對既有測試／外部使用保留舊路徑；真正 SQLx 實作已在 persistence。
pub use analytics_persistence::{EventSummary, summary_for_source_and_type};
~~~

此 re-export 讓原本 pg_store::EventSummary、pg_store::summary_for_source_and_type 的舊測試可編譯；原 summary_for_source 仍以 EventSummary 解碼 PostgreSQL 的相同欄位，故**不應**保留第二份同名 struct。pg_store 的其它函式原樣保留，無需重新貼整個事件攝取流程。

在 analytics_apps/analytics_server/src/lib.rs::source_kind_summary 的原正式呼叫：

~~~rust
let rows = pg_store::summary_for_source_and_type(&state.pool, &source, &event_type)
    .await
    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
~~~

只將函式路徑換成以下完整語句：

~~~rust
let rows = analytics_persistence::summary_for_source_and_type(
    &state.pool, &source, &event_type
)
.await
.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
~~~

**注意：** 如果原始碼的錯誤映射不是 SERVICE_UNAVAILABLE，保留原函式後面的 map_err/status 原樣；唯一必要替換是 pg_store::summary_for_source_and_type → analytics_persistence::summary_for_source_and_type，不改原 handler 的權限驗證、StatusCode、回應 Json 或路由。原正式函式與 API 測試已存在。

在 analytics_apps/analytics_server/src/password_login.rs::dashboard 原有的 kind 非空分支：

~~~rust
pg_store::summary_for_source_and_type(&state.pool, source, kind).await
~~~

替換為：

~~~rust
analytics_persistence::summary_for_source_and_type(&state.pool, source, kind).await
~~~

其餘 kind 空值仍用 pg_store::summary_for_source；原 session()、restricted、來源白名單、Query、render_dashboard_filtered 及錯誤分支全部保留。不得把 SQLx 搬入 Leptos UI、繞過 Dashboard 的登入守衛，或讓前端持有 PostgreSQL DSN。

### Green C：Cargo dependency、feature、lock 的精確更新

在 analytics_apps/analytics_server/Cargo.toml 的 [dependencies] 加入：

~~~toml
analytics-persistence = { path = "../analytics_persistence" }
~~~

同檔現有 feature 由：

~~~toml
[features]
postgres-integration = []
~~~

替換為：

~~~toml
[features]
postgres-integration = ["analytics-persistence/postgres-integration"]
~~~

因此執行 server 的 PostgreSQL 整合測試時能一致啟用被依賴的 persistence feature；單獨執行 persistence 測試仍可 -p analytics-persistence --features postgres-integration。analytics-persistence 的 Cargo.toml 及 lock 內原有依賴全部保留，不額外升級。analytics_apps/Cargo.lock 僅在 [[package]] name="analytics-server" version="0.2.34" dependencies 新增 "analytics-persistence"（在 "argon2" 前）：

~~~diff
@@ [[package]] analytics-server / dependencies
 dependencies = [
+ "analytics-persistence",
  "argon2",
  "axum",
~~~

從 analytics_apps 執行 cargo metadata --locked 確認新增依賴邊；若提示 lock 不一致，先查漏掉上述一行，不用 cargo update 全庫升級。Rust imports 可用 analytics_persistence::... 全限定路徑，不加不存在的 analytics_ui crate；PostgreSQL feature 仍由 analytics_apps workspace 的 sqlx postgres/runtime-tokio/macros 提供。沒有新 migration、UI metadata、HTTP 路由或 Leptos hydration 功能。

### Refactor 與舊 Day 更正／未完成缺口

Refactor 的結果是 SQLx 精確類型查詢只有一份，Dashboard 與 API 共享 adapter，pg_store 只保留相容性 re-export；SQLx 的 full-source summary 與 ingestion 仍在 server，analytics-persistence **只承接一個有 caller 的真實切片**。後續搬剩餘 SQL 時需同步呼叫者、錯誤契約、整合測試與 Cargo，不能因 lib.rs 不再空就宣稱全庫已分層。

Day34 ensure_default 已接 main／CLI，預設 admin 密碼強制變更主線保留；其首次啟動、並行初始化、密碼撤銷、PostgreSQL／HTTP／瀏覽器未於本次重跑。Day33 summary_count_label 已存在，不重教；Day28 原 SSR 測試舊 API 應使用現行 view!／.to_html() 並保留原文歷史，不能將過時斷言當成新功能。Day24 FIFO 已按原 manifest 結案，不重複增加；角色／來源 ACL、SSE、明細、gRPC、共用認證仍待實作及驗收。

## ③ 套件與軟體版本（2026-10-25 官方最新穩定）

| 項目 | rust_ai_ring 宣告／Cargo.lock | 官方最新穩定與證據 |
| --- | --- | --- |
| Rust/Cargo | rust-toolchain=stable；安裝版未量測 | [1.99.1（10/15）](https://blog.rust-lang.org/releases/) |
| SQLx/Postgres | 0.9 runtime-tokio/macros/postgres → 0.9.0 | [0.9.0](https://docs.rs/crate/sqlx/latest) |
| Axum | 0.8/ws → 0.8.9 | [0.8.9](https://docs.rs/crate/axum/latest) |
| Leptos SSR | 0.8/ssr → 0.8.22 | [0.8.22](https://docs.rs/crate/leptos/latest) |
| Tokio | 1.54/full → 1.54.1 | [1.54.1](https://docs.rs/crate/tokio/latest) |
| Serde / UUID | 1.0/derive → 1.0.229；1.29/v4 → 1.29.0 | [Serde](https://docs.rs/crate/serde/latest)／[UUID](https://docs.rs/crate/uuid/latest) |
| PostgreSQL server | runtime 未啟動／安裝版未量測 | [18.6 穩定，19 RC1 非穩定](https://www.postgresql.org/docs/release/) |

四份來源 lock blob：chat 8fe9db93e078e8633f283834fd01e6805d97f9b3、pet b38668fac8b3d8cd105d54d3baf9e3a8f0b3e1a5、analytics 9f6a1720ec999263d162a1e8c90c0fbc1b3dd6a7、crates 5321bfdaaaa24bb796c40092e39a4aa8a27f2d8f。SQLx 0.9.0 與 PostgreSQL 18.6 是不同軟體層；實際 DB 以 SHOW server_version 核實，未假定本機安裝。edition 2024 至少 Rust 1.85；建議 stable 1.99.1，其他套件最低 MSRV 矩陣未實跑。此次只增加同 workspace path dependency，不更新第三方版本，不用 RC/nightly 替代穩定版。

## ④ 驗證：工作目錄、Red/Green 預期與實際狀態

**在使用者的 rust_ai_ring/analytics_apps 目錄**，使用隔離的 PostgreSQL 測試 DB（SQLx #[sqlx::test] 需要建立／清理測試 DB 權限，勿指向正式資料庫）：

~~~bash
cd analytics_apps
cargo metadata --locked --format-version 1 --no-deps
# Red 先預期 persistence 缺符號；套完整 Green 後重新執行：
cargo test --locked -p analytics-persistence --features postgres-integration day35_exact_kind_adapter_tests
cargo test --locked -p analytics-server --features postgres-integration source_kind_summary_tests
cargo test --locked -p analytics-server --features postgres-integration
cargo test --locked -p analytics-server --all-targets
cargo check --locked -p analytics-server --all-targets
cargo fmt --all -- --check
# 實機：SHOW server_version; 以新管理者 Session 打開 /dashboard?source=web_chat&event_type=chat.message.persisted
~~~

Red：新 persistence API 缺符號；Green：精確類型 web_chat=2、desktop_pet=1、其他類型及 SQL injection=0、rollback 不計數；舊 pg_store 呼叫路徑與兩個正式 caller 維持可編譯。真實 HTTP/瀏覽器另測：匿名 GET /dashboard 不顯示業務資料；已登入且改完初始密碼的管理者可依來源／類型切換；來源 token 只能呼叫授權的機器 summary API，不可當人員 Session；跨來源不能混計，空結果清楚顯示；錯誤與重試不洩漏 SQL/DSN。PostgreSQL migration checksum 失敗需讀日誌定位，不應修改既有 migration 假造綠燈。

**實際：** 本次只在 copilot_use 發布教學，產品 rust_ai_ring 未套用，Rust／PostgreSQL／HTTP／Leptos SSR／瀏覽器與 CI 未在本次執行，Gate 2 未驗收。Day34 舊來源的 CI 結果不能推定此 SHA 或此未套用切片成功；最新來源 CI／DB 結果另需獨立核實。剩餘缺口：角色／來源 ACL、SSE、明細、gRPC／Protobuf、真正管理者權限、剩餘 SQL adapter 搬移、整合測試與 UI 驗收。保留歷史 commit/blob/SHA256，不更新 last_full_comparison；本篇精確 commit 全文讀回及七首頁/ENTRY/進度另存交付證據。
