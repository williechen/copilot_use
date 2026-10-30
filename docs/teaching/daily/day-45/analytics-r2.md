# Analytics Day45 r2｜Role 預設、頁面 ACL 與人員獨立加減權

> 依2026-10-31使用者當次確認修訂同一Day45；原版[analytics.md](analytics.md)及其commit `d7f44708e02ea805a61d6623c666f15b04fed362`保留，不重複推進Day。現行套用以本r2為準。

## ① 目標、來源與完整旅程

產品 `64eff527283fd05657841297879aefeaadd31f64` 唯讀；規則／需求／架構與 learning-state／manifest 讀取基準 `copilot_use@738aeab0e689ebcc9699d0555a598480c8cede46`，Analytics next_day=45。已讀 Day44 logout／fixture 與正式 main→password_login::build；不把 legacy admin::router 當作正式登入入口。

現況僅 hardcode admin 登入，uac_role／uac_acl 等表已存在但正式人員查詢未接角色／來源授權。本篇最小完整旅程：已完成首次改密碼的管理員→Leptos新增唯讀帳號表單→交易寫帳號／角色／來源grant／成功稽核→新帳號以隨機salt Argon2雜湊驗證→強制改密碼並撤銷舊Session→重新登入→只查授權來源的Dashboard、summary、精確kind及CSV。建立失敗回409／422，未成功不留下半套帳號；同一Session每次重新讀角色／來源，縮權及停用立即影響下一請求。

本切片新增管理員建立viewer與初始grant，不提供公開註冊，不把服務Bearer當人員登入。角色／來源編輯頁、寄信重設與即時連線尚待接續；不宣稱整個帳號管理或全產品Gate2通過。正式業務路由只有原password_login builder；未啟用legacy admin Router，不重寫第二套UI。

## Refactor｜Analytics Session PG lookup 移至 persistence

累積問題：password_login 同時處理Cookie、HTTP、PG Session lookup。移出 login_session::lookup 接 token digest／PgPool，回完整SessionRow tuple或SQLx錯誤；原fingerprint、首次改密碼與touch SQL仍由既有caller維持，這次未宣稱整套登入已搬Application。沒有把Axum／HeaderMap帶入persistence，既有path依賴及postgres features原樣保留。

完整逐檔變更如下。原day23初始化／密碼登入、day24原子改密碼／撤銷、day26 Session touch、day37Cookie、day39CSV、day44Logout CSRF均保留斷言，仍穿過正式session caller到新Adapter；新增viewer測試是功能Green，不以測試數取代斷言映射。跨案shared登入與全庫CI不算本案重構。

```diff
diff --git a/analytics_apps/analytics_persistence/src/lib.rs b/analytics_apps/analytics_persistence/src/lib.rs
index 0239d2b..56fefdc 100644
--- a/analytics_apps/analytics_persistence/src/lib.rs
+++ b/analytics_apps/analytics_persistence/src/lib.rs
@@ -1,3 +1,4 @@
+pub mod login_session;
 /// PostgreSQL 聚合查詢結果，供 JSON API 與 Leptos SSR 共用。
 #[derive(Debug, serde::Serialize, sqlx::FromRow, PartialEq, Eq)]
 pub struct EventSummary {
diff --git a/analytics_apps/analytics_persistence/src/login_session.rs b/analytics_apps/analytics_persistence/src/login_session.rs
new file mode 100644
index 0000000..880d467
--- /dev/null
+++ b/analytics_apps/analytics_persistence/src/login_session.rs
@@ -0,0 +1,24 @@
+use sqlx::PgPool;
+
+pub type SessionRow = (bool, bool, String, String, String);
+
+/// The caller hashes the token; this adapter owns only the PostgreSQL lookup.
+pub async fn lookup(pool: &PgPool, token_hash: &str) -> Result<Option<SessionRow>, sqlx::Error> {
+    sqlx::query_as(
+        r#"SELECT s.restricted
+            ,u.is_init_pd
+            ,u.user_pd
+            ,s.credential_fingerprint
+            ,u.id 
+        FROM uac_login_session s 
+        JOIN uac_user u 
+            ON u.id=s.user_id 
+        WHERE s.token_hash=$1 
+            AND s.expires_at>floor(extract(epoch from now()))::bigint 
+            AND u.user_status='active' 
+            AND lower(u.user_acnt)='admin'"#,
+    )
+    .bind(token_hash)
+    .fetch_optional(pool)
+    .await
+}
diff --git a/analytics_apps/analytics_server/src/password_login.rs b/analytics_apps/analytics_server/src/password_login.rs
index 0a51803..3a6d662 100644
--- a/analytics_apps/analytics_server/src/password_login.rs
+++ b/analytics_apps/analytics_server/src/password_login.rs
@@ -142,24 +142,9 @@ async fn login(
 // Recheck account status, mandatory-change state and password fingerprint on each protected request.
 async fn session(state: &IngestState, headers: &HeaderMap) -> Result<(bool, String), StatusCode> {
     let raw = raw_cookie(headers).ok_or(StatusCode::UNAUTHORIZED)?;
-    let found: Option<(bool, bool, String, String, String)> = sqlx::query_as(
-        r#"SELECT s.restricted
-            ,u.is_init_pd
-            ,u.user_pd
-            ,s.credential_fingerprint
-            ,u.id 
-        FROM uac_login_session s 
-        JOIN uac_user u 
-            ON u.id=s.user_id 
-        WHERE s.token_hash=$1 
-            AND s.expires_at>floor(extract(epoch from now()))::bigint 
-            AND u.user_status='active' 
-            AND lower(u.user_acnt)='admin'"#,
-    )
-    .bind(digest(&raw))
-    .fetch_optional(&state.pool)
-    .await
-    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    let found = analytics_persistence::login_session::lookup(&state.pool, &digest(&raw))
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
     let (restricted, must_change, current, fingerprint, id) =
         found.ok_or(StatusCode::UNAUTHORIZED)?;
     if digest(&current) != fingerprint {
```

## ② Red → 完整 Green

核心Red／Green預估15分鐘，未實際計時；Refactor、PG準備、版本／交付另計。先套Refactor，再套Red。在隔離測試PG設定 `DATABASE_URL=postgres://<test-user>@127.0.0.1:<test-port>/<test-db>`，帳號需可建SQLx測試DB；本次用55485與lesson_day45，絕不用正式DB。命令：

```sh
cargo test --manifest-path analytics_apps/Cargo.toml -p analytics-server --features postgres-integration --test day45_viewer --locked
```

以下完整Red，使用正式Router、真migration、表單／Cookie，不以mock授權helper充數。預期舊Router在GET /accounts/new回404而非200；首次缺DATABASE_URL僅為環境錯誤，另存日誌不算Red。

