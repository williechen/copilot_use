# 2026-10-15｜P0 Chat Day 12｜WebSocket 房間 Typing 即時通知

來源文章：[2026-10-15｜P0 Chat Day 12｜WebSocket 房間 Typing 即時通知](https://app.notion.com/p/3fae51d45093812bbf88c3e7f51d6522?pvs=204)。

## 1. 學習大綱
- P0 Chat Day 12（約 20 分鐘）：從最新 main 的 `ClientEvent::Typing` 已可解析、`handle_client_event` 卻直接忽略的真實缺口，完成**同房間即時 typing 狀態廣播**，不建立平行 Hub／Port。
- 來源：`williechen/rust_ai_chat@5a0a8cee0d1fb4d0cd248869d7b24ebb1d653399`；`apps/server/src/ws.rs`、`apps/server/src/hub.rs`、`crates/shared/src/lib.rs`。產品方向：`williechen/copilot_use` 教學分支 `docs/FUTURE_WORK.md` 的 Chat presence／typing／room isolation。上次 Day 11 的寫入時房間存在性核對已在 source，不能重做。
- 本切片是**匿名連線層的房間 typing 提示**；目前沒有 session/ACL/author identity，不宣稱已完成登入身分綁定或 presence。此事件不入 SQLite，不能冒稱 durable Outbox。
## 2. 實作內容
### Red：先寫會失敗的契約測試
在 `crates/shared/src/lib.rs` 原 `tests` 內追加：
```rust
#[test]
fn typing_changed_roundtrip() {
    let event = ServerEvent::TypingChanged {
        room_id: "room-a".into(),
        active: true,
    };
    let json = serde_json::to_string(&event).unwrap();
    assert_eq!(
        serde_json::from_str::<ServerEvent>(&json).unwrap(),
        event
    );
}
```
在 `apps/server/src/ws.rs` 原 `mod test` 內追加：
```rust
#[test]
fn typing_is_only_forwarded_to_connection_room() {
    use super::typing_event_for_room;
    assert!(typing_event_for_room("room-a", "room-b", true).is_none());
    assert!(typing_event_for_room("room-a", " ", true).is_none());
    assert_eq!(
        typing_event_for_room("room-a", "room-a", false),
        Some(shared::ServerEvent::TypingChanged {
            room_id: "room-a".into(),
            active: false,
        })
    );
}
```
預期 Red：`TypingChanged` 及 `typing_event_for_room` 尚不存在，編譯失敗。
### 最小 Green：擴充既有協定與 WS 呼叫端
`crates/shared/src/lib.rs` 的既有 `ServerEvent` enum 加入：
```rust
TypingChanged { room_id: String, active: bool },
```
`apps/server/src/ws.rs` 在 `room_matches_connection` 後新增：
```rust
fn typing_event_for_room(
    connection_room: &str,
    event_room: &str,
    active: bool,
) -> Option<ServerEvent> {
    room_matches_connection(connection_room, event_room).then(|| {
        ServerEvent::TypingChanged {
            room_id: connection_room.to_owned(),
            active,
        }
    })
}
```
在既有 `handle_client_event` 中**只替換**原本忽略 `Typing` 的 match arm：
```rust
ClientEvent::Typing { room_id, active } => {
    match typing_event_for_room(&event_room_id, &room_id, active) {
        Some(event) => {
            // 共用既有 RoomHub，不產生 SQLite 訊息紀錄。
            state.room_hub.publish(event_room_id, event);
        }
        None => {
            send_event(
                socket,
                ServerEvent::Error {
                    code: "room_mismatch".into(),
                    message: "typing room does not match connection room".into(),
                },
            )
            .await?;
        }
    }
}
```
既有 `RoomHub::subscribe` 在 `handle_socket` 內會將同房間事件傳給該房間訂閱者；沒有新增未使用 Adapter。
### Refactor 與安全界線
- 保留既有 `room_matches_connection`；只使用連線路徑中的 room ID 作為廣播目標，拒絕跨房間偽造。
- 當前 Hub 包含發送端訂閱者，UI 若不想顯示自己的 typing，須於後續具備可靠 connection identity 時處理；不可捏造 user ID。
- 本功能只提供短暫訊號，沒有 disconnect 清除、超時 TTL、session／ACL、跨程序 fan-out 或重連恢復；這些留在 `FUTURE_WORK.md` 原項目，不標完成。
### 實際套用補充（2026-10-15）
本地已提交 `a57e532c125f5f18d7fd2a250401efde0c9d2612`。Typing 協定、房間 guard、Hub 呼叫與本日兩個測試已套用；`apps/server/src/lib.rs` 另外將 SQLite 連線設定由 `DATABASE_SQLITE_URL` 改為 `CHAT_DATABASE_URL`。啟動時提供可建立檔案的 SQLite URI，例如 `sqlite://data/chat.sqlite3?mode=rwc`，並先建立父目錄。本次只使用隔離測試 DB。原教學程式保留，實際檔案見[精確提交](https://github.com/williechen/rust_ai_chat/tree/a57e532c125f5f18d7fd2a250401efde0c9d2612)；未新增 session／ACL／TTL／UI 顯示功能。
## 3. 套件版本
- `serde`／`serde_json`／`axum`／`tokio` 沿用 workspace 既有宣告，不增加依賴。
- 本次未重新核對 Cargo.lock 精確版本、官方 stable／MSRV；版本 Gate 標示**未查證**，不得猜測。
## 4. 驗證項目
- 建議：`cargo test -p shared typing_changed_roundtrip`、`cargo test -p server typing_is_only_forwarded_to_connection_room`、`cargo test -p server`、`cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets -- -D warnings`。
- Red 預期編譯失敗；Green 預期同房間廣播、跨房間拒絕、不新增 DB 寫入。另需實際兩個 WS client／兩個房間 E2E 驗證及 session／ACL 後續測試。
- 本 Day 程式與新增測試已套用；本地協定／server 測試、Release、真實 WS 廣播／房間隔離／不寫入訊息通過。GitHub CI 狀態另列如下。
- 狀態：既有 Day 12 已發布；程式已套用，本地切片功能驗證通過；最新 GitHub CI 已通過，整體正式驗收仍按適用範圍分開記錄。
### 來源驗證最終判定（2026-10-16）
**目前狀態：test-workspace 的 Analytics 測試環境缺口已修正，最新 GitHub CI 全部通過。** `rust_ai_chat/main@d05438950b96e03c9c6c6f8f2c7e3573cc191a25` 的 6／6 workflows、18／18 checks 均為 Success；[Test All／test-workspace 成功紀錄](https://github.com/williechen/rust_ai_chat/actions/runs/38765474606/job/116527415647)。最新 test.yml 已提供獨立 PostgreSQL、`ANALYTICS_DATABASE_URL` 與兩個合成來源 token。「缺少 Analytics 資料庫與 token」是舊提交 `a57e532` 的失敗原因，已不再是當前阻礙。
本地正式驗證仍精確對應 `a57e532c125f5f18d7fd2a250401efde0c9d2612`：107 項 Rust、3 項 Node、fmt、嚴格 Clippy、SSR／兩個 WASM、Linux 原生／Chat Leptos／Pet 前端 Release，以及 14 項實際 HTTP／WS／DB 檢查通過。已唯讀比對 `a57e532..d054389`，只有 `test.yml` 與 `persistence.yml` 改動，應用程式／測試／依賴鎖均未變；因此保留原本地證據，不冒稱已對新 SHA 重跑全部本地驗證。[精確來源、CI 與可攜式證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/source-review/rust-ai-chat-validation.json)。
Day 12 已由使用者套用；原撰寫基底 `5a0a8ce` 與全部原教學程式區塊保留。Analytics HTTP 已接 PostgreSQL，Typing／來源認證／持久冪等／併發／重啟／DB 故障測試通過。Pet 實際 GUI 移動、跨平台安裝包、gRPC／SSE 銜接及正式憑證安全審查仍未由本次驗證完成；不將 CI 全綠當作全部 Day／P9／週六或學習階段完成。未讀取私人 .env／AI key，未執行付費 AI，未寫入 rust_ai_chat。
