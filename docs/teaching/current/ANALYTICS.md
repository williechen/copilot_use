## 1. 學習大綱
本次更新核對（main `1853876e0eb1cfe8e4562be8b3c661b55f1bc54b`／候選 `80ba01e6d965d0e3ff10bef18a5cbd17f01e8f05`）：main 真實 HTTP／PG／Origin／CSRF／來源 token 分離／登出測試通過，但活動回應 COOKIE_RENEWED: False。舊 20261018073931 migration 已套用的隔離 DB 升級至 32 仍實際失敗 previously applied but is missing。新增 root SQLite auth／pet／ai_person schema 並沒有接上 Analytics 帳密／角色／來源 scope。候選 Cookie 修正未合併；完整產品未驗收。 精確命令與本輪結果見 [最新複驗證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-1853876/REPORT.md)。原版 794da6e／候選 8e84699 的 100 passed 是先前紀錄；本輪數字及尚未驗證項目以最新報告為準。
本次複驗 main `1853876e0eb1cfe8e4562be8b3c661b55f1bc54b`；修正候選 `80ba01e6d965d0e3ff10bef18a5cbd17f01e8f05` 尚未合併 main。Day18 原本的伺服器半切片已套用：csrf_for 先剔除過期 Session，再 touch_session 延長 expires。但登入 Cookie 只有固定 Max-Age=1800，受保護回應不刷新；因此「持續操作仍有效」的瀏覽器全流程尚未完成。
本篇直接統整已套用部分、實測 Red 與完整 Cookie 同步候選，不改成新規則、不增加 Day。原主體學習 15 分鐘；Cookie 補教及 HTTP／PG 回歸另估 25 分鐘；跨 1800 秒的瀏覽器 soak 另估至少 35 分鐘，尚未執行。
## 2. 實作內容
### Red：本次已實際執行
用獨立 PostgreSQL 與合成登入碼啟動 main 的重建 binary，登入 → Dashboard 200 → 受保護查詢 200，但活動回應沒有 Set-Cookie。`python3 scripts/acceptance/day18_http.py analytics require-renewal` 在 main 因「valid activity did not renew browser cookie」失敗。既有 day18_activity_extends_valid_session_only 與過期拒絕測試通過，僅證明 server 半邊，不能冒充 Cookie 全流程 Red/Green。
### 完整 Green：受保護成功 GET 同步兩個期限的候選
main 的 Session／TTL／touch_session 保留；將 cookie_key／csrf_for 的區段換成下列完整內容，並新增同區域的 session_cookie／refresh_session。csrf_for 現在只做驗證，續期集中在成功回應；失敗／過期／撤銷不復活，不處理來源服務 token：
```rust
fn cookie_value(headers: &HeaderMap) -> Option<&str> {
    let value = headers
        .get(header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .filter_map(|v| v.trim().split_once('='))
        .find(|(k, _)| *k == "analytics_session")?
        .1;
    if value.len() != 64 || !value.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    Some(value)
}

fn cookie_key(headers: &HeaderMap) -> Option<String> {
    cookie_value(headers).map(hash)
}

fn csrf_for(state: &Admin, headers: &HeaderMap) -> Option<String> {
    let key = cookie_key(headers)?;
    let now = Instant::now();
    let mut sessions = state.sessions.lock().ok()?;
    sessions.retain(|_, session| session.expires > now);
    let session = sessions.get(&key)?;
    Some(session.csrf.clone())
}

// Refresh both expiry clocks only after a successful protected GET. Logout and
// login never pass through this layer. Recheck after the handler so a revoked or
// expired session cannot be resurrected by an in-flight response.
fn session_cookie(raw: &str) -> axum::http::HeaderValue {
    format!(
        "analytics_session={raw}; HttpOnly; SameSite=Strict; Path=/; Max-Age={}",
        TTL.as_secs()
    )
    .parse()
    .expect("validated hex session cookie")
}

async fn refresh_session(
    State(state): State<Admin>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    let raw = if request.method() == axum::http::Method::GET {
        cookie_value(request.headers()).map(str::to_owned)
    } else {
        None
    };
    let mut response = next.run(request).await;
    if response.status().is_success() {
        if let Some(raw) = raw {
            if let Ok(mut sessions) = state.sessions.lock() {
                let now = Instant::now();
                sessions.retain(|_, session| session.expires > now);
                if let Some(session) = sessions.get_mut(&hash(&raw)) {
                    touch_session(session, now);
                    response
                        .headers_mut()
                        .insert(header::SET_COOKIE, session_cookie(&raw));
                }
            }
        }
    }
    response
}
```
login 的既有 sessions.insert 完成後，回應結尾改為：
```rust
let mut response = Redirect::to("/dashboard").into_response();
response.headers_mut().insert(header::SET_COOKIE, session_cookie(&raw));
response
```
將 build 的完整函式替換為下列內容，讓 login/logout 位於續期 layer 之外；logout 原有 Origin／CSRF 檢查、刪除 Session、Max-Age=0 全保留：
```rust
pub fn build(ingest: IngestState, config: Option<Config>) -> Router {
    let admin = Admin {
        ingest: ingest.clone(),
        config,
        sessions: Arc::new(Mutex::new(HashMap::new())),
        attempts: Arc::new(Mutex::new((Instant::now(), 0))),
    };
    let protected = Router::new()
        .route("/", get(dashboard))
        .route("/dashboard", get(dashboard))
        .route("/api/v1/events/{source}/summary", get(summary))
        .route(
            "/api/v1/events/{source}/summary/{event_type}",
            get(kind_summary),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            admin.clone(),
            refresh_session,
        ))
        .route("/login", get(login_page).post(login))
        .route("/logout", post(logout))
        .with_state(admin);

    Router::new()
        .route("/healthz", get(crate::health))
        .route("/api/v1/events", post(crate::ingest))
        .with_state(ingest)
        .merge(protected)
        .layer(DefaultBodyLimit::max(4096))
        .layer(axum::middleware::map_response(
            |mut response: Response| async move {
                response
                    .headers_mut()
                    .insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
                response
            },
        ))
}
```
[完整可套用 patch](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-1853876/day18-functional-fixes.patch) 與其中 apps/stats_server/src/[admin.rs](http://admin.rs) 的 TCP 回歸測試包含全部變更。沒有新增帳密／角色 API，也沒有把 key 登入改稱 admin/admin 帳密登入。
### Refactor／舊 Day 更正補充
Day17／Day18 原 server-only Green 屬已套用半切片；Cookie 同步是尚未合併的候選。成功受保護 GET 才更新 server expires 和 Cookie Max-Age，讀同一 TTL；未知、格式錯誤、過期 Cookie 不刷新，登出仍刪除 Cookie。HTTPS Secure、可設定閒置期限、帳號／密碼／撤銷與 SSE 權限仍是既有產品缺口，不由此片段推定完成。
## 3. 套件版本
Rust 1.99.1／Cargo 1.99.1；Cargo.lock：Axum 0.8.9、Leptos 0.8.22、SQLx 0.9.0、Tokio 1.53.3、Tauri 2.12.4、tauri-build 2.7.3、wasm-bindgen 0.2.129、web-sys 0.3.106、Tower 0.5.3。Ubuntu 26.04.1／PostgreSQL 18.6／Node 22.22.1／cargo-leptos 0.3.11／Trunk 0.21.14。本次不新增產品依賴；MSRV／跨平台未另行驗證。 Analytics 是 native Leptos SSR／Axum，不是本次 Chat 的 WASM；本機只使用 HTTP loopback，沒有宣稱 HTTPS Secure Cookie 已驗證。
## 4. 驗證項目
main：14 單元＋2 idempotency 契約通過；獨立 ANALYTICS_TEST_DATABASE_URL 的 login_query_postgres_and_logout_flow 本輪再次通過。真實 HTTP 已測匿名 Dashboard 303、匿名／來源 token 代人員 API 401、Origin 403、登入 303、提交 201／重送 200、PG 查詢 200、非法來源 400、錯 CSRF 403、登出 303、舊 Cookie 401；活動 Cookie 續期失敗。
候選：先前及本輪的精確命令與結果列於 [本次正式複驗與原始證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-1853876/REPORT.md)；完整回歸 105 passed／1 ignored，另開 postgres-integration 並 include-ignored 的獨立 PG suite 22 passed／0 ignored。TCP 測試跨初始 2 秒 deadline 活動仍 200，再用測試專用 30ms idle deadline、真實等待 60ms 後回 401，不刷新且不復活；生產 TTL 仍 1800 秒。候選 HTTP Cookie／CSRF／登出通過，不等於 main 已修正。
最新 main 另將 migration 20261018073931 改名為 20261018073932；本輪先套用舊版本再執行新版本，實測失敗「migration 20261018073931 was previously applied but is missing in the resolved migrations」。新建 DB 通過不代表既有 DB 可升級；這項兼容修正尚未完成，也不在 Cookie 候選 patch 中。既有 31 檔與 checksum 需要恢復並依是否已套用 32 處理兼容，兩種既有資料庫路徑仍需重測，沒有操作正式 DB 的 migration 記錄。
未執行 30 分鐘瀏覽器 soak；帳密 admin/admin、首登改密碼／Email、加鹽雜湊持久化、角色／來源 scope、立即停用／縮權、逐帳號等待、重設密碼與 SSE 存續撤銷未完成或未驗證。正式完整功能驗收維持未通過。