```diff
diff --git a/analytics_apps/analytics_server/tests/day45_viewer.rs b/analytics_apps/analytics_server/tests/day45_viewer.rs
new file mode 100644
index 0000000..a3d9eb5
--- /dev/null
+++ b/analytics_apps/analytics_server/tests/day45_viewer.rs
@@ -0,0 +1,191 @@
+#![cfg(feature = "postgres-integration")]
+mod support;
+use analytics_server::{IngestState, IngestTokens, bootstrap, password_login};
+use axum::http::{StatusCode, header};
+use std::sync::Arc;
+use support::http::{body_text, response_cookie, send};
+
+fn token(html: &str, action: &str) -> String {
+    html.split(&format!("action=\"{action}\""))
+        .nth(1)
+        .unwrap()
+        .split("name=\"csrf\" value=\"")
+        .nth(1)
+        .unwrap()
+        .split('"')
+        .next()
+        .unwrap()
+        .into()
+}
+#[sqlx::test(migrations = "../migrations")]
+async fn admin_creates_viewer_first_change_and_source_revoke_affects_same_session(
+    pool: sqlx::PgPool,
+) {
+    let admin = bootstrap::initialize(&pool, "lesson-password".into())
+        .await
+        .unwrap();
+    sqlx::query("UPDATE uac_user SET is_init_pd=FALSE WHERE id=$1")
+        .bind(&admin)
+        .execute(&pool)
+        .await
+        .unwrap();
+    let app = password_login::build(IngestState {
+        pool: pool.clone(),
+        tokens: Arc::new(IngestTokens {
+            chat: "chat-fixture".into(),
+            pet: "pet-fixture".into(),
+        }),
+    });
+    let login = send(
+        app.clone(),
+        "/login",
+        Some("username=admin&password=lesson-password"),
+        None,
+    )
+    .await;
+    assert_eq!(login.status(), StatusCode::SEE_OTHER);
+    let cookie = response_cookie(&login);
+    let page = send(app.clone(), "/accounts/new", None, Some(&cookie)).await;
+    assert_eq!(page.status(), StatusCode::OK);
+    let csrf = token(&body_text(page).await, "/accounts");
+    let form = format!(
+        "username=reader&password=reader-password&email=reader%40example.test&chat=on&csrf={csrf}"
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/accounts",
+            Some(&form.replace(&csrf, "wrong")),
+            Some(&cookie)
+        )
+        .await
+        .status(),
+        StatusCode::FORBIDDEN
+    );
+    let created = send(app.clone(), "/accounts", Some(&form), Some(&cookie)).await;
+    assert_eq!(created.status(), StatusCode::SEE_OTHER);
+    assert_eq!(
+        created.headers()[header::LOCATION],
+        "/accounts/new?created=1"
+    );
+    let encoded: String =
+        sqlx::query_scalar("SELECT user_pd FROM uac_user WHERE user_acnt='reader'")
+            .fetch_one(&pool)
+            .await
+            .unwrap();
+    assert!(encoded.starts_with("$argon2"));
+    assert!(!encoded.contains("reader-password"));
+    assert_eq!(
+        send(app.clone(), "/accounts", Some(&form), Some(&cookie))
+            .await
+            .status(),
+        StatusCode::CONFLICT
+    );
+    let login = send(
+        app.clone(),
+        "/login",
+        Some("username=reader&password=reader-password"),
+        None,
+    )
+    .await;
+    assert_eq!(login.status(), StatusCode::SEE_OTHER);
+    assert_eq!(login.headers()[header::LOCATION], "/change-password");
+    let reader = response_cookie(&login);
+    assert_eq!(
+        send(
+            app.clone(),
+            "/api/v1/events/web_chat/summary",
+            None,
+            Some(&reader)
+        )
+        .await
+        .status(),
+        StatusCode::FORBIDDEN
+    );
+    let page = body_text(send(app.clone(), "/change-password", None, Some(&reader)).await).await;
+    let csrf = token(&page, "/change-password");
+    let changed=send(app.clone(),"/change-password",Some(&format!("current_password=reader-password&new_password=changed-password&confirm_password=changed-password&email=reader%40example.test&csrf={csrf}")),Some(&reader)).await;
+    assert_eq!(changed.status(), StatusCode::SEE_OTHER);
+    assert_eq!(
+        send(app.clone(), "/dashboard", None, Some(&reader))
+            .await
+            .status(),
+        StatusCode::SEE_OTHER
+    );
+    let reader = response_cookie(
+        &send(
+            app.clone(),
+            "/login",
+            Some("username=reader&password=changed-password"),
+            None,
+        )
+        .await,
+    );
+    for path in [
+        "/dashboard?source=web_chat",
+        "/api/v1/events/web_chat/summary",
+        "/api/v1/events/web_chat/summary/chat.message.persisted",
+        "/api/v1/events/web_chat/export.csv",
+    ] {
+        assert_eq!(
+            send(app.clone(), path, None, Some(&reader)).await.status(),
+            StatusCode::OK,
+            "{path}"
+        );
+    }
+    for path in [
+        "/dashboard?source=desktop_pet",
+        "/api/v1/events/desktop_pet/summary",
+        "/api/v1/events/desktop_pet/summary/pet.sleep",
+        "/api/v1/events/desktop_pet/export.csv",
+        "/accounts/new",
+    ] {
+        assert_eq!(
+            send(app.clone(), path, None, Some(&reader)).await.status(),
+            StatusCode::FORBIDDEN,
+            "{path}"
+        );
+    }
+    let csrf = token(
+        &body_text(send(app.clone(), "/change-password", None, Some(&reader)).await).await,
+        "/logout",
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/accounts",
+            Some(&format!(
+                "username=evil&password=evil-password&email=evil%40example.test&chat=on&csrf={csrf}"
+            )),
+            Some(&reader)
+        )
+        .await
+        .status(),
+        StatusCode::FORBIDDEN
+    );
+    sqlx::query(
+        "DELETE FROM uac_user_acl WHERE user_id=(SELECT id FROM uac_user WHERE user_acnt='reader')",
+    )
+    .execute(&pool)
+    .await
+    .unwrap();
+    assert_eq!(
+        send(
+            app.clone(),
+            "/api/v1/events/web_chat/export.csv",
+            None,
+            Some(&reader)
+        )
+        .await
+        .status(),
+        StatusCode::FORBIDDEN
+    );
+    let audit: i64 = sqlx::query_scalar(
+        "SELECT count(*) FROM analytics_account_audit WHERE actor_id=$1 AND action='create_viewer'",
+    )
+    .bind(admin)
+    .fetch_one(&pool)
+    .await
+    .unwrap();
+    assert_eq!(audit, 1);
+}
```

`analytics-day45-green-r2.patch`附件基準為已套Refactor、尚未套任何Green／Red；已包含同一Red測試及後續回歸，因此做過Red先reverse Red再套Green。逐檔完整修改：

- migration沿用UAC role／ACL／user_role／user_acl，不另造平行權限欄位；只新增建立帳號稽核表及帳號case-fold唯一index。既有重複帳號會令migration明確失敗，不能刪資料硬過；須先備份、盤點、由管理者處理。
- bootstrap 在首次及已存在admin路徑確保admin角色，不覆寫帳密；角色ID與名稱明確，不依使用者任意同名role授權。只對既有保留admin帳號補角色，viewer不可自行升權。
- login查實際帳號，session lookup重新查active角色；change-password解除admin硬碼，沿既有Session／目前密碼／CSRF、唯一Email、首次改密碼與全Session撤銷。
- accounts::page 是Leptos SSR；create接受已改密碼且具帳號管理ACL的人員（角色預設或個人獨立加權）、同源與Session綁定CSRF。交易再次鎖住當前人員／Session／ACL，重查角色預設或個人加權、deny、有效期與fingerprint後，原子寫人員、viewer角色、來源allow及成功稽核；合法管理員的業務失敗另記狀態碼、不寫密碼／token。JSON／Form extractor的協定錯誤尚未形成管理操作，回既有Axum錯誤。
- dashboard、summary、source_kind_summary、CSV在查統計前都經require_source_session；deny優先，讀當下授權而非登入時快照。Pet-only預設Dashboard會選Pet；未授權來源回403／中文說明。跨來源導覽與管理連結的可見性不作安全判定，伺服器仍拒絕越權。

