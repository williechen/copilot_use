# Chat Day23：註冊 SQLite 契約查證（本地驗證修訂）

狀態：pending／非正式文章。最新 rust_ai_chat/main `f38360cd537daf6dc20e1d3868221bb56390cc7c`；教學分支基底 `fc4a72825dac985e7d0a8e8e72edb7848349caa0`。Day22（含）以前正文與證據封存，不修改。本檔是前置契約查證，不另算新 Day，也不取代登入主體。

## 1. 現況與舊候選錯誤

舊來源 `5bcef70219b480a7999fdd0fbed5384d8d28d9be` 的性別、生日為 NOT NULL，當時「省略欄位註冊回500」可重現；最新 main 已將 `mbr_user.user_sexy`、`user_birthday` 改為 nullable。完整 root migrations 執行後 PRAGMA table_info 兩欄 notnull=0，現有 signup INSERT 不填兩欄可成功。

因此移除舊候選的 DROP TABLE／RENAME 重建 SQL。最新 fresh DB **不需要**再修 nullable，也不能把已通過案例冒稱新的 Red。不得為套用本候選改任何既有 migration、修 checksum 或刪除正式 DB。

## 2. 真正程式契約與驗證程式

`apps/server/src/lib.rs` 建 pool→`persistence::migrate`（SQLx 完整 root migrator）→merge signup routes。GET /signup 原生表單、POST /signup 使用 form-urlencoded，Origin 必須為 http://127.0.0.1:3000，body 上限4096。Argon2 0.6 單參數 hash_password 在 blocking thread 執行；user_status=active，成功303 /signin，尚不建立 Session。大小寫唯一性由現有 20261019221857_member_identity.sql 的 uq_mbr_user_account／uq_mbr_user_email 保障。

main 仍只驗欄位非空，既有短帳號測試會回409而非422。既有 `chat-day22-corrections.md` 的伺服器驗證修正應在當日 Refactor 後作一個 FIFO 補教，不在本檔聲稱已套用產品。

以下是本輪實際執行的 integration test，新增於隔離副本 `apps/server/tests/day23_contract.rs`。歷史 schema fixture 為舊 SHA 的原始 auth SQL，保存在 validation/tests/fixtures/old-auth.sql；只供專用測試 DB 使用。

```rust
use axum::{body::Body, http::{Request,StatusCode}};
use tower::ServiceExt;
#[tokio::test]
async fn day23_fresh_signup_contract() {
 let pool=sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
 persistence::migrate(&pool).await.unwrap();
 let response=server::signup::routes(pool.clone()).oneshot(Request::post("/signup").header("origin","http://127.0.0.1:3000").header("content-type","application/x-www-form-urlencoded").body(Body::from("username=day23learner&email=day23%40example.test&display_name=Day23&password=lesson-only-password")).unwrap()).await.unwrap();
 assert_eq!(response.status(),StatusCode::SEE_OTHER);
 let (hash,sex,birthday):(String,Option<String>,Option<String>)=sqlx::query_as("SELECT user_pd,user_sexy,user_birthday FROM mbr_user").fetch_one(&pool).await.unwrap();
 assert_eq!((sex,birthday),(None,None));
 use argon2::{Argon2,PasswordHash,PasswordVerifier};
 Argon2::default().verify_password(b"lesson-only-password",&PasswordHash::new(&hash).unwrap()).unwrap();
}
#[tokio::test]
async fn day23_old_schema_returns_500() {
 let pool=sqlx::sqlite::SqlitePoolOptions::new().max_connections(1).connect("sqlite::memory:").await.unwrap();
 sqlx::raw_sql(include_str!("fixtures/old-auth.sql")).execute(&pool).await.unwrap();
 let r=server::signup::routes(pool.clone()).oneshot(Request::post("/signup").header("origin","http://127.0.0.1:3000").header("content-type","application/x-www-form-urlencoded").body(Body::from("username=day23learner&email=day23%40example.test&display_name=Day23&password=lesson-only-password")).unwrap()).await.unwrap();
 assert_eq!(r.status(),StatusCode::INTERNAL_SERVER_ERROR);
 let count:i64=sqlx::query_scalar("SELECT count(*) FROM mbr_user").fetch_one(&pool).await.unwrap();assert_eq!(count,0);
}
```

## 3. migration 安全與套件

最新 tracked schema 的 mbr_user 關聯表包含 mbr_user_role、mbr_user_perm、mbr_room、mbr_exclude，欄位註解中的 FK 不等於真正 FOREIGN KEY 約束。兩個大小寫唯一索引存在；本輪 fresh schema 未查到依附 mbr_user 的 trigger／view。這份盤點僅適用來源 schema，不能代替使用者 DB 的自訂索引／trigger／view／外鍵盤點。新增 Session 使用獨立 additive CREATE TABLE／INDEX，沒有 DROP mbr_user。

最新 main 改過歷史 auth SQL，故不能把 fresh schema 成功推論為既有 DB 升級成功。本輪用舊 SHA 全套 migration 建隔離庫，再以最新 migrator 執行，實際得到「previously applied but has been modified」；原 migration 記錄數未變。這證明阻擋存在，不是通用修復。SQLx 每個 migration 的 transaction 不等於整批 migration 全有或全無；仍須先備份、查實際 _sqlx_migrations 與有效歷史基底，另訂前向方案。禁止 migrate repair、重建正式 DB 或直接套用舊 DROP/RENAME 提案。

manifest／lock 精確版本及官方穩定版核對見登入候選第3節與 validation/packages-latest.json。本輪未升級來源依賴。

## 4. 實際結果與限制

`cargo test --locked -p server day23 -- --nocapture`：本輪5項通過，其中本檔舊schema500／最新完整schema303兩項通過，且最新註冊後性別／生日為NULL、Argon2密碼驗證成功。其餘3項覆蓋登入與Session及舊庫checksum拒絕，見登入候選。

main library 全測為8通過／6失敗，另五項失敗涉及 rooms.desc 必填；不能宣稱server整套全綠。完整logs、來源SHA、可套用隔離patch與重現步驟見 validation/REPORT.md。未操作正式DB、未改封存Notion頁、未發布Day23、未推進next_day，未建立排程。
