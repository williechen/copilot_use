# Analytics Day34 r1｜首次 admin/admin 初始化及強制改密碼銜接

## ① 學習大綱

交付日期：2026-10-25（Asia/Taipei）。文件基準 `williechen/copilot_use/main@fd426c68facdfdfd97616cf04a8bbc6badfe02cc`；唯一程式來源 `williechen/rust_ai_ring/main@54f5d80a5678e68e164d56e6d9f11c57cdff1f36`，全程唯讀，由使用者套用本文程式。舊名稱 rust_ai_chat 僅供歷史追溯；資源輸出名稱與外部契約依來源保留。

已重新讀取共同規則、SOURCE_LAYOUT、SYSTEM_ARCHITECTURE、TEACHING_ALIGNMENT、FUTURE_WORK、USER_FLOWS、AUTHENTICATION、UI_ALIGNMENT_REQUIREMENTS、learning-state、manifest、三案 ENTRY 及上一篇。即時 next_day 為 34；文章發布、程式套用、功能驗收分開記錄。

本日功能：全新帳號庫首次啟動建立 admin/admin，加鹽雜湊持久保存並標記 is_init_pd；既有庫不覆寫、不解鎖停用帳號、不把已改的密碼重設。首次 admin 登入只能到 Leptos 改密碼／Email 頁，完成後撤銷原 Session，重新登入使用新密碼。這補足最早流程，先不深化 Dashboard 標籤或角色模型。

來源的 bootstrap::initialize 要求 8–128 bytes，bin/bootstrap_admin 又要求輸入自訂密碼，主服務 main 未呼叫初始化；password_login::login 同樣拒絕 4 bytes 的 admin。只修其中一處仍無法完成基本鏈。本日重用原 uac_user/uac_login_session、Argon2、首次改密碼交易與 Leptos LoginView/ChangePasswordView。

真實鏈：startup migration → ensure_default（SQLx PostgreSQL transaction）→ Leptos /login → Axum password_login::login → uac_user／受限 Session → /change-password Leptos 表單 → 原 change_password 的 CSRF／驗舊密碼／唯一 Email／更新 hash／撤銷 Session → 新密碼登入。既有 pg_store 與 bootstrap 仍在 analytics-server，analytics-persistence 的 lib.rs 仍空，本日不把它宣稱為已接通；亦不另建無呼叫者 application 骨架。

Day33 summary_count_label 已在 dashboard_ui.rs；此功能不重寫，原 PG／瀏覽器驗收缺口保留。

## ② Red → 完整最小 Green → Refactor

### Red：從全新 DB 走正式帳密與改密碼路由