```diff
diff --git a/analytics_apps/analytics_persistence/src/login_session.rs b/analytics_apps/analytics_persistence/src/login_session.rs
index 880d467..7bd1bec 100644
--- a/analytics_apps/analytics_persistence/src/login_session.rs
+++ b/analytics_apps/analytics_persistence/src/login_session.rs
@@ -16,7 +16,8 @@ pub async fn lookup(pool: &PgPool, token_hash: &str) -> Result<Option<SessionRow
         WHERE s.token_hash=$1 
             AND s.expires_at>floor(extract(epoch from now()))::bigint 
             AND u.user_status='active' 
-            AND lower(u.user_acnt)='admin'"#,
+            AND EXISTS(SELECT 1 FROM uac_user_role ur JOIN uac_role r ON r.id=ur.role_id
+                WHERE ur.user_id=u.id AND r.id IN ('analytics-admin','analytics-viewer') AND r.role_status='active')"#,
     )
     .bind(token_hash)
     .fetch_optional(pool)
diff --git a/analytics_apps/analytics_server/src/accounts.rs b/analytics_apps/analytics_server/src/accounts.rs
new file mode 100644
index 0000000..fe3dfa5
--- /dev/null
+++ b/analytics_apps/analytics_server/src/accounts.rs
@@ -0,0 +1,183 @@
+use crate::{
+    IngestState,
+    password_login::{digest, renewed, session},
+};
+use argon2::{Argon2, PasswordHasher};
+use axum::{
+    Form,
+    extract::{Query, State},
+    http::{HeaderMap, StatusCode, header},
+    response::{Html, IntoResponse, Redirect, Response},
+};
+use leptos::prelude::*;
+
+async fn account_manager(
+    state: &IngestState,
+    headers: &HeaderMap,
+) -> Result<(String, String), StatusCode> {
+    let (restricted, raw) = session(state, headers).await?;
+    if restricted {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    // Page/action authority is ACL-based, including independent user additions.
+    // The production page_acl middleware already checks the same capability.
+    let actor: Option<String> =
+        sqlx::query_scalar("SELECT user_id FROM uac_login_session WHERE token_hash=$1")
+            .bind(digest(&raw))
+            .fetch_optional(&state.pool)
+            .await
+            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    Ok((actor.ok_or(StatusCode::UNAUTHORIZED)?, raw))
+}
+#[derive(Default, serde::Deserialize)]
+pub(crate) struct Notice {
+    created: Option<String>,
+}
+pub(crate) async fn page(
+    State(state): State<IngestState>,
+    headers: HeaderMap,
+    Query(query): Query<Notice>,
+) -> Result<Response, StatusCode> {
+    let (_, raw) = account_manager(&state, &headers).await?;
+    let csrf = digest(&format!("create-account:{raw}"));
+    let message = if query.created.as_deref() == Some("1") {
+        "已建立唯讀帳號；請安全交付初始密碼，首次登入須改密碼。"
+    } else {
+        ""
+    };
+    let html=view!{
+        <html lang="zh-Hant"><head><title>"新增唯讀帳號"</title></head><body><main>
+        <h1>"新增唯讀帳號"</h1><p role="status">{message}</p>
+        <form method="post" action="/accounts">
+        <input type="hidden" name="csrf" value=csrf/>
+        <label>"帳號"<input name="username" required minlength="3" maxlength="64" autocomplete="off"/></label>
+        <label>"初始密碼"<input type="password" name="password" required minlength="8" maxlength="128" autocomplete="new-password"/></label>
+        <label>"Email"<input type="email" name="email" required maxlength="254"/></label>
+        <label><input type="checkbox" name="chat"/>"可讀 Chat"</label>
+        <label><input type="checkbox" name="pet"/>"可讀 Pet"</label>
+        <button type="submit">"建立唯讀帳號"</button></form>
+        <a href="/dashboard">"返回後台"</a>
+        </main></body></html>
+    }.to_html();
+    Ok(renewed(Html(html), &raw))
+}
+#[derive(serde::Deserialize)]
+pub(crate) struct NewViewer {
+    username: String,
+    password: String,
+    email: String,
+    chat: Option<String>,
+    pet: Option<String>,
+    csrf: String,
+}
+fn db_error(e: sqlx::Error) -> StatusCode {
+    if let sqlx::Error::Database(d) = &e {
+        if d.is_unique_violation() {
+            return StatusCode::CONFLICT;
+        }
+    }
+    StatusCode::SERVICE_UNAVAILABLE
+}
+pub(crate) async fn create(
+    State(state): State<IngestState>,
+    headers: HeaderMap,
+    Form(form): Form<NewViewer>,
+) -> Result<Response, StatusCode> {
+    let (actor, raw) = account_manager(&state, &headers).await?;
+    if headers.get(header::ORIGIN).and_then(|h| h.to_str().ok()) != Some("http://127.0.0.1:3100")
+        || form.csrf != digest(&format!("create-account:{raw}"))
+    {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    let result = create_inner(&state, &actor, &raw, form).await;
+    if let Err(status) = &result {
+        sqlx::query("INSERT INTO analytics_account_audit(id,actor_id,target_id,action,result) VALUES($1,$2,'uncreated','create_viewer_rejected',$3)")
+            .bind(uuid::Uuid::new_v4().to_string()).bind(&actor).bind(status.as_str())
+            .execute(&state.pool).await.map_err(db_error)?;
+    }
+    result
+}
+
+async fn create_inner(
+    state: &IngestState,
+    actor: &str,
+    raw: &str,
+    form: NewViewer,
+) -> Result<Response, StatusCode> {
+    let username = form.username.trim().to_ascii_lowercase();
+    let email = form.email.trim().to_ascii_lowercase();
+    let parts: Vec<_> = email.split('@').collect();
+    if !(3..=64).contains(&username.len())
+        || !username
+            .bytes()
+            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
+        || !(8..=128).contains(&form.password.len())
+        || email.len() > 254
+        || !email.is_ascii()
+        || email
+            .bytes()
+            .any(|b| b.is_ascii_whitespace() || b.is_ascii_control())
+        || parts.len() != 2
+        || parts[0].is_empty()
+        || !parts[1].contains('.')
+        || parts[1].starts_with('.')
+        || parts[1].ends_with('.')
+        || (form.chat.is_none() && form.pet.is_none())
+        || form.chat.as_deref().is_some_and(|v| v != "on")
+        || form.pet.as_deref().is_some_and(|v| v != "on")
+    {
+        return Err(StatusCode::UNPROCESSABLE_ENTITY);
+    }
+    let encoded = tokio::task::spawn_blocking(move || {
+        Argon2::default()
+            .hash_password(form.password.as_bytes())
+            .map(|h| h.to_string())
+    })
+    .await
+    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
+    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    let id = uuid::Uuid::new_v4().to_string();
+    let mut tx = state.pool.begin().await.map_err(db_error)?;
+    // Recheck effective page/action permission before mutating account and grants.
+    // Lock the user/session/ACL identity while rechecking current effective permission.
+    // User allow can add this capability independently; user deny wins over every role.
+    let authorized:Option<(String,String)>=sqlx::query_as(
+        "SELECT u.user_pd,s.credential_fingerprint FROM uac_user u
+         JOIN uac_login_session s ON s.user_id=u.id
+         JOIN uac_acl a ON a.id='analytics-accounts' AND a.acl_status='active'
+         WHERE u.id=$1 AND s.token_hash=$2 AND s.expires_at>floor(extract(epoch from now()))::bigint
+         AND s.restricted=FALSE AND u.user_status='active' AND u.is_init_pd=FALSE
+         AND (EXISTS(SELECT 1 FROM uac_user_role ur JOIN uac_role r ON r.id=ur.role_id
+              JOIN uac_role_acl ra ON ra.role_id=r.id AND ra.acl_id=a.id
+              WHERE ur.user_id=u.id AND r.role_status='active')
+              OR EXISTS(SELECT 1 FROM uac_user_acl ua WHERE ua.user_id=u.id AND ua.acl_id=a.id AND ua.acl_filter='allow'))
+         AND NOT EXISTS(SELECT 1 FROM uac_user_acl ua JOIN uac_acl denied ON denied.id=ua.acl_id
+              WHERE ua.user_id=u.id AND denied.acl_path='/accounts/new' AND ua.acl_filter='deny')
+         FOR SHARE OF u,s,a")
+        .bind(actor).bind(digest(raw)).fetch_optional(&mut *tx).await.map_err(db_error)?;
+    let (credential, fingerprint) = authorized.ok_or(StatusCode::FORBIDDEN)?;
+    if digest(&credential) != fingerprint {
+        return Err(StatusCode::UNAUTHORIZED);
+    }
+    sqlx::query("INSERT INTO uac_user(id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at) VALUES($1,$2,$3,$4,$2,'active',TRUE,now(),now())")
+        .bind(&id).bind(username).bind(encoded).bind(email).execute(&mut *tx).await.map_err(db_error)?;
+    sqlx::query(
+        "INSERT INTO uac_user_role(user_id,role_id,created_at) VALUES($1,'analytics-viewer',now())",
+    )
+    .bind(&id)
+    .execute(&mut *tx)
+    .await
+    .map_err(db_error)?;
+    for (enabled, acl) in [
+        (form.chat.is_some(), "analytics-chat"),
+        (form.pet.is_some(), "analytics-pet"),
+    ] {
+        if enabled {
+            sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES($1,$2,'allow',now())").bind(&id).bind(acl).execute(&mut *tx).await.map_err(db_error)?;
+        }
+    }
+    sqlx::query("INSERT INTO analytics_account_audit(id,actor_id,target_id,action,result) VALUES($1,$2,$3,'create_viewer','created')")
+        .bind(uuid::Uuid::new_v4().to_string()).bind(actor).bind(&id).execute(&mut *tx).await.map_err(db_error)?;
+    tx.commit().await.map_err(db_error)?;
+    Ok(renewed(Redirect::to("/accounts/new?created=1"), &raw))
+}
diff --git a/analytics_apps/analytics_server/src/bootstrap.rs b/analytics_apps/analytics_server/src/bootstrap.rs
index 365f840..976fc74 100644
--- a/analytics_apps/analytics_server/src/bootstrap.rs
+++ b/analytics_apps/analytics_server/src/bootstrap.rs
@@ -48,6 +48,8 @@ pub async fn initialize(pool: &sqlx::PgPool, password: String) -> Result<String,
         .bind(hash)
         .execute(&mut *tx).await.map_err(|_| "管理員寫入失敗")?;
 
+    sqlx::query("INSERT INTO uac_user_role(user_id,role_id,created_at) VALUES($1,'analytics-admin',now()) ON CONFLICT DO NOTHING")
+        .bind(&id).execute(&mut *tx).await.map_err(|_| "管理員角色寫入失敗")?;
     tx.commit().await.map_err(|_| "管理員提交失敗")?;
 
     Ok(id)
@@ -72,6 +74,8 @@ pub async fn ensure_default(pool: &sqlx::PgPool) -> Result<String, String> {
         return Err("管理員身分重複；停止初始化，需人工查核".into());
     }
     if let Some((id,)) = admins.into_iter().next() {
+        sqlx::query("INSERT INTO uac_user_role(user_id,role_id,created_at) VALUES($1,'analytics-admin',now()) ON CONFLICT DO NOTHING")
+        .bind(&id).execute(&mut *tx).await.map_err(|_| "管理員角色寫入失敗")?;
         tx.commit()
             .await
             .map_err(|_| "初始化查核提交失敗".to_owned())?;
@@ -99,6 +103,8 @@ pub async fn ensure_default(pool: &sqlx::PgPool) -> Result<String, String> {
          VALUES ($1,'admin',$2,'admin@localhost.invalid','管理員','active',TRUE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)"
     ).bind(&id).bind(hash).execute(&mut *tx).await
         .map_err(|_| "管理員寫入失敗".to_owned())?;
+    sqlx::query("INSERT INTO uac_user_role(user_id,role_id,created_at) VALUES($1,'analytics-admin',now()) ON CONFLICT DO NOTHING")
+        .bind(&id).execute(&mut *tx).await.map_err(|_| "管理員角色寫入失敗")?;
     tx.commit().await.map_err(|_| "管理員提交失敗".to_owned())?;
     Ok(id)
 }
diff --git a/analytics_apps/analytics_server/src/dashboard_ui.rs b/analytics_apps/analytics_server/src/dashboard_ui.rs
index afe7f08..c5621d0 100644
--- a/analytics_apps/analytics_server/src/dashboard_ui.rs
+++ b/analytics_apps/analytics_server/src/dashboard_ui.rs
@@ -89,7 +89,8 @@ fn DashboardView(
                             "查詢完成"
                         }}
                     </p>
-                    <p>"目前僅支援管理員；角色與來源 ACL 尚待接通。"</p>
+                    <a href="/accounts/new">"帳號管理：新增唯讀帳號"</a>
+                    <p>"來源查詢由伺服器依角色及授權範圍檢查。"</p>
                     <form method="post" action="/logout">
                         <input type="hidden" name="csrf" value=logout_csrf/>
                         <button type="submit">"登出"</button>
diff --git a/analytics_apps/analytics_server/src/export.rs b/analytics_apps/analytics_server/src/export.rs
index c62aa32..d9f888a 100644
--- a/analytics_apps/analytics_server/src/export.rs
+++ b/analytics_apps/analytics_server/src/export.rs
@@ -20,7 +20,7 @@ pub async fn export_summary(
     Query(query): Query<ExportQuery>,
     headers: HeaderMap,
 ) -> Result<Response, StatusCode> {
-    let raw = crate::password_login::require_query_session(&state, &headers).await?;
+    let raw = crate::password_login::require_source_session(&state, &headers, &source).await?;
     if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
         return Err(StatusCode::BAD_REQUEST);
     }
diff --git a/analytics_apps/analytics_server/src/lib.rs b/analytics_apps/analytics_server/src/lib.rs
index 5d12b20..0557173 100644
--- a/analytics_apps/analytics_server/src/lib.rs
+++ b/analytics_apps/analytics_server/src/lib.rs
@@ -1,3 +1,4 @@
+mod accounts;
 pub mod admin;
 pub mod bootstrap;
 mod csv;
@@ -5,6 +6,7 @@ mod dashboard_summary;
 pub mod dashboard_ui;
 mod dashboard_url;
 pub mod export;
+mod page_acl;
 pub mod password_login;
 pub mod pg_store;
 mod session_cookie;
@@ -200,7 +202,7 @@ async fn source_kind_summary(
     headers: HeaderMap,
 ) -> Result<axum::response::Response, StatusCode> {
     // 先驗人員 Session，再驗來源／事件類型，最後才碰統計資料。
-    let raw = password_login::require_query_session(&state, &headers).await?;
+    let raw = password_login::require_source_session(&state, &headers, &source).await?;
     if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
         return Err(StatusCode::BAD_REQUEST);
     }
diff --git a/analytics_apps/analytics_server/src/page_acl.rs b/analytics_apps/analytics_server/src/page_acl.rs
new file mode 100644
index 0000000..7dbaff4
--- /dev/null
+++ b/analytics_apps/analytics_server/src/page_acl.rs
@@ -0,0 +1,68 @@
+use crate::{
+    IngestState,
+    password_login::{digest, session},
+};
+use axum::{
+    extract::{MatchedPath, Request, State},
+    http::StatusCode,
+    middleware::Next,
+    response::{IntoResponse, Redirect, Response},
+};
+
+/// Templates are the same keys registered by the production Router and UAC ACL.
+pub(crate) async fn require(
+    State(state): State<IngestState>,
+    request: Request,
+    next: Next,
+) -> Response {
+    let route = request
+        .extensions()
+        .get::<MatchedPath>()
+        .map(|p| p.as_str())
+        .unwrap_or(request.uri().path());
+    let acl = match route {
+        "/" | "/dashboard" => "/dashboard",
+        "/accounts" | "/accounts/new" => "/accounts/new",
+        "/change-password" => "/change-password",
+        "/api/v1/events/{source}/summary"
+        | "/api/v1/events/{source}/summary/{event_type}"
+        | "/api/v1/events/{source}/export.csv" => route,
+        // Login/password recovery, logout, health and service ingest are separate contracts.
+        _ => return next.run(request).await,
+    };
+    let (restricted, raw) = match session(&state, request.headers()).await {
+        Ok(v) => v,
+        Err(StatusCode::UNAUTHORIZED) if acl == "/dashboard" => {
+            return Redirect::to("/login").into_response();
+        }
+        Err(e) => return e.into_response(),
+    };
+    // The existing dashboard sends first-login users to the mandatory password form.
+    if restricted && acl == "/dashboard" {
+        return next.run(request).await;
+    }
+    let permitted = allowed(&state, &raw, acl).await;
+    match permitted {
+        Ok(true) => next.run(request).await,
+        Ok(false) => (StatusCode::FORBIDDEN, "此角色沒有此頁面或功能的 ACL 權限。").into_response(),
+        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
+    }
+}
+
+pub(crate) async fn allowed(
+    state: &IngestState,
+    raw: &str,
+    path: &str,
+) -> Result<bool, sqlx::Error> {
+    sqlx::query_scalar(
+        "SELECT EXISTS(SELECT 1 FROM uac_login_session s JOIN uac_user u ON u.id=s.user_id
+         JOIN uac_acl a ON a.acl_path=$2 AND a.acl_status='active'
+         WHERE s.token_hash=$1 AND u.user_status='active'
+         AND (EXISTS(SELECT 1 FROM uac_user_role ur JOIN uac_role r ON r.id=ur.role_id
+          JOIN uac_role_acl ra ON ra.role_id=r.id AND ra.acl_id=a.id
+          WHERE ur.user_id=u.id AND r.role_status='active')
+          OR EXISTS(SELECT 1 FROM uac_user_acl ua WHERE ua.user_id=u.id AND ua.acl_id=a.id AND ua.acl_filter='allow'))
+         AND NOT EXISTS(SELECT 1 FROM uac_user_acl ua JOIN uac_acl denied ON denied.id=ua.acl_id
+          WHERE ua.user_id=u.id AND denied.acl_path=$2 AND ua.acl_filter='deny'))")
+        .bind(digest(&raw)).bind(path).fetch_one(&state.pool).await
+}
diff --git a/analytics_apps/analytics_server/src/password_login.rs b/analytics_apps/analytics_server/src/password_login.rs
index 3a6d662..ab96864 100644
--- a/analytics_apps/analytics_server/src/password_login.rs
+++ b/analytics_apps/analytics_server/src/password_login.rs
@@ -15,7 +15,7 @@ use sha2::{Digest, Sha256};
 
 const ORIGIN: &str = "http://127.0.0.1:3100";
 
-fn digest(value: &str) -> String {
+pub(crate) fn digest(value: &str) -> String {
     Sha256::digest(value.as_bytes())
         .iter()
         .map(|b| format!("{b:02x}"))
@@ -54,22 +54,29 @@ async fn login(
         return Err(StatusCode::FORBIDDEN);
     }
 
-    if !form.username.trim().eq_ignore_ascii_case("admin")
+    if form.username.len() > 64
+        || form.username.trim().is_empty()
+        || !form
+            .username
+            .trim()
+            .bytes()
+            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
         || !(form.password == "admin" || (8..=128).contains(&form.password.len()))
     {
         return Err(StatusCode::UNAUTHORIZED);
     }
 
-    // Existing bootstrap only provisions admin. Reject duplicate case-folded identities rather than guessing one.
+    // Reject duplicate case-folded identities rather than guessing one.
     let found: Vec<(String, String, bool, String)> = sqlx::query_as(
         r#"SELECT id
             ,user_pd
             ,is_init_pd
             ,user_status 
         FROM uac_user 
-        WHERE lower(user_acnt)='admin' 
+        WHERE lower(user_acnt)=lower($1) 
         LIMIT 2"#,
     )
+    .bind(form.username.trim())
     .fetch_all(&state.pool)
     .await
     .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
@@ -140,7 +147,10 @@ async fn login(
 }
 
 // Recheck account status, mandatory-change state and password fingerprint on each protected request.
-async fn session(state: &IngestState, headers: &HeaderMap) -> Result<(bool, String), StatusCode> {
+pub(crate) async fn session(
+    state: &IngestState,
+    headers: &HeaderMap,
+) -> Result<(bool, String), StatusCode> {
     let raw = raw_cookie(headers).ok_or(StatusCode::UNAUTHORIZED)?;
     let found = analytics_persistence::login_session::lookup(&state.pool, &digest(&raw))
         .await
@@ -174,16 +184,30 @@ async fn session(state: &IngestState, headers: &HeaderMap) -> Result<(bool, Stri
     Ok((restricted || must_change, raw))
 }
 
-/// 已完成首次改密碼的有效管理者，才可讀取業務統計。
-/// 角色與來源 ACL 尚未建置，不得冒稱檢視者已獲授權。
-pub(crate) async fn require_query_session(
+/// A live person Session and source grant are required on every business query.
+pub(crate) async fn require_source_session(
     state: &IngestState,
     headers: &HeaderMap,
+    source: &str,
 ) -> Result<String, StatusCode> {
     let (restricted, raw) = session(state, headers).await?;
     if restricted {
         return Err(StatusCode::FORBIDDEN);
     }
+    if !matches!(source, "web_chat" | "desktop_pet") {
+        return Err(StatusCode::BAD_REQUEST);
+    }
+    let path = match source {
+        "web_chat" => "/api/v1/events/web_chat",
+        "desktop_pet" => "/api/v1/events/desktop_pet",
+        _ => unreachable!(),
+    };
+    let allowed = crate::page_acl::allowed(state, &raw, path)
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    if !allowed {
+        return Err(StatusCode::FORBIDDEN);
+    }
     Ok(raw)
 }
 
@@ -260,7 +284,6 @@ async fn change_password(
         WHERE s.token_hash=$1 
             AND s.expires_at>floor(extract(epoch from now()))::bigint 
             AND u.user_status='active' 
-            AND lower(u.user_acnt)='admin' 
         FOR UPDATE OF u,s"#,
     )
     .bind(digest(&raw))
@@ -341,12 +364,24 @@ async fn dashboard(
         return renewed(Redirect::to("/change-password"), &raw);
     }
 
-    let source = query.source.as_deref().unwrap_or("web_chat");
+    let default_source = if query.source.is_none() {
+        match require_source_session(&state, &headers, "web_chat").await {
+            Ok(_) => "web_chat",
+            Err(StatusCode::FORBIDDEN) => "desktop_pet",
+            Err(status) => return status.into_response(),
+        }
+    } else {
+        "web_chat"
+    };
+    let source = query.source.as_deref().unwrap_or(default_source);
     let source_label = match source {
         "web_chat" => "Chat",
         "desktop_pet" => "Pet",
         _ => return StatusCode::BAD_REQUEST.into_response(),
     };
+    if let Err(status) = require_source_session(&state, &headers, source).await {
+        return (status, "您沒有此來源的檢視權限。").into_response();
+    }
     let kind = query.event_type.as_deref().unwrap_or("");
     if !kind.is_empty() && (!crate::valid_event_type(kind) || kind.contains('\0')) {
         return StatusCode::BAD_REQUEST.into_response();
@@ -383,7 +418,7 @@ async fn summary(
     Path(source): Path<String>,
     headers: HeaderMap,
 ) -> Result<Response, StatusCode> {
-    let raw = require_query_session(&state, &headers).await?;
+    let raw = require_source_session(&state, &headers, &source).await?;
     if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
         return Err(StatusCode::BAD_REQUEST);
     }
@@ -430,6 +465,8 @@ async fn logout(
 
 pub fn build(state: IngestState) -> Router {
     Router::new()
+        .route("/accounts/new", get(crate::accounts::page))
+        .route("/accounts", post(crate::accounts::create))
         .route("/login", get(login_page).post(login))
         .route("/logout", post(logout))
         .route("/change-password", get(change_page).post(change_password))
@@ -446,6 +483,10 @@ pub fn build(state: IngestState) -> Router {
         )
         .route("/healthz", get(crate::health))
         .route("/api/v1/events", post(crate::ingest))
+        .layer(axum::middleware::from_fn_with_state(
+            state.clone(),
+            crate::page_acl::require,
+        ))
         .layer(DefaultBodyLimit::max(4096))
         .layer(axum::middleware::map_response(
             |mut r: Response| async move {
diff --git a/analytics_apps/analytics_server/tests/day45_viewer.rs b/analytics_apps/analytics_server/tests/day45_viewer.rs
new file mode 100644
index 0000000..ba6f8a5
--- /dev/null
+++ b/analytics_apps/analytics_server/tests/day45_viewer.rs
@@ -0,0 +1,285 @@
+#![cfg(feature = "postgres-integration")]
+mod support;
+use analytics_server::{IngestState, IngestTokens, bootstrap, password_login};
+use axum::http::{StatusCode, header};
+use std::sync::Arc;
+use support::http::{body_text, response_cookie, send};
+
+fn token(html: &str, action: &str) -> String {
+    html.split(&format!("action=\"{action}\""))
+        .nth(1)
+        .unwrap()
+        .split("name=\"csrf\" value=\"")
+        .nth(1)
+        .unwrap()
+        .split('"')
+        .next()
+        .unwrap()
+        .into()
+}
+#[sqlx::test(migrations = "../migrations")]
+async fn admin_creates_viewer_first_change_and_source_revoke_affects_same_session(
+    pool: sqlx::PgPool,
+) {
+    let admin = bootstrap::initialize(&pool, "lesson-password".into())
+        .await
+        .unwrap();
+    sqlx::query("UPDATE uac_user SET is_init_pd=FALSE WHERE id=$1")
+        .bind(&admin)
+        .execute(&pool)
+        .await
+        .unwrap();
+    let app = password_login::build(IngestState {
+        pool: pool.clone(),
+        tokens: Arc::new(IngestTokens {
+            chat: "chat-fixture".into(),
+            pet: "pet-fixture".into(),
+        }),
+    });
+    let login = send(
+        app.clone(),
+        "/login",
+        Some("username=admin&password=lesson-password"),
+        None,
+    )
+    .await;
+    assert_eq!(login.status(), StatusCode::SEE_OTHER);
+    let cookie = response_cookie(&login);
+    let page = send(app.clone(), "/accounts/new", None, Some(&cookie)).await;
+    assert_eq!(page.status(), StatusCode::OK);
+    let csrf = token(&body_text(page).await, "/accounts");
+    let form = format!(
+        "username=reader&password=reader-password&email=reader%40example.test&chat=on&csrf={csrf}"
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/accounts",
+            Some(&form.replace(&csrf, "wrong")),
+            Some(&cookie)
+        )
+        .await
+        .status(),
+        StatusCode::FORBIDDEN
+    );
+    let created = send(app.clone(), "/accounts", Some(&form), Some(&cookie)).await;
+    assert_eq!(created.status(), StatusCode::SEE_OTHER);
+    assert_eq!(
+        created.headers()[header::LOCATION],
+        "/accounts/new?created=1"
+    );
+    let encoded: String =
+        sqlx::query_scalar("SELECT user_pd FROM uac_user WHERE user_acnt='reader'")
+            .fetch_one(&pool)
+            .await
+            .unwrap();
+    assert!(encoded.starts_with("$argon2"));
+    assert!(!encoded.contains("reader-password"));
+    assert_eq!(
+        send(app.clone(), "/accounts", Some(&form), Some(&cookie))
+            .await
+            .status(),
+        StatusCode::CONFLICT
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/accounts",
+            Some(&form.replace("username=reader", "username=reader2")),
+            Some(&cookie)
+        )
+        .await
+        .status(),
+        StatusCode::CONFLICT
+    );
+    let orphan: i64 = sqlx::query_scalar("SELECT count(*) FROM uac_user WHERE user_acnt='reader2'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(orphan, 0);
+    let login = send(
+        app.clone(),
+        "/login",
+        Some("username=reader&password=reader-password"),
+        None,
+    )
+    .await;
+    assert_eq!(login.status(), StatusCode::SEE_OTHER);
+    assert_eq!(login.headers()[header::LOCATION], "/change-password");
+    let reader = response_cookie(&login);
+    assert_eq!(
+        send(
+            app.clone(),
+            "/api/v1/events/web_chat/summary",
+            None,
+            Some(&reader)
+        )
+        .await
+        .status(),
+        StatusCode::FORBIDDEN
+    );
+    let page = body_text(send(app.clone(), "/change-password", None, Some(&reader)).await).await;
+    let csrf = token(&page, "/change-password");
+    let changed=send(app.clone(),"/change-password",Some(&format!("current_password=reader-password&new_password=changed-password&confirm_password=changed-password&email=reader%40example.test&csrf={csrf}")),Some(&reader)).await;
+    assert_eq!(changed.status(), StatusCode::SEE_OTHER);
+    assert_eq!(
+        send(app.clone(), "/dashboard", None, Some(&reader))
+            .await
+            .status(),
+        StatusCode::SEE_OTHER
+    );
+    let reader = response_cookie(
+        &send(
+            app.clone(),
+            "/login",
+            Some("username=reader&password=changed-password"),
+            None,
+        )
+        .await,
+    );
+    for path in [
+        "/dashboard?source=web_chat",
+        "/api/v1/events/web_chat/summary",
+        "/api/v1/events/web_chat/summary/chat.message.persisted",
+        "/api/v1/events/web_chat/export.csv",
+    ] {
+        assert_eq!(
+            send(app.clone(), path, None, Some(&reader)).await.status(),
+            StatusCode::OK,
+            "{path}"
+        );
+    }
+    for path in [
+        "/dashboard?source=desktop_pet",
+        "/api/v1/events/desktop_pet/summary",
+        "/api/v1/events/desktop_pet/summary/pet.sleep",
+        "/api/v1/events/desktop_pet/export.csv",
+        "/accounts/new",
+    ] {
+        assert_eq!(
+            send(app.clone(), path, None, Some(&reader)).await.status(),
+            StatusCode::FORBIDDEN,
+            "{path}"
+        );
+    }
+    let csrf = token(
+        &body_text(send(app.clone(), "/change-password", None, Some(&reader)).await).await,
+        "/logout",
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/accounts",
+            Some(&format!(
+                "username=evil&password=evil-password&email=evil%40example.test&chat=on&csrf={csrf}"
+            )),
+            Some(&reader)
+        )
+        .await
+        .status(),
+        StatusCode::FORBIDDEN
+    );
+    sqlx::query(
+        "DELETE FROM uac_user_acl WHERE user_id=(SELECT id FROM uac_user WHERE user_acnt='reader')",
+    )
+    .execute(&pool)
+    .await
+    .unwrap();
+    assert_eq!(
+        send(
+            app.clone(),
+            "/api/v1/events/web_chat/export.csv",
+            None,
+            Some(&reader)
+        )
+        .await
+        .status(),
+        StatusCode::FORBIDDEN
+    );
+    // A Pet-only viewer can enter the default dashboard after a fresh login too.
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) SELECT id,'analytics-pet','allow',now() FROM uac_user WHERE user_acnt='reader'")
+        .execute(&pool).await.unwrap();
+    let page = send(app.clone(), "/dashboard", None, Some(&reader)).await;
+    assert_eq!(page.status(), StatusCode::OK);
+    assert!(body_text(page).await.contains("Pet 事件"));
+    // Independent user addition can grant a capability absent from the viewer role.
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) SELECT id,'analytics-accounts','allow',now() FROM uac_user WHERE user_acnt='reader'").execute(&pool).await.unwrap();
+    let delegated = send(app.clone(), "/accounts/new", None, Some(&reader)).await;
+    assert_eq!(delegated.status(), StatusCode::OK);
+    let csrf = token(&body_text(delegated).await, "/accounts");
+    let created=send(app.clone(),"/accounts",Some(&format!("username=delegated&password=delegated-password&email=delegated%40example.test&pet=on&csrf={csrf}")),Some(&reader)).await;
+    assert_eq!(created.status(), StatusCode::SEE_OTHER);
+    sqlx::query("UPDATE uac_user_acl SET acl_filter='deny' WHERE acl_id='analytics-accounts' AND user_id=(SELECT id FROM uac_user WHERE user_acnt='reader')").execute(&pool).await.unwrap();
+    assert_eq!(
+        send(app.clone(), "/accounts/new", None, Some(&reader))
+            .await
+            .status(),
+        StatusCode::FORBIDDEN
+    );
+    assert_eq!(send(app.clone(),"/accounts",Some(&format!("username=denied&password=denied-password&email=denied%40example.test&pet=on&csrf={csrf}")),Some(&reader)).await.status(),StatusCode::FORBIDDEN);
+    // Page permissions are role->ACL, independent from source grants.
+    sqlx::query("DELETE FROM uac_role_acl WHERE role_id='analytics-viewer' AND acl_id='analytics-dashboard'").execute(&pool).await.unwrap();
+    assert_eq!(
+        send(app.clone(), "/dashboard", None, Some(&reader))
+            .await
+            .status(),
+        StatusCode::FORBIDDEN
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/api/v1/events/desktop_pet/summary",
+            None,
+            Some(&reader)
+        )
+        .await
+        .status(),
+        StatusCode::OK
+    );
+    sqlx::query("INSERT INTO uac_role_acl(role_id,acl_id,created_at) VALUES('analytics-viewer','analytics-dashboard',now())").execute(&pool).await.unwrap();
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) SELECT id,'analytics-dashboard','deny',now() FROM uac_user WHERE user_acnt='reader'").execute(&pool).await.unwrap();
+    assert_eq!(
+        send(app.clone(), "/dashboard", None, Some(&reader))
+            .await
+            .status(),
+        StatusCode::FORBIDDEN
+    );
+    sqlx::query("UPDATE uac_user SET user_status='disabled' WHERE user_acnt='reader'")
+        .execute(&pool)
+        .await
+        .unwrap();
+    assert_eq!(
+        send(
+            app.clone(),
+            "/api/v1/events/desktop_pet/summary",
+            None,
+            Some(&reader)
+        )
+        .await
+        .status(),
+        StatusCode::UNAUTHORIZED
+    );
+    let rejected:i64=sqlx::query_scalar("SELECT count(*) FROM analytics_account_audit WHERE action='create_viewer_rejected' AND result='409'").fetch_one(&pool).await.unwrap();
+    assert_eq!(rejected, 2);
+    // A per-user subtraction also applies to an administrator's default source grant.
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES($1,'analytics-chat','deny',now())").bind(&admin).execute(&pool).await.unwrap();
+    assert_eq!(
+        send(
+            app.clone(),
+            "/api/v1/events/web_chat/summary",
+            None,
+            Some(&cookie)
+        )
+        .await
+        .status(),
+        StatusCode::FORBIDDEN
+    );
+    let audit: i64 = sqlx::query_scalar(
+        "SELECT count(*) FROM analytics_account_audit WHERE actor_id=$1 AND action='create_viewer'",
+    )
+    .bind(admin)
+    .fetch_one(&pool)
+    .await
+    .unwrap();
+    assert_eq!(audit, 1);
+}
diff --git a/analytics_apps/migrations/20261031074500_viewer_access.sql b/analytics_apps/migrations/20261031074500_viewer_access.sql
new file mode 100644
index 0000000..80769d5
--- /dev/null
+++ b/analytics_apps/migrations/20261031074500_viewer_access.sql
@@ -0,0 +1,31 @@
+-- Additive access policy using the existing UAC role/ACL tables.
+CREATE UNIQUE INDEX uq_uac_account_day45 ON uac_user(lower(trim(user_acnt)));
+INSERT INTO uac_role(id,role_name,role_status,created_at,updated_at) VALUES
+('analytics-admin','system_admin','active',now(),now()),
+('analytics-viewer','viewer','active',now(),now());
+INSERT INTO uac_acl(id,acl_name,acl_path,acl_category,acl_status,created_at,updated_at) VALUES
+('analytics-chat','Chat read','/api/v1/events/web_chat','action','active',now(),now()),
+('analytics-pet','Pet read','/api/v1/events/desktop_pet','action','active',now(),now());
+INSERT INTO uac_user_role(user_id,role_id,created_at)
+SELECT id,'analytics-admin',now() FROM uac_user WHERE lower(user_acnt)='admin';
+CREATE TABLE analytics_account_audit (
+ id TEXT PRIMARY KEY, actor_id TEXT NOT NULL, target_id TEXT NOT NULL,
+ action TEXT NOT NULL, result TEXT NOT NULL, created_at TIMESTAMPTZ NOT NULL DEFAULT now()
+);
+
+-- Role decides who the person is; ACL decides which page/action can be used.
+INSERT INTO uac_acl(id,acl_name,acl_path,acl_category,acl_status,created_at,updated_at) VALUES
+('analytics-dashboard','Dashboard','/dashboard','page','active',now(),now()),
+('analytics-accounts','Account management','/accounts/new','page','active',now(),now()),
+('analytics-password','Own password','/change-password','page','active',now(),now()),
+('analytics-summary','Summary query','/api/v1/events/{source}/summary','action','active',now(),now()),
+('analytics-kind','Kind query','/api/v1/events/{source}/summary/{event_type}','action','active',now(),now()),
+('analytics-export','CSV export','/api/v1/events/{source}/export.csv','action','active',now(),now());
+INSERT INTO uac_role_acl(role_id,acl_id,created_at)
+SELECT 'analytics-admin',id,now() FROM uac_acl WHERE id IN
+('analytics-dashboard','analytics-accounts','analytics-password','analytics-summary','analytics-kind','analytics-export');
+INSERT INTO uac_role_acl(role_id,acl_id,created_at)
+SELECT 'analytics-viewer',id,now() FROM uac_acl WHERE id IN
+('analytics-dashboard','analytics-password','analytics-summary','analytics-kind','analytics-export');
+
+INSERT INTO uac_role_acl(role_id,acl_id,created_at) VALUES('analytics-admin','analytics-chat',now()),('analytics-admin','analytics-pet',now());
```

