# Chat Day45｜排除名單生效於進房及即時連線

## ① 目標、現況與來源

來源產品 `64eff527283fd05657841297879aefeaadd31f64`，文件 parent `738aeab0e689ebcc9699d0555a598480c8cede46`；規則以該 parent 的 COMMON_LEARNING_RULES.md 為準。已讀需求、架構、入口與 Day44；next_day=45，未發現既有 Day45。Day44 已實作房間存在判斷，這次接續最早安全缺口：mbr_exclude 有資料仍可進房。

操作旅程：匿名看公開清單→點房／登入回跳→伺服器檢查 Session、房間存在及排除名單→允許者聊天，被排除者收到拒絕原因。既有 WS 也要在握手、收／送事件及每秒重驗時檢查，避免繼續收資料。這是現行公開房間的明確排除切片；私人房、角色與加入／管理 UI 尚待接續，不宣稱完整 ACL。

正式鏈：lib.rs 保留 GET /api/rooms 公開與 require_member；/room/{room_id}→require_room_member→SqliteRoomRepository／room_access；/ws/{room_id}→ws_handler→authorize_socket→active_member／room_access，handle_socket 每分支重驗。Leptos SSR／hydrate、既有登入回跳及資料庫 schema 不改。

## Refactor｜Session SQL 移到 Chat persistence Adapter

先整理累積的 Session 邊界：原 auth_session 同時解析 HTTP Cookie 與查 SQLite。移出 active_member_id，只收 digest／pool、回 Option 或 SQLx error；HTTP Adapter 仍負責 401／503。沒有把 axum 或 raw token 帶進 persistence，也沒有修改 shared 或跨 workspace CI。沿用既有 chat-persistence 相依，無新增 Cargo／feature。

以下逐檔完整 patch：新增 member_session.rs、lib.rs module、替換 auth_session 的完整查詢單元；原 auth_session::tests 的 Cookie／CSRF、停用、撤銷斷言原封不動，由正式 active_member caller 穿過新 Adapter；room_auth 的匿名／過期／公開列表與 Day44 存在性斷言全保留。

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

## ② Red → 完整 Green

核心 Red／Green 預估15分鐘，未實際計時；Refactor、來源／版本查核、交付與驗收另計。先套 Refactor，再套 Red。Red 使用真 SQLite migration、有效 Session 與正式 require_room_member：依序允許→新增 mbr_exclude 拒絕→刪除排除再允許，公開清單持續可讀。

```diff
diff --git a/chat_apps/chat_persistence/src/room_access.rs b/chat_apps/chat_persistence/src/room_access.rs
deleted file mode 100644
index e69de29..0000000
diff --git a/chat_apps/chat_server/src/room_auth.rs b/chat_apps/chat_server/src/room_auth.rs
index 88ca6ad..bce3d24 100644
--- a/chat_apps/chat_server/src/room_auth.rs
+++ b/chat_apps/chat_server/src/room_auth.rs
@@ -298,3 +298,76 @@ mod day44_existing_room_gate_tests {
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
```

執行 `cargo test --manifest-path chat_apps/Cargo.toml -p chat-server --lib day45_tests --locked`，預期舊實作在 excluded=true 時實際200、期待403，這才是行為 Red；環境／編譯失敗不能當此證據。

Green 從「已套 Refactor、尚未套 Red」基準套用，已包含同一 Red 測試，因此不要再重複套 Red。已做 Red 的副本請先 reverse Red，再套 Green。以下包含所有新檔與 caller 變更：

```diff
diff --git a/chat_apps/chat_persistence/src/lib.rs b/chat_apps/chat_persistence/src/lib.rs
index e3a3c4e..e4c3157 100644
--- a/chat_apps/chat_persistence/src/lib.rs
+++ b/chat_apps/chat_persistence/src/lib.rs
@@ -1,4 +1,5 @@
 pub mod member_session;
+pub mod room_access;
 use async_trait::async_trait;
 use chat_application::repository::{MessageRepository, RepositoryError, RoomRepository};
 use chat_domain::Room;
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
diff --git a/chat_apps/chat_server/src/room_auth.rs b/chat_apps/chat_server/src/room_auth.rs
index 88ca6ad..a06a99b 100644
--- a/chat_apps/chat_server/src/room_auth.rs
+++ b/chat_apps/chat_server/src/room_auth.rs
@@ -24,7 +24,7 @@ pub async fn require_room_member(
     next: Next,
 ) -> Result<Response, StatusCode> {
     match active_member(&pool, request.headers()).await {
-        Ok(_) => {
+        Ok(member) => {
             // 登入成功也不能讓不存在的房間進入 SSR／WebSocket 畫面。
             // 沿用正式 SQLite RoomRepository，而不是在 middleware 複製 SQL。
             use chat_application::repository::RoomRepository;
@@ -37,6 +37,14 @@ pub async fn require_room_member(
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
@@ -298,3 +306,76 @@ mod day44_existing_room_gate_tests {
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
diff --git a/chat_apps/chat_server/src/ws.rs b/chat_apps/chat_server/src/ws.rs
index 22ebb9e..9f8239a 100644
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
@@ -75,10 +78,27 @@ fn typing_event_for_room(
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
@@ -93,7 +113,7 @@ async fn handle_socket(
     loop {
         tokio::select! {
             _ = recheck.tick() => {
-                if !socket_session_active(&pool, &headers).await {
+                if authorize_socket(&pool, &headers, &room_id).await.is_err() {
                     let _ = socket.send(Message::Close(None)).await;
                     break;
                 }
@@ -108,7 +128,7 @@ async fn handle_socket(
                     }
                 };
                 // 不因握手時曾通過就永久信任 Cookie。
-                if !socket_session_active(&pool, &headers).await {
+                if authorize_socket(&pool, &headers, &room_id).await.is_err() {
                     let _ = socket.send(Message::Close(None)).await;
                     break;
                 }
@@ -123,7 +143,7 @@ async fn handle_socket(
             event = room_events.recv() => {
                 match event {
                     Ok(event) => {
-                        if !socket_session_active(&pool, &headers).await {
+                        if authorize_socket(&pool, &headers, &room_id).await.is_err() {
                             let _ = socket.send(Message::Close(None)).await;
                             break;
                         }
```