將以下完整模組加入 `analytics_apps/analytics_server/src/bootstrap.rs` 末端，保留 initialize 與既有 day22_tests。只套 Red 時 ensure_default 未定義，預期編譯失敗；加入 Green A 後、尚未修 login 時，HTTP 測試預期在 303 斷言得到 401。是預期流程，未在本輪實跑。
```rust
#[cfg(all(test, feature = "postgres-integration"))]
mod day34_default_tests {
    use super::ensure_default;
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    use axum::{body::{Body, to_bytes}, http::{Request, StatusCode}};
    use sha2::{Digest, Sha256};
    use std::sync::Arc;
    use tower::ServiceExt;

    #[sqlx::test(migrations = "../migrations")]
    async fn day34_parallel_start_preserves_one_salted_admin(pool: sqlx::PgPool) {
        let (a, b) = tokio::join!(ensure_default(&pool), ensure_default(&pool));
        assert_eq!(a.unwrap(), b.unwrap());
        let rows: Vec<(String, bool)> = sqlx::query_as(
            "SELECT user_pd,is_init_pd FROM uac_user WHERE lower(user_acnt)='admin'"
        ).fetch_all(&pool).await.unwrap();
        assert_eq!(rows.len(), 1);
        assert!(rows[0].1);
        let hash = rows[0].0.clone();
        assert_ne!(hash, "admin");
        Argon2::default().verify_password(b"admin", &PasswordHash::new(&hash).unwrap()).unwrap();
        ensure_default(&pool).await.unwrap();
        let after: String = sqlx::query_scalar("SELECT user_pd FROM uac_user WHERE user_acnt='admin'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(after, hash);
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn day34_real_login_change_and_restart_never_reset_password(pool: sqlx::PgPool) {
        ensure_default(&pool).await.unwrap();
        let app = crate::password_login::build(crate::IngestState {
            pool: pool.clone(),
            tokens: Arc::new(crate::IngestTokens {
                chat: "fixture-chat".into(), pet: "fixture-pet".into()
            }),
        });
        let login_page = app.clone().oneshot(
            Request::get("/login").body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(login_page.status(), StatusCode::OK);
        let body = to_bytes(login_page.into_body(), 16384).await.unwrap();
        assert!(std::str::from_utf8(&body).unwrap().contains("analytics-password"));

        let login = app.clone().oneshot(Request::post("/login")
            .header("origin", "http://127.0.0.1:3100")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from("username=admin&password=admin")).unwrap()
        ).await.unwrap();
        assert_eq!(login.status(), StatusCode::SEE_OTHER);
        assert_eq!(login.headers()["location"], "/change-password");
        let cookie = login.headers()["set-cookie"].to_str().unwrap()
            .split(';').next().unwrap().to_owned();
        let raw = cookie.strip_prefix("analytics_session=").unwrap();
        let csrf: String = Sha256::digest(format!("change-password:{raw}").as_bytes())
            .iter().map(|b| format!("{b:02x}")).collect();

        let denied = app.clone().oneshot(Request::get("/dashboard")
            .header("cookie", &cookie).body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(denied.status(), StatusCode::SEE_OTHER);
        assert_eq!(denied.headers()["location"], "/change-password");
        let change_page = app.clone().oneshot(Request::get("/change-password")
            .header("cookie", &cookie).body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(change_page.status(), StatusCode::OK);
        let body = to_bytes(change_page.into_body(), 16384).await.unwrap();
        assert!(std::str::from_utf8(&body).unwrap().contains("change-current"));

        let changed = app.clone().oneshot(Request::post("/change-password")
            .header("origin", "http://127.0.0.1:3100")
            .header("cookie", &cookie)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(format!("current_password=admin&new_password=changed-pass-34&confirm_password=changed-pass-34&email=admin%40example.test&csrf={csrf}"))).unwrap()
        ).await.unwrap();
        assert_eq!(changed.status(), StatusCode::SEE_OTHER);
        assert_eq!(changed.headers()["location"], "/login");
        let before: (String, String, bool) = sqlx::query_as(
            "SELECT user_pd,user_email,is_init_pd FROM uac_user WHERE user_acnt='admin'"
        ).fetch_one(&pool).await.unwrap();
        assert_eq!(before.1, "admin@example.test");
        assert!(!before.2);
        ensure_default(&pool).await.unwrap(); // 與重啟啟動路徑相同。
        let after: (String, String, bool) = sqlx::query_as(
            "SELECT user_pd,user_email,is_init_pd FROM uac_user WHERE user_acnt='admin'"
        ).fetch_one(&pool).await.unwrap();
        assert_eq!(after, before);
        let stale = app.clone().oneshot(Request::get("/api/v1/events/web_chat/summary")
            .header("cookie", &cookie).body(Body::empty()).unwrap()
        ).await.unwrap();
        assert_eq!(stale.status(), StatusCode::UNAUTHORIZED);
        for (password, expected) in [("admin", StatusCode::UNAUTHORIZED),
            ("changed-pass-34", StatusCode::SEE_OTHER)] {
            let result = app.clone().oneshot(Request::post("/login")
                .header("origin", "http://127.0.0.1:3100")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(format!("username=admin&password={password}"))).unwrap()
            ).await.unwrap();
            assert_eq!(result.status(), expected);
            if expected == StatusCode::SEE_OTHER {
                assert_eq!(result.headers()["location"], "/dashboard");
            }
        }
    }
}
```

### Green A：補齊具鎖定、冪等及不重設語意的初始化

