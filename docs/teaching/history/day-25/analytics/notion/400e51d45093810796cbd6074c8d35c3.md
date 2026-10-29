## 1. 學習大綱／實際狀態

Analytics Day 25 新原教學：已完成首次改密碼的管理員，仍可從 Dashboard 的「修改密碼與Email」連結再次進入表單、送出新密碼及 Email，完成後必須重新登入。這是從真實使用者流程出發，不重建管理員 bootstrap，也不重做已存在的 PostgreSQL Session。

來源 rust\_ai\_chat/main 08b891a22c1934a2bf0b1edbb103c9b16bcf12de，apps/stats\_server/src/password\_[login.rs:223](http://login.rs:223)-264。現有 change\_page 在第228行對 !restricted 直接 Redirect::to("/dashboard")，但 Dashboard 第383行仍有指向 /change-password 的連結；因此首次強制改密碼成功後，使用者無法再次開啟變更表單。既有 change\_password POST handler 已驗當前密碼、Argon2雜湊、新密碼與 Email、CSRF、PG transaction，成功後會撤銷該管理員所有 Session。從最早缺失的 GET 入口完成一個可用的最小 Green，不修改已封存 Day24。

預期 15 分鐘完成學習主體；Refactor、PG 環境設定及驗收另計。資料庫是 PostgreSQL，絕對不可用 Chat/Pet 的 SQLite fixture 測試或刪除正式資料。

## 2. Red → 完整 Green → Refactor；FIFO Day24 補教

<b>Red：新增整個檔案 apps/stats\_server/tests/day25\_repeat\_password\_</b>[<b>change.rs</b>](http://change.rs)。這是使用既有 password\_login::build 真 HTTP Router、真 PostgreSQL 連線、真 password\_login endpoint 的整合切片，先驗證第一次更換，再對使用新密碼的登入者開啟第二次表單與成功提交；既有 main 第二次 GET 會回 303 而非 200。測試只在隔離測試資料庫執行。

```rs
#![cfg(feature = "postgres-integration")]

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode, header},
};
use stats_server::{IngestState, IngestTokens, password_login};
use std::sync::Arc;
use tower::ServiceExt;

fn app(pool: sqlx::PgPool) -> axum::Router {
    password_login::build(IngestState {
        pool,
        tokens: Arc::new(IngestTokens {
            chat: "fixture-chat".into(),
            pet: "fixture-pet".into(),
        }),
    })
}
async fn request(
    app: axum::Router,
    method: &str,
    path: &str,
    form: Option<String>,
    cookie: Option<&str>,
) -> axum::response::Response {
    let mut builder = if method == "POST" {
        Request::post(path)
            .header(header::ORIGIN, "http://127.0.0.1:3100")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
    } else {
        Request::get(path)
    };
    if let Some(cookie) = cookie {
        builder = builder.header(header::COOKIE, cookie);
    }
    app.oneshot(builder.body(form.map_or_else(Body::empty, Body::from)).unwrap())
        .await.unwrap()
}
fn cookie(response: &axum::response::Response) -> String {
    response.headers()[header::SET_COOKIE]
        .to_str().unwrap().split(';').next().unwrap().to_owned()
}
async fn csrf(response: axum::response::Response) -> String {
    let html = String::from_utf8(
        to_bytes(response.into_body(), 16000).await.unwrap().to_vec()
    ).unwrap();
    html.split("name=\"csrf\" value=\"")
        .nth(1).unwrap().split('"').next().unwrap().to_owned()
}

#[sqlx::test(migrations = "./migrations")]
async fn day25_admin_can_change_password_again_after_initialization(pool: sqlx::PgPool) {
    stats_server::bootstrap::initialize(&pool, "lesson-first-password".into())
        .await.unwrap();
    let first = request(app(pool.clone()), "POST", "/login",
        Some("username=admin&password=lesson-first-password".into()), None).await;
    assert_eq!(first.status(), StatusCode::SEE_OTHER);
    assert_eq!(first.headers()[header::LOCATION], "/change-password");
    let c1 = cookie(&first);
    let initial_page = request(app(pool.clone()), "GET", "/change-password",
        None, Some(&c1)).await;
    assert_eq!(initial_page.status(), StatusCode::OK);
    let token1 = csrf(initial_page).await;
    let initial_form = format!(
        "current_password=lesson-first-password&new_password=lesson-second-password&confirm_password=lesson-second-password&email=first%40example.test&csrf={token1}"
    );
    let changed1 = request(app(pool.clone()), "POST", "/change-password",
        Some(initial_form), Some(&c1)).await;
    assert_eq!(changed1.status(), StatusCode::SEE_OTHER);

    let second_login = request(app(pool.clone()), "POST", "/login",
        Some("username=admin&password=lesson-second-password".into()), None).await;
    assert_eq!(second_login.status(), StatusCode::SEE_OTHER);
    assert_eq!(second_login.headers()[header::LOCATION], "/dashboard");
    let c2 = cookie(&second_login);
    // Old main returns 303 to /dashboard instead of rendering change form.
    let second_page = request(app(pool.clone()), "GET", "/change-password",
        None, Some(&c2)).await;
    assert_eq!(second_page.status(), StatusCode::OK);
    let token2 = csrf(second_page).await;
    let next_form = format!(
        "current_password=lesson-second-password&new_password=lesson-third-password&confirm_password=lesson-third-password&email=second%40example.test&csrf={token2}"
    );
    let changed2 = request(app(pool.clone()), "POST", "/change-password",
        Some(next_form), Some(&c2)).await;
    assert_eq!(changed2.status(), StatusCode::SEE_OTHER);
    assert_eq!(changed2.headers()[header::LOCATION], "/login");
    let stored_email: String = sqlx::query_scalar(
        "SELECT user_email FROM uac_user WHERE lower(user_acnt)='admin'"
    ).fetch_one(&pool).await.unwrap();
    assert_eq!(stored_email, "second@example.test");
    let sessions: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM uac_login_session"
    ).fetch_one(&pool).await.unwrap();
    assert_eq!(sessions, 0);
    assert_eq!(
        request(app(pool.clone()), "POST", "/login",
            Some("username=admin&password=lesson-second-password".into()), None)
            .await.status(),
        StatusCode::UNAUTHORIZED
    );
    let third = request(app(pool), "POST", "/login",
        Some("username=admin&password=lesson-third-password".into()), None).await;
    assert_eq!(third.status(), StatusCode::SEE_OTHER);
    assert_eq!(third.headers()[header::LOCATION], "/dashboard");
}
```

<b>完整最小 Green：只修改 apps/stats\_server/src/password\_</b>[<b>login.rs</b>](http://login.rs)<b> 現有 change\_page function</b>，其他 session、CSRF、Argon2、PG transaction、重設 Session 語意一律沿用。

找到：

```rs
    let (restricted, raw) = session(&state, &headers).await?;
    if !restricted {
        return Ok(renewed(Redirect::to("/dashboard"), &raw));
    }

    let csrf = digest(&format!("change-password:{raw}"));
```

替換為：

```rs
    // 首次登入和後續主動修改均須是有效登入者，
    // Session 再驗仍由 session() 完成。
    let (restricted, raw) = session(&state, &headers).await?;
    let csrf = digest(&format!("change-password:{raw}"));
```

<b>這是整個 Green 差異</b>，因為既有 change\_page 下半段已根據 restricted 顯示「首次登入須更換密碼與Email」或「修改密碼與Email」；既有 POST 在真 PostgreSQL transaction 內驗證目前密碼，更新 Argon2 密碼雜湊與 Email，刪除原管理員 Session 並轉回 /login。不要重新新增平行 handler、migration 或信箱欄位，也不要跳過 session() 而使未登入者得以使用表單。

Refactor：在另一切片把管理員變更個資、首次強制改密碼與 Session 行為抽成可測試服務；但抽象化不能改變此日最小可用 HTTP 行為。未完成：真實舊 PostgreSQL migration checksum 升級、管理員自助通知、Email 驗證、角色權限管理、完整瀏覽器/UI、正式 TLS。教學 Red/Green 可套用，但本次不宣稱已在產品跑過。

<b>FIFO：Day24 舊日補教（置於新 Day25 Red/Green/Refactor 後）</b>。Day24 的 apps/stats\_server/tests/day24\_password\_[change.rs](http://change.rs) 在 main 引用 ../migrations/20261021071000\_unique\_account\_email.sql，該檔不存在，完整 postgres-integration test 會在編譯期失敗。此問題與本日「已初始化者再次開表單」不同，不可偽裝成 Day25 原始 Red。要補正 Day24 的 schema 假設，先在隔離 PostgreSQL 上確認既有大小寫唯一 Email index 的實際定義，再選擇新增 forward migration，使舊資料不刪除、衝突保留供人工處理；不能修改歷史 20261018073932\_create\_auth\_user.sql 的 checksum 來掩蓋舊庫 VersionMismatch。舊測試 fixture 尚有 SQL 欄位引號錯誤，應另在新教學補上修正 diff 與 log；本篇的 Day25 測試刻意放在獨立的 test target，避免借用尚不能編譯的 Day24 測試。這些舊問題 <b>尚未實際修復或驗收</b>，僅作已發布的更正教學紀錄；不改封存 Day24 原文。

## 3. Cargo 套件、資料庫相容性

來源 workspace 0.2.23、edition 2024；apps/stats\_server/Cargo.toml 引入 axum 0.8、argon2 0.6、sqlx 0.9、tokio 1.54、sha2 0.11、Leptos SSR 0.8、rpassword 7.5、uuid 1.28；測試 dev 依賴 tower 0.5 util、serde\_json.workspace。本日無新 dependency/DB schema，不更改 Cargo.lock 或正式資料庫。PG 版本 18.6 僅為前次隔離驗證環境，不代表 production。

## 4. 命令、預期／實際、CI 與限制

```bash
# 在隔離測試 PostgreSQL、具備 DATABASE_URL 與 sqlx 測試權限下
cargo test --locked -p stats-server --features postgres-integration --test day25_repeat_password_change -- --nocapture
cargo check --locked -p stats-server
cargo fmt --all -- --check
# 此 Day24 既有 test target 有缺檔，須獨立處理，勿誤報全部通過：
cargo test --locked -p stats-server --features postgres-integration --test day24_password_change --no-run
```

預期 Red：第一次登入、首次修改成功，第二次登入成功，但第二次 GET /change-password 回 303 而測試要求 200；Green：第二次 GET 回 200、有新 CSRF 表單、第二次 POST 303、Email 更新、Session 全撤銷、舊密碼 401、新密碼可登入。

<b>本輪 Day25 全部命令未實際執行</b>，沒有新的 PG integration log、CI、程式部署與產品驗收。既有 main SHA 08b891a22c1934a2bf0b1edbb103c9b16bcf12de 的 [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39595880206) completed/success；[Rust Dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39595880453) completed/success；[Node](https://github.com/williechen/rust_ai_chat/actions/runs/39595880309) failure。Test All 未包含新增 Day25 test target；前次 [本地報告](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-21-day24-main-08b891a/REPORT.md) PG 可執行子集 28 passed，但完整 PG feature 測試因 Day24 include\_str 檔案缺失而未能編譯；舊庫升級 VersionMismatch(20261018073932) 未解決。

驗收結論：本次只正式交付原始 Day25 TDD 程式教學與 FIFO 更正紀錄，產品程式未套用、Red/Green 未執行、功能驗收 false。不得以成功發布 Notion 取代真 PostgreSQL、Session 及前後端的實際驗收。

### Day25 需求覆核更正

登入、登出、Cookie 與 Session 必須共用一套實作；Analytics 以 PostgreSQL 儲存介面及管理員政策接入，共用程式不代表與 Chat 共用登入狀態。本文只移除 change\_page 的提早跳轉，尚未完成共用套件要求。

Day24 缺失 SQL 檔造成的完整 feature 編譯失敗、舊庫 checksum 升級與資料保留驗證仍待修復；單獨測試 Day25 target 不足以宣稱全部測試通過。需提供修正差異與真 PostgreSQL 的 Red／Green、完整 feature 編譯及舊資料升級紀錄。

目前僅文章發布及全文讀回完成，Red／Green 與功能驗收均未完成；Day26 不代表上述要求已驗收，不得據此繼續遞增或恢復排程。Day24 及以前保持封存。