is_excluded 使用參數綁定與 EXISTS，不改公開清單、不猜測 room_status 中文值。DB 錯誤回503或關閉 WS，不能當成未排除。SSR 回403加中文原因；WS 升級前拒絕，已連線者在下一事件或一秒檢查關閉。交易競態仍有查核與後續動作間的時間窗，本切片不聲稱授權與訊息落盤已原子化；後續應把寫入授權納入用例交易。

### FIFO／上一篇承接

Day25 shared-auth 是本案最早 pending：現有 shared Cookie/hash 被呼叫，但跨案共同登入／登出／Session 完整接線及驗證仍未完成。本篇只移 Chat SQL，不替跨案 G-02 抵算，FIFO 維持 pending。Day44 房間存在與回跳測試持續保留，沒有重發舊 Day。

## ③ 2026-10-31 官方穩定版與相容性

Rust/Cargo 本機 1.99.1；[官方 Rust 1.99.1（2026-10-15）](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)。下表當日直接查 crates.io 官方 registry，排除預發行及 yanked；完整 JSON、MSRV 與查核來源隨文保存。edition 2024；沒有升級 lock。

| 套件 | manifest | lock | 官方 stable | MSRV |
| --- | --- | --- | --- | --- |
| [argon2](https://crates.io/api/v1/crates/argon2) | 0.6 | 0.6.0 | 0.6.0 | 1.85 |
| [async-trait](https://crates.io/api/v1/crates/async-trait) | 0.1 | 0.1.92 | 0.1.92 | 1.71 |
| [axum](https://crates.io/api/v1/crates/axum) | 0.8 | 0.8.9 | 0.8.9 | 1.80 |
| [leptos](https://crates.io/api/v1/crates/leptos) | 0.8 | 0.8.22 | 0.8.22 | 1.88 |
| [leptos_axum](https://crates.io/api/v1/crates/leptos_axum) | 0.8 | 0.8.10 | 0.8.10 | 1.88 |
| [leptos_config](https://crates.io/api/v1/crates/leptos_config) | 0.8 | 0.8.10 | 0.8.10 | 1.88 |
| [leptos_router](https://crates.io/api/v1/crates/leptos_router) | 0.8 | 0.8.17 | 0.8.17 | 1.88 |
| [libsqlite3-sys](https://crates.io/api/v1/crates/libsqlite3-sys) | package manifest / transitive | 0.37.0 | 0.39.1 | 1.88.0 |
| [serde](https://crates.io/api/v1/crates/serde) | 1.0 | 1.0.229 | 1.0.229 | 1.56 |
| [serde_json](https://crates.io/api/v1/crates/serde_json) | 1.0 | 1.0.154 | 1.0.154 | 1.71 |
| [sha2](https://crates.io/api/v1/crates/sha2) | 0.11 | 0.11.1 | 0.11.1 | 1.88 |
| [sqlx](https://crates.io/api/v1/crates/sqlx) | 0.9 | 0.9.0 | 0.9.0 | 1.94.0 |
| [thiserror](https://crates.io/api/v1/crates/thiserror) | 2.0 | 2.0.21 | 2.0.21 | 1.77 |
| [tokio](https://crates.io/api/v1/crates/tokio) | 1.54 | 1.54.1 | 1.54.1 | 1.85 |
| [tower](https://crates.io/api/v1/crates/tower) | package manifest / transitive | 0.5.3 | 0.5.3 | 1.64.0 |
| [uuid](https://crates.io/api/v1/crates/uuid) | 1.29 | 1.29.0 | 1.29.0 | 1.89.0 |

SQLx 0.9 要求 Rust 1.94；Leptos 0.8 要求 1.88，目前編譯器符合。libsqlite3-sys lock 0.37.0 低於最新 0.39.1：SQLx SQLite driver 綁定其原生 links，不能單獨升級造成重複 sqlite3 links；本篇保留 SQLx 支援的 lock，升級須一起重核 driver、SQLite 編譯及舊庫。MSRV 是必要條件，不能據此推論 DB、WASM 或 GUI 已驗收。

## ④ 驗證、套用與下一步

附件順序：精確產品 SHA→chat-day45-refactor.patch→chat-day45-green.patch。先 `git apply --check` 再 apply，舊測試不能刪；回復用反序 `git apply -R`，不改 DB。候選副本的測試與原始 exit 記錄見 chat-validation.json／日誌。來源精確 SHA 的七個 workflow 本次查得全 success，僅代表原產品，不涵蓋本篇候選。

Gate1：本文、完整 Red／Green／Refactor、版本、附件與讀回證據交付後判定。Gate2：程式尚未由使用者套用，完整 ACL／正式瀏覽器／真 WS 撤銷仍待驗，不因原 CI 綠燈通過。未碰正式 DB、秘密或付費 AI；產品庫沒有提交。

下一步：使用者套用後，以同一精確 SHA 啟動 SSR＋hydrate，登入 A／B、排除 A、確認 A 403與既有 WS 關閉／B不受影響；再接私人房明確授權及管理流程。完整產品 Gate2=false，last_full_comparison 不更新。
