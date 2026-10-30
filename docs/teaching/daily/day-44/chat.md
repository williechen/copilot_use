# Chat Day44｜登入後只允許進入真正存在的聊天室

> 2026-10-30 本地驗證修訂 r3。最新產品唯讀 `4b1008b95bc3e170381fcd1db84695e421f036bd`；下列原 Refactor／Green 程式保留 `7ce39f0166a2120539e5908b33c88322a930c35a` 重現基準。最新 main 已有對應功能，不重複套用。next_day 已是45，不再加 Day。原發布版本與讀回保留於封存。

## 本次本地結果

最新主分支已實作存在房間門檻；一般／SSR各62項通過、WASM通過。最新正式caller已改名為`room_path`，與原教學命名一致。 [四workspace、真套用、原始命令及阻塞](../../../source-review/local-validation-2026-10-30-day44-r2/README.md)。原發布全文以ZIP保存，原交付讀回不能冒認本次修訂；完整Gate2未完成。

## ① 目標、缺口與正式呼叫鏈

Day43 已將房間搜尋清除接上正式 Leptos `RoomList`。本日處理更早的登入進房缺口：`chat_server::application` 的 `GET /room/{room_id}` 經 `room_auth::require_room_member` 僅驗 Session；會員即使輸入不存在的合法 UUID 仍能收到 SSR 200，直到前端取得房名失敗才看見模糊訊息。新行為是：**有效會員＋已儲存 UUID → 200；有效會員＋不存在 UUID → 404；匿名＋合法 UUID → /signin?next=/room/UUID；不規範路徑拒絕；DB 查詢失敗 → 503**。公開 `GET /api/rooms` 與既有 WS 的 401 不改。此日尚未新增房主／逐房 ACL，不能把存在檢查冒稱授權完成。

正式鏈：`chat_apps/chat_server/src/lib.rs::application` → `room_auth::require_room_member` → `auth_session::active_member`（SQLite Session）→ `chat_persistence::SqliteRoomRepository::find_by_id`（現有 `chat_application::repository::RoomRepository` Port）→ `ssr::app_handler` → `chat_ui::RoomPage`。不新增 SQL 或 migration。

## Refactor｜Chat 專案房間路徑正規化與測試責任

這是**與 Red／Green 並列、獨立套用**的 Chat 專案重構。修改前 `room_auth.rs` 的匿名分支自行解析 `/room/` 與 UUID，且未檢查 canonical 小寫形式；房間路徑格式決策混在 Session middleware。修改後新增 `chat_server/src/room_path.rs`，單一 `canonical_room_id` 驗證完整路徑、拒絕多餘段落／query／非標準 UUID，`lib.rs` 明確 `mod room_path;`，原匿名 redirect caller 改用此函式；不碰 Pet、Analytics、共用 crates、全庫 CI。

**舊斷言承接：** 原 `room_auth::day41_room_return_tests` 的合法匿名回跳與非法 ID 拒絕保留；`day25_tests` 的公開列表／會員限制、`auth_session` 的撤銷與 Cookie、`chat_ui` 的 Session 導覽均不刪。新 `room_path::tests` 另驗合法 canonical、非法 UUID、多餘 segment／query。修改範圍只有下列三個檔案（其中一個新增），依序套 Refactor 再套 Green：

