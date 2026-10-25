# Analytics Day38 r2｜機器上報拒絕重複 Authorization Header

> **2026-10-26 教學交付**。精確唯讀程式來源 [rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9](https://github.com/williechen/rust_ai_ring/tree/9b89656f964b99e0cb89ba421855704a35c5ecb9)；文件規則 [合併後共用規則](../../../COMMON_LEARNING_RULES.md)。本文提供正式 Router／SQLx 測試、最小 Green 與直接相關測試；**未修改產品、未執行 Red／Green／PostgreSQL／CI，功能未驗收**。

本修訂保留[原版](analytics.md)，以當前精確來源提供[完整可套用patch](analytics-r2.patch)。核心60分鐘只計②主Red／Green；局部Refactor、工程與其他交付另計。這是同Day修訂，next_day維持39。

從未改過的來源使用完整patch；若先手動加入Red測試，依Green正文補程式，不重複套入已含測試的patch。來源有本地變更先審查差異，不覆寫。完整patch的SHA256：`89e2bd9fe90c59c6949ce952aa39787eea51093d759f01ee725e45e7be71969f`。

規則精確來源：copilot_use `43b0c4f4eb5d6b65d6d3155a473b2a673d944046`，共用規則 blob `420147c5202bc2dd44fe44a4bb83da2c1d91d5cf`。

## ① 功能說明與上一 Day 缺口

[Day37 r3](../day-37/analytics-r3.md) 已將**人員 Session Cookie** 的歧義拒絕接到兩種 summary Router；最新來源仍保留此修補，不重教、不重新宣稱已驗收。另一個**獨立的機器入口** `analytics_apps/analytics_server/src/lib.rs::ingest` 目前用 `headers.get(AUTHORIZATION)` 取第一個值，`IngestTokens::authorized` 僅比對那個字串。若同一請求帶兩條 `Authorization`，第一條合法就可能被當成合法事件；代理、下游與稽核系統對多值解讀可能不同。本日收斂 HTTP 邊界：**必須恰有一條可解析 Authorization，且其 Bearer token 與事件 source_app 相符**。未授權請求不得寫入 `product_events`；合法單一 Chat／Pet 來源仍可各自上報。

正式鏈：Chat／Pet 機器客戶端 → `POST /api/v1/events` → `analytics_server::ingest`（Axum HTTP／header）→ `IngestTokens::authorized` → `pg_store::persist_event`（既有 SQLx PostgreSQL transaction／冪等）→ 201/200/409 ACK。此路由是機器上報，**不能要求人員 Cookie**；`/api/v1/events/{source}/summary` 則繼續使用 Day37 的人員 Session。Leptos Dashboard、Application／Domain 空範本不因本篇變成已接通；未實作的短效服務憑證／配對／輪替、管理員與檢視者角色／來源 ACL 仍待完成。

## ② Red → 完整最小 Green → Refactor

### Red：新增真正 Router + 真 PostgreSQL 的 Header 歧義測試

新增 `analytics_apps/analytics_server/tests/day38_ingest_authorization.rs` **完整檔案**。來源已存在的正常攝取／去重測試是回歸基準，不當作新 Red。這個新測試在未改生產碼時，`[合法,非法]` 與 `[合法,合法]` 多條 Header 預期 401，但原 `headers.get` 只看第一個，預期實際 201；**這是預期行為 Red，尚未執行**。測試使用 `#[sqlx::test]` 的隔離 PG fixture，不複製事件 SQL 演算法。

```rust
#![cfg(feature = "postgres-integration")]

use analytics_server::{IngestState, IngestTokens, build_app};
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode, header},
};
use std::sync::Arc;
use tower::ServiceExt;

async fn post_event(app: &Router, source: &str, auth: &[&str]) -> StatusCode {
    let body = serde_json::json!({
        "schema_version": 1,
        "event_id": uuid::Uuid::new_v4(),
        "source_app": source,
        "event_type": "test.ingest.authorization"
    });
    let mut request = Request::post("/api/v1/events")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    for value in auth {
        request.headers_mut().append(
            header::AUTHORIZATION,
            value.parse().unwrap(),
        );
    }
    app.clone().oneshot(request).await.unwrap().status()
}

#[sqlx::test(migrations = "../migrations")]
async fn ambiguous_machine_authorization_is_rejected_before_db_write(
    pool: sqlx::PgPool,
) {
    let app = build_app(IngestState {
        pool: pool.clone(),
        tokens: Arc::new(IngestTokens {
            chat: "fixture-chat".into(),
            pet: "fixture-pet".into(),
        }),
    });

    assert_eq!(
        post_event(&app, "web_chat", &["Bearer fixture-chat"]).await,
        StatusCode::CREATED
    );
    for auth in [
        vec!["Bearer fixture-chat", "Bearer fixture-pet"],
        vec!["Bearer fixture-chat", "Bearer fixture-chat"],
        vec!["Bearer fixture-pet", "Bearer fixture-chat"],
        vec!["Bearer fixture-pet"],
        vec![],
    ] {
        assert_eq!(
            post_event(&app, "web_chat", &auth).await,
            StatusCode::UNAUTHORIZED,
            "authorization headers: {auth:?}"
        );
    }
    assert_eq!(
        post_event(&app, "desktop_pet", &["Bearer fixture-pet"]).await,
        StatusCode::CREATED
    );
    assert_eq!(
        post_event(&app, "desktop_pet", &["Bearer fixture-chat"]).await,
        StatusCode::UNAUTHORIZED
    );
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM product_events")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 2, "only the two valid single-header events persist");
}
```

### Green：集中唯一 Header 解析，正式 ingest 和測試 helper 共用

在 `analytics_apps/analytics_server/src/lib.rs` 的 `impl IngestTokens` **之前**新增完整函式；沿用檔首已有 `use axum::http::{HeaderMap, header::AUTHORIZATION};`，不新增 import 或套件：

```rust
fn single_authorization(headers: &HeaderMap) -> Option<&str> {
    let mut values = headers.get_all(AUTHORIZATION).iter();
    let first = values.next()?.to_str().ok()?;
    if values.next().is_some() {
        return None;
    }
    Some(first)
}
```

在**正式** `pub async fn ingest` 內，把原本這段：

```rust
    let authorization = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
```

替換成：

```rust
    let authorization = single_authorization(&headers).unwrap_or("");
```

保留後續原始碼**完整不變**，它仍是正式路由與真正 PG Adapter：

```rust
    if !state.tokens.authorized(&event.source_app, authorization) {
        return StatusCode::UNAUTHORIZED;
    }
    match pg_store::persist_event(&state.pool, &event).await {
        Ok(outcome) => outcome_status(outcome),
        Err(PersistError::Invalid) => StatusCode::BAD_REQUEST,
        Err(PersistError::Database(_)) => StatusCode::SERVICE_UNAVAILABLE,
    }
```

同檔 `#[cfg(test)] async fn source_summary` 的原 `headers.get(AUTHORIZATION)` 解析段也換成同一行 `let authorization = single_authorization(&headers).unwrap_or("");`，避免測試 helper 持續維護較寬鬆的 Header 契約；該 helper **不是正式 query 路由**，仍僅 `cfg(test)`，不能藉它宣稱業務查詢可用機器 token。這是本次 Refactor，無新增對外 API／第二套認證。

**驗證順序**：先判斷事件 schema／source/type（原 `valid_event`），再拒絕 0 或 >1 個 Authorization，才允許 SQLx 寫入。單值 token 的固定長度比較原樣保留；拒絕多值不是短效憑證／防重播的完整方案。

### 局部 Refactor：只共用本功能的 Header 決策

正式 ingest 與既有測試 helper 共用唯一解析器，其餘持久化／人員Session斷言保留。跨專案CI責任隔離已移到[每日工程完整工作包](../../../engineering/2026-10-26/ci-isolation.md)，不占本篇主題、Red／Green時間或功能變更範圍。

### Cargo、模組、真 caller 與測試保留

四個 workspace `chat_apps`、`pet_apps`、`analytics_apps`、`crates` 各有獨立 Cargo.toml/Cargo.lock。此切片改 `analytics_apps/analytics_server/src/lib.rs`，新增 `analytics_apps/analytics_server/tests/day38_ingest_authorization.rs`；package `analytics-server`、`analytics-persistence`，Rust crates `analytics_server`／`analytics_persistence`。現有 `axum`、`sqlx`（postgres/macros/runtime-tokio）、`uuid`、`serde_json`、`tower`（dev）已足夠；`postgres-integration` feature 已連到 persistence。**不新增 Cargo dependencies、features、imports、migrations、Leptos metadata 或 UI**。正式 `build_app → password_login::build → /api/v1/events` 不改路由，只有 ingest 使用新解析器；機器 token 不授權 Dashboard。

| 既有測試／入口 | 處置 | 保留契約 |
| --- | --- | --- |
| `postgres_event_store`／`idempotency_contract` | 保留回歸 | 真 PG 寫入／衝突／冪等與錯誤 |
| `day37_query_cookie` | 保留回歸 | 人員 Cookie 唯一性、summary 401／403／200 |
| `source_summary`（test-only） | 共用解析器，不提升成產品路由 | 測試專用來源 token 邊界 |
| 新 `day38_ingest_authorization` | 新增 PG Router 測試 | 重複 Header 拒絕、單值兩來源成功、DB 不受污染 |

## ③ 當日官方穩定版本與採用判斷

2026-10-26 台北日期，crates/Rust於 `2026-10-25T23:01:56.157211+00:00` 重查；排除預發行與 yanked。[完整宣告／lock／官方網址／原始回應SHA](r2-versions.json)。來源9b與648外部宣告及resolved一致；9b內部package為0.2.37。

| 套件／工具 | manifest 宣告 | lock resolved／已安裝 | 官方穩定版／MSRV |
| --- | --- | --- | --- |
| axum | `0.8` | 0.8.9 | [0.8.9](https://crates.io/crates/axum/0.8.9)／1.80 |
| sqlx | `0.9` | 0.9.0 | [0.9.0](https://crates.io/crates/sqlx/0.9.0)／1.94.0 |
| tokio | `1.54` | 1.54.1 | [1.54.1](https://crates.io/crates/tokio/1.54.1)／1.85 |
| uuid | `1.29` | 1.29.0 | [1.29.0](https://crates.io/crates/uuid/1.29.0)／1.89.0 |
| tower | `見package/target表` | 0.5.3 | [0.5.3](https://crates.io/crates/tower/0.5.3)／1.64.0 |
| serde | `1.0` | 1.0.229 | [1.0.229](https://crates.io/crates/serde/1.0.229)／1.56 |
| serde_json | `1.0` | 1.0.154 | [1.0.154](https://crates.io/crates/serde_json/1.0.154)／1.71 |
| argon2 | `0.6` | 0.6.0 | [0.6.0](https://crates.io/crates/argon2/0.6.0)／1.85 |
| sha2 | `0.11` | 0.10.9,0.11.1 | [0.11.1](https://crates.io/crates/sha2/0.11.1)／1.88 |

Rust官方stable 1.99.1，[官方channel](https://static.rust-lang.org/dist/channel-rust-stable.toml)，本機rustc/cargo同版。最高所列MSRV不高於本機Rust，但不等於SSR／WASM／native／DB建置通過。相符resolved維持；若有差異優先以最新穩定版驗證遷移，本包不改lock、安裝工具或升級。

PostgreSQL官方穩定18.6（排除19RC）；已安裝psql18.6是客戶端，伺服器版本未在本次啟動核驗。來源workflow使用postgres:latest，並未因工程proposal改成18；本篇用隔離PG18測試方案。SQLx CLI未安裝，官方0.9.0，須與庫同版遷移驗證，不自動安裝或跑migration。
[PostgreSQL官方支援版本](https://www.postgresql.org/support/versioning/)。

資料庫／native當日官方來源、已安裝探測與採用理由見[版本詳細核對](../../../source-review/2026-10-26-versions-and-summary-refresh.md)，不能以metadata宣稱功能已驗收。
## ④ 驗證命令、狀態與舊 Day 更正

```bash
cd rust_ai_ring/analytics_apps
# DATABASE_URL 指向可建立 SQLx 測試 DB 的隔離 PostgreSQL 18 測試角色。
export DATABASE_URL='postgres://postgres:postgres@localhost:5432/analytics_ci'
cargo test --locked -p analytics-server --features postgres-integration --test day38_ingest_authorization
cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration
# 原略過項依實際 fixture 環境另執行；不把 ignored 計為通過。
# CI沿產品現行workflow追蹤；跨專案job拆分另見工程工作包。
```

| Gate | 預期 | 本次實際 |
| --- | --- | --- |
| Red | 多條 Header 以合法 token 開頭會被舊正式 ingest 接受，測試失敗 | 未執行；僅來源可推論 |
| Green PG／Router | 單值 Chat/Pet 201；重複／跨來源 401；只有兩筆事件 | 未執行 |
| 原有 PG／Cookie／冪等 | 原測試斷言完整保留且通過 | 未執行 |
| 人員權限／GUI | Query 仍用 Session，Leptos／角色／來源 ACL 另驗 | 未驗收 |

**Day37 更正補充**：人員 Cookie 歧義已修正，不代表機器 Authorization 歧義已修正；本篇是新的獨立入口切片。Day35／36 的精確類型查詢、Dashboard Session 仍保留。固定來源 token 是過渡，不能取代服務配對、短效憑證、輪替與撤銷。本次不更動既有 Day 正文、來源、原 commit/blob/SHA256、`last_full_comparison`；只交付下一 Day 的文件與使用者可套用的程式；跨專案CI另列工程。

本次新增靜態驗證：在隔離來源快照，`git apply --check`通過；完整修改檔以rustfmt解析通過。[逐檔來源blob與結果](r2-static-checks.json)。這兩項不代表編譯、Red失敗、Green通過、DB、native、GUI或CI；上述仍未執行。產品倉庫未套用本候選內容。
