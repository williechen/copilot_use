# 2026-10-16｜P0 Chat Day 13｜HTTP 建房名稱驗證與 201 Created

來源文章：[2026-10-16｜P0 Chat Day 13｜HTTP 建房名稱驗證與 201 Created](https://app.notion.com/p/3fae51d4509381e499e5e02cfba58336?pvs=204)。

## 1. 學習大綱
P0／Chat Day 13。主體實作 20 分鐘；Refactor／必要補加不計入學習時數。
來源：rust_ai_chat/main `2aecb0f02f9176287e848b7e4897f28ef3929072`；copilot_use/main `49e1d946dd3c22ffcc4b1aa0e7715cb5b1686724`；產品教學分支 `1d1668b97e65ccfafc073829f131c93133c8bb7e`。已讀共同規則、P9_TARGET、FUTURE_WORK、SYSTEM_ARCHITECTURE、learning-state 與 Notion 共同／專案／Day 12；共同規則為準。當前 P0，最晚切點 2026-12-12，未達切點也無提前完成證據，不切換階段。首次週六完整比對尚未成功，last_full_comparison 保持 null；本日只核對選題相關差異，不冒稱全量完成。
對應 FUTURE_WORK「參考適配：Room／作者／ID／category、HTTP 建房 DTO／status」及 Chat 訊息／房間模型原項目。確切缺口：最新 `apps/server/src/http.rs` 的 create_room 接受空白／任意長度／NUL 名稱，成功預設回 200；成品 `apps/server/src/main.rs`::create_room 拒絕這些名稱並回 201。
沿用學習庫的 server 產生 UUID、default-category、CreateRoomRequest\{name\} → CreateRoomResponse\{id\}；成品的 caller ID、Session／ACL／chat_rooms 交易不直接搬入。學習庫 `crates/ui/src/room_api.rs` 使用 response.ok()，因此可接受 201，無須改 UI。Day 12 typing 已存在，不重教。
## 2. 實作內容
### Red：先加入行為測試
在 `apps/server/src/http.rs` 檔尾新增以下私有模組。現有 imports 已足夠；不新增依賴。測試直接呼叫真實 HTTP handler 並經 IntoResponse，搭配隔離 SQLite；完整 router／socket HTTP 仍列為待執行。
```rust
#[cfg(test)]
mod room_http_tests {
    use super::*;
    use axum::response::IntoResponse;
    use std::sync::Arc;

    async fn fixture() -> (AppState, sqlx::SqlitePool) {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:").await.unwrap();
        persistence::migrate(&pool).await.unwrap();
        let state = AppState::new(
            leptos_config::LeptosOptions::default(),
            Arc::new(persistence::SqliteRoomRepository::new(pool.clone())),
            Arc::new(persistence::SqliteMessageRepository::new(pool.clone())),
        );
        (state, pool)
    }

    #[tokio::test]
    async fn invalid_room_names_never_write() {
        let (state, pool) = fixture().await;
        for name in ["".to_string(), " \n\t".into(), "a\0b".into(),
                     "x".repeat(201), "房".repeat(67)] {
            let response = create_room(
                State(state.clone()), Json(CreateRoomRequest { name }),
            ).await.into_response();
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM rooms")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count, 0);
        pool.close().await;
    }

    #[tokio::test]
    async fn valid_room_returns_201_and_saved_id() {
        let (state, pool) = fixture().await;
        let name = " 房間 ".to_string();
        let response = create_room(
            State(state), Json(CreateRoomRequest { name: name.clone() }),
        ).await.into_response();
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
        let created: CreateRoomResponse = serde_json::from_slice(&body).unwrap();
        assert!(uuid::Uuid::parse_str(&created.id).is_ok());
        let row: (String, String) = sqlx::query_as(
            "SELECT name, category_room_id FROM rooms WHERE id = ?1"
        ).bind(created.id).fetch_one(&pool).await.unwrap();
        // 保留原名稱；trim 只檢查是否全為空白。
        assert_eq!(row.0, name);
        assert_eq!(row.1, "default-category");
        pool.close().await;
    }

    #[tokio::test]
    async fn database_failure_is_not_created() {
        let (state, pool) = fixture().await;
        pool.close().await;
        let response = create_room(
            State(state.clone()), Json(CreateRoomRequest { name: "有效名稱".into() }),
        ).await.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        // 即使 DB 關閉，無效輸入仍應先被拒絕。
        let response = create_room(
            State(state), Json(CreateRoomRequest { name: " ".into() }),
        ).await.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
```
先執行第 4 節命令：原碼預期 invalid 名稱回 200、合法名稱也回 200，使斷言失敗。這是預期 Red，尚未實際執行。
### 最小 Green：替換既有 create_room
只替換 `apps/server/src/http.rs` 的 create_room，health、HealthResponse 與既有 imports 保留：
```rust
pub async fn create_room(
    State(state): State<AppState>,
    Json(request): Json<CreateRoomRequest>,
) -> Result<(StatusCode, Json<CreateRoomResponse>), StatusCode> {
    // 與成品相同：長度採 UTF-8 位元組，拒絕全空白與 NUL。
    if request.name.trim().is_empty()
        || request.name.len() > 200
        || request.name.contains('\0')
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let room = chat_application::create_room(
        state.room_repository.as_ref(),
        chat_application::CreateRoom {
            id: uuid::Uuid::new_v4().to_string(),
            category_room_id: "default-category".to_string(),
            name: request.name,
        },
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // Repository 成功才回 201；JSON 契約仍只含既有 id。
    Ok((StatusCode::CREATED, Json(CreateRoomResponse { id: room.id })))
}
```
呼叫鏈維持 POST /api/rooms → http::create_room → chat_application::create_room → RoomRepository::create → SqliteRoomRepository::create。SQL 留在 persistence，沒有新增 Port、schema 或 migration。其他檔案 N/A。
### Refactor
名稱驗證與回應狀態統一由 `http::create_room` 處理：先拒絕非法名稱，再呼叫既有 `RoomRepository`，寫入成功後回傳 201 與既有 JSON。保留 `CreateRoomRequest`、`CreateRoomResponse`、UUID 產生方式與 `default-category`，UI 繼續以 `response.ok()` 接受成功回應。
#### 整合現有測試：移除 workspace 根目錄 tests
本項是現有測試的歸屬整合與重構，加入既有 Day 13 第 2 節 Refactor；不計為新功能、不推進 Day，也不占三專案主體合計 60 分鐘。以下依唯讀參考庫 `3e58e650bd63bcd21ba58e7aabf751542401ef80` 盤點，由你在 `/home/willie/Documents/rust_ai_chat` 套用。原有測試內容、斷言與產品行為保持；依序完成 Chat、Pet、Analytics 的搬移後，才移除共同測試套件。
先在任何搬移前，於 workspace 根目錄保存原測試清單：
```bash
cargo test --locked -p workspace-tests -- --list
```
本篇負責下列檔案，執行位置為 workspace 根目錄：
```bash
mkdir -p crates/chat_application/tests
mkdir -p apps/server/tests
git mv tests/chat/persist_publish.rs crates/chat_application/tests/persist_publish.rs
git mv tests/chat/product_event_contract.rs crates/chat_application/tests/product_event_contract.rs
git mv tests/server/day5_tests.rs apps/server/tests/day5_tests.rs
```
`chat-application` 已具備 async-trait、chat-domain、uuid 與 tokio；`server` 已具備 shared、tokio，沿用既有依賴，不新增公開 API。
三篇搬移全部完成後，在同一次重構中收尾：
1. 根 `Cargo.toml` 的 `workspace.members` 移除 `"tests"`；移除 `tests/Cargo.toml` 的 `workspace-tests` 套件。新位置採 Cargo 自動探索整合測試，不要搬回原本 `autotests = false` 或舊 `[[test]]` 清單。
2. `.github/workflows/test.yml` 原 `cargo test --locked -p workspace-tests` 改為下列四個擁有測試的套件；保留既有測試環境設定與其他工作，不新增本機 Docker 需求。
```bash
cargo test --locked -p chat-application -p pet-domain -p server -p stats-server --tests
```
1. 確認 `tests/common/mod.rs` 仍為空檔、`tests/analytics/postgres_ingest.rs` 仍只有空白後，以 `git rm` 移除這兩個空檔及 `tests/Cargo.toml`，再以 `rmdir` 移除殘留空目錄。若檔案已有內容，先整合到對應套件，不可直接刪除；所有有斷言的檔案必須先完成搬移。
2. 執行 `cargo metadata --offline --format-version 1 --no-deps > /dev/null` 讓 Cargo 同步套件關係與 lock；若需要鎖檔解析，再執行 `cargo generate-lockfile --offline`，檢查 `Cargo.lock` 僅移除 `workspace-tests` 及重接 `stats-server` 的既有 tower 依賴。若有無關版本變動，先還原該次 lock 變動並排除解析原因；不使用 `cargo update` 升級套件。
3. 在原生本機測試環境先用 `cargo test --locked -p workspace-tests -- --list` 保存搬移前清單；搬移後以四套件的 `--tests -- --list` 比對函式名稱、數量與 ignored 狀態，再執行測試。這條搬移前命令必須在第 1 步之前執行。
```bash
cargo test --locked -p chat-application -p pet-domain -p server -p stats-server --tests -- --list
cargo test --locked -p chat-application -p pet-domain -p server -p stats-server --tests
cargo fmt --all -- --check
test ! -d tests
rg -n 'workspace-tests|tests/(chat|pet|analytics|server)/' Cargo.toml Cargo.lock .github
```
最後一條預期沒有舊套件／路徑命中（`rg` 無命中退出碼為 1）；不能只以整體綠燈認定測試未遺漏。四套件可能同時列出既有其他測試，應逐一確認這次搬移的 11 個非空測試檔都有被探索，斷言逐位元保留。Analytics HTTP 契約測試仍需原有 PostgreSQL 與 `ANALYTICS_DATABASE_URL`、`ANALYTICS_CHAT_TOKEN`、`ANALYTICS_PET_TOKEN`；使用本機測試資料庫與合成 Token，不指向正式資料或讀取私人 `.env`。原有 `postgres_event_store` 測試及其 feature gate 保留，不因空白的 `postgres_ingest.rs` 被移除而刪除。
本次只盤點並提供重構步驟；參考庫維持唯讀，尚未套用搬移，也未執行搬移後測試。原文的版本與 CI 證據屬原教學基準，不能當作這次重構已通過的結果。
## 3. 套件版本
workspace 0.2.12／edition 2024。最新 Cargo.toml 宣告 axum 0.8、serde 1.0、tokio 1.53、sqlx 0.9、uuid 1.28；同 SHA Cargo.lock 分別為 0.8.9、1.0.229、1.53.3、0.9.0、1.28.0；serde_json 1.0.154、leptos 0.8.22。維持既有版本，無新依賴／無升級。官方最新 stable、MSRV、安全公告本次未查證；基準 native／WASM CI 通過不能代替新程式相容性驗證。
## 4. 驗證項目
### 實作邊界與未完成項目
驗證放在 HTTP 邊界，因本切片明確是 HTTP DTO 契約；Application 的其他直接呼叫者仍須之後在原房間模型項目補齊共用業務不變量，不能聲稱全系統驗證已完成。保留周邊空白，避免默默更名；200 位元組上限與成品一致，不當作 200 個中文字。建房 Session／ACL、category 選擇、Outbox、history、安全撤權不在本日，仍留原 FUTURE_WORK。DB 失敗沿用既有 500，沒有擴大錯誤重構。
### 驗證命令與結果
本次重新查核最新來源 SHA 的六份 workflow 定義與實際紀錄：Rust AI Chat、Test All、Persistence、Node、Rust Dashboard、Rust Desktop Pet，6/6 workflows、18/18 checks 均 completed/success，沒有 skipped/cancelled/pending check。main 未設 branch protection 必要 contexts；另無 legacy commit-status context（status API 的 pending/total_count=0 只表示沒有 legacy status，並非一個待完成工作），此庫適用 Gate 為上述 Actions/check-runs，已逐一核對，沒有用歷史 CI 替代。[匹配 SHA 的 CI](https://github.com/williechen/rust_ai_chat/actions?query=branch%3Amain)。基準 CI 通過只允許教學發布，不代表以下新增程式已套用或測試通過。
依序執行：
```bash
cargo test --locked -p server room_http_tests
cargo fmt --all -- --check
cargo clippy --locked -p server --all-targets -- -D warnings
cargo check --locked -p server --features ssr
cargo check --locked -p ui --features hydrate --target wasm32-unknown-unknown
```
預期 Green：非法名稱 400 且 rooms 數量為 0；合法名稱 201、JSON ID 對應實際 SQLite row；DB 關閉 500；UI 仍接受 201。另需在隔離 SQLite 啟動 server，實際 POST /api/rooms 核對 400／201／JSON／row；此 HTTP E2E 尚未執行。
本次只產生教學，未套用以上程式、未執行新測試、未宣稱 Red／Green 實測成功；既有 source CI 已重新查核。發布與讀回由本次交易紀錄確認；功能狀態：程式尚待套用、功能尚未由本日實作證明、正式驗收未通過。未讀密鑰、未呼叫付費 AI、未修改兩庫產品 source／tag。
