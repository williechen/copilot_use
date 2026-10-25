# Analytics Day36 r1｜精確事件統計 API 改用人員 Session，不再接受來源 token 查詢

## ① 學習大綱：修復真實查詢路由的授權邊界

**2026-10-25；文件 `williechen/copilot_use/main`；唯一程式 `williechen/rust_ai_ring/main@61ed5930bd9ff38e0d5822eea2983121001d8422`；即時 `next_day.analytics=36`；教學已交付／程式未套用／未測試／未驗收。** 銜接 [Day35 r2 精確類型 SQLx adapter 與路由](../day-35/analytics-r2.md)。其後 `analytics_persistence::summary_for_source_and_type` 已接通，但 `analytics_server/src/lib.rs::source_kind_summary` 的正式 `GET /api/v1/events/{source}/summary/{event_type}` 仍使用 `Authorization: Bearer <來源 token>`，與已確認「所有業務查詢須有**人員 Session**，服務憑證僅用於 ingestion」衝突。這不是只改 HTTP 狀態碼的測試。

真實鏈：`analytics_server/src/main.rs` → `password_login::build` 路由 → `source_kind_summary` → `analytics_persistence` PostgreSQL；管理者 `/login`、`/change-password`、`/dashboard` 和未篩選 `/api/v1/events/{source}/summary` 已使用 `password_login::session`（DB 檢查 token_hash、期限、帳號 active、密碼 fingerprint、首次改密碼限制、活動續期），但**精確類型路由繞過**。本篇重用同一 Session 及 Cookie 續期邏輯，拒絕來源 token 單獨查詢；**保留** `POST /api/v1/events` 機器來源 token 的現行攝取契約，避免破壞自動上報。

本日只有 admin 帳號已接通；角色／來源授權尚未完成，不宣稱檢視者 ACL 已實作。未登入 401、首次改密碼未完成 403、有效管理員 Session 200、登出／密碼變更／停用後 401；失敗時不可查 DB 或回傳統計。保留 Leptos SSR Dashboard、SQLx 精確聚合、來源參數校驗與錯誤契約。

## ② Red → 完整最小 Green → Refactor

### Red：正式路由用來源 token 竟能讀統計

先在 `analytics_apps/analytics_server/src/lib.rs` 原 `day35_live_router_exposes_exact_kind_with_source_acl` 測試中，把成功請求改成**只有來源 Bearer token 時必須 401**：

~~~rust
// 原測試以 Bearer fixture-chat-only 期待 200；Day36 Red 先改為：
let source_token_only = app.clone().oneshot(
    Request::get("/api/v1/events/web_chat/summary/chat.message.persisted")
        .header(AUTHORIZATION, "Bearer fixture-chat-only")
        .body(Body::empty()).unwrap()
).await.unwrap();
assert_eq!(source_token_only.status(), StatusCode::UNAUTHORIZED);
~~~

現行來源這項 Red **行為失敗**（實際原路由會 200），不是缺符號。再加測「有效人員 Cookie 才能讀」、「首次改密碼限制」、「登出後撤銷」，原程式沒有 Session 檢查，同樣不能通過。不要以 `#[cfg(test)] source_summary` 的舊機器 token helper 代替正式路由。

### Green A：只共用既有人員 Session，不複製 PostgreSQL 認證 SQL

`analytics_apps/analytics_server/src/password_login.rs` 的 `fn renewed` 改為 `pub(crate) fn renewed`（**只改可見性，原函式內容、Cookie SameSite/HttpOnly/Max-Age 完全不變**）。在同檔 `session` 結束後、`change_page` 前新增：

