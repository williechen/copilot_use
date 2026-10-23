# Chat Day30｜公開房間清單、建立房間須驗證會員 Session

## ① 學習大綱與真實現況

文件正本：`williechen/copilot_use/main`，規則 `docs/COMMON_LEARNING_RULES.md`（daily-tdd-minimum-v1／github-only-v1）。程式只讀來源：`williechen/rust_ai_chat/main@9c2b7cf5a4c60a08254445911916c4ed9b02d7b3`。本日沿 Day29：`apps/server/src/lib.rs` 已以 `room_auth::require_member` 保護 `/ws/{room_id}`，不重教 WebSocket Session；`/room/{room_id}` 亦已有同一 middleware。但同檔 `.route("/api/rooms", get(http::list_rooms).post(http::create_room))` 將建立操作公開。現有 `crates/ui/src/room_api.rs::create_room` 使用同源 `fetch` POST；`apps/server/src/http.rs::create_room` 驗證名稱後寫入 `chat_application::create_room → persistence::SqliteRoomRepository`。匿名者因此仍可直接建立房間。需求允許公開讀取清單，但禁止匿名建立。

本日最小切片：Leptos 房間建立 UI → 同源 `POST /api/rooms`（瀏覽器 Cookie）→ Axum Session middleware → `active_member`（SQLite `mbr_session/mbr_user`）→ 既有 HTTP handler → application → SQLite rooms；`GET /api/rooms` 仍公開。此切片只限制「登入會員能否建立」；尚無房間角色/擁有權 ACL。學習：Chat 15、Pet 15、Analytics 15、共用 15，共 60 分鐘；程式整合與驗收另計。

## ② 真 Red → 完整最小 Green → Refactor／FIFO

**Red：生產 Router 的可重現失敗**。在隔離 SQLite 依現有 migrations 啟動服務，無 Cookie 使用 `curl -i -X POST http://127.0.0.1:3000/api/rooms -H 'Content-Type: application/json' -d '{"name":"匿名房間"}'`。現有公開 route 對有效名稱可返回 201 並寫入一列；安全預期 401、rooms 數量不變。這是由精確 source 推導的預期 Red，**本次未實際執行**。既有 `http::room_http_tests::valid_room_returns_201_and_saved_id` 直接呼叫 handler，不能用來證明 Router ACL。

**Green：完整 Router 修改**。在 `apps/server/src/lib.rs` 既有 `guarded_ws` 之後、`let app` 之前加入：

```rust
// 建立房間需要有效會員；公開列表保持可讀。
let guarded_room_create = Router::new()
    .route("/api/rooms", axum::routing::post(http::create_room))
    .route_layer(axum::middleware::from_fn_with_state(
        db.clone(),
        room_auth::require_member,
    ));
```

將 `let app = Router::new()` 鏈中原本這一行：

```rust
.route("/api/rooms", get(http::list_rooms).post(http::create_room))
```

**完整替換**成以下一行；並在原本 `.merge(guarded_ws)` 後、`.with_state(state)` 前合併受保護 Router：

```rust
.route("/api/rooms", get(http::list_rooms))
.route("/api/rooms/{id}", get(http::get_room))
// 其餘既有 /signin、/、guarded_room、guarded_ws 不變
.merge(guarded_room_create)
```

**注意**：此處 `/api/rooms/{id}` 僅展示相鄰上下文，保留其原有一次 route，**不要再次新增**；也不要重複合併 `guarded_room_create`。最終同一路徑 GET 公開、POST 經 middleware，不得留下公開 POST。若 Axum 0.8 對兩個 Router 合併相同路徑不同 method 的行為或型別有差異，以編譯及 Router HTTP 回歸為準；不得因未驗證而宣稱 Green 通過。

**新增真實 Router 測試**：在 `apps/server/src/room_auth.rs` 既有 `day25_tests` module 中加入此測試，沿用同 module 的 `use super::*;`、`tower::ServiceExt` 與 `axum` imports：

