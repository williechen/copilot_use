# 2026-10-17｜P0 Chat Day 16｜清楚選房與聊天

## 1. 學習大綱
修訂 r3，2026-10-17。以 `rust_ai_chat/main` `b2733b7f78dce1da1aa07555122384c27109e4e8` 為基底。**Red 是本日 Green 的前置測試；只有 Red 不算完成，也不會因此刪掉 Green 或改排下一 Day。**
來源已有舊版 Day16 Red；本篇補齊對應 Green 與主流程可套用程式，已在獨立教學草稿驗證，尚未套用參考庫。不是只提供計畫。Day15 原文與既有程式不改。
交付流程：首頁顯示真實房間清單 → 選房並看清目前房間 → 在訊息區閱讀本次連線收到的訊息 → 多行輸入並送出 → 收到伺服器回覆。基本畫面包含房間側欄、目前房間標題、訊息區與輸入區；小螢幕仍能選房及送訊。
既有基礎為 `GET /api/rooms`、`room_api::list_rooms`、`/room/:room_id`、WebSocket。未提供的歷史訊息、時間、在線人數與個人身分不可用假資料補齊。名稱搜尋可附加，但不是本日主體。
## 2. 實作內容
### Red：先跑既有前置測試
以下是已套用的原測試，保留斷言：
```rust
#[cfg(test)]
mod day16_room_search_tests {
    use super::filter_rooms;

    #[test]
    fn search_matches_chinese_names_and_preserves_ids() {
        let rooms = vec![
            ("room_1".to_owned(), "Rust 技術交流".to_owned()),
            ("room_2".to_owned(), "生活閒聊".to_owned()),
        ];
        assert_eq!(filter_rooms(&rooms, " 技術 "), vec![rooms[0].clone()]);
        assert_eq!(filter_rooms(&rooms, ""), rooms);
        assert!(filter_rooms(&rooms, "不存在").is_empty());
    }
}
```
來源原樣重跑：缺少 `filter_rooms`，E0432，預期 Red。
### Green：完整程式與接線
房間側欄列出真實名稱並標記目前房間；選房採完整導頁，讓舊 WebSocket 隨頁面釋放。訊息區、多行輸入、連線狀態與失敗草稿接回既有協定，不製造歷史訊息、本人身分或時間資料。
以下補丁包含本專案所有必要修改，無省略函式；也提供每個變更檔的完整內容。套用前核對基底 SHA 並保留自己的修改，不能對不同版本盲貼。變更入口及呼叫鏈直接見補丁，未變更檔案沿用基底。
- [`crates/ui/Cargo.toml` 完整檔案](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/day16/green/crates/ui/Cargo.toml)
- [`crates/ui/src/lib.rs` 完整檔案](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/day16/green/crates/ui/src/lib.rs)
- [`crates/ui/src/chat_socket.rs` 完整檔案](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/day16/green/crates/ui/src/chat_socket.rs)
- [`crates/ui/src/chat.css` 完整檔案](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/day16/green/crates/ui/src/chat.css)
[下載完整 Green 補丁](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/day16/chat-green.patch)。將補丁存成 `chat-green.patch`，在參考庫根目錄操作：
```sh
git apply --check --ignore-whitespace chat-green.patch
git apply --ignore-whitespace chat-green.patch
```
以下為同一份可套用完整補丁：
```diff
diff --git a/crates/ui/Cargo.toml b/crates/ui/Cargo.toml
--- a/crates/ui/Cargo.toml
+++ b/crates/ui/Cargo.toml
@@ -32,4 +32,6 @@
     "Response",
     "WebSocket",
     "Window",
+    "Location",
+    "Event",
 ] }
diff --git a/crates/ui/src/lib.rs b/crates/ui/src/lib.rs
--- a/crates/ui/src/lib.rs
+++ b/crates/ui/src/lib.rs
@@ -25,7 +25,7 @@
 
 fn author_label(author: &MessageAuthor) -> String {
     match author {
-        MessageAuthor::AnonymousUser => "You".to_string(),
+        MessageAuthor::AnonymousUser => "匿名使用者".to_string(),
         MessageAuthor::User { user_id } => format!("User {}", user_id),
         MessageAuthor::Ai { persona_id } => format!("AI {}", persona_id),
         MessageAuthor::System => "System".to_string(),
@@ -41,6 +41,7 @@
                 <meta name="viewport" content="width=device-width, initial-scale=1"/>
                 <title>"Rust AI Chat"</title>
                 <HydrationScripts options />
+                <style>{include_str!("chat.css")}</style>
             </head>
             <body>
                 <App />
@@ -59,7 +60,7 @@
 pub fn App() -> impl IntoView {
     view! {
         <Router>
-            <main>
+            <main class="app">
                 <h1>"Rust AI Chat"</h1>
                 <Routes fallback=|| view!{
                     <p>"404 - Page Not Found"</p>
@@ -134,7 +135,9 @@
         let room_id = params.read().get("room_id");
 
         match room_id {
-            Some(room_id) => view! { <ChatPage room_id /> }.into_any(),
+            Some(room_id) => {
+                view! { <div class="chat-layout"><RoomList /><ChatPage room_id /></div> }.into_any()
+            }
             None => view! { <p>"Invalid Room Id"</p> }.into_any(),
         }
     }
@@ -144,6 +147,8 @@
 pub fn ChatPage(room_id: String) -> impl IntoView {
     let (draft, set_draft) = signal(String::new());
 
+    let (connection, set_connection) = signal(String::from("尚未連線"));
+    let heading = room_id.clone();
     let (messages, set_messages) = signal(Vec::<ChatMessage>::new());
 
     #[cfg(not(target_arch = "wasm32"))]
@@ -151,22 +156,37 @@
 
     #[cfg(target_arch = "wasm32")]
     let socket = {
-        let url = format!("ws://localhost:3000/ws/{room_id}");
-        Rc::new(
-            ChatSocket::connect(&url, move |event| {
+        let location = web_sys::window().unwrap().location();
+        let protocol = if location.protocol().unwrap_or_default() == "https:" {
+            "wss"
+        } else {
+            "ws"
+        };
+        let url = format!(
+            "{protocol}://{}/ws/{room_id}",
+            location.host().unwrap_or_default()
+        );
+        match ChatSocket::connect(
+            &url,
+            move |event| {
                 if let ServerEvent::MessageCreated(message) = event {
                     set_messages.update(|messages| messages.push(message));
                 }
-            })
-            .expect("Failed to connect to WebSocket"),
-        )
+            },
+            move |status| set_connection.set(status.into()),
+        ) {
+            Ok(socket) => Some(Rc::new(socket)),
+            Err(_) => {
+                set_connection.set("連線失敗，請重新進房".into());
+                None
+            }
+        }
     };
-
     #[cfg(not(target_arch = "wasm32"))]
-    let socket = Rc::new(ChatSocket::server_stub());
+    let socket = Some(Rc::new(ChatSocket::server_stub()));
 
     let send = {
-        let socket = Rc::clone(&socket);
+        let socket = socket.clone();
 
         move |_| {
             let room_id = room_id.clone();
@@ -179,16 +199,25 @@
 
             let event = ClientEvent::SendMessage { room_id, content };
 
-            if socket.send(&event).is_ok() {
-                set_draft.set(String::new());
+            match socket
+                .as_ref()
+                .ok_or_else(|| "尚未連線".to_owned())
+                .and_then(|s| s.send(&event))
+            {
+                Ok(()) => {
+                    set_draft.set(String::new());
+                    set_connection.set("已送至連線，等待伺服器回覆".into());
+                }
+                Err(_) => set_connection.set("傳送失敗，草稿保留；請重新進房".into()),
             }
         }
     };
 
     view! {
-        <section>
-            <h2>"Room"</h2>
-            <ul>
+        <section class="conversation">
+            <header><h2>{format!("房間 {heading}")}</h2><p role="status">{move || connection.get()}</p></header>
+            <p>"顯示本次連線收到的訊息；歷史載入尚未接通。"</p>
+            <ul class="messages" aria-live="polite">
                 {move || messages.get().into_iter().map(|message|{
                     view! {
                         <li>
@@ -200,14 +229,14 @@
                     }}).collect_view()}
             </ul>
             <div>
-                <input
-                    type="text"
+                <textarea
+                    aria-label="聊天訊息"
                     placeholder="Type a message..."
                     prop:value=move || draft.get()
                     on:input=move |event| set_draft.set(event_target_value(&event))
-                />
-
-                <button type="button" on:click=send>"Send"</button>
+                ></textarea>
+
+                <button type="button" disabled=move || draft.get().trim().is_empty() on:click=send>"Send"</button>
             </div>
         </section>
     }
@@ -215,12 +244,16 @@
 
 #[component]
 fn RoomList() -> impl IntoView {
+    let params = use_params_map();
+    let (query, set_query) = signal(String::new());
     let (rows, set_rows) = signal(Vec::<(String, String)>::new());
     let (loading, set_loading) = signal(false);
     let (status, set_status) = signal(String::from("請讀取房間列表"));
     view! {
         <section>
             <h2>"房間列表"</h2>
+            <a href="/" rel="external">"首頁／建立房間"</a>
+            <input type="search" aria-label="搜尋房間" placeholder="房間名稱" prop:value=move || query.get() on:input=move |ev| set_query.set(event_target_value(&ev)) />
             <button type="button" disabled=move || loading.get() on:click=move |_| {
                 if loading.get_untracked() { return; }
                 set_loading.set(true);
@@ -241,8 +274,8 @@
                 { set_loading.set(false); }
             }>"讀取／重新整理房間"</button>
             <p role="status">{move || status.get()}</p>
-            <ul>{move || rows.get().into_iter().map(|(id, name)| view! {
-                <li><a href=format!("/room/{id}")>{name}</a></li>
+            <ul>{move || filter_rooms(&rows.get(), &query.get()).into_iter().map(|(id, name)| view! {
+                <li><a rel="external" aria-current=if params.read().get("room_id").as_deref()==Some(id.as_str()) { Some("page") } else { None } href=format!("/room/{id}")>{name}</a></li>
             }).collect_view()}</ul>
         </section>
     }
@@ -263,3 +296,27 @@
         assert!(filter_rooms(&rooms, "不存在").is_empty());
     }
 }
+
+fn filter_rooms(rows: &[(String, String)], query: &str) -> Vec<(String, String)> {
+    let needle = query.trim().to_lowercase();
+    rows.iter()
+        .filter(|(_, name)| name.to_lowercase().contains(&needle))
+        .cloned()
+        .collect()
+}
+
+#[cfg(all(test, feature = "ssr"))]
+mod day16_layout_tests {
+    use super::*;
+    #[test]
+    fn room_page_has_real_room_identity_and_multiline_composer() {
+        let owner = Owner::new();
+        let html =
+            owner.with(|| view! { <ChatPage room_id="room-fixture".to_owned() /> }.to_html());
+        assert!(html.contains("room-fixture"));
+        assert!(html.contains("<textarea"));
+        assert!(html.contains("尚未連線"));
+        assert!(html.contains("歷史載入尚未接通"));
+        assert!(!html.contains("Online: 100"));
+    }
+}
diff --git a/crates/ui/src/chat_socket.rs b/crates/ui/src/chat_socket.rs
--- a/crates/ui/src/chat_socket.rs
+++ b/crates/ui/src/chat_socket.rs
@@ -11,19 +11,23 @@
 #[cfg(target_arch = "wasm32")]
 mod browser {
     use wasm_bindgen::{JsCast, closure::Closure};
-    use web_sys::{MessageEvent, WebSocket};
+    use web_sys::{Event, MessageEvent, WebSocket};
 
     use super::*;
 
     pub struct ChatSocket {
         socket: WebSocket,
         _onmessage: Closure<dyn FnMut(MessageEvent)>,
+        _onopen: Closure<dyn FnMut(Event)>,
+        _onclose: Closure<dyn FnMut(Event)>,
+        _onerror: Closure<dyn FnMut(Event)>,
     }
 
     impl ChatSocket {
-        pub fn connect<F>(url: &str, mut on_event: F) -> Result<Self, String>
+        pub fn connect<F, S>(url: &str, mut on_event: F, on_status: S) -> Result<Self, String>
         where
             F: 'static + FnMut(ServerEvent),
+            S: 'static + Fn(&'static str),
         {
             let socket = WebSocket::new(url)
                 .map_err(|err| format!("Failed to create WebSocket: {:?}", err))?;
@@ -31,28 +35,47 @@
             let onmessage =
                 Closure::wrap(
                     Box::new(move |event: MessageEvent| match event.data().as_string() {
-                        Some(text) => {
-                            web_sys::console::error_1(&format!("Received text: {}", text).into());
-                            match decode_server_event(&text) {
-                                Ok(event) => on_event(event),
-                                Err(err) => web_sys::console::error_1(
-                                    &format!("Failed to decode server event: {:?}", err).into(),
-                                ),
-                            }
-                        }
+                        Some(text) => match decode_server_event(&text) {
+                            Ok(event) => on_event(event),
+                            Err(err) => web_sys::console::error_1(
+                                &format!("Failed to decode server event: {:?}", err).into(),
+                            ),
+                        },
                         None => return,
                     }) as Box<dyn FnMut(MessageEvent)>,
                 );
 
+            let status = std::rc::Rc::new(on_status);
+            status("連線中…");
+            let opened = status.clone();
+            let closed = status.clone();
+            let failed = status.clone();
+            let onopen =
+                Closure::wrap(Box::new(move |_: Event| opened("已連線")) as Box<dyn FnMut(Event)>);
+            let onclose =
+                Closure::wrap(Box::new(move |_: Event| closed("連線已關閉，請重新進房"))
+                    as Box<dyn FnMut(Event)>);
+            let onerror =
+                Closure::wrap(Box::new(move |_: Event| failed("連線錯誤，請重新進房"))
+                    as Box<dyn FnMut(Event)>);
+            socket.set_onopen(Some(onopen.as_ref().unchecked_ref()));
+            socket.set_onclose(Some(onclose.as_ref().unchecked_ref()));
+            socket.set_onerror(Some(onerror.as_ref().unchecked_ref()));
             socket.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
 
             Ok(Self {
                 socket,
                 _onmessage: onmessage,
+                _onopen: onopen,
+                _onclose: onclose,
+                _onerror: onerror,
             })
         }
 
         pub fn send(&self, event: &ClientEvent) -> Result<(), String> {
+            if self.socket.ready_state() != WebSocket::OPEN {
+                return Err("尚未連線".into());
+            }
             let json = serde_json::to_string(event)
                 .map_err(|err| format!("Failed to serialize event: {:?}", err))?;
             self.socket
@@ -64,6 +87,9 @@
     impl Drop for ChatSocket {
         fn drop(&mut self) {
             self.socket.set_onmessage(None);
+            self.socket.set_onopen(None);
+            self.socket.set_onclose(None);
+            self.socket.set_onerror(None);
             let _ = self.socket.close();
         }
     }
diff --git a/crates/ui/src/chat.css b/crates/ui/src/chat.css
new file mode 100644
--- /dev/null
+++ b/crates/ui/src/chat.css
@@ -0,0 +1,2 @@
+*{box-sizing:border-box}body{margin:0;background:#eef2f6;color:#233044;font:16px system-ui}.app{max-width:1200px;margin:auto;padding:20px}button,input,textarea{font:inherit;padding:10px;margin:6px 0}button{cursor:pointer}a{color:#185ca5}.chat-layout{display:grid;grid-template-columns:260px minmax(0,1fr);gap:20px}.chat-layout>section{background:white;padding:18px;border-radius:14px}.chat-layout>section:first-child ul{padding:0;list-style:none}.chat-layout a{display:block;padding:8px;overflow-wrap:anywhere}.conversation{min-width:0}.conversation header{border-bottom:1px solid #ddd}.messages{min-height:180px;max-height:48vh;overflow:auto;list-style:none;padding:0}.messages li{margin:12px 0;padding:12px;background:#edf3fa;border-radius:12px;white-space:pre-wrap;overflow-wrap:anywhere}.messages strong{display:block}textarea{width:100%;min-height:90px;resize:vertical}@media(max-width:700px){.chat-layout{grid-template-columns:1fr}.app{padding:10px}}
+a[aria-current="page"]{background:#dbe9fc;font-weight:700;border-radius:8px}
```