### r2：role、ACL、uac_user_acl 的完整語意與新增 Red

使用者確認：`uac_role`區分角色，`uac_acl`定義功能／頁面，`uac_role_acl`給角色預設權限，`uac_user_acl`可以獨立增加或減少個人權限。有效授權為 **active角色的預設允許 或 個人allow，且沒有個人deny**；沒有允許則拒絕。deny優先也適用管理員。身分仍須active帳號、有效Session與既有role分類；個人加頁面權限不會偷偷修改角色名稱。

本版把Dashboard、帳號管理、自身密碼、summary、kind與CSV各自登錄在ACL；Router middleware以正式MatchedPath template查當下授權。來源授權也是對應action ACL，與頁面ACL分別查核。新增page_acl::allowed共用同案有效權限判斷，不以admin帳號字串或第二套硬式role gate蓋掉個人加權。

例如viewer預設沒有帳號管理頁；加`uac_user_acl(analytics-accounts,allow)`後可開表單並執行其對應建立功能，角色仍viewer；改為deny後同一Cookie的GET及POST都403。管理員被個人deny某來源亦403，並非管理員永遠略過個人減權。本篇沒有自動授予此例外，只有測試在隔離PG明確設定；管理這些加減權的UI與稽核流程仍是後續工作。

新增行為Red（在原r1 Green候選上套，尚無page ACL判定）：刪除viewer Dashboard role_acl後仍200，期待403；`analytics-acl-red.log` exit101。以下是當時的完整增量測試patch；r2完整Green已包含新測試及個人獨立加減／管理員deny的後續斷言，不要重複套。

