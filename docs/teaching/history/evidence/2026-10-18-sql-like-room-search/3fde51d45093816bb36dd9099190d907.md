## 1. 學習大綱
名稱搜尋依使用者更正採後端參數化SQL LIKE：參考庫候選 `4c1e5f710ade90a84dad83555a3e06ae6027ad77`（codex/sql-like-room-search，基底fa0888e／前一候選2f60c73）；copilot_use產品 `78d5796200547bf9c1776b8dc82cbc0d842bf512`（codex/p0-table-trial，延續948685f）。兩者均未合併main。參考庫相關測試34通過、真HTTP12案與瀏覽器三筆／空結果／503重試通過；產品相關測試22通過、真HTTP5案與公開／成員權限及先搜尋再LIMIT測試通過，兩庫嚴格Clippy通過。先前全量main126／1及舊候選129／0仍只適用原SHA，本次沒有重跑PG／Pet全量、沒有宣稱整體驗收。詳[SQL LIKE修正證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-sql-like-room-search/REPORT.md)。
目標仍為完整名稱包含搜尋切片；新增Query DTO與SQL接線，沿既有四節修訂，不新增Day。Red／Green另估20分鐘，原生及帳號等既有產品要求不算此切片已完成。
## 2. 實作內容
### 真實 Red
最新main的filter_room比較room_name完全相等，不是room ID。直接請求GET /api/rooms?q=Rust%20技術交流時，main fa0888e仍忽略q而回全部5筆，預期3筆的assert失敗。舊候選「取全量再client contains」已由本次使用者指定的資料庫LIKE方案取代；舊證據只保留其歷史SHA。
### 完整最小 Green：查詢在資料庫執行
入口契約採GET /api/rooms?q=房名。空白查全部，其他輸入先trim，再做文字包含搜尋；英文大小寫沿SQLite LIKE語意。百分號、底線與反斜線視為房名中的字面字元，escape後以bind傳參數，不將使用者字串拼入SQL。UI只顯示API結果，不再下載全量後篩選。
在crates/chat_application/src/[repository.rs](http://repository.rs)既有RoomRepository增加方法；其他既有方法保留：
```rust
async fn search_by_name(&self, query: &str) -> Result<Vec<Room>, RepositoryError>;
```
測試fixture的RoomRepository實作亦須補此方法；未使用搜尋的fixture可回self.list().await，正式SqliteRoomRepository必須執行以下SQL。在crates/persistence/src/[lib.rs](http://lib.rs)既有impl以這兩個方法取代原list：
```rust
async fn list(&self) -> Result<Vec<Room>, RepositoryError> {
    self.search_by_name("").await
}

async fn search_by_name(&self, query: &str) -> Result<Vec<Room>, RepositoryError> {
    // Escape LIKE metacharacters; the user supplies a name, not a SQL pattern.
    let literal = query
        .trim()
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_");
    let pattern = format!("%{literal}%");
    sqlx::query_as::<_, RoomRow>(
        r"SELECT id, category_room_id, name FROM rooms WHERE name LIKE ?1 ESCAPE '\' ORDER BY name, id",
    )
    .bind(pattern)
    .fetch_all(&self.pool)
    .await
    .map(|rows| rows.into_iter().map(Room::from).collect())
    .map_err(|e| RepositoryError::RoomAccessError(e.to_string()))
}
```
apps/server/src/[http.rs](http://http.rs)增加RoomSearch，完整替換list_rooms；既有RoomDetails DTO及路由GET /api/rooms不變：
```rust
#[derive(Default, serde::Deserialize)]
pub struct RoomSearch {
    #[serde(default)]
    q: String,
}

pub async fn list_rooms(
    State(state): State<AppState>,
    Query(search): Query<RoomSearch>,
) -> Result<Json<Vec<RoomDetails>>, StatusCode> {
    if search.q.len() > 200 || search.q.contains('\0') {
        return Err(StatusCode::BAD_REQUEST);
    }
    let rooms = state
        .room_repository
        .search_by_name(&search.q)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        rooms
            .into_iter()
            .map(|room| RoomDetails {
                id: room.id,
                category_room_id: room.category_room_id,
                name: room.name,
            })
            .collect(),
    ))
}

```
檔案頂端extract匯入改為`use axum::extract::{Query, State};`。超過200 UTF-8 bytes或含NUL回400；不以錯誤輸入查全庫。SQL先WHERE再ORDER BY，保留真實ID。
crates/ui/Cargo.toml既有web-sys features加入`"UrlSearchParams"`；crates/ui/src/room_[api.rs](http://api.rs)保留create_room，將舊filter_room移除，清單／查詢helper完整如下：
```rust
pub async fn list_rooms() -> Result<Vec<(String, String)>, String> {
    search_rooms("").await
}

pub async fn search_rooms(query: &str) -> Result<Vec<(String, String)>, String> {
    let params = web_sys::UrlSearchParams::new().map_err(|_| "無法建立查詢")?;
    params.append("q", query);
    let url = format!("/api/rooms?{}", params.to_string());
    let window = web_sys::window().ok_or("無法取得瀏覽器")?;
    let value = JsFuture::from(window.fetch_with_str(&url))
        .await
        .map_err(|_| "房間讀取失敗")?;
    let response: web_sys::Response = value.dyn_into().map_err(|_| "無效回應")?;
    if !response.ok() {
        return Err(format!("讀取失敗：{}", response.status()));
    }
    let text = JsFuture::from(response.text().map_err(|_| "無法讀取回應")?)
        .await
        .map_err(|_| "無法讀取回應")?
        .as_string()
        .ok_or("無效文字")?;
    let rows: Vec<serde_json::Value> = serde_json::from_str(&text).map_err(|_| "房間格式錯誤")?;
    rows.into_iter()
        .map(|row| {
            let id = row["id"].as_str().ok_or("缺少房間 ID")?;
            let name = row["name"].as_str().ok_or("缺少房間名稱")?;
            // 此學習入口沿用現有 UUID／fixture ID，不把任意字串當 URL。
            if id.is_empty()
                || !id
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
            {
                return Err("房間 ID 不適用目前入口".into());
            }
            Ok((id.to_owned(), name.to_owned()))
        })
        .collect()
}

```
RoomList的query_room完整改為：
```rust
let query_room = move || {
    if loading.get_untracked() {
        return;
    }
    set_loading.set(true);
    set_status.set("查詢中…".into());
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(async move {
        let submitted = query.get_untracked();
        match room_api::search_rooms(&submitted).await {
            Ok(value) => {
                set_status.set(
                    if value.is_empty() {
                        if submitted.trim().is_empty() {
                            "尚無房間"
                        } else {
                            "沒有符合條件的房間"
                        }
                    } else {
                        "查詢完成，請選擇房間"
                    }
                    .into(),
                );
                set_rows.set(value);
            }
            Err(message) => {
                set_status.set(format!("查詢失敗：{message}，請重試"));
            }
        }
        set_loading.set(false);
    });
    #[cfg(not(target_arch = "wasm32"))]
    {
        set_loading.set(false);
    }
};

```
畫面列表由`filter_rooms(&rows.get(), &query.get())`改為`rows.get()`後沿用原map／真實ID連結。重新整理按鈕清空query、呼叫list_rooms取得空條件的後端結果，不在查詢失敗前清空舊rows。filter_rooms只保留為歷史純函式測試，不再由正式畫面呼叫，不能把它的測試當SQL查詢證據。
### Refactor及舊Day更正
UI只負責輸入／URL編碼／載入與錯誤；API負責輸入限制，Repository負責SQL。SQLite LIKE支援此切片的英文不分大小寫與中文部分名稱；未宣稱完整Unicode case folding。%關鍵字%前置萬用字元通常不能以一般B-tree加速，本次未新增無效索引或FTS機制。
ChatPage仍保留前一候選已恢復的「歷史載入尚未接通」提示；main套用時須保留這個SSR限制說明，不能因搜尋已修就假稱歷史完成。完整候選及SQL增量patch、Repository／HTTP回歸見[本次證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-sql-like-room-search/REPORT.md)。
### action log欄位修正與完整重測
本次新增表的真實Red：省略id與明確NULL都被SQLite接受。不要覆寫已套用的20261018122554原migration；新增20261018130000_require_chat_action_id.sql：
```sql
-- Forward-only repair. Preserve valid rows; NULL legacy IDs abort the migration
-- transaction so an operator can repair identity without silently losing logs.
CREATE TABLE chat_action_log_required_id (
    id TEXT NOT NULL PRIMARY KEY,
    action_type VARCHAR(255) NOT NULL,
    action_content TEXT,
    ref_table VARCHAR(255),
    ref_id TEXT,
    ref_column VARCHAR(255),
    created_user_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO chat_action_log_required_id
    (id, action_type, action_content, ref_table, ref_id, ref_column, created_user_id, created_at)
SELECT id, action_type, action_content, ref_table, ref_id, ref_column, created_user_id, created_at
FROM chat_action_log;
DROP TABLE chat_action_log;
ALTER TABLE chat_action_log_required_id RENAME TO chat_action_log;
```
有效舊資料須保留；有NULL舊ID時INSERT失敗、整份migration交易回滾，先由操作者修復既有身分，再重試；不可默默刪row或偽造操作者。以下測試放在crates/persistence/src/[lib.rs](http://lib.rs)既有tests模組內：
```rust
#[sqlx::test(migrations = "../../migrations")]
async fn chat_action_log_requires_stable_identity(pool: SqlitePool) {
    sqlx::query("INSERT INTO chat_action_log(id, action_type, created_user_id) VALUES ('fixture-log', 'send', 'fixture-user')")
        .execute(&pool).await.unwrap();
    for sql in [
        "INSERT INTO chat_action_log(action_type, created_user_id) VALUES ('send', 'fixture-user')",
        "INSERT INTO chat_action_log(id, action_type, created_user_id) VALUES (NULL, 'send', 'fixture-user')",
        "UPDATE chat_action_log SET id = NULL WHERE id = 'fixture-log'",
        "INSERT INTO chat_action_log(id, action_type, created_user_id) VALUES ('fixture-log', 'send', 'fixture-user')",
    ] {
        assert!(
            sqlx::query(sql).execute(&pool).await.is_err(),
            "accepted invalid log identity: {sql}"
        );
    }
    let count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM chat_action_log WHERE id = 'fixture-log'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 1);
}
```
本輪另外實測有效舊row升級與重新連線讀回、NULL舊row失敗後原資料／結構／migration history保留，共六案通過。created_user_id目前只要求非空，仍沒有綁定真實已授權人員；這個Green只修穩定紀錄身分，不宣稱完整稽核流程完成。
## 3. 套件版本
本輪實際 Rust／Cargo 1.99.1；SQLx 0.9.0、Axum 0.8.9、Leptos 0.8.22、Tokio 1.53.3、Tauri 2.12.4、tauri-build 2.7.3、wasm-bindgen 0.2.129（Cargo.lock）。cargo-leptos 0.3.11、Trunk 0.21.14；Ubuntu 26.04.1、PostgreSQL 18.6。本輪WebSocket測試用Node 22.22.1，先前Node24為歷史環境；未另驗跨平台／MSRV。候選沒有新增產品依賴。
## 4. 驗證項目
參考庫命令：cargo test --locked -p chat-application -p persistence -p server -p ui --features ui/ssr，34 passed／0 failed／0 ignored；相關all-targets Clippy與cargo leptos build通過。原main真HTTP q查詢忽略參數，Red exit1；SQL候選同查詢完整3筆。
真HTTP12案覆蓋完整／部分中文、rUsT、空白、無相符、字面%／_／反斜線／引號、注入字串及過長／NUL。瀏覽器實際請求帶URL編碼q，顯示3筆，空結果提示正確；503保留3筆及重試通過。程式用bind，不以拼接SQL字串處理輸入。
產品copilot_use相關命令測試22通過；真HTTP5案通過；其整合案例還驗證110個不相符房間不擋住SQL查詢、公開目錄不洩漏私房、一般成員不可越權、合法房主可查自己的私房。產品正式主線未合併，未將參考庫測試直接當產品驗收。
本次兩庫工作分支無遠端CI，不拿前輪main17個success jobs當本次候選證據；未重跑全workspace／PG／Pet原生，也未驗收完整帳號／ACL／歷史／跨平台。先前action-log／Cookie等證據保留原版本身分。完整命令、兩庫SHA、patch及剩餘缺口見[本次完整證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-sql-like-room-search/REPORT.md)。v1規則及next_day各19不變。
