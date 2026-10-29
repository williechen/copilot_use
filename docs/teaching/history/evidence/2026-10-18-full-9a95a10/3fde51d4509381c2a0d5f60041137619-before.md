## 1. 學習大綱
本輪main `63f66e7742ecd09cd5199a47dbacc9df666af416`；候選 `3eace139da323fc906133636fae93e4d54159173` 未合併main。目標仍為登入後的活動Session切片：保留人員Session與來源服務身分分離，讓伺服器expires和瀏覽器Cookie期限一起更新。main新建PG migration、登入／query／CSRF／Origin／來源身分分離／冪等／logout撤銷通過；活動Cookie續期assert實際失敗。舊PG migration31→32升級與啟動仍VersionMissing。候選同步server expiry／Cookie後真實HTTP通過，短deadline TCP到期401／不復活通過；未合併main。帳密admin/admin表單仍422缺key，角色／scope／1800秒browser soak未完成或未驗證。
目前入口main會先PgPool連線並執行apps/stats_server/migrations，再建立admin::build Router。新建PG啟動已通過；SQLx舊migration31改名32的既有庫升級及啟動仍失敗。這不是教學發文阻擋，但必須保留為產品驗收缺口。
原Red／Green學習15分鐘；真實HTTP／PG及短deadline回歸另估20分鐘，角色、帳密與migration兼容另列原需求，沒有增加規則。
## 2. 實作內容
### 真實 Red
main `python3 day18_http.py analytics require-renewal` 實際exit1，COOKIE_RENEWED: False。csrf_for已有touch_session但只延長server expires，活動Dashboard的200回應沒有Set-Cookie Max-Age=1800。基礎HTTP腳本exit0只代表其他assert通過，不能冒稱這個Cookie gate通過。
### 完整最小 Green
在apps/stats_server/src/[admin.rs](http://admin.rs)，將main原本從`fn cookie_key`到`fn document`之前的helper區，完整換成下面程式。既有TTL=1800、hash、Session／Admin與origin_ok不變；cookie_value為新增，不與舊cookie_key重複保留。csrf_for只檢查並清除逾期，續期交給成功的受保護GET回應層，避免在handler尚未通過時提前刷新：
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
登入函式整個替換如下；Login表單仍是既有key原型，沒有假裝新增帳密系統：
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
    attempts.1 += 1;

    // Only accept the generated 256-bit login key, not a low-entropy password hashed with SHA256.
    if form.key.len() != 64
        || !form.key.bytes().all(|c| c.is_ascii_hexdigit())
        || !same(&hash(&form.key), &config.key_hash)
    {
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
Router建置函式完整改為以下，refresh layer只放在受保護頁面／查詢路由，登入、登出與機器上報維持各自邊界：
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
[以本次main為parent的完整patch](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-full-63f66e7/functional-fixes.patch)包含完整差異與真實TCP回歸測試。成功GET後再次確認Session仍存在且未過期，才同時touch與刷新同一Cookie；logout、已撤銷與過期不能被這層復活。這只是候選，main尚未套用。
### Refactor及舊Day更正
cookie_value解析只接受64位hex，cookie_key與Set-Cookie重用同一值，避免從Cookie名稱或未驗證輸入拼接header。Day17原98f30d8「沒有touch_session」保留歷史身分；現在不能重複新增該函式，也不能因server-only測試通過就宣稱瀏覽器生命周期完整。
PG migration31／32兼容、admin/admin首次改密碼、Email、角色／來源scope與立即縮權屬原產品待完成項目，候選沒有實作。root AI人物表在Chat SQLite，不能用它當成Analytics人員帳密已完成的證據。
## 3. 套件版本
本輪實際 Rust／Cargo 1.99.1；SQLx 0.9.0、Axum 0.8.9、Leptos 0.8.22、Tokio 1.53.3、Tauri 2.12.4、tauri-build 2.7.3、wasm-bindgen 0.2.129（Cargo.lock）。cargo-leptos 0.3.11、Trunk 0.21.14；Ubuntu 26.04.1、PostgreSQL 18.6。測試用 bundled Node 24.19.0，與先前 Node22 紀錄分開；未另驗跨平台／MSRV。候選沒有新增產品依賴。
## 4. 驗證項目
同完整workspace命令並啟用stats-server/postgres-integration、include-ignored：main126 passed／1 failed／0 ignored；候選128 passed／0 failed／0 ignored。Stats單元／所屬PG與契約21通過，已包含在workspace總數，不能再相加當唯一案例數。
main真實HTTP：匿名Dashboard303、匿名／服務token不能登入人員query401、Origin／錯誤CSRF403、正確key登入303、來源上報201／冪等200、PG summary200、logout303及舊Cookie401通過；活動Cookie assert失敗。候選同流程與require-renewal通過，COOKIE_RENEWED: True。
候選TCP測試跨初始2秒期限後仍200，再將測試Session deadline設30ms並實際等60ms，逾時回401且不刷新／不復活。生產TTL仍1800秒；這不是30分鐘瀏覽器soak。main已有expired-session檢查測試通過，但其server-only／router證據不能代替瀏覽器Cookie續期。
舊PG31基線先套用再跑最新migration，實際missing migration31；最新main啟動同DB回VersionMissing(20261018073931)。workspace-tests的analytics_postgres_ingest被未宣告feature擋住，直接啟用的命令exit101；該target的0 tests不算覆蓋，Stats套件本身的真PG測試已執行。
帳密表單username/password實測422缺key；角色／scope／立即停用及長連線撤銷未完成或未驗證，1800秒browser soak未執行。精確命令、來源SHA、CI job、HTTP、PG與剩餘gate見[本輪完整證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-full-63f66e7/REPORT.md)；正式完整功能維持未驗收。
最新main的17個CI jobs皆success；Test All/test-stats-server未開postgres-integration／include-ignored，不能代替本輪獨立PG、真實Cookie／升級測試；Rust Dashboard/build-dashboard為建置。精確job連結與時間見本輪報告。
