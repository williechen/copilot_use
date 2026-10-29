# Chat Day36 r1｜登出後既有 WebSocket 會話定期撤銷，阻止後續訊息

## ① 學習大綱：從真實 WS 入口補 Session 生命週期

**日期：2026-10-25；文件庫：`williechen/copilot_use/main`；唯一程式來源：`williechen/rust_ai_ring/main@61ed5930bd9ff38e0d5822eea2983121001d8422`；教學已撰寫／程式未套用／新測試未執行／未驗收。** 以即時 `next_day.chat=36` 銜接 [Day35 房名規則](../day-35/chat.md)；該規則已在後續產品來源接通，無須重貼。本日最小切片是**已登入 WS 連線在使用者登出、停用或 Session 到期後，不能持續收發**，而非另造登入系統。

核對到的真實鏈：`chat_ui` 的 `chat_socket.rs` → `/ws/{room_id}` → `chat_server/src/lib.rs` 的 `guarded_ws`（僅在握手時 `room_auth::require_member`）→ `ws::ws_handler` → `handle_socket` → `ChatService`／`chat_persistence` SQLite 訊息與 `RoomHub`。`logout.rs` 刪除 `mbr_session`，`auth_session::active_member` 已驗證 `token_hash`、`expires_at`、`mbr_user.user_status`，但既有 `handle_socket` 未重查，導致握手後已登出的連線仍可寫入。此日**重用**同一 SQLite Session adapter，不新增 token cache、平行 Hub、WS 身分系統或 Leptos UI。

範圍：收到任一 client frame 前、轉送廣播前，以及閒置期間每秒檢查 Session；無效時送 WS Close 並退出；DB 不可用也 fail closed。Session 仍由登入建立、登出刪除，沒有改 schema。**並發界線**：驗證與訊息 INSERT 之間仍可能與登出競態；若要求嚴格的同一交易撤銷／訊息線性化，須後續將 Session 驗證與訊息寫入放進同一 SQLite 交易；本日不虛稱原子保證。逐房 ACL、作者身分、歷史訊息、匿名選房回跳仍未完成。

## ② Red → 完整最小 Green → Refactor

### Red：既有 WS 握手守衛不能保護已建立連線

在 `chat_apps/chat_server/src/ws.rs` 原有 `#[cfg(test)] mod test` 增加以下兩項測試（**先不要加入 Green 函式**）；此時 `socket_session_active` 不存在，預期為缺少符號的**編譯 Red**，不是環境錯誤。測試使用現有 migration／Session schema，不自造 SQLite 表：

~~~rust
#[sqlx::test(migrations = "../migrations")]
async fn day36_revoked_session_is_not_active(pool: sqlx::SqlitePool) {
    use super::socket_session_active;
    use axum::http::{HeaderMap, header};
    use shared::auth::token_digest;
    let raw = "a".repeat(64);
    sqlx::query(
        "INSERT INTO mbr_user
         (id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at)
         VALUES('day36-user','day36','hash','day36@example.invalid','Day36','active',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)"
    ).execute(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at)
         VALUES('day36-session','day36-user',?1,unixepoch()+60)"
    ).bind(token_digest(&raw)).execute(&pool).await.unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(header::COOKIE, format!("chat_session={raw}").parse().unwrap());
    assert!(socket_session_active(&pool, &headers).await);
    sqlx::query("DELETE FROM mbr_session WHERE id='day36-session'")
        .execute(&pool).await.unwrap();
    assert!(!socket_session_active(&pool, &headers).await);
}

#[sqlx::test(migrations = "../migrations")]
async fn day36_database_error_fails_closed(pool: sqlx::SqlitePool) {
    use super::socket_session_active;
    use axum::http::{HeaderMap, header};
    let mut headers = HeaderMap::new();
    headers.insert(header::COOKIE, format!("chat_session={}", "a".repeat(64)).parse().unwrap());
    pool.close().await;
    assert!(!socket_session_active(&pool, &headers).await);
}
~~~

### Green A：真實路由將 DB 注入 WS（無 AppState 全庫簽名變更）

`chat_apps/chat_server/src/lib.rs` 的 `guarded_ws` **完整替換為**：