### Refactor：根目錄 tests 遷移，保留原斷言
`tests/chat/persist_publish.rs` → `crates/chat_application/tests/persist_publish.rs`；`tests/chat/product_event_contract.rs` → `crates/chat_application/tests/product_event_contract.rs`。既有 `apps/server/tests/root_hub_contract.rs` 保留。

搬移前，以 `tests/Cargo.toml` 的 target、測試名稱與數量建立基準；搬移時保留原斷言，調整目的 package 的 dev-dependencies，再移除原 `[[test]]` 與不再使用的相依。不要拆分原始碼內嵌單元測試。同步 `.github/workflows/test.yml`：目的測試須在具備原 DB／環境設定的 job 執行；不得將需要 DB 的案例直接丟給未配置 DB 的 job。其他根目錄批次尚未完成時保留 `workspace-tests` 與 CI；全部驗證後才移除根目錄測試容器、workspace member 與空目錄，更新 lockfile。測試名稱／數量不得無故減少或重複，既有失敗與功能變更另記，不在純搬移中偷改斷言。


完整 [根目錄 tests 遷移補丁](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/day16/root-tests-refactor.patch) 包含 8 份原測試搬移、空 target 清理、Cargo／lock／CI 更新；[逐檔 SHA256 與案例清單](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/day16/test-relocation-evidence.json) 證明 38 個案例原文不變，不拆原始檔內嵌測試。
**此共同補丁要在三份 Green 全部套用後才執行一次**；不要只搬單一專案就先刪掉其餘 workspace-tests。
```sh
git apply --check --ignore-whitespace root-tests-refactor.patch
git apply --ignore-whitespace root-tests-refactor.patch
cargo test --locked -p chat-application -p pet-domain -p stats-server -p desktop-pet-frontend
```
CI 的相關整合測試保留於有 PostgreSQL 與 fixture 環境的 job，移除會缺 DB 的重複 stats-server job；新增前台單元測試執行。真實 DB 專項需另行明確啟用，不用 ignored 冒充已驗收。
### 舊 Day 更正補充
Day15 保持封存。舊版 Day16 的搜尋／取消／總數 Red 保留，Green 已補回並配合主流程；全文與程式不再換成只有工作清單。修訂前文字可由 Git 歷史追溯。Notion 舊 Day16 ID 目前 NOT_FOUND、還原工具也回 target_not_found，因此按本次「重建 Day16」要求，在各原專案入口下重建同一 Day 的唯一現行頁並更新索引；不新增 Day17、不提高 next_day。
## 3. 套件版本
Leptos 0.8.22、leptos_router 0.8.17、web-sys 0.3.106。
以上為本輪實際 Cargo.lock 解析版本，workspace 0.2.15／edition 2024；native／WASM 檢查如第 4 節。沒有宣稱官方最新 stable、MSRV 或安全公告已重新核實。全檔案與 lock 的一致性見 [程式清單](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/day16/code-manifest.json)。
## 4. 驗證項目
### 已執行結果
`cargo test -p ui --features ssr --lib`：3 passed；SSR check 與 WASM hydrate check：passed。
三專案根目錄搬移與相關回歸合計 72 passed、0 failed，另有 1 個 DB 專項預設 ignored；該專項已另外明確執行並通過，不能與預設批次混算。完整四份補丁在乾淨基底的 39 個路徑已實際套用並比對完整內容。
```sh
cargo test -p ui --features ssr --lib
cargo check -p ui --features ssr
cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features hydrate
```
詳細結果與日誌：[驗證紀錄](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/day16/validation.json)、[日誌](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/day16/logs/)。
### 待執行的人工流程驗收
尚未執行新畫面的瀏覽器／雙瀏覽器 E2E：開首頁→讀列表→選房→確認選中標記及 room ID→兩個瀏覽器同房收發→換房不串訊→多行／長字串→斷線送訊保留草稿→窄螢幕。畫面只顯示本次連線訊息，沒有假歷史。
### 來源 CI 與套用狀態
[來源 Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39005304871) 的最新基底為 16 success／2 failure：`test-ui`、`test-stats-server`。本輪以本機 Red 重現相應缺實作／匯入錯誤，再驗證教學 Green；參考庫未套用，遠端 CI 未被本輪修復。**教學 Green 可用與本機檢查通過，不等於參考庫已 Green，也不等於上述人工 GUI 已驗收。**
