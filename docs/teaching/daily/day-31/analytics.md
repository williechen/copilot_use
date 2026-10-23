# Analytics Day31 r1｜將首次與日後改密碼頁接到 Leptos SSR

## ① 學習大綱、來源與功能解說

本次由最新進度取得 `next_day.analytics=31`，依 Chat→Pet→Analytics 交付。文件庫 `williechen/copilot_use/main` 初讀 SHA `259a1e6f08a6522ef596fe1448af4ac2bed56167`；唯一程式來源為唯讀 `williechen/rust_ai_chat/main@1d1866dbbb30255fafa4c7f74b1868919662913c`。規則 `daily-tdd-minimum-v1 / github-only-v1`，發布前再次取兩庫 SHA、規則 blob、當案 state／manifest；實際 parent 與讀回結果另存證據。

上一篇的 Leptos 登入頁已套用，本日不重教。`password_login::build` 已將 `/change-password` 接到 GET `change_page`、POST `change_password`；GET 仍以 `Html(format!(...))` 寫整頁，label 也沒有對應 input id。既有 POST 已核對 Origin、CSRF、目前密碼、新密碼確認、唯一 Email，交易更新 PostgreSQL 後撤銷 Session。因此今天只遷移這個真正的操作頁，不重建認證流程。

完整鏈：瀏覽器首次登入或 Dashboard 修改密碼連結 → Axum `change_page` → 原 `session()` 重驗 PostgreSQL → Leptos `ChangePasswordView` → 原生 POST `/change-password` → 既有密碼使用案例／SQLx transaction → Session 撤銷 → 重新登入。UI 不存取 pool、不持有服務 token。此路徑目前認證與 SQLx 尚集中在 password_login 模組；這是來源事實，沒有虛構已抽離的 Application Port，也不為檔名一致新建骨架。

本切片重用既有 SSR module，讓首次與後續修改都可在 Leptos 畫面操作並有可關聯的標籤；不是新的 Dashboard 篩選功能。現行 bootstrap 要求至少8 bytes 的互動密碼，login 也拒絕5 bytes 的 admin，故需求中的自動 `admin/admin` 初始化仍有差異，未由此 UI 遷移解決；管理員建帳／角色／來源 ACL 等另留原待辦。

核心教學 15 分鐘，三案各15＋共用15共60；版本、補教、文件交付與 DB／瀏覽器驗收另計。此環境 `rustc --version` 回 `/bin/bash: line 1: rustc: command not found`（exit127），因此本次未套用 Green、未執行 Red 或 Rust／PostgreSQL／瀏覽器驗證。

## ② Red → 完整最小 Green → Refactor → 必要補充

### Red A：先測真正的 router 頁面

在 `apps/stats_server/tests/day24_password_change.rs` 尾端新增以下測試。沿用該檔原有 `app`、`send`、`cookie` fixture 以及 feature gate。來源已有的表單雖可送出，但沒有 `for="change-current"` 和對應 id，故預期此測試在 HTML 斷言失敗，屬行為 Red；本次未執行。

```rust
#[sqlx::test(migrations = "./migrations")]
async fn day31_change_page_has_connected_labels_and_existing_csrf(pool: sqlx::PgPool) {
    stats_server::bootstrap::initialize(&pool, "lesson-only-password".into())
        .await.unwrap();
    let login = send(app(pool.clone()), "/login",
        Some("username=admin&password=lesson-only-password"), None).await;
    assert_eq!(login.status(), StatusCode::SEE_OTHER);
    let c = cookie(&login);
    let page = send(app(pool.clone()), "/change-password", None, Some(&c)).await;
    assert_eq!(page.status(), StatusCode::OK);
    assert!(page.headers().contains_key(header::SET_COOKIE));
    let bytes = axum::body::to_bytes(page.into_body(), 16000).await.unwrap();
    let html = String::from_utf8(bytes.to_vec()).unwrap();
    assert_eq!(html.matches("<!doctype html>").count(), 1);
    assert!(html.contains("for=\"change-current\""));
    assert!(html.contains("id=\"change-current\""));
    assert!(html.contains("action=\"/change-password\""));
    assert!(html.contains("首次登入須更換密碼與Email"));
    let csrf = html.split("name=\"csrf\" value=\"").nth(1)
        .unwrap().split('"').next().unwrap();
    assert_eq!(csrf.len(), 64);
    assert!(csrf.bytes().all(|b| b.is_ascii_hexdigit()));
    // 沒有 Cookie 的同一個正式 router，仍不可取得表單。
    assert_eq!(send(app(pool), "/change-password", None, None).await.status(),
        StatusCode::UNAUTHORIZED);
}
```

