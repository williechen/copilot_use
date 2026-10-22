# Analytics Day19 修正版：成功登入不累加失敗額度
## 1. 學習大綱
來源 `rust_ai_chat/main=f692c2ebe7b414fe49858c1ab1c4a33ce7c3dfee`；教學分支 `44dabb3eebed12639cd949b93521cbdc47a401fb`。本篇為本地查核修正版，未寫回 Notion、未發布、未推進 Day。主體 Red 5 分鐘、Green 10 分鐘，Refactor／額外驗證另計；實際測試結果見 `驗證結果.md`。完整修改檔在 `source/`，patch 基底為上述來源 SHA。

目標：同一60秒固定窗口內，11次合法登入均303；10次錯誤key均401，之後429。成功既不累加也不清零先前失敗；已滿10次時正確key仍429，直到窗口重置。這是既有全域單一額度，不是每帳號／IP，也不是滾動時間窗。

呼叫鏈：build→POST /login→Axum Form extractor→handler內Origin檢查→Config存在性→attempts Mutex／窗口／門檻→64字元hex及SHA256比較→失敗累加／成功Session→Set-Cookie與/dashboard redirect。錯誤Form可能在handler前先被extractor拒絕，不能聲稱所有請求都先檢查Origin。原程式在key驗證前對成功也累加。前置沿用測試app()／request()，不用真實秘密或DB；不是admin/admin帳密或角色實作。
## 2. 實作內容
### Red
在 `apps/stats_server/src/admin.rs::tests` 加入以下兩個測試，沿用既有 imports、app()、request()，不另開重複tests模組。先不修改login，實際重現第11次成功429，以及9錯→成功→再錯過早429。

```rust
#[tokio::test]
    async fn day19_successful_logins_do_not_consume_failure_budget() {
        let router = app();
        for attempt in 1..=11 {
            let response = router.clone().oneshot(request(
                "POST", "/login", &format!("key={}", "a".repeat(64)),
            )).await.unwrap();
            assert_eq!(response.status(), StatusCode::SEE_OTHER, "success {attempt}");
            assert_eq!(response.headers()[header::LOCATION], "/dashboard");
            let cookie = response.headers()[header::SET_COOKIE].to_str().unwrap();
            for attribute in ["analytics_session=", "HttpOnly", "SameSite=Strict", "Path=/", "Max-Age=1800"] {
                assert!(cookie.contains(attribute), "missing {attribute}");
            }
        }
        for attempt in 1..=10 {
            let response = router.clone().oneshot(request(
                "POST", "/login", &format!("key={}", "b".repeat(64)),
            )).await.unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "failure {attempt}");
            assert!(!response.headers().contains_key(header::SET_COOKIE));
        }
        for key in ["b".repeat(64), "a".repeat(64)] {
            let response = router.clone().oneshot(request("POST", "/login", &format!("key={key}"))).await.unwrap();
            assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
            assert!(!response.headers().contains_key(header::SET_COOKIE));
        }
    }

    #[tokio::test]
    async fn day19_success_does_not_reset_previous_failures() {
        let router = app();
        for _ in 0..9 {
            assert_eq!(router.clone().oneshot(request("POST", "/login", "key=wrong")).await.unwrap().status(), StatusCode::UNAUTHORIZED);
        }
        assert_eq!(router.clone().oneshot(request("POST", "/login", &format!("key={}", "a".repeat(64)))).await.unwrap().status(), StatusCode::SEE_OTHER);
        assert_eq!(router.clone().oneshot(request("POST", "/login", "key=wrong")).await.unwrap().status(), StatusCode::UNAUTHORIZED);
        assert_eq!(router.oneshot(request("POST", "/login", "key=wrong")).await.unwrap().status(), StatusCode::TOO_MANY_REQUESTS);
    }
```

### 完整最小 Green
以以下整個函式替換同檔login；其餘hash/same、Config、Session、Cookie、Origin及路由沿用原碼。唯一行為修正是將attempts.1累加移到key驗證失敗分支，保留Mutex內檢查與累加的原子範圍。

```rust
async fn login(
    State(state): State<Admin>,
    headers: HeaderMap,
    Form(form): Form<Login>,
) -> Response {
    if !origin_ok(&headers) {
        return StatusCode::FORBIDDEN.into_response();
    }

    let Some(config) = &state.config else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    let mut attempts = match state.attempts.lock() {
        Ok(v) => v,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };

    if attempts.0.elapsed() >= Duration::from_secs(60) {
        *attempts = (Instant::now(), 0);
    }

    if attempts.1 >= 10 {
        return StatusCode::TOO_MANY_REQUESTS.into_response();
    }

    // Only accept the generated 256-bit login key, not a low-entropy password hashed with SHA256.
    if form.key.len() != 64
        || !form.key.bytes().all(|c| c.is_ascii_hexdigit())
        || !same(&hash(&form.key), &config.key_hash)
    {
        attempts.1 += 1;
        return (StatusCode::UNAUTHORIZED, "登入失敗，請返回登入頁重試").into_response();
    }

    drop(attempts);

    let raw = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());

    let mut sessions = match state.sessions.lock() {
        Ok(s) => s,
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    sessions.retain(|_, s| s.expires > Instant::now());
    if sessions.len() >= 1024 {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    if let Some(old) = cookie_key(&headers) {
        sessions.remove(&old);
    }
    sessions.insert(
        hash(&raw),
        Session {
            expires: Instant::now() + TTL,
            csrf: Uuid::new_v4().simple().to_string(),
        },
    );

    let mut response = Redirect::to("/dashboard").into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, session_cookie(&raw));
    response
}
```

### Refactor
本切片不再抽額外架構；保留驗證運算與回應建立順序、drop(attempts)、舊Session替換及Cookie契約。成功不重置額度，避免有人以合法登入重置共用限制；窗口逾60秒的重置原碼沿用，時間經過與併發行為本輪未新增實測。不要記錄key或hash。全域額度與Session1024容量仍是後續問題，不因11次成功測試而聲稱可無限登入。
## 3. 套件版本
Cargo.lock：axum0.8.9、sqlx0.9.0、tokio1.53.3、sha2直接依賴0.11.1（lock另有間接0.10.9）、serde1.0.229、tower0.5.3、leptos0.8.22。
本輪 Linux x86_64，rustc 1.99.1／cargo 1.99.1、stable 工具鏈，WASM target 已安裝。專案未宣告 rust-version，最低 MSRV 未驗證；其他作業系統未驗證。無新增套件或 migration，Cargo.lock 未改。

## 4. 驗證項目
```sh
cargo test --locked -p stats-server day19
cargo test --locked -p stats-server
```
測試使用既有connect_lazy的不可連線fixture地址；登入路徑不需查PG，並檢查303 Location、Cookie屬性、401／429不發Cookie，以及成功不清零既有錯誤。它是Axum oneshot測試，並非真TCP／真PostgreSQL或瀏覽器驗收。完整PG整合、真HTTP、60秒實時等待、併發及cookie長時間存活未測。既有Origin／CSRF回歸測試結果見驗證表。來源Dashboard CI是check/build，Test All另有stats測試，都不含尚未提交的Day19候選。
