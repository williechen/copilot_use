# Analytics Day38｜機器上報拒絕重複 Authorization Header，PostgreSQL 驗證獨立於 Pet 原生 CI

> **2026-10-26 教學交付**。精確唯讀程式來源 [rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9](https://github.com/williechen/rust_ai_ring/tree/9b89656f964b99e0cb89ba421855704a35c5ecb9)；文件規則 `daily-tdd-minimum-v1`、`functional-layering-v1`、`test-refactor-v1`。本文提供正式 Router／SQLx 測試、最小 Green 與 CI 必要接線；**未修改產品、未執行 Red／Green／PostgreSQL／CI，功能未驗收**。

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

### Refactor：CI 的責任隔離（完整 workflow）

最新來源 `.github/workflows/persistence.yml` 將 Chat persistence → **Pet server + persistence（Tauri native）** → Analytics server + persistence（PG）串成**同一 job**；前段 Pet native 失敗就會 skip Analytics PG。這是已知 CI 選取缺陷，與本篇新增 PG 測試直接相關。使用者套用時以以下**完整 YAML**取代 `.github/workflows/persistence.yml`，不是另建不會被呼叫的測試命令；原 `test-persistence` required check 如被 branch protection 使用，需同步更新 required checks。原 Pet server 測試不刪除，只移至正確 native job；三個資料測試不互相 `needs`，不以 `continue-on-error` 吞失敗。此 workflow 只針對本次相關 DB／原生責任；其它原有 workflow 不變。

```yaml
name: Persistence
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
permissions:
  contents: read
concurrency:
  group: persistence-${{ github.ref }}
  cancel-in-progress: true
env:
  CARGO_TERM_COLOR: always
jobs:
  chat-sqlite:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Chat SQLite adapter tests
        working-directory: chat_apps
        env:
          DATABASE_URL: "sqlite::memory:"
        run: cargo test --locked -p chat-persistence

  pet-sqlite:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Pet SQLite adapter tests (no Tauri)
        working-directory: pet_apps
        env:
          DATABASE_URL: "sqlite::memory:"
        run: cargo test --locked -p pet-persistence

  pet-native:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Tauri native dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y build-essential pkg-config curl wget file \
            libssl-dev libxdo-dev librsvg2-dev libwebkit2gtk-4.1-dev \
            libayatana-appindicator3-dev
      - uses: dtolnay/rust-toolchain@stable
      - name: Pet native/server tests (not GUI acceptance)
        working-directory: pet_apps
        env:
          DATABASE_URL: "sqlite::memory:"
        run: cargo test --locked -p pet-server

  analytics-postgres:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:18
        env:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: analytics_ci
        ports: ["5432:5432"]
        options: >-
          --health-cmd "pg_isready -U postgres -d analytics_ci"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Analytics SQLx adapter + real Router integration
        working-directory: analytics_apps
        env:
          DATABASE_URL: postgres://postgres:postgres@localhost:5432/analytics_ci
          ANALYTICS_DATABASE_URL: postgres://postgres:postgres@localhost:5432/analytics_ci
          ANALYTICS_CHAT_TOKEN: local-ci-chat-token
          ANALYTICS_PET_TOKEN: local-ci-pet-token
        run: cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration
```

**CI 契約**：`chat-sqlite` 與 `pet-sqlite` 只驗資料 adapter；`pet-native` 保留 Tauri server 原生編譯／測試（非真 GUI）；`analytics-postgres` 同時選 server 真 Router + persistence 自身測試。獨立 job 失敗會使 workflow 失敗，但不因 Pet failure skip PG。沒有為空的 analytics-application/domain/ui 建立假測試或宣稱已接線。若 required checks 原先指定 `test-persistence`，先在 GitHub branch rules 更新成新 job 名稱，否則 PR 可能永遠等待；此管理動作**尚未執行**。本篇提供完整必要 CI 接線，不修改 rust_ai_ring/main。

### Cargo、模組、真 caller 與測試保留

四個 workspace `chat_apps`、`pet_apps`、`analytics_apps`、`crates` 各有獨立 Cargo.toml/Cargo.lock。此切片改 `analytics_apps/analytics_server/src/lib.rs`，新增 `analytics_apps/analytics_server/tests/day38_ingest_authorization.rs`，調整 `.github/workflows/persistence.yml`；package `analytics-server`、`analytics-persistence`，Rust crates `analytics_server`／`analytics_persistence`。現有 `axum`、`sqlx`（postgres/macros/runtime-tokio）、`uuid`、`serde_json`、`tower`（dev）已足夠；`postgres-integration` feature 已連到 persistence。**不新增 Cargo dependencies、features、imports、migrations、Leptos metadata 或 UI**。正式 `build_app → password_login::build → /api/v1/events` 不改路由，只有 ingest 使用新解析器；機器 token 不授權 Dashboard。

| 既有測試／入口 | 處置 | 保留契約 |
| --- | --- | --- |
| `postgres_event_store`／`idempotency_contract` | 保留回歸 | 真 PG 寫入／衝突／冪等與錯誤 |
| `day37_query_cookie` | 保留回歸 | 人員 Cookie 唯一性、summary 401／403／200 |
| `source_summary`（test-only） | 共用解析器，不提升成產品路由 | 測試專用來源 token 邊界 |
| 新 `day38_ingest_authorization` | 新增 PG Router 測試 | 重複 Header 拒絕、單值兩來源成功、DB 不受污染 |
| `pet-server` 原測試 | 保留並在 native job 執行 | Tauri／IPC 測試不能冒充純 persistence |

## ③ 交付日官方最新穩定版本

官方核對日期 **2026-10-26 05:35 台北**（[crates.io API／四個 lock 版本原始證據](../../../source-review/version-refresh-2026-10-26/evidence.json)）；9b89656f964b99e0cb89ba421855704a35c5ecb9 的依賴未變。[Rust 1.99.1（10/15）](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)、[axum 0.8.9](https://crates.io/crates/axum/0.8.9)（MSRV 1.80）、[sqlx 0.9.0](https://crates.io/crates/sqlx/0.9.0)（1.94）、[tokio 1.54.1](https://crates.io/crates/tokio/1.54.1)（1.85）、[uuid 1.29.0](https://crates.io/crates/uuid/1.29.0)（1.89）、[tower 0.5.3](https://crates.io/crates/tower/0.5.3)（1.64）。Analytics lock 已相符；[PostgreSQL 官方支援版本](https://www.postgresql.org/support/versioning/)最新 18.6，CI pin major 18 並在 job 實際驗證，不以服務啟動代替 SQLx 成功。

## ④ 驗證命令、狀態與舊 Day 更正

```bash
cd rust_ai_ring/analytics_apps
# DATABASE_URL 指向可建立 SQLx 測試 DB 的隔離 PostgreSQL 18 測試角色。
export DATABASE_URL='postgres://postgres:postgres@localhost:5432/analytics_ci'
cargo test --locked -p analytics-server --features postgres-integration --test day38_ingest_authorization
cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration
# 原略過項依實際 fixture 環境另執行；不把 ignored 計為通過。
# CI：push/PR 後分別查 chat-sqlite、pet-sqlite、pet-native、analytics-postgres。
```

| Gate | 預期 | 本次實際 |
| --- | --- | --- |
| Red | 多條 Header 以合法 token 開頭會被舊正式 ingest 接受，測試失敗 | 未執行；僅來源可推論 |
| Green PG／Router | 單值 Chat/Pet 201；重複／跨來源 401；只有兩筆事件 | 未執行 |
| 原有 PG／Cookie／冪等 | 原測試斷言完整保留且通過 | 未執行 |
| CI 責任 | Pet native 失敗不使 Analytics PG skipped；整體仍正確失敗 | YAML 尚未套用／未執行 |
| 人員權限／GUI | Query 仍用 Session，Leptos／角色／來源 ACL 另驗 | 未驗收 |

**Day37 更正補充**：人員 Cookie 歧義已修正，不代表機器 Authorization 歧義已修正；本篇是新的獨立入口切片。Day35／36 的精確類型查詢、Dashboard Session 仍保留。固定來源 token 是過渡，不能取代服務配對、短效憑證、輪替與撤銷。本次不更動既有 Day 正文、來源、原 commit/blob/SHA256、`last_full_comparison`；只交付下一 Day 的文件與使用者可套用的程式／CI。
