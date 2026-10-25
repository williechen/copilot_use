# Analytics Day38 r3｜機器上報拒絕重複 Authorization Header

> **2026-10-26 教學交付**。精確唯讀程式來源 [rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9](https://github.com/williechen/rust_ai_ring/tree/9b89656f964b99e0cb89ba421855704a35c5ecb9)；文件規則 [合併後共用規則](../../../COMMON_LEARNING_RULES.md)。本文提供正式 Router／SQLx 測試、最小 Green 與直接相關測試；**未修改產品、未執行 Red／Green／PostgreSQL／CI，功能未驗收**。

本修訂保留[原版](analytics.md)，以當前精確來源提供[完整可套用patch](analytics-r2.patch)。核心60分鐘只計②主Red／Green；同篇專案Refactor獨立段落與其餘作業另計。這是同Day修訂，next_day維持39。

從未改過的來源使用完整patch；若先手動加入Red測試，依Green正文補程式，不重複套入已含測試的patch。來源有本地變更先審查差異，不覆寫。完整patch的SHA256：`89e2bd9fe90c59c6949ce952aa39787eea51093d759f01ee725e45e7be71969f`。

規則精確來源：copilot_use `92913e7aae67b317fe25dab75f6ec56721042d35`，共用規則blob `492b50b2e16f1d8997ab55028bfc3c4d072af363`；專案Refactor與②Red／Green並列，功能程式與patch沿用r2精確來源。

## ① 功能說明與上一 Day 缺口

[Day37 r3](../day-37/analytics-r3.md) 已將**人員 Session Cookie** 的歧義拒絕接到兩種 summary Router；最新來源仍保留此修補，不重教、不重新宣稱已驗收。另一個**獨立的機器入口** `analytics_apps/analytics_server/src/lib.rs::ingest` 目前用 `headers.get(AUTHORIZATION)` 取第一個值，`IngestTokens::authorized` 僅比對那個字串。若同一請求帶兩條 `Authorization`，第一條合法就可能被當成合法事件；代理、下游與稽核系統對多值解讀可能不同。本日收斂 HTTP 邊界：**必須恰有一條可解析 Authorization，且其 Bearer token 與事件 source_app 相符**。未授權請求不得寫入 `product_events`；合法單一 Chat／Pet 來源仍可各自上報。

正式鏈：Chat／Pet 機器客戶端 → `POST /api/v1/events` → `analytics_server::ingest`（Axum HTTP／header）→ `IngestTokens::authorized` → `pg_store::persist_event`（既有 SQLx PostgreSQL transaction／冪等）→ 201/200/409 ACK。此路由是機器上報，**不能要求人員 Cookie**；`/api/v1/events/{source}/summary` 則繼續使用 Day37 的人員 Session。Leptos Dashboard、Application／Domain 空範本不因本篇變成已接通；未實作的短效服務憑證／配對／輪替、管理員與檢視者角色／來源 ACL 仍待完成。

## 專案 Refactor

本節在同篇與②Red／Green並列，按Analytics整體架構、共用依賴、測試與CI的累積進度每日調整；不是Green完成後才開始的步驟，也不限定於當日功能切片。另計時間，不增加Day。

### 專案目標與累積現況

| 工程責任 | 需銜接的邊界 |
| --- | --- |
| 帳號與Session | bootstrap／登入／改密碼／撤銷用例、身份與Session Port，正式Router與Leptos接線 |
| 查詢與權限 | 統計查詢／角色／來源ACL用例、DTO與授權責任，PG Adapter不承擔HTTP決策 |
| 攝取與事件 | 機器憑證／事件驗證／冪等與持久化邊界，與人員Session分開 |
| 測試與PG CI | 隔離PG fixture、正式Router／Adapter與用例測試，功能斷言與準備流程分離；獨立PG選取 |
| 共用依賴／組裝 | 檢視四workspace公開契約、shared、Cargo normal／build／dev／target依賴、features與正式caller；先承接再清舊碼 |

先完成專案caller／Port／Adapter與斷言基線，再依相依選完整包。新功能與未完成需求依FUTURE_WORK另追蹤，不能把表內規劃寫成已存在或已重構。

### 既有測試遷移基線（專案清單的一部分）

| 舊測試／群組 | 判斷 | 處理與必留斷言 | 目標責任 |
| --- | --- | --- | --- |
| day37_real_query_routes_reject_ambiguous_session_cookies | 正式兩路由共同契約 | 按路由×Cookie情境參數化；不得刪一條路由覆蓋 | 正式Router＋真PG |
| first_admin_login_has_persistent_restricted_session | 首次登入／Session持久化契約 | 與Day37查詢不同，保留restricted／改密碼跳轉與DB斷言 | 正式Router＋真PG |
| 各PG測試重複bootstrap／Router／login準備 | fixture重複候選 | 可抽測試support；每測試自有DB與情境斷言，initialize仍有fixture用途 | analytics-server tests/support（擬議） |

來源 `rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9`。此表是來源核對後的擬議映射，非已遷移清單；完整跨案對照見 [測試覆蓋表](test-coverage-map.csv)。同層同契約才整併；跨層保留接線、SQL、HTTP／IPC或DOM獨有的驗證責任。fixture抽共用準備，不複製業務實作或將情境斷言藏入共用helper。



### 本次完整調整：CI責任隔離

原Persistence workflow將Chat→Pet server/native→Analytics PG串在同一job，前段故障會使後段不執行。analytics-postgres同時保留analytics-server與analytics-persistence並明選postgres-integration，真PG／Router不因Pet原生故障被略過。這不等於角色／來源ACL或Application／Domain已完成。

下面是取代產品來源 `.github/workflows/persistence.yml` 的完整YAML，三案共用同一份內容及證據。本r3把先前已交付的方案納入同篇獨立段落，不重複計為新增工程成果；其餘Application／Domain／Adapter的完整重構仍列待辦。

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

### 測試／CI承接與套用條件

| 原選取 | 新責任入口 | 必留範圍 |
| --- | --- | --- |
| chat-persistence | chat-sqlite | SQLite Adapter原斷言全部保留 |
| pet-persistence | pet-sqlite | SQLite Adapter原斷言全部保留，不帶Tauri |
| pet-server | pet-native | 原server／IPC與native測試保留，GUI另驗 |
| analytics-server＋analytics-persistence | analytics-postgres | postgres-integration、真PG與正式Router／資料層測試 |

四job無互相needs、無continue-on-error，不刪測試換全綠。若required checks仍指定舊test-persistence，使用者同步調整新job名稱；本次未改分支規則。PG18是擬議服務major，實際minor及DB結果須套用後讀回。其它workflow保留；純內層／HTTP／WS／IPC／UI覆蓋仍須完整基線核對，不能從此workflow推論全專案覆蓋完整。

### 實際驗證、回復與下一步

方案來源為 `9b89656f964b99e0cb89ba421855704a35c5ecb9`。原方案已通過精確來源的patch套用檢查與YAML解析；產品未套用，Actions、DB、native與GUI未執行。本r3未新增執行或套用結果。

使用者套用後，以隔離分支的Pet native可控失敗確認其餘job仍執行且整體失敗，恢復後重跑全部原斷言。回復時還原該workflow及相應required checks，不回退其它產品變更。

下一步補齊帳號／Session、查詢／ACL、機器攝取／憑證、PG與Leptos的caller、Port／Adapter、有效斷言及CI選取基線，再依真相依提供完整用例與測試遷移；前案未套用不得假設已存在。這些未完成項不能由本篇Green或CI方案代替，也不新增FIFO或更新last_full_comparison。

[原完整差異patch](../../../engineering/2026-10-26/ci-isolation.patch) · [原驗證設計](../../../engineering/2026-10-26/ci-isolation.md) · [專案盤點](../../functional-refactoring/analytics.md) · [跨日累積進度](../../functional-refactoring/PROGRESS.md) · [r2原文](analytics-r2.md)。

## ② 功能實作：Red → 完整 Green

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

同檔 `#[cfg(test)] async fn source_summary` 的原 `headers.get(AUTHORIZATION)` 解析段也換成同一行 `let authorization = single_authorization(&headers).unwrap_or("");`，避免測試 helper 持續維護較寬鬆的 Header 契約；該 helper **不是正式 query 路由**，仍僅 `cfg(test)`，不能藉它宣稱業務查詢可用機器 token。這是完整Green所需的共用解析，無新增對外API／第二套認證。

**驗證順序**：先判斷事件 schema／source/type（原 `valid_event`），再拒絕 0 或 >1 個 Authorization，才允許 SQLx 寫入。單值 token 的固定長度比較原樣保留；拒絕多值不是短效憑證／防重播的完整方案。

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

沿用r2的靜態驗證：在相同精確來源快照，`git apply --check`通過；完整修改檔以rustfmt解析通過。本r3重組正文，未重跑以上檢查或任何產品測試。[逐檔來源blob與結果](r2-static-checks.json)。這兩項不代表編譯、Red失敗、Green通過、DB、native、GUI或CI；上述仍未執行。產品倉庫未套用本候選內容。
