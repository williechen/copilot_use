# Analytics Day45 r4｜重新接線 page 功能樹、CRUD與首次自身改密碼

## 目標、精確來源與最後需求

2026-10-31；產品唯讀 `f6a04015188da2155af6c264d0e6326e6a922dca`，文件基準 `c89ad285f757f35cb8dbf215b697ff8e5ded38c4`。[共用授權契約](../../../requirements/COMMON_AUTHORIZATION.md)、[驗證規則](../../../requirements/AUTHENTICATION.md)及最新使用者確認優先。三案next_day均46；這是既有Day45修訂，不新增Day，FIFO與last_full_comparison不變。核心60分鐘只計原Red/Green；本修訂、Refactor、版本／驗證／交付另計，不倒填為60分鐘已完成新功能。

本版保留你在f6採用的20261031074500_viewer_access.sql逐位元不變：analytics-token是認證管理category、自身密碼在會員管理、管理憑證page與admin grant均保留。原r3基於64的patch不再直接套用。本版新增20261031120000_initial_password_personal_grant.sql，為既有active且待首次修改者補缺少的自身密碼page/update個人allow，已有角色grant不重複加、已有個人deny不覆寫。建立viewer在同交易加password-update個人allow，不恢復你移除的viewer角色update或accounts-read預設。若個人deny造成首次改密碼受阻，必須修權限配置，不能跳過首次修改或自動覆蓋deny。

正式鏈：main→password_login::build→page_acl::require→handler；Session lookup移到PG Adapter，去除綁死admin/viewer角色ID的限制。帳號狀態、到期與credential fingerprint仍每請求核對；身份有效不等於有任何業務權限。bootstrap只為新建立admin指派角色，啟動不會恢復既有帳號已撤銷角色。

uac_acl只是有效定義；page與四種CRUD action分別由uac_role_acl、uac_user_acl保存grant／個人例外。有效角色聯集+個人allow−個人deny，deny優先，沒有grant預設拒絕。後端操作仍檢page+對應action，source另有獨立page/read限制。category有效狀態與結構、既有明示父deny關閉子樹規則保留，category不須allow、不產生下層grant。

顯示是page→category及page→action：至少一個有效page才顯示category；page-only可顯示自身密碼與會員管理，不須read/update才顯示category；沒有update不出現修改表單。action-only不顯示page/category。/navigation與真正修改密碼、帳號及dashboard HTML使用同一份投影，舊的無條件管理連結及來源導覽移除；有page與action才出現對應action，標籤安全escape。未實作的API/UI definition只作樹標籤，不製造可操作的假路由。

首次login→同一/change-password ACL page；POST仍須page+update，並在密碼交易內再次核對ACL。成功更新、清除首次狀態並撤銷全部舊Session後重新登入→/dashboard；未完成或被deny不能以第二次登入繞過。帳號建立仍同交易重驗page/create，保存viewer角色、個人source grant與audit。CRUD定義不製造不存在的DELETE帳號API，真Router仍405。

12個逐檔變更均在完整patch：analytics_persistence/lib.rs、login_session.rs；analytics_server/accounts.rs、bootstrap.rs、dashboard_ui.rs、export.rs、lib.rs、page_acl.rs、password_login.rs、tests/day45_final.rs、tests/day45_viewer.rs；新增向前migration。

## Red

在獨立精確f6副本只新增下列測試，執行真Router與隔離DB；exit=101、2項失敗。[真實Red紀錄](analytics-day45-final-red-r4.log)。原產品page-only沒有功能樹；即使個人deny密碼update，實際POST仍303而非403。後續新增的完整有效斷言包含於Green patch，不將編譯失敗或靜態hunk當成此行為Red。

```sh
DATABASE_URL=postgres://willie@127.0.0.1:55485/final_day45 CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 cargo test --manifest-path analytics_apps/Cargo.toml -p analytics-server --features postgres-integration --test day45_final --locked
```

完整Red測試patch（在獨立Red副本使用）：