```diff
diff --git a/chat_apps/chat_server/src/lib.rs b/chat_apps/chat_server/src/lib.rs
--- a/chat_apps/chat_server/src/lib.rs
+++ b/chat_apps/chat_server/src/lib.rs
@@ -3,6 +3,7 @@
 pub mod hub;
 pub mod logout;
 pub mod room_auth;
+mod room_path;
 pub mod signin;
 pub mod signup;
 pub mod ssr;
diff --git a/chat_apps/chat_server/src/room_auth.rs b/chat_apps/chat_server/src/room_auth.rs
--- a/chat_apps/chat_server/src/room_auth.rs
+++ b/chat_apps/chat_server/src/room_auth.rs
@@ -26,11 +26,7 @@
     match active_member(&pool, request.headers()).await {
         Ok(_) => Ok(next.run(request).await),
         Err(StatusCode::UNAUTHORIZED) => {
-            let id = request
-                .uri()
-                .path()
-                .strip_prefix("/room/")
-                .and_then(|value| uuid::Uuid::parse_str(value).ok())
+            let id = crate::room_path::canonical_room_id(request.uri().path())
                 .ok_or(StatusCode::UNAUTHORIZED)?;
             Ok(Redirect::to(&format!("/signin?next=/room/{id}")).into_response())
         }
diff --git a/chat_apps/chat_server/src/room_path.rs b/chat_apps/chat_server/src/room_path.rs
new file mode 100644
--- /dev/null
+++ b/chat_apps/chat_server/src/room_path.rs
@@ -0,0 +1,22 @@
+/// 僅允許規範化 UUID 房間路徑；不接受 query、片段或多餘 path segment。
+pub(crate) fn canonical_room_id(path: &str) -> Option<String> {
+    let raw = path.strip_prefix("/room/")?;
+    let id = uuid::Uuid::parse_str(raw).ok()?;
+    (id.to_string() == raw).then(|| id.to_string())
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    #[test]
+    fn canonical_room_path_is_fail_closed() {
+        let id = "00000000-0000-0000-0000-000000000001";
+        assert_eq!(canonical_room_id(&format!("/room/{id}")), Some(id.into()));
+        for invalid in ["/room/", "/room/not-a-uuid",
+            "/room/00000000000000000000000000000001",
+            "/room/00000000-0000-0000-0000-000000000001/extra",
+            "/room/00000000-0000-0000-0000-000000000001?x=1"] {
+            assert_eq!(canonical_room_id(invalid), None, "{invalid}");
+        }
+    }
+}
```

Refactor 已在原文基準真套用；合成後加 [fmt 修補](chat-day44-format-r1.patch)，SSR server lib29、UI21通過；最新main的`room_path`＋`room_auth` 已納入62項 SSR 回歸。最新main已採用`room_path`；原附件仍只用於7ce重現。下一步為本案 Session SQL／Repository 錯誤與 fixture 分層，維持401／403／503差異。

## ② Red → 完整 Green（核心預估 15 分鐘；實際未計時）

**Red（同篇功能測試）**：在 `room_auth.rs::day44_existing_room_gate_tests` 建立隔離 SQLite migrations、實際會員、有效 Session、已存在 UUID 房間與不存在 UUID。原版 `require_room_member` 對登入會員的兩個 UUID 都回 200，故「不存在應回 404」是**預期行為 Red**；新增測試在舊來源不存在時亦屬缺測試符號，不把編譯失敗算行為 Red。測試保留匿名合法房間回跳斷言。**Red 未實跑**。

**Green**：成功驗 Session 後，以既有 `SqliteRoomRepository` 查詢指定房間，精確比對回傳 ID；不存在或不一致回 404，Adapter 失敗回 503；匿名仍只回登入回跳，不在未登入時公開房間存在與否。正式 `application` Router 與 Leptos SSR 無需改動，因原本就接到此 middleware。下列完整 patch（以 Refactor 套用後為基準）含正式 caller 與真 DB 測試，不是偽碼：

