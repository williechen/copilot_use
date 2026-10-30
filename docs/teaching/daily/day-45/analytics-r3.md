# Analytics Day45 r3｜category → page → action 與 CRUD 權限

## ① 目標、來源與已確認契約

2026-10-31 使用者確認：Analytics ACL新增「會員管理、資訊分析、認證管理」三個category，階層為 **category → page → action**，action處理CRUD判斷。角色仍由uac_role區分、uac_role_acl給角色預設；uac_user_acl可獨立增加／減少個人的功能與頁面權限。

本篇是同一Day45的r3修訂，不發布Day46、不遞增next_day。文件parent `96f76aebc3be396cbd791e25f2f15f2254e404e0`，最新state／manifest三案next_day=46；原[r1](analytics.md)與[r2](analytics-r2.md)、原commit／blob／SHA256均保留。產品仍是唯讀 `64eff527283fd05657841297879aefeaadd31f64`，未採用任何Day45 Green。

完整使用者旅程承接：獲授權人員開帳號頁→只具read時不能新增→另外獲create action後建立viewer→首次改密碼→依頁面read與來源read查後台／summary／kind／CSV；減權後同一Session的下一請求立即拒絕。正式main→password_login::build→page_acl middleware→handler；新增帳號在交易內重驗page+create後才寫帳號、角色、來源與稽核。

[單一ACL契約](../../../requirements/ANALYTICS_ACL.md)區分直接確認與本候選採用的組合規則。category是容器，allow不自動授予下層CRUD；page與目標action各需有效allow。category／page／action的個人deny優先，父節點停用或結構缺失拒絕。這些組合規則在候選中實際測試，沒有宣稱使用者曾逐項指定所有繼承細節。

## Refactor｜承接本案 Session PG Adapter，不重複計功

本r3沿用Day45原獨立Refactor：Session lookup由HTTP模組移至analytics_persistence::login_session，Adapter只收digest與PgPool、回完整SessionRow／SQLx錯誤；HTTP仍處理Cookie、401／503、密碼fingerprint與touch。Cargo／postgres features及既有caller不變，不修改shared或全庫CI。

以下為同一份完整Refactor，沒有在本次修訂增加重構完成數。原day23帳密、day24原子改密碼／撤銷、day26活動Session、day37Cookie、day39CSV及day44Logout CSRF的有效斷言全部保留，透過正式Session caller接到Adapter。新增階層CRUD屬功能Green，不拿來重複計功。

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

### Red：只有page權限不能執行create

在原r2候選（精確產品來源＋Refactor＋r2 Green）套以下完整增量Red。已有viewer的個人帳號page allow，但未授予create時，POST /accounts必須403且不得留下帳號。原r2會實際303並新增，所以本次真正行為Red exit101；不是missing-symbol、DB或匯入錯誤。

```diff
diff --git a/analytics_apps/analytics_server/tests/day45_viewer.rs b/analytics_apps/analytics_server/tests/day45_viewer.rs
--- a/analytics_apps/analytics_server/tests/day45_viewer.rs
+++ b/analytics_apps/analytics_server/tests/day45_viewer.rs
@@ -207,6 +207,12 @@
     let delegated = send(app.clone(), "/accounts/new", None, Some(&reader)).await;
     assert_eq!(delegated.status(), StatusCode::OK);
     let csrf = token(&body_text(delegated).await, "/accounts");
+    // A page allow does not imply its Create action.
+    let denied=send(app.clone(),"/accounts",Some(&format!("username=pageonly&password=pageonly-password&email=pageonly%40example.test&pet=on&csrf={csrf}")),Some(&reader)).await;
+    assert_eq!(denied.status(),StatusCode::FORBIDDEN);
+    let count:i64=sqlx::query_scalar("SELECT count(*) FROM uac_user WHERE user_acnt='pageonly'").fetch_one(&pool).await.unwrap();
+    assert_eq!(count,0);
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) SELECT id,'analytics-accounts-create','allow',now() FROM uac_user WHERE user_acnt='reader'").execute(&pool).await.unwrap();
     let created=send(app.clone(),"/accounts",Some(&format!("username=delegated&password=delegated-password&email=delegated%40example.test&pet=on&csrf={csrf}")),Some(&reader)).await;
     assert_eq!(created.status(), StatusCode::SEE_OTHER);
     sqlx::query("UPDATE uac_user_acl SET acl_filter='deny' WHERE acl_id='analytics-accounts' AND user_id=(SELECT id FROM uac_user WHERE user_acnt='reader')").execute(&pool).await.unwrap();
```

