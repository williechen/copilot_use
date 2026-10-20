發布版r1。來源 `rust_ai_chat/main f38360cd537daf6dc20e1d3868221bb56390cc7c`，教學基底 `03875483e5d6bad7f1bfaf25a2a26ec4658a186a`；規則daily-tdd-minimum-v1，Notion共同頁最後修改2026-10-20T15:10:18.060Z。Day22（含）以前封存不改，所有程式僅在隔離副本測試，由使用者另行套用。
## 1. 學習大綱／現況
目前 `bootstrap::initialize` 已把管理員與Argon2雜湊存進PostgreSQL，CLI也存在；但執行入口 `/login` 仍依 `ANALYTICS_ADMIN_KEY_SHA256` 驗固定64字元key，沒有查 `uac_user`。`is_init_pd=true` 只是標記，沒有真正把首次登入隔離。
本日接通「真實帳號密碼→驗證既有雜湊→PostgreSQL持久Session→首次登入只能看受限頁→登出」最小切片。已完成初始化的既有admin可讀最低限度Dashboard總數與來源summary。**首次密碼更換提交尚未提供**，新初始化帳號不能進報表；受限頁只有說明與登出，不放沒有作用的改密碼按鈕。
主體Red5分鐘／Green20分鐘，使用共用彈性10分鐘；Refactor與Day22補教5分鐘、版本與證據閱讀另估8分鐘。大流程跨日延續，不宣稱完整登入／強制改密碼／P0驗收完成。
資料流：同一3100 listener的HTML表單→新 `password_login::build` 路由→既有PgPool與 `uac_user`→blocking Argon2驗證→新增session表只存token digest與密碼雜湊指紋→HttpOnly cookie→每次查詢重驗有效帳號、密碼指紋與首次更換旗標。成功GET更新DB閒置期限並續發30分鐘cookie；登出要求相同Origin並刪DB Session。
新runtime與公開 `build_app` 都改呼叫新builder，不merge舊key登入；舊 `admin.rs` 保留為歷史程式與回歸測試，不代表新runtime仍接受key。新Dashboard暫時只呈現真實Chat總數，舊完整篩選畫面／kind路由未接回，不能以教學切片冒稱等價替換或直接部署。來源事件上報仍走既有machine token，不能用來源token登入人員後台。
`copilot_use`產品目前 `apps/stats_server/src/main.rs`／`lib.rs::AppState` 使用獨立 `ANALYTICS_ADMIN_TOKEN` 與治理API契約；不因名稱相似視為本篇已整合。前置只操作專用PostgreSQL測試DB，既有migration checksum有問題就停止核對，不repair／刪庫。
## 2. 實作內容：Red → 完整最小 Green → Refactor
### Red：帳密登入必須進入正確權限狀態
先加入以下 `apps/stats_server/tests/day23_password_login.rs`，執行 `cargo test --locked -p stats-server --features postgres-integration --test day23_password_login`。實際E0432：尚無 `stats_server::password_login` 模組，屬預期缺符號Red；不是把連線或匯入錯誤冒充行為失敗。SQLx測試使用專用 `DATABASE_URL`，新增migration後會由完整runner建測試DB。
```rust
#![cfg(feature = "postgres-integration")]
use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use stats_server::{IngestState, IngestTokens, password_login};
use std::sync::Arc;
use tower::ServiceExt;
fn app(pool: sqlx::PgPool) -> axum::Router {
    password_login::build(IngestState {
        pool,
        tokens: Arc::new(IngestTokens {
            chat: "fixture-chat".into(),
            pet: "fixture-pet".into(),
        }),
    })
}
async fn send(
    app: axum::Router,
    path: &str,
    body: Option<&str>,
    cookie: Option<&str>,
) -> axum::response::Response {
    let mut r = if body.is_some() {
        Request::post(path)
            .header("origin", "http://127.0.0.1:3100")
            .header("content-type", "application/x-www-form-urlencoded")
    } else {
        Request::get(path)
    };
    if let Some(c) = cookie {
        r = r.header(header::COOKIE, c);
    }
    app.oneshot(
        r.body(body.map_or_else(Body::empty, |s| Body::from(s.to_owned())))
            .unwrap(),
    )
    .await
    .unwrap()
}
fn cookie(r: &axum::response::Response) -> String {
    r.headers()[header::SET_COOKIE]
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_owned()
}
#[sqlx::test(migrations = "./migrations")]
async fn day23_bootstrapped_password_is_restricted_before_first_change(pool: sqlx::PgPool) {
    stats_server::bootstrap::initialize(&pool, "lesson-only-password".into())
        .await
        .unwrap();
    assert_eq!(
        send(
            app(pool.clone()),
            "/login",
            Some("username=admin&password=wrong"),
            None
        )
        .await
        .status(),
        StatusCode::UNAUTHORIZED
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM uac_login_session")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    let ok = send(
        app(pool.clone()),
        "/login",
        Some("username=ADMIN&password=lesson-only-password"),
        None,
    )
    .await;
    assert_eq!(ok.status(), StatusCode::SEE_OTHER);
    assert_eq!(ok.headers()[header::LOCATION], "/change-password");
    let c = cookie(&ok);
    assert_eq!(
        send(app(pool.clone()), "/dashboard", None, Some(&c))
            .await
            .headers()[header::LOCATION],
        "/change-password"
    );
    assert_eq!(
        send(
            app(pool.clone()),
            "/api/v1/events/web_chat/summary",
            None,
            Some(&c)
        )
        .await
        .status(),
        StatusCode::FORBIDDEN
    );
    let pending = send(app(pool.clone()), "/change-password", None, Some(&c)).await;
    assert_eq!(pending.status(), StatusCode::OK);
    let body = axum::body::to_bytes(pending.into_body(), 100000)
        .await
        .unwrap();
    assert!(String::from_utf8_lossy(&body).contains("尚未提供改密碼提交"));
    assert!(
        stats_server::bootstrap::initialize(&pool, "different-password".into())
            .await
            .is_err()
    );
    let flag: bool = sqlx::query_scalar("SELECT is_init_pd FROM uac_user")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(flag);
}
#[sqlx::test(migrations = "./migrations")]
async fn day23_ready_account_session_survives_new_pool_and_is_revocable(pool: sqlx::PgPool) {
    stats_server::bootstrap::initialize(&pool, "lesson-only-password".into())
        .await
        .unwrap();
    // 已完成首次密碼更換的既有帳號fixture；此SQL不是提供給使用者繞過首次改密碼的操作。
    sqlx::query("UPDATE uac_user SET is_init_pd=FALSE")
        .execute(&pool)
        .await
        .unwrap();
    let ok = send(
        app(pool.clone()),
        "/login",
        Some("username=admin&password=lesson-only-password"),
        None,
    )
    .await;
    assert_eq!(ok.status(), StatusCode::SEE_OTHER);
    assert_eq!(ok.headers()[header::LOCATION], "/dashboard");
    let set = ok.headers()[header::SET_COOKIE].to_str().unwrap();
    for attr in ["HttpOnly", "SameSite=Strict", "Max-Age=1800"] {
        assert!(set.contains(attr));
    }
    let c = cookie(&ok);
    let raw = c.split_once('=').unwrap().1;
    let stored: String = sqlx::query_scalar("SELECT token_hash FROM uac_login_session")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_ne!(stored, raw);
    assert_eq!(stored.len(), 64);
    let options = pool.connect_options();
    pool.close().await;
    let reopened = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect_with((*options).clone())
        .await
        .unwrap();
    let ok = send(app(reopened.clone()), "/dashboard", None, Some(&c)).await;
    assert_eq!(ok.status(), StatusCode::OK);
    assert!(ok.headers().contains_key(header::SET_COOKIE));
    assert_eq!(
        send(
            app(reopened.clone()),
            "/api/v1/events/web_chat/summary",
            None,
            Some(&c)
        )
        .await
        .status(),
        StatusCode::OK
    );
    assert_eq!(
        send(
            app(reopened.clone()),
            "/api/v1/events/web_chat/summary",
            None,
            None
        )
        .await
        .status(),
        StatusCode::UNAUTHORIZED
    );
    sqlx::query("UPDATE uac_user SET user_status='disabled'")
        .execute(&reopened)
        .await
        .unwrap();
    assert_eq!(
        send(
            app(reopened.clone()),
            "/api/v1/events/web_chat/summary",
            None,
            Some(&c)
        )
        .await
        .status(),
        StatusCode::UNAUTHORIZED
    );
    sqlx::query("UPDATE uac_user SET user_status='active'")
        .execute(&reopened)
        .await
        .unwrap();
    sqlx::query("UPDATE uac_login_session SET expires_at=0")
        .execute(&reopened)
        .await
        .unwrap();
    assert_eq!(
        send(
            app(reopened),
            "/api/v1/events/web_chat/summary",
            None,
            Some(&c)
        )
        .await
        .status(),
        StatusCode::UNAUTHORIZED
    );
}
```
### Green：完整可套用修改
以下為對精確main的完整patch，包含新route module、main與公開builder接線、新additive migration、CLI訊息、四項測試及舊schema唯讀fixtures。`git apply --check`已通過。若已先加入上方Red檔，不可重複新增；可以在乾淨隔離副本一次套完整patch。歷史fixture只用於測試，正式migrations沒有改寫。
```diff
diff --git a/apps/stats_server/src/password_login.rs b/apps/stats_server/src/password_login.rs
new file mode 100644
index 0000000..20143bd
--- /dev/null
+++ b/apps/stats_server/src/password_login.rs
@@ -0,0 +1,226 @@
+use crate::{IngestState, pg_store};
+use argon2::{Argon2, PasswordHash, PasswordVerifier};
+use axum::{
+    Form, Json, Router,
+    extract::{DefaultBodyLimit, Path, State},
+    http::{HeaderMap, StatusCode, header},
+    response::{Html, IntoResponse, Redirect, Response},
+    routing::{get, post},
+};
+use serde::Deserialize;
+use sha2::{Digest, Sha256};
+const ORIGIN: &str = "http://127.0.0.1:3100";
+fn digest(value: &str) -> String {
+    Sha256::digest(value.as_bytes())
+        .iter()
+        .map(|b| format!("{b:02x}"))
+        .collect()
+}
+fn cookie(raw: &str) -> String {
+    format!("analytics_session={raw}; HttpOnly; SameSite=Strict; Path=/; Max-Age=1800")
+}
+fn raw_cookie(headers: &HeaderMap) -> Option<String> {
+    let raw = headers
+        .get(header::COOKIE)?
+        .to_str()
+        .ok()?
+        .split(';')
+        .filter_map(|v| v.trim().split_once('='))
+        .find(|(k, _)| *k == "analytics_session")?
+        .1;
+    (raw.len() == 64 && raw.bytes().all(|c| c.is_ascii_hexdigit())).then(|| raw.to_owned())
+}
+fn renewed(response: impl IntoResponse, raw: &str) -> Response {
+    let mut response = response.into_response();
+    response.headers_mut().insert(
+        header::SET_COOKIE,
+        cookie(raw).parse().expect("validated hex token"),
+    );
+    response
+}
+#[derive(Deserialize)]
+struct Login {
+    username: String,
+    password: String,
+}
+async fn login_page() -> Html<&'static str> {
+    Html(
+        r#"<!doctype html><html lang="zh-Hant"><meta charset="utf-8"><h1>Analytics 帳密登入</h1><form method="post" action="/login"><label>帳號<input name="username" autocomplete="username" required></label><label>密碼<input type="password" name="password" autocomplete="current-password" required></label><button>登入</button></form></html>"#,
+    )
+}
+async fn login(
+    State(state): State<IngestState>,
+    headers: HeaderMap,
+    Form(form): Form<Login>,
+) -> Result<Response, StatusCode> {
+    if headers.get(header::ORIGIN).and_then(|h| h.to_str().ok()) != Some(ORIGIN) {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    if !form.username.trim().eq_ignore_ascii_case("admin")
+        || !(8..=128).contains(&form.password.len())
+    {
+        return Err(StatusCode::UNAUTHORIZED);
+    }
+    // Existing bootstrap only provisions admin. Reject duplicate case-folded identities rather than guessing one.
+    let found: Vec<(String,String,bool,String)> = sqlx::query_as("SELECT id,user_pd,is_init_pd,user_status FROM uac_user WHERE lower(user_acnt)='admin' LIMIT 2")
+        .fetch_all(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
+    if found.len() != 1 {
+        return Err(StatusCode::UNAUTHORIZED);
+    }
+    let (id, encoded, restricted, status) = found.into_iter().next().unwrap();
+    if status != "active" {
+        return Err(StatusCode::UNAUTHORIZED);
+    }
+    let fingerprint = digest(&encoded);
+    let verified = tokio::task::spawn_blocking(move || {
+        let parsed = PasswordHash::new(&encoded).map_err(|_| ())?;
+        Argon2::default()
+            .verify_password(form.password.as_bytes(), &parsed)
+            .map_err(|_| ())
+    })
+    .await
+    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    verified.map_err(|_| StatusCode::UNAUTHORIZED)?;
+    let raw = format!(
+        "{}{}",
+        uuid::Uuid::new_v4().simple(),
+        uuid::Uuid::new_v4().simple()
+    );
+    let mut tx = state
+        .pool
+        .begin()
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    sqlx::query("DELETE FROM uac_login_session WHERE expires_at <= floor(extract(epoch from now()))::bigint").execute(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
+    sqlx::query("INSERT INTO uac_login_session(token_hash,user_id,credential_fingerprint,restricted,expires_at) VALUES ($1,$2,$3,$4,floor(extract(epoch from now()))::bigint+1800)")
+        .bind(digest(&raw)).bind(id).bind(fingerprint).bind(restricted).execute(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
+    tx.commit()
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    Ok(renewed(
+        Redirect::to(if restricted {
+            "/change-password"
+        } else {
+            "/dashboard"
+        }),
+        &raw,
+    ))
+}
+// Recheck account status, mandatory-change state and password fingerprint on each protected request.
+async fn session(state: &IngestState, headers: &HeaderMap) -> Result<(bool, String), StatusCode> {
+    let raw = raw_cookie(headers).ok_or(StatusCode::UNAUTHORIZED)?;
+    let found:Option<(bool,bool,String,String,String)> = sqlx::query_as("SELECT s.restricted,u.is_init_pd,u.user_pd,s.credential_fingerprint,u.id FROM uac_login_session s JOIN uac_user u ON u.id=s.user_id WHERE s.token_hash=$1 AND s.expires_at>floor(extract(epoch from now()))::bigint AND u.user_status='active' AND lower(u.user_acnt)='admin'")
+        .bind(digest(&raw)).fetch_optional(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
+    let (restricted, must_change, current, fingerprint, id) =
+        found.ok_or(StatusCode::UNAUTHORIZED)?;
+    if digest(&current) != fingerprint {
+        return Err(StatusCode::UNAUTHORIZED);
+    }
+    let refreshed=sqlx::query("UPDATE uac_login_session SET expires_at=floor(extract(epoch from now()))::bigint+1800 WHERE token_hash=$1 AND expires_at>floor(extract(epoch from now()))::bigint AND EXISTS (SELECT 1 FROM uac_user WHERE id=$2 AND user_pd=$3 AND user_status='active')")
+        .bind(digest(&raw)).bind(id).bind(current).execute(&state.pool).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
+    if refreshed.rows_affected() != 1 {
+        return Err(StatusCode::UNAUTHORIZED);
+    }
+    Ok((restricted || must_change, raw))
+}
+async fn change_page(
+    State(state): State<IngestState>,
+    headers: HeaderMap,
+) -> Result<Response, StatusCode> {
+    let (restricted, raw) = session(&state, &headers).await?;
+    if !restricted {
+        return Ok(renewed(Redirect::to("/dashboard"), &raw));
+    }
+    Ok(renewed(
+        Html(
+            "<!doctype html><html lang=\"zh-Hant\"><meta charset=\"utf-8\"><h1>首次登入須更換密碼與Email</h1><p>帳密已驗證，後台查詢仍被拒絕。本切片尚未提供改密碼提交，請勿清除資料庫標記繞過限制。</p><form method=\"post\" action=\"/logout\"><button>登出</button></form></html>",
+        ),
+        &raw,
+    ))
+}
+async fn dashboard(State(state): State<IngestState>, headers: HeaderMap) -> Response {
+    let (restricted, raw) = match session(&state, &headers).await {
+        Ok(v) => v,
+        Err(StatusCode::UNAUTHORIZED) => return Redirect::to("/login").into_response(),
+        Err(e) => return e.into_response(),
+    };
+    if restricted {
+        return renewed(Redirect::to("/change-password"), &raw);
+    }
+    let rows = match pg_store::summary_for_source(&state.pool, "web_chat").await {
+        Ok(v) => v,
+        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
+    };
+    let Some(total) = crate::summary_total(&rows) else {
+        return StatusCode::SERVICE_UNAVAILABLE.into_response();
+    };
+    renewed(
+        Html(format!(
+            "<!doctype html><html lang=\"zh-Hant\"><meta charset=\"utf-8\"><h1>Analytics 管理後台</h1><p>已完成密碼初始化的管理員：Chat 已提交事件總數 {total}</p><p>詳細查詢畫面與管理操作將於後續切片接回。</p><form method=\"post\" action=\"/logout\"><button>登出</button></form></html>"
+        )),
+        &raw,
+    )
+}
+async fn summary(
+    State(state): State<IngestState>,
+    Path(source): Path<String>,
+    headers: HeaderMap,
+) -> Result<Response, StatusCode> {
+    let (restricted, raw) = session(&state, &headers).await?;
+    if restricted {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
+        return Err(StatusCode::BAD_REQUEST);
+    }
+    let rows = pg_store::summary_for_source(&state.pool, &source)
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    Ok(renewed(Json(rows), &raw))
+}
+async fn logout(
+    State(state): State<IngestState>,
+    headers: HeaderMap,
+) -> Result<Response, StatusCode> {
+    if headers.get(header::ORIGIN).and_then(|h| h.to_str().ok()) != Some(ORIGIN) {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    let raw = raw_cookie(&headers).ok_or(StatusCode::UNAUTHORIZED)?;
+    sqlx::query("DELETE FROM uac_login_session WHERE token_hash=$1")
+        .bind(digest(&raw))
+        .execute(&state.pool)
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    Ok((
+        StatusCode::SEE_OTHER,
+        [
+            (header::LOCATION, "/login"),
+            (
+                header::SET_COOKIE,
+                "analytics_session=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0",
+            ),
+        ],
+    )
+        .into_response())
+}
+
+pub fn build(state: IngestState) -> Router {
+    Router::new()
+        .route("/login", get(login_page).post(login))
+        .route("/logout", post(logout))
+        .route("/change-password", get(change_page))
+        .route("/", get(dashboard))
+        .route("/dashboard", get(dashboard))
+        .route("/api/v1/events/{source}/summary", get(summary))
+        .route("/healthz", get(crate::health))
+        .route("/api/v1/events", post(crate::ingest))
+        .layer(DefaultBodyLimit::max(4096))
+        .layer(axum::middleware::map_response(
+            |mut r: Response| async move {
+                r.headers_mut()
+                    .insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
+                r
+            },
+        ))
+        .with_state(state)
+}
diff --git a/apps/stats_server/src/lib.rs b/apps/stats_server/src/lib.rs
index 7601006..e723078 100644
--- a/apps/stats_server/src/lib.rs
+++ b/apps/stats_server/src/lib.rs
@@ -1,4 +1,5 @@
 pub mod admin;
+pub mod password_login;
 pub mod bootstrap;
 pub mod pg_store;
 
@@ -153,7 +154,7 @@ fn summary_total(rows: &[pg_store::EventSummary]) -> Option<i64> {
 }
 
 pub fn build_app(state: IngestState) -> Router {
-    admin::build(state, None)
+    password_login::build(state)
 }
 
 #[cfg(test)]
diff --git a/apps/stats_server/src/main.rs b/apps/stats_server/src/main.rs
index fbebf48..018df59 100644
--- a/apps/stats_server/src/main.rs
+++ b/apps/stats_server/src/main.rs
@@ -1,4 +1,4 @@
-use stats_server::{IngestState, IngestTokens, admin};
+use stats_server::{IngestState, IngestTokens, password_login};
 use std::sync::Arc;
 
 #[tokio::main]
@@ -17,19 +17,14 @@ async fn main() -> Result<(), Box<dyn std::error::Error>> {
         return Err("來源憑證設定不合法".into());
     }
 
-    // Migration 由既有 DB 管理流程預先套用；不可修改已套用 migration。
-    let config = admin::Config::from_env()?;
-    let app = admin::build(
-        IngestState {
-            pool,
-            tokens: Arc::new(IngestTokens { chat, pet }),
-        },
-        Some(config),
-    );
+    let app = password_login::build(IngestState {
+        pool,
+        tokens: Arc::new(IngestTokens { chat, pet }),
+    });
 
     println!("Stats server running on http://127.0.0.1:3100");
 
-    // 尚無 auth 和儲存，不可綁定公開地址。
+    // 本地教學切片；首次改密碼提交與完整管理尚未接通。
     let listener = tokio::net::TcpListener::bind("127.0.0.1:3100").await?;
     axum::serve(listener, app).await?;
 
diff --git a/apps/stats_server/src/bin/bootstrap_admin.rs b/apps/stats_server/src/bin/bootstrap_admin.rs
index acaa3c8..ea47c1c 100644
--- a/apps/stats_server/src/bin/bootstrap_admin.rs
+++ b/apps/stats_server/src/bin/bootstrap_admin.rs
@@ -14,6 +14,6 @@ async fn main() -> Result<(), Box<dyn std::error::Error>> {
     let id = stats_server::bootstrap::initialize(&pool, password)
         .await
         .map_err(std::io::Error::other)?;
-    println!("管理員已建立：{id}；帳密登入與首次改密碼仍待接通");
+    println!("管理員已建立：{id}；首次改密碼提交仍待接通；初始Session不開放後台");
     Ok(())
 }
diff --git a/apps/stats_server/migrations/20261020230000_password_login_session.sql b/apps/stats_server/migrations/20261020230000_password_login_session.sql
new file mode 100644
index 0000000..fcddd16
--- /dev/null
+++ b/apps/stats_server/migrations/20261020230000_password_login_session.sql
@@ -0,0 +1,9 @@
+-- Additive migration: do not rewrite existing auth migrations or checksums.
+CREATE TABLE uac_login_session (
+    token_hash TEXT PRIMARY KEY,
+    user_id VARCHAR(40) NOT NULL REFERENCES uac_user(id) ON DELETE CASCADE,
+    credential_fingerprint TEXT NOT NULL,
+    restricted BOOLEAN NOT NULL,
+    expires_at BIGINT NOT NULL
+);
+CREATE INDEX idx_uac_login_session_expiry ON uac_login_session(expires_at);
diff --git a/apps/stats_server/tests/day23_password_login.rs b/apps/stats_server/tests/day23_password_login.rs
new file mode 100644
index 0000000..7520e70
--- /dev/null
+++ b/apps/stats_server/tests/day23_password_login.rs
@@ -0,0 +1,320 @@
+#![cfg(feature = "postgres-integration")]
+use axum::{
+    body::Body,
+    http::{Request, StatusCode, header},
+};
+use stats_server::{IngestState, IngestTokens, password_login};
+use std::sync::Arc;
+use tower::ServiceExt;
+fn app(pool: sqlx::PgPool) -> axum::Router {
+    password_login::build(IngestState {
+        pool,
+        tokens: Arc::new(IngestTokens {
+            chat: "fixture-chat".into(),
+            pet: "fixture-pet".into(),
+        }),
+    })
+}
+async fn send(
+    app: axum::Router,
+    path: &str,
+    body: Option<&str>,
+    cookie: Option<&str>,
+) -> axum::response::Response {
+    let mut r = if body.is_some() {
+        Request::post(path)
+            .header("origin", "http://127.0.0.1:3100")
+            .header("content-type", "application/x-www-form-urlencoded")
+    } else {
+        Request::get(path)
+    };
+    if let Some(c) = cookie {
+        r = r.header(header::COOKIE, c);
+    }
+    app.oneshot(
+        r.body(body.map_or_else(Body::empty, |s| Body::from(s.to_owned())))
+            .unwrap(),
+    )
+    .await
+    .unwrap()
+}
+fn cookie(r: &axum::response::Response) -> String {
+    r.headers()[header::SET_COOKIE]
+        .to_str()
+        .unwrap()
+        .split(';')
+        .next()
+        .unwrap()
+        .to_owned()
+}
+#[sqlx::test(migrations = "./migrations")]
+async fn day23_bootstrapped_password_is_restricted_before_first_change(pool: sqlx::PgPool) {
+    stats_server::bootstrap::initialize(&pool, "lesson-only-password".into())
+        .await
+        .unwrap();
+    assert_eq!(
+        send(
+            app(pool.clone()),
+            "/login",
+            Some("username=admin&password=wrong"),
+            None
+        )
+        .await
+        .status(),
+        StatusCode::UNAUTHORIZED
+    );
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM uac_login_session")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count, 0);
+    let ok = send(
+        app(pool.clone()),
+        "/login",
+        Some("username=ADMIN&password=lesson-only-password"),
+        None,
+    )
+    .await;
+    assert_eq!(ok.status(), StatusCode::SEE_OTHER);
+    assert_eq!(ok.headers()[header::LOCATION], "/change-password");
+    let c = cookie(&ok);
+    assert_eq!(
+        send(app(pool.clone()), "/dashboard", None, Some(&c))
+            .await
+            .headers()[header::LOCATION],
+        "/change-password"
+    );
+    assert_eq!(
+        send(
+            app(pool.clone()),
+            "/api/v1/events/web_chat/summary",
+            None,
+            Some(&c)
+        )
+        .await
+        .status(),
+        StatusCode::FORBIDDEN
+    );
+    let pending = send(app(pool.clone()), "/change-password", None, Some(&c)).await;
+    assert_eq!(pending.status(), StatusCode::OK);
+    let body = axum::body::to_bytes(pending.into_body(), 100000)
+        .await
+        .unwrap();
+    assert!(String::from_utf8_lossy(&body).contains("尚未提供改密碼提交"));
+    assert!(
+        stats_server::bootstrap::initialize(&pool, "different-password".into())
+            .await
+            .is_err()
+    );
+    let flag: bool = sqlx::query_scalar("SELECT is_init_pd FROM uac_user")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert!(flag);
+}
+#[sqlx::test(migrations = "./migrations")]
+async fn day23_ready_account_session_survives_new_pool_and_is_revocable(pool: sqlx::PgPool) {
+    stats_server::bootstrap::initialize(&pool, "lesson-only-password".into())
+        .await
+        .unwrap();
+    // 已完成首次密碼更換的既有帳號fixture；此SQL不是提供給使用者繞過首次改密碼的操作。
+    sqlx::query("UPDATE uac_user SET is_init_pd=FALSE")
+        .execute(&pool)
+        .await
+        .unwrap();
+    let ok = send(
+        app(pool.clone()),
+        "/login",
+        Some("username=admin&password=lesson-only-password"),
+        None,
+    )
+    .await;
+    assert_eq!(ok.status(), StatusCode::SEE_OTHER);
+    assert_eq!(ok.headers()[header::LOCATION], "/dashboard");
+    let set = ok.headers()[header::SET_COOKIE].to_str().unwrap();
+    for attr in ["HttpOnly", "SameSite=Strict", "Max-Age=1800"] {
+        assert!(set.contains(attr));
+    }
+    let c = cookie(&ok);
+    let raw = c.split_once('=').unwrap().1;
+    let stored: String = sqlx::query_scalar("SELECT token_hash FROM uac_login_session")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_ne!(stored, raw);
+    assert_eq!(stored.len(), 64);
+    let options = pool.connect_options();
+    pool.close().await;
+    let reopened = sqlx::postgres::PgPoolOptions::new()
+        .max_connections(2)
+        .connect_with((*options).clone())
+        .await
+        .unwrap();
+    let ok = send(app(reopened.clone()), "/dashboard", None, Some(&c)).await;
+    assert_eq!(ok.status(), StatusCode::OK);
+    assert!(ok.headers().contains_key(header::SET_COOKIE));
+    assert_eq!(
+        send(
+            app(reopened.clone()),
+            "/api/v1/events/web_chat/summary",
+            None,
+            Some(&c)
+        )
+        .await
+        .status(),
+        StatusCode::OK
+    );
+    assert_eq!(
+        send(
+            app(reopened.clone()),
+            "/api/v1/events/web_chat/summary",
+            None,
+            None
+        )
+        .await
+        .status(),
+        StatusCode::UNAUTHORIZED
+    );
+    sqlx::query("UPDATE uac_user SET user_status='disabled'")
+        .execute(&reopened)
+        .await
+        .unwrap();
+    assert_eq!(
+        send(
+            app(reopened.clone()),
+            "/api/v1/events/web_chat/summary",
+            None,
+            Some(&c)
+        )
+        .await
+        .status(),
+        StatusCode::UNAUTHORIZED
+    );
+    sqlx::query("UPDATE uac_user SET user_status='active'")
+        .execute(&reopened)
+        .await
+        .unwrap();
+    sqlx::query("UPDATE uac_login_session SET expires_at=0")
+        .execute(&reopened)
+        .await
+        .unwrap();
+    assert_eq!(
+        send(
+            app(reopened),
+            "/api/v1/events/web_chat/summary",
+            None,
+            Some(&c)
+        )
+        .await
+        .status(),
+        StatusCode::UNAUTHORIZED
+    );
+}
+
+#[sqlx::test(migrations = "./migrations")]
+async fn day23_origin_legacy_key_logout_and_credential_revocation(pool: sqlx::PgPool) {
+    stats_server::bootstrap::initialize(&pool, "lesson-only-password".into())
+        .await
+        .unwrap();
+    let bad_origin = Request::post("/login")
+        .header("origin", "http://evil.invalid")
+        .header("content-type", "application/x-www-form-urlencoded")
+        .body(Body::from("username=admin&password=lesson-only-password"))
+        .unwrap();
+    assert_eq!(
+        app(pool.clone())
+            .oneshot(bad_origin)
+            .await
+            .unwrap()
+            .status(),
+        StatusCode::FORBIDDEN
+    );
+    assert_eq!(send(app(pool.clone()),"/login",Some("username=admin&key=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),None).await.status(),StatusCode::UNPROCESSABLE_ENTITY);
+    let bearer = Request::get("/api/v1/events/web_chat/summary")
+        .header(header::AUTHORIZATION, "Bearer fixture-chat")
+        .body(Body::empty())
+        .unwrap();
+    assert_eq!(
+        app(pool.clone()).oneshot(bearer).await.unwrap().status(),
+        StatusCode::UNAUTHORIZED
+    );
+    let ok = send(
+        app(pool.clone()),
+        "/login",
+        Some("username=admin&password=lesson-only-password"),
+        None,
+    )
+    .await;
+    let c = cookie(&ok);
+    let bad_logout = Request::post("/logout")
+        .header("origin", "http://evil.invalid")
+        .header(header::COOKIE, &c)
+        .body(Body::empty())
+        .unwrap();
+    assert_eq!(
+        app(pool.clone())
+            .oneshot(bad_logout)
+            .await
+            .unwrap()
+            .status(),
+        StatusCode::FORBIDDEN
+    );
+    let out = send(app(pool.clone()), "/logout", Some(""), Some(&c)).await;
+    assert_eq!(out.status(), StatusCode::SEE_OTHER);
+    assert!(
+        out.headers()[header::SET_COOKIE]
+            .to_str()
+            .unwrap()
+            .contains("Max-Age=0")
+    );
+    assert_eq!(
+        send(app(pool.clone()), "/change-password", None, Some(&c))
+            .await
+            .status(),
+        StatusCode::UNAUTHORIZED
+    );
+    let ok = send(
+        app(pool.clone()),
+        "/login",
+        Some("username=admin&password=lesson-only-password"),
+        None,
+    )
+    .await;
+    let c = cookie(&ok);
+    sqlx::query("UPDATE uac_user SET user_pd='invalidated-for-test'")
+        .execute(&pool)
+        .await
+        .unwrap();
+    assert_eq!(
+        send(app(pool), "/change-password", None, Some(&c))
+            .await
+            .status(),
+        StatusCode::UNAUTHORIZED
+    );
+}
+
+#[sqlx::test(migrations = "./tests/fixtures/prior-auth")]
+async fn day23_additive_session_migration_preserves_admin(pool: sqlx::PgPool) {
+    let id = stats_server::bootstrap::initialize(&pool, "lesson-only-password".into())
+        .await
+        .unwrap();
+    let before: (String, bool) =
+        sqlx::query_as("SELECT user_pd,is_init_pd FROM uac_user WHERE id=$1")
+            .bind(&id)
+            .fetch_one(&pool)
+            .await
+            .unwrap();
+    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
+    let after: (String, bool) =
+        sqlx::query_as("SELECT user_pd,is_init_pd FROM uac_user WHERE id=$1")
+            .bind(id)
+            .fetch_one(&pool)
+            .await
+            .unwrap();
+    assert_eq!(before, after);
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM uac_login_session")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count, 0);
+}
diff --git a/apps/stats_server/tests/fixtures/prior-auth/20261014223748_create_product_events.sql b/apps/stats_server/tests/fixtures/prior-auth/20261014223748_create_product_events.sql
new file mode 100644
index 0000000..2aecd5e
--- /dev/null
+++ b/apps/stats_server/tests/fixtures/prior-auth/20261014223748_create_product_events.sql
@@ -0,0 +1,12 @@
+-- Add migration script here
+CREATE TABLE product_events (
+    source_app VARCHAR(32) NOT NULL
+        CHECK (source_app IN ('web_chat', 'desktop_pet')),
+    event_id VARCHAR(40) NOT NULL
+        CHECK (char_length(event_id) = 36),
+    schema_version SMALLINT NOT NULL CHECK (schema_version = 1),
+    event_type VARCHAR(128) NOT NULL
+        CHECK (event_type <> '' AND event_type = btrim(event_type)),
+    received_at TIMESTAMPTZ NOT NULL DEFAULT now(),
+    PRIMARY KEY (source_app, event_id)
+);
\ No newline at end of file
diff --git a/apps/stats_server/tests/fixtures/prior-auth/20261018114026_creat_protocol.sql b/apps/stats_server/tests/fixtures/prior-auth/20261018114026_creat_protocol.sql
new file mode 100644
index 0000000..32adb02
--- /dev/null
+++ b/apps/stats_server/tests/fixtures/prior-auth/20261018114026_creat_protocol.sql
@@ -0,0 +1,26 @@
+-- Add migration script here
+CREATE TABLE analytic_token_log(
+    id TEXT NOT NULL PRIMARY KEY,
+    token_src TEXT NOT NULL, -- 憑證來源(chat)
+    token_type TEXT NOT NULL,  -- 憑證類型(jwt, sha256, md5)
+    token_context TEXT NOT NULL,  -- 憑證內容
+    
+    ref_token_src TEXT,  -- 參考的憑證來源(analytic)
+    ref_token_type TEXT,  -- 參考的憑證類型(jwt, sha256, md5)
+    ref_token_context TEXT,  -- 參考的憑證內容
+
+    expired_at TIMESTAMP,  -- 憑證過期時間
+    token_status TEXT,  -- 憑證狀態
+    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
+);
+
+CREATE TABLE analytic_log(
+    id TEXT NOT NULL PRIMARY KEY,
+    local_token TEXT NOT NULL,  -- 本地的憑證
+    remote_token TEXT,  -- 遠端的憑證
+    json_data TEXT NOT NULL,  -- 接收的JSON數據
+    get_status TEXT,  -- 接收狀態
+    get_at TIMESTAMP,  -- 接收時間
+    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
+);
+
diff --git a/apps/stats_server/tests/fixtures/prior-auth/20261018073932_create_auth_user.sql b/apps/stats_server/tests/fixtures/prior-auth/20261018073932_create_auth_user.sql
new file mode 100644
index 0000000..cfb963b
--- /dev/null
+++ b/apps/stats_server/tests/fixtures/prior-auth/20261018073932_create_auth_user.sql
@@ -0,0 +1,108 @@
+-- Add migration script here
+CREATE TABLE uac_user (
+    id varchar(40) PRIMARY KEY, -- 使用者Pkno
+    user_acnt TEXT NOT NULL, -- 使用者名稱
+    user_pd TEXT NOT NULL, -- 密碼雜湊
+    user_email TEXT NOT NULL, -- 電子郵件
+    user_name TEXT NOT NULL, -- 使用者名稱
+    user_status TEXT NOT NULL, -- 使用者狀態
+    is_init_pd BOOLEAN NOT NULL, -- 是否初始化密碼
+    expired_pd_at TIMESTAMP, -- 密碼過期時間
+    last_login_at TIMESTAMP, -- 最後登入時間
+    created_at TIMESTAMP NOT NULL, -- 創建時間
+    updated_at TIMESTAMP NOT NULL -- 更新時間
+);
+
+CREATE TABLE uac_role (
+    id varchar(40) PRIMARY KEY, -- 角色Pkno
+    role_name TEXT NOT NULL, -- 角色名稱
+    role_status TEXT NOT NULL, -- 角色狀態
+    created_at TIMESTAMP NOT NULL, -- 創建時間
+    updated_at TIMESTAMP NOT NULL -- 更新時間
+);
+
+CREATE TABLE uac_user_role (
+    user_id varchar(40) NOT NULL, -- 使用者Pkno
+    role_id varchar(40) NOT NULL, -- 角色Pkno
+    created_at TIMESTAMP NOT NULL, -- 創建時間
+    CONSTRAINT pk_uac_user_role PRIMARY KEY (user_id, role_id)
+);
+
+CREATE TABLE uac_acl (
+    id varchar(40) PRIMARY KEY, -- 頁面Pkno
+    acl_name TEXT NOT NULL, -- 頁面名稱
+    acl_path TEXT NOT NULL, -- 頁面路徑
+    acl_category TEXT NOT NULL, -- 頁面分類(項目: category，頁面: page，功能:action)
+    acl_parent_id varchar(40), -- 父頁面Pkno
+    acl_status TEXT NOT NULL, -- 頁面狀態
+    created_at TIMESTAMP NOT NULL, -- 創建時間
+    updated_at TIMESTAMP NOT NULL -- 更新時間
+);
+
+CREATE TABLE uac_role_acl (
+    role_id varchar(40) NOT NULL, -- 角色Pkno
+    acl_id varchar(40) NOT NULL, -- 頁面Pkno
+    created_at TIMESTAMP NOT NULL, -- 創建時間
+    CONSTRAINT pk_uac_role_acl PRIMARY KEY (role_id, acl_id)
+);
+
+CREATE TABLE uac_user_acl (
+    user_id varchar(40) NOT NULL, -- 使用者Pkno
+    acl_id varchar(40) NOT NULL, -- 頁面Pkno
+    acl_filter TEXT NOT NULL, -- 頁面加減
+    created_at TIMESTAMP NOT NULL, -- 創建時間
+    CONSTRAINT pk_uac_user_acl PRIMARY KEY (user_id, acl_id)
+);
+
+
+CREATE TABLE mbr_user (
+    id varchar(40) PRIMARY KEY, -- 使用者Pkno
+    user_acnt TEXT NOT NULL, -- 使用者名稱
+    user_pd TEXT NOT NULL, -- 密碼雜湊
+    user_email TEXT NOT NULL, -- 電子郵件
+    user_name TEXT NOT NULL, -- 使用者名稱
+    user_status TEXT NOT NULL, -- 使用者狀態
+    is_init_pd BOOLEAN NOT NULL, -- 是否初始化密碼
+    expired_pd_at TIMESTAMP, -- 密碼過期時間
+    last_login_at TIMESTAMP, -- 最後登入時間
+    created_at TIMESTAMP NOT NULL, -- 創建時間
+    updated_at TIMESTAMP NOT NULL -- 更新時間
+);
+
+CREATE TABLE mbr_role (
+    id varchar(40) PRIMARY KEY, -- 角色Pkno
+    role_name TEXT NOT NULL, -- 角色名稱
+    role_status TEXT NOT NULL, -- 角色狀態
+    created_at TIMESTAMP NOT NULL, -- 創建時間
+    updated_at TIMESTAMP NOT NULL -- 更新時間
+);
+
+CREATE TABLE mbr_user_role (
+    user_id varchar(40) NOT NULL, -- 使用者Pkno
+    role_id varchar(40) NOT NULL, -- 角色Pkno
+    created_at TIMESTAMP NOT NULL, -- 創建時間
+    CONSTRAINT pk_mbr_user_role PRIMARY KEY (user_id, role_id)
+);
+
+CREATE TABLE mbr_perm (
+    id varchar(40) PRIMARY KEY, -- 權限Pkno
+    perm_name TEXT NOT NULL, -- 權限名稱
+    perm_status TEXT NOT NULL, -- 權限狀態
+    created_at TIMESTAMP NOT NULL, -- 創建時間
+    updated_at TIMESTAMP NOT NULL -- 更新時間
+);
+
+CREATE TABLE mbr_role_perm (
+    role_id varchar(40) NOT NULL, -- 角色Pkno
+    perm_id varchar(40) NOT NULL, -- 權限Pkno
+    created_at TIMESTAMP NOT NULL, -- 創建時間
+    CONSTRAINT pk_mbr_role_perm PRIMARY KEY (role_id, perm_id)
+);
+
+CREATE TABLE mbr_user_perm (
+    user_id varchar(40) NOT NULL, -- 使用者Pkno
+    perm_id varchar(40) NOT NULL, -- 權限Pkno
+    perm_filter TEXT NOT NULL, -- 權限加減
+    created_at TIMESTAMP NOT NULL, -- 創建時間
+    CONSTRAINT pk_mbr_user_perm PRIMARY KEY (user_id, perm_id)
+);
\ No newline at end of file
```
### Refactor／Session與權限邊界
只允許既有bootstrap會建立的 `admin` 帳號，以大小寫無關方式查找；若既有資料有兩筆同名大小寫帳號就拒絕，不隨機選一筆，也不自動刪除或合併。登入前後都檢查active狀態；密碼改變後舊Session的credential fingerprint不符即拒絕。token為兩個隨機UUIDv4，不存明文token；cookie為HttpOnly／SameSite=Strict，本地HTTP listener不設Secure，對外TLS部署另需設計。
Session保留登入時的restricted狀態，並與現在的 `is_init_pd` 做OR；不能靠清除目前旗標，把原受限Session提升成一般Session。測試中的 `is_init_pd=FALSE` 僅用來模擬「本來已完成首次改密碼的既有帳號」，不是提供給使用者繞過限制的操作。首次更換提交、唯一Email更新、改密碼所有Session撤銷的正式操作流程都還要做，不能執行測試SQL作為替代。
新增 `uac_login_session` 只使用CREATE TABLE／INDEX，外鍵連到既有uac_user；不改舊auth migration。升級測試先用本篇main新增前的完整migrations建立管理員，再跑新runner，密碼hash與首次更換旗標都保留。這只驗這個明確基底，不代表所有使用者歷史DB相容。
### 舊 Day 更正補充（程式／套件差異）：Analytics Day22
對應FIFO `analytics-day22-postfreeze-20261020`，本日只補這一個單元。main已有 `bootstrap.rs` 與 `bootstrap_admin`，不重新新增。保留現有transaction、EXCLUSIVE表鎖、並發只建立一個admin、已有admin就拒絕且不覆寫密碼。Argon2 0.6單參數 `hash_password` 會產生隨機鹽，不能套回0.5的SaltString／rand_core API；並發初始化與hash保持已用真PostgreSQL重測。
現有CLI接受8–128 bytes並互動輸入，舊文12–128與正式需求的預設admin/admin尚未對齊；本切片沿用真實來源，不偷偷改回舊範例或宣稱預設初始化完成。舊 `is_init_pd=true` 沒有權限效果，本日透過真實新builder確實拒絕初始Session讀報表，但**改密碼提交仍未完成**。CLI的成功訊息同步改成「首次改密碼提交仍待接通；初始Session不開放後台」。
## 3. 套件版本
workspace0.2.22／edition2024，rustc／cargo1.99.1，專用PostgreSQL18.6。lock精確值並對照[crates.io](https://crates.io)官方穩定版：Argon2 0.6.0、Axum0.8.9、SQLx0.9.0、Tokio1.54.0、UUID1.28.0、SHA2 0.11.1（來源亦有0.10.9，workspace用0.11）、Leptos0.8.22、rpassword7.5.4。此篇無新增套件、無cargo update或來源lock修改。
Native／SSR相關程式及所有targets已編譯；新頁為server回應HTML，不新增WASM。MSRV、Windows／macOS、瀏覽器GUI與正式部署未驗。
## 4. 驗證項目與實際結果
- 原main：在專用PostgreSQL18.6設定 `DATABASE_URL` 及 `ANALYTICS_TEST_DATABASE_URL`，`cargo test --locked -p stats-server --features postgres-integration -- --include-ignored`，26 passed／0 failed／0 ignored。
- Red：新測試因缺password_login模組E0432，`red.log`；環境／快取錯誤不計入Red。
- Green：同一完整測試命令，30 passed／0 failed／0 ignored，其中26項是保留的舊程式測試，4項是新帳密／Session／additive升級測試；舊key測試通過不能作為新登入驗收。
- 新測試覆蓋：bootstrap後錯密碼不建Session、大小寫admin帳號、303受限頁、受限查詢403、既有ready帳號成功查詢、token只存digest、新pool與新router仍可用、閒置續發cookie、停用／逾期／登出／改hash拒絕舊Session、錯Origin403、舊key表單422、來源token不能讀人員summary，以及additive migration保留原hash／旗標。
- `cargo check --locked -p stats-server --all-targets --features postgres-integration` 通過，包含真正main與CLI接線。
- 真實CLI：專用DB上以TTY兩次輸入相同教學密碼，`bootstrap_admin` exit0，僅建立一筆admin，沒有讀產品秘密。
- 真TCP listener：未設定舊admin key，實際啟動新main；GET登入頁200、舊key表單422、錯Origin403、帳密登入303、受限頁200、報表403、登出303、舊cookie401。僅記錄狀態與斷言，不輸出token／密碼hash。
- 完整patch在精確main archive執行 `git apply --check` 通過。
[完整patch、測試／CLI／HTTP紀錄、版本與CI證據](https://github.com/williechen/copilot_use/tree/teaching/rust-ai-chat-alignment/docs/teaching/day23/analytics-validation)。測試cluster只綁127.0.0.1:55439，完成後已停止；既有5432服務未動。
### 同來源CI快照
- [Rust Desktop Pet / 39428806744](https://github.com/williechen/rust_ai_chat/actions/runs/39428806744)：success，UTC 2026-10-20T13:26:13Z。
- [Rust AI Chat / 39428806840](https://github.com/williechen/rust_ai_chat/actions/runs/39428806840)：success，UTC 2026-10-20T13:26:52Z。
- [Persistence / 39428806698](https://github.com/williechen/rust_ai_chat/actions/runs/39428806698)：failure，UTC 2026-10-20T13:18:09Z。
- [Test All / 39428806748](https://github.com/williechen/rust_ai_chat/actions/runs/39428806748)：failure，UTC 2026-10-20T13:19:11Z。
- [Rust Dashboard / 39428806718](https://github.com/williechen/rust_ai_chat/actions/runs/39428806718)：success，UTC 2026-10-20T13:17:59Z。
- [Node / 39428806798](https://github.com/williechen/rust_ai_chat/actions/runs/39428806798)：success，UTC 2026-10-20T13:15:52Z。
workflow精確SHA均為本篇main；CI不包含新教學程式，未把job未涵蓋內容算驗收。
### 限制與下一步
新初始化帳號目前停在可登出的受限頁，尚無首次更換密碼／唯一Email提交，因此**完整使用者登入到報表流程尚未完成**。只有測試中明確模擬已完成初始化的既有帳號能進最小Dashboard；不能清除旗標繞過流程。帳號級持久化倍增等待、後台可設定閒置逾時、管理者建立其他帳號／角色／來源範圍、詳細查詢畫面、正式密碼修改重設、SSE與完整GUI都未實作驗收。新runtime不再使用舊全域key登入的限流，帳號等待需要在後續登入主線接通；本篇不作可對外部署的安全完成聲明。
Session已驗新pool／router重建後讀回，但沒有宣稱整套程序重啟或跨平台驗收。教學發布、使用者套用、產品整合與功能驗收分別記錄；不修改Day22（含）以前正文，也不新增排程。
