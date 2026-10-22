## 1. 學習大綱：完成首次改密碼與唯一 Email，再重新登入

本日最小切片是「受限 Session→真正改密碼表單→驗目前密碼／新密碼確認／唯一 Email→單一 PostgreSQL 交易更新並撤銷所有登入 Session→回登入頁→用新密碼進後台」。來源 <code>rust_ai_chat/main</code> 為 <code>4f0cd00b8d571bdaf120298efebf0d62b795498f</code>，workspace 0.2.23；教學初始基準 <code>f3b32b6a27d06db72e40f7d92a1530f52d549342</code>，Chat、Pet Day24已完整交付，才依序處理 Analytics。

原本 <code>apps/stats_server/src/password_login.rs</code> 只提供 GET <code>/change-password</code> 的待辦訊息，沒有 POST，所以管理員登入後卡在受限狀態。本篇不再優先調限流細節。已完成初始化的帳號也可從 dashboard 的連結進入同一表單，仍要驗目前密碼，不用 SQL 手動清 flag 繞過限制。

真實主線沿用 main 的 <code>password_login::build</code> 路由與相同 <code>IngestState.pool</code>，沒有另開假 API。GET 先驗 Session，表單輸出與本次不可預測 token 綁定的 CSRF 摘要；POST 檢查精確 Origin、有效 Session、CSRF、新密碼長度／確認／不同舊密碼，以及基本 Email 格式；交易內重新鎖定使用者與 Session、重驗密碼指紋和目前密碼，更新加鹽雜湊、Email與 <code>is_init_pd=false</code>，刪該帳號全部 Session，成功才提交與清 cookie。

規則 daily-tdd-minimum-v1，GitHub 規則 blob <code>ebebad6b3d671b7cdfa48a5f121a9958010b15b4</code>，共同 Notion 更新 <code>2026-10-20T16:32:31.684Z</code>。Day23（含）以前封存，不回寫正文；本篇只更動隔離教學副本。copilot\_use 自身 Application／Adapter／管理員模型不同，不把本篇視為產品整合。主體 Red／Green 估15分鐘，Refactor／migration 5分鐘，版本與驗證閱讀另8分鐘；三案主體約50分鐘。

前置是本機專用 PostgreSQL 與既有管理員初始化。測試沿用真正 bootstrap 的至少8-byte密碼政策；目前原始碼沒有需求中的首次 admin/admin（4-byte）初始化，不把這個差異冒稱已完成。帳號仍只允許唯一 admin，完整 RBAC、帳號級持久失敗等待、寄信重設、可配置閒置時間仍未接通；改密碼也沒有不存在的持久失敗計數可清。本篇沒有執行付費 AI、讀取正式秘密或修改既存5432服務。

## 2. 實作內容：Red → 完整最小 Green → Refactor

### Red：實際 POST 仍為 405

新增下方完整 patch 內的 <code>apps/stats_server/tests/day24_password_change.rs</code>，在原始 handler 執行 <code>cargo test --locked -p stats-server --features postgres-integration --test day24_password_change</code>。第一個情境先透過真正 bootstrap 與登入取得受限 cookie，送錯誤 CSRF 的改密碼表單；預期403，但 main 只有 GET，實際405。這是預期功能缺口的行為 Red，不是環境、連線或匯入失敗。

```rs
// 完整 setup、請求 helper 與兩項測試都包含於下方 patch。
let denied = send(app(pool.clone()), "/change-password",
    Some("current_password=lesson-only-password&new_password=new-lesson-password&confirm_password=new-lesson-password&email=learner%40example.test&csrf=wrong"),
    Some(&c)).await;
assert_eq!(denied.status(), StatusCode::FORBIDDEN);
// 原始實際：left 405，right 403。
```

### Green：完整適用修改

以下完整 patch 含 handler、表單／dashboard 連結、POST 接線、新增 Email unique migration、兩項 PostgreSQL 流程測試，以及目前程式內「尚未提供提交」的舊斷言／CLI訊息調整。改的是未封存的新教學副本測試，不改封存 Notion Day23。若已手動新增 Red 測試，套其餘修改即可；或在乾淨來源一次套完整 patch，不重複新增測試。原始 patch apply check 通過，沒有新增依賴或更新 lock。