```diff
diff --git a/analytics_apps/analytics_server/tests/day45_viewer.rs b/analytics_apps/analytics_server/tests/day45_viewer.rs
--- a/analytics_apps/analytics_server/tests/day45_viewer.rs
+++ b/analytics_apps/analytics_server/tests/day45_viewer.rs
@@ -202,6 +202,13 @@
     let page = send(app.clone(), "/dashboard", None, Some(&reader)).await;
     assert_eq!(page.status(), StatusCode::OK);
     assert!(body_text(page).await.contains("Pet 事件"));
+    // Page permissions are role->ACL, independent from source grants.
+    sqlx::query("DELETE FROM uac_role_acl WHERE role_id='analytics-viewer' AND acl_id='analytics-dashboard'").execute(&pool).await.unwrap();
+    assert_eq!(send(app.clone(),"/dashboard",None,Some(&reader)).await.status(),StatusCode::FORBIDDEN);
+    assert_eq!(send(app.clone(),"/api/v1/events/desktop_pet/summary",None,Some(&reader)).await.status(),StatusCode::OK);
+    sqlx::query("INSERT INTO uac_role_acl(role_id,acl_id,created_at) VALUES('analytics-viewer','analytics-dashboard',now())").execute(&pool).await.unwrap();
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) SELECT id,'analytics-dashboard','deny',now() FROM uac_user WHERE user_acnt='reader'").execute(&pool).await.unwrap();
+    assert_eq!(send(app.clone(),"/dashboard",None,Some(&reader)).await.status(),StatusCode::FORBIDDEN);
     sqlx::query("UPDATE uac_user SET user_status='disabled' WHERE user_acnt='reader'")
         .execute(&pool)
         .await
```