在同一 `bootstrap.rs` 中、測試模組前新增以下完整函式。原檔 imports `use argon2::{Argon2, PasswordHasher};` 已足夠，SQLx/Tokio/UUID 使用全路徑。保留原 initialize(public 自訂密碼工具 API) 與測試，不修改其 8 bytes 契約；產品正式啟動及 CLI 改呼叫下面入口：
```rust
/// 只在空帳號庫建立預設管理員；存在資料時不改任何帳密或狀態。
pub async fn ensure_default(pool: &sqlx::PgPool) -> Result<String, String> {
    let mut tx = pool.begin().await.map_err(|_| "無法開始初始化".to_owned())?;
    sqlx::query("LOCK TABLE uac_user IN EXCLUSIVE MODE")
        .execute(&mut *tx).await.map_err(|_| "無法鎖定帳號表".to_owned())?;
    let admins: Vec<(String,)> = sqlx::query_as(
        "SELECT id FROM uac_user WHERE lower(user_acnt)='admin' LIMIT 2"
    ).fetch_all(&mut *tx).await.map_err(|_| "無法查核管理員".to_owned())?;
    if admins.len() > 1 {
        return Err("管理員身分重複；停止初始化，需人工查核".into());
    }
    if let Some((id,)) = admins.into_iter().next() {
        tx.commit().await.map_err(|_| "初始化查核提交失敗".to_owned())?;
        return Ok(id);
    }
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM uac_user")
        .fetch_one(&mut *tx).await.map_err(|_| "無法查核帳號庫".to_owned())?;
    if count != 0 {
        return Err("非空帳號庫缺管理員；不自動重建預設帳密".into());
    }
    // 沿用 Argon2 0.6 PasswordHasher 的隨機 salt，不儲存明碼。
    let hash = tokio::task::spawn_blocking(|| {
        Argon2::default().hash_password(b"admin")
            .map(|h| h.to_string()).map_err(|_| "密碼雜湊失敗".to_owned())
    }).await.map_err(|_| "密碼工作失敗".to_owned())??;
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO uac_user (id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at)
         VALUES ($1,'admin',$2,'admin@localhost.invalid','管理員','active',TRUE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)"
    ).bind(&id).bind(hash).execute(&mut *tx).await
        .map_err(|_| "管理員寫入失敗".to_owned())?;
    tx.commit().await.map_err(|_| "管理員提交失敗".to_owned())?;
    Ok(id)
}
```

兩個並行啟動均會在 EXCLUSIVE table lock 後查核；只有第一個空庫執行 INSERT，第二個沿用已建立的 ID/hash。非空庫缺 admin 或存在大小寫歧義則 fail closed，不製造另一個預設管理員。既有停用／密碼／Email／is_init_pd 全部保持原值；不在 startup 對既有帳號重新 hash。

### Green B：同時接回主啟動與現有 CLI

在 `analytics_apps/analytics_server/src/main.rs` 既有來源 token 合法性檢查 `if chat.is_empty() ... { return Err(...); }` **結束後，`let app = password_login::build(...)` 之前**新增：
```rust
analytics_server::bootstrap::ensure_default(&pool)
    .await
    .map_err(std::io::Error::other)?;
```

此時原 migration 已成功、服務設定已驗證，且還未 bind/serve；初始化失敗使啟動失敗，不對外提供半完成登入流程。`pool` 仍由參照借用，之後移入 IngestState 不變。