### Red B：SSR 元件與 escape 契約

在 `apps/stats_server/src/dashboard_ui.rs` 尾端新增；Green 前因 `render_change_password` 不存在而預期編譯失敗，與 Red A 的行為失敗分開記錄。

```rust
#[cfg(test)]
mod day31_tests {
    #[test]
    fn change_form_keeps_wire_names_and_escapes_attribute_values() {
        let html = super::render_change_password(true, "a\" onfocus=\"bad".into());
        assert_eq!(html.matches("<!doctype html>").count(), 1);
        for name in ["current_password", "new_password", "confirm_password", "email", "csrf"] {
            assert!(html.contains(&format!("name=\"{name}\"")));
        }
        assert!(html.contains("&quot;"));
        assert!(!html.contains("value=\"a\" onfocus="));
        assert!(html.contains("首次登入須更換密碼與Email"));
        let ordinary = super::render_change_password(false, "test-csrf".into());
        assert!(ordinary.contains("修改密碼與Email"));
        assert!(!ordinary.contains("首次登入須更換密碼與Email"));
        assert!(!ordinary.contains("lesson-only-password"));
    }
}
```

### Green A：新增完整 Leptos 元件

在 `apps/stats_server/src/dashboard_ui.rs` 尾端加入以下全文，沿用檔案已有的 `use leptos::prelude::*;`。LoginView、DashboardView 與原測試保留；不重複加 doctype、不新增 crate。

```rust
#[component]
fn ChangePasswordView(restricted: bool, csrf: String) -> impl IntoView {
    let heading = if restricted {
        "首次登入須更換密碼與Email"
    } else { "修改密碼與Email" };
    view! {
        <html lang="zh-Hant">
            <head><meta charset="utf-8"/><title>"修改密碼"</title></head>
            <body><main>
                <h1>{heading}</h1>
                <form method="post" action="/change-password">
                    <input type="hidden" name="csrf" value=csrf/>
                    <p>
                        <label for="change-current">"目前密碼"</label>
                        <input id="change-current" type="password" name="current_password"
                            autocomplete="current-password" required maxlength="128"/>
                    </p>
                    <p>
                        <label for="change-new">"新密碼"</label>
                        <input id="change-new" type="password" name="new_password"
                            autocomplete="new-password" required minlength="8" maxlength="128"/>
                    </p>
                    <p>
                        <label for="change-confirm">"確認新密碼"</label>
                        <input id="change-confirm" type="password" name="confirm_password"
                            autocomplete="new-password" required minlength="8" maxlength="128"/>
                    </p>
                    <p>
                        <label for="change-email">"唯一Email"</label>
                        <input id="change-email" type="email" name="email"
                            autocomplete="email" required maxlength="254"/>
                    </p>
                    <button type="submit">"儲存並重新登入"</button>
                </form>
                <p>"成功後所有裝置都須重新登入。Email不驗證；寄信重設尚未接通。"</p>
                <form method="post" action="/logout">
                    <button type="submit">"登出"</button>
                </form>
            </main></body>
        </html>
    }
}

pub fn render_change_password(restricted: bool, csrf: String) -> String {
    format!("<!doctype html>{}", view! {
        <ChangePasswordView restricted csrf/>
    }.to_html())
}
```

### Green B：完整替換 Axum GET handler

在 `apps/stats_server/src/password_login.rs`，僅完整替換 `change_page` 函式：

```rust
async fn change_page(
    State(state): State<IngestState>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    let (restricted, raw) = session(&state, &headers).await?;
    let csrf = digest(&format!("change-password:{raw}"));
    Ok(renewed(
        Html(crate::dashboard_ui::render_change_password(restricted, csrf)),
        &raw,
    ))
}
```