~~~rust
let guarded_ws = Router::new()
    .route("/ws/{room_id}", any(ws::ws_handler))
    // 此 pool 僅給已存在的 WS handler 重驗 Session；不改其他路由的 State。
    .layer(axum::Extension(db.clone()))
    .route_layer(axum::middleware::from_fn_with_state(
        db.clone(),
        room_auth::require_member,
    ));
~~~

`chat_apps/chat_server/src/ws.rs`：既有 `use crate::state::AppState;`、`RoomHub`、`ChatService` 等 imports 保留，**補**：

~~~rust
use crate::auth_session::active_member;
use axum::{Extension, http::HeaderMap};
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::time::MissedTickBehavior;
~~~

同檔 `ws_handler` **整個替換**（原 `room_repository.find_by_id` 404/500 契約不變）：

~~~rust
pub async fn ws_handler(
    Path(room_id): Path<String>,
    State(state): State<AppState>,
    Extension(pool): Extension<SqlitePool>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> Response {
    match state.room_repository.find_by_id(&room_id).await {
        Ok(Some(_)) => ws.on_upgrade(move |socket| {
            handle_socket(socket, state, room_id, pool, headers)
        }),
        Ok(None) => Response::builder()
            .status(404).body("Room not found".into()).unwrap(),
        Err(error) => {
            eprintln!("websocket room lookup error: {error:?}");
            Response::builder().status(500)
                .body("Internal server error".into()).unwrap()
        }
    }
}
~~~

在 `ws.rs` `handle_socket` 之前**新增** Session helper，並將 `handle_socket` **整個替換**。保留原 `send_event`、`persist_ws_message`、`handle_client_event` 及所有既有事件檢查，不要改其 SQL／broadcast 契約：

~~~rust
async fn socket_session_active(pool: &SqlitePool, headers: &HeaderMap) -> bool {
    active_member(pool, headers).await.is_ok()
}

async fn handle_socket(
    mut socket: WebSocket,
    state: AppState,
    room_id: String,
    pool: SqlitePool,
    headers: HeaderMap,
) {
    let (room_sender, mut room_events) = state.room_hub.subscribe(room_id.clone());
    let mut recheck = tokio::time::interval(Duration::from_secs(1));
    recheck.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        tokio::select! {
            _ = recheck.tick() => {
                if !socket_session_active(&pool, &headers).await {
                    let _ = socket.send(Message::Close(None)).await;
                    break;
                }
            }
            incoming = socket.recv() => {
                let Some(result) = incoming else { break; };
                let message = match result {
                    Ok(message) => message,
                    Err(error) => {
                        eprintln!("websocket receive error: {error}");
                        break;
                    }
                };
                // 不因握手時曾通過就永久信任 Cookie。
                if !socket_session_active(&pool, &headers).await {
                    let _ = socket.send(Message::Close(None)).await;
                    break;
                }
                if let Message::Text(text) = message
                    && let Err(error) = handle_client_event(
                        &mut socket, &state, text.as_str(), room_id.clone()
                    ).await {
                        eprintln!("client message error: {error}");
                        break;
                    }
            }
            event = room_events.recv() => {
                match event {
                    Ok(event) => {
                        if !socket_session_active(&pool, &headers).await {
                            let _ = socket.send(Message::Close(None)).await;
                            break;
                        }
                        if let Err(error) = send_event(&mut socket, event).await {
                            eprintln!("websocket send error: {error}");
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(skipped)) => {
                        eprintln!("websocket receiver lagged skipped: {skipped} events");
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
    drop(room_events);
    state.room_hub.cleanup(room_id, &room_sender);
}
~~~

### Refactor、Cargo 與上一篇補充

這次只讓 `ws_handler` 取得現有 `SqlitePool` 和 Cookie header，避免修改 `AppState::new` 的大量測試呼叫端；`active_member` 是唯一 Session 驗證規則，沒有複製 SQL。既有 `guarded_room`、`/api/rooms`、`signin.rs`、`logout.rs`、`chat_ui` SSR／hydrate、`chat_persistence`、`crates/shared` **不修改**。`chat_apps/Cargo.toml` 已有 `axum 0.8/ws`、`sqlx 0.9/sqlite`、`tokio 1.54/full`；`chat_server/Cargo.toml` 已直接依賴三者，故**沒有新增 Cargo dependency／feature、Cargo.lock 變更或 Leptos metadata 變更**。使用 `Extension<SqlitePool>` 是 Axum 0.8 現有功能，並非新增 crate。

Day35 已在後續來源接通共用房名規則，本日不回填 Day35；Day26 `logout` 只撤銷 DB Session，舊測試只驗新 HTTP 請求，沒有涵蓋既有 WS，故新增上述 Red。Day25 共用認證 FIFO、選房回跳與逐房 ACL 繼續追蹤；`service_name="rust_ai_chat"` 是現存歷史程式字串，**不是現行 repository 名稱**。沒有足夠證據判斷既有 helper 可刪，故不做無關清理。

## ③ 套件與軟體版本（2026-10-25 官方穩定版）

| 技術 | 當次 manifest → Cargo.lock | 2026-10-25 官方最新穩定／來源與判斷 |
| --- | --- | --- |
| Rust/Cargo | 四 workspace `rust-toolchain.toml` 為 stable（安裝版未測） | [Rust 1.99.1，10/15 發行](https://blog.rust-lang.org/releases/)；edition 2024 最低 1.85，建議 stable 1.99.1 |
| Axum WS | `chat_apps` 0.8/ws → **0.8.9** | [axum 0.8.9](https://docs.rs/crate/axum/latest)，沿用，無新 feature |
| Tokio | 1.54/full → **1.54.1** | [tokio 1.54.1](https://docs.rs/crate/tokio/latest)，`interval` 已可用 |
| SQLx SQLite | 0.9/sqlite/runtime-tokio/macros → **0.9.0** | [sqlx 0.9.0](https://docs.rs/crate/sqlx/latest)，沿用 |
| Leptos / Router | 0.8 → **0.8.22 / 0.8.17** | [Leptos](https://docs.rs/crate/leptos/latest)／[Router](https://docs.rs/crate/leptos_router/latest)；本日 UI 無改 |
| SQLite | SQLx 實際連結的 engine 未量測 | [SQLite 3.54.1](https://www.sqlite.org/download.html) 為官方發行，**不是**本機 engine 宣告 |

四份 lock blob（來源 `61ed5930`）：Chat `384bcb23a13bc0f4e876f29e3037f2181ebbc3d5`、Pet `7097f943da5c88be851979cec74641fa35e6e613`、Analytics `44abb16f1ec3f74fa181c69d46e8bd984a090f21`、Crates `c5ec8079981d59244ce5482258424cd8ff57f9a3`。已核對四份 workspace manifests/locks；不自動升級，不將 RC/nightly 視為穩定。native/WASM 與實際 MSRV 組合未執行，若使用者工具鏈落後須先查 `rustc --version`。

## ④ 驗證步驟與實際狀態

**在使用者產品庫**（不是 copilot_use）執行；`cargo test` 會驗上述 SQLite Session adapter，WS 原生連線須另做真 HTTP Upgrade／瀏覽器測試：

~~~bash
cd rust_ai_ring/chat_apps
cargo metadata --locked --format-version 1 --no-deps
cargo test --locked -p chat-server --features ssr day36_
cargo test --locked -p chat-server --features ssr
cargo test --locked -p chat-ui --features ssr
cargo check --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown
cargo fmt --all -- --check
# 在隔離 SQLite：SELECT sqlite_version();
~~~

Red 預期新 helper 不存在而編譯失敗；Green 預期撤銷、過期、停用與 DB 失敗均關閉 WS；實機以真登入 Cookie 開兩個同房 WS，A 登出後 ≤1 秒內 A 連線收到 Close、B 繼續可用；重試 A 送訊息不增加 `messages` 筆數、不觸發 RoomHub，停用帳號及過期同理。測試競態須加入同交易 Session／INSERT 的後續切片；未經真 WS/DB/GUI 測試不能宣稱全數通過。

**本次實際**：僅寫入 copilot_use 教學，`rust_ai_ring` 完全唯讀；新 Red、Green、SSR、WASM、SQLite、WS E2E 與 CI 均**未執行**，程式**未套用**、Gate 2 **未驗收**。Day36 文件提交／全文 SHA256／首頁同步與狀態另記；`last_full_comparison` 不變。