```diff
diff --git a/chat_apps/chat_server/src/room_auth.rs b/chat_apps/chat_server/src/room_auth.rs
--- a/chat_apps/chat_server/src/room_auth.rs
+++ b/chat_apps/chat_server/src/room_auth.rs
@@ -24,187 +24,244 @@
     next: Next,
 ) -> Result<Response, StatusCode> {
     match active_member(&pool, request.headers()).await {
-        Ok(_) => Ok(next.run(request).await),
-        Err(StatusCode::UNAUTHORIZED) => {
-            let id = crate::room_path::canonical_room_id(request.uri().path())
-                .ok_or(StatusCode::UNAUTHORIZED)?;
-            Ok(Redirect::to(&format!("/signin?next=/room/{id}")).into_response())
-        }
-        Err(error) => Err(error),
-    }
-}
-
-#[cfg(test)]
-mod day41_room_return_tests {
-    use super::*;
-    use axum::{Router, body::Body, http::Request, routing::get};
-    use tower::ServiceExt;
-
-    #[tokio::test]
-    async fn anonymous_room_link_redirects_but_invalid_room_id_is_rejected() {
-        let pool = sqlx::sqlite::SqlitePoolOptions::new()
-            .connect_lazy("sqlite::memory:")
-            .unwrap();
-        let app = Router::new()
-            .route("/room/{room_id}", get(|| async { StatusCode::OK }))
-            .route_layer(axum::middleware::from_fn_with_state(
-                pool,
-                require_room_member,
-            ));
-        let id = uuid::Uuid::new_v4();
-        let response = app
-            .clone()
-            .oneshot(
-                Request::get(format!("/room/{id}"))
-                    .body(Body::empty())
-                    .unwrap(),
-            )
-            .await
-            .unwrap();
-        assert_eq!(response.status(), StatusCode::SEE_OTHER);
-        assert_eq!(
-            response.headers()["location"],
-            format!("/signin?next=/room/{id}")
-        );
-        let bad = app
-            .oneshot(
-                Request::get("/room/not-a-uuid")
-                    .body(Body::empty())
-                    .unwrap(),
-            )
-            .await
-            .unwrap();
-        assert_eq!(bad.status(), StatusCode::UNAUTHORIZED);
-    }
-}
-
-#[cfg(test)]
-mod day25_tests {
-    use super::*;
-    use axum::{
-        Router,
-        body::Body,
-        http::{Request, StatusCode},
-        routing::get,
-    };
-    use sha2::{Digest, Sha256};
-    use tower::ServiceExt;
-
-    #[tokio::test]
-    async fn room_page_requires_active_sqlite_session() {
-        let db = sqlx::sqlite::SqlitePoolOptions::new()
-            .max_connections(1)
-            .connect("sqlite::memory:")
-            .await
-            .unwrap();
-        sqlx::raw_sql(include_str!(
-            "../../migrations/20261018073931_create_auth_user.sql"
-        ))
-        .execute(&db)
-        .await
-        .unwrap();
-        sqlx::raw_sql(include_str!(
-            "../../migrations/20261020152737_mbr_sessions.sql"
-        ))
-        .execute(&db)
-        .await
-        .unwrap();
-        sqlx::query("INSERT INTO mbr_user (id,user_acnt,user_pd,user_email,user_name,user_status,user_sexy,user_birthday,is_init_pd,created_at,updated_at) VALUES ('m1','member','unused','m1@example.test','Member','active','', '2000-01-01',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
-            .execute(&db).await.unwrap();
-        let token = "a".repeat(64);
-        let hash = Sha256::digest(token.as_bytes())
-            .iter()
-            .map(|b| format!("{b:02x}"))
-            .collect::<String>();
-        sqlx::query("INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at) VALUES('s1','m1',?1,unixepoch()+60)")
-            .bind(hash).execute(&db).await.unwrap();
-        let app = Router::new()
-            .route("/room/{room_id}", get(|| async { StatusCode::OK }))
-            .route_layer(axum::middleware::from_fn_with_state(
-                db.clone(),
-                require_member,
-            ));
-        for (cookie, expected) in [
-            (None, StatusCode::UNAUTHORIZED),
-            (
-                Some("chat_session=invalid".to_owned()),
-                StatusCode::UNAUTHORIZED,
-            ),
-            (Some(format!("chat_session={token}")), StatusCode::OK),
-        ] {
-            let mut request = Request::get("/room/real-room");
-            if let Some(value) = cookie {
-                request = request.header("cookie", value);
-            }
-            let response = app
-                .clone()
-                .oneshot(request.body(Body::empty()).unwrap())
-                .await
-                .unwrap();
-            assert_eq!(response.status(), expected);
-        }
-        sqlx::query("UPDATE mbr_session SET expires_at=unixepoch()-1 WHERE id='s1'")
-            .execute(&db)
-            .await
-            .unwrap();
-        let expired = app
-            .oneshot(
-                Request::get("/room/real-room")
-                    .header("cookie", format!("chat_session={token}"))
-                    .body(Body::empty())
-                    .unwrap(),
-            )
-            .await
-            .unwrap();
-        assert_eq!(expired.status(), StatusCode::UNAUTHORIZED);
-    }
-
-    #[tokio::test]
-    async fn anonymous_create_is_rejected_while_public_list_is_available() {
-        let db = sqlx::sqlite::SqlitePoolOptions::new()
-            .max_connections(1)
-            .connect("sqlite::memory:")
-            .await
-            .unwrap();
-        chat_persistence::migrate(&db).await.unwrap();
-        let state = crate::state::AppState::new(
-            leptos_config::get_configuration(None)
-                .unwrap()
-                .leptos_options,
-            std::sync::Arc::new(chat_persistence::SqliteRoomRepository::new(db.clone())),
-            std::sync::Arc::new(chat_persistence::SqliteMessageRepository::new(db.clone())),
-        );
-        let room_methods = get(crate::http::list_rooms).merge(
-            axum::routing::post(crate::http::create_room).route_layer(
-                axum::middleware::from_fn_with_state(db.clone(), require_member),
-            ),
-        );
-        let app = Router::new()
-            .route("/api/rooms", room_methods)
-            .with_state(state);
-
-        let anonymous = app
-            .clone()
-            .oneshot(
-                Request::post("/api/rooms")
-                    .header("content-type", "application/json")
-                    .body(Body::from(r#"{"name":"anonymous"}"#))
-                    .unwrap(),
-            )
-            .await
-            .unwrap();
-        assert_eq!(anonymous.status(), StatusCode::UNAUTHORIZED);
-        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM rooms")
-            .fetch_one(&db)
-            .await
-            .unwrap();
-        assert_eq!(count, 0);
-
-        let public = app
-            .oneshot(Request::get("/api/rooms").body(Body::empty()).unwrap())
-            .await
-            .unwrap();
-        assert_eq!(public.status(), StatusCode::OK);
+        Ok(_) => {
+            // 登入成功也不能讓不存在的房間進入 SSR／WebSocket 畫面。
+            // 沿用正式 SQLite RoomRepository，而不是在 middleware 複製 SQL。
+            use chat_application::repository::RoomRepository;
+            let id = crate::room_path::canonical_room_id(request.uri().path())
+                .ok_or(StatusCode::NOT_FOUND)?;
+            let repo = chat_persistence::SqliteRoomRepository::new(pool.clone());
+            let room = repo.find_by_id(&id).await
+                .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+            if !room.is_some_and(|room| room.id == id) {
+                return Err(StatusCode::NOT_FOUND);
+            }
+            Ok(next.run(request).await)
+        },
+        Err(StatusCode::UNAUTHORIZED) => {
+            let id = crate::room_path::canonical_room_id(request.uri().path())
+                .ok_or(StatusCode::UNAUTHORIZED)?;
+            Ok(Redirect::to(&format!("/signin?next=/room/{id}")).into_response())
+        }
+        Err(error) => Err(error),
+    }
+}
+
+#[cfg(test)]
+mod day41_room_return_tests {
+    use super::*;
+    use axum::{Router, body::Body, http::Request, routing::get};
+    use tower::ServiceExt;
+
+    #[tokio::test]
+    async fn anonymous_room_link_redirects_but_invalid_room_id_is_rejected() {
+        let pool = sqlx::sqlite::SqlitePoolOptions::new()
+            .connect_lazy("sqlite::memory:")
+            .unwrap();
+        let app = Router::new()
+            .route("/room/{room_id}", get(|| async { StatusCode::OK }))
+            .route_layer(axum::middleware::from_fn_with_state(
+                pool,
+                require_room_member,
+            ));
+        let id = uuid::Uuid::new_v4();
+        let response = app
+            .clone()
+            .oneshot(
+                Request::get(format!("/room/{id}"))
+                    .body(Body::empty())
+                    .unwrap(),
+            )
+            .await
+            .unwrap();
+        assert_eq!(response.status(), StatusCode::SEE_OTHER);
+        assert_eq!(
+            response.headers()["location"],
+            format!("/signin?next=/room/{id}")
+        );
+        let bad = app
+            .oneshot(
+                Request::get("/room/not-a-uuid")
+                    .body(Body::empty())
+                    .unwrap(),
+            )
+            .await
+            .unwrap();
+        assert_eq!(bad.status(), StatusCode::UNAUTHORIZED);
+    }
+}
+
+#[cfg(test)]
+mod day25_tests {
+    use super::*;
+    use axum::{
+        Router,
+        body::Body,
+        http::{Request, StatusCode},
+        routing::get,
+    };
+    use sha2::{Digest, Sha256};
+    use tower::ServiceExt;
+
+    #[tokio::test]
+    async fn room_page_requires_active_sqlite_session() {
+        let db = sqlx::sqlite::SqlitePoolOptions::new()
+            .max_connections(1)
+            .connect("sqlite::memory:")
+            .await
+            .unwrap();
+        sqlx::raw_sql(include_str!(
+            "../../migrations/20261018073931_create_auth_user.sql"
+        ))
+        .execute(&db)
+        .await
+        .unwrap();
+        sqlx::raw_sql(include_str!(
+            "../../migrations/20261020152737_mbr_sessions.sql"
+        ))
+        .execute(&db)
+        .await
+        .unwrap();
+        sqlx::query("INSERT INTO mbr_user (id,user_acnt,user_pd,user_email,user_name,user_status,user_sexy,user_birthday,is_init_pd,created_at,updated_at) VALUES ('m1','member','unused','m1@example.test','Member','active','', '2000-01-01',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
+            .execute(&db).await.unwrap();
+        let token = "a".repeat(64);
+        let hash = Sha256::digest(token.as_bytes())
+            .iter()
+            .map(|b| format!("{b:02x}"))
+            .collect::<String>();
+        sqlx::query("INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at) VALUES('s1','m1',?1,unixepoch()+60)")
+            .bind(hash).execute(&db).await.unwrap();
+        let app = Router::new()
+            .route("/room/{room_id}", get(|| async { StatusCode::OK }))
+            .route_layer(axum::middleware::from_fn_with_state(
+                db.clone(),
+                require_member,
+            ));
+        for (cookie, expected) in [
+            (None, StatusCode::UNAUTHORIZED),
+            (
+                Some("chat_session=invalid".to_owned()),
+                StatusCode::UNAUTHORIZED,
+            ),
+            (Some(format!("chat_session={token}")), StatusCode::OK),
+        ] {
+            let mut request = Request::get("/room/real-room");
+            if let Some(value) = cookie {
+                request = request.header("cookie", value);
+            }
+            let response = app
+                .clone()
+                .oneshot(request.body(Body::empty()).unwrap())
+                .await
+                .unwrap();
+            assert_eq!(response.status(), expected);
+        }
+        sqlx::query("UPDATE mbr_session SET expires_at=unixepoch()-1 WHERE id='s1'")
+            .execute(&db)
+            .await
+            .unwrap();
+        let expired = app
+            .oneshot(
+                Request::get("/room/real-room")
+                    .header("cookie", format!("chat_session={token}"))
+                    .body(Body::empty())
+                    .unwrap(),
+            )
+            .await
+            .unwrap();
+        assert_eq!(expired.status(), StatusCode::UNAUTHORIZED);
+    }
+
+    #[tokio::test]
+    async fn anonymous_create_is_rejected_while_public_list_is_available() {
+        let db = sqlx::sqlite::SqlitePoolOptions::new()
+            .max_connections(1)
+            .connect("sqlite::memory:")
+            .await
+            .unwrap();
+        chat_persistence::migrate(&db).await.unwrap();
+        let state = crate::state::AppState::new(
+            leptos_config::get_configuration(None)
+                .unwrap()
+                .leptos_options,
+            std::sync::Arc::new(chat_persistence::SqliteRoomRepository::new(db.clone())),
+            std::sync::Arc::new(chat_persistence::SqliteMessageRepository::new(db.clone())),
+        );
+        let room_methods = get(crate::http::list_rooms).merge(
+            axum::routing::post(crate::http::create_room).route_layer(
+                axum::middleware::from_fn_with_state(db.clone(), require_member),
+            ),
+        );
+        let app = Router::new()
+            .route("/api/rooms", room_methods)
+            .with_state(state);
+
+        let anonymous = app
+            .clone()
+            .oneshot(
+                Request::post("/api/rooms")
+                    .header("content-type", "application/json")
+                    .body(Body::from(r#"{"name":"anonymous"}"#))
+                    .unwrap(),
+            )
+            .await
+            .unwrap();
+        assert_eq!(anonymous.status(), StatusCode::UNAUTHORIZED);
+        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM rooms")
+            .fetch_one(&db)
+            .await
+            .unwrap();
+        assert_eq!(count, 0);
+
+        let public = app
+            .oneshot(Request::get("/api/rooms").body(Body::empty()).unwrap())
+            .await
+            .unwrap();
+        assert_eq!(public.status(), StatusCode::OK);
+        db.close().await;
+    }
+}
+
+#[cfg(test)]
+mod day44_existing_room_gate_tests {
+    use super::*;
+    use axum::{Router, body::Body, http::Request, routing::get};
+    use tower::ServiceExt;
+
+    #[tokio::test]
+    async fn signed_in_room_navigation_rejects_missing_and_allows_saved_room() {
+        let db = sqlx::sqlite::SqlitePoolOptions::new()
+            .max_connections(1).connect("sqlite::memory:").await.unwrap();
+        chat_persistence::migrate(&db).await.unwrap();
+        let room_id = uuid::Uuid::new_v4().to_string();
+        let missing_id = uuid::Uuid::new_v4().to_string();
+        sqlx::query("INSERT INTO rooms(id,category_room_id,name,desc,created_at,updated_at)
+            VALUES(?1,'default-category','實際房間','',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
+            .bind(&room_id).execute(&db).await.unwrap();
+        sqlx::query("INSERT INTO mbr_user
+            (id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at)
+            VALUES('day44-user','day44','hash','day44@example.invalid','Member','active',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
+            .execute(&db).await.unwrap();
+        let token = "a".repeat(64);
+        sqlx::query("INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at)
+            VALUES('day44-session','day44-user',?1,unixepoch()+60)")
+            .bind(shared::auth::token_digest(&token)).execute(&db).await.unwrap();
+        let app = Router::new()
+            .route("/room/{room_id}", get(|| async { StatusCode::OK }))
+            .route_layer(axum::middleware::from_fn_with_state(db.clone(), require_room_member));
+        for (id, expected) in [(&room_id, StatusCode::OK), (&missing_id, StatusCode::NOT_FOUND)] {
+            let response = app.clone().oneshot(
+                Request::get(format!("/room/{id}"))
+                    .header("cookie", format!("chat_session={token}"))
+                    .body(Body::empty()).unwrap()
+            ).await.unwrap();
+            assert_eq!(response.status(), expected, "{id}");
+        }
+        let anonymous = app.clone().oneshot(
+            Request::get(format!("/room/{room_id}")).body(Body::empty()).unwrap()
+        ).await.unwrap();
+        assert_eq!(anonymous.status(), StatusCode::SEE_OTHER);
+        assert_eq!(anonymous.headers()["location"], format!("/signin?next=/room/{room_id}"));
         db.close().await;
     }
 }
```