完整取代 `analytics_apps/analytics_server/src/bin/bootstrap_admin.rs`：
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let url = std::env::var("ANALYTICS_DATABASE_URL")?;
    let pool = sqlx::PgPool::connect(&url).await?;
    sqlx::migrate!("../migrations").run(&pool).await?;
    analytics_server::bootstrap::ensure_default(&pool)
        .await.map_err(std::io::Error::other)?;
    println!("管理員初始化查核完成；既有帳密與狀態保持原值");
    Ok(())
}
```

CLI 與 startup 由同一函式判定首次／既有資料；不再由 CLI 私自選另一套預設密碼政策。原 rpassword 依賴可暫留，不因不再呼叫而強制更新其他 lock package。

### Green C：讓預設 admin 密碼抵達雜湊驗證

在 `analytics_apps/analytics_server/src/password_login.rs::login`，只將原 username/password 長度檢查的整個 if 替換成：
```rust
if !form.username.trim().eq_ignore_ascii_case("admin")
    || !(form.password == "admin" || (8..=128).contains(&form.password.len()))
{
    return Err(StatusCode::UNAUTHORIZED);
}
```

這只容許預設值 admin 進入原 Argon2 驗證，**不**繞過 hash／active 檢查、不產生特殊萬用密碼。已改密碼的帳號輸入 admin 仍驗證失敗。原 change_password 新密碼仍至少 8 bytes、不能與舊密碼相同，Email 唯一、交易與 Session 撤銷原樣保留；所有 Dashboard/API 仍檢查 restricted。

現行 Leptos LoginView 沒有 minlength=8，已能送出預設密碼；ChangePasswordView 的新密碼欄位 minlength=8 保留。頁面已透過 render_login/render_change_password 接到正式 Axum handlers，無需另建 HTML/JavaScript 頁面或 WASM bundle。

### Cargo、imports、metadata 與檔案清單

僅修改 server 的 `src/bootstrap.rs`、`src/main.rs`、`src/bin/bootstrap_admin.rs`、`src/password_login.rs` 四檔。無新 crate、搬移、migration、Cargo dependency/feature 或 lock 變動。analytics-server 已有 argon2 0.6、sqlx 0.9 postgres/macros/runtime-tokio、tokio 1.54/full、uuid 1.29/v4、Leptos 0.8/ssr；dev-dependencies tower 0.5/util、sha2/serde_json 現有依賴滿足測試。Rust crate 用 analytics_server，package/-p 為 analytics-server，目錄 analytics_server。Analytics 是現有 server 內 native Leptos SSR；不新增 Chat 的 cargo-leptos metadata，亦不宣稱已有 Dashboard hydration。

### Refactor

主啟動與 CLI 共用單一預設初始化用例；鎖與 SQLx 保持同一 transaction，成功後才建立 HTTP router。既有自訂 initialize API 保留相容性，但不再是正常首次產品啟動路徑。後續若移到 analytics-persistence，需將實際 imports、caller、測試及 Cargo 一併搬移；本日不預先建立空 port。

### 上一 Day 未完成事項與舊 Day 更正補充

Day33 功能已存在而本文未重新驗收；其舊路徑 apps/stats_server 應在後續套用時對應 analytics_apps/analytics_server，`-p stats-server` 改為 `-p analytics-server`，命令從 analytics_apps 執行。原發文 commit/blob/SHA256 不回寫。Analytics Day24 FIFO 已依 manifest 結案，不重複增加或關閉 Day；角色/來源 ACL、SSE、明細、登入等待及共用認證仍未完成。

## ③ 套件與軟體版本

2026-10-25 重新核對官方發布頁／套件原始文件；最新穩定與 lock 分開，未以 nightly 的 docs.rs 建置工具鏈當 MSRV，也不把 RC 當正式版。本日保留來源 lock，不做無關升級。

| 項目 | 宣告／解析或環境 | 官方最新穩定與來源 |
| --- | --- | --- |
| Rust/Cargo | 各 workspace rust-toolchain.toml = stable；未量測安裝版 | [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) |
| Leptos | 0.8 / 0.8.22 | [0.8.22（排除 0.9 RC）](https://github.com/leptos-rs/leptos/releases) |
| SQLx | 0.9 / 0.9.0 | [0.9.0](https://github.com/transact-rs/sqlx/discussions/4271) |
| Tokio | 1.54 / 1.54.1 | [1.54.1](https://github.com/tokio-rs/tokio/releases) |
| Serde | 1.0 + derive / 1.0.229 | [1.0.229](https://docs.rs/crate/serde/latest) |
| UUID | 1.29 + serde,v4,js / 1.29.0 | [1.29.0](https://docs.rs/crate/uuid/latest) |
| Axum | 0.8 + ws / 0.8.9 | [0.8.9](https://github.com/tokio-rs/axum/releases) |
| Argon2 | 0.6 / 0.6.0 | [0.6.0](https://docs.rs/crate/argon2/latest) |
| SHA-2 | 0.11 / 0.11.1 | [0.11.1](https://docs.rs/crate/sha2/latest) |
| tower（測試） | 0.5 + util / 0.5.3 | [0.5.3](https://docs.rs/crate/tower/latest) |
| PostgreSQL | 伺服器未啟動，安裝版未量測 | [18.6](https://www.postgresql.org/support/versioning/) |

相容性：各 workspace edition 2024；Tokio 1.54 系列官方 MSRV 1.85，edition 2024 亦至少需要 1.85。本文建議已核對的 stable 1.99.1，未宣稱在每個依賴的最低 MSRV 測試。Leptos 0.8 API 沿用 `view!`／`IntoView`／`.to_html()`；native SSR 與 WASM 各自驗證。SQLx 0.9 既有 runtime-tokio/macros 保留；Chat/Pet 使用 sqlite，Analytics 使用 postgres。實際 SQLite 以 `SELECT sqlite_version()`、PostgreSQL 以 `SHOW server_version` 讀取，未在此環境測量 runtime。CLI 不增安裝步驟；使用隨 Rust toolchain 的 Cargo/rustfmt/clippy。沒有執行付費 AI 或讀取正式秘密。

crates.io API 的瀏覽讀取回報「not accessible via this tool」，已改用上述官方 release／套件文件；該錯誤不當成版本證據。其餘未變更依賴仍由四份原始 lock 固定；完整 manifests/lock blob 證據見本目錄 `source-snapshot.json`。

## ④ 驗證步驟與結果

以下命令從使用者的 `rust_ai_ring` 根目錄進入所屬 workspace，先只套 Red，再套 Green 後重跑。不得在 copilot_use 執行產品建置。

```bash
cd analytics_apps
rustc -Vv
cargo -V
cargo test --locked -p analytics-server
# 只使用隔離測試用 PostgreSQL；在本機自行設定 DATABASE_URL，不回傳或記錄秘密。
# sqlx::test 必須能建立隔離測試 database；不要指向正式 DB。
cargo test --locked -p analytics-server --features postgres-integration day34_default_tests
cargo test --locked -p analytics-server --features postgres-integration --all-targets
cargo check --locked -p analytics-server --all-targets
cargo fmt --all -- --check
# 實機驗證時另設 ANALYTICS_DATABASE_URL 與既有兩個 fixture 來源 token。
cargo run --locked -p analytics-server --bin bootstrap_admin
cargo run --locked -p analytics-server --bin analytics-server
```

Red／Green 的預期見第②節；本次只交付文件，**未套用產品、未執行上述 Rust 測試／DB／瀏覽器或 GUI**，沒有冒稱 Red 已失敗或 Green 已通過。文件的 JSON、相對連結、精確 commit 全文與雜湊另做交付驗證。

實機流程：全新隔離 DB 啟動→Leptos 登入 admin/admin→只能進改密碼頁→改為 8+ bytes 並設定唯一 Email→原 Cookie 失效→重新登入→Dashboard→停止／重啟服務→新密碼仍有效、admin 舊密碼失敗。另測空庫兩實例並行、既有停用帳號不重啟用、非空庫缺 admin fail closed；後兩項為仍待追加的實機驗證，不以正向測試代替。既有兩個 fixture 來源 token 僅供機器管道，不能作人員登入。

CI 來源限定 `54f5d80a5678e68e164d56e6d9f11c57cdff1f36`。[該 SHA Actions](https://github.com/williechen/rust_ai_ring/actions?query=branch%3Amain) 查詢時，connector 的 PR-only workflow_runs 回空，combined statuses 也回空；這不能證明 push workflows 不存在或全部成功。後續已由 Actions REST 核實 push workflows：[Test All](https://github.com/williechen/rust_ai_ring/actions/runs/40101549290) 的 test-analytics-server（Test analytics server）失敗；[Persistence](https://github.com/williechen/rust_ai_ring/actions/runs/40101549347) 的 test-persistence（Test analytics server with PostgreSQL）失敗；Node、四個 workspace metadata、三案 check/build workflows 成功。完整 job/step 與時間見 [來源與 CI 附件](source-snapshot.json) 及 [補充說明](CI_SUPPLEMENT.md)，CI 並非全綠；尚未取得失敗完整日誌，不猜測根因；待修項包含舊報告的 Clippy／migration checksum 及本次切片待執行驗證。不得清空既有資料庫或改舊 migration checksum 迴避問題。

尚待完成：來源 CI 的 PostgreSQL 測試失敗待讀日誌定位；本篇全新建庫、並行初始化與 HTTP／瀏覽器尚待實跑；角色來源 ACL、SSE、明細及共用認證未完成。本篇未結案 FIFO、未封存、未更動 last_full_comparison；每日局部核對不等於全面比對。文章 commit 全文讀回通過後才可同步 next_day 34→35；Gate 2 仍未通過。
