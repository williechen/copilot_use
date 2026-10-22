## ① 學習大綱／來源現況

Analytics Day 26。來源 rust\_ai\_chat/main <code>aaa9194a966aefae1ef4724f5d2b576b4289c550</code>，教學分支 <code>c0777e681e3ddbf4a67926734f592c6b52c2ee8d</code>，規則 daily-tdd-minimum-v1。既有 <code>password_login.rs::login</code> 會 INSERT <code>uac_login_session</code>，<code>session()</code>、<code>change_page()</code>、<code>dashboard()</code> 會讀取或刷新它。但這次讀回的五個 <code>apps/stats_server/migrations</code> 中未找到此表 DDL，<code>20261020154237_password_login_session.sql</code> 只有一個分號。現有 PostgreSQL 31 test 通過的歷史敘述與此靜態檢查並不一致，需在真正資料庫核對，不將任何一者當成驗收。本日選最早影響實際登入流程的 Session schema 契約，不更動封存文章或既有 migration。

## ② Red／完整最小 Green／Refactor

Red：對全新空的 PostgreSQL 執行現有 bootstrap → POST /login → GET /change-password。若 Session 表不存在，預期 POST 登入無法儲存 Session；這是待執行的行為預期，不是本次已測 Red。

Green：新增 <code>apps/stats_server/migrations/20261022070000_session_contract.sql</code> 全文：

```sql
CREATE TABLE IF NOT EXISTS uac_login_session (
    token_hash TEXT PRIMARY KEY,
    user_id VARCHAR(40) NOT NULL REFERENCES uac_user(id) ON DELETE CASCADE,
    credential_fingerprint TEXT NOT NULL,
    restricted BOOLEAN NOT NULL,
    expires_at BIGINT NOT NULL,
    CONSTRAINT session_hash_length CHECK (char_length(token_hash) = 64),
    CONSTRAINT session_fingerprint_length CHECK (char_length(credential_fingerprint) = 64)
);
CREATE INDEX IF NOT EXISTS idx_uac_login_session_user ON uac_login_session(user_id);
CREATE INDEX IF NOT EXISTS idx_uac_login_session_expiry ON uac_login_session(expires_at);
```

不改舊 SQL 與 checksum，不刪資料。若真實環境已存在表但 schema 不同，先比對欄位／約束與歷史 checksum，再決定版本化升級；<code>IF NOT EXISTS</code> 不是現存資料相容性驗證。

新增 <code>apps/stats_server/tests/day26_session_contract.rs</code> 完整測試：

```rs
#![cfg(feature = "postgres-integration")]
use axum::{body::Body,http::{header,Request,StatusCode}};
use stats_server::{bootstrap,password_login,IngestState,IngestTokens};
use std::sync::Arc;
use tower::ServiceExt;

#[sqlx::test(migrations = "./migrations")]
async fn first_admin_login_has_persistent_restricted_session(pool: sqlx::PgPool) {
    bootstrap::initialize(&pool, "lesson-test-password".into()).await.unwrap();
    let app = password_login::build(IngestState {
        pool:pool.clone(),
        tokens:Arc::new(IngestTokens{chat:"test-chat".into(),pet:"test-pet".into()})
    });
    let login = app.clone().oneshot(Request::post("/login")
        .header(header::ORIGIN,"http://127.0.0.1:3100")
        .header(header::CONTENT_TYPE,"application/x-www-form-urlencoded")
        .body(Body::from("username=admin&password=lesson-test-password")).unwrap())
        .await.unwrap();
    assert_eq!(login.status(),StatusCode::SEE_OTHER);
    assert_eq!(login.headers()[header::LOCATION],"/change-password");
    let cookie = login.headers()[header::SET_COOKIE].to_str().unwrap()
        .split(';').next().unwrap().to_owned();
    let count:i64 = sqlx::query_scalar("SELECT count(*) FROM uac_login_session")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(count,1);
    let change = app.clone().oneshot(Request::get("/change-password")
        .header(header::COOKIE,&cookie).body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(change.status(),StatusCode::OK);
    let dashboard = app.oneshot(Request::get("/dashboard")
        .header(header::COOKIE,&cookie).body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(dashboard.status(),StatusCode::SEE_OTHER);
    assert_eq!(dashboard.headers()[header::LOCATION],"/change-password");
}
```

Refactor：Session 的 Cookie、token、policy 後續以共用 Port 抽取，Analytics 維持 PostgreSQL Adapter、Chat 維持 SQLite Adapter；不得共享 Session 狀態。Analytics Day24 補教已完成，不重新回填。此篇尚未宣稱完整 role／source ACL。

## ③ Cargo／套件版本

workspace v0.2.25、edition 2024；sqlx 0.9（postgres/macros/runtime-tokio）、Axum 0.8、Tokio 1.54、argon2 0.6。無新增 Rust 套件。歷史證據採 PostgreSQL 18.6，但本次未連 DB 或核實 lock 精確解析版本／MSRV。

## ④ 命令、預期與實際驗證

<code>DATABASE_URL=<isolated-postgres> cargo test -p stats-server --features postgres-integration --test day26_session_contract</code>：預期真實登入 303、Session 資料持久、首次改密頁 200、未改密後台重新導向；本次未執行。<br /><code>DATABASE_URL=<isolated-postgres> cargo test -p stats-server --features postgres-integration -- --include-ignored</code>：要求完整 PostgreSQL target；本次未執行。<br />另須查 <code>_sqlx_migrations</code> 歷史 checksum、PostgreSQL catalog 表結構與事件列數前後快照；本次未執行，未修改資料庫。<br />來源 CI：Rust Dashboard Run 39651403341、SHA <code>aaa9194a966aefae1ef4724f5d2b576b4289c550</code> completed/success，未包含本篇尚未套用的 migration，不能當新測試綠燈。角色／來源 ACL、服務配對、資料保留升級、真正後台操作仍待完成；教學不等於產品驗收。