來源精確SHA仍64eff527，未套用r1；本r2 Green是對同一來源＋Refactor的**完整替代patch**，不能接在r1 Green後重複套。如果使用者之後已套r1，不可改已執行migration checksum，須另做向前migration；本次並未操作那種資料庫。原文章／patch／commit／讀回證據均保留。本次需求正文亦補上role／ACL／個人加減權契約，沒有宣稱產品已套用。

### 斷言承接、故障與補教

原登入／密碼／Session／CSV／logout斷言不刪。新旅程驗證雜湊非明碼、重複Email／帳號409、wrong-CSRF拒絕、首次改密碼前業務403、修改密碼後舊Session失效、四種Chat入口200／四種Pet入口403、未獲獨立授權的viewer不能建帳號、撤銷grant立即403、Pet-only預設可讀與停用401、成功／失敗稽核。這些都經正式Router與真PG；沒有以source string helper測試替代流程。

Day44 logout CSRF及其PG fixture已在產品，不重複套舊patch；本篇全套回歸保留。Analytics無較早pending FIFO；Chat／Pet待續項不挪入本篇、不重複計功。尚未新增角色／來源編輯UI與郵件重設，後續應沿原需求補上，不能把DB測試直接改grant說成管理介面完成。

## ③ 2026-10-31 官方穩定版與相容性

