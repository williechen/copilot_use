# Chat Day 23：註冊前置 SQLite 契約修補（未驗證候選）

狀態：**pending／不得冒充 Day23 Green**。來源 rust_ai_chat/main `5bcef70219b480a7999fdd0fbed5384d8d28d9be`；教學分支寫入前 `b8fd48cfc77a864764363e572e5d75bdcd5a9eae`。Day22（含）以前文章封存、不得改。此候選用於真實 Red/Green 前的資料契約核對；登入與 Session 尚需另一完整 Red/Green。

## 已由程式核實的前置錯誤

`migrations/20261018073931_create_auth_user.sql` 中 `mbr_user.user_sexy`、`mbr_user.user_birthday` 為 `NOT NULL` 且沒有 `DEFAULT`。現有 `apps/server/src/signup.rs` 的 `INSERT INTO mbr_user` 未提供這兩欄。按 SQLite NOT NULL 約束，成功註冊會被 DB 拒絕；現有 409/422 測試若未執行不能當作成功證據。已讀後續 root migrations：member_identity 只新增大小寫唯一索引；mbr_room 只新增房間關聯表，沒有解除上述限制。

**避免捏造使用者性別／生日。** 表單需求不要求收集這些個資，因此建議以新的*前向 migration* 使兩欄可為 NULL，保留舊資料值，且不更改任何已套用歷史 migration。此提案必須在專用 SQLite DB 與完整外鍵／索引／trigger 盤點後測試，才可作為 Green。禁止直接在正式 DB 執行本文範例。

## Red（先以真實遷移 schema 重現）

檔案：`apps/server/src/signup.rs` 測試模組。新增如下案例，使用既有 `routes(pool.clone())`：

```rust
#[tokio::test]
async fn day23_signup_fresh_schema_returns_303_and_persists() {
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1).connect("sqlite::memory:").await.unwrap();
    sqlx::raw_sql(include_str!("../../../migrations/20261018073931_create_auth_user.sql"))
        .execute(&pool).await.unwrap();
    sqlx::raw_sql(include_str!("../../../migrations/20261019221857_member_identity.sql"))
        .execute(&pool).await.unwrap();
    let response = routes(pool.clone())
        .oneshot(Request::post("/signup")
            .header("origin", "http://127.0.0.1:3000")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(
                "username=day23learner&email=day23%40example.test&display_name=Day23&password=lesson-only-password"
            )).unwrap()).await.unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::SEE_OTHER);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM mbr_user")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(count, 1);
}
```

預期舊 schema 的實際回應不是 303，而是資料庫 NOT NULL 失敗導致 500；**本輪尚未實際執行此 Red**，不得將推論當作測試紀錄。

## Green 前向 migration 設計（待整合測試）

新檔案建議：`migrations/20261020XXXXXX_mbr_optional_profile_fields.sql`（時間戳命名先檢查當時分支，不能和既有檔案撞名）。必須保留所有原始欄位與原有資料，不用 `ALTER ... DROP COLUMN` 毀損使用者資料。

```sql
CREATE TABLE mbr_user_day23 (
    id varchar(40) NOT NULL PRIMARY KEY,
    user_acnt TEXT NOT NULL,
    user_pd TEXT NOT NULL,
    user_email TEXT NOT NULL,
    user_name TEXT NOT NULL,
    user_status TEXT NOT NULL,
    user_sexy TEXT,
    user_birthday TIMESTAMP,
    is_init_pd BOOLEAN NOT NULL,
    expired_pd_at TIMESTAMP,
    last_login_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
INSERT INTO mbr_user_day23 (
    id,user_acnt,user_pd,user_email,user_name,user_status,
    user_sexy,user_birthday,is_init_pd,expired_pd_at,last_login_at,created_at,updated_at
)
SELECT id,user_acnt,user_pd,user_email,user_name,user_status,
       user_sexy,user_birthday,is_init_pd,expired_pd_at,last_login_at,created_at,updated_at
FROM mbr_user;
DROP TABLE mbr_user;
ALTER TABLE mbr_user_day23 RENAME TO mbr_user;
CREATE UNIQUE INDEX uq_mbr_user_account ON mbr_user(lower(user_acnt));
CREATE UNIQUE INDEX uq_mbr_user_email ON mbr_user(lower(user_email));
```

此 SQL 只是候選。使用前須查 `sqlite_master` 及 `pragma_foreign_key_list` 找出所有指向 mbr_user 的外鍵、trigger、view、其他索引。若存在相依物，依實際 schema 改寫 migration 並於 transaction/備份回復方案下測試；不可假定 DROP/RENAME 在所有現有 DB 都安全。實際 sqlite/sqlx migration transaction 行為亦須驗證。

## Red/Green 的測試要求

1. 專用 fresh SQLite：原 schema 的 Red 必須真的失敗於 NOT NULL；加新 migration 後同一 HTTP 行為變 303，且密碼雜湊可驗證。
2. 模擬既有 DB：先插入一筆包含性別／生日的合法舊資料，再跑前向 migration；原 ID、歷史值、帳密 hash 與大小寫唯一索引全部保留；`pragma_foreign_key_check` 無違規。
3. 以完整 migration runner 驗證既有 checksum 沒被更改；任何 mismatch 立即停止人工確認，不執行 `sqlx migrate repair`、不重建正式 DB。
4. 登入最小切片仍須另外提供 Account verification + durable Session + Cookie + server protected endpoint 的同一 Day 完整程式與測試，否則 Day23 只能標未完成。
5. 新版 Argon2 0.6 使用現有專案的單參數 `hash_password`，不可引入封存 Day22 的 0.5 範例；版本依精確當日 `Cargo.toml`／`Cargo.lock`。

## 本輪實際限制及續接點

直接嘗試 `git ls-remote https://github.com/williechen/rust_ai_chat.git HEAD`，原始錯誤：`fatal: unable to access 'https://github.com/williechen/rust_ai_chat.git/': Could not resolve host: github.com`。因此目前本地執行環境無法 clone／build／test 專案。GitHub connector 可唯讀檢索，教學分支可寫入證據。尚未執行 Day23 測試、Notion 正式發布、全文讀回或狀態遞增。此文件是具體可操作的下一步與 Red／migration 候選，不是完整已驗證登入教學。

CI 來源：`rust_ai_chat/main` 上 Persistence run 39426649469 `test-persistence` 的 `Test persistence` step 失敗，`Run database migrations` step 成功；沒有取得失敗 log，不應推測此 CI 失敗就是上述 NOT NULL 問題。
