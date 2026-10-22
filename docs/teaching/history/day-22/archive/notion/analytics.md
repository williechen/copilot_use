## 1. 學習大綱
Day22 r2 延續原頁：透過管理者明確執行的 CLI → 隱藏輸入密碼 → Argon2id → PostgreSQL transaction／帳號表鎖 → 只建立一次 admin。本日不新增匿名 bootstrap HTTP，沿用現有 uac_user，不覆寫已存在帳號。主體 Red／Green 估 15 分鐘；Refactor 3 分鐘，來源／版本／驗證閱讀另估 8 分鐘。三案主體合計 45 分鐘，保留共用彈性 15 分鐘；額外閱讀／Refactor 不藏進主體估時。
來源 rust_ai_chat/main `da5155265bf66ef9d495fd38913ad6cb93e1bac8`，copilot_use 教學基底 `b3db92fcc1ff488b758eee5702f320d623d603b8`，規則 daily-tdd-minimum-v1，Notion 共同頁最後修改 2026-10-19T08:19:19.122Z。2026-10-20 Asia/Taipei 重讀必要規則、進度、manifest、三專案入口及 Day21。入口歷史 next_day=19／17 不覆蓋目前狀態 22。
main 仍以固定 key 的 admin::Config 與記憶體 Session 登入；Day21 加 username 欄位沒有帳密驗證。現有 PostgreSQL uac_user 有 user_pd、is_init_pd 等欄位但尚無初始化呼叫者。本切片 CLI 使用相同 ANALYTICS_DATABASE_URL 與 migrations，供下一段帳密登入使用；不把新帳號接到舊 key Session，也不宣稱首次登入改密碼已生效。
copilot_use 使用 day9_analytics::AnalyticsState／admin_token 與不同 auth schema，沒有本教學 initialize 契約，產品仍須獨立整合。來源全庫唯讀；本頁是供使用者套用的教學，不變更正式秘密、不連使用者正式 DB、不執行付費 AI。
## 2. 實作內容
### Red：真 PostgreSQL 與並發初始化
在 apps/stats_server/src/[lib.rs](http://lib.rs) 加 `pub mod bootstrap;`；新增 apps/stats_server/src/[bootstrap.rs](http://bootstrap.rs)，先只放下列測試。以專用本機測試 PostgreSQL 的 DATABASE_URL 執行 `cargo test -p stats-server --features postgres-integration day22_bootstrap`。初始化函式尚不存在，預期缺少 initialize 符號的編譯失敗。本輪未執行此 Red；連線／權限失敗與缺失符號失敗要分開。
SQLx test 使用獨立資料庫及既有 migrations；測試帳號須能建立測試 DB，勿指向正式 DB。沒有這個 feature 的既有 CI 不會執行本測試，零測試不可當成通過。
```rust
#[cfg(all(test, feature = "postgres-integration"))]
mod day22_tests {
    use super::*;
    use argon2::{PasswordHash, PasswordVerifier};
    #[sqlx::test(migrations = "./migrations")]
    async fn day22_bootstrap_serializes_and_preserves_hash(pool: sqlx::PgPool) {
        let password = "lesson-only-password".to_owned();
        let (a, b) = tokio::join!(initialize(&pool, password.clone()), initialize(&pool, password.clone()));
        assert_eq!(usize::from(a.is_ok()) + usize::from(b.is_ok()), 1);
        assert!(initialize(&pool, "short".into()).await.is_err());
        let (count,): (i64,) = sqlx::query_as("SELECT count(*) FROM uac_user").fetch_one(&pool).await.unwrap();
        assert_eq!(count, 1);
        let (hash, must_change): (String, bool) = sqlx::query_as("SELECT user_pd,is_init_pd FROM uac_user WHERE user_acnt='admin'")
            .fetch_one(&pool).await.unwrap();
        assert!(must_change);
        assert!(hash.starts_with("$argon2id$"));
        Argon2::default().verify_password(password.as_bytes(), &PasswordHash::new(&hash).unwrap()).unwrap();
        assert!(initialize(&pool, "different-lesson-password".into()).await.is_err());
        let (after,): (String,) = sqlx::query_as("SELECT user_pd FROM uac_user WHERE user_acnt='admin'").fetch_one(&pool).await.unwrap();
        assert_eq!(after, hash);
    }
}
```
### Green：完整初始化 Adapter
在 apps/stats_server/Cargo.toml 的 package 加 `default-run = "stats-server"`，使新增 binary 不破壞既有預設 cargo run；dependencies 加：
```toml
argon2 = "0.5"
rand_core = { version = "0.6", features = ["getrandom"] }
rpassword = "7"
```
更新 Cargo.lock。將以下完整程式放在 [bootstrap.rs](http://bootstrap.rs) 測試之前：
```rust
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};

pub async fn initialize(pool: &sqlx::PgPool, password: String) -> Result<String, String> {
    if !(12..=128).contains(&password.len()) {
        return Err("密碼須為 12 到 128 bytes".into());
    }
    let hash = tokio::task::spawn_blocking(move || {
        Argon2::default().hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .map(|hash| hash.to_string()).map_err(|_| "密碼雜湊失敗")
    }).await.map_err(|_| "密碼工作失敗")??;
    let mut tx = pool.begin().await.map_err(|_| "無法開始初始化")?;
    // 同時啟動的初始化會序列化；不使用先查後寫而無鎖的競態。
    sqlx::query("LOCK TABLE uac_user IN EXCLUSIVE MODE").execute(&mut *tx).await.map_err(|_| "無法鎖定帳號表")?;
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM uac_user WHERE lower(user_acnt)='admin')")
        .fetch_one(&mut *tx).await.map_err(|_| "無法查核管理員")?;
    if exists { return Err("管理員已存在；不覆寫密碼".into()); }
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO uac_user (id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at) VALUES ($1,'admin',$2,'admin@localhost.invalid','管理員','active',TRUE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
        .bind(&id).bind(hash).execute(&mut *tx).await.map_err(|_| "管理員寫入失敗")?;
    tx.commit().await.map_err(|_| "管理員提交失敗")?;
    Ok(id)
}
```
表鎖使同時初始化序列化，後到者只讀到已存在而拒絕，不重設原密碼。既有 admin／ADMIN 都拒絕；若舊資料已有多個大小寫變體，需人工清理，這段不刪資料。一般帳號管理、DB 唯一索引與角色授權仍待後續；這個鎖只保障使用本初始化流程的並發，不聲稱限制任意直接 SQL。
新增 apps/stats_server/src/bin/bootstrap_[admin.rs](http://admin.rs) 全文：
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let url = std::env::var("ANALYTICS_DATABASE_URL")?;
    let pool = sqlx::PgPool::connect(&url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let password = rpassword::prompt_password("初始化 admin 密碼（不回顯）：")?;
    let confirm = rpassword::prompt_password("再次輸入：")?;
    if password != confirm { return Err("兩次密碼不同".into()); }
    let id = stats_server::bootstrap::initialize(&pool, password).await
        .map_err(std::io::Error::other)?;
    println!("管理員已建立：{id}；帳密登入與首次改密碼仍待接通");
    Ok(())
}
```
由管理者在與服務相同且已確認的 ANALYTICS_DATABASE_URL 環境，手動執行：
```bash
cargo run -p stats-server --bin bootstrap_admin
```
密碼由終端不回顯輸入兩次，不放命令參數或日誌；只輸出建立的 ID。CLI 先 migrate，再初始化；migrations checksum／missing 或 DB 權限錯誤直接停止，不刪庫、不改舊 migration。原 apps/stats_server/src/[main.rs](http://main.rs) 不加自動初始化，每次服務啟動不會覆寫密碼。
user_email 暫填保留網域 [admin@localhost](mailto:admin@localhost).invalid，不能接收郵件；真實 Email／首次變更需後續接線。is_init_pd=TRUE 只是持久化的待改密碼標記，舊 key handler 尚未讀它，不能聲稱已強制改密碼。uac_role／uac_user_role 尚未賦權；本日只建立身分，不讓它繞過現有保護頁。
### Refactor 與必要補充
把雜湊與 transaction 放獨立 Adapter，CLI 只負責輸入／連線／結果。Domain 不依賴 PostgreSQL 或終端。後續登入 handler 必須驗 Argon2、檢查狀態與 is_init_pd、建立有效 Session，再加後台授權；不把舊 64 字元登入碼當成帳密流程。Day21 不改寫，原 Day22 草稿把 Bootstrap struct 當測試的缺失由本頁完整測試取代；未新增補教任務或額外 Day。
## 3. 套件版本
**2026-10-20 版本差異勘誤（尚未重驗 Green）**：最新 rust_ai_chat/main `cabacb75e946b03c67c455ce13a7a4ea85652f8e` 的 workspace 為 0.2.22／edition 2024；Cargo.toml 宣告 Axum 0.8、SQLx 0.9、Tokio 1.54、UUID 1.28、Leptos 0.8、`argon2 = "0.6"`（stats-server 以 workspace 使用）、`rpassword = "7.5"`，lock 實際解析 Argon2 0.6.0、Tokio 1.54.0、rpassword 7.5.4。原 Day22 Green 的 `argon2 = "0.5"`／`rand_core 0.6` 屬舊候選，**不可原樣套用**。新版 main 已有 [bootstrap.rs](http://bootstrap.rs) 及 bootstrap_[admin.rs](http://admin.rs)；應對照現存邏輯、Argon2 API、PostgreSQL 契約與測試，不重複新增檔案。舊 CI/測試證據僅證明舊 SHA，不是新版驗收。
## 4. 驗證項目
命令 `cargo test -p stats-server --features postgres-integration day22_bootstrap --no-run` 可先查編譯；真 DB 另執行相同命令去除 --no-run。預期並發兩次只有一次成功、uac_user 一筆、Argon2id 能驗密碼、is_init_pd=true、重複初始化不改雜湊；實際結果見本輪附註。Red 未執行，CLI 互動／服務登入未實測，候選不代表 main 已套用。
來源 [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39302157333) 精確上列 main SHA，test-stats-server 成功，2026-10-19T18:24:01Z–18:25:57Z，命令 cargo test --locked -p stats-server，沒有 postgres-integration feature。只覆蓋既有程式，不證明本篇 PG 初始化或帳密登入通過。
待套用驗證：使用專用資料庫執行 CLI、輸入兩次密碼，獨立 SQL 讀帳號／hash／is_init_pd；再次執行必須拒絕且原 hash 不變，服務重啟後 row 仍在。後續帳密登入、強制改密碼、持久 Session／授權、Email、角色與完整 P0 Gate 未完成；產品整合與正式功能驗收均未完成。
本輪實際結果：隔離候選編譯通過；PostgreSQL 18.6 專用臨時 cluster 上，cargo test -p stats-server --features postgres-integration day22_bootstrap：1 passed／0 failed／0 ignored。並發初始化、雜湊驗證與不覆寫已實測；其他 target 零測試未計入。首次 cluster 因 Unix socket 路徑超長未啟動，測試發生 PoolTimedOut；修正為僅本機 TCP 後重跑通過，原錯誤保留。本輪未連正式 DB、未執行 Red／CLI 互動／帳密登入／完整功能驗收。
本輪來源證據：GitHub 規則 blob 9f5efa59fee115405d2c3e5892c0460fed5bbc0f（教學基底 b3db92fcc1ff488b758eee5702f320d623d603b8）；隔離教學候選 commit 43d445ef367460ce1bb4e01f9b0c90d66f0db77a，未推送 rust_ai_chat。完整 patch SHA-256：8aedb7c8d4e01ce568201a779b05c7ff873ab8ace0c92b01ab4f328dcf43c50a，已對精確 main 基底 git apply --check 通過。copilot_use 基底 CI Verify copilot_use / run 39332356560 成功，compose-e2e skipped；僅屬產品既有程式證據，不代替本日教學測試。讀回與完整命令紀錄存教學分支 docs/teaching/day22/。
