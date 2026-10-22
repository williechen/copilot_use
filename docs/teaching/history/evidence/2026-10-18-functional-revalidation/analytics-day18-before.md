## 1. 學習大綱
目標：讓 Analytics 已登入使用者在有效 Session 內進行受保護讀取時延長伺服器端閒置期限；過期 Session 不得藉存取復活。此為 Day17 既有候選方案的可套用完整最小伺服器端 Green，**不宣稱瀏覽器 Cookie 已同步滑動續期**。主體 Red/Green 15 分鐘。
真實來源 rust_ai_chat/main 98f30d8b3593d80103b1a4bdbb9f0cb8cb08069e：apps/stats_server/src/[admin.rs:61](http://admin.rs:61)-72 定義 Session \{ expires, csrf \}、Admin.sessions；:78-91 cookie_key 驗證 analytics_session 64 字元十六進位並 hash；:93-97 csrf_for 只 retain 過期項後讀 csrf，不更新 expires；:213-228 login 寫 Session 並設定 Max-Age=1800；:286 起 dashboard、:360/369 API 透過 csrf_for 驗證。呼叫鏈：GET /dashboard 或受保護 API → csrf_for → cookie_key → sessions HashMap → Session.expires/CSRF → 後續 Dashboard/PG 查詢。來源服務 token 與人員 Session 分離。
## 2. 實作內容
### Red（先寫，尚未執行）
在 apps/stats_server/src/[admin.rs](http://admin.rs) 既有 tests 模組增加：
```rust
#[test]
fn day18_activity_extends_valid_session_only() {
    let now = Instant::now();
    let mut session = Session {
        expires: now + Duration::from_secs(5),
        csrf: "fixture".into(),
    };
    touch_session(&mut session, now + Duration::from_secs(2));
    assert_eq!(session.expires, now + Duration::from_secs(2) + TTL);
}
```
舊程式沒有 touch_session，預期先編譯失敗；這是設計的 Red，未執行。另以既有 `origin_and_expired_sessions_fail_closed` 回歸過期拒絕。
### 完整最小 Green
在 Session 定義後加入：
```rust
// 僅對仍有效且已通過 cookie 查找的 Session 延長閒置期限。
fn touch_session(session: &mut Session, now: Instant) {
    session.expires = now + TTL;
}
```
將原本 [admin.rs:93](http://admin.rs:93)-97 的 csrf_for 完整替換：
```rust
fn csrf_for(state: &Admin, headers: &HeaderMap) -> Option<String> {
    let key = cookie_key(headers)?;
    let now = Instant::now();
    let mut sessions = state.sessions.lock().ok()?;
    sessions.retain(|_, session| session.expires > now);
    let session = sessions.get_mut(&key)?;
    touch_session(session, now);
    Some(session.csrf.clone())
}
```
順序必須是先解析 Cookie → 清理過期 Session → 僅對存在且未過期的 Session 延長；未知、錯誤或過期 Cookie 返回 None。既有 login、logout 的 CSRF/origin 驗證、PostgreSQL summary、來源 token 全部保留。這是完整最小伺服器端切片；瀏覽器 Cookie 仍 Max-Age=1800，超過 30 分鐘可能消失，不可宣稱端到端滑動 Session。
### Refactor／補教
下一切片在受保護回應同步刷新 Cookie 並驗證安全屬性（HttpOnly、SameSite、Secure/HTTPS），再抽取 TTL 設定、明確規範讀取與登出是否刷新。Day17 原頁保留，Day18 補足明確的失效前檢查與實際受保護入口鏈。
## 3. 套件版本
apps/stats_server/Cargo.toml：axum、sqlx、tokio、uuid、sha2 取 workspace 版本，tower dev 0.5；不新增依賴。精確 Cargo.lock/MSRV 本次未核實。
## 4. 驗證項目
來源 SHA rust_ai_chat/main 98f30d8b3593d80103b1a4bdbb9f0cb8cb08069e（唯讀）；教學分支發布前 SHA bd613c51cb972311d2183241e7b30dca25d85da7；規則 daily-tdd-minimum-v1。
`cargo test -p stats-server day18_activity_extends_valid_session_only`：預期 Green 通過，實際未執行。`cargo test -p stats-server origin_and_expired_sessions_fail_closed`：預期過期 Session 仍被拒，實際未執行。HTTP：登入 → Dashboard → 有效 Session 讀取 → 伺服器端 expires 延後；逾時拒絕、logout 撤銷、CSRF/origin 拒絕；未執行。PostgreSQL 測試僅在具獨立 ANALYTICS_TEST_DATABASE_URL 時執行；未執行。CI：未取得匹配此教學 patch 的精確 workflow/job 成功證據，不標記通過。程式套用／產品整合／功能驗收均未核實。待完成：實際套用與測試、Cookie 同步滑動續期、端到端驗收。