```diff
diff --git a/analytics_apps/analytics_server/tests/day45_final.rs b/analytics_apps/analytics_server/tests/day45_final.rs
new file mode 100644
index 0000000..5cf975b
--- /dev/null
+++ b/analytics_apps/analytics_server/tests/day45_final.rs
@@ -0,0 +1,57 @@
+#![cfg(feature = "postgres-integration")]
+mod support;
+use analytics_server::{IngestState, IngestTokens, bootstrap, password_login};
+use axum::http::{StatusCode, header};
+use std::sync::Arc;
+use support::http::{body_text, response_cookie, send};
+async fn fixture(pool: &sqlx::PgPool) -> (axum::Router, String, String) {
+    let id=bootstrap::initialize(pool,"final-password".into()).await.unwrap();
+    sqlx::query("INSERT INTO uac_user_role(user_id,role_id,created_at) VALUES($1,'analytics-admin',now()) ON CONFLICT DO NOTHING").bind(&id).execute(pool).await.unwrap();
+    let app=password_login::build(IngestState{pool:pool.clone(),tokens:Arc::new(IngestTokens{chat:"chat-fixture".into(),pet:"pet-fixture".into()})});
+    let response=send(app.clone(),"/login",Some("username=admin&password=final-password"),None).await;
+    assert_eq!(response.status(),StatusCode::SEE_OTHER);
+    assert_eq!(response.headers()[header::LOCATION],"/change-password");
+    (app,response_cookie(&response),id)
+}
+#[sqlx::test(migrations = "../migrations")]
+async fn page_drives_category_and_actions_do_not_grant_pages(pool:sqlx::PgPool){
+    let (app,cookie,id)=fixture(&pool).await;
+    sqlx::query("DELETE FROM uac_user_role WHERE user_id=$1").bind(&id).execute(&pool).await.unwrap();
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES($1,'analytics-password','allow',now())").bind(&id).execute(&pool).await.unwrap();
+    let response=send(app.clone(),"/change-password",None,Some(&cookie)).await;
+    assert_eq!(response.status(),StatusCode::OK);
+    let html=body_text(response).await;
+    assert!(html.contains("data-acl-category=\"analytics-members\""),"page-only must display category");
+    assert!(html.contains("data-acl-page=\"analytics-password\""));
+    assert!(!html.contains("data-acl-action=\"analytics-password-update\""));
+    assert!(!html.contains("action=\"/change-password\""),"no update => no form");
+    assert!(!html.contains("data-acl-category=\"analytics-info\""));
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES($1,'analytics-password-update','allow',now())").bind(&id).execute(&pool).await.unwrap();
+    let html=body_text(send(app.clone(),"/change-password",None,Some(&cookie)).await).await;
+    assert!(html.contains("data-acl-action=\"analytics-password-update\""));
+    assert!(html.contains("action=\"/change-password\""));
+    sqlx::query("UPDATE uac_user_acl SET acl_filter='deny' WHERE user_id=$1 AND acl_id='analytics-password'").bind(&id).execute(&pool).await.unwrap();
+    assert_eq!(send(app.clone(),"/change-password",None,Some(&cookie)).await.status(),StatusCode::FORBIDDEN);
+    let nav=body_text(send(app.clone(),"/navigation",None,Some(&cookie)).await).await;
+    assert!(!nav.contains("data-acl-page=\"analytics-password\""));
+    assert!(!nav.contains("data-acl-category=\"analytics-members\""));
+    assert!(!nav.contains("data-acl-action=\"analytics-password-update\""));
+    assert_eq!(send(app,"/dashboard",None,Some(&cookie)).await.status(),StatusCode::SEE_OTHER,"mandatory change cannot be bypassed by missing grants");
+}
+#[sqlx::test(migrations = "../migrations")]
+async fn deny_beats_roles_and_removing_override_restores_default(pool:sqlx::PgPool){
+    let (app,cookie,id)=fixture(&pool).await;
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES($1,'analytics-password-update','deny',now())").bind(&id).execute(&pool).await.unwrap();
+    let before:(String,bool)=sqlx::query_as("SELECT user_pd,is_init_pd FROM uac_user WHERE id=$1").bind(&id).fetch_one(&pool).await.unwrap();
+    let raw=cookie.strip_prefix("analytics_session=").unwrap();
+    use sha2::{Digest,Sha256};
+    let csrf:String=Sha256::digest(format!("change-password:{raw}").as_bytes()).iter().map(|b|format!("{b:02x}")).collect();
+    let form=format!("current_password=final-password&new_password=changed-password&confirm_password=changed-password&email=admin%40example.test&csrf={csrf}");
+    assert_eq!(send(app.clone(),"/change-password",Some(&form),Some(&cookie)).await.status(),StatusCode::FORBIDDEN);
+    let after:(String,bool)=sqlx::query_as("SELECT user_pd,is_init_pd FROM uac_user WHERE id=$1").bind(&id).fetch_one(&pool).await.unwrap();assert_eq!(before,after);
+    sqlx::query("DELETE FROM uac_user_acl WHERE user_id=$1 AND acl_id='analytics-password-update'").bind(&id).execute(&pool).await.unwrap();
+    assert_eq!(send(app.clone(),"/change-password",Some(&form),Some(&cookie)).await.status(),StatusCode::SEE_OTHER);
+    let login=send(app.clone(),"/login",Some("username=admin&password=changed-password"),None).await;
+    assert_eq!(login.headers()[header::LOCATION],"/dashboard");
+    assert_eq!(send(app,"/change-password",None,Some(&cookie)).await.status(),StatusCode::UNAUTHORIZED,"all old sessions revoked");
+}
```

## Green

[完整組合patch](analytics-day45-final-r4.patch)直接對精確f6套用，已包含原Day45功能、Session Refactor與本修訂測試；**不要與舊Day45 Green／Refactor或本次Red patch累加**。讀者先在獨立Red副本重現失敗，再在乾淨f6副本套此完整patch。

```sh
git rev-parse HEAD # 必須是 f6a04015188da2155af6c264d0e6326e6a922dca
git apply --check analytics-day45-final-r4.patch
git apply analytics-day45-final-r4.patch
cargo test --manifest-path analytics_apps/Cargo.toml -p analytics-server --features postgres-integration --locked --no-fail-fast
```

逐檔完整變更如下，不省略新檔或caller接線：

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
index 0000000..36c8e28
--- /dev/null
+++ b/analytics_apps/analytics_persistence/src/login_session.rs
@@ -0,0 +1,23 @@
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
+            AND u.user_status='active'"#,
+    )
+    .bind(token_hash)
+    .fetch_optional(pool)
+    .await
+}
diff --git a/analytics_apps/analytics_server/src/accounts.rs b/analytics_apps/analytics_server/src/accounts.rs
new file mode 100644
index 0000000..b9f4475
--- /dev/null
+++ b/analytics_apps/analytics_server/src/accounts.rs
@@ -0,0 +1,200 @@
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
+    let mut html=view!{
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
+    let can_create =
+        crate::page_acl::allowed(&state, &raw, "/accounts/new", crate::page_acl::Crud::Create)
+            .await
+            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    if !can_create {
+        if let Some(start) = html.find("<form method=\"post\" action=\"/accounts\"") {
+            if let Some(end) = html[start..].find("</form>") {
+                html.replace_range(start..start + end + 7, "<p>沒有新增 action。</p>");
+            }
+        }
+    }
+    Ok(renewed(
+        Html(crate::page_acl::decorate(&state, &raw, html).await?),
+        &raw,
+    ))
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
+    // Viewer role intentionally lacks password-update. Provision this required self action
+    // as an explicit personal grant, not by restoring the removed role grant.
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES($1,'analytics-password-update','allow',now())")
+        .bind(&id).execute(&mut *tx).await.map_err(db_error)?;
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
index 365f840..5b2fddc 100644
--- a/analytics_apps/analytics_server/src/bootstrap.rs
+++ b/analytics_apps/analytics_server/src/bootstrap.rs
@@ -22,8 +22,8 @@ pub async fn initialize(pool: &sqlx::PgPool, password: String) -> Result<String,
 
     let exists: bool = sqlx::query_scalar(
         r#"SELECT EXISTS(
-            SELECT 1 
-            FROM uac_user 
+            SELECT 1
+            FROM uac_user
             WHERE lower(user_acnt)='admin')"#,
     )
     .fetch_one(&mut *tx)