~~~rust
/// 已完成首次改密碼的有效管理者，才可讀取業務統計。
/// 角色與來源 ACL 尚未建置，不得冒稱檢視者已獲授權。
pub(crate) async fn require_query_session(
    state: &IngestState,
    headers: &HeaderMap,
) -> Result<String, StatusCode> {
    let (restricted, raw) = session(state, headers).await?;
    if restricted {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(raw)
}
~~~

此函式只使用既有 `session`：持久化驗證、過期、密碼變更與帳號停用都由它處理；`restricted` 不等於授權成功。沒有新增另一個管理者 token 表、SQL、前端密鑰或 API。

### Green B：**正式** exact-kind handler 改用 Cookie Session

`analytics_apps/analytics_server/src/lib.rs` 原 `async fn source_kind_summary`（原第 185–208 行）**完整替換**為：

~~~rust
async fn source_kind_summary(
    State(state): State<IngestState>,
    axum::extract::Path((source, event_type)):
        axum::extract::Path<(String, String)>,
    headers: HeaderMap,
) -> Result<axum::response::Response, StatusCode> {
    // 先驗人員 Session，再驗來源／事件類型，最後才碰統計資料。
    let raw = password_login::require_query_session(&state, &headers).await?;
    if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
        return Err(StatusCode::BAD_REQUEST);
    }
    if !valid_event_type(&event_type) || event_type.contains('\0') {
        return Err(StatusCode::BAD_REQUEST);
    }
    let rows = summary_for_source_and_type(&state.pool, &source, &event_type)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(password_login::renewed(Json(rows), &raw))
}
~~~

**正式路由不需新增**：`password_login.rs::build` 原有
`.route("/api/v1/events/{source}/summary/{event_type}", get(crate::source_kind_summary))`
已連到此 handler；`/dashboard` 及 `/api/v1/events/{source}/summary` 仍共用 `session`。`POST /api/v1/events` 繼續走 `ingest` 來源 token，不可誤改成需要人員 Cookie。`analytics_persistence` 的 SQLx query、`EventSummary` 與 migration 完全不動。Rust module 名稱仍 `analytics_server`，Cargo package 為 `analytics-server`。

### Green C：同步修改真正會失敗的舊測試與 HTTP 整合測試

`analytics_apps/analytics_server/src/lib.rs` 的 `mod source_kind_summary_tests` 中，**原** `authorization_and_filter_validation_precede_database_access` 測試整個替換成以下函式；既有 `fn state(pool)`、`fn headers(token)` 可保留：

~~~rust
#[tokio::test]
async fn day36_source_token_alone_cannot_read_even_before_database_access() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://fixture:fixture@127.0.0.1:1/unused")
        .unwrap();
    pool.close().await;
    let state = state(pool);
    for (source, kind) in [
        ("web_chat", "chat.message.persisted"),
        ("desktop_pet", "chat.message.persisted"),
        ("unknown", "chat.message.persisted"),
        ("web_chat", " "),
    ] {
        let result = source_kind_summary(
            State(state.clone()),
            Path((source.into(), kind.into())),
            headers("fixture-chat-only"),
        ).await;
        assert_eq!(result.unwrap_err(), StatusCode::UNAUTHORIZED);
    }
}
~~~

同一 module 原 `filters_committed_deduplicated_rows_by_source_and_kind` 保留所有 SQLx insert／rollback／UUID 型別與後續斷言，**只把**原 `let state = state(pool.clone()); let Json(rows) = source_kind_summary(...).await.unwrap();` 這一段替換為：

~~~rust
let rows = analytics_persistence::summary_for_source_and_type(
    &pool, "web_chat", "chat.message.persisted"
).await.unwrap();
~~~

這項舊測試從此只驗**真正的 SQLx Adapter**，不再用來源 token 假裝通過人員 HTTP 授權；`rows` 仍為原 `Vec<EventSummary>`，後續 `assert_eq!(..., rows)` 不必修改。`source_summary_tests` 測試的 `#[cfg(test)]` 歷史 helper 非正式 route，本日不恢復舊機器查詢；若未來清理須先遷移有效斷言，不能誤刪 ingestion 的來源 token 測試。

`analytics_apps/analytics_server/src/lib.rs` **整個替換**原最下方 `day35_live_router_exposes_exact_kind_with_source_acl`（原第 689–744 行），改成正式 HTTP router + 真 PostgreSQL Session 的整合測試（保留原 Day35 文章與歷史證據）：

