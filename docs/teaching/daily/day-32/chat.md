# Chat Day32 r1｜聊天室重新整理時明確揭露歷史尚未載入

## ① 學習大綱：功能、現況、前置條件

**今日最小切片**：進入已授權聊天室後，畫面必須如實告知「目前只顯示本次連線收到的訊息；重新整理不會載入 SQLite 歷史」。當訊息陣列為空，呈現可辨識的空狀態；收到 WebSocket 訊息後自動移除空狀態。這是直接可見的 Leptos 聊天頁行為，不以 helper 或 API 回傳成功冒充完整歷史功能。

來源分工：文件 `williechen/copilot_use/main`，教學原始碼唯讀 `williechen/rust_ai_chat/main@be0494a2a70be91eb74b2b726970a23fc3e5c044`。本次規則 `daily-tdd-minimum-v1 + github-only-v1`；以 `next_day.chat=32` 接續 Day31 個人頁建房。現行 `crates/ui/src/lib.rs::ChatPage` 以空 `Vec<ChatMessage>` 起始，僅在 `ServerEvent::MessageCreated` 推入新訊息；`crates/persistence/src/lib.rs` 的 `SqliteMessageRepository` 已可寫入，但沒有供本頁讀回歷史的查詢接線。來源 SSR 測試 `room_page_has_real_room_identity_and_multiline_composer` 要求「歷史載入尚未接通」，目前 UI 沒有該字樣；正式來源驗證報告亦記錄 SSR 10 通過／1 失敗。這項既有失敗是選題證據，不假稱本日新增 Red 已跑過。

真實呼叫鏈：登入／Session → Axum `/room/{room_id}` 的 `require_member` → `ssr::app_handler` → Leptos `RoomPage → ChatPage` → WASM `ChatSocket` → Axum `/ws/{room_id}` → `chat_application::persist_in_existing_room` → SQLx SQLite → `MessageCreated`。本日只更正 UI 對現有資料流的呈現；**沒有**新增歷史 API、沒有宣稱逐房 ACL、歷史查詢或作者身分已完成。使用者自行在 rust_ai_chat 工作副本套用；不得將本文程式碼寫入 copilot_use 的產品程式。

## ② 實作內容：Red → 完整 Green → Refactor

### Red：在真實 Leptos SSR 頁固定可觀察契約

在 `crates/ui/src/lib.rs` 末端加入下列**新**測試；不要刪改既有 `day16_layout_tests`。它使用實際 `ChatPage` 的 SSR 輸出，先要求具體空狀態及清楚的資料範圍提示。來源未包含這兩段文字，預期為**斷言失敗**，不是編譯／匯入失敗；本次沒有執行。

```rust
#[cfg(all(test, feature = "ssr"))]
mod day32_history_disclosure_tests {
    use super::*;
    #[test]
    fn room_explicitly_discloses_missing_history_and_empty_live_messages() {
        let owner = Owner::new();
        let html = owner.with(|| {
            view! { <ChatPage room_id="day32-room".to_owned() /> }.to_html()
        });
        assert!(html.contains("day32-room"));
        assert!(html.contains("歷史載入尚未接通"));
        assert!(html.contains("重新整理不會載入已儲存訊息"));
        assert!(html.contains("目前尚無本次連線收到的訊息"));
        assert!(html.contains("aria-live=\"polite\""));
    }
}
```

行為 Red（需真瀏覽器重現）：登入→個人頁建立房間→進房→發送一則訊息→重新整理。SQLite 中仍可有紀錄，但原畫面既不載入舊訊息也不說明原因，容易誤認訊息已遺失。請分開記錄「DB 有資料」和「畫面沒有歷史」，不要以瀏覽器空白判定資料庫遺失。

### Green：只替換真實 `ChatPage` 的訊息區

在 `crates/ui/src/lib.rs::ChatPage` 的 `view! { <section class="conversation"> ... }` 中，**完整替換原本的 `<ul class="messages" ...>...</ul>` 區塊**為下列內容；其餘 `draft`、`connection`、`messages` signals、WebSocket `MessageCreated` callback、`send` closure、`textarea` 與送出按鈕**原樣保留**。因此無新依賴、無 SQL migration、無平行元件，也不破壞現有真實送訊息鏈。

