# Day45 最後來源追驗｜Chat

產品`d784f32c6bff86249dd84fd4294d055a4c774a49`。最新d784產品Chat原樣重新執行44項測試通過；其Chat子樹與28相同。WASM check在28通過，未將該次WASM結果改標成新SHA實跑。原f6完整patch已採用，不能重套。

[本回合完整證據](../../../source-review/day45-2026-10-31-d784f32/README.md) · [精確SHA／實測區分](../../../source-review/day45-2026-10-31-d784f32/latest-source.json)。本段優先於以下28初次驗證及更早歷史原文。

---

# Day45 最新驗證補記｜Chat｜2026-10-31

產品精確SHA `28b71b67907062f804b2358c41fde349c9b18ca9`；本補記優先於下方歷史候選敘述。產品已採用Day45內容；本回合對28原樣跑Chat SSR回歸44通過、WASM hydrate check exit0。會員只限本人、三項action及既有排除／Session斷言承接。原文的f6完整patch現在僅供歷史重現，勿對28重套。沒有新增功能patch／Refactor計功；瀏覽器互動與完整Gate2仍未驗收。[實測log](../../../source-review/day45-2026-10-31-d784f32/chat-product.log)、[WASM](../../../source-review/day45-2026-10-31-d784f32/chat-product-wasm.log)。

[三案驗證／修正條件](../../../source-review/day45-2026-10-31-d784f32/README.md) · [命令／exit與限制](../../../source-review/day45-2026-10-31-d784f32/review.json) · [清暫存紀錄](../../../source-review/day45-2026-10-31-d784f32/cleanup.json)。本次不新增Day；next_day仍46，FIFO與last_full_comparison未變；原Red／Green／獨立Refactor與歷史證據保留。

---

# Chat Day45 r2｜會員 action 與只限本人的資料／密碼管理

## 目標、精確來源與最後需求

2026-10-31；產品唯讀 `f6a04015188da2155af6c264d0e6326e6a922dca`，文件基準 `c89ad285f757f35cb8dbf215b697ff8e5ded38c4`。[共用授權契約](../../../requirements/COMMON_AUTHORIZATION.md)、[驗證規則](../../../requirements/AUTHENTICATION.md)及最新使用者確認優先。三案next_day均46；這是既有Day45修訂，不新增Day，FIFO與last_full_comparison不變。核心60分鐘只計原Red/Green；本修訂、Refactor、版本／驗證／交付另計，不倒填為60分鐘已完成新功能。

正式接線：main→signin routes／member_self routes；會員身分由chat_session→active_member取得；/account顯示入口、/api/member-capabilities與實際mutation共用有效action。/account/rooms及/api/rooms在寫入前重驗Session與ROOM action，JSON client先取同Session的CSRF；公開GET /api/rooms保留。/account/profile只改本人資料；/account/password只改本人密碼，資料／密碼是兩個不同action。表單deny_unknown_fields拒絕傳入user_id，自己的功能路由也拒絕query指定對象，其他會員路徑不存在。角色及個人allow不會擴大操作對象。

mbr_perm只定義action；mbr_user_role指派角色；有效的mbr_role_perm取聯集，再套mbr_user_perm allow/deny，deny優先；刪個人例外回到角色預設，停用定義或角色即失效。沒有會員category/page層。新增向前migration明確建立member-default角色及三項action預設grant、給既有active會員指派此角色，註冊同交易指派；這是本候選為保留既有建立房間能力與提供必要自身功能採用的預設配置，不宣稱使用者指定所有角色均自動有權。既有個人deny不覆寫。

密碼驗目前密碼／確認、新舊不同、長度、CSRF及Origin；Argon2隨機鹽，更新與撤銷本人所有Session同交易，任何一步失敗回滾。首次狀態未完成時login固定開同一個/account/password，無action則拒絕；不能靠再次登入略過，完成後下一次登入回/account。房間與WS也擋未完成首次修改；Day45原排除檢查與每秒／收發重驗承接。

13個逐檔變更均包含於下方完整patch：chat_persistence/lib.rs、member_session.rs、room_access.rs；chat_server/auth_session.rs、lib.rs、member_self.rs、room_auth.rs、signin.rs、signup.rs、ws.rs、tests/day45_member_self.rs；chat_ui/room_api.rs；新增20261031123000_member_actions.sql。既有http.rs的低階handler測試保留；產品JSON入口改用強制授權wrapper，實際Router由新增整合測試驗證。

## Red

在獨立精確f6副本只新增下列測試，執行真Router與隔離DB；exit=101、3項失敗。[真實Red紀錄](chat-day45-final-red-r2.log)。原產品本人資料／密碼路由404；缺建立action仍實際寫入房間201而非403。後續新增的完整有效斷言包含於Green patch，不將編譯失敗或靜態hunk當成此行為Red。

```sh
CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 cargo test --manifest-path chat_apps/Cargo.toml -p chat-server --features ssr --test day45_member_self --locked
```

完整Red測試patch（在獨立Red副本使用）：

```diff
diff --git a/chat_apps/chat_server/tests/day45_member_self.rs b/chat_apps/chat_server/tests/day45_member_self.rs
new file mode 100644
index 0000000..4de4dcc
--- /dev/null
+++ b/chat_apps/chat_server/tests/day45_member_self.rs
@@ -0,0 +1,213 @@
+use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
+use axum::{
+    Router,
+    body::{Body, to_bytes},
+    http::{Request, StatusCode},
+};
+use shared::auth::token_digest;
+use tower::ServiceExt;
+async fn send(
+    app: Router,
+    path: &str,
+    body: Option<String>,
+    cookie: &str,
+) -> axum::response::Response {
+    let mut req = Request::builder()
+        .method(if body.is_some() { "POST" } else { "GET" })
+        .uri(path)
+        .header("cookie", cookie)
+        .header("origin", "http://127.0.0.1:3000");
+    if body.is_some() {
+        req = req.header("content-type", "application/x-www-form-urlencoded");
+    }
+    app.oneshot(req.body(Body::from(body.unwrap_or_default())).unwrap())
+        .await
+        .unwrap()
+}
+async fn fixture() -> (sqlx::SqlitePool, Router, String) {
+    let pool = sqlx::sqlite::SqlitePoolOptions::new()
+        .max_connections(1)
+        .connect("sqlite::memory:")
+        .await
+        .unwrap();
+    chat_persistence::migrate(&pool).await.unwrap();
+    let hash = Argon2::default()
+        .hash_password(b"member-password")
+        .unwrap()
+        .to_string();
+    for id in ["self-member", "other-member"] {
+        sqlx::query("INSERT INTO mbr_user(id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at) VALUES(?1,?1,?2,?1||'@example.test',?1,'active',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)").bind(id).bind(&hash).execute(&pool).await.unwrap();
+    }
+    for (id, name) in [
+        ("member-room-create", "建立聊天室"),
+        ("member-self-profile-update", "修改本人資料"),
+        ("member-self-password-update", "修改本人密碼"),
+    ] {
+        sqlx::query("INSERT INTO mbr_perm(id,perm_name,perm_status,created_at,updated_at) VALUES(?1,?2,'active',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING").bind(id).bind(name).execute(&pool).await.unwrap();
+        sqlx::query("INSERT INTO mbr_user_perm(user_id,perm_id,perm_filter,created_at) VALUES('self-member',?1,'allow',CURRENT_TIMESTAMP)").bind(id).execute(&pool).await.unwrap();
+    }
+    let raw = "a".repeat(64);
+    sqlx::query("INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at) VALUES('fixture-session','self-member',?1,unixepoch()+600)").bind(token_digest(&raw)).execute(&pool).await.unwrap();
+    let app = chat_server::signin::routes(pool.clone());
+    (pool, app, format!("chat_session={raw}"))
+}
+fn csrf(cookie: &str, operation: &str) -> String {
+    token_digest(&format!(
+        "{operation}:{}",
+        cookie.strip_prefix("chat_session=").unwrap()
+    ))
+}
+#[tokio::test]
+async fn self_profile_is_separate_and_cannot_target_another_member() {
+    let (pool, app, cookie) = fixture().await;
+    assert_eq!(
+        send(app.clone(), "/account/profile", None, &cookie)
+            .await
+            .status(),
+        StatusCode::OK
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/account/profile?user_id=other-member",
+            None,
+            &cookie
+        )
+        .await
+        .status(),
+        StatusCode::UNPROCESSABLE_ENTITY
+    );
+    let payload = format!(
+        "display_name=Changed&email=changed%40example.test&csrf={}",
+        csrf(&cookie, "member-profile")
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/account/profile",
+            Some(format!("{payload}&user_id=other-member")),
+            &cookie
+        )
+        .await
+        .status(),
+        StatusCode::UNPROCESSABLE_ENTITY
+    );
+    let unchanged: String =
+        sqlx::query_scalar("SELECT user_name FROM mbr_user WHERE id='other-member'")
+            .fetch_one(&pool)
+            .await
+            .unwrap();
+    assert_eq!(unchanged, "other-member");
+    assert_eq!(
+        send(
+            app.clone(),
+            "/account/profile",
+            Some(payload.clone()),
+            &cookie
+        )
+        .await
+        .status(),
+        StatusCode::SEE_OTHER
+    );
+    let name: String = sqlx::query_scalar("SELECT user_name FROM mbr_user WHERE id='self-member'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(name, "Changed");
+    sqlx::query("UPDATE mbr_user_perm SET perm_filter='deny' WHERE user_id='self-member' AND perm_id='member-self-profile-update'").execute(&pool).await.unwrap();
+    assert_eq!(
+        send(app.clone(), "/account/profile", Some(payload), &cookie)
+            .await
+            .status(),
+        StatusCode::FORBIDDEN
+    );
+    assert_eq!(
+        send(app, "/account/password", None, &cookie).await.status(),
+        StatusCode::OK,
+        "profile deny must not remove password action"
+    );
+}
+#[tokio::test]
+async fn room_creation_requires_action_and_deny_leaves_no_room() {
+    let (pool, app, cookie) = fixture().await;
+    sqlx::query("UPDATE mbr_user_perm SET perm_filter='deny' WHERE user_id='self-member' AND perm_id='member-room-create'").execute(&pool).await.unwrap();
+    let form = format!("name=Forbidden&csrf={}", csrf(&cookie, "create-room"));
+    assert_eq!(
+        send(app.clone(), "/account/rooms", Some(form), &cookie)
+            .await
+            .status(),
+        StatusCode::FORBIDDEN
+    );
+    let response = send(app.clone(), "/api/member-capabilities", None, &cookie).await;
+    assert_eq!(response.status(), StatusCode::OK);
+    let bytes = to_bytes(response.into_body(), 65536).await.unwrap();
+    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
+    assert_eq!(value["create_room"], false);
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM rooms")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count, 0);
+    let body = to_bytes(
+        send(app, "/account", None, &cookie).await.into_body(),
+        65536,
+    )
+    .await
+    .unwrap();
+    assert!(!String::from_utf8_lossy(&body).contains("action=\"/account/rooms\""));
+}
+#[tokio::test]
+async fn password_change_is_self_only_csrf_checked_salted_and_revokes_sessions() {
+    let (pool, app, cookie) = fixture().await;
+    let form = format!(
+        "current_password=member-password&new_password=changed-password&confirm_password=changed-password&csrf={}",
+        csrf(&cookie, "member-password")
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/account/password",
+            Some(format!("{form}&user_id=other-member")),
+            &cookie
+        )
+        .await
+        .status(),
+        StatusCode::UNPROCESSABLE_ENTITY
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/account/password",
+            Some(form.replace(&csrf(&cookie, "member-password"), "wrong")),
+            &cookie
+        )
+        .await
+        .status(),
+        StatusCode::FORBIDDEN
+    );
+    assert_eq!(
+        send(app.clone(), "/account/password", Some(form), &cookie)
+            .await
+            .status(),
+        StatusCode::SEE_OTHER
+    );
+    let hash: String = sqlx::query_scalar("SELECT user_pd FROM mbr_user WHERE id='self-member'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert!(hash.starts_with("$argon2id$"));
+    Argon2::default()
+        .verify_password(b"changed-password", &PasswordHash::new(&hash).unwrap())
+        .unwrap();
+    let other: String = sqlx::query_scalar("SELECT user_pd FROM mbr_user WHERE id='other-member'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    Argon2::default()
+        .verify_password(b"member-password", &PasswordHash::new(&other).unwrap())
+        .unwrap();
+    assert_eq!(
+        send(app, "/account/password", None, &cookie).await.status(),
+        StatusCode::UNAUTHORIZED
+    );
+}
```

