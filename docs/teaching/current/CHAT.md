# 2026-10-17｜P0 Chat Day 15｜首頁房間列表與選房流程

來源文章：[Notion 原頁](https://app.notion.com/p/3fbe51d450938130a3c0c93171105f3a)。

## 1. 學習大綱
主體 Red 5 分＋Green 15 分。使用者從首頁讀取 SQLite 房間列表、選房進入現有聊天室；保留建立房間功能。API 與畫面是同一主功能的必要接線，不各算新功能。登入／ACL 尚缺，這是本機學習流程；正式私人房間須先授權過濾。
來源：rust_ai_chat/main `3e58e650bd63bcd21ba58e7aabf751542401ef80`；2026-10-17 重新讀取 main、相關 source／呼叫端及 CI，六個 workflow 均 success。這是既有來源 CI，不能驗證下列教學增量。copilot_use 教學分支基準 `adad4f26964b2518f47a9fde76544b544cd44334`；產品 `9f632c76ee432d77c1d20f46da9485d982d49c9e` 僅沿用舊靜態證據，本次未重新驗收產品。修訂版 Day15-r2：沿用原頁，原頁標記未套用且最新 main 缺少本次入口，故重整未實作內容。程式未套用、測試未執行、GUI／流程未驗收。
靜態現況：`crates/ui/src/lib.rs::HomePage` 只有 create_room；`room_api.rs` 只有 POST；server 的 GET 僅支援單房間。既有 Day14 不改寫。
## 2. 實作內容
### Red
從首頁嘗試「讀取列表 → 選既有房間 → 進房」，目前無列表入口，屬預期手動 UI Red，尚未執行。另在 `apps/server/src/http.rs::room_details_tests` 新增：
```rust
#[tokio::test]
async fn list_rooms_returns_persisted_room() {
    let (state, pool) = fixture().await;
    let response = list_rooms(State(state)).await.into_response();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), 8192).await.unwrap();
    let rooms: Vec<RoomDetails> = serde_json::from_slice(&body).unwrap();
    assert_eq!(rooms.len(), 1);
    assert_eq!(rooms[0].id, "room-14");
    pool.close().await;
}
```
預期新增 list_rooms 前編譯失敗；不能把既有 GET 單房間的測試當成本日 Red。
### 最小 Green
**檔案 A** `crates/chat_application/src/repository.rs`：在既有 `RoomRepository` trait 內新增：
```rust
async fn list(&self) -> Result<Vec<Room>, RepositoryError>;
```
所有實作此 trait 的既有測試替身（`crates/chat_application/src/lib.rs` 的 `ExistingRoom`、`MissingRoom`、`WrongRoom`）都需各加入：
```rust
async fn list(&self) -> Result<Vec<Room>, RepositoryError> {
    Ok(Vec::new())
}
```
**檔案 B** `crates/persistence/src/lib.rs`：在既有 `impl RoomRepository for SqliteRoomRepository` 中新增：
```rust
async fn list(&self) -> Result<Vec<Room>, RepositoryError> {
    sqlx::query_as::<_, RoomRow>(
        "SELECT id, category_room_id, name FROM rooms ORDER BY name, id"
    )
    .fetch_all(&self.pool)
    .await
    .map(|rows| rows.into_iter().map(Room::from).collect())
    .map_err(|e| RepositoryError::RoomAccessError(e.to_string()))
}
```
**檔案 C** `apps/server/src/http.rs`：在 `get_room` 旁新增：
```rust
pub async fn list_rooms(
    State(state): State<AppState>,
) -> Result<Json<Vec<RoomDetails>>, StatusCode> {
    let rooms = state.room_repository.list().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(rooms.into_iter().map(|room| RoomDetails {
        id: room.id,
        category_room_id: room.category_room_id,
        name: room.name,
    }).collect()))
}
```
**檔案 D** `apps/server/src/lib.rs`：把原本 `.route("/api/rooms", post(http::create_room))` **替換**為：
```rust
.route("/api/rooms", get(http::list_rooms).post(http::create_room))
```
呼叫鏈：HTTP GET → AppState → RoomRepository::list → SqliteRoomRepository → SQLite → JSON。**安全界線：目前參考庫沒有登入授權，這是學習 API，不可直接對外公開私人房間；copilot_use 保留既有 Session/ACL，正式產品必須先依使用者過濾。**
**前台接線 E**：在 `crates/ui/src/room_api.rs` 末尾新增：
```rust
// 接續既有 create_room；不重建另一個 HTTP client。
pub async fn list_rooms() -> Result<Vec<(String, String)>, String> {
    let window = web_sys::window().ok_or("無法取得瀏覽器")?;
    let value = JsFuture::from(window.fetch_with_str("/api/rooms"))
        .await.map_err(|_| "房間讀取失敗")?;
    let response: web_sys::Response = value.dyn_into().map_err(|_| "無效回應")?;
    if !response.ok() { return Err(format!("讀取失敗：{}", response.status())); }
    let text = JsFuture::from(response.text().map_err(|_| "無法讀取回應")?)
        .await.map_err(|_| "無法讀取回應")?
        .as_string().ok_or("無效文字")?;
    let rows: Vec<serde_json::Value> = serde_json::from_str(&text)
        .map_err(|_| "房間格式錯誤")?;
    rows.into_iter().map(|row| {
        let id = row["id"].as_str().ok_or("缺少房間 ID")?;
        let name = row["name"].as_str().ok_or("缺少房間名稱")?;
        // 此學習入口沿用現有 UUID／fixture ID，不把任意字串當 URL。
        if id.is_empty() || !id.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_') {
            return Err("房間 ID 不適用目前入口".into());
        }
        Ok((id.to_owned(), name.to_owned()))
    }).collect()
}
```
**前台接線 F**：在 `crates/ui/src/lib.rs` 的 HomePage view 歡迎文字後加入 `<RoomList />`，其他建房／聊天程式保留。新增完整元件：
```rust
#[component]
fn RoomList() -> impl IntoView {
    let (rows, set_rows) = signal(Vec::<(String, String)>::new());
    let (loading, set_loading) = signal(false);
    let (status, set_status) = signal(String::from("請讀取房間列表"));
    view! {
        <section>
            <h2>"房間列表"</h2>
            <button type="button" disabled=move || loading.get() on:click=move |_| {
                if loading.get_untracked() { return; }
                set_loading.set(true);
                set_rows.set(Vec::new());
                set_status.set("載入中…".into());
                #[cfg(target_arch = "wasm32")]
                wasm_bindgen_futures::spawn_local(async move {
                    match room_api::list_rooms().await {
                        Ok(value) => {
                            set_status.set(if value.is_empty() { "尚無房間" } else { "請選擇房間" }.into());
                            set_rows.set(value);
                        }
                        Err(message) => set_status.set(message),
                    }
                    set_loading.set(false);
                });
                #[cfg(not(target_arch = "wasm32"))]
                { set_loading.set(false); }
            }>"讀取／重新整理房間"</button>
            <p role="status">{move || status.get()}</p>
            <ul>{move || rows.get().into_iter().map(|(id, name)| view! {
                <li><a href=format!("/room/{id}")>{name}</a></li>
            }).collect_view()}</ul>
        </section>
    }
}
```
完整呼叫鏈：首頁按鈕 → room_api::list_rooms → GET /api/rooms → RoomRepository::list → SQLite → Leptos 列表 → /room/\{id\} → 既有 RoomPage／ChatPage／WebSocket。未修改其他檔案 N/A；沒有重做平行 Port。
### Refactor
共用 RoomDetails 後端 DTO；前台暫用現有 serde_json，待契約穩定再抽共享 DTO。保留所有 trait 替身及舊測試覆蓋，未搬完根 tests 不刪除 target／CI。既有聊天歷史、登入／ACL、重新連線未在本日宣稱完成。
### 舊 Day 更正補充
Day14 已實作，原文封存不回填。本次只原頁重整未套用 Day15：將原 API 切片接到首頁與選房流程，沒有新增第二個 Day15。
## 3. 套件版本
沿用來源 workspace axum／sqlx／tokio、ui 的 Leptos 0.8、leptos_router、serde_json、web-sys 0.3、wasm-bindgen 0.2；不新增依賴。Cargo 宣告已讀，lock／官方最新 stable／MSRV／安全公告未重新核實，不宣稱符合最新版；native／WASM 未編譯。
## 4. 驗證項目
待執行：`cargo test --manifest-path apps/server/Cargo.toml list_rooms_returns_persisted_room`；`cargo test --workspace --locked`；`cargo check -p ui --features ssr`；`cargo check -p ui --features hydrate --target wasm32-unknown-unknown`；`cargo fmt --all -- --check`。
GUI 從真實首頁驗證載入、空列表、讀取失敗、重試、選房、既有建房後重返列表與 SQLite 重啟保留。HTTP 確認 GET 200／DB 故障 500／POST 201；以兩個瀏覽器驗證所選房間 WS。上述命令、手動 Red／Green、DB 與 GUI 全部未執行。既有 CI success 只適用來源 SHA；不得標示本次增量通過。