```rust
            <p class="history-disclosure" role="note">
                "歷史載入尚未接通：目前只顯示本次連線收到的訊息；重新整理不會載入已儲存訊息。"
            </p>
            <div class="message-history" aria-live="polite">
                {move || {
                    if messages.get().is_empty() {
                        view! {
                            <p class="messages-empty">
                                "目前尚無本次連線收到的訊息"
                            </p>
                        }.into_any()
                    } else {
                        view! {
                            <ul class="messages">
                                {messages.get().into_iter().map(|message| view! {
                                    <li>
                                        <strong>{author_label(&message.author)}": "</strong>
                                        {message.content}
                                    </li>
                                }).collect_view()}
                            </ul>
                        }.into_any()
                    }
                }}
            </div>
```

**為何是完整 Green**：初始 `messages=[]` 時可讀到提示與空狀態；既有 `ChatSocket` 收到 `MessageCreated` 後 `set_messages.update` 觸發 signal，空狀態消失並改以既有 `author_label` 與訊息內容顯示；換房仍使用既有 Router 和 WebSocket。提示永遠保留，直到日後真正實作歷史載入才以版本化新 Day 取代。此實作**不**保證 WS 成功回覆或 SQL 持久化，亦不會將匿名作者誤標成已登入本人。

### Refactor：責任、回歸與最小差異

保留 `ChatPage` 的 `messages` 單一訊息來源，不另外複製陣列；以 `aria-live="polite"` 讓新訊息和空狀態更新可被輔助技術辨識。若 `ChatPage` 未來改接正式歷史查詢，應由 Application/Repository 加入房間限定查詢及分頁，再由 Leptos 使用真實結果；**不可**把「歷史已載入」字樣直接替換，或用假資料通過測試。既有 Day16 的 SSR 測試預期 `歷史載入尚未接通`，此 Green 同時滿足，但不能把來源原本失敗冒稱今日測試已通過。

### 上一 Day 未完成事項補充

Day31 的「個人頁建房」已在最新來源中存在，最新正式驗證記錄真瀏覽器建立／進房／登出通過；其原文章的未套用記載屬當時快照。逐房 ACL、歷史、作者身分及既有 SSR 回歸缺口仍須依精確來源與實測分開追蹤，本日不重新發 Day31、不更新其正文或原始證據。

### 舊 Day 更正補充（FIFO）

`makeup_queue` 最早未結案 Chat 項為 `chat-day25-shared-auth-20261022`：共用登入、登出、Cookie 與 Session 仍須通過真實呼叫者核對。本日不把一個 UI 提示當成共用認證重構；保留 FIFO **pending**，未結案、未改 Day25。後續需要在實際呼叫鏈中使用共用儲存介面並測 Session 撤銷／帳號停用／登入後回跳；本日沒有完成或執行該補教。

## ③ 套件與軟體版本（2026-10-24，官方穩定版核對）

`rust_ai_chat/Cargo.toml` 宣告 Leptos／Router `0.8`、Axum `0.8`、SQLx `0.9`、Tokio `1.54`；`rust-toolchain.toml` 為 `stable`，未固定 patch。下表的「lock」來自來源 `Cargo.lock@1b637de69d62d6413631b11b3e1af2bc85caa40d`；官方版本連結為當日核對依據，預覽版／RC 不列為穩定版。