```sh
DATABASE_URL=postgres://<isolated-test-user>@127.0.0.1:<test-port>/<test-db>   cargo test --manifest-path analytics_apps/Cargo.toml -p analytics-server   --features postgres-integration --test day45_viewer --locked
```

測試用本次隔離PG18.6、port55485／lesson_day45，SQLx自行建立隔離測試DB，沒有碰正式DB／秘密／付費AI。核心Red／Green未實際計時；Refactor、版本與交付另計，不以60分鐘刪減完整內容。產生Red腳本第一次因fixture文字定位不符出現`ValueError: substring not found`、尚未跑測試；修正後才執行上列真正行為Red。

### Green：完整替代r3 patch

套用基準為**產品64eff527＋原Refactor，尚未套任何Green**。以下`analytics-day45-green-r3.patch`已包含整個Day45建立viewer、Session／來源、階層ACL與全部測試；不是只貼出靜態SQL hunk。請在乾淨副本套Refactor→本r3 Green，不要接在r1或r2 Green後重複套。若已採用舊候選並執行migration，必須另做向前migration，不得改已執行checksum；本次產品main尚未採用舊候選。

完整接線：

- migration重用acl_category、acl_parent_id，建立三個category、八個功能page與各自CRUD子action，不新增階層欄位。page的acl_path是路由／模板，action的acl_path是create/read/update/delete，授權時必須一起核對parent。
- Admin預設擁有候選各page與CRUD；viewer預設read，另有修改自身密碼的update；來源page由建立帳號表單明確選定。個人page allow不會自動取得create，個人action allow也不能略過其page。
- page_acl::operation由伺服器固定映射：GET／HEAD=read，POST /accounts=create，POST /change-password=update；DELETE /accounts分類為delete但未實作路由仍405。沒有實作的update／delete route維持不可用，不能因seed了action就宣稱API已完成。
- PERMISSION_SQL驗證唯一page、唯一目標action與完整三層／啟用狀態；category allow不作CRUD繼承，個人deny可減掉category子樹、page或單一action。未知／歧義／損壞結構拒絕。來源檢查同樣使用page+read，不保留管理員硬式略過。
- accounts::create_inner在既有帳號建立交易內鎖住人員／Session、驗fingerprint，再用同一PERMISSION_SQL重查page+create。HTTP及交易共用同案規則，保留Argon2、CSRF、首次改密碼、rollback及稽核。

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
index 0000000..2282e00
--- /dev/null
+++ b/analytics_apps/analytics_server/src/accounts.rs
@@ -0,0 +1,182 @@
+use crate::{
+    IngestState,
+    password_login::{digest, renewed, session},
+};
+use argon2::{Argon2, PasswordHasher};
+use axum::{
+    Form,
+    extract::{Query, State},
+    http::{HeaderMap, StatusCode, header},
+    response::{Html, Redirect, Response},
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
+    // Lock identity/credential state, then recheck page + Create action in this transaction.
+    let authorized:Option<(String,String)>=sqlx::query_as(
+        "SELECT u.user_pd,s.credential_fingerprint FROM uac_user u JOIN uac_login_session s ON s.user_id=u.id
+         WHERE u.id=$1 AND s.token_hash=$2 AND s.expires_at>floor(extract(epoch from now()))::bigint
+          AND s.restricted=FALSE AND u.user_status='active' AND u.is_init_pd=FALSE FOR SHARE OF u,s")
+        .bind(actor).bind(digest(raw)).fetch_optional(&mut *tx).await.map_err(db_error)?;
+    let (credential, fingerprint) = authorized.ok_or(StatusCode::FORBIDDEN)?;
+    if digest(&credential) != fingerprint {
+        return Err(StatusCode::UNAUTHORIZED);
+    }
+    let permitted: bool = sqlx::query_scalar(crate::page_acl::PERMISSION_SQL)
+        .bind(digest(raw))
+        .bind("/accounts/new")
+        .bind("create")
+        .fetch_one(&mut *tx)
+        .await
+        .map_err(db_error)?;
+    if !permitted {
+        return Err(StatusCode::FORBIDDEN);
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
index 0000000..970c8cf
--- /dev/null
+++ b/analytics_apps/analytics_server/src/page_acl.rs
@@ -0,0 +1,260 @@
+use crate::{
+    IngestState,
+    password_login::{digest, session},
+};
+use axum::{
+    extract::{MatchedPath, Request, State},
+    http::{Method, StatusCode},
+    middleware::Next,
+    response::{IntoResponse, Redirect, Response},
+};
+
+#[derive(Clone, Copy)]
+pub(crate) enum Crud {
+    Create,
+    Read,
+    Update,
+    Delete,
+}
+impl Crud {
+    fn key(self) -> &'static str {
+        match self {
+            Self::Create => "create",
+            Self::Read => "read",
+            Self::Update => "update",
+            Self::Delete => "delete",
+        }
+    }
+}
+
+// The server maps actual protocol operations; the caller cannot submit its own permission name.
+fn operation(route: &str, method: &Method) -> Option<Crud> {
+    match (route, method) {
+        ("/accounts", &Method::POST) => Some(Crud::Create),
+        ("/accounts", &Method::DELETE) => Some(Crud::Delete),
+        ("/change-password", &Method::POST) => Some(Crud::Update),
+        (_, &Method::GET | &Method::HEAD) => Some(Crud::Read),
+        _ => None,
+    }
+}
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
+    let page = match route {
+        "/" | "/dashboard" => "/dashboard",
+        "/accounts" | "/accounts/new" => "/accounts/new",
+        "/change-password" => "/change-password",
+        "/api/v1/events/{source}/summary"
+        | "/api/v1/events/{source}/summary/{event_type}"
+        | "/api/v1/events/{source}/export.csv" => route,
+        _ => return next.run(request).await,
+    };
+    let Some(crud) = operation(route, request.method()) else {
+        return StatusCode::METHOD_NOT_ALLOWED.into_response();
+    };
+    let (restricted, raw) = match session(&state, request.headers()).await {
+        Ok(v) => v,
+        Err(StatusCode::UNAUTHORIZED) if page == "/dashboard" => {
+            return Redirect::to("/login").into_response();
+        }
+        Err(e) => return e.into_response(),
+    };
+    if restricted && page == "/dashboard" {
+        return next.run(request).await;
+    }
+    match allowed(&state, &raw, page, crud).await {
+        Ok(true) => next.run(request).await,
+        Ok(false) => (StatusCode::FORBIDDEN, "沒有此頁面或 CRUD action 的權限。").into_response(),
+        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
+    }
+}
+
+// Requires a valid three-level tree and separate effective grants on both page and action.
+// User deny on category/page/action wins; allowing a category never grants descendant CRUD.
+pub(crate) const PERMISSION_SQL: &str = r#"
+WITH node AS (
+ SELECT p.id AS page_id,c.id AS category_id,a.id AS action_id
+ FROM uac_acl p JOIN uac_acl c ON c.id=p.acl_parent_id
+ JOIN uac_acl a ON a.acl_parent_id=p.id
+ WHERE p.acl_category='page' AND p.acl_path=$2 AND p.acl_status='active'
+  AND c.acl_category='category' AND c.acl_parent_id IS NULL AND c.acl_status='active'
+  AND a.acl_category='action' AND a.acl_path=$3 AND a.acl_status='active'
+  AND (SELECT count(*) FROM uac_acl p2 WHERE p2.acl_category='page' AND p2.acl_path=$2)=1
+  AND (SELECT count(*) FROM uac_acl a2 WHERE a2.acl_parent_id=p.id AND a2.acl_category='action' AND a2.acl_path=$3)=1
+), person AS (
+ SELECT s.user_id FROM uac_login_session s JOIN uac_user u ON u.id=s.user_id
+ WHERE s.token_hash=$1 AND s.expires_at>floor(extract(epoch from now()))::bigint
+  AND u.user_status='active'
+)
+SELECT EXISTS(SELECT 1 FROM node n CROSS JOIN person p
+ WHERE NOT EXISTS(SELECT 1 FROM uac_user_acl ua WHERE ua.user_id=p.user_id
+   AND ua.acl_id IN(n.category_id,n.page_id,n.action_id) AND ua.acl_filter='deny')
+ AND NOT EXISTS(SELECT 1 FROM (VALUES(n.page_id),(n.action_id)) AS needed(id)
+   WHERE NOT EXISTS(SELECT 1 FROM uac_user_role ur JOIN uac_role r ON r.id=ur.role_id
+     JOIN uac_role_acl ra ON ra.role_id=r.id AND ra.acl_id=needed.id
+     WHERE ur.user_id=p.user_id AND r.role_status='active')
+    AND NOT EXISTS(SELECT 1 FROM uac_user_acl ua WHERE ua.user_id=p.user_id
+      AND ua.acl_id=needed.id AND ua.acl_filter='allow')))
+"#;
+pub(crate) async fn allowed(
+    state: &IngestState,
+    raw: &str,
+    path: &str,
+    operation: Crud,
+) -> Result<bool, sqlx::Error> {
+    sqlx::query_scalar(PERMISSION_SQL)
+        .bind(digest(raw))
+        .bind(path)
+        .bind(operation.key())
+        .fetch_one(&state.pool)
+        .await
+}
+
+#[cfg(all(test, feature = "postgres-integration"))]
+mod hierarchy_tests {
+    use super::*;
+    use crate::{IngestTokens, bootstrap};
+    use std::sync::Arc;
+
+    #[sqlx::test(migrations = "../migrations")]
+    async fn three_levels_and_each_crud_grant_are_checked_independently(pool: sqlx::PgPool) {
+        let id = bootstrap::initialize(&pool, "hierarchy-password".into())
+            .await
+            .unwrap();
+        sqlx::query("UPDATE uac_user SET is_init_pd=FALSE WHERE id=$1")
+            .bind(&id)
+            .execute(&pool)
+            .await
+            .unwrap();
+        let credential: String = sqlx::query_scalar("SELECT user_pd FROM uac_user WHERE id=$1")
+            .bind(&id)
+            .fetch_one(&pool)
+            .await
+            .unwrap();
+        let raw = "f".repeat(64);
+        sqlx::query("INSERT INTO uac_login_session(token_hash,user_id,credential_fingerprint,restricted,expires_at) VALUES($1,$2,$3,FALSE,floor(extract(epoch from now()))::bigint+1800)")
+            .bind(digest(&raw)).bind(&id).bind(digest(&credential)).execute(&pool).await.unwrap();
+        let state = IngestState {
+            pool: pool.clone(),
+            tokens: Arc::new(IngestTokens {
+                chat: "fixture-chat".into(),
+                pet: "fixture-pet".into(),
+            }),
+        };
+        let roots:Vec<String>=sqlx::query_scalar("SELECT acl_name FROM uac_acl WHERE acl_category='category' AND acl_parent_id IS NULL ORDER BY acl_name").fetch_all(&pool).await.unwrap();
+        assert_eq!(roots.len(), 3);
+        for name in ["會員管理", "資訊分析", "認證管理"] {
+            assert!(roots.iter().any(|n| n == name));
+        }
+        let malformed:i64=sqlx::query_scalar("SELECT count(*) FROM uac_acl a LEFT JOIN uac_acl p ON p.id=a.acl_parent_id LEFT JOIN uac_acl c ON c.id=p.acl_parent_id WHERE a.acl_category='action' AND (p.acl_category IS DISTINCT FROM 'page' OR c.acl_category IS DISTINCT FROM 'category' OR c.acl_parent_id IS NOT NULL)").fetch_one(&pool).await.unwrap();
+        assert_eq!(malformed, 0);
+        for op in [Crud::Create, Crud::Read, Crud::Update, Crud::Delete] {
+            assert!(allowed(&state, &raw, "/accounts/new", op).await.unwrap());
+        }
+        sqlx::query("DELETE FROM uac_role_acl WHERE role_id='analytics-admin' AND acl_id='analytics-accounts-delete'").execute(&pool).await.unwrap();
+        assert!(
+            !allowed(&state, &raw, "/accounts/new", Crud::Delete)
+                .await
+                .unwrap()
+        );
+        assert!(
+            allowed(&state, &raw, "/accounts/new", Crud::Read)
+                .await
+                .unwrap()
+        );
+        sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES($1,'analytics-accounts-delete','allow',now())").bind(&id).execute(&pool).await.unwrap();
+        assert!(
+            allowed(&state, &raw, "/accounts/new", Crud::Delete)
+                .await
+                .unwrap()
+        );
+        sqlx::query("UPDATE uac_user_acl SET acl_filter='deny' WHERE user_id=$1 AND acl_id='analytics-accounts-delete'").bind(&id).execute(&pool).await.unwrap();
+        assert!(
+            !allowed(&state, &raw, "/accounts/new", Crud::Delete)
+                .await
+                .unwrap()
+        );
+        assert!(
+            allowed(&state, &raw, "/accounts/new", Crud::Create)
+                .await
+                .unwrap()
+        );
+        // Parent deny closes the subtree, leaving the other category usable.
+        sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES($1,'analytics-members','deny',now())").bind(&id).execute(&pool).await.unwrap();
+        assert!(
+            !allowed(&state, &raw, "/accounts/new", Crud::Read)
+                .await
+                .unwrap()
+        );
+        assert!(
+            allowed(&state, &raw, "/dashboard", Crud::Read)
+                .await
+                .unwrap()
+        );
+        sqlx::query("DELETE FROM uac_user_acl WHERE user_id=$1 AND acl_id='analytics-members'")
+            .bind(&id)
+            .execute(&pool)
+            .await
+            .unwrap();
+        // Invalid/missing parent, inactive root, and duplicate action fail closed.
+        sqlx::query(
+            "UPDATE uac_acl SET acl_parent_id='analytics-accounts' WHERE id='analytics-accounts'",
+        )
+        .execute(&pool)
+        .await
+        .unwrap();
+        assert!(
+            !allowed(&state, &raw, "/accounts/new", Crud::Read)
+                .await
+                .unwrap()
+        );
+        sqlx::query(
+            "UPDATE uac_acl SET acl_parent_id='analytics-members' WHERE id='analytics-accounts'",
+        )
+        .execute(&pool)
+        .await
+        .unwrap();
+        sqlx::query("UPDATE uac_acl SET acl_status='inactive' WHERE id='analytics-auth'")
+            .execute(&pool)
+            .await
+            .unwrap();
+        assert!(
+            !allowed(&state, &raw, "/change-password", Crud::Update)
+                .await
+                .unwrap()
+        );
+        sqlx::query("INSERT INTO uac_acl(id,acl_name,acl_path,acl_category,acl_parent_id,acl_status,created_at,updated_at) VALUES('duplicate-read','read','read','action','analytics-accounts','active',now(),now())").execute(&pool).await.unwrap();
+        assert!(
+            !allowed(&state, &raw, "/accounts/new", Crud::Read)
+                .await
+                .unwrap()
+        );
+    }
+    #[test]
+    fn wire_operations_do_not_trust_a_client_supplied_action() {
+        assert!(matches!(
+            operation("/accounts", &Method::POST),
+            Some(Crud::Create)
+        ));
+        assert!(matches!(
+            operation("/change-password", &Method::POST),
+            Some(Crud::Update)
+        ));
+        assert!(matches!(
+            operation("/dashboard", &Method::GET),
+            Some(Crud::Read)
+        ));
+        assert!(matches!(
+            operation("/accounts", &Method::DELETE),
+            Some(Crud::Delete)
+        ));
+        assert!(operation("/accounts", &Method::PATCH).is_none());
+    }
+}
diff --git a/analytics_apps/analytics_server/src/password_login.rs b/analytics_apps/analytics_server/src/password_login.rs
index 3a6d662..dc2cbae 100644
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
+    let allowed = crate::page_acl::allowed(state, &raw, path, crate::page_acl::Crud::Read)
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
index 0000000..dd0e79c
--- /dev/null
+++ b/analytics_apps/analytics_server/tests/day45_viewer.rs
@@ -0,0 +1,310 @@
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
+    // ACL classification does not manufacture an unimplemented DELETE route.
+    {
+        use tower::ServiceExt;
+        let response = app
+            .clone()
+            .oneshot(
+                axum::http::Request::delete("/accounts")
+                    .header(header::COOKIE, &cookie)
+                    .body(axum::body::Body::empty())
+                    .unwrap(),
+            )
+            .await
+            .unwrap();
+        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
+    }
+
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
+    // A page allow does not imply its Create action.
+    let denied=send(app.clone(),"/accounts",Some(&format!("username=pageonly&password=pageonly-password&email=pageonly%40example.test&pet=on&csrf={csrf}")),Some(&reader)).await;
+    assert_eq!(denied.status(), StatusCode::FORBIDDEN);
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM uac_user WHERE user_acnt='pageonly'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count, 0);
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) SELECT id,'analytics-accounts-create','allow',now() FROM uac_user WHERE user_acnt='reader'").execute(&pool).await.unwrap();
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
index 0000000..9df7224
--- /dev/null
+++ b/analytics_apps/migrations/20261031074500_viewer_access.sql
@@ -0,0 +1,39 @@
+-- Additive access policy using the existing UAC role/ACL tables.
+CREATE UNIQUE INDEX uq_uac_account_day45 ON uac_user(lower(trim(user_acnt)));
+INSERT INTO uac_role(id,role_name,role_status,created_at,updated_at) VALUES
+('analytics-admin','system_admin','active',now(),now()),
+('analytics-viewer','viewer','active',now(),now());
+-- Exactly three levels. Existing acl_category and acl_parent_id are reused.
+INSERT INTO uac_acl(id,acl_name,acl_path,acl_category,acl_parent_id,acl_status,created_at,updated_at) VALUES
+('analytics-members','會員管理','','category',NULL,'active',now(),now()),
+('analytics-info','資訊分析','','category',NULL,'active',now(),now()),
+('analytics-auth','認證管理','','category',NULL,'active',now(),now());
+INSERT INTO uac_acl(id,acl_name,acl_path,acl_category,acl_parent_id,acl_status,created_at,updated_at) VALUES
+('analytics-accounts','帳號管理','/accounts/new','page','analytics-members','active',now(),now()),
+('analytics-dashboard','分析後台','/dashboard','page','analytics-info','active',now(),now()),
+('analytics-chat','Chat來源','/api/v1/events/web_chat','page','analytics-info','active',now(),now()),
+('analytics-pet','Pet來源','/api/v1/events/desktop_pet','page','analytics-info','active',now(),now()),
+('analytics-summary','彙總查詢','/api/v1/events/{source}/summary','page','analytics-info','active',now(),now()),
+('analytics-kind','類型查詢','/api/v1/events/{source}/summary/{event_type}','page','analytics-info','active',now(),now()),
+('analytics-export','CSV匯出','/api/v1/events/{source}/export.csv','page','analytics-info','active',now(),now()),
+('analytics-password','自身密碼','/change-password','page','analytics-auth','active',now(),now());
+INSERT INTO uac_acl(id,acl_name,acl_path,acl_category,acl_parent_id,acl_status,created_at,updated_at)
+SELECT p.id || '-' || operation, operation, operation, 'action', p.id, 'active', now(),now()
+FROM uac_acl p CROSS JOIN (VALUES('create'),('read'),('update'),('delete')) AS crud(operation)
+WHERE p.id IN ('analytics-accounts','analytics-dashboard','analytics-chat','analytics-pet','analytics-summary','analytics-kind','analytics-export','analytics-password');
+INSERT INTO uac_user_role(user_id,role_id,created_at)
+SELECT id,'analytics-admin',now() FROM uac_user WHERE lower(user_acnt)='admin';
+CREATE TABLE analytics_account_audit (
+ id TEXT PRIMARY KEY, actor_id TEXT NOT NULL, target_id TEXT NOT NULL,
+ action TEXT NOT NULL, result TEXT NOT NULL, created_at TIMESTAMPTZ NOT NULL DEFAULT now()
+);
+-- Admin default: all seeded pages and their CRUD actions (no implicit category grant).
+INSERT INTO uac_role_acl(role_id,acl_id,created_at)
+SELECT 'analytics-admin',a.id,now() FROM uac_acl a
+WHERE a.id IN ('analytics-accounts','analytics-dashboard','analytics-chat','analytics-pet','analytics-summary','analytics-kind','analytics-export','analytics-password')
+ OR a.acl_parent_id IN ('analytics-accounts','analytics-dashboard','analytics-chat','analytics-pet','analytics-summary','analytics-kind','analytics-export','analytics-password');
+-- Viewer default: read operations; source page visibility is independently granted per user.
+INSERT INTO uac_role_acl(role_id,acl_id,created_at)
+SELECT 'analytics-viewer',a.id,now() FROM uac_acl a
+WHERE a.id IN ('analytics-dashboard','analytics-password','analytics-summary','analytics-kind','analytics-export','analytics-password-update')
+ OR (a.acl_category='action' AND a.acl_path='read' AND a.acl_parent_id IN ('analytics-accounts','analytics-dashboard','analytics-chat','analytics-pet','analytics-summary','analytics-kind','analytics-export','analytics-password'));
```

### 有效斷言與目前未實作範圍

原viewer新增→首次改密碼→來源讀取／CSV→撤銷／停用的旅程繼續保留；原個人加權測試改為明列page與create action兩筆，先斷言page-only POST403與無殘留，再加create後成功，不能直接刪掉舊有效成功／拒絕斷言。

新增真PG政策測試驗證三個category名稱及action→page→category結構、CRUD四項獨立判斷、只移除delete不影響read、個人新增delete與deny覆蓋、category deny隔離其他類別、錯誤parent／自循環、停用root與重複action拒絕；協定測試驗證operation固定映射，未實作DELETE route不開放。

本修訂只建立ACL資料契約與正式授權判斷；角色／階層ACL／個人加減權編輯頁、會員修改／刪除API及所有既有寫入的交易ACL邊界仍待後續。新增帳號的交易內重驗已接通，其餘操作不因本篇測試被追認。page讀取／action CRUD與來源範圍三者不可互相替代。

FIFO無新結案；Chat／Pet舊待續保持原值，last_full_comparison不變，沒有以本修訂推進Day或週比對。

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

本修訂於同日重查官方registry（analytics-versions-r3.json），無新增套件、無Cargo／lock變動。PG實測18.6；[官方版本政策](https://www.postgresql.org/support/versioning/)最新stable18.6，19RC排除。SQLx0.9／Rust1.99.1可編譯此候選；真PG測試不等於舊庫升級通過，新增唯一帳號index仍須盤點舊資料重複，不刪資料強行升級。

## ④ 實測、交付與下一步

- 行為Red：原r2 page-only寫入實際303、期待403，exit101，見analytics-hierarchy-red.log。
- Green：正式Router／真PG完整旅程通過，見analytics-hierarchy-green.log。
- 最終回歸：62通過、1既有legacy ignored，見analytics-hierarchy-final.log；ignored並未執行或抵算成功。
- 乾淨精確來源＋Refactor的r3 Green `git apply --check` exit0；完整附件、SHA256及來源CI見analytics-validation-r3.json／publication-r3.json。來源七個workflow success僅屬原產品，不涵蓋未套用候選。

Gate1內容及版本完成，精確commit全文讀回後才同步同一Day的state／manifest／入口；next_day維持三案46。Gate2=false：產品尚未採用，未跑正式瀏覽器、真實舊PG升級、階層權限編輯UI或即時連線縮權，不把政策測試當成完整CRUD產品驗收。

下一步依新需求接通角色／個人權限編輯流程與稽核，再以實際產品SHA驗證category導覽、page可見性及各CRUD操作。回復程式以r3 Green→Refactor反序reverse；若測試庫已跑新migration，依備份／向前修訂處理，不能直接改歷史checksum。本次只寫copilot_use/main，rust_ai_ring保持唯讀。
