# 2026-10-16｜P0 Analytics Day 13｜來源隔離的 PostgreSQL 統計查詢

來源文章：[2026-10-16｜P0 Analytics Day 13｜來源隔離的 PostgreSQL 統計查詢](https://app.notion.com/p/3fae51d4509381808a9cd813423d52d1?pvs=204)。

## 1. 學習大綱
P0／Analytics Day 13；主體實作學習時數 25 分鐘。學習時數只計第 2 節的主體實作（Red 與最小 Green）；Refactor／必要補加不計入學習時數，第 1、3、4 節亦不計入。三專案每日主體實作學習時數合計最多 60 分鐘。
來源：rust_ai_chat/main `2aecb0f02f9176287e848b7e4897f28ef3929072`；copilot_use/main `49e1d946dd3c22ffcc4b1aa0e7715cb5b1686724`；產品教學分支 `1d1668b97e65ccfafc073829f131c93133c8bb7e`。已讀共同規則、P9_TARGET、FUTURE_WORK、SYSTEM_ARCHITECTURE、learning-state 與 Notion 共同／專案／Day 12；共同規則為準。當前 P0，最晚切點 2026-12-12，未達切點也無提前完成證據，不切換階段。首次週六完整比對尚未成功，last_full_comparison 保持 null；本日只核對選題相關差異，不冒稱全量完成。
對應 FUTURE_WORK 原「同服務 Dashboard／Ingest／Query」與「四欄事件及來源權限、獨立 PostgreSQL／持久冪等」。最新 HTTP ingest 已接 pg_store::persist_event，Day 12 不再是 501；目前 build_app 只有 /healthz 與 POST /api/v1/events，沒有查詢入口。
成品 `apps/stats_server/src/lib.rs`::summary、[persistence.rs](http://persistence.rs)::EventStore::summary 已能依來源／事件類型聚合；學習庫沿用現有 pg_store 與 IngestState，新增來源授權的 GET /api/v1/events/\{source\}/summary。成品以獨立 API token 查全域資料；本日為避免直接搬入其整套狀態與管理權限，沿用學習庫來源 token，只能讀同來源聚合，不提供跨來源管理查詢。
## 2. 實作內容
### Red：先定義真實 handler／PostgreSQL 行為
在 `apps/stats_server/src/lib.rs` 檔尾新增以下私有模組（保留既有 tests）。先跑第 4 節命令，預期缺 source_summary／summary_for_source 而編譯失敗。合成 fixture 不讀環境密鑰；兩個 sqlx::test 僅在 postgres-integration feature 下執行，使用隔離測試 DB：
```rust
#[cfg(test)]
mod source_summary_tests {
    use super::*;
    use axum::extract::Path;

    fn fixture_tokens() -> Arc<IngestTokens> {
        Arc::new(IngestTokens {
            chat: "fixture-chat-only".into(),
            pet: "fixture-pet-only".into(),
        })
    }
    fn headers(token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
        headers
    }

    #[tokio::test]
    async fn summary_rejects_cross_source_before_database_access() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://fixture:fixture@127.0.0.1:1/unused").unwrap();
        pool.close().await;
        let state = IngestState { pool, tokens: fixture_tokens() };
        for (source, headers, expected) in [
            ("desktop_pet", headers("fixture-chat-only"), StatusCode::UNAUTHORIZED),
            ("web_chat", HeaderMap::new(), StatusCode::UNAUTHORIZED),
            ("unknown", headers("fixture-chat-only"), StatusCode::BAD_REQUEST),
            ("web_chat", headers("fixture-chat-only"), StatusCode::SERVICE_UNAVAILABLE),
        ] {
            let result = source_summary(
                State(state.clone()), Path(source.into()), headers,
            ).await;
            assert_eq!(result.err().unwrap(), expected);
        }
    }

    #[cfg(feature = "postgres-integration")]
    #[sqlx::test(migrations = "./migrations")]
    async fn summary_counts_committed_rows_per_source_and_kind(pool: PgPool) {
        let id = Uuid::new_v4();
        for (source, event_id, kind) in [
            ("web_chat", id, "chat.message.persisted"),
            ("web_chat", Uuid::new_v4(), "chat.message.persisted"),
            ("web_chat", Uuid::new_v4(), "chat.room.created"),
            ("desktop_pet", id, "pet.cleanup.succeeded"),
        ] {
            let event = EventInput {
                schema_version: 1, event_id,
                source_app: source.into(), event_type: kind.into(),
            };
            assert_eq!(pg_store::persist_event(&pool, &event).await.unwrap(),
                       PersistOutcome::Stored);
            assert_eq!(pg_store::persist_event(&pool, &event).await.unwrap(),
                       PersistOutcome::AlreadyExists);
        }
        let state = IngestState { pool: pool.clone(), tokens: fixture_tokens() };
        let Json(chat) = source_summary(
            State(state.clone()), Path("web_chat".into()), headers("fixture-chat-only"),
        ).await.unwrap();
        assert_eq!(chat.len(), 2);
        assert_eq!(chat[0].event_type, "chat.message.persisted");
        assert_eq!(chat[0].total, 2);
        assert_eq!(chat[1].event_type, "chat.room.created");
        assert_eq!(chat[1].total, 1);
        assert!(chat.iter().all(|v| v.source_app == "web_chat"));
        let Json(pet) = source_summary(
            State(state), Path("desktop_pet".into()), headers("fixture-pet-only"),
        ).await.unwrap();
        assert_eq!(pet.len(), 1);
        assert_eq!(pet[0].source_app, "desktop_pet");
        assert_eq!(pet[0].total, 1);

        // 查詢不依賴 event_id 儲存型別；既有 migration 保持不變。
        sqlx::query(
            "ALTER TABLE product_events ALTER COLUMN event_id TYPE UUID USING event_id::uuid"
        ).execute(&pool).await.unwrap();
        assert_eq!(pg_store::summary_for_source(&pool, "web_chat").await.unwrap(), chat);
    }

    #[cfg(feature = "postgres-integration")]
    #[sqlx::test(migrations = "./migrations")]
    async fn empty_and_rolled_back_events_are_not_counted(pool: PgPool) {
        assert!(pg_store::summary_for_source(&pool, "web_chat").await.unwrap().is_empty());
        let mut tx = pool.begin().await.unwrap();
        sqlx::query(
            "INSERT INTO product_events(source_app,event_id,schema_version,event_type)
             VALUES ('web_chat',$1,1,'chat.message.persisted')"
        ).bind(Uuid::new_v4().to_string()).execute(&mut *tx).await.unwrap();
        tx.rollback().await.unwrap();
        assert!(pg_store::summary_for_source(&pool, "web_chat").await.unwrap().is_empty());
    }
}
```
第一個測試的 pool 先關閉，證明未授權在查 DB 前回 401，合法授權才碰到 pool 並回 503；不是把 DB 故障測成成功。資料庫測試實際寫入並重送，驗證去重後計數、相同 ID 不同來源隔離、事件類型分組、空資料與 rollback。
### 最小 Green：擴充現有 PostgreSQL Adapter 與 HTTP
`apps/stats_server/src/pg_store.rs` 保留現有 persist_event／lookup_existing_event／錯誤型別，在檔尾新增：
```rust
#[derive(Debug, serde::Serialize, sqlx::FromRow, PartialEq, Eq)]
pub struct EventSummary {
    pub source_app: String,
    pub event_type: String,
    pub total: i64,
}

pub async fn summary_for_source(
    pool: &PgPool,
    source: &str,
) -> Result<Vec<EventSummary>, sqlx::Error> {
    // 來源值一律綁定；同一 SQL statement 取得一致的聚合快照。
    sqlx::query_as::<_, EventSummary>(
        "SELECT source_app, event_type, COUNT(*)::bigint AS total
         FROM product_events
         WHERE source_app = $1
         GROUP BY source_app, event_type
         ORDER BY event_type",
    ).bind(source).fetch_all(pool).await
}
```
`apps/stats_server/src/lib.rs` 在 build_app 之前新增：
```rust
async fn source_summary(
    State(state): State<IngestState>,
    axum::extract::Path(source): axum::extract::Path<String>,
    headers: HeaderMap,
) -> Result<Json<Vec<pg_store::EventSummary>>, StatusCode> {
    if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
        return Err(StatusCode::BAD_REQUEST);
    }
    let authorization = headers.get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok()).unwrap_or("");
    // URL 來源必須與憑證相符；不接受另一來源的 token。
    if !state.tokens.authorized(&source, authorization) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let rows = pg_store::summary_for_source(&state.pool, &source)
        .await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(rows))
}
```
完整替換同檔既有 build_app：
```rust
pub fn build_app(state: IngestState) -> Router {
    Router::new()
        .route("/healthz", get(health))
        .route("/api/v1/events", post(ingest))
        .route("/api/v1/events/{source}/summary", get(source_summary))
        .with_state(state)
        .layer(DefaultBodyLimit::max(4096))
}
```
其他 imports 已存在，無新 crate／無 main 設定變更。真實流程：GET 路由 → source_summary → 檢查 source／來源 token → pg_store::summary_for_source → PostgreSQL group by → JSON；DB 失敗回 503，不回空成功。空資料回 200 與 \[\]。輸出只含 source_app、event_type、total，不包含憑證／本文／路徑／event_id。
### Refactor
沿用上方 `pg_store::summary_for_source` 的單一參數化 SQL 與 `source_summary` 的授權／回應映射；SQL 留在 Adapter，HTTP handler 不複製聚合邏輯。套用時保留現有攝取函式及測試，只按本節指定位置新增／替換。本次沒有額外必要補加實作。
### 必要補加實作
N/A。本次為文章格式與同步修正，沒有新增功能實作；日後需要補加時，在本節末尾沿用 Red → 最小 Green → Refactor → 驗證格式接續完整實作。
## 3. 套件版本
workspace 0.2.12／edition 2024。沿用 axum 宣告 0.8／lock 0.8.9、sqlx 0.9／0.9.0（既有 postgres、macros feature）、serde 1.0／1.0.229、tokio 1.53／1.53.3、uuid 1.28／1.28.0；新 DTO 使用既有 serde Serialize 與 SQLx FromRow，不新增／升級依賴。官方 stable／MSRV／安全公告本次未另查證；既有 CI 不代表新增查詢已通過相容性測試。
## 4. 驗證項目
### 實作邊界與未完成項目
SQL 僅在既有 Adapter，採 \$1 bind，避免用字串拼 SQL 或記憶體 HashSet 假聚合；單次 SELECT 對應一次 statement snapshot。查詢不引用 event_id，測試在同一隔離 DB 將 VARCHAR(40) 改 UUID 後仍可讀；這是測試 schema 操作，不更改已套用 migration，也不表示舊 ingest 在 UUID 模式已驗收。
沿用 Day 12 手寫 token 比較，其正式密碼學安全審查未完成，loopback 不等於可公開部署。沒有新增 gRPC／Protobuf／ACK、SSE、Dashboard UI、治理／封存／撤回、Token 費用／額度或 OTel。這些留原 FUTURE_WORK 項目，不標完成；本日統計是已存事件列數，不是獨立使用者、帳單或釋放空間。未變檔 N/A。
### 驗證命令與結果
本次重新查核最新來源 SHA 的六份 workflow 定義與實際紀錄：Rust AI Chat、Test All、Persistence、Node、Rust Dashboard、Rust Desktop Pet，6/6 workflows、18/18 checks 均 completed/success，沒有 skipped/cancelled/pending check。main 未設 branch protection 必要 contexts；另無 legacy commit-status context（status API 的 pending/total_count=0 只表示沒有 legacy status，並非一個待完成工作），此庫適用 Gate 為上述 Actions/check-runs，已逐一核對，沒有用歷史 CI 替代。[匹配 SHA 的 CI](https://github.com/williechen/rust_ai_chat/actions?query=branch%3Amain)。基準 CI 通過只允許教學發布，不代表以下新增程式已套用或測試通過。
```bash
cargo test --locked -p stats-server source_summary_tests
cargo test --locked -p stats-server --features postgres-integration source_summary_tests
cargo test --locked -p stats-server --features postgres-integration --test postgres_event_store
cargo fmt --all -- --check
cargo clippy --locked -p stats-server --all-targets --features postgres-integration -- -D warnings
```
資料庫測試須由使用者提供專用 PostgreSQL 與測試 runner 所需 DATABASE_URL（例如本地隔離 fixture），sqlx::test 每測試獨立建庫並套用原 migrations；不得指向私人或正式 DB。這裡未讀取／列印任何密鑰，不新增載入 .env 的指令。
預期 Red 缺新函式；Green 同來源 200 JSON、跨來源／無憑證 401、未知來源 400、DB 關閉 503、重送不加計、rollback 不可見。另需啟動隔離服務後以合成憑證實際 HTTP GET 核對同一路由、Content-Type、上述 status、JSON 及 DB 計數；本日私有 handler 測試不能代替 router/socket E2E。
本次未套用程式、未執行新測試或 PostgreSQL／HTTP E2E，Red／Green 為待執行預期；來源 SHA 的 CI 已重新查核。程式尚待套用、功能尚未由本日實作證明、正式驗收未通過；教學發布與讀回依實際交易紀錄。未讀密鑰、未執行付費 AI、未修改 tag。