所有 POST 使用的欄位名、CSRF purpose、Origin、Cookie 續期和首次／日後修改區分保持原契約。未登入仍在渲染前被 session 拒絕，不先輸出受保護內容。`renewed` 續期使用原始 bearer，但只把既有 HttpOnly Cookie 設回 response；元件只收到摘要 CSRF，不傳 raw token 或 hash。伺服器 8..128 bytes 規則仍是安全界線，HTML minlength 只是輔助，中文 bytes 邊界仍需測。

### Refactor

頁面與 label/input 規則集中於 Leptos，Axum 不再維護完整改密碼 HTML 字串。只用已有的 `.to_html()` SSR 路徑，不引入 server functions／hydrate／JS 第二份表單；目前 analytics 是原生表單多頁流程，之後互動篩選另作增量。既有 POST SQLx transaction、Argon2 與 session policy 未改；沒有藉 UI 遷移宣稱分層或共用認證已完成。

### 上一 Day 未完成項目補充

Day30 r2 的 Leptos LoginView 已進入最新來源，不能再列「登入頁尚未套用」。先前獨立驗證報告以 `974db9ff...` 及 `1d1866db...` 核對程式差異；本次僅重讀最新程式與 CI，不重跑其 PostgreSQL／瀏覽器成果。第④節列出登入→強制改密碼→Session 撤銷→重新登入與日後再改密碼的回歸，避免只測元件字串。Day29 的來源切換與未知來源拒絕，歷史證據保留，未由本篇覆蓋事件類型查詢／ACL／SSE。

### 舊 Day 更正補充與 FIFO

Analytics Day24 補教已完成，沒有新的 pending Analytics FIFO 項；不重開、不增加補教計數。現行測試 fixture 採 bootstrap 指定長密碼，與需求 admin/admin 尚有差異，保留在原基本登入待辦；本篇沒有悄悄把測試密碼當產品需求。其他案的 Chat／Pet pending 不變。Day25 以前原文與證據不回寫。

## ③ 套件與軟體版本（2026-10-23）