**相依／接線**：`chat-server` 原有 `chat-application`、`chat-persistence`、`shared`、`uuid`、`sqlx`、`tower`；Cargo.toml／Cargo.lock／features **不變**。用 `RoomRepository` trait import 使 `find_by_id` 真正 dispatch 至 SQLite Adapter。HTTP 404 是不存在，不等於私房 ACL 403。舊測試未遷移／刪除，新增斷言與舊斷言分層：路徑純單元、Session／middleware 真 SQLite、原 HTTP／SSR。

## ③ 當日官方穩定版、lock 與相容性

2026-10-30 查官方 Rust releases 與 docs.rs 的 crate stable 頁，排除 RC／alpha；`chat_apps/Cargo.toml` 宣告及 `chat_apps/Cargo.lock@bd4da0101ec6282d67965674159aa965bd1ae35e` 解析如下。**本機 rustc／cargo 1.99.1，lock 未升級**。

| 項目 | manifest → resolved | 最新 stable／限制 | 官方來源 |
| --- | --- | --- | --- |
| Rust stable | 1.99.1 (2026-10-15) | stable channel, no installed rustc | https://blog.rust-lang.org/releases/ |
| Axum | ^0.8 → 0.8.9 | 0.8.9 | https://docs.rs/crate/axum/latest |
| SQLx SQLite | ^0.9 → 0.9.0 | 0.9.0; MSRV 1.94 | https://docs.rs/crate/sqlx/latest |
| Tokio | ^1.54 → 1.54.1 | 1.54.1 | https://docs.rs/crate/tokio/latest |
| UUID | ^1.29 → 1.29.0 | 1.29.0 | https://docs.rs/crate/uuid/latest |
| Argon2 | ^0.6 → 0.6.0 | 0.6.0 | https://docs.rs/crate/argon2/latest |
| Leptos / router (既有 SSR) | ^0.8 → 0.8.22 / 0.8.17 | 0.8.22 / 0.8.17; 0.9 RC 不採用 | https://docs.rs/crate/leptos/latest · https://docs.rs/crate/leptos_router/latest |
| SHA2 / serde / serde_json | ^0.11 / ^1 / ^1 | 0.11.1 / 1.0.229 / 1.0.154 | https://docs.rs/crate/sha2/latest · https://docs.rs/crate/serde/latest · https://docs.rs/crate/serde_json/latest |

