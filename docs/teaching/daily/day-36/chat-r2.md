# Chat Day36 r2｜登出後既有 WebSocket 會話定期撤銷，阻止後續訊息

> **2026-10-25 實測更新：程式已存在於 [rust_ai_ring/main@5c713553](https://github.com/williechen/rust_ai_ring/commit/5c713553daee4055738239f047d6c774fcc7a7a1)；相關測試已執行通過；完整產品尚未驗收。** 本次只更新文件，產品庫保持唯讀。下方問題分析及 Red／Green 教材以原始 `61ed5930` 為基準；最新 main 已含 Green，請勿重複套用。
>
> 本修訂取代「目前程式未套用／未測試」摘要；[原始文章](chat.md)及其 commit/blob/SHA256、原始交付證據保持不變。Day36 不加號，next_day 維持 37。各命令、原始錯誤與環境復原見[完整實測報告](../../../source-review/2026-10-25-day36-verification-5c713553.md)，[logs／腳本與證據包](../../../source-review/day36-verification-5c713553/evidence.zip)。

## ① 學習大綱：從真實 WS 入口補 Session 生命週期

**日期：2026-10-25；文件庫：`williechen/copilot_use/main`；唯一程式來源：`williechen/rust_ai_ring/main@61ed5930bd9ff38e0d5822eea2983121001d8422`；原始教材來源；目前實測狀態以下方第④節為準。** 以即時 `next_day.chat=36` 銜接 [Day35 房名規則](../day-35/chat.md)；該規則已在後續產品來源接通，無須重貼。本日最小切片是**已登入 WS 連線在使用者登出、停用或 Session 到期後，不能持續收發**，而非另造登入系統。

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

## ③ 原始教材套件版本表（官方最新宣稱本次未重新核實）

本次已量測 Rust/Cargo 1.99.1、native/WASM target，並核對四 workspace 的 manifests／locks；所有 Cargo.toml 與 Cargo.lock 相對 `61ed5930` 未變更。以下版本表保留原始教學查核範圍，不據此宣稱本次重新查證了所有官方最新版本；SQLx 連結的 SQLite engine 未量測。

| 技術 | 當次 manifest → Cargo.lock | 2026-10-25 官方最新穩定／來源與判斷 |
| --- | --- | --- |
| Rust/Cargo | 四 workspace `rust-toolchain.toml` 為 stable（安裝版未測） | [Rust 1.99.1，10/15 發行](https://blog.rust-lang.org/releases/)；edition 2024 最低 1.85，建議 stable 1.99.1 |
| Axum WS | `chat_apps` 0.8/ws → **0.8.9** | [axum 0.8.9](https://docs.rs/crate/axum/latest)，沿用，無新 feature |
| Tokio | 1.54/full → **1.54.1** | [tokio 1.54.1](https://docs.rs/crate/tokio/latest)，`interval` 已可用 |
| SQLx SQLite | 0.9/sqlite/runtime-tokio/macros → **0.9.0** | [sqlx 0.9.0](https://docs.rs/crate/sqlx/latest)，沿用 |
| Leptos / Router | 0.8 → **0.8.22 / 0.8.17** | [Leptos](https://docs.rs/crate/leptos/latest)／[Router](https://docs.rs/crate/leptos_router/latest)；本日 UI 無改 |
| SQLite | SQLx 實際連結的 engine 未量測 | [SQLite 3.54.1](https://www.sqlite.org/download.html) 為官方發行，**不是**本機 engine 宣告 |

四份 lock blob（來源 `61ed5930`）：Chat `384bcb23a13bc0f4e876f29e3037f2181ebbc3d5`、Pet `7097f943da5c88be851979cec74641fa35e6e613`、Analytics `44abb16f1ec3f74fa181c69d46e8bd984a090f21`、Crates `c5ec8079981d59244ce5482258424cd8ff57f9a3`。已核對四份 workspace manifests/locks；不自動升級，不將 RC/nightly 視為穩定。native/WASM 與實際 MSRV 組合未執行，若使用者工具鏈落後須先查 `rustc --version`。


## ④ 已實際執行的驗證與剩餘缺口

精確來源：`5c713553daee4055738239f047d6c774fcc7a7a1`。使用 archive 解出的無 `.git` 隔離副本；原本兩個本地 repo 及產品遠端均未改動。下列命令於 `chat_apps` 執行，全部最終 exit 0。環境為 `CARGO_TARGET_DIR=/tmp/day36-verify-target`、`CARGO_BUILD_JOBS=2`、dev/test debug=0、`CARGO_INCREMENTAL=0`；沒有更新 lock、依賴或產品程式。

~~~bash
cargo metadata --locked --offline --format-version 1 --no-deps
cargo fmt --all -- --check
cargo test --locked --offline -p chat-server --features ssr day36_
cargo test --locked --offline -p chat-server --features ssr
cargo test --locked --offline -p chat-ui --features ssr
cargo check --locked --offline -p chat-ui --features hydrate --target wasm32-unknown-unknown
cargo build --locked --offline -p chat-server --features ssr
~~~

Day36 兩項 helper 測試通過；完整 server **31 passed**、UI SSR **14 passed**，hydrate WASM check 通過。在另一份 `61ed5930` 只加入 Red，`socket_session_active` 缺符號 E0432／exit 101，符合預期；不是環境失敗。

額外執行 `python3 work/chat_ws_probe.py`，直接啟動本次編譯的 server，以隔離 SQLite 經真 HTTP 註冊／登入取得兩個不同帳號 Cookie，建立同房兩條真 WebSocket，實測五種情境全部通過：登出後閒置關閉、登出立即送訊息未寫入、Session 過期關閉、帳號停用關閉、廣播前撤銷檢查；有效 B 繼續使用，無效 A 不收到 B 廣播。到期／停用以隔離 DB 注入，登出走正式 HTTP。

**更正延遲承諾：**「每秒重查」不是硬性「≤1 秒」。從撤銷 DB commit／HTTP logout 回應完成後量測至 Close：閒置 0.9756s、立即送訊息 0.0008s、過期 1.0006s、停用 0.9916s、廣播 0.0421s。僅每情境單次探測，未測負載；DB/socket I/O 沒有硬性上界。最初含登出往返的時間不作純撤銷延遲證據。

**尚未驗收：** Session 檢查與 message INSERT 不在同一交易，登出競態仍存在；逐房 ACL、作者／歷史訊息、負載下撤銷延遲及完整瀏覽器 UI 未驗。新 Day36 單測本身只驗 helper，真 WS 證據來自本次額外腳本。

原始錯誤如實保留：首次 native 編譯遇暫存配額 os error 122，調整本次自建快取後重跑成功。Chat 探測曾誤用同帳號作控制組，Analytics 探測初版誤用 /health；修正後的成功 log 才作結論，原失敗另存證據包。獨立 PG 與 server 均已停止，本次快取已清理。

**判定：程式在上述精確 main 已存在，測試已執行；完整產品仍未驗收，不封存、不推進 Day、不更新 last_full_comparison。**