## Green

[完整組合patch](chat-day45-final-r2.patch)直接對精確f6套用，已包含原Day45功能、Session Refactor與本修訂測試；**不要與舊Day45 Green／Refactor或本次Red patch累加**。讀者先在獨立Red副本重現失敗，再在乾淨f6副本套此完整patch。

```sh
git rev-parse HEAD # 必須是 f6a04015188da2155af6c264d0e6326e6a922dca
git apply --check chat-day45-final-r2.patch
git apply chat-day45-final-r2.patch
cargo test --manifest-path chat_apps/Cargo.toml -p chat-server --features ssr --locked --no-fail-fast
```

逐檔完整變更如下，不省略新檔或caller接線：

```diff
diff --git a/chat_apps/chat_persistence/src/lib.rs b/chat_apps/chat_persistence/src/lib.rs
index cc44d6d..732f250 100644
--- a/chat_apps/chat_persistence/src/lib.rs
+++ b/chat_apps/chat_persistence/src/lib.rs
@@ -1,3 +1,5 @@
+pub mod member_session;
+pub mod room_access;
 use async_trait::async_trait;
 use chat_application::repository::{MessageRepository, RepositoryError, RoomRepository};
 use chat_domain::Room;
@@ -107,11 +109,11 @@ impl RoomRepository for SqliteRoomRepository {
         sqlx::query_as::<_, RoomRow>(
             r"SELECT id
                , category_room_id
-               , name 
-            FROM rooms 
-            WHERE name 
-            LIKE ?1 
-            ESCAPE '\' 
+               , name
+            FROM rooms
+            WHERE name
+            LIKE ?1
+            ESCAPE '\'
             ORDER BY name, id",
         )
         .bind(pattern)
diff --git a/chat_apps/chat_persistence/src/member_session.rs b/chat_apps/chat_persistence/src/member_session.rs
new file mode 100644
index 0000000..6725386
--- /dev/null
+++ b/chat_apps/chat_persistence/src/member_session.rs
@@ -0,0 +1,15 @@
+use sqlx::SqlitePool;
+
+/// SQLite adapter accepts a digest, never a raw bearer token or HTTP headers.
+pub async fn active_member_id(
+    pool: &SqlitePool,
+    digest: &str,
+) -> Result<Option<String>, sqlx::Error> {
+    sqlx::query_scalar(
+        "SELECT u.id FROM mbr_session s JOIN mbr_user u ON u.id=s.mbr_user_id
+         WHERE s.token_hash=?1 AND s.expires_at>unixepoch() AND u.user_status='active'",
+    )
+    .bind(digest)
+    .fetch_optional(pool)
+    .await
+}
diff --git a/chat_apps/chat_persistence/src/room_access.rs b/chat_apps/chat_persistence/src/room_access.rs
new file mode 100644
index 0000000..934db68
--- /dev/null
+++ b/chat_apps/chat_persistence/src/room_access.rs
@@ -0,0 +1,12 @@
+use sqlx::SqlitePool;
+
+/// Explicit exclusion takes precedence. This slice leaves existing public-room semantics intact.
+pub async fn is_excluded(pool: &SqlitePool, member: &str, room: &str) -> Result<bool, sqlx::Error> {
+    sqlx::query_scalar(
+        "SELECT EXISTS(SELECT 1 FROM mbr_exclude WHERE mbr_user_id=?1 AND room_id=?2)",
+    )
+    .bind(member)
+    .bind(room)
+    .fetch_one(pool)
+    .await
+}
diff --git a/chat_apps/chat_server/src/auth_session.rs b/chat_apps/chat_server/src/auth_session.rs
index b2ff414..80e0629 100644
--- a/chat_apps/chat_server/src/auth_session.rs
+++ b/chat_apps/chat_server/src/auth_session.rs
@@ -14,19 +14,9 @@ fn session_cookie(headers: &HeaderMap) -> Result<String, StatusCode> {
 /// No plaintext bearer token is ever stored in SQLite.
 pub async fn active_member(pool: &SqlitePool, headers: &HeaderMap) -> Result<String, StatusCode> {
     let raw = session_cookie(headers)?;
-    let member: Option<String> = sqlx::query_scalar(
-        r#"SELECT u.id 
-        FROM mbr_session s
-        JOIN mbr_user u 
-            ON u.id=s.mbr_user_id
-        WHERE s.token_hash=?1 
-          AND s.expires_at>unixepoch()
-          AND u.user_status='active'"#,
-    )
-    .bind(token_digest(&raw))
-    .fetch_optional(pool)
-    .await
-    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    let member = chat_persistence::member_session::active_member_id(pool, &token_digest(&raw))
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
     member.ok_or(StatusCode::UNAUTHORIZED)
 }
 
diff --git a/chat_apps/chat_server/src/lib.rs b/chat_apps/chat_server/src/lib.rs
index 96f85b5..1d054a4 100644
--- a/chat_apps/chat_server/src/lib.rs
+++ b/chat_apps/chat_server/src/lib.rs
@@ -2,6 +2,7 @@ pub mod auth_session;
 pub mod http;
 pub mod hub;
 pub mod logout;
+pub mod member_self;
 pub mod room_auth;
 mod room_path;
 pub mod signin;
@@ -61,10 +62,8 @@ pub async fn application() -> Result<(), Box<dyn std::error::Error>> {
 
     // 建立房間需要有效會員；公開列表保持可讀。
     // GET 公開；只替 POST method 加上會員 Session middleware。
-    let room_methods =
-        get(http::list_rooms).merge(axum::routing::post(http::create_room).route_layer(
-            axum::middleware::from_fn_with_state(db.clone(), room_auth::require_member),
-        ));
+    let room_methods = get(http::list_rooms)
+        .merge(axum::routing::post(member_self::create_room_api).with_state(db.clone()));
 
     let app = Router::new()
         .route_service(&pkg_path, pkg_service)
diff --git a/chat_apps/chat_server/src/member_self.rs b/chat_apps/chat_server/src/member_self.rs
new file mode 100644
index 0000000..f8f2b20
--- /dev/null
+++ b/chat_apps/chat_server/src/member_self.rs
@@ -0,0 +1,369 @@
+//! Member self-service: actions only; identity and scope always come from the member Session.
+use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
+use axum::{
+    Form, Json, Router,
+    extract::{DefaultBodyLimit, State},
+    http::{HeaderMap, StatusCode, header},
+    response::{Html, IntoResponse, Redirect, Response},
+    routing::get,
+};
+use shared::auth::{cookie_token, token_digest};
+use sqlx::SqlitePool;
+pub const ROOM: &str = "member-room-create";
+pub const PROFILE: &str = "member-self-profile-update";
+pub const PASSWORD: &str = "member-self-password-update";
+const PERMISSION: &str = r#"SELECT EXISTS(SELECT 1 FROM mbr_perm p JOIN mbr_user u ON u.id=?1
+WHERE p.id=?2 AND p.perm_status='active' AND u.user_status='active'
+AND NOT EXISTS(SELECT 1 FROM mbr_user_perm x WHERE x.user_id=u.id AND x.perm_id=p.id AND x.perm_filter='deny')
+AND (EXISTS(SELECT 1 FROM mbr_user_perm x WHERE x.user_id=u.id AND x.perm_id=p.id AND x.perm_filter='allow')
+OR EXISTS(SELECT 1 FROM mbr_user_role ur JOIN mbr_role r ON r.id=ur.role_id
+JOIN mbr_role_perm rp ON rp.role_id=r.id AND rp.perm_id=p.id
+WHERE ur.user_id=u.id AND r.role_status='active')))"#;
+fn db_error(e: sqlx::Error) -> StatusCode {
+    match e {
+        sqlx::Error::Database(d) if d.is_unique_violation() => StatusCode::CONFLICT,
+        _ => StatusCode::SERVICE_UNAVAILABLE,
+    }
+}
+fn raw(headers: &HeaderMap) -> Result<String, StatusCode> {
+    headers
+        .get(header::COOKIE)
+        .and_then(|v| v.to_str().ok())
+        .and_then(|v| cookie_token(v, "chat_session"))
+        .ok_or(StatusCode::UNAUTHORIZED)
+}
+fn csrf(headers: &HeaderMap, operation: &str) -> Result<String, StatusCode> {
+    Ok(token_digest(&format!("{operation}:{}", raw(headers)?)))
+}
+fn verify_csrf(headers: &HeaderMap, operation: &str, value: &str) -> Result<(), StatusCode> {
+    if headers.get(header::ORIGIN).and_then(|h| h.to_str().ok()) != Some("http://127.0.0.1:3000")
+        || csrf(headers, operation)? != value
+    {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    Ok(())
+}
+pub async fn allowed(pool: &SqlitePool, user: &str, action: &str) -> Result<bool, StatusCode> {
+    sqlx::query_scalar(PERMISSION)
+        .bind(user)
+        .bind(action)
+        .fetch_one(pool)
+        .await
+        .map_err(db_error)
+}
+pub async fn require_action(
+    pool: &SqlitePool,
+    headers: &HeaderMap,
+    action: &str,
+) -> Result<String, StatusCode> {
+    let user = crate::auth_session::active_member(pool, headers).await?;
+    if !allowed(pool, &user, action).await? {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    let initial: bool = sqlx::query_scalar("SELECT is_init_pd FROM mbr_user WHERE id=?1")
+        .bind(&user)
+        .fetch_one(pool)
+        .await
+        .map_err(db_error)?;
+    if initial && action != PASSWORD {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    Ok(user)
+}
+// Revalidate identity and action in the same SQLite transaction as the mutation.
+async fn transaction_user(
+    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
+    headers: &HeaderMap,
+    action: &str,
+) -> Result<(String, String), StatusCode> {
+    let row:Option<(String,String,bool)>=sqlx::query_as("SELECT u.id,u.user_pd,u.is_init_pd FROM mbr_user u JOIN mbr_session s ON s.mbr_user_id=u.id WHERE s.token_hash=?1 AND s.expires_at>unixepoch() AND u.user_status='active'").bind(token_digest(&raw(headers)?)).fetch_optional(&mut **tx).await.map_err(db_error)?;
+    let (id, hash, initial) = row.ok_or(StatusCode::UNAUTHORIZED)?;
+    let permit: bool = sqlx::query_scalar(PERMISSION)
+        .bind(&id)
+        .bind(action)
+        .fetch_one(&mut **tx)
+        .await
+        .map_err(db_error)?;
+    if !permit || (initial && action != PASSWORD) {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    Ok((id, hash))
+}
+pub fn routes(pool: SqlitePool) -> Router {
+    Router::new()
+        .route("/api/member-capabilities", get(capabilities))
+        .route("/account/profile", get(profile_page).post(profile))
+        .route("/account/password", get(password_page).post(password))
+        .route_layer(axum::middleware::from_fn(reject_target_query))
+        .layer(DefaultBodyLimit::max(4096))
+        .with_state(pool)
+}
+fn page(title: &str, form: &str) -> Html<String> {
+    Html(format!(
+        "<!doctype html><html lang=\"zh-Hant\"><head><meta charset=\"utf-8\"><title>{title}</title></head><body><main><h1>{title}</h1>{form}<a href=\"/account\">個人管理</a></main></body></html>"
+    ))
+}
+async fn profile_page(
+    State(pool): State<SqlitePool>,
+    headers: HeaderMap,
+) -> Result<impl IntoResponse, StatusCode> {
+    require_action(&pool, &headers, PROFILE).await?;
+    Ok((
+        [(header::CACHE_CONTROL, "no-store")],
+        page(
+            "修改本人資料",
+            &format!(
+                "<form method=\"post\" action=\"/account/profile\"><input type=\"hidden\" name=\"csrf\" value=\"{}\"><label>顯示名稱<input name=\"display_name\" required maxlength=\"32\"></label><label>Email<input type=\"email\" name=\"email\" required maxlength=\"254\"></label><button>儲存本人資料</button></form>",
+                csrf(&headers, "member-profile")?
+            ),
+        ),
+    ))
+}
+async fn password_page(
+    State(pool): State<SqlitePool>,
+    headers: HeaderMap,
+) -> Result<impl IntoResponse, StatusCode> {
+    let user = require_action(&pool, &headers, PASSWORD).await?;
+    let initial: bool = sqlx::query_scalar("SELECT is_init_pd FROM mbr_user WHERE id=?1")
+        .bind(user)
+        .fetch_one(&pool)
+        .await
+        .map_err(db_error)?;
+    Ok((
+        [(header::CACHE_CONTROL, "no-store")],
+        page(
+            if initial {
+                "首次登入須修改本人密碼"
+            } else {
+                "修改本人密碼"
+            },
+            &format!(
+                "<form method=\"post\" action=\"/account/password\"><input type=\"hidden\" name=\"csrf\" value=\"{}\"><label>目前密碼<input type=\"password\" name=\"current_password\" autocomplete=\"current-password\" required maxlength=\"128\"></label><label>新密碼<input type=\"password\" name=\"new_password\" autocomplete=\"new-password\" required minlength=\"8\" maxlength=\"128\"></label><label>確認新密碼<input type=\"password\" name=\"confirm_password\" autocomplete=\"new-password\" required minlength=\"8\" maxlength=\"128\"></label><button>修改本人密碼</button></form>",
+                csrf(&headers, "member-password")?
+            ),
+        ),
+    ))
+}
+#[derive(serde::Deserialize)]
+#[serde(deny_unknown_fields)]
+struct Profile {
+    display_name: String,
+    email: String,
+    csrf: String,
+}
+async fn profile(
+    State(pool): State<SqlitePool>,
+    headers: HeaderMap,
+    Form(form): Form<Profile>,
+) -> Result<Response, StatusCode> {
+    require_action(&pool, &headers, PROFILE).await?;
+    verify_csrf(&headers, "member-profile", &form.csrf)?;
+    let name = form.display_name.trim();
+    let email = form.email.trim().to_ascii_lowercase();
+    let parts: Vec<_> = email.split('@').collect();
+    if !(1..=32).contains(&name.chars().count())
+        || name.chars().any(char::is_control)
+        || !email.is_ascii()
+        || email.len() > 254
+        || email
+            .bytes()
+            .any(|b| b.is_ascii_whitespace() || b.is_ascii_control())
+        || parts.len() != 2
+        || parts[0].is_empty()
+        || !parts[1].contains('.')
+        || parts[1].starts_with('.')
+        || parts[1].ends_with('.')
+    {
+        return Err(StatusCode::UNPROCESSABLE_ENTITY);
+    }
+    let mut tx = pool.begin().await.map_err(db_error)?;
+    let (id, _) = transaction_user(&mut tx, &headers, PROFILE).await?;
+    sqlx::query(
+        "UPDATE mbr_user SET user_name=?1,user_email=?2,updated_at=CURRENT_TIMESTAMP WHERE id=?3",
+    )
+    .bind(name)
+    .bind(email)
+    .bind(id)
+    .execute(&mut *tx)
+    .await
+    .map_err(db_error)?;
+    tx.commit().await.map_err(db_error)?;
+    Ok((
+        [(header::CACHE_CONTROL, "no-store")],
+        Redirect::to("/account"),
+    )
+        .into_response())
+}
+#[derive(serde::Deserialize)]
+#[serde(deny_unknown_fields)]
+struct Password {
+    current_password: String,
+    new_password: String,
+    confirm_password: String,
+    csrf: String,
+}
+async fn password(
+    State(pool): State<SqlitePool>,
+    headers: HeaderMap,
+    Form(form): Form<Password>,
+) -> Result<Response, StatusCode> {
+    let user = require_action(&pool, &headers, PASSWORD).await?;
+    verify_csrf(&headers, "member-password", &form.csrf)?;
+    if form.current_password.is_empty()
+        || form.current_password.len() > 128
+        || !(8..=128).contains(&form.new_password.len())
+        || form.new_password != form.confirm_password
+        || form.new_password == form.current_password
+    {
+        return Err(StatusCode::UNPROCESSABLE_ENTITY);
+    }
+    let current: String = sqlx::query_scalar("SELECT user_pd FROM mbr_user WHERE id=?1")
+        .bind(user)
+        .fetch_one(&pool)
+        .await
+        .map_err(db_error)?;
+    let checked = current.clone();
+    let encoded = tokio::task::spawn_blocking(move || {
+        let parsed = PasswordHash::new(&checked).map_err(|_| StatusCode::UNAUTHORIZED)?;
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
+    let mut tx = pool.begin().await.map_err(db_error)?;
+    let (id, hash) = transaction_user(&mut tx, &headers, PASSWORD).await?;
+    if hash != current {
+        return Err(StatusCode::UNAUTHORIZED);
+    }
+    sqlx::query(
+        "UPDATE mbr_user SET user_pd=?1,is_init_pd=FALSE,updated_at=CURRENT_TIMESTAMP WHERE id=?2",
+    )
+    .bind(encoded)
+    .bind(&id)
+    .execute(&mut *tx)
+    .await
+    .map_err(db_error)?;
+    sqlx::query("DELETE FROM mbr_session WHERE mbr_user_id=?1")
+        .bind(&id)
+        .execute(&mut *tx)
+        .await
+        .map_err(db_error)?;
+    tx.commit().await.map_err(db_error)?;
+    Ok((
+        [
+            (
+                header::SET_COOKIE,
+                "chat_session=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0",
+            ),
+            (header::CACHE_CONTROL, "no-store"),
+        ],
+        Redirect::to("/signin"),
+    )
+        .into_response())
+}
+pub async fn create_room(
+    pool: &SqlitePool,
+    headers: &HeaderMap,
+    name: String,
+) -> Result<String, StatusCode> {
+    require_action(pool, headers, ROOM).await?;
+    if chat_application::validate_room_name(&name).is_err() {
+        return Err(StatusCode::UNPROCESSABLE_ENTITY);
+    }
+    let mut tx = pool.begin().await.map_err(db_error)?;
+    transaction_user(&mut tx, headers, ROOM).await?;
+    let id = uuid::Uuid::new_v4().to_string();
+    sqlx::query("INSERT INTO rooms(id,category_room_id,name,desc,created_at,updated_at) VALUES(?1,'default-category',?2,'',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)").bind(&id).bind(name).execute(&mut *tx).await.map_err(db_error)?;
+    tx.commit().await.map_err(db_error)?;
+    Ok(id)
+}
+pub async fn create_room_api(
+    State(pool): State<SqlitePool>,
+    headers: HeaderMap,
+    Json(input): Json<shared::CreateRoomRequest>,
+) -> Result<impl IntoResponse, StatusCode> {
+    require_action(&pool, &headers, ROOM).await?;
+    let token = headers
+        .get("x-csrf-token")
+        .and_then(|v| v.to_str().ok())
+        .ok_or(StatusCode::FORBIDDEN)?;
+    verify_csrf(&headers, "create-room", token)?;
+    let id = create_room(&pool, &headers, input.name).await?;
+    Ok((StatusCode::CREATED, Json(shared::CreateRoomResponse { id })))
+}
+pub async fn account_markup(
+    pool: &SqlitePool,
+    headers: &HeaderMap,
+    mut html: String,
+) -> Result<String, StatusCode> {
+    let user = crate::auth_session::active_member(pool, headers).await?;
+    let initial: bool = sqlx::query_scalar("SELECT is_init_pd FROM mbr_user WHERE id=?1")
+        .bind(&user)
+        .fetch_one(pool)
+        .await
+        .map_err(db_error)?;
+    if !allowed(pool, &user, ROOM).await? || initial {
+        if let Some(start) = html.find("<form method=\"post\" action=\"/account/rooms\"") {
+            if let Some(end) = html[start..].find("</form>") {
+                html.replace_range(start..start + end + 7, "");
+            }
+        }
+    }
+    let mut links = String::new();
+    for (action, path, label) in [
+        (PROFILE, "/account/profile", "修改本人資料"),
+        (PASSWORD, "/account/password", "修改本人密碼"),
+    ] {
+        if allowed(pool, &user, action).await? && (!initial || action == PASSWORD) {
+            links.push_str(&format!("<a href=\"{path}\">{label}</a>"));
+        }
+    }
+    Ok(html.replacen("</main>", &format!("{links}</main>"), 1))
+}
+
+pub async fn initial(pool: &SqlitePool, user: &str) -> Result<bool, StatusCode> {
+    sqlx::query_scalar("SELECT is_init_pd FROM mbr_user WHERE id=?1")
+        .bind(user)
+        .fetch_one(pool)
+        .await
+        .map_err(db_error)
+}
+
+#[derive(serde::Serialize)]
+struct Capabilities {
+    create_room: bool,
+    update_profile: bool,
+    update_password: bool,
+    create_csrf: String,
+}
+async fn capabilities(
+    State(pool): State<SqlitePool>,
+    headers: HeaderMap,
+) -> Result<impl IntoResponse, StatusCode> {
+    let user = crate::auth_session::active_member(&pool, &headers).await?;
+    let restricted = initial(&pool, &user).await?;
+    Ok((
+        [(header::CACHE_CONTROL, "no-store")],
+        Json(Capabilities {
+            create_room: !restricted && allowed(&pool, &user, ROOM).await?,
+            update_profile: !restricted && allowed(&pool, &user, PROFILE).await?,
+            update_password: allowed(&pool, &user, PASSWORD).await?,
+            create_csrf: csrf(&headers, "create-room")?,
+        }),
+    ))
+}
+
+async fn reject_target_query(
+    request: axum::extract::Request,
+    next: axum::middleware::Next,
+) -> Result<Response, StatusCode> {
+    if request.uri().query().is_some_and(|q| !q.is_empty()) {
+        return Err(StatusCode::UNPROCESSABLE_ENTITY);
+    }
+    Ok(next.run(request).await)
+}
diff --git a/chat_apps/chat_server/src/room_auth.rs b/chat_apps/chat_server/src/room_auth.rs
index 88ca6ad..8980638 100644
--- a/chat_apps/chat_server/src/room_auth.rs
+++ b/chat_apps/chat_server/src/room_auth.rs
@@ -13,7 +13,10 @@ pub async fn require_member(
     request: Request,
     next: Next,
 ) -> Result<Response, StatusCode> {
-    active_member(&pool, request.headers()).await?;
+    let user = active_member(&pool, request.headers()).await?;
+    if crate::member_self::initial(&pool, &user).await? {
+        return Err(StatusCode::FORBIDDEN);
+    }
     Ok(next.run(request).await)
 }
 
@@ -24,7 +27,10 @@ pub async fn require_room_member(
     next: Next,
 ) -> Result<Response, StatusCode> {
     match active_member(&pool, request.headers()).await {
-        Ok(_) => {
+        Ok(member) => {
+            if crate::member_self::initial(&pool, &member).await? {
+                return Ok(Redirect::to("/account/password").into_response());
+            }
             // 登入成功也不能讓不存在的房間進入 SSR／WebSocket 畫面。
             // 沿用正式 SQLite RoomRepository，而不是在 middleware 複製 SQL。
             use chat_application::repository::RoomRepository;
@@ -37,6 +43,14 @@ pub async fn require_room_member(
             if !room.is_some_and(|room| room.id == id) {
                 return Err(StatusCode::NOT_FOUND);
             }
+            if chat_persistence::room_access::is_excluded(&pool, &member, &id)
+                .await
+                .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
+            {
+                return Ok(
+                    (StatusCode::FORBIDDEN, "您已被此聊天室排除，無法進入。").into_response()
+                );
+            }
             Ok(next.run(request).await)
         }
         Err(StatusCode::UNAUTHORIZED) => {
@@ -298,3 +312,76 @@ mod day44_existing_room_gate_tests {
         db.close().await;
     }
 }
+
+#[cfg(test)]
+mod day45_tests {
+    use super::*;
+    use axum::{Router, body::Body, http::Request, routing::get};
+    use tower::ServiceExt;
+    #[tokio::test]
+    async fn excluded_member_cannot_enter_but_public_listing_stays_public() {
+        let db = sqlx::sqlite::SqlitePoolOptions::new()
+            .max_connections(1)
+            .connect("sqlite::memory:")
+            .await
+            .unwrap();
+        chat_persistence::migrate(&db).await.unwrap();
+        let id = uuid::Uuid::new_v4().to_string();
+        sqlx::query("INSERT INTO rooms(id,category_room_id,name,desc,created_at,updated_at) VALUES(?,'default-category','ACL','',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
+            .bind(&id).execute(&db).await.unwrap();
+        sqlx::query("INSERT INTO mbr_user(id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at) VALUES('u45','u45','hash','u45@example.test','Member','active',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
+            .execute(&db).await.unwrap();
+        let token = "d".repeat(64);
+        sqlx::query("INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at) VALUES('s45','u45',?,unixepoch()+60)")
+            .bind(shared::auth::token_digest(&token)).execute(&db).await.unwrap();
+        let app = Router::new()
+            .route("/room/{room_id}", get(|| async { "ROOM" }))
+            .route_layer(axum::middleware::from_fn_with_state(
+                db.clone(),
+                require_room_member,
+            ))
+            .route("/api/rooms", get(|| async { "PUBLIC" }));
+        for excluded in [false, true, false] {
+            sqlx::query("DELETE FROM mbr_exclude WHERE mbr_user_id='u45'")
+                .execute(&db)
+                .await
+                .unwrap();
+            if excluded {
+                sqlx::query(
+                    "INSERT INTO mbr_exclude(id,mbr_user_id,room_id) VALUES('e45','u45',?)",
+                )
+                .bind(&id)
+                .execute(&db)
+                .await
+                .unwrap();
+            }
+            let response = app
+                .clone()
+                .oneshot(
+                    Request::get(format!("/room/{id}"))
+                        .header("cookie", format!("chat_session={token}"))
+                        .body(Body::empty())
+                        .unwrap(),
+                )
+                .await
+                .unwrap();
+            assert_eq!(
+                response.status(),
+                if excluded {
+                    StatusCode::FORBIDDEN
+                } else {
+                    StatusCode::OK
+                }
+            );
+            assert_eq!(
+                app.clone()
+                    .oneshot(Request::get("/api/rooms").body(Body::empty()).unwrap())
+                    .await
+                    .unwrap()
+                    .status(),
+                StatusCode::OK
+            );
+        }
+        db.close().await;
+    }
+}
diff --git a/chat_apps/chat_server/src/signin.rs b/chat_apps/chat_server/src/signin.rs
index cee7ea9..002ab99 100644
--- a/chat_apps/chat_server/src/signin.rs
+++ b/chat_apps/chat_server/src/signin.rs
@@ -25,7 +25,8 @@ pub fn routes(pool: SqlitePool) -> Router {
         .route("/api/session", get(session_status))
         .route("/account/rooms", axum::routing::post(account_create_room))
         .layer(DefaultBodyLimit::max(4096))
-        .with_state(pool)
+        .with_state(pool.clone())
+        .merge(crate::member_self::routes(pool))
 }
 
 /// Valid, active SQLite member Session; never expose the bearer token.
@@ -66,11 +67,11 @@ async fn login(
         return Err(StatusCode::UNAUTHORIZED);
     }
 
-    let found: Option<(String, String, bool)> = sqlx::query_as(
+    let found: Option<(String, String, bool, bool)> = sqlx::query_as(
         r#"SELECT id
                 ,user_pd
-                ,(user_status='active') 
-            FROM mbr_user 
+                ,(user_status='active'),is_init_pd
+            FROM mbr_user
             WHERE lower(user_acnt)=lower(?1)"#,
     )
     .bind(name)
@@ -78,7 +79,7 @@ async fn login(
     .await
     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
 
-    let (user_id, hash, active) = found.ok_or(StatusCode::UNAUTHORIZED)?;
+    let (user_id, hash, active, initial) = found.ok_or(StatusCode::UNAUTHORIZED)?;
     if !active {
         return Err(StatusCode::UNAUTHORIZED);
     }
@@ -109,7 +110,11 @@ async fn login(
     ).bind(id).bind(user_id).bind(hashed).execute(&pool).await
         .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;
     let cookie = format!("chat_session={token}; HttpOnly; SameSite=Lax; Path=/; Max-Age=86400");
-    let destination = safe_room_return(&input.next).unwrap_or("/account");
+    let destination = if initial {
+        "/account/password"
+    } else {
+        safe_room_return(&input.next).unwrap_or("/account")
+    };
     Ok((
         StatusCode::SEE_OTHER,
         [
@@ -139,8 +144,19 @@ async fn account(
     State(pool): State<SqlitePool>,
     headers: HeaderMap,
 ) -> Result<Html<String>, StatusCode> {
-    crate::auth_session::active_member(&pool, &headers).await?;
-    account_html(&headers, String::new(), String::new(), None)
+    let user = crate::auth_session::active_member(&pool, &headers).await?;
+    let initial: bool = sqlx::query_scalar("SELECT is_init_pd FROM mbr_user WHERE id=?1")
+        .bind(user)
+        .fetch_one(&pool)
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    if initial {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    let Html(html) = account_html(&headers, String::new(), String::new(), None)?;
+    Ok(Html(
+        crate::member_self::account_markup(&pool, &headers, html).await?,
+    ))
 }
 
 #[derive(Deserialize)]
@@ -154,13 +170,12 @@ async fn account_create_room(
     headers: HeaderMap,
     Form(form): Form<AccountRoomForm>,
 ) -> Result<Response, StatusCode> {
-    crate::auth_session::active_member(&pool, &headers).await?;
+    crate::member_self::require_action(&pool, &headers, crate::member_self::ROOM).await?;
     if headers.get(header::ORIGIN).and_then(|h| h.to_str().ok()) != Some("http://127.0.0.1:3000")
         || form.csrf != crate::auth_session::csrf_for_create_room(&headers)?
     {
         return Err(StatusCode::FORBIDDEN);
     }
-
     if let Err(message) = chat_application::validate_room_name(&form.name) {
         return Ok((
             StatusCode::UNPROCESSABLE_ENTITY,
@@ -168,38 +183,18 @@ async fn account_create_room(
         )
             .into_response());
     }
-    let repo = chat_persistence::SqliteRoomRepository::new(pool);
-    let created = chat_application::create_room(
-        &repo,
-        chat_application::CreateRoom {
-            id: uuid::Uuid::new_v4().to_string(),
-            category_room_id: "default-category".into(),
-            name: form.name.clone(),
-        },
+    let id = crate::member_self::create_room(&pool, &headers, form.name).await?;
+    let Html(html) = account_html(
+        &headers,
+        "已建立；可進房或回公開清單".into(),
+        String::new(),
+        Some(id),
+    )?;
+    Ok((
+        StatusCode::CREATED,
+        Html(crate::member_self::account_markup(&pool, &headers, html).await?),
     )
-    .await;
-    match created {
-        Ok(room) => Ok((
-            StatusCode::CREATED,
-            account_html(
-                &headers,
-                "已建立；可進房或回公開清單".into(),
-                String::new(),
-                Some(room.id),
-            )?,
-        )
-            .into_response()),
-        Err(_) => Ok((
-            StatusCode::SERVICE_UNAVAILABLE,
-            account_html(
-                &headers,
-                "未確認建立成功；請先查清單再決定是否重送".into(),
-                form.name,
-                None,
-            )?,
-        )
-            .into_response()),
-    }
+        .into_response())
 }
 
 #[cfg(test)]
@@ -370,6 +365,7 @@ mod day35_account_room_tests {
         chat_persistence::migrate(&pool).await.unwrap();
         sqlx::query("INSERT INTO mbr_user (id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at) VALUES ('day35','learner35','unused','day35@example.test','Learner','active',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
             .execute(&pool).await.unwrap();
+        sqlx::query("INSERT INTO mbr_user_role(user_id,role_id,created_at) VALUES('day35','member-default',CURRENT_TIMESTAMP)").execute(&pool).await.unwrap();
         let raw = "b".repeat(64);
         sqlx::query("INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at) VALUES ('s35','day35',?1,unixepoch()+60)")
             .bind(shared::auth::token_digest(&raw))
diff --git a/chat_apps/chat_server/src/signup.rs b/chat_apps/chat_server/src/signup.rs
index 8fcf6fc..a0d4eee 100644
--- a/chat_apps/chat_server/src/signup.rs
+++ b/chat_apps/chat_server/src/signup.rs
@@ -130,6 +130,7 @@ async fn signup(
         .begin()
         .await
         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
+    let member_id = uuid::Uuid::new_v4().to_string();
     sqlx::query(
         r#"INSERT INTO mbr_user (id
             ,user_acnt
@@ -142,7 +143,7 @@ async fn signup(
             ,updated_at
         ) VALUES (?1,?2,?3,?4,?5,'active',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)"#,
     )
-    .bind(uuid::Uuid::new_v4().to_string())
+    .bind(&member_id)
     .bind(username)
     .bind(password_hash)
     .bind(email)
@@ -154,6 +155,8 @@ async fn signup(
         _ => StatusCode::INTERNAL_SERVER_ERROR,
     })?;
 
+    sqlx::query("INSERT INTO mbr_user_role(user_id,role_id,created_at) VALUES(?1,'member-default',CURRENT_TIMESTAMP)")
+        .bind(&member_id).execute(&mut *tx).await.map_err(|_|StatusCode::SERVICE_UNAVAILABLE)?;
     // 成功註冊不沿用目前瀏覽器的帳號或選房；資料與撤銷同交易提交。
     if let Some(token) = headers
         .get("cookie")
diff --git a/chat_apps/chat_server/src/ws.rs b/chat_apps/chat_server/src/ws.rs
index 22ebb9e..5d1aa1c 100644
--- a/chat_apps/chat_server/src/ws.rs
+++ b/chat_apps/chat_server/src/ws.rs
@@ -8,7 +8,7 @@ use axum::{
         ws::{Message, WebSocket, WebSocketUpgrade},
     },
     http::HeaderMap,
-    response::Response,
+    response::{IntoResponse, Response},
 };
 use shared::{ClientEvent, ServerEvent};
 use sqlx::SqlitePool;
@@ -36,6 +36,9 @@ pub async fn ws_handler(
     headers: HeaderMap,
     ws: WebSocketUpgrade,
 ) -> Response {
+    if let Err(status) = authorize_socket(&pool, &headers, &room_id).await {
+        return status.into_response();
+    }
     match state.room_repository.find_by_id(&room_id).await {
         Ok(Some(_)) => {
             ws.on_upgrade(move |socket| handle_socket(socket, state, room_id, pool, headers))
@@ -75,10 +78,30 @@ fn typing_event_for_room(
     })
 }
 
+#[cfg(test)]
 async fn socket_session_active(pool: &SqlitePool, headers: &HeaderMap) -> bool {
     active_member(pool, headers).await.is_ok()
 }
 
+async fn authorize_socket(
+    pool: &SqlitePool,
+    headers: &HeaderMap,
+    room: &str,
+) -> Result<(), axum::http::StatusCode> {
+    use axum::http::StatusCode;
+    let member = active_member(pool, headers).await?;
+    if crate::member_self::initial(pool, &member).await? {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    if chat_persistence::room_access::is_excluded(pool, &member, room)
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
+    {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    Ok(())
+}
+
 async fn handle_socket(
     mut socket: WebSocket,
     state: AppState,
@@ -93,7 +116,7 @@ async fn handle_socket(
     loop {
         tokio::select! {
             _ = recheck.tick() => {
-                if !socket_session_active(&pool, &headers).await {
+                if authorize_socket(&pool, &headers, &room_id).await.is_err() {
                     let _ = socket.send(Message::Close(None)).await;
                     break;
                 }
@@ -108,7 +131,7 @@ async fn handle_socket(
                     }
                 };
                 // 不因握手時曾通過就永久信任 Cookie。
-                if !socket_session_active(&pool, &headers).await {
+                if authorize_socket(&pool, &headers, &room_id).await.is_err() {
                     let _ = socket.send(Message::Close(None)).await;
                     break;
                 }
@@ -123,7 +146,7 @@ async fn handle_socket(
             event = room_events.recv() => {
                 match event {
                     Ok(event) => {
-                        if !socket_session_active(&pool, &headers).await {
+                        if authorize_socket(&pool, &headers, &room_id).await.is_err() {
                             let _ = socket.send(Message::Close(None)).await;
                             break;
                         }
diff --git a/chat_apps/chat_server/tests/day45_member_self.rs b/chat_apps/chat_server/tests/day45_member_self.rs
new file mode 100644
index 0000000..d7cb2d2
--- /dev/null
+++ b/chat_apps/chat_server/tests/day45_member_self.rs
@@ -0,0 +1,383 @@
+use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
+use axum::{
+    Router,
+    body::{Body, to_bytes},
+    http::{Request, StatusCode},
+};
+use shared::auth::token_digest;
+use tower::ServiceExt;
+async fn send(
+    app: Router,
+    path: &str,
+    body: Option<String>,
+    cookie: &str,
+) -> axum::response::Response {
+    let mut req = Request::builder()
+        .method(if body.is_some() { "POST" } else { "GET" })
+        .uri(path)
+        .header("cookie", cookie)
+        .header("origin", "http://127.0.0.1:3000");
+    if body.is_some() {
+        req = req.header("content-type", "application/x-www-form-urlencoded");
+    }
+    app.oneshot(req.body(Body::from(body.unwrap_or_default())).unwrap())
+        .await
+        .unwrap()
+}
+async fn fixture() -> (sqlx::SqlitePool, Router, String) {
+    let pool = sqlx::sqlite::SqlitePoolOptions::new()
+        .max_connections(1)
+        .connect("sqlite::memory:")
+        .await
+        .unwrap();
+    chat_persistence::migrate(&pool).await.unwrap();
+    let hash = Argon2::default()
+        .hash_password(b"member-password")
+        .unwrap()
+        .to_string();
+    for id in ["self-member", "other-member"] {
+        sqlx::query("INSERT INTO mbr_user(id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at) VALUES(?1,?1,?2,?1||'@example.test',?1,'active',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)").bind(id).bind(&hash).execute(&pool).await.unwrap();
+    }
+    for (id, name) in [
+        ("member-room-create", "建立聊天室"),
+        ("member-self-profile-update", "修改本人資料"),
+        ("member-self-password-update", "修改本人密碼"),
+    ] {
+        sqlx::query("INSERT INTO mbr_perm(id,perm_name,perm_status,created_at,updated_at) VALUES(?1,?2,'active',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING").bind(id).bind(name).execute(&pool).await.unwrap();
+        sqlx::query("INSERT INTO mbr_user_perm(user_id,perm_id,perm_filter,created_at) VALUES('self-member',?1,'allow',CURRENT_TIMESTAMP)").bind(id).execute(&pool).await.unwrap();
+    }
+    let raw = "a".repeat(64);
+    sqlx::query("INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at) VALUES('fixture-session','self-member',?1,unixepoch()+600)").bind(token_digest(&raw)).execute(&pool).await.unwrap();
+    let app = chat_server::signin::routes(pool.clone());
+    (pool, app, format!("chat_session={raw}"))
+}
+fn csrf(cookie: &str, operation: &str) -> String {
+    token_digest(&format!(
+        "{operation}:{}",
+        cookie.strip_prefix("chat_session=").unwrap()
+    ))
+}
+#[tokio::test]
+async fn self_profile_is_separate_and_cannot_target_another_member() {
+    let (pool, app, cookie) = fixture().await;
+    assert_eq!(
+        send(app.clone(), "/account/profile", None, &cookie)
+            .await
+            .status(),
+        StatusCode::OK
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/account/profile?user_id=other-member",
+            None,
+            &cookie
+        )
+        .await
+        .status(),
+        StatusCode::UNPROCESSABLE_ENTITY
+    );
+    let payload = format!(
+        "display_name=Changed&email=changed%40example.test&csrf={}",
+        csrf(&cookie, "member-profile")
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/account/profile",
+            Some(format!("{payload}&user_id=other-member")),
+            &cookie
+        )
+        .await
+        .status(),
+        StatusCode::UNPROCESSABLE_ENTITY
+    );
+    let unchanged: String =
+        sqlx::query_scalar("SELECT user_name FROM mbr_user WHERE id='other-member'")
+            .fetch_one(&pool)
+            .await
+            .unwrap();
+    assert_eq!(unchanged, "other-member");
+    assert_eq!(
+        send(
+            app.clone(),
+            "/account/profile",
+            Some(payload.clone()),
+            &cookie
+        )
+        .await
+        .status(),
+        StatusCode::SEE_OTHER
+    );
+    let name: String = sqlx::query_scalar("SELECT user_name FROM mbr_user WHERE id='self-member'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(name, "Changed");
+    sqlx::query("UPDATE mbr_user_perm SET perm_filter='deny' WHERE user_id='self-member' AND perm_id='member-self-profile-update'").execute(&pool).await.unwrap();
+    assert_eq!(
+        send(app.clone(), "/account/profile", Some(payload), &cookie)
+            .await
+            .status(),
+        StatusCode::FORBIDDEN
+    );
+    assert_eq!(
+        send(app, "/account/password", None, &cookie).await.status(),
+        StatusCode::OK,
+        "profile deny must not remove password action"
+    );
+}
+#[tokio::test]
+async fn room_creation_requires_action_and_deny_leaves_no_room() {
+    let (pool, app, cookie) = fixture().await;
+    sqlx::query("UPDATE mbr_user_perm SET perm_filter='deny' WHERE user_id='self-member' AND perm_id='member-room-create'").execute(&pool).await.unwrap();
+    let form = format!("name=Forbidden&csrf={}", csrf(&cookie, "create-room"));
+    assert_eq!(
+        send(app.clone(), "/account/rooms", Some(form), &cookie)
+            .await
+            .status(),
+        StatusCode::FORBIDDEN
+    );
+    let response = send(app.clone(), "/api/member-capabilities", None, &cookie).await;
+    assert_eq!(response.status(), StatusCode::OK);
+    let bytes = to_bytes(response.into_body(), 65536).await.unwrap();
+    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
+    assert_eq!(value["create_room"], false);
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM rooms")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count, 0);
+    let body = to_bytes(
+        send(app, "/account", None, &cookie).await.into_body(),
+        65536,
+    )
+    .await
+    .unwrap();
+    assert!(!String::from_utf8_lossy(&body).contains("action=\"/account/rooms\""));
+}
+#[tokio::test]
+async fn password_change_is_self_only_csrf_checked_salted_and_revokes_sessions() {
+    let (pool, app, cookie) = fixture().await;
+    let form = format!(
+        "current_password=member-password&new_password=changed-password&confirm_password=changed-password&csrf={}",
+        csrf(&cookie, "member-password")
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/account/password",
+            Some(format!("{form}&user_id=other-member")),
+            &cookie
+        )
+        .await
+        .status(),
+        StatusCode::UNPROCESSABLE_ENTITY
+    );
+    assert_eq!(
+        send(
+            app.clone(),
+            "/account/password",
+            Some(form.replace(&csrf(&cookie, "member-password"), "wrong")),
+            &cookie
+        )
+        .await
+        .status(),
+        StatusCode::FORBIDDEN
+    );
+    assert_eq!(
+        send(app.clone(), "/account/password", Some(form), &cookie)
+            .await
+            .status(),
+        StatusCode::SEE_OTHER
+    );
+    let hash: String = sqlx::query_scalar("SELECT user_pd FROM mbr_user WHERE id='self-member'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert!(hash.starts_with("$argon2id$"));
+    Argon2::default()
+        .verify_password(b"changed-password", &PasswordHash::new(&hash).unwrap())
+        .unwrap();
+    let other: String = sqlx::query_scalar("SELECT user_pd FROM mbr_user WHERE id='other-member'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    Argon2::default()
+        .verify_password(b"member-password", &PasswordHash::new(&other).unwrap())
+        .unwrap();
+    assert_eq!(
+        send(app, "/account/password", None, &cookie).await.status(),
+        StatusCode::UNAUTHORIZED
+    );
+}
+#[tokio::test]
+async fn member_roles_union_overrides_and_disabled_definitions() {
+    let (pool, _, cookie) = fixture().await;
+    sqlx::query("DELETE FROM mbr_user_perm WHERE user_id='self-member'")
+        .execute(&pool)
+        .await
+        .unwrap();
+    for (role, perm) in [
+        ("role-a", "member-room-create"),
+        ("role-b", "member-self-profile-update"),
+    ] {
+        sqlx::query("INSERT INTO mbr_role(id,role_name,role_status,created_at,updated_at) VALUES(?1,?1,'active',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)").bind(role).execute(&pool).await.unwrap();
+        sqlx::query("INSERT INTO mbr_user_role(user_id,role_id,created_at) VALUES('self-member',?1,CURRENT_TIMESTAMP)").bind(role).execute(&pool).await.unwrap();
+        sqlx::query(
+            "INSERT INTO mbr_role_perm(role_id,perm_id,created_at) VALUES(?1,?2,CURRENT_TIMESTAMP)",
+        )
+        .bind(role)
+        .bind(perm)
+        .execute(&pool)
+        .await
+        .unwrap();
+    }
+    use chat_server::member_self::{PASSWORD, PROFILE, ROOM, allowed};
+    assert!(allowed(&pool, "self-member", ROOM).await.unwrap());
+    assert!(allowed(&pool, "self-member", PROFILE).await.unwrap());
+    assert!(!allowed(&pool, "self-member", PASSWORD).await.unwrap());
+    sqlx::query("INSERT INTO mbr_user_perm(user_id,perm_id,perm_filter,created_at) VALUES('self-member','member-room-create','deny',CURRENT_TIMESTAMP)").execute(&pool).await.unwrap();
+    assert!(!allowed(&pool, "self-member", ROOM).await.unwrap());
+    sqlx::query("DELETE FROM mbr_user_perm WHERE user_id='self-member'")
+        .execute(&pool)
+        .await
+        .unwrap();
+    assert!(allowed(&pool, "self-member", ROOM).await.unwrap());
+    sqlx::query("UPDATE mbr_role SET role_status='inactive' WHERE id='role-a'")
+        .execute(&pool)
+        .await
+        .unwrap();
+    assert!(!allowed(&pool, "self-member", ROOM).await.unwrap());
+    sqlx::query("UPDATE mbr_perm SET perm_status='inactive' WHERE id='member-self-profile-update'")
+        .execute(&pool)
+        .await
+        .unwrap();
+    assert!(!allowed(&pool, "self-member", PROFILE).await.unwrap());
+    assert!(cookie.starts_with("chat_session="));
+}
+#[tokio::test]
+async fn mandatory_change_cannot_be_bypassed_and_next_login_opens_account() {
+    let (pool, app, cookie) = fixture().await;
+    sqlx::query("UPDATE mbr_user SET is_init_pd=TRUE WHERE id='self-member'")
+        .execute(&pool)
+        .await
+        .unwrap();
+    let response = send(
+        app.clone(),
+        "/login",
+        Some("username=self-member&password=member-password".into()),
+        &cookie,
+    )
+    .await;
+    assert_eq!(response.headers()["location"], "/account/password");
+    assert_eq!(
+        send(app.clone(), "/account", None, &cookie).await.status(),
+        StatusCode::FORBIDDEN
+    );
+    sqlx::query("UPDATE mbr_user_perm SET perm_filter='deny' WHERE user_id='self-member' AND perm_id='member-self-password-update'").execute(&pool).await.unwrap();
+    assert_eq!(
+        send(app.clone(), "/account/password", None, &cookie)
+            .await
+            .status(),
+        StatusCode::FORBIDDEN
+    );
+    sqlx::query("UPDATE mbr_user_perm SET perm_filter='allow' WHERE user_id='self-member' AND perm_id='member-self-password-update'").execute(&pool).await.unwrap();
+    let form = format!(
+        "current_password=member-password&new_password=changed-password&confirm_password=changed-password&csrf={}",
+        csrf(&cookie, "member-password")
+    );
+    assert_eq!(
+        send(app.clone(), "/account/password", Some(form), &cookie)
+            .await
+            .status(),
+        StatusCode::SEE_OTHER
+    );
+    let response = send(
+        app,
+        "/login",
+        Some("username=self-member&password=changed-password".into()),
+        &cookie,
+    )
+    .await;
+    assert_eq!(response.headers()["location"], "/account");
+}
+#[tokio::test]
+async fn json_room_route_uses_same_action_and_csrf() {
+    let (pool, app, cookie) = fixture().await;
+    let app = app.merge(
+        Router::new()
+            .route(
+                "/api/rooms",
+                axum::routing::post(chat_server::member_self::create_room_api),
+            )
+            .with_state(pool.clone()),
+    );
+    for (token, expected) in [
+        ("wrong".into(), StatusCode::FORBIDDEN),
+        (csrf(&cookie, "create-room"), StatusCode::CREATED),
+    ] {
+        let response = app
+            .clone()
+            .oneshot(
+                Request::post("/api/rooms")
+                    .header("cookie", &cookie)
+                    .header("origin", "http://127.0.0.1:3000")
+                    .header("content-type", "application/json")
+                    .header("x-csrf-token", token)
+                    .body(Body::from(r#"{"name":"Allowed"}"#))
+                    .unwrap(),
+            )
+            .await
+            .unwrap();
+        assert_eq!(response.status(), expected);
+    }
+    sqlx::query("UPDATE mbr_user_perm SET perm_filter='deny' WHERE user_id='self-member' AND perm_id='member-room-create'").execute(&pool).await.unwrap();
+    let response = app
+        .oneshot(
+            Request::post("/api/rooms")
+                .header("cookie", &cookie)
+                .header("origin", "http://127.0.0.1:3000")
+                .header("content-type", "application/json")
+                .header("x-csrf-token", csrf(&cookie, "create-room"))
+                .body(Body::from(r#"{"name":"Denied"}"#))
+                .unwrap(),
+        )
+        .await
+        .unwrap();
+    assert_eq!(response.status(), StatusCode::FORBIDDEN);
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM rooms")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count, 1);
+}
+#[tokio::test]
+async fn failed_session_revocation_rolls_password_back() {
+    let (pool, app, cookie) = fixture().await;
+    let before: String = sqlx::query_scalar("SELECT user_pd FROM mbr_user WHERE id='self-member'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    sqlx::raw_sql("CREATE TRIGGER fail_session_delete BEFORE DELETE ON mbr_session BEGIN SELECT RAISE(ABORT,'fixture revoke failure'); END;").execute(&pool).await.unwrap();
+    let form = format!(
+        "current_password=member-password&new_password=changed-password&confirm_password=changed-password&csrf={}",
+        csrf(&cookie, "member-password")
+    );
+    assert_eq!(
+        send(app, "/account/password", Some(form), &cookie)
+            .await
+            .status(),
+        StatusCode::SERVICE_UNAVAILABLE
+    );
+    let after: String = sqlx::query_scalar("SELECT user_pd FROM mbr_user WHERE id='self-member'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(before, after);
+    let sessions: i64 = sqlx::query_scalar("SELECT count(*) FROM mbr_session")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(sessions, 1);
+}
diff --git a/chat_apps/chat_ui/src/room_api.rs b/chat_apps/chat_ui/src/room_api.rs
index 4c93d90..b8be21a 100644
--- a/chat_apps/chat_ui/src/room_api.rs
+++ b/chat_apps/chat_ui/src/room_api.rs
@@ -24,6 +24,33 @@ pub async fn create_room(name: &str) -> Result<CreateRoomResponse, String> {
 
     let window = web_sys::window().ok_or_else(|| "browser window is unavailable".to_string())?;
 
+    // Same live member Session and effective action used by the form and backend.
+    let capability: web_sys::Response =
+        JsFuture::from(window.fetch_with_str("/api/member-capabilities"))
+            .await
+            .map_err(|_| "無法取得會員權限")?
+            .dyn_into()
+            .map_err(|_| "權限回應錯誤")?;
+    if !capability.ok() {
+        return Err("無法取得會員權限".into());
+    }
+    let text = JsFuture::from(capability.text().map_err(|_| "權限回應錯誤")?)
+        .await
+        .map_err(|_| "權限回應錯誤")?
+        .as_string()
+        .ok_or("權限回應錯誤")?;
+    let value: serde_json::Value = serde_json::from_str(&text).map_err(|_| "權限回應錯誤")?;
+    if value.get("create_room").and_then(|v| v.as_bool()) != Some(true) {
+        return Err("沒有建立聊天室 action".into());
+    }
+    let csrf = value
+        .get("create_csrf")
+        .and_then(|v| v.as_str())
+        .ok_or("缺少 CSRF")?;
+    request
+        .headers()
+        .set("X-CSRF-Token", csrf)
+        .map_err(|_| "無法設定 CSRF")?;
     let response_value = JsFuture::from(window.fetch_with_request(&request))
         .await
         .map_err(|error| format!("create room request failed: {error:?}"))?;
diff --git a/chat_apps/migrations/20261031123000_member_actions.sql b/chat_apps/migrations/20261031123000_member_actions.sql
new file mode 100644
index 0000000..b6ff9ba
--- /dev/null
+++ b/chat_apps/migrations/20261031123000_member_actions.sql
@@ -0,0 +1,13 @@
+-- Members have actions only: definitions never grant access by themselves.
+INSERT INTO mbr_perm(id,perm_name,perm_status,created_at,updated_at) VALUES
+('member-room-create','建立聊天室','active',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP),
+('member-self-profile-update','修改本人資料','active',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP),
+('member-self-password-update','修改本人密碼','active',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);
+-- Explicit default role preserves existing create-room capability and provides required self-service.
+INSERT INTO mbr_role(id,role_name,role_status,created_at,updated_at)
+VALUES('member-default','一般會員','active',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);
+INSERT INTO mbr_role_perm(role_id,perm_id,created_at)
+SELECT 'member-default',id,CURRENT_TIMESTAMP FROM mbr_perm
+WHERE id IN('member-room-create','member-self-profile-update','member-self-password-update');
+INSERT INTO mbr_user_role(user_id,role_id,created_at)
+SELECT id,'member-default',CURRENT_TIMESTAMP FROM mbr_user WHERE user_status='active';
```

## Refactor

獨立Refactor承接Day45的Chat Session SQLite Adapter：Cookie／CSRF仍在HTTP層，digest查詢移到chat_persistence/member_session.rs；原Cookie、帳號停用、Session撤銷斷言保持。它整理累積Session邊界，不是資料／密碼Green後的小整理，不修改其他專案、shared或全庫CI。原Refactor曾發布，本修訂不重複計完成數；新完整組合patch已含此搬移。以下保留完整Refactor供逐檔檢視；不要先套它再套本次完整組合patch。

```diff
diff --git a/chat_apps/chat_persistence/src/lib.rs b/chat_apps/chat_persistence/src/lib.rs
index cc44d6d..e3a3c4e 100644
--- a/chat_apps/chat_persistence/src/lib.rs
+++ b/chat_apps/chat_persistence/src/lib.rs
@@ -1,3 +1,4 @@
+pub mod member_session;
 use async_trait::async_trait;
 use chat_application::repository::{MessageRepository, RepositoryError, RoomRepository};
 use chat_domain::Room;
diff --git a/chat_apps/chat_persistence/src/member_session.rs b/chat_apps/chat_persistence/src/member_session.rs
new file mode 100644
index 0000000..6725386
--- /dev/null
+++ b/chat_apps/chat_persistence/src/member_session.rs
@@ -0,0 +1,15 @@
+use sqlx::SqlitePool;
+
+/// SQLite adapter accepts a digest, never a raw bearer token or HTTP headers.
+pub async fn active_member_id(
+    pool: &SqlitePool,
+    digest: &str,
+) -> Result<Option<String>, sqlx::Error> {
+    sqlx::query_scalar(
+        "SELECT u.id FROM mbr_session s JOIN mbr_user u ON u.id=s.mbr_user_id
+         WHERE s.token_hash=?1 AND s.expires_at>unixepoch() AND u.user_status='active'",
+    )
+    .bind(digest)
+    .fetch_optional(pool)
+    .await
+}
diff --git a/chat_apps/chat_server/src/auth_session.rs b/chat_apps/chat_server/src/auth_session.rs
index b2ff414..80e0629 100644
--- a/chat_apps/chat_server/src/auth_session.rs
+++ b/chat_apps/chat_server/src/auth_session.rs
@@ -14,19 +14,9 @@ fn session_cookie(headers: &HeaderMap) -> Result<String, StatusCode> {
 /// No plaintext bearer token is ever stored in SQLite.
 pub async fn active_member(pool: &SqlitePool, headers: &HeaderMap) -> Result<String, StatusCode> {
     let raw = session_cookie(headers)?;
-    let member: Option<String> = sqlx::query_scalar(
-        r#"SELECT u.id 
-        FROM mbr_session s
-        JOIN mbr_user u 
-            ON u.id=s.mbr_user_id
-        WHERE s.token_hash=?1 
-          AND s.expires_at>unixepoch()
-          AND u.user_status='active'"#,
-    )
-    .bind(token_digest(&raw))
-    .fetch_optional(pool)
-    .await
-    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    let member = chat_persistence::member_session::active_member_id(pool, &token_digest(&raw))
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
     member.ok_or(StatusCode::UNAUTHORIZED)
 }
 
```

