# Chat Day29｜WebSocket 握手前驗證 SQLite 會員 Session

## ① 學習大綱與真實現況

唯讀程式來源：`williechen/rust_ai_chat/main@143d6b3f5180b669860b1300286b89464c669434`；教學規則：`williechen/copilot_use/docs/COMMON_LEARNING_RULES.md`，blob `5df88ba6892c377e46b6a76a934b9f59b51694e7`；規則 `daily-tdd-minimum-v1 / github-only-v1`。本文僅教學，不修改程式。

已核對 `crates/ui/src/lib.rs::SignInPage` 具備登入表單，`apps/server/src/signin.rs` 使用 SQLite `mbr_session`；`apps/server/src/auth_session.rs::active_member` 驗證 Cookie、token 雜湊、期限與會員啟用狀態；`apps/server/src/room_auth.rs::require_member` 已保護 `GET /room/{room_id}`。但 `apps/server/src/lib.rs` 仍把 `/ws/{room_id}` 直接掛在公開 Router，`ws::ws_handler` 僅檢查房間存在，沒有驗證會員。Day28 登入表單已套用，不重做。

本日最小切片：瀏覽器 `ChatPage` → `/ws/{room_id}` → Axum middleware → `active_member` → SQLite `mbr_session` → 原有 `ws_handler` → WebSocket upgrade。匿名、偽造或過期 Session 在升級前被拒絕；仍未完成逐房間 ACL。Chat、Pet、Analytics 各 15 分鐘，彈性 15 分鐘，共 60 分鐘；整合測試另計。

## ② 真 Red → 完整最小 Green → Refactor／FIFO

**Red（生產入口）：** 在套用前啟動隔離測試伺服器及已存在的房間，使用無 Cookie 的合法 WebSocket Upgrade 請求 `GET /ws/{room_id}`。目前 Router 未套用 `require_member`，若房間存在，預期可進入升級；安全需求則要求 HTTP 401。這是依程式靜態分析的**預期失敗**，未實際執行。單獨測試 middleware 不能取代生產路由整合測試。

**Green：** 在 `apps/server/src/lib.rs` 既有 `guarded_room` 後新增 `guarded_ws`，從公開主 Router 移除原本的 `.route("/ws/{room_id}", any(ws::ws_handler))`，並在 `.with_state(state)` 之前合併：

```rust
let guarded_ws = Router::new()
    .route("/ws/{room_id}", any(ws::ws_handler))
    .route_layer(axum::middleware::from_fn_with_state(
        db.clone(),
        room_auth::require_member,
    ));

let app = Router::new()
    .route_service(&pkg_path, pkg_service)
    .route("/health", get(http::health))
    .route("/api/rooms", get(http::list_rooms).post(http::create_room))
    .route("/api/rooms/{id}", get(http::get_room))
    .route("/signin", get(ssr::app_handler))
    .route("/", get(ssr::app_handler))
    .merge(guarded_room)
    .merge(guarded_ws)
    .with_state(state)
    .merge(signup_routes)
    .merge(signin_routes)
    .merge(logout_routes);
```

保留原本的 `guarded_room` 宣告、`db`、`state` 與其他路由；不要在同一 Router 再宣告第二條未受保護的 `/ws/{room_id}`。Middleware 會在 Upgrade 前驗證會員，並沿用既有 SQLx SQLite Session，不新增資料表或 Cookie 政策。

**Red/Green 回歸測試：** 在隔離測試環境用相同房間分別測試無 Cookie、偽造 Cookie、過期 Cookie、有效會員 Cookie；前三者預期 401，最後一者預期 101 並能正常收發。須用真正 HTTP/WebSocket Upgrade 路由測試，不可只用模擬 handler 回傳 200 的測試替代。

**Refactor／FIFO：** 身分驗證維持在 `room_auth::require_member`，不要在 `ws_handler` 複製 Cookie 或 SQL。此修補只證明會員身分，不代表房間成員 ACL 已完成；`POST /api/rooms` 也仍需授權。Day25 `chat-day25-shared-auth-20261022` 更正項繼續 pending，未驗證完整登入／登出共用政策，不回寫 Day25（含）以前內容。

## ③ Cargo、lock 與版本

`rust_ai_chat` workspace `version = "0.2.28"`、`edition = "2024"`；`axum = "0.8"`、`sqlx = "0.9"`、`leptos = "0.8"`、`tokio = "1.54"`、`argon2 = "0.6"`（皆為 manifest 約束，不是 lockfile resolved 精確版本）。本日不新增 crate、migration 或修改 Cargo.lock。未驗證本機 rustc、MSRV。

## ④ 命令、預期／實際、CI 與限制

| 命令／流程 | 預期 | 本次實際 |
| --- | --- | --- |
| `cargo check --locked -p server --features ssr` | 新 Router 型別及 middleware 可編譯 | 未執行 |
| `cargo test --locked -p server room_page_requires_active_sqlite_session` | 原有會員 Session 驗證不退化 | 未執行 |
| 匿名 WebSocket Upgrade `/ws/{existing_room}` | 401，不能取得 101 | 未執行 |
| 偽造／過期 Cookie WebSocket Upgrade | 401 | 未執行 |
| 有效會員登入後 WebSocket Upgrade 與訊息收發 | 101 且功能不退化 | 未執行 |
| `POST /api/rooms`、跨房間 ACL | 尚未修復，另列後續缺口 | 未執行 |

來源 SHA：`143d6b3f5180b669860b1300286b89464c669434`。本次未取得此 SHA 的精確 CI workflow/job 成功證據，不能引用其他 SHA 的 CI 充當驗收。本文是 GitHub 教學交付，沒有執行 Red/Green、修改 `rust_ai_chat`、操作 DB、部署或完成產品功能驗收。