SQLx 0.9 的 MSRV 1.94，Rust 2024 edition 及現行 stable 1.99.1 足夠；Leptos SSR／WASM hydrate 仍須分開實測。這些是版本 metadata，不能代替 build。

## ④ 驗證、Gate 與未完成

**文件與真套用**：原 Refactor→Green 已實際 check／apply；格式修補後編譯／測試通過，精確命令與退出碼見本次證據。原始程式／附件不覆寫為另一個來源版本；最新 main 的 caller 依 source-inventory 核對。完整官方套件／MSRV Gate 尚未全量重核，不冒稱完整文件版本 Gate。

**本地已執行、完整 Gate2 未完成**：最新 main 的四 workspace 檢查、Chat62／SSR62與hydrate WASM通過；未在此 SHA 重跑獨立瀏覽器／完整HTTP與WS旅程。原 Chat 行為 Red 沒有重跑，不將基準測試成功當 Red。產品沒有候選提交／推送；以下為需依來源重現的命令：
```bash
cargo test --manifest-path chat_apps/Cargo.toml -p chat-server room_path
cargo test --manifest-path chat_apps/Cargo.toml -p chat-server room_auth
cargo test --manifest-path chat_apps/Cargo.toml -p chat-server --features ssr
cargo test --manifest-path chat_apps/Cargo.toml -p chat-ui --features ssr
cargo check --manifest-path chat_apps/Cargo.toml --workspace
```
並用真 SQLite 登入／登出、存在／不存在房間、Cookie 撤銷、WS 401、公開列表與房間 ACL 旅程驗證。下一個功能優先：會員身份與房主／逐房 ACL（含 WS 重驗）；此功能已在最新 main 實作並有回歸測試，但不代表完整逐房 ACL 已驗收。FIFO `chat-day25-shared-auth-20261022` 仍 pending；未經完整補教不結案。核心 60 分鐘只分配本篇 Red／Green 預估 15 分鐘，Refactor、版本、驗證與交付另計。