## 版本與實際驗證

[本回合官方查核](final-rewire-versions.json)：[Rust官方stable channel](https://static.rust-lang.org/dist/channel-rust-stable.toml)為1.99.1；鎖定Argon2 0.6.0、Axum 0.8.9、Leptos 0.8.22、SQLx 0.9.0、Tokio 1.54.1，與[官方crate registry](https://crates.io/)stable相符；Leptos 0.9 RC不採用。SQLx 0.9宣告Rust最低1.94，不能以舊1.85/MSRV承諾整條依賴；本機rustc/Cargo 1.99.1實測，未做最低版本編譯。PG實測18.6，[官方release](https://www.postgresql.org/docs/release/)19仍RC，沒有切換RC。Cargo.lock及相依版本未更改。

在另一份乾淨f6匯出套完整patch，逐個變更檔與候選SHA256相同、git apply --check與diff --check均0；[套用證據](final-rewire-clean-apply.json)。真實完整回歸：**44 passed、0 ignored，exit0**，[命令／限制](chat-day45-validation-r2.json)、[完整log](chat-day45-final-r2.log)。另跑Chat UI WASM hydrate cargo check，exit0；不是瀏覽器互動驗收。

未執行／尚未完成：瀏覽器互動與實際hydration旅程；Chat管理端完整category/page/action管理UI及管理身分路由；本次會員實作不冒充管理端驗收；正式／既有使用者SQLite備份升級；全產品Gate2與全庫CI。Gate1是本文、完整patch、版本及實驗證據的文件交付；**候選自動測試通過不等於產品已套用或完整Gate2通過**。rust_ai_ring未寫入。下一步由使用者套用後，對實際新產品SHA驗證互動、舊DB備份與剩餘產品功能，不沿用本次候選結果冒充新SHA實測。
