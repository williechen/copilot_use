# Day23 第2節追加單元：Day22 更正補充

狀態：pending，尚未發布 Day23。執行前須重讀 main、規則、套件、makeup_queue；預期 next_day=23，但本文件不推進計數。只在本日新 Red／完整 Green／Refactor 結束後追加，不取代本日新實作。依 FIFO，每篇最多一個舊 Day 單元，發布與全文讀回後才可標結案。

來源核對基底 `cabacb75e946b03c67c455ce13a7a4ea85652f8e`，規則 daily-tdd-minimum-v1 ＋ 2026-10-20 封存順序補充。

## 舊 Day 更正補充（程式／套件差異）：Chat Day22

先列觀察：main 目前編譯成功，但既有註冊測試短帳號實際 409／預期 422。其後再列修正與證據，不能把這個失敗當成新登入功能的 Red。

### Argon2 與欄位驗證

0.6 的單參數 hash_password 自帶隨機鹽；保留 workspace argon2=0.6，不加舊 SaltString／OsRng。保留目前 8-byte 密碼政策並明示與原文 12 的差異；恢復帳號 3–32 ASCII 字元、顯示名稱 1–32 字元、Email 基本格式與長度、密碼 8–128 bytes 的伺服器檢查。這是候選方案，套用前仍須以當日需求與最新來源為準。

完整修改只涉及 apps/server/src/signup.rs；以下 patch 可對本次 SHA 套用，保留現有0.6雜湊及路由，不重建已存在的 signup 或 migration。

```diff
--- a/apps/server/src/signup.rs
+++ b/apps/server/src/signup.rs
@@ -71,8 +71,18 @@
     let username = form.username.trim().to_owned();
     let email = form.email.trim().to_ascii_lowercase();
     let display = form.display_name.trim().to_owned();
-    // 本切片 Email 為基本格式檢查；驗證信另列後續。
-    if username.is_empty() || email.is_empty() || display.is_empty() || form.password.is_empty() {
+    // 保留新版畫面的 8-byte 密碼政策；所有限制都必須在伺服器重驗。
+    // Email 僅做基本格式檢查，驗證信仍屬後續功能。
+    let email_parts: Vec<_> = email.split('@').collect();
+    if !(3..=32).contains(&username.len())
+        || !username.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
+        || !(1..=32).contains(&display.chars().count())
+        || !email.is_ascii() || email.len() > 254 || email_parts.len() != 2
+        || email_parts[0].is_empty() || !email_parts[1].contains('.')
+        || email_parts[1].starts_with('.') || email_parts[1].ends_with('.')
+        || email.bytes().any(|b| b.is_ascii_whitespace() || b.is_ascii_control())
+        || !(8..=128).contains(&form.password.len())
+    {
         return Err(StatusCode::UNPROCESSABLE_ENTITY);
     }
 
@@ -191,3 +201,47 @@
         assert_eq!(response.status(), StatusCode::FORBIDDEN);
     }
 }
+
+#[cfg(test)]
+mod day23_correction_tests {
+    use super::*;
+    use axum::{body::Body, http::Request};
+    use tower::ServiceExt;
+
+    #[tokio::test]
+    async fn day23_invalid_signup_is_rejected_before_db_write() {
+        let pool = sqlx::sqlite::SqlitePoolOptions::new().max_connections(1)
+            .connect("sqlite::memory:").await.unwrap();
+        sqlx::raw_sql(include_str!("../../../migrations/20261018073931_create_auth_user.sql"))
+            .execute(&pool).await.unwrap();
+        sqlx::raw_sql(include_str!("../../../migrations/20261019221857_member_identity.sql"))
+            .execute(&pool).await.unwrap();
+        let app = routes(pool.clone());
+        let good = "username=learner&email=learner%40example.test&display_name=Learner&password=12345678";
+        // DB 沒有既存帳號，422 不能靠重複 Email 的 409 掩蓋。
+        for bad in [
+            good.replace("username=learner", "username=x"),
+            good.replace("username=learner", "username=bad-name"),
+            good.replace("learner%40example.test", "not-an-email"),
+            good.replace("display_name=Learner", "display_name=%20%20"),
+            good.replace("password=12345678", "password=1234567"),
+            good.replace("username=learner", &format!("username={}", "x".repeat(33))),
+            good.replace("display_name=Learner", &format!("display_name={}", "x".repeat(33))),
+            good.replace("password=12345678", &format!("password={}", "x".repeat(129))),
+        ] {
+            let response = app.clone().oneshot(Request::post("/signup")
+                .header("origin", "http://127.0.0.1:3000")
+                .header("content-type", "application/x-www-form-urlencoded")
+                .body(Body::from(bad)).unwrap()).await.unwrap();
+            assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
+        }
+        let (count,): (i64,) = sqlx::query_as("SELECT count(*) FROM mbr_user").fetch_one(&pool).await.unwrap();
+        assert_eq!(count, 0);
+        // 下界與新版表單一致；保持 0.6 的自動隨機鹽 API。
+        let response = app.oneshot(Request::post("/signup")
+            .header("origin", "http://127.0.0.1:3000")
+            .header("content-type", "application/x-www-form-urlencoded")
+            .body(Body::from(good)).unwrap()).await.unwrap();
+        assert_eq!(response.status(), StatusCode::SEE_OTHER);
+    }
+}
```

候選實測：cargo test --locked -p server --features ssr：19 passed／0 failed／0 ignored。原 main：13 passed／1 failed。修正只在隔離目錄，未套用 main；完整 logs 在 docs/teaching/evidence/2026-10-20-day22-freeze/。

### migration 差異（待確認 DB 歷史）

沿用 main 現有 20261019221857_member_identity.sql；不要再新增舊範例的 20261020060000_member_identity.sql。20261018094242_create_ai_person.sql 被修改，從舊 main da515526 升級已重現 checksum mismatch；fresh DB 成功不證明舊庫可升級。下一步需人工核對實際 _sqlx_migrations 與備份，決定有效歷史基底後新增前向 migration，不改舊 checksum／刪庫／盲目重送。尚無足以套用全部使用者歷史的通用修補，不能標此項已解決。

### 第3／4節同步

第3節列 workspace0.2.22、Argon2 0.6.0／Tokio1.54.0 與當日官方版本；第4節分別列 main 失敗與隔離候選通過，保留缺少真瀏覽器／重啟／Session驗收。登入／授權主線仍未完成；不得用本單元代替當日新實作。