| 項目／官方發佈來源 | manifest／精確 lock 或安裝狀態 | 官方最新穩定、建議 |
| --- | --- | --- |
| [Rust／Cargo](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) | stable；edition2024／resolver3；本機 rustc 不存在 | 1.99.1，建議驗證使用此版 |
| [Leptos](https://docs.rs/crate/leptos/latest) | workspace0.8 + ssr／0.8.22 | 0.8.22，沿用 |
| [Axum](https://docs.rs/axum/latest/axum/) | 0.8／0.8.9 | 0.8.9，沿用 |
| [SQLx](https://docs.rs/sqlx/latest/sqlx/) | 0.9 + runtime-tokio/postgres/macros／0.9.0 | 0.9.0，沿用 |
| [Tokio](https://docs.rs/crate/tokio/latest) | 1.54/full／1.54.1 | 1.54.1，沿用 |
| [Serde](https://docs.rs/crate/serde/latest)／[JSON](https://docs.rs/crate/serde_json/latest) | 1.0／1.0.229、1.0.154 | 相同；沿用表單解碼及 fixture |
| [Argon2](https://docs.rs/crate/argon2/latest) | 0.6／0.6.0 | 0.6.0，沿用既有加鹽驗證 |
| [SHA2](https://docs.rs/crate/sha2/latest) | 0.11／0.11.1 | 0.11.1；lock另有傳遞0.10.9，不修改 |
| [UUID](https://docs.rs/crate/uuid/latest) | 1.28／1.28.0 | 1.28.0，沿用既有 Session |
| [Tower](https://docs.rs/crate/tower/latest) | dev0.5/util／0.5.3 | 0.5.3，沿用 router 測試 |
| [PostgreSQL](https://www.postgresql.org/) | 無 server engine manifest pin；本機安裝版未查／未連DB | 18.6，建議隔離驗證用18.6；19 RC1排除 |

上述 lock 由 `1d1866dbbb30255fafa4c7f74b1868919662913c` Cargo.lock 全文解析，除工具／engine 外列出的直接套件均與當日官方穩定版相同，沒有版本遷移。版本來源為維護者官方公告／公開發布文件，非搜尋摘要猜測；本篇不安裝 CLI、不執行 bootstrap binary，fixture 直接呼叫 library initialize，所以未新增工具依賴。

[Leptos manifest](https://docs.rs/crate/leptos/latest/source/Cargo.toml) MSRV1.88、[SQLx manifest](https://docs.rs/crate/sqlx/latest/source/Cargo.toml) MSRV1.94.0；採Rust1.99.1滿足這兩個宣告門檻，全相依圖仍須編譯驗證。Native SSR 沿 `stats-server` 既有 `leptos/ssr` feature，這個表單沒有 WASM build 或新 hydration。PostgreSQL18.6 是正式穩定系列，19RC1不能當穩定版；現有 migrations、schema與checksum保持不變。本次不 cargo update、不變更 Cargo.lock、不操作舊庫／正式DB、不宣稱環境已升級。

## ④ 驗證命令、預期／實際、CI 與缺口

先在使用者套用後的 rust_ai_chat 工作副本設定全新外部 `CARGO_TARGET_DIR`：

```sh
cargo test --locked -p stats-server day31_tests
cargo check --locked -p stats-server
```

PG tests 必須使用專用全新 loopback PostgreSQL cluster。確認 `DATABASE_URL` 是該測試cluster的管理連線；SQLx test 會在此建立隔離測試庫，不可提供既有／正式DB連線，也不可執行清空或修改checksum以使測試通過。未配置此隔離前置就標未執行，不自動安裝或重建資料庫。配置後：

```sh
cargo test --locked -p stats-server --features postgres-integration --test day24_password_change day31_change_page
cargo test --locked -p stats-server --features postgres-integration --test day24_password_change
cargo test --locked -p stats-server --features postgres-integration --test day25_repeat_password_change
cargo test --locked -p stats-server --features postgres-integration -- --include-ignored
```

| 項目 | 預期 | 本次實際 |
| --- | --- | --- |
| Red A 真 router／Red B missing symbol | 舊頁 label契約失敗／函式不存在；分開記錄 | 未執行 |
| 新SSR與route tests | 單doctype、對應label、CSRF續期、未登入拒絕 | 未執行 |
| 既有day24/day25 targets | 錯CSRF/Origin拒絕、錯舊密碼拒絕、Email衝突、成功撤銷、二次改密碼 | 未執行 |
| 完整postgres-integration全部targets含ignored | 全新建庫、schema、事件、Session無退化 | 未執行 |
| 瀏覽器登入→首次改密碼→重新登入→日後修改 | 鍵盤可達、label聚焦、原密碼不可回填、成功後舊Session失效 | 未執行 |
| 截斷／中文密碼bytes／Email邊界與窄螢幕 | 伺服器規則與畫面一致，無敏感值洩漏 | 未執行 |

CI精確來源 SHA `1d1866dbbb30255fafa4c7f74b1868919662913c`：[Test All run39932989041](https://github.com/williechen/rust_ai_chat/actions/runs/39932989041) completed/success，2026-10-23T09:38:46Z；`test-stats-server` job120345579188 success，已讀workflow命令為 `cargo test --locked -p stats-server`。該job沒有 `postgres-integration`，不能冒充PG targets已跑；也不含本文新程式。來源 [Rust Dashboard run39932989009](https://github.com/williechen/rust_ai_chat/actions/runs/39932989009) success（2026-10-23T09:39:11Z），本次未逐步核對其job覆蓋，不拿它宣稱新頁驗收。

本篇只有文件交付，程式未套用、DB／HTTP／瀏覽器驗收未執行。Gate1須經獨立文章commit、精確SHA全文讀回、blob/SHA256保存與條件同步後才推進；Gate2待完成，不封存。角色／來源ACL、持久登入等待、admin/admin初始化差異、共用認證及重設信仍是缺口。`last_full_comparison`不更新，不宣稱完成全面比對；僅寫copilot_use文件，rust_ai_chat嚴格唯讀。
