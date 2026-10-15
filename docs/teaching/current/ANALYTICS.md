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
### 實際套用補充（2026-10-15）
本地已提交 `a57e532c125f5f18d7fd2a250401efde0c9d2612`。HTTP state／來源 token／PostgreSQL 呼叫／回應映射與本日兩個測試均已套用，真實路徑已實測。`apps/stats_server/src/main.rs` 在 main 開頭另加入 `dotenv::dotenv().ok();`；`apps/stats_server/Cargo.toml` 與 `tests/Cargo.toml` 新增既有 workspace 的 dotenv 依賴，tests 同時加入 sqlx。精確檔案與完整差異見[實際提交](https://github.com/williechen/rust_ai_chat/commit/a57e532c125f5f18d7fd2a250401efde0c9d2612)，原教學程式區塊保留。
`tests/analytics/event_kind_tests.rs` 與 `http_contract_tests.rs` 已配合 `IngestState`／`build_app(state)` 更新：合法輸入但沒有 token 預期 401，舊測試名稱雖仍含 unimplemented／501，斷言已改為 401。執行 workspace 測試須先建立隔離 PostgreSQL，設定 `ANALYTICS_DATABASE_URL` 與兩個不同的合成來源 token；HTTP 實測前套用 `apps/stats_server/migrations`。本次未讀取私人 .env；現行 CI 尚缺上述 fixture，詳見第 4 節與[待套用的 CI 測試環境設定](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/source-review/DAY12_CI_ENVIRONMENT.md)。`persistence.yml` 的 env／service 不會傳到 `test.yml` 的 test-workspace job；此設定範例未由助手寫入學習倉庫。
## 3. 套件版本
- 沿用既有 `axum`、`sqlx`、`tokio`、`serde`、`uuid`；實際套用另在 stats-server／workspace-tests 加入既有 workspace 的 `dotenv`，Cargo.lock 精確版本為 `0.15.0`；workspace-tests 同時使用既有 `sqlx`。
- Cargo.lock 精確版本、官方 stable／MSRV／安全公告本次**未重新查證**。
## 4. 驗證項目
- 建議：`cargo test -p stats-server`、`cargo test -p stats-server --features postgres-integration`（需隔離測試 DB）、`cargo fmt --all -- --check`、`cargo clippy -p stats-server --all-targets -- -D warnings`。
- HTTP＋DB E2E 必測：無 token 401、跨來源 token 401、無效 payload 400、首次 201、相同 ID 相同內容 200、相同 ID 不同內容 409、DB 不可用 503 且無成功 ACK、重啟後仍去重、同 ID 並發及來源隔離。
- 必須在測試前建立獨立 PostgreSQL、套用既有 migrations；不得用 HashSet 成功取代 DB 測試。gRPC／Protobuf 與 SSE 尚未涵蓋。
- 程式與新增測試已套用；本地真實 HTTP／PostgreSQL 的 401／400／201／200／409／503、來源隔離、同 ID 併發與重啟去重通過。GitHub CI 另列，gRPC／SSE／正式安全與整體驗收未完成。
### 來源驗證最終判定（2026-10-15）
使用者已將本 Day 實作提交於 `a57e532c125f5f18d7fd2a250401efde0c9d2612`，驗證取樣時本地工作目錄乾淨；其後出現未提交的 `persistence.yml` 設定調整，未納入此 SHA 證據。先驗本地，再核對同 SHA GitHub。**本地 Linux 正式驗證通過；同 SHA GitHub CI 未通過。** 本地 107 項 Rust、3 項 Node、fmt、嚴格 Clippy、SSR／兩個 WASM、Linux 原生／Chat Leptos／Pet 前端 Release 均通過；14 項實際 HTTP／WS／DB 檢查通過，包含來源認證、PostgreSQL 寫入／重複／衝突、併發去重、重啟後去重、DB 中斷回 503，以及 Typing 房間隔離與不寫入訊息。
GitHub `Test All / test-workspace` 已失敗；[失敗工作與日誌](https://github.com/williechen/rust_ai_chat/actions/runs/38745289793/job/116457220577)顯示 `tests/analytics/event_kind_tests.rs:47、68` 因缺少 `ANALYTICS_DATABASE_URL` 出現 `NotPresent`。該工作也須提供 `ANALYTICS_CHAT_TOKEN`、`ANALYTICS_PET_TOKEN` 與獨立 PostgreSQL。本次本地已用合成 token／隔離 DB 通過；不沿用舊 SHA CI Success。其他未結束的 job 仍列執行中，精確查驗時間與計數見[可攜式驗證證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/source-review/rust-ai-chat-validation.json)。
原教學撰寫基底 `5a0a8ce` 與全部原程式區塊保留，實際套用／驗證來源更新為 `a57e532c125f5f18d7fd2a250401efde0c9d2612`。Analytics HTTP 已接上 PostgreSQL，不再固定回 501。Pet 實際 GUI 移動、跨平台安裝包、gRPC／SSE 銜接及正式憑證安全審查未由本次驗證完成；不宣稱全部 Day／P9／週六驗收完成。不讀取私人 .env／AI key、不執行付費 AI、不前移 Day／階段。