```diff
diff --git a/apps/stats_server/migrations/20261021071000_unique_account_email.sql b/apps/stats_server/migrations/20261021071000_unique_account_email.sql
new file mode 100644
index 0000000..d98da8b
--- /dev/null
+++ b/apps/stats_server/migrations/20261021071000_unique_account_email.sql
@@ -0,0 +1,2 @@
+-- Additive: pre-existing duplicates stop migration; never delete users to make it pass.
+CREATE UNIQUE INDEX uq_uac_user_email ON uac_user(lower(trim(user_email)));
diff --git a/apps/stats_server/src/bin/bootstrap_admin.rs b/apps/stats_server/src/bin/bootstrap_admin.rs
index ea47c1c..5da878c 100644
--- a/apps/stats_server/src/bin/bootstrap_admin.rs
+++ b/apps/stats_server/src/bin/bootstrap_admin.rs
@@ -14,6 +14,6 @@ async fn main() -> Result<(), Box<dyn std::error::Error>> {
     let id = stats_server::bootstrap::initialize(&pool, password)
         .await
         .map_err(std::io::Error::other)?;
-    println!("管理員已建立：{id}；首次改密碼提交仍待接通；初始Session不開放後台");
+    println!("管理員已建立：{id}；首次登入請修改密碼與Email；完成前不開放後台");
     Ok(())
 }
diff --git a/apps/stats_server/src/main.rs b/apps/stats_server/src/main.rs
index 018df59..56eee57 100644
--- a/apps/stats_server/src/main.rs
+++ b/apps/stats_server/src/main.rs
@@ -24,7 +24,7 @@ async fn main() -> Result<(), Box<dyn std::error::Error>> {
 
     println!("Stats server running on http://127.0.0.1:3100");
 
-    // 本地教學切片；首次改密碼提交與完整管理尚未接通。
+    // 本地教學切片；首次改密碼已接通，完整管理與角色授權另行完成。
     let listener = tokio::net::TcpListener::bind("127.0.0.1:3100").await?;
     axum::serve(listener, app).await?;
 
diff --git a/apps/stats_server/src/password_login.rs b/apps/stats_server/src/password_login.rs
index b75b957..eb564ec 100644
--- a/apps/stats_server/src/password_login.rs
+++ b/apps/stats_server/src/password_login.rs
@@ -1,5 +1,5 @@
 use crate::{IngestState, pg_store};
-use argon2::{Argon2, PasswordHash, PasswordVerifier};
+use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
 use axum::{
     Form, Json, Router,
     extract::{DefaultBodyLimit, Path, State},
@@ -225,29 +225,121 @@ async fn change_page(
     headers: HeaderMap,
 ) -> Result<Response, StatusCode> {
     let (restricted, raw) = session(&state, &headers).await?;
-    if !restricted {
-        return Ok(renewed(Redirect::to("/dashboard"), &raw));
-    }
+    let csrf = digest(&format!("change-password:{raw}"));
+    let heading = if restricted {
+        "首次登入須更換密碼與Email"
+    } else {
+        "修改密碼與Email"
+    };
     Ok(renewed(
-        Html(
-            r#"<!doctype html>
-            <html lang="zh-Hant">
-                <head>
-                    <meta charset="utf-8">
-                </head>
-                <body>
-                    <h1>首次登入須更換密碼與Email</h1>
-                    <p>帳密已驗證，後台查詢仍被拒絕。本切片尚未提供改密碼提交，請勿清除資料庫標記繞過限制。</p>
-                    <form method="post" action="/logout">
-                        <button>登出</button>
-                    </form>
-                </body>
-            </html>"#,
-        ),
+        Html(format!(
+            r#"<!doctype html><html lang="zh-Hant"><head><meta charset="utf-8"><title>修改密碼</title></head><body><h1>{heading}</h1>
+<form method="post" action="/change-password">
+<input type="hidden" name="csrf" value="{csrf}">
+<label>目前密碼<input type="password" name="current_password" autocomplete="current-password" required maxlength="128"></label>
+<label>新密碼<input type="password" name="new_password" autocomplete="new-password" required minlength="8" maxlength="128"></label>
+<label>確認新密碼<input type="password" name="confirm_password" autocomplete="new-password" required minlength="8" maxlength="128"></label>
+<label>唯一Email<input type="email" name="email" autocomplete="email" required maxlength="254"></label>
+<button>儲存並重新登入</button></form>
+<p>成功後所有裝置都須重新登入。Email不驗證；寄信重設尚未接通。</p>
+<form method="post" action="/logout"><button>登出</button></form></body></html>"#
+        )),
         &raw,
     ))
 }
 
+#[derive(Deserialize)]
+struct PasswordChange {
+    current_password: String,
+    new_password: String,
+    confirm_password: String,
+    email: String,
+    csrf: String,
+}
+
+async fn change_password(
+    State(state): State<IngestState>,
+    headers: HeaderMap,
+    Form(form): Form<PasswordChange>,
+) -> Result<Response, StatusCode> {
+    if headers.get(header::ORIGIN).and_then(|h| h.to_str().ok()) != Some(ORIGIN) {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    let (_, raw) = session(&state, &headers).await?;
+    if form.csrf != digest(&format!("change-password:{raw}")) {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    let email = form.email.trim().to_ascii_lowercase();
+    let pieces: Vec<_> = email.split('@').collect();
+    if !(8..=128).contains(&form.new_password.len())
+        || form.current_password.is_empty()
+        || form.current_password.len() > 128
+        || form.new_password != form.confirm_password
+        || form.new_password == form.current_password
+        || email.len() > 254
+        || !email.is_ascii()
+        || email
+            .bytes()
+            .any(|b| b.is_ascii_whitespace() || b.is_ascii_control())
+        || pieces.len() != 2
+        || pieces[0].is_empty()
+        || !pieces[1].contains('.')
+        || pieces[1].starts_with('.')
+        || pieces[1].ends_with('.')
+    {
+        return Err(StatusCode::UNPROCESSABLE_ENTITY);
+    }
+    let mut tx = state
+        .pool
+        .begin()
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    // 重驗本次有效 Session，鎖定會員與 Session，不能靠前端或先前 GET 的結果授權。
+    let row:Option<(String,String,String)>=sqlx::query_as(
+        "SELECT u.id,u.user_pd,s.credential_fingerprint FROM uac_user u JOIN uac_login_session s ON s.user_id=u.id WHERE s.token_hash=$1 AND s.expires_at>floor(extract(epoch from now()))::bigint AND u.user_status='active' AND lower(u.user_acnt)='admin' FOR UPDATE OF u,s")
+        .bind(digest(&raw)).fetch_optional(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
+    let (id, current, fingerprint) = row.ok_or(StatusCode::UNAUTHORIZED)?;
+    if digest(&current) != fingerprint {
+        return Err(StatusCode::UNAUTHORIZED);
+    }
+    let encoded = tokio::task::spawn_blocking(move || {
+        let parsed = PasswordHash::new(&current).map_err(|_| StatusCode::UNAUTHORIZED)?;
+        Argon2::default()
+            .verify_password(form.current_password.as_bytes(), &parsed)
+            .map_err(|_| StatusCode::UNAUTHORIZED)?;
+        Argon2::default()
+            .hash_password(form.new_password.as_bytes())
+            .map(|h| h.to_string())
+            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)
+    })
+    .await
+    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)??;
+    sqlx::query("UPDATE uac_user SET user_pd=$1,user_email=$2,is_init_pd=FALSE,updated_at=now() WHERE id=$3")
+        .bind(encoded).bind(email).bind(&id).execute(&mut *tx).await.map_err(|e|match e {
+            sqlx::Error::Database(ref d) if d.is_unique_violation()=>StatusCode::CONFLICT,
+            _=>StatusCode::SERVICE_UNAVAILABLE,
+        })?;
+    sqlx::query("DELETE FROM uac_login_session WHERE user_id=$1")
+        .bind(id)
+        .execute(&mut *tx)
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    tx.commit()
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
 async fn dashboard(State(state): State<IngestState>, headers: HeaderMap) -> Response {
     let (restricted, raw) = match session(&state, &headers).await {
         Ok(v) => v,
@@ -272,7 +364,7 @@ async fn dashboard(State(state): State<IngestState>, headers: HeaderMap) -> Resp
                     <meta charset="utf-8">
                 </head>
                 <body>
-                    <h1>Analytics 管理後台</h1>
+                    <h1>Analytics 管理後台</h1><a href="/change-password">修改密碼與Email</a>
                     <p>已完成密碼初始化的管理員：Chat 已提交事件總數 {total}</p>
                     <p>詳細查詢畫面與管理操作將於後續切片接回。</p>
                     <form method="post" action="/logout">
@@ -333,7 +425,7 @@ pub fn build(state: IngestState) -> Router {
     Router::new()
         .route("/login", get(login_page).post(login))
         .route("/logout", post(logout))
-        .route("/change-password", get(change_page))
+        .route("/change-password", get(change_page).post(change_password))
         .route("/", get(dashboard))
         .route("/dashboard", get(dashboard))
         .route("/api/v1/events/{source}/summary", get(summary))
diff --git a/apps/stats_server/tests/day23_password_login.rs b/apps/stats_server/tests/day23_password_login.rs
index d6b176d..09d1d9b 100644
--- a/apps/stats_server/tests/day23_password_login.rs
+++ b/apps/stats_server/tests/day23_password_login.rs
@@ -100,7 +100,7 @@ async fn day23_bootstrapped_password_is_restricted_before_first_change(pool: sql
     let body = axum::body::to_bytes(pending.into_body(), 100000)
         .await
         .unwrap();
-    assert!(String::from_utf8_lossy(&body).contains("尚未提供改密碼提交"));
+    assert!(String::from_utf8_lossy(&body).contains("儲存並重新登入"));
     assert!(
         stats_server::bootstrap::initialize(&pool, "different-password".into())
             .await
diff --git a/apps/stats_server/tests/day24_password_change.rs b/apps/stats_server/tests/day24_password_change.rs
new file mode 100644
index 0000000..fd4727f
--- /dev/null
+++ b/apps/stats_server/tests/day24_password_change.rs
@@ -0,0 +1,321 @@
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
+
+#[sqlx::test(migrations = "./migrations")]
+async fn day24_password_change_is_atomic_revokes_all_sessions_and_requires_relogin(
+    pool: sqlx::PgPool,
+) {
+    stats_server::bootstrap::initialize(&pool, "lesson-only-password".into())
+        .await
+        .unwrap();
+    let login = send(
+        app(pool.clone()),
+        "/login",
+        Some("username=admin&password=lesson-only-password"),
+        None,
+    )
+    .await;
+    let c = cookie(&login);
+    // Before Green this is 405: POST is not implemented.
+    let denied=send(app(pool.clone()),"/change-password",Some("current_password=lesson-only-password&new_password=new-lesson-password&confirm_password=new-lesson-password&email=learner%40example.test&csrf=wrong"),Some(&c)).await;
+    assert_eq!(denied.status(), StatusCode::FORBIDDEN);
+    let page = send(app(pool.clone()), "/change-password", None, Some(&c)).await;
+    let html = String::from_utf8(
+        axum::body::to_bytes(page.into_body(), 10000)
+            .await
+            .unwrap()
+            .to_vec(),
+    )
+    .unwrap();
+    let csrf = html
+        .split("name=\"csrf\" value=\"")
+        .nth(1)
+        .unwrap()
+        .split('"')
+        .next()
+        .unwrap();
+    assert_eq!(csrf.len(), 64);
+    let body = format!(
+        "current_password=lesson-only-password&new_password=new-lesson-password&confirm_password=new-lesson-password&email=learner%40example.test&csrf={csrf}"
+    );
+    let wrong = body.replace(
+        "current_password=lesson-only-password",
+        "current_password=wrong-password",
+    );
+    assert_eq!(
+        send(
+            app(pool.clone()),
+            "/change-password",
+            Some(&wrong),
+            Some(&c)
+        )
+        .await
+        .status(),
+        StatusCode::UNAUTHORIZED
+    );
+    let mismatch = body.replace(
+        "confirm_password=new-lesson-password",
+        "confirm_password=not-matching-password",
+    );
+    assert_eq!(
+        send(
+            app(pool.clone()),
+            "/change-password",
+            Some(&mismatch),
+            Some(&c)
+        )
+        .await
+        .status(),
+        StatusCode::UNPROCESSABLE_ENTITY
+    );
+    let foreign = app(pool.clone())
+        .oneshot(
+            Request::post("/change-password")
+                .header("origin", "https://foreign.example")
+                .header("content-type", "application/x-www-form-urlencoded")
+                .header("cookie", &c)
+                .body(Body::from(body.clone()))
+                .unwrap(),
+        )
+        .await
+        .unwrap();
+    assert_eq!(foreign.status(), StatusCode::FORBIDDEN);
+    sqlx::query("INSERT INTO uac_user(id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at) VALUES('other','other','unused','learner@example.test','Other','active',FALSE,now(),now())").execute(&pool).await.unwrap();
+    assert_eq!(
+        send(app(pool.clone()), "/change-password", Some(&body), Some(&c))
+            .await
+            .status(),
+        StatusCode::CONFLICT
+    );
+    let flag: bool =
+        sqlx::query_scalar("SELECT is_init_pd FROM uac_user WHERE lower(user_acnt)='admin'")
+            .fetch_one(&pool)
+            .await
+            .unwrap();
+    assert!(flag);
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
+    sqlx::query("DELETE FROM uac_user WHERE id='other'")
+        .execute(&pool)
+        .await
+        .unwrap();
+    let second = send(
+        app(pool.clone()),
+        "/login",
+        Some("username=admin&password=lesson-only-password"),
+        None,
+    )
+    .await;
+    let c2 = cookie(&second);
+    let done = send(app(pool.clone()), "/change-password", Some(&body), Some(&c)).await;
+    assert_eq!(done.status(), StatusCode::SEE_OTHER);
+    assert_eq!(done.headers()[header::LOCATION], "/login");
+    assert!(
+        done.headers()[header::SET_COOKIE]
+            .to_str()
+            .unwrap()
+            .contains("Max-Age=0")
+    );
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM uac_login_session")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count, 0);
+    for old in [&c, &c2] {
+        assert_eq!(
+            send(
+                app(pool.clone()),
+                "/api/v1/events/web_chat/summary",
+                None,
+                Some(old)
+            )
+            .await
+            .status(),
+            StatusCode::UNAUTHORIZED
+        );
+    }
+    let row: (bool, String, String) = sqlx::query_as(
+        "SELECT is_init_pd,user_email,user_pd FROM uac_user WHERE lower(user_acnt)='admin'",
+    )
+    .fetch_one(&pool)
+    .await
+    .unwrap();
+    assert!(!row.0);
+    assert_eq!(row.1, "learner@example.test");
+    assert_ne!(row.2, "new-lesson-password");
+    assert_eq!(
+        send(
+            app(pool.clone()),
+            "/login",
+            Some("username=admin&password=lesson-only-password"),
+            None
+        )
+        .await
+        .status(),
+        StatusCode::UNAUTHORIZED
+    );
+    let login = send(
+        app(pool.clone()),
+        "/login",
+        Some("username=admin&password=new-lesson-password"),
+        None,
+    )
+    .await;
+    assert_eq!(login.headers()[header::LOCATION], "/dashboard");
+    assert_eq!(
+        send(app(pool.clone()), "/dashboard", None, Some(&cookie(&login)))
+            .await
+            .status(),
+        StatusCode::OK
+    );
+}
+
+#[sqlx::test(migrations = "./migrations")]
+async fn day24_duplicate_migration_stops_and_revocation_failure_rolls_back(pool: sqlx::PgPool) {
+    stats_server::bootstrap::initialize(&pool, "lesson-only-password".into())
+        .await
+        .unwrap();
+    sqlx::query("DROP INDEX uq_uac_user_email")
+        .execute(&pool)
+        .await
+        .unwrap();
+    sqlx::query("INSERT INTO uac_user(id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at) VALUES('duplicate','other','unused',' ADMIN@LOCALHOST.INVALID ','Other','active',FALSE,now(),now())").execute(&pool).await.unwrap();
+    assert!(
+        sqlx::raw_sql(include_str!(
+            "../migrations/20261021071000_unique_account_email.sql"
+        ))
+        .execute(&pool)
+        .await
+        .is_err()
+    );
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM uac_user")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count, 2); // No deletion to force a migration pass.
+    sqlx::query("DELETE FROM uac_user WHERE id='duplicate'")
+        .execute(&pool)
+        .await
+        .unwrap(); // Fixture-only manual resolution.
+    sqlx::raw_sql(include_str!(
+        "../migrations/20261021071000_unique_account_email.sql"
+    ))
+    .execute(&pool)
+    .await
+    .unwrap();
+    let login = send(
+        app(pool.clone()),
+        "/login",
+        Some("username=admin&password=lesson-only-password"),
+        None,
+    )
+    .await;
+    let c = cookie(&login);
+    let page = send(app(pool.clone()), "/change-password", None, Some(&c)).await;
+    let html = String::from_utf8(
+        axum::body::to_bytes(page.into_body(), 10000)
+            .await
+            .unwrap()
+            .to_vec(),
+    )
+    .unwrap();
+    let csrf = html
+        .split("name=\"csrf\" value=\"")
+        .nth(1)
+        .unwrap()
+        .split('"')
+        .next()
+        .unwrap();
+    let before: String = sqlx::query_scalar("SELECT user_pd FROM uac_user")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    sqlx::raw_sql("CREATE FUNCTION fixture_fail_revoke() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'fixture'; END; $$; CREATE TRIGGER fixture_fail_revoke BEFORE DELETE ON uac_login_session FOR EACH ROW EXECUTE FUNCTION fixture_fail_revoke();").execute(&pool).await.unwrap();
+    let body = format!(
+        "current_password=lesson-only-password&new_password=new-lesson-password&confirm_password=new-lesson-password&email=learner%40example.test&csrf={csrf}"
+    );
+    assert_eq!(
+        send(app(pool.clone()), "/change-password", Some(&body), Some(&c))
+            .await
+            .status(),
+        StatusCode::SERVICE_UNAVAILABLE
+    );
+    let row: (String, bool, String) =
+        sqlx::query_as("SELECT user_pd,is_init_pd,user_email FROM uac_user")
+            .fetch_one(&pool)
+            .await
+            .unwrap();
+    assert_eq!(row, (before, true, "admin@localhost.invalid".into()));
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM uac_login_session")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count, 1);
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
+}
```