~~~rust
#[cfg(feature = "postgres-integration")]
#[sqlx::test(migrations = "../migrations")]
async fn day36_live_router_requires_person_session_not_source_token(pool: PgPool) {
    use axum::{body::{Body, to_bytes}, http::{Request, header}};
    use sha2::{Digest, Sha256};
    use tower::ServiceExt;

    let digest = |s: &str| Sha256::digest(s.as_bytes())
        .iter().map(|b| format!("{b:02x}")).collect::<String>();
    let user_id = crate::bootstrap::ensure_default(&pool).await.unwrap();
    // 只在隔離測試 DB 建立「已完成首次改密碼」的 Session fixture；
    // 正式修改密碼流程另由 day27/day34 HTTP 測試覆蓋。
    sqlx::query("UPDATE uac_user SET is_init_pd=FALSE WHERE id=$1")
        .bind(&user_id).execute(&pool).await.unwrap();
    let password_hash: String =
        sqlx::query_scalar("SELECT user_pd FROM uac_user WHERE id=$1")
            .bind(&user_id).fetch_one(&pool).await.unwrap();
    let raw = "a".repeat(64);
    sqlx::query(
        "INSERT INTO uac_login_session
         (token_hash,user_id,credential_fingerprint,restricted,expires_at)
         VALUES ($1,$2,$3,FALSE,floor(extract(epoch from now()))::bigint+1800)"
    ).bind(digest(&raw)).bind(&user_id).bind(digest(&password_hash))
        .execute(&pool).await.unwrap();

    let event = EventInput {
        schema_version: 1,
        event_id: Uuid::new_v4(),
        source_app: "web_chat".into(),
        event_type: "chat.message.persisted".into(),
    };
    assert_eq!(pg_store::persist_event(&pool, &event).await.unwrap(),
               pg_store::PersistOutcome::Stored);
    let app = crate::build_app(IngestState {
        pool: pool.clone(),
        tokens: Arc::new(IngestTokens {
            chat: "fixture-chat-only".into(),
            pet: "fixture-pet-only".into(),
        }),
    });
    let path = "/api/v1/events/web_chat/summary/chat.message.persisted";
    for token in ["fixture-chat-only", "fixture-pet-only"] {
        let denied = app.clone().oneshot(
            Request::get(path)
                .header(AUTHORIZATION, format!("Bearer {token}"))
                .body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(denied.status(), StatusCode::UNAUTHORIZED);
    }
    let cookie = format!("analytics_session={raw}");
    let ok = app.clone().oneshot(
        Request::get(path).header(header::COOKIE, &cookie)
            .body(Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(ok.status(), StatusCode::OK);
    assert!(ok.headers().contains_key(header::SET_COOKIE)); // 活動續期
    let body = to_bytes(ok.into_body(), 8192).await.unwrap();
    let rows: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["source_app"], "web_chat");
    assert_eq!(rows[0]["event_type"], "chat.message.persisted");
    assert_eq!(rows[0]["total"], 1);

    // 同一已登入人員仍需通過事件類型校驗。
    let bad = app.clone().oneshot(
        Request::get("/api/v1/events/web_chat/summary/%20")
            .header(header::COOKIE, &cookie)
            .body(Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(bad.status(), StatusCode::BAD_REQUEST);

    // 登出/密碼變更/停用都由相同 Session adapter 拒絕。
    sqlx::query("DELETE FROM uac_login_session WHERE token_hash=$1")
        .bind(digest(&raw)).execute(&pool).await.unwrap();
    let revoked = app.oneshot(
        Request::get(path).header(header::COOKIE, &cookie)
            .body(Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(revoked.status(), StatusCode::UNAUTHORIZED);
}
~~~

上述測試以隔離 PG 測 Session + HTTP + SQLx，未測瀏覽器、真登入表單與角色來源 ACL。若測試環境對 URL 百分號解碼有不同處理，應保留 handler 層的無效類型斷言並核對 400，不可把不合法輸入放行；不要因 404/400 差異掩蓋權限問題。

### Refactor、Cargo 與上一篇／舊 Day 補充

`password_login::require_query_session` 是既有 `session` 的小型可重用 guard；`source_kind_summary` 只負責查詢參數、SQLx Adapter、JSON 回應與續期 Cookie。`analytics_apps/Cargo.toml` 的 `axum 0.8`、`sqlx 0.9/postgres`、`tokio 1.54`、`argon2 0.6`、`sha2 0.11` 及 `analytics_server/Cargo.toml` 的 `analytics-persistence`／`postgres-integration` 已具備；**沒有新增 Cargo dependency／feature、Cargo.lock、Leptos metadata、migration 或 UI 變更**。`analytics_ui` 仍未承接正式獨立互動頁面，不能因 SSR renderer 存在就宣稱 CSR/hydration 完成。`pet-application`／`pet-persistence` 與本案無關。

Day35 r2 已將精確類型查詢接正式路由，原 r1/r2 正文與 blob 保留；本篇修正其來源 token 授權缺口，不能覆寫原 Day。Day34 管理員首次登入／改密碼及 `ensure_default` 已在來源，測試 fixture 不等於新的登入實作。Day24 FIFO 原已結案不重算。後續仍須角色與來源 ACL、SSE 連線撤銷、CSRF、服務配對／輪替、明細及 gRPC；本篇**不宣稱**這些已完成。

## ③ 套件與軟體版本（2026-10-25 官方最新穩定）

| 項目 | manifest → Cargo.lock（61ed5930） | 官方最新穩定版／判斷 |
| --- | --- | --- |
| Rust/Cargo | 四 workspace toolchain stable，實際安裝未量測 | [Rust 1.99.1（10/15）](https://blog.rust-lang.org/releases/)；edition 2024 最低 1.85 |
| Axum | 0.8/ws → **0.8.9** | [axum 0.8.9](https://docs.rs/crate/axum/latest)；保留 Router/Response |
| SQLx PostgreSQL | 0.9/runtime-tokio/macros/postgres → **0.9.0** | [SQLx 0.9.0](https://docs.rs/crate/sqlx/latest)；原 PG adapter |
| Argon2 / SHA2 | 0.6 → **0.6.0**；0.11 → **0.11.1** | [Argon2](https://docs.rs/crate/argon2/latest)／[SHA2](https://docs.rs/crate/sha2/latest)；Session 仍沿用既有雜湊 |
| Tokio / Serde / UUID | 1.54 / 1.0 / 1.29 → **1.54.1 / 1.0.229 / 1.29.0** | [Tokio](https://docs.rs/crate/tokio/latest)／[Serde](https://docs.rs/crate/serde/latest)／[UUID](https://docs.rs/crate/uuid/latest) |
| Leptos SSR | 0.8 → **0.8.22** | [Leptos](https://docs.rs/crate/leptos/latest)，本日不修改 UI |
| PostgreSQL server | 實際 runtime 未量測 | [18.6（2026-08-13）](https://www.postgresql.org/docs/release/18.6/) 為官方穩定；19 beta/RC 不列入 |

四份 lock blob：Chat `384bcb23a13bc0f4e876f29e3037f2181ebbc3d5`、Pet `7097f943da5c88be851979cec74641fa35e6e613`、Analytics `44abb16f1ec3f74fa181c69d46e8bd984a090f21`、Crates `c5ec8079981d59244ce5482258424cd8ff57f9a3`。上述為交付當日官網／docs.rs 的最新穩定核對；不是本機安裝／升級證明。`SHOW server_version` 與完整 PG schema/migration checksums 須由使用者實測；Rust 2024 MSRV 與 native DB 相容組合尚未執行，維持現有 lock。

## ④ 驗證：完整命令、預期／實際與缺口

使用**隔離 PostgreSQL**、SQLx `#[sqlx::test]` 所需的建庫權限，於使用者產品庫執行（勿對正式 DB）：

~~~bash
cd rust_ai_ring/analytics_apps
cargo metadata --locked --format-version 1 --no-deps
# Red：現行來源 token-only 回 200，預期 401（測試行為失敗）
cargo test --locked -p analytics-server --features postgres-integration day36_
cargo test --locked -p analytics-server --features postgres-integration source_kind_summary_tests
cargo test --locked -p analytics-server --features postgres-integration
cargo test --locked -p analytics-server --all-targets
cargo check --locked -p analytics-server --all-targets
cargo fmt --all -- --check
# PostgreSQL：SHOW server_version; SELECT version FROM _sqlx_migrations;
~~~

Green 預期：只有來源 token 的 exact-kind GET 回 401；有效管理者 Cookie 回 200、JSON 精確總數、`Set-Cookie` 續期；首次強制改密碼時 403，登出／停用／密碼變更或過期後 401；錯誤來源／類型 400，PG 失敗 503；`POST /api/v1/events` 仍可用正確機器 token 攝取並去重。另在真瀏覽器以登入→改密碼→Dashboard→精確 API→登出重試驗證，不得以來源 token 冒充管理員。

**實際**：本回合只在 copilot_use 寫教學；`rust_ai_ring` 程式未套用，新 Red／Green、PostgreSQL、HTTP、SSR、瀏覽器與 CI 均**未執行**，Gate 2 **未驗收**。角色／來源授權、正式 Session 與 ingestion 機器 token 的分離仍需完整端到端驗證；不更新 `last_full_comparison`。