```rust
#[tokio::test]
async fn anonymous_create_is_rejected_while_public_list_is_available() {
    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    persistence::migrate(&db).await.unwrap();
    let state = crate::state::AppState::new(
        leptos_config::get_configuration(None).unwrap().leptos_options,
        std::sync::Arc::new(persistence::SqliteRoomRepository::new(db.clone())),
        std::sync::Arc::new(persistence::SqliteMessageRepository::new(db.clone())),
    );
    let guarded_create = Router::new()
        .route("/api/rooms", axum::routing::post(crate::http::create_room))
        .route_layer(axum::middleware::from_fn_with_state(
            db.clone(), require_member,
        ));
    let app = Router::new()
        .route("/api/rooms", get(crate::http::list_rooms))
        .merge(guarded_create)
        .with_state(state);

    let anonymous = app.clone().oneshot(
        Request::post("/api/rooms")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name":"anonymous"}"#)).unwrap(),
    ).await.unwrap();
    assert_eq!(anonymous.status(), StatusCode::UNAUTHORIZED);
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM rooms")
        .fetch_one(&db).await.unwrap();
    assert_eq!(count, 0);

    let public = app.oneshot(
        Request::get("/api/rooms").body(Body::empty()).unwrap(),
    ).await.unwrap();
    assert_eq!(public.status(), StatusCode::OK);
    db.close().await;
}
```

此測試刻意經過真實 Axum Router 和 SQLite，但使用測試建構的 route；**仍須**驗證生產 `application()` 路由接線、有效 Session 的 POST 201、登出後 401。測試 Red 應先以原公開 POST Router 執行並確認失敗，再套用 Green；直接在測試中提前使用 guarded router 會失去 Red 證明，不能冒充已執行 TDD。既有 Session fixture 可沿 `room_auth::day25_tests` 建立 `mbr_user/mbr_session` 與 token digest 驗證有效、偽造、過期 Cookie。

**Refactor**：只重用 `room_auth::require_member`，不在 `http::create_room` 重複解析 Cookie；保留 `chat_application` 與 SQLite adapter 既有契約。未實作 CSRF 對策、建立者 ownership 與逐房 ACL，這些不可因 401 而視為完成。

**FIFO 舊 Day 補教（本篇最多一項）**：`chat-day25-shared-auth-20261022` 仍 pending。Day25 舊文及證據不回寫；原登入／登出／Cookie／Session 尚有共用化缺口。此處只識別需沿真實 `signin.rs`、`logout.rs`、`auth_session.rs` 統一 token/hash/撤銷策略；未提供已核實完整補教實作，因此**本篇不結案、不變更 makeup_queue**。

## ③ 套件版本

本案沿用 `rust_ai_chat/Cargo.toml` 的既有 workspace 依賴，`apps/server/Cargo.toml` 已含 `axum.workspace`、`sqlx.workspace`、`tower = { version = "0.5", features = ["util"] }`（dev）。不新增 crate、migration、Cargo.lock 異動；lockfile resolved 版本／MSRV 本次**未確認**，不可將 manifest 限制誤稱精確鎖定版本。

## ④ 命令、預期／實際、CI/job/SHA 與限制

| 命令或 Gate | 預期 | 本次實際 |
| --- | --- | --- |
| `cargo test --locked -p server anonymous_create_is_rejected_while_public_list_is_available` | 無 Cookie POST 401、無 DB 寫入；公開 GET 200 | 未執行 |
| `cargo check --locked -p server --features ssr` | Axum 合併 Router 可編譯 | 未執行 |
| `curl -i -X POST http://127.0.0.1:3000/api/rooms -H 'Content-Type: application/json' -d '{"name":"匿名房間"}'` | 401，不能建立 | 未執行 |
| 有效 Cookie POST，登出後相同 Cookie POST | 先 201；撤銷後 401 | 未執行 |
| `cargo test --locked -p server` | 既有 SSR/Session/Room tests 無退化 | 未執行 |
| Browser Leptos 操作 + SQLite 重開讀回 | 建立結果持久且有權限 | 未執行 |

CI 精確來源 SHA `9c2b7cf5a4c60a08254445911916c4ed9b02d7b3`：`Rust AI Chat` workflow run 39891849782 為 success；`Test All` 39891849769 為 success；`Persistence` 39891849758 為 success；`Node` 39891849761 為 failure。各 job 與本切片的覆蓋未逐一核實；這是**來源既有 CI** 而非本文 Green 測試通過。未執行 Red、Green、DB、瀏覽器、部署、正式功能驗收，也**未寫入 rust_ai_chat**。最終逐房成員 ACL、CSRF 與登入登出共同政策仍待後續實作。