| 元件 | 官方最新穩定 | 宣告／lock 或安裝 | 判斷及官方來源 |
| --- | --- | --- | --- |
| Rust／Cargo | **1.99.1**（2026-10-15） | toolchain=`stable`，本次未取得實際安裝版 | Rust 2024 與既有 0.8／0.9 系列需按原 lock 重跑；[Rust 官方發行](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) |
| Leptos | **0.8.22** | `0.8`／0.8.22 | 同主版，保持 SSR／hydrate 配對；[官方 crate](https://docs.rs/crate/leptos/latest) |
| leptos_router | **0.8.17** | `0.8`／0.8.17 | 不採 0.9 RC；[官方 crate](https://docs.rs/crate/leptos_router/latest) |
| leptos_axum／leptos_config | **0.8.10／0.8.10** | `0.8`／0.8.10、0.8.10 | 與 Leptos 0.8 同系；[Axum 橋接](https://docs.rs/crate/leptos_axum/latest)、[設定](https://docs.rs/crate/leptos_config/latest) |
| Axum | **0.8.9** | `0.8`／0.8.9 | 不改現有 Router／Session；[官方 crate](https://docs.rs/crate/axum/latest) |
| SQLx | **0.9.0** | `0.9`／0.9.0 | 不變更 SQLite adapter；[官方 crate](https://docs.rs/crate/sqlx/latest) |
| Tokio | **1.54.1** | `1.54`／1.54.1 | 既有 async runtime；[官方 crate](https://docs.rs/crate/tokio/latest) |
| SQLite CLI／engine | **3.54.1**（2026-10-16） | CLI／執行時 SQLite 版本**未讀得** | SQLx 使用之 SQLite 版本不等於系統 CLI；套用前以 `sqlite3 --version`、`SELECT sqlite_version()` 分開確認；[SQLite 官方版本史](https://www.sqlite.org/chronology.html) |

本切片不需要新增或升級任何套件；以現有 lock 為建議基準。MSRV 依依賴實際 `rust-version`、WASM `wasm32-unknown-unknown` 與 native/SSR 分開建置確認；本次未取得本機編譯器、CLI 或 SQLite runtime 版本，**不得**視為已驗證相容。UI 沒有直接存取 DB，亦不需 PostgreSQL 或 Tauri。

## ④ 驗證項目：命令、預期、實際、缺口

1. **Red**：先新增測試但尚未套 Green，`cargo test -p ui --features ssr day32_history_disclosure_tests -- --nocapture`；預期為文字斷言失敗。**實際：未執行**（未修改唯讀 rust_ai_chat）。
2. **Green**：貼上完整訊息區後重跑上述測試，並執行 `cargo test -p ui --features ssr day16_layout_tests -- --nocapture`；預期兩者通過。**實際：未執行**。
3. **回歸**：`cargo test -p server --features ssr`、`cargo test --workspace`、`cargo build -p server --features ssr`、`cargo check -p ui --features hydrate --target wasm32-unknown-unknown`；預期無回歸，**實際：未執行**。
4. **真實操作／DB**：隔離 SQLite 註冊→登入→個人頁建立房間→進房，確認 SSR 提示與空狀態；送出訊息，確認 WS 收到後空狀態消失；SQL `SELECT room_id,content FROM messages WHERE room_id=?` 確認已寫入；重新整理後提示仍在，歷史仍不會載入。跨房／失效 Session／窄螢幕與螢幕閱讀器另測。**實際：未執行**。
5. **現有 CI 參考**：`rust_ai_chat@be0494a2a70be91eb74b2b726970a23fc3e5c044` 的 [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39977648774) 與 [Rust AI Chat](https://github.com/williechen/rust_ai_chat/actions/runs/39977648582) 於 2026-10-23 完成且 workflow 結論 success；**不涵蓋本篇尚未套用的 Green**。同 SHA 的正式報告指出 SSR 仍有 1 項失敗，不能由 workflow 成功推論所有 SSR／瀏覽器驗收通過。

**交付與驗收分離**：本文是可供使用者套用的完整教學，並非對 rust_ai_chat 的程式提交；程式套用＝未執行，Red／Green／DB／瀏覽器＝未執行，功能驗收＝未通過。未完成：歷史訊息查詢與重新載入、已登入作者身分、逐房 ACL、WebSocket 重連／撤銷、Day25 共用認證 FIFO，以及原有 SSR 回歸實測。下一切片應依最新來源重新選取最早尚未接通的真實使用者流程，不能因文件 Day 推進視為產品完成。
