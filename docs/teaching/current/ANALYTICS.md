# 2026-10-15｜P0 Analytics Day 12｜HTTP 來源驗證與 PostgreSQL 持久化接線

來源文章：[2026-10-15｜P0 Analytics Day 12｜HTTP 來源驗證與 PostgreSQL 持久化接線](https://app.notion.com/p/3fae51d4509381ebb1e9f8482b5bf44e?pvs=204)。

## 1. 學習大綱
- P0 Analytics Day 12（約 25 分鐘）：補齊 **HTTP JSON 攝取 → 來源驗證 → PostgreSQL 持久化 → ACK** 的最小實際呼叫路徑。Day 11 已有 `pg_store::persist_event`，但 `ingest` 對有效事件仍回 `501`；本日不重建 EventRepository 或 HashSet 假資料層。
- 來源：`williechen/rust_ai_chat@5a0a8cee0d1fb4d0cd248869d7b24ebb1d653399`：`apps/stats_server/src/lib.rs`、`pg_store.rs`、`main.rs`、`Cargo.toml`。對照 `copilot_use/docs/FUTURE_WORK.md` 的「Analytics 參考 501／記憶體 fake 與 PostgreSQL／來源認證／gRPC 銜接」。此切片是保留的**HTTP 相容入口**，不是宣稱 gRPC／Protobuf 或 SSE 已完成。
- 目標：無授權不寫 DB、source 與憑證綁定、有效事件 commit 後 ACK、重送可辨識、identity conflict 不回成功、DB 故障不回成功。保持 loopback 綁定。
## 2. 實作內容
### Red：先定義失敗案例
在 `apps/stats_server/src/lib.rs` 原有 `mod tests` 內追加：
```rust
#[test]
fn token_must_match_the_claimed_source() {
    let tokens = IngestTokens {
        chat: "chat-secret".into(),
        pet: "pet-secret".into(),
    };
    assert!(tokens.authorized("web_chat", "Bearer chat-secret"));
    assert!(tokens.authorized("desktop_pet", "Bearer pet-secret"));
    assert!(!tokens.authorized("desktop_pet", "Bearer chat-secret"));
    assert!(!tokens.authorized("web_chat", "Bearer "));
    assert!(!tokens.authorized("web_chat", "chat-secret"));
}
#[test]
fn persist_outcomes_have_distinct_http_codes() {
    use pg_store::PersistOutcome::*;
    assert_eq!(outcome_status(Stored), StatusCode::CREATED);
    assert_eq!(outcome_status(AlreadyExists), StatusCode::OK);
    assert_eq!(outcome_status(IdentityConflict), StatusCode::CONFLICT);
}
```
預期 Red：`IngestTokens`／`outcome_status` 尚不存在。
### Green：實際連接原有 PostgreSQL Adapter
在 `apps/stats_server/src/lib.rs` 保留既有 `EventInput`、`valid_event`、`pg_store` 及既有單元測試；新增以下 import：
```rust
use axum::{
    extract::State,
    http::{HeaderMap, header::AUTHORIZATION},
};
use sqlx::PgPool;
use std::sync::Arc;
use pg_store::{PersistError, PersistOutcome};
```
新增狀態、授權及回應映射：
```rust
#[derive(Clone)]
pub struct IngestTokens {
    pub chat: String,
    pub pet: String,
}
#[derive(Clone)]
pub struct IngestState {
    pub pool: PgPool,
    pub tokens: Arc<IngestTokens>,
}
impl IngestTokens {
    fn authorized(&self, source: &str, header: &str) -> bool {
        let token = match source {
            "web_chat" => &self.chat,
            "desktop_pet" => &self.pet,
            _ => return false,
        };
        // 不接受空憑證；固定長度逐位比較避免提早依內容退出。
        if token.is_empty() || token.len() > 512 { return false; }
        let expected = format!("Bearer {token}");
        let a = expected.as_bytes();
        let b = header.as_bytes();
        let mut diff = a.len() ^ b.len();
        for i in 0..a.len().max(b.len()) {
            diff |= usize::from(
                a.get(i).copied().unwrap_or(0) ^ b.get(i).copied().unwrap_or(0)
            );
        }
        diff == 0
    }
}
fn outcome_status(value: PersistOutcome) -> StatusCode {
    match value {
        PersistOutcome::Stored => StatusCode::CREATED,
        PersistOutcome::AlreadyExists => StatusCode::OK,
        PersistOutcome::IdentityConflict => StatusCode::CONFLICT,
    }
}
```
**替換** `ingest` 與 `build_app`，保留 `health`：
```rust
pub async fn ingest(
    State(state): State<IngestState>,
    headers: HeaderMap,
    Json(event): Json<EventInput>,
) -> StatusCode {
    if !valid_event(&event) { return StatusCode::BAD_REQUEST; }
    let authorization = headers.get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok()).unwrap_or("");
    if !state.tokens.authorized(&event.source_app, authorization) {
        return StatusCode::UNAUTHORIZED;
    }
    match pg_store::persist_event(&state.pool, &event).await {
        Ok(outcome) => outcome_status(outcome),
        Err(PersistError::Invalid) => StatusCode::BAD_REQUEST,
        Err(PersistError::Database(_)) => StatusCode::SERVICE_UNAVAILABLE,
    }
}
pub fn build_app(state: IngestState) -> Router {
    Router::new()
        .route("/healthz", get(health))
        .route("/api/v1/events", post(ingest))
        .with_state(state)
        .layer(DefaultBodyLimit::max(4096))
}
```
`apps/stats_server/src/main.rs` **完整替換**：
```rust
use stats_server::{IngestState, IngestTokens, build_app};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 啟動時 fail-closed；不得把密鑰值寫入日誌。
    let database_url = std::env::var("ANALYTICS_DATABASE_URL")?;
    let chat = std::env::var("ANALYTICS_CHAT_TOKEN")?;
    let pet = std::env::var("ANALYTICS_PET_TOKEN")?;
    if chat.is_empty() || pet.is_empty() || chat == pet
        || chat.len() > 512 || pet.len() > 512
    {
        return Err("來源憑證設定不合法".into());
    }
    let pool = sqlx::PgPool::connect(&database_url).await?;
    // Migration 由既有 DB 管理流程預先套用；不可修改已套用 migration。
    let app = build_app(IngestState {
        pool,
        tokens: Arc::new(IngestTokens { chat, pet }),
    });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3100").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```
真實呼叫路徑：`main` 建立 `PgPool`／來源憑證 → `build_app(state)` → `POST /api/v1/events` → `ingest` 驗證 → `pg_store::persist_event` → PostgreSQL `product_events` → 依 `PersistOutcome` 回應。有效事件只有 DB 完成後才回 201／200。
### Refactor／必要安全與功能限制
- 保留 `pg_store` 原 SQL 和資料庫唯一鍵，不使用 `InMemoryDeduplicator` 代替持久冪等；不將 SQLx 放進 Domain。
- 來源憑證從**執行時環境**注入，教學／測試不讀真實密鑰、不印出值。上述手寫位元比較僅供本地最小切片，**尚未經密碼學安全審查**；正式安全 Gate 應替換為經審查的 constant-time 驗證或受信任本地身分機制，並驗證 header／token 長度上限與憑證輪替。
- `source_app` 與所選 token 綁定；不接受只有合法 JSON 就寫入。Loopback 限制不等於來源認證，亦不代表可公開部署。
- PostgreSQL 冪等查詢與寫入併發、故障重試、逐事件 ACK、gRPC／Protobuf、HTTP Query／SSE 均須後續測試／切片；不因本次 HTTP Adapter 存在就標記完成。
## 3. 套件版本
- 沿用 `apps/stats_server/Cargo.toml` 已宣告的 `axum`、`sqlx`、`tokio`、`serde`、`uuid`；不新增依賴。
- Cargo.lock 精確版本、官方 stable／MSRV／安全公告本次**未重新查證**。
## 4. 驗證項目
- 建議：`cargo test -p stats-server`、`cargo test -p stats-server --features postgres-integration`（需隔離測試 DB）、`cargo fmt --all -- --check`、`cargo clippy -p stats-server --all-targets -- -D warnings`。
- HTTP＋DB E2E 必測：無 token 401、跨來源 token 401、無效 payload 400、首次 201、相同 ID 相同內容 200、相同 ID 不同內容 409、DB 不可用 503 且無成功 ACK、重啟後仍去重、同 ID 並發及來源隔離。
- 必須在測試前建立獨立 PostgreSQL、套用既有 migrations；不得用 HashSet 成功取代 DB 測試。gRPC／Protobuf 與 SSE 尚未涵蓋。
- **本次沒有套用程式或執行新增測試／DB E2E；只驗證來源 SHA 的既有 CI Success**。功能正式驗收：未完成。
### 來源驗證同步（2026-10-15）
先完成本地來源驗證，再核對相同 SHA `5a0a8cee0d1fb4d0cd248869d7b24ebb1d653399` 的 GitHub CI。本地 102 項 Rust 測試（含真實隔離 PostgreSQL）及 3 項 Node 測試通過，SSR／WASM 檢查、Linux 原生／Chat Leptos／Pet 前端 release 建置通過；實際 HTTP／WS／SQLite 檢查通過。嚴格 Clippy **未通過**：`apps/desktop_pet/src/file_organizer.rs:309` 的 `expect(dead_code)` 已失效；GitHub 6／6 workflows、18／18 checks 成功，不包含這項嚴格 Clippy Gate。[精確來源、命令與限制](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/source-review/rust-ai-chat-validation.json)。
本次只驗證來源既有程式；本 Day 新增功能及測試仍尚待套用，未執行其 Green／GUI／功能 E2E，不將基線通過當成本 Day 功能驗收。未讀取 AI key，未執行付費 AI 呼叫。