### Refactor：不把部分寫入當作完成

Email 新增索引採 <code>lower(trim(user_email))</code>，與伺服器 trim＋英文小寫一致。舊資料有衝突時 migration直接失敗、保留全部帳號；不能刪資料或改歷史 checksum 讓它通過。測試刻意插入大小寫／空白衝突，確認建索引失敗且兩筆都存在；fixture 的人工刪除只限隔離測試，不是給真實舊庫的升級指令。

密碼更新、首次旗標與全部 Session 撤銷同交易：Email409或撤銷503都回滾。測試用 PostgreSQL trigger 製造撤銷失敗，確認原雜湊、Email、首次旗標及原 Session 均保留，仍不能進報表。成功時不自動登入；舊 cookie、其他裝置 cookie 和舊密碼都失效，必須用新密碼重新登入。

CSRF 值是 domain-separated <code>SHA256("change-password:" + session bearer)</code>，bearer本身不輸出到表單或DB；表單只含摘要，GET與POST均必須通過有效 Session，另以 SameSite=Strict 與精確 Origin 做本機開發防護。POST交易再確認有效期、帳號啟用與密碼指紋，不能憑先前GET的成功直接授權。對外部署需要HTTPS、Secure cookie與可配置來源，這些本篇未實作。

### 舊 Day 更正補充