Rust/Cargo 本機 1.99.1；[官方 Rust 1.99.1（2026-10-15）](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)。下表當日直接查 crates.io 官方 registry，排除預發行及 yanked；完整 JSON、MSRV 與查核來源隨文保存。edition 2024；沒有升級 lock。

| 套件 | manifest | lock | 官方 stable | MSRV |
| --- | --- | --- | --- | --- |
| [argon2](https://crates.io/api/v1/crates/argon2) | 0.6 | 0.6.0 | 0.6.0 | 1.85 |
| [async-trait](https://crates.io/api/v1/crates/async-trait) | 0.1 | 0.1.92 | 0.1.92 | 1.71 |
| [axum](https://crates.io/api/v1/crates/axum) | 0.8 | 0.8.9 | 0.8.9 | 1.80 |
| [leptos](https://crates.io/api/v1/crates/leptos) | 0.8 | 0.8.22 | 0.8.22 | 1.88 |
| [leptos_config](https://crates.io/api/v1/crates/leptos_config) | 0.8 | 0.8.10 | 0.8.10 | 1.88 |
| [libsqlite3-sys](https://crates.io/api/v1/crates/libsqlite3-sys) | package manifest / transitive | 0.37.0 | 0.39.1 | 1.88.0 |
| [rpassword](https://crates.io/api/v1/crates/rpassword) | package manifest / transitive | 7.5.4 | 7.5.4 | 1.85 |
| [serde](https://crates.io/api/v1/crates/serde) | 1.0 | 1.0.229 | 1.0.229 | 1.56 |
| [serde_json](https://crates.io/api/v1/crates/serde_json) | 1.0 | 1.0.154 | 1.0.154 | 1.71 |
| [sha2](https://crates.io/api/v1/crates/sha2) | 0.11 | 0.11.1 | 0.11.1 | 1.88 |
| [sqlx](https://crates.io/api/v1/crates/sqlx) | 0.9 | 0.9.0 | 0.9.0 | 1.94.0 |
| [thiserror](https://crates.io/api/v1/crates/thiserror) | 2.0 | 2.0.21 | 2.0.21 | 1.77 |
| [tokio](https://crates.io/api/v1/crates/tokio) | 1.54 | 1.54.1 | 1.54.1 | 1.85 |
| [tower](https://crates.io/api/v1/crates/tower) | package manifest / transitive | 0.5.3 | 0.5.3 | 1.64.0 |
| [uuid](https://crates.io/api/v1/crates/uuid) | 1.29 | 1.29.0 | 1.29.0 | 1.89.0 |

SQLx 0.9 要求 Rust 1.94；Leptos 0.8 要求 1.88，目前編譯器符合。libsqlite3-sys lock 0.37.0 低於最新 0.39.1：SQLx SQLite driver 綁定其原生 links，不能單獨升級造成重複 sqlite3 links；本篇保留 SQLx 支援的 lock，升級須一起重核 driver、SQLite 編譯及舊庫。MSRV 是必要條件，不能據此推論 DB、WASM 或 GUI 已驗收。

[PostgreSQL官方支援版本](https://www.postgresql.org/support/versioning/)本日最新stable18.6，19RC1排除；本機隔離PG18.6與libpq18.6，SQLx0.9 postgres。測試只用新測試庫，沒有真實舊PG備份、無舊庫升級成功宣告。新增唯一index須先查歷史重複帳號；migration不改舊檔checksum，不在正式DB自動執行。Leptos維持0.8.22 SSR，Axum0.8.9；無Cargo／lock變更。官方JSON與四workspace精確manifest／lock blob與metadata另見source-inventory.json；metadata只證明依賴宣告，不等於build／DB驗收。

## ④ 實際驗證、交付與下一步

命令、exit、實際結果與失敗保存analytics-validation.json、analytics-validation-r2.json及各log。隔離PG首次Unix socket路徑超107bytes啟動失敗，改短路徑/tmp/codex-day45-pg成功；首次測試缺DATABASE_URL失敗，重跑取得真正404→200行為Red。Green旅程及全套PG回歸結果由日誌記錄，不用舊SHA或CI冒稱候選已驗收。來源SHA七workflow已成功，僅屬原產品。

正式main尚未套用本篇；Gate2=false。未執行瀏覽器真Leptos表單旅程、實際舊PG資料升級、角色／來源管理介面、即時授權撤銷、寄信重設、完整帳號稽核覆蓋。新表單雖以Leptos render，仍須瀏覽器鍵盤／錯誤／CSRF／安全交付初始密碼驗證；目前非成功回應沿既有HTTP錯誤，不回填密碼。

套用：精確來源→Refactor→Green，先git apply --check，備份測試資料並在隔離副本跑migration／以上測試，再由使用者決定產品採用。回復程式反序reverse，已執行的新migration不隨意刪表或改歷史checksum；須依備份與向前修訂方案處理。本篇只在copilot_use發布教學與patch，rust_ai_ring無提交。下一步補帳號停用／角色與來源調整的管理流程及其稽核，再驗既有Session／即時連線縮權；last_full_comparison保持原值。

## r2 驗證補記

`analytics-effective-acl-final.log`為最終來源＋Refactor＋r2候選全套回歸；其中新增獨立人員allow→可開頁並建立、改deny→GET／POST拒絕、角色Dashboard ACL撤銷但另有API權限不受影響、admin來源deny亦生效。真PG行為Red與最終結果分列，不追認r1結果。角色／人員權限編輯UI尚未交付；本篇只接通授權判斷與帳號建立操作。