@@ -48,6 +48,8 @@ pub async fn initialize(pool: &sqlx::PgPool, password: String) -> Result<String,
         .bind(hash)
         .execute(&mut *tx).await.map_err(|_| "管理員寫入失敗")?;
 
+    sqlx::query("INSERT INTO uac_user_role(user_id,role_id,created_at) VALUES($1,'analytics-admin',now()) ON CONFLICT DO NOTHING")
+        .bind(&id).execute(&mut *tx).await.map_err(|_| "管理員角色寫入失敗")?;
     tx.commit().await.map_err(|_| "管理員提交失敗")?;
 
     Ok(id)
@@ -72,6 +74,7 @@ pub async fn ensure_default(pool: &sqlx::PgPool) -> Result<String, String> {
         return Err("管理員身分重複；停止初始化，需人工查核".into());
     }
     if let Some((id,)) = admins.into_iter().next() {
+        // Existing identities retain their current grants, including deliberate role revocation.
         tx.commit()
             .await
             .map_err(|_| "初始化查核提交失敗".to_owned())?;
@@ -99,6 +102,8 @@ pub async fn ensure_default(pool: &sqlx::PgPool) -> Result<String, String> {
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
index 0000000..bcf5c1a
--- /dev/null
+++ b/analytics_apps/analytics_server/src/page_acl.rs
@@ -0,0 +1,434 @@
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
+    if restricted && page != "/change-password" {
+        return if page == "/dashboard" {
+            Redirect::to("/change-password").into_response()
+        } else {
+            StatusCode::FORBIDDEN.into_response()
+        };
+    }
+    let permit = if request.method() == Method::GET
+        && matches!(page, "/change-password" | "/accounts/new")
+    {
+        page_allowed(&state, &raw, page).await
+    } else {
+        allowed(&state, &raw, page, crud).await
+    };
+    match permit {
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
+        sqlx::query("UPDATE uac_acl SET acl_status='inactive' WHERE id='analytics-members'")
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
+
+// A page is visible without any action grant. Category visibility is a projection of pages.
+const PAGE_SQL: &str = r#"
+WITH person AS (
+ SELECT s.user_id FROM uac_login_session s JOIN uac_user u ON u.id=s.user_id
+ WHERE s.token_hash=$1 AND s.expires_at>floor(extract(epoch from now()))::bigint AND u.user_status='active'
+), granted AS (
+ SELECT a.* FROM uac_acl a CROSS JOIN person p WHERE a.acl_status='active'
+ AND NOT EXISTS(SELECT 1 FROM uac_user_acl x WHERE x.user_id=p.user_id AND x.acl_id=a.id AND x.acl_filter='deny')
+ AND (EXISTS(SELECT 1 FROM uac_user_acl x WHERE x.user_id=p.user_id AND x.acl_id=a.id AND x.acl_filter='allow')
+ OR EXISTS(SELECT 1 FROM uac_user_role ur JOIN uac_role r ON r.id=ur.role_id JOIN uac_role_acl ra ON ra.role_id=r.id
+ WHERE ur.user_id=p.user_id AND r.role_status='active' AND ra.acl_id=a.id))
+)
+SELECT c.id,c.acl_name,p.id,p.acl_name,p.acl_path,a.id,a.acl_name
+FROM granted p JOIN uac_acl c ON c.id=p.acl_parent_id AND c.acl_category='category'
+ AND c.acl_parent_id IS NULL AND c.acl_status='active'
+CROSS JOIN person who
+LEFT JOIN granted a ON a.acl_category='action' AND a.acl_parent_id=p.id
+ AND a.acl_path IN ('create','read','update','delete')
+ AND (SELECT count(*) FROM uac_acl other WHERE other.acl_parent_id=p.id AND other.acl_category='action' AND other.acl_path=a.acl_path)=1
+WHERE p.acl_category='page'
+ AND (SELECT count(*) FROM uac_acl other WHERE other.acl_category='page' AND other.acl_path=p.acl_path)=1
+ AND NOT EXISTS(SELECT 1 FROM uac_user_acl x WHERE x.user_id=who.user_id AND x.acl_id=c.id AND x.acl_filter='deny')
+ORDER BY c.id,p.id,a.id
+"#;
+type TreeRow = (
+    String,
+    String,
+    String,
+    String,
+    String,
+    Option<String>,
+    Option<String>,
+);
+async fn rows(state: &IngestState, raw: &str) -> Result<Vec<TreeRow>, sqlx::Error> {
+    sqlx::query_as(PAGE_SQL)
+        .bind(digest(raw))
+        .fetch_all(&state.pool)
+        .await
+}
+async fn page_allowed(state: &IngestState, raw: &str, path: &str) -> Result<bool, sqlx::Error> {
+    Ok(rows(state, raw).await?.iter().any(|r| r.4 == path))
+}
+fn escape(s: &str) -> String {
+    s.replace('&', "&amp;")
+        .replace('<', "&lt;")
+        .replace('>', "&gt;")
+        .replace('"', "&quot;")
+        .replace('\'', "&#39;")
+}
+pub(crate) async fn tree(state: &IngestState, raw: &str) -> Result<String, sqlx::Error> {
+    let mut html = String::from("<nav aria-label=\"功能樹\"><ul>");
+    let mut category = String::new();
+    let mut page = String::new();
+    for (cid, cname, pid, pname, path, aid, aname) in rows(state, raw).await? {
+        if cid != category {
+            if !page.is_empty() {
+                html.push_str("</ul></li>");
+                page.clear();
+            }
+            if !category.is_empty() {
+                html.push_str("</ul></li>");
+            }
+            html.push_str(&format!(
+                "<li data-acl-category=\"{}\">{}<ul>",
+                escape(&cid),
+                escape(&cname)
+            ));
+            category = cid;
+        }
+        if pid != page {
+            if !page.is_empty() {
+                html.push_str("</ul></li>");
+            }
+            html.push_str(&format!("<li data-acl-page=\"{}\">", escape(&pid)));
+            // API definitions remain visible labels; do not manufacture unimplemented UI routes.
+            let href = match path.as_str() {
+                "/api/v1/events/web_chat" => "/dashboard?source=web_chat",
+                "/api/v1/events/desktop_pet" => "/dashboard?source=desktop_pet",
+                other => other,
+            };
+            if matches!(
+                path.as_str(),
+                "/dashboard"
+                    | "/accounts/new"
+                    | "/change-password"
+                    | "/api/v1/events/web_chat"
+                    | "/api/v1/events/desktop_pet"
+            ) {
+                html.push_str(&format!(
+                    "<a href=\"{}\">{}</a>",
+                    escape(href),
+                    escape(&pname)
+                ));
+            } else {
+                html.push_str(&escape(&pname));
+            }
+            html.push_str("<ul>");
+            page = pid;
+        }
+        if let Some(id) = aid {
+            html.push_str(&format!(
+                "<li data-acl-action=\"{}\">{}</li>",
+                escape(&id),
+                escape(&aname.unwrap_or_default())
+            ));
+        }
+    }
+    if !page.is_empty() {
+        html.push_str("</ul></li>");
+    }
+    if !category.is_empty() {
+        html.push_str("</ul></li>");
+    }
+    html.push_str("</ul></nav>");
+    Ok(html)
+}
+pub(crate) async fn navigation(
+    State(state): State<IngestState>,
+    headers: axum::http::HeaderMap,
+) -> Result<axum::response::Html<String>, StatusCode> {
+    let (_, raw) = session(&state, &headers).await?;
+    Ok(axum::response::Html(
+        tree(&state, &raw)
+            .await
+            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?,
+    ))
+}
+// Shared management shell on actual HTML callers, not just a standalone projection test.
+pub(crate) async fn decorate(
+    state: &IngestState,
+    raw: &str,
+    mut html: String,
+) -> Result<String, StatusCode> {
+    let nav = tree(state, raw)
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    // Remove legacy unconditional shortcuts; the grant-derived tree owns management links.
+    for literal in [
+        "<a href=\"/change-password\">修改密碼與 Email</a>",
+        "<a href=\"/accounts/new\">帳號管理：新增唯讀帳號</a>",
+    ] {
+        html = html.replace(literal, "");
+    }
+    if let Some(start) = html.find("<nav aria-label=\"事件來源\"") {
+        if let Some(end) = html[start..].find("</nav>") {
+            html.replace_range(start..start + end + 6, "");
+        }
+    }
+    if !allowed(state, raw, "/api/v1/events/{source}/export.csv", Crud::Read)
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
+    {
+        if let Some(label) = html.find("匯出目前篩選 CSV") {
+            if let Some(start) = html[..label].rfind("<a ") {
+                if let Some(end) = html[label..].find("</a>") {
+                    html.replace_range(start..label + end + 4, "");
+                }
+            }
+        }
+    }
+    Ok(html.replacen("<main>", &format!("<main>{nav}"), 1))
+}
diff --git a/analytics_apps/analytics_server/src/password_login.rs b/analytics_apps/analytics_server/src/password_login.rs
index 0a51803..cf59cf9 100644
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
-            ,user_status 
-        FROM uac_user 
-        WHERE lower(user_acnt)='admin' 
+            ,user_status
+        FROM uac_user
+        WHERE lower(user_acnt)=lower($1)
         LIMIT 2"#,
     )
+    .bind(form.username.trim())
     .fetch_all(&state.pool)
     .await
     .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
@@ -104,7 +111,7 @@ async fn login(
         .await
         .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
     sqlx::query(
-        r#"DELETE FROM uac_login_session 
+        r#"DELETE FROM uac_login_session
         WHERE expires_at <= floor(extract(epoch from now()))::bigint"#,
     )
     .execute(&mut *tx)
@@ -140,41 +147,29 @@ async fn login(
 }
 
 // Recheck account status, mandatory-change state and password fingerprint on each protected request.
-async fn session(state: &IngestState, headers: &HeaderMap) -> Result<(bool, String), StatusCode> {
+pub(crate) async fn session(
+    state: &IngestState,
+    headers: &HeaderMap,
+) -> Result<(bool, String), StatusCode> {
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
         return Err(StatusCode::UNAUTHORIZED);
     }
     let refreshed = sqlx::query(
-        r#"UPDATE uac_login_session 
-            SET expires_at=floor(extract(epoch from now()))::bigint+1800 
-        WHERE token_hash=$1 
-            AND expires_at>floor(extract(epoch from now()))::bigint 
+        r#"UPDATE uac_login_session
+            SET expires_at=floor(extract(epoch from now()))::bigint+1800
+        WHERE token_hash=$1
+            AND expires_at>floor(extract(epoch from now()))::bigint
             AND EXISTS (
-                SELECT 1 
-                FROM uac_user 
-                WHERE id=$2 
-                    AND user_pd=$3 
+                SELECT 1
+                FROM uac_user
+                WHERE id=$2
+                    AND user_pd=$3
                     AND user_status='active')"#,
     )
     .bind(digest(&raw))
@@ -189,16 +184,30 @@ async fn session(state: &IngestState, headers: &HeaderMap) -> Result<(bool, Stri
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
 
@@ -208,12 +217,31 @@ async fn change_page(
 ) -> Result<Response, StatusCode> {
     let (restricted, raw) = session(&state, &headers).await?;
     let csrf = digest(&format!("change-password:{raw}"));
+    let can_update = crate::page_acl::allowed(
+        &state,
+        &raw,
+        "/change-password",
+        crate::page_acl::Crud::Update,
+    )
+    .await
+    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    let mut html = crate::dashboard_ui::render_change_password_with_logout(
+        restricted,
+        csrf,
+        digest(&format!("logout:{raw}")),
+    );
+    if !can_update {
+        if let Some(start) = html.find("<form method=\"post\" action=\"/change-password\"") {
+            if let Some(end) = html[start..].find("</form>") {
+                html.replace_range(
+                    start..start + end + 7,
+                    "<p>沒有修改密碼 action；請聯絡管理員處理授權。</p>",
+                );
+            }
+        }
+    }
     Ok(renewed(
-        Html(crate::dashboard_ui::render_change_password_with_logout(
-            restricted,
-            csrf,
-            digest(&format!("logout:{raw}")),
-        )),
+        Html(crate::page_acl::decorate(&state, &raw, html).await?),
         &raw,
     ))
 }
@@ -268,14 +296,13 @@ async fn change_password(
     let row: Option<(String, String, String)> = sqlx::query_as(
         r#"SELECT u.id
             ,u.user_pd
-            ,s.credential_fingerprint 
-        FROM uac_user u 
-        JOIN uac_login_session s 
-            ON s.user_id=u.id 
-        WHERE s.token_hash=$1 
-            AND s.expires_at>floor(extract(epoch from now()))::bigint 
-            AND u.user_status='active' 
-            AND lower(u.user_acnt)='admin' 
+            ,s.credential_fingerprint
+        FROM uac_user u
+        JOIN uac_login_session s
+            ON s.user_id=u.id
+        WHERE s.token_hash=$1
+            AND s.expires_at>floor(extract(epoch from now()))::bigint
+            AND u.user_status='active'
         FOR UPDATE OF u,s"#,
     )
     .bind(digest(&raw))
@@ -286,6 +313,16 @@ async fn change_password(
     if digest(&current) != fingerprint {
         return Err(StatusCode::UNAUTHORIZED);
     }
+    let permit: bool = sqlx::query_scalar(crate::page_acl::PERMISSION_SQL)
+        .bind(digest(&raw))
+        .bind("/change-password")
+        .bind("update")
+        .fetch_one(&mut *tx)
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    if !permit {
+        return Err(StatusCode::FORBIDDEN);
+    }
     let encoded = tokio::task::spawn_blocking(move || {
         let parsed = PasswordHash::new(&current).map_err(|_| StatusCode::UNAUTHORIZED)?;
         Argon2::default()
@@ -299,11 +336,11 @@ async fn change_password(
     .await
     .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)??;
     sqlx::query(
-        r#"UPDATE uac_user 
+        r#"UPDATE uac_user
             SET user_pd=$1
                 ,user_email=$2
                 ,is_init_pd=FALSE
-                ,updated_at=now() 
+                ,updated_at=now()
         WHERE id=$3"#,
     )
     .bind(encoded)
@@ -356,12 +393,24 @@ async fn dashboard(
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
@@ -380,17 +429,18 @@ async fn dashboard(
         return StatusCode::SERVICE_UNAVAILABLE.into_response();
     };
 
-    renewed(
-        Html(crate::dashboard_ui::render_dashboard_with_rows_csrf(
-            source_label,
-            total,
-            source,
-            kind,
-            &rows,
-            &digest(&format!("logout:{raw}")),
-        )),
-        &raw,
-    )
+    let html = crate::dashboard_ui::render_dashboard_with_rows_csrf(
+        source_label,
+        total,
+        source,
+        kind,
+        &rows,
+        &digest(&format!("logout:{raw}")),
+    );
+    match crate::page_acl::decorate(&state, &raw, html).await {
+        Ok(html) => renewed(Html(html), &raw),
+        Err(e) => e.into_response(),
+    }
 }
 
 async fn summary(
@@ -398,7 +448,7 @@ async fn summary(
     Path(source): Path<String>,
     headers: HeaderMap,
 ) -> Result<Response, StatusCode> {
-    let raw = require_query_session(&state, &headers).await?;
+    let raw = require_source_session(&state, &headers, &source).await?;
     if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
         return Err(StatusCode::BAD_REQUEST);
     }
@@ -445,6 +495,9 @@ async fn logout(
 
 pub fn build(state: IngestState) -> Router {
     Router::new()
+        .route("/navigation", get(crate::page_acl::navigation))
+        .route("/accounts/new", get(crate::accounts::page))
+        .route("/accounts", post(crate::accounts::create))
         .route("/login", get(login_page).post(login))
         .route("/logout", post(logout))
         .route("/change-password", get(change_page).post(change_password))
@@ -461,6 +514,10 @@ pub fn build(state: IngestState) -> Router {
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
diff --git a/analytics_apps/analytics_server/tests/day45_final.rs b/analytics_apps/analytics_server/tests/day45_final.rs
new file mode 100644
index 0000000..12ed3c6
--- /dev/null
+++ b/analytics_apps/analytics_server/tests/day45_final.rs
@@ -0,0 +1,258 @@
+#![cfg(feature = "postgres-integration")]
+mod support;
+use analytics_server::{IngestState, IngestTokens, bootstrap, password_login};
+use axum::http::{StatusCode, header};
+use std::sync::Arc;
+use support::http::{body_text, response_cookie, send};
+async fn fixture(pool: &sqlx::PgPool) -> (axum::Router, String, String) {
+    let id = bootstrap::initialize(pool, "final-password".into())
+        .await
+        .unwrap();
+    sqlx::query("INSERT INTO uac_user_role(user_id,role_id,created_at) VALUES($1,'analytics-admin',now()) ON CONFLICT DO NOTHING").bind(&id).execute(pool).await.unwrap();
+    let app = password_login::build(IngestState {
+        pool: pool.clone(),
+        tokens: Arc::new(IngestTokens {
+            chat: "chat-fixture".into(),
+            pet: "pet-fixture".into(),
+        }),
+    });
+    let response = send(
+        app.clone(),
+        "/login",
+        Some("username=admin&password=final-password"),
+        None,
+    )
+    .await;
+    assert_eq!(response.status(), StatusCode::SEE_OTHER);
+    assert_eq!(response.headers()[header::LOCATION], "/change-password");
+    (app, response_cookie(&response), id)
+}
+#[sqlx::test(migrations = "../migrations")]
+async fn page_drives_category_and_actions_do_not_grant_pages(pool: sqlx::PgPool) {
+    let (app, cookie, id) = fixture(&pool).await;
+    sqlx::query("DELETE FROM uac_user_role WHERE user_id=$1")
+        .bind(&id)
+        .execute(&pool)
+        .await
+        .unwrap();
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES($1,'analytics-password','allow',now())").bind(&id).execute(&pool).await.unwrap();
+    let response = send(app.clone(), "/change-password", None, Some(&cookie)).await;
+    assert_eq!(response.status(), StatusCode::OK);
+    let html = body_text(response).await;
+    assert!(
+        html.contains("data-acl-category=\"analytics-members\""),
+        "page-only must display category"
+    );
+    assert!(html.contains("data-acl-page=\"analytics-password\""));
+    assert!(!html.contains("data-acl-action=\"analytics-password-update\""));
+    assert!(
+        !html.contains("action=\"/change-password\""),
+        "no update => no form"
+    );
+    assert!(!html.contains("data-acl-category=\"analytics-info\""));
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES($1,'analytics-password-update','allow',now())").bind(&id).execute(&pool).await.unwrap();
+    let html = body_text(send(app.clone(), "/change-password", None, Some(&cookie)).await).await;
+    assert!(html.contains("data-acl-action=\"analytics-password-update\""));
+    assert!(html.contains("action=\"/change-password\""));
+    sqlx::query("UPDATE uac_user_acl SET acl_filter='deny' WHERE user_id=$1 AND acl_id='analytics-password'").bind(&id).execute(&pool).await.unwrap();
+    assert_eq!(
+        send(app.clone(), "/change-password", None, Some(&cookie))
+            .await
+            .status(),
+        StatusCode::FORBIDDEN
+    );
+    let nav = body_text(send(app.clone(), "/navigation", None, Some(&cookie)).await).await;
+    assert!(!nav.contains("data-acl-page=\"analytics-password\""));
+    assert!(!nav.contains("data-acl-category=\"analytics-members\""));
+    assert!(!nav.contains("data-acl-action=\"analytics-password-update\""));
+    assert_eq!(
+        send(app, "/dashboard", None, Some(&cookie)).await.status(),
+        StatusCode::SEE_OTHER,
+        "mandatory change cannot be bypassed by missing grants"
+    );
+}
+#[sqlx::test(migrations = "../migrations")]
+async fn deny_beats_roles_and_removing_override_restores_default(pool: sqlx::PgPool) {
+    let (app, cookie, id) = fixture(&pool).await;
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES($1,'analytics-password-update','deny',now())").bind(&id).execute(&pool).await.unwrap();
+    let before: (String, bool) =
+        sqlx::query_as("SELECT user_pd,is_init_pd FROM uac_user WHERE id=$1")
+            .bind(&id)
+            .fetch_one(&pool)
+            .await
+            .unwrap();
+    let raw = cookie.strip_prefix("analytics_session=").unwrap();
+    use sha2::{Digest, Sha256};
+    let csrf: String = Sha256::digest(format!("change-password:{raw}").as_bytes())
+        .iter()
+        .map(|b| format!("{b:02x}"))
+        .collect();
+    let form = format!(
+        "current_password=final-password&new_password=changed-password&confirm_password=changed-password&email=admin%40example.test&csrf={csrf}"
+    );
+    assert_eq!(
+        send(app.clone(), "/change-password", Some(&form), Some(&cookie))
+            .await
+            .status(),
+        StatusCode::FORBIDDEN
+    );
+    let after: (String, bool) =
+        sqlx::query_as("SELECT user_pd,is_init_pd FROM uac_user WHERE id=$1")
+            .bind(&id)
+            .fetch_one(&pool)
+            .await
+            .unwrap();
+    assert_eq!(before, after);
+    sqlx::query("DELETE FROM uac_user_acl WHERE user_id=$1 AND acl_id='analytics-password-update'")
+        .bind(&id)
+        .execute(&pool)
+        .await
+        .unwrap();
+    assert_eq!(
+        send(app.clone(), "/change-password", Some(&form), Some(&cookie))
+            .await
+            .status(),
+        StatusCode::SEE_OTHER
+    );
+    let login = send(
+        app.clone(),
+        "/login",
+        Some("username=admin&password=changed-password"),
+        None,
+    )
+    .await;
+    assert_eq!(login.headers()[header::LOCATION], "/dashboard");
+    assert_eq!(
+        send(app, "/change-password", None, Some(&cookie))
+            .await
+            .status(),
+        StatusCode::UNAUTHORIZED,
+        "all old sessions revoked"
+    );
+}
+#[sqlx::test(migrations = false)]
+async fn upgrade_from_exact_f6_migrations_preserves_checksums_and_deny(pool: sqlx::PgPool) {
+    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../migrations");
+    let dir = std::env::temp_dir().join(format!("day45-upgrade-{}", uuid::Uuid::new_v4()));
+    std::fs::create_dir(&dir).unwrap();
+    for entry in std::fs::read_dir(&root).unwrap() {
+        let path = entry.unwrap().path();
+        if path.extension().is_some_and(|x| x == "sql")
+            && !path
+                .file_name()
+                .unwrap()
+                .to_str()
+                .unwrap()
+                .starts_with("20261031120000")
+        {
+            std::fs::copy(&path, dir.join(path.file_name().unwrap())).unwrap();
+        }
+    }
+    let old = sqlx::migrate::Migrator::new(dir.as_path()).await.unwrap();
+    old.run(&pool).await.unwrap();
+    let checksums: Vec<(i64, Vec<u8>)> =
+        sqlx::query_as("SELECT version,checksum FROM _sqlx_migrations ORDER BY version")
+            .fetch_all(&pool)
+            .await
+            .unwrap();
+    assert_eq!(checksums.len(), 7);
+    for id in ["initial-allow", "initial-deny"] {
+        sqlx::query("INSERT INTO uac_user(id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at) VALUES($1,$1,'fixture-hash',$1||'@example.test',$1,'active',TRUE,now(),now())").bind(id).execute(&pool).await.unwrap();
+    }
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES('initial-deny','analytics-password-update','deny',now())").execute(&pool).await.unwrap();
+    let new = sqlx::migrate::Migrator::new(root.as_path()).await.unwrap();
+    new.run(&pool).await.unwrap();
+    new.run(&pool).await.unwrap();
+    for (version, checksum) in checksums {
+        let actual: Vec<u8> =
+            sqlx::query_scalar("SELECT checksum FROM _sqlx_migrations WHERE version=$1")
+                .bind(version)
+                .fetch_one(&pool)
+                .await
+                .unwrap();
+        assert_eq!(actual, checksum);
+    }
+    let grants:Vec<(String,String)>=sqlx::query_as("SELECT user_id,acl_filter FROM uac_user_acl WHERE acl_id='analytics-password-update' ORDER BY user_id").fetch_all(&pool).await.unwrap();
+    assert_eq!(
+        grants,
+        vec![
+            ("initial-allow".into(), "allow".into()),
+            ("initial-deny".into(), "deny".into())
+        ]
+    );
+    let viewer_update:i64=sqlx::query_scalar("SELECT count(*) FROM uac_role_acl WHERE role_id='analytics-viewer' AND acl_id='analytics-password-update'").fetch_one(&pool).await.unwrap();
+    assert_eq!(viewer_update, 0);
+    std::fs::remove_dir_all(dir).unwrap();
+}
+#[sqlx::test(migrations = "../migrations")]
+async fn active_role_union_personal_override_and_definition_state(pool: sqlx::PgPool) {
+    let (app, cookie, id) = fixture(&pool).await;
+    sqlx::query("DELETE FROM uac_user_role WHERE user_id=$1")
+        .bind(&id)
+        .execute(&pool)
+        .await
+        .unwrap();
+    for role in ["page-role", "action-role"] {
+        sqlx::query("INSERT INTO uac_role(id,role_name,role_status,created_at,updated_at) VALUES($1,$1,'active',now(),now())").bind(role).execute(&pool).await.unwrap();
+        sqlx::query("INSERT INTO uac_user_role(user_id,role_id,created_at) VALUES($1,$2,now())")
+            .bind(&id)
+            .bind(role)
+            .execute(&pool)
+            .await
+            .unwrap();
+    }
+    for (role, acl) in [
+        ("page-role", "analytics-password"),
+        ("action-role", "analytics-password-update"),
+    ] {
+        sqlx::query("INSERT INTO uac_role_acl(role_id,acl_id,created_at) VALUES($1,$2,now())")
+            .bind(role)
+            .bind(acl)
+            .execute(&pool)
+            .await
+            .unwrap();
+    }
+    let html = body_text(send(app.clone(), "/change-password", None, Some(&cookie)).await).await;
+    assert!(html.contains("action=\"/change-password\""));
+    sqlx::query("UPDATE uac_role SET role_status='inactive' WHERE id='action-role'")
+        .execute(&pool)
+        .await
+        .unwrap();
+    let html = body_text(send(app.clone(), "/change-password", None, Some(&cookie)).await).await;
+    assert!(!html.contains("action=\"/change-password\""));
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES($1,'analytics-password-update','allow',now())").bind(&id).execute(&pool).await.unwrap();
+    let html = body_text(send(app.clone(), "/change-password", None, Some(&cookie)).await).await;
+    assert!(html.contains("action=\"/change-password\""));
+    sqlx::query("UPDATE uac_acl SET acl_status='inactive' WHERE id='analytics-password-update'")
+        .execute(&pool)
+        .await
+        .unwrap();
+    let html = body_text(send(app.clone(), "/change-password", None, Some(&cookie)).await).await;
+    assert!(!html.contains("action=\"/change-password\""));
+}
+#[sqlx::test(migrations = "../migrations")]
+async fn restart_does_not_restore_revoked_role_and_tree_escapes_labels(pool: sqlx::PgPool) {
+    let (app, cookie, id) = fixture(&pool).await;
+    sqlx::query("DELETE FROM uac_user_role WHERE user_id=$1")
+        .bind(&id)
+        .execute(&pool)
+        .await
+        .unwrap();
+    bootstrap::ensure_default(&pool).await.unwrap();
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM uac_user_role WHERE user_id=$1")
+        .bind(&id)
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count, 0);
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) VALUES($1,'analytics-password','allow',now())").bind(&id).execute(&pool).await.unwrap();
+    sqlx::query(
+        "UPDATE uac_acl SET acl_name='<script>alert(1)</script>' WHERE id='analytics-password'",
+    )
+    .execute(&pool)
+    .await
+    .unwrap();
+    let html = body_text(send(app, "/navigation", None, Some(&cookie)).await).await;
+    assert!(!html.contains("<script>"));
+    assert!(html.contains("&lt;script&gt;"));
+}
diff --git a/analytics_apps/analytics_server/tests/day45_viewer.rs b/analytics_apps/analytics_server/tests/day45_viewer.rs
new file mode 100644
index 0000000..b17c4af
--- /dev/null
+++ b/analytics_apps/analytics_server/tests/day45_viewer.rs
@@ -0,0 +1,320 @@
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
+    let html = body_text(page).await;
+    assert!(html.contains("Pet 全部事件類型"));
+    assert!(html.contains("data-acl-page=\"analytics-pet\""));
+    assert!(!html.contains("data-acl-page=\"analytics-chat\""));
+    // Independent user addition can grant a capability absent from the viewer role.
+    sqlx::query("INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at) SELECT id,'analytics-accounts','allow',now() FROM uac_user WHERE user_acnt='reader'").execute(&pool).await.unwrap();
+    let delegated = send(app.clone(), "/accounts/new", None, Some(&reader)).await;
+    assert_eq!(delegated.status(), StatusCode::OK);
+    assert!(!body_text(delegated).await.contains("action=\"/accounts\""));
+    // A forged direct request must still fail even when the UI hides Create.
+    use sha2::{Digest, Sha256};
+    let raw = reader.strip_prefix("analytics_session=").unwrap();
+    let csrf: String = Sha256::digest(format!("create-account:{raw}").as_bytes())
+        .iter()
+        .map(|b| format!("{b:02x}"))
+        .collect();
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
diff --git a/analytics_apps/migrations/20261031120000_initial_password_personal_grant.sql b/analytics_apps/migrations/20261031120000_initial_password_personal_grant.sql
new file mode 100644
index 0000000..77a782b
--- /dev/null
+++ b/analytics_apps/migrations/20261031120000_initial_password_personal_grant.sql
@@ -0,0 +1,10 @@
+-- Preserve adopted role defaults. Required initial change is provisioned per person.
+-- Never overwrite an explicit personal deny; administrators must resolve that conflict.
+INSERT INTO uac_user_acl(user_id,acl_id,acl_filter,created_at)
+SELECT u.id,a.id,'allow',now() FROM uac_user u
+CROSS JOIN (VALUES('analytics-password'),('analytics-password-update')) a(id)
+WHERE u.is_init_pd=TRUE AND u.user_status='active'
+AND NOT EXISTS(SELECT 1 FROM uac_user_acl x WHERE x.user_id=u.id AND x.acl_id=a.id)
+AND NOT EXISTS(SELECT 1 FROM uac_user_role ur JOIN uac_role r ON r.id=ur.role_id
+ JOIN uac_role_acl ra ON ra.role_id=r.id WHERE ur.user_id=u.id
+ AND r.role_status='active' AND ra.acl_id=a.id);
```

## Refactor

獨立Refactor沿用本案Day45 Session PostgreSQL Adapter：HTTP處理Cookie、fingerprint／狀態與續期，analytics_persistence/login_session只接digest／PgPool並回SessionRow或SQLx error。這是累積持久化邊界整理，不修改Chat、Pet、共用crate或全庫CI。原Day23帳密、Day24原子改密碼／撤銷、Day26續期、Day37 Cookie、Day39 CSV、Day44 Logout斷言承接。本次仍是同一份Refactor，不重複增加計功；移除角色白名單是本次授權Green修正。下列為原完整Refactor；本次組合patch已含，不可重複先套。

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

## 版本與實際驗證

[本回合官方查核](final-rewire-versions.json)：[Rust官方stable channel](https://static.rust-lang.org/dist/channel-rust-stable.toml)為1.99.1；鎖定Argon2 0.6.0、Axum 0.8.9、Leptos 0.8.22、SQLx 0.9.0、Tokio 1.54.1，與[官方crate registry](https://crates.io/)stable相符；Leptos 0.9 RC不採用。SQLx 0.9宣告Rust最低1.94，不能以舊1.85/MSRV承諾整條依賴；本機rustc/Cargo 1.99.1實測，未做最低版本編譯。PG實測18.6，[官方release](https://www.postgresql.org/docs/release/)19仍RC，沒有切換RC。Cargo.lock及相依版本未更改。

在另一份乾淨f6匯出套完整patch，逐個變更檔與候選SHA256相同、git apply --check與diff --check均0；[套用證據](final-rewire-clean-apply.json)。真實完整回歸：**67 passed、1 ignored，exit0**，[命令／限制](analytics-day45-validation-r4.json)、[完整log](analytics-day45-final-r4.log)。包含SQLx七→八份migration升級／再次執行冪等、原checksum保留、初始個人deny不被覆蓋，以及角色聯集、撤權／角色停用、page-only功能樹、action-only隱藏、first-change→下次首頁、舊Session拒絕。

未執行／尚未完成：瀏覽器互動與實際部署；使用者既有PostgreSQL備份升級；本次只驗隔離f6七份SQLx migrations→八份；ACL／role／個人權限編輯UI、帳號update/delete API與/security頁的完整實作；原legacy ignored測試1項；完整Gate2及全庫CI。Gate1是本文、完整patch、版本及實驗證據的文件交付；**候選自動測試通過不等於產品已套用或完整Gate2通過**。rust_ai_ring未寫入。下一步由使用者套用後，對實際新產品SHA驗證互動、舊DB備份與剩餘產品功能，不沿用本次候選結果冒充新SHA實測。