保留封存 Day23 的「受限登入完成、首次改密碼未接通」歷史事實，本日才提供提交。先前 Day22 補教已結案，不重開、不因補教或CI再遞增。表單錯誤目前以HTTP狀態碼回應，尚未提供逐欄位友善提示與密碼重設寄信；不要把最小成功流程當完整管理後台。

## 3. 套件版本與平台界線

本日 manifest／lock原樣保留，主要為 Axum 0.8.9、Argon2 0.6.0、SQLx 0.9.0、Serde 1.0.229、SHA2 workspace 0.11（lock另有傳遞依賴0.10.9）、Tokio 1.54.0、UUID 1.28.0、Leptos 0.8.22；本次官方 registry 查得 stable 與直接使用版本相同。[官方 Argon2 版本與API資料](https://crates.io/crates/argon2)。Argon2 0.6 的 <code>hash_password</code> 自動使用隨機鹽，驗證沿用 <code>PasswordHash</code>／<code>PasswordVerifier</code>，不套用舊版手動 SaltString API。

本機 Rust/Cargo 1.99.1、Linux native、PostgreSQL18.6；不是用SQLite取代PG。SSR表單由既有 Axum HTML 路由呈現，無新增WASM。native bins編譯成功；最低MSRV、Windows、macOS、正式TLS代理及跨瀏覽器UI未測。套件最新不表示升級來源 lock 或產品CI涵蓋本日隔離 patch。

## 4. 驗證項目、證據與未完成

測試使用新建的專用本機 PostgreSQL cluster／55439；測試URL只指向隔離庫，不能照貼到正式環境。下列命令先用自己的測試URL設定 <code>DATABASE_URL</code> 與 <code>ANALYTICS_TEST_DATABASE_URL</code>，不得輸出真密鑰或 cookie。

- <code>cargo test --locked -p stats-server --features postgres-integration --test day24_password_change</code>：main Red實際405／預期403。
- <code>cargo test --locked -p stats-server --features postgres-integration -- --include-ignored</code>：最終30項通過、0失敗。兩項新增流程涵蓋 Origin／CSRF拒絕、目前密碼錯誤、確認不一致、唯一Email409回滾、成功撤銷所有Session、舊密碼失效、新登入進後台、migration衝突安全停止、撤銷失敗交易回滾。既有PG測試也全部執行。
- <code>cargo build --locked -p stats-server --bins</code>：通過，含真正初始化CLI；CLI已以不回顯的合成測試密碼完成初始化。
- 真TCP HTTP／3100：受限登入報表403→取得真表單CSRF→POST改密碼303/login並清cookie→舊cookie401→停止並重啟整個程序→舊密碼401→新密碼登入到dashboard200，四項流程結果全通過；不是只測 handler helper。
- <code>git apply --cached --check candidate.patch</code>：精確4f0cd00來源基準通過。
- 未跑真瀏覽器表單互動、真實舊庫升級、帳號失敗計數、忘記密碼寄信與完整角色授權。隔離DB完成後停止並清除，既存5432與copilot\_use資料保留。

精確來源4f0cd00的CI（與教學patch分開）如下；job成功只證明實際執行範圍，不能代替上面的PG、真HTTP與程序重啟：

- [Node](https://github.com/williechen/rust_ai_chat/actions/runs/39511377669)：completed/success；2026-10-20T22:54:14Z；jobs test-node=success
- [Rust Dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39511377730)：completed/success；2026-10-20T22:56:25Z；jobs build-dashboard=success, check-dashboard=success
- [Rust AI Chat](https://github.com/williechen/rust_ai_chat/actions/runs/39511377716)：in\_progress/None；2026-10-20T22:58:18Z；jobs check-chat=None, build-chat=None
- [Persistence](https://github.com/williechen/rust_ai_chat/actions/runs/39511377839)：completed/success；2026-10-20T22:58:59Z；jobs test-persistence=success
- [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39511377875)：completed/success；2026-10-20T23:05:24Z；jobs test-chat-application=success, test-server=success, test-pet-domain=success, test-ui=success, test-shared=success, test-stats-server=success, test-ai-core=success, test-desktop-pet=success, test-chat-domain=success
- [Rust Desktop Pet](https://github.com/williechen/rust_ai_chat/actions/runs/39511377861)：completed/success；2026-10-20T23:06:02Z；jobs build-pet=success, check-pet=success

[完整patch、30項測試log、真HTTP腳本與重啟結果、CI及套件原始資料](https://github.com/williechen/copilot_use/tree/teaching/rust-ai-chat-alignment/docs/teaching/day24/analytics-validation)。本篇未套用產品原始碼，教學發布、使用者套用、產品整合與完整功能驗收分別記錄。

只有本篇完整四節、完整程式、來源SHA與Analytics入口索引實際寫入並全文讀回，才能以重新取得的branch SHA與state blob條件式把Analytics next\_day 24→25；保留Chat25、Pet25/r2、makeup\_queue與last\_full\_comparison。同證據重試不重複遞增，短測試內容寫入不足以推進，衝突先重讀。三案完成後驗證總進度，正式排程本次不恢復，也不建立重複任務。
