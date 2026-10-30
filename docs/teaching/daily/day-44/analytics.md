# Analytics Day44｜後台登出表單加入 Session 專屬 CSRF，拒絕跨站登出

> 2026-10-30 本地驗證修訂 r3。最新產品唯讀 `4b1008b95bc3e170381fcd1db84695e421f036bd`；下列原 Refactor／Green 程式保留 `7ce39f0166a2120539e5908b33c88322a930c35a` 重現基準。最新 main 已有對應功能，不重複套用。next_day 已是45，不再加 Day。原發布版本與讀回保留於封存。

## 本次本地結果

最新4b1008b已實作logout CSRF並採用兩個舊fixture修正；本次在該主分支精確SHA重跑PG63項通過。 [四workspace、真套用、原始命令及阻塞](../../../source-review/local-validation-2026-10-30-day44-r2/README.md)。原發布全文以ZIP保存，原交付讀回不能冒認本次修訂；完整Gate2未完成。

## ① 目標、缺口、正式使用者流程

Day43 已接通 Leptos Dashboard 來源切換保留精確事件類型。原文基準的 Analytics 後台已支援管理員持久化、首次改密碼、Session 與登出；但 `password_login::logout` 只檢查 Origin＋Cookie，`DashboardView` 與 `ChangePasswordView` 的 POST `/logout` 沒有 Session 綁定的 CSRF hidden field。Origin 比對是防護之一，不應取代明確表單 token。本日 Green（最新 main 已實作）使登入後的 Dashboard 和首次／一般修改密碼頁都渲染**同一 Session 專屬 logout token**，POST `/logout` 必須提交該 token 才刪除 PostgreSQL Session；錯誤 token → 403，既有 Session 保留；正確 token → 303 回登入頁並撤銷 Session。

正式鏈：`analytics_apps/analytics_server/src/password_login.rs::dashboard/change_page`（PG 驗證 Session、判斷首次改密碼）→ `dashboard_ui` 的真 Leptos SSR 表單（兩頁）→ POST `/logout` → `raw_cookie`（Analytics 獨立 Cookie）→ `digest("logout:"+raw)` → `DELETE FROM uac_login_session` → 303。使用者來源 token、`admin.rs` 舊路由、Chat/Pet Session 與其他後台 API 均不混用；本日不冒稱角色／來源 ACL 已完成。

## Refactor｜Analytics CSV URL 決策從 SSR 元件搬到同案純模組

**獨立 Refactor**：原 `dashboard_ui.rs::DashboardView` 在 Leptos HTML 建構時自行拼 `/api/v1/events/{source}/export.csv?event_type=...`；相同 URL 編碼責任已在 `dashboard_url.rs`。把 CSV 連結完整搬成 `dashboard_url::export_csv_url(source,event_type)`，正式 `DashboardView` 改 import/call；`encode_query_component`、原 CSV handler、資料來源、授權／查詢及資料表完全不變。新增空篩選、中文與 `&` 的純單元測試。這是 Analytics 專案自己的 UI URL 重構，**不是本日 CSRF Green 的後續整理**，不碰 Chat、Pet、shared 或全庫 CI。

**舊斷言承接**：原 `dashboard_ui::day40_csv_filter_ui_tests` 的真 Leptos SSR 連結、`dashboard_url::tests` 編碼、`export.rs` 的 HTTP／PG 與 CSV escaping、Day41/42/43 明細／來源切換仍原位執行；新 `export_url_refactor_tests` 只補純 URL 責任，不以它取代正式 Router 測試。逐檔完整 Refactor patch：

```diff
diff --git a/analytics_apps/analytics_server/src/dashboard_ui.rs b/analytics_apps/analytics_server/src/dashboard_ui.rs
--- a/analytics_apps/analytics_server/src/dashboard_ui.rs
+++ b/analytics_apps/analytics_server/src/dashboard_ui.rs
@@ -3,29 +3,22 @@
 use analytics_persistence::EventSummary;
 
 use crate::dashboard_summary::summary_count_label;
-use crate::dashboard_url::{encode_query_component, source_switch_url};
-
-#[component]
-fn DashboardView(
-    source_label: &'static str,
-    total: i64,
-    source: String,
-    event_type: String,
-    rows: Vec<(String, i64)>,
-) -> impl IntoView {
-    let chat_current = (source_label == "Chat").then_some("page");
-    let pet_current = (source_label == "Pet").then_some("page");
-    let chat_source_url = source_switch_url("web_chat", &event_type);
-    let pet_source_url = source_switch_url("desktop_pet", &event_type);
-    let clear_url = format!("/dashboard?source={source}");
-    let export_url = if event_type.is_empty() {
-        format!("/api/v1/events/{source}/export.csv")
-    } else {
-        format!(
-            "/api/v1/events/{source}/export.csv?event_type={}",
-            encode_query_component(&event_type)
-        )
-    };
+use crate::dashboard_url::{encode_query_component, export_csv_url, source_switch_url};
+
+#[component]
+fn DashboardView(
+    source_label: &'static str,
+    total: i64,
+    source: String,
+    event_type: String,
+    rows: Vec<(String, i64)>,
+) -> impl IntoView {
+    let chat_current = (source_label == "Chat").then_some("page");
+    let pet_current = (source_label == "Pet").then_some("page");
+    let chat_source_url = source_switch_url("web_chat", &event_type);
+    let pet_source_url = source_switch_url("desktop_pet", &event_type);
+    let clear_url = format!("/dashboard?source={source}");
+    let export_url = export_csv_url(&source, &event_type);
     let filter_description = if event_type.is_empty() {
         "全部事件類型".to_string()
     } else {
diff --git a/analytics_apps/analytics_server/src/dashboard_url.rs b/analytics_apps/analytics_server/src/dashboard_url.rs
--- a/analytics_apps/analytics_server/src/dashboard_url.rs
+++ b/analytics_apps/analytics_server/src/dashboard_url.rs
@@ -22,29 +22,52 @@
     }
 }
 
-#[cfg(test)]
-mod tests {
-    use super::encode_query_component;
-
-    #[test]
-    fn existing_csv_url_contract_keeps_utf8_and_reserved_characters() {
-        assert_eq!(
-            encode_query_component("a &中文"),
-            "a%20%26%E4%B8%AD%E6%96%87"
-        );
-        assert_eq!(encode_query_component("a-b_c.~"), "a-b_c.~");
-        assert_eq!(encode_query_component("<script>"), "%3Cscript%3E");
-    }
-
-    #[test]
-    fn source_switch_preserves_encoded_filter_and_empty_semantics() {
-        assert_eq!(
-            super::source_switch_url("desktop_pet", "a &中文"),
-            "/dashboard?source=desktop_pet&event_type=a%20%26%E4%B8%AD%E6%96%87"
-        );
-        assert_eq!(
-            super::source_switch_url("web_chat", ""),
-            "/dashboard?source=web_chat"
-        );
+/// Reuse the exact-kind encoder for the authenticated CSV export link.
+pub(crate) fn export_csv_url(source: &str, event_type: &str) -> String {
+    if event_type.is_empty() {
+        format!("/api/v1/events/{source}/export.csv")
+    } else {
+        format!(
+            "/api/v1/events/{source}/export.csv?event_type={}",
+            encode_query_component(event_type)
+        )
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::encode_query_component;
+
+    #[test]
+    fn existing_csv_url_contract_keeps_utf8_and_reserved_characters() {
+        assert_eq!(
+            encode_query_component("a &中文"),
+            "a%20%26%E4%B8%AD%E6%96%87"
+        );
+        assert_eq!(encode_query_component("a-b_c.~"), "a-b_c.~");
+        assert_eq!(encode_query_component("<script>"), "%3Cscript%3E");
+    }
+
+    #[test]
+    fn source_switch_preserves_encoded_filter_and_empty_semantics() {
+        assert_eq!(
+            super::source_switch_url("desktop_pet", "a &中文"),
+            "/dashboard?source=desktop_pet&event_type=a%20%26%E4%B8%AD%E6%96%87"
+        );
+        assert_eq!(
+            super::source_switch_url("web_chat", ""),
+            "/dashboard?source=web_chat"
+        );
+    }
+}
+
+#[cfg(test)]
+mod export_url_refactor_tests {
+    use super::*;
+    #[test]
+    fn csv_url_keeps_exact_kind_and_no_filter_contract() {
+        assert_eq!(export_csv_url("web_chat", ""), "/api/v1/events/web_chat/export.csv");
+        assert_eq!(export_csv_url("desktop_pet", "pet.中文 &"),
+            "/api/v1/events/desktop_pet/export.csv?event_type=pet.%E4%B8%AD%E6%96%87%20%26");
     }
 }
```

Refactor 後續：將 Analytics `password_login` 的 Session SQL、錯誤與時間政策逐步搬至同案 Application/Adapter 並保留 PG、HTTP、CSRF 斷言；不重複全專案 G-01/G-02。驗證 `cargo test --manifest-path analytics_apps/Cargo.toml -p analytics-server export_url_refactor_tests` **已在最新 main 的一般／PG suite 執行，URL 與 SSR 斷言保留**。

## ② Red → 完整 Green（核心預估15分鐘；原 Red 未重跑，Green 真PG已跑）

**Red**：新增 `tests/day44_logout_csrf.rs`（`postgres-integration` feature、`#[sqlx::test]` 真 PG migration）從管理員初始化、POST 帳密登入、GET 首次改密碼頁，擷取真正 `/logout` 表單的 CSRF；錯 token 必須 403 且 Session 數仍 1，異來源 403，正確 token 303 且 Session 數歸零、原 Cookie 401。原版本未驗 token，對 `csrf=wrong` 會錯誤登出（預期行為 Red）；新的 SSR 渲染器在原版不存在亦會編譯 Red。原 Red **未實際重跑**；最新 main 新測試1項已通過，不能把 SQL／環境錯誤當 Red。保留 Day24/25/26 的密碼修改、Session／PG 斷言與 Day29–43 Leptos SSR／來源查詢。

**Green**：`DashboardView` 與 `ChangePasswordView` 都加入 `logout_csrf` 並在 POST 表單輸出 hidden field；原 `render_dashboard_with_rows`、`render_change_password` 保留供既有 SSR 呼叫／測試，正式已授權 caller 改用 `*_csrf`／`*_with_logout` 傳入 `digest("logout:"+raw)`。後端新增 `LogoutForm`、驗證 Session token 派生 CSRF，只有驗過才 DELETE。使用現有 `serde::Deserialize`、`axum::Form`、`sha2`、`sqlx`，不新增 crate／Cargo feature／migration；密碼變更原有 CSRF 值仍用 `digest("change-password:"+raw)`，不能互用。

**全部修改檔與完整 Green diff**（以本篇 Refactor 套用後為基準）：

```diff
diff --git a/analytics_apps/analytics_server/src/dashboard_ui.rs b/analytics_apps/analytics_server/src/dashboard_ui.rs
--- a/analytics_apps/analytics_server/src/dashboard_ui.rs
+++ b/analytics_apps/analytics_server/src/dashboard_ui.rs
@@ -12,408 +12,446 @@
     source: String,
     event_type: String,
     rows: Vec<(String, i64)>,
-) -> impl IntoView {
-    let chat_current = (source_label == "Chat").then_some("page");
-    let pet_current = (source_label == "Pet").then_some("page");
-    let chat_source_url = source_switch_url("web_chat", &event_type);
-    let pet_source_url = source_switch_url("desktop_pet", &event_type);
-    let clear_url = format!("/dashboard?source={source}");
-    let export_url = export_csv_url(&source, &event_type);
-    let filter_description = if event_type.is_empty() {
-        "全部事件類型".to_string()
-    } else {
-        format!("精確事件類型：{event_type}")
-    };
-
-    let count_label = summary_count_label(source_label, &event_type, total);
-    let filter_source = source.clone();
-
-    view! {
-        <html lang="zh-Hant">
-            <head>
-                <meta charset="utf-8"/>
-                <title>"Analytics 管理後台"</title>
-            </head>
-            <body>
-                <main>
-                    <h1>"Analytics 管理後台"</h1>
-                    <nav aria-label="事件來源">
-                        <a href=chat_source_url aria-current=chat_current>
-                            "Chat 事件"
-                        </a>
-                        <a href=pet_source_url aria-current=pet_current>
-                            "Pet 事件"
-                        </a>
-                    </nav>
-                    <a href="/change-password">"修改密碼與 Email"</a>
-                    <a href=export_url>"匯出目前篩選 CSV"</a>
-                    <form method="get" action="/dashboard">
-                        <input type="hidden" name="source" value=source/>
-                        <label for="event-type">"事件類型（精確比對）"</label>
-                        <input
-                            id="event-type"
-                            name="event_type"
-                            value=event_type
-                            maxlength="128"
-                            placeholder="例如 chat.message.persisted"
-                        />
-                        <button type="submit">"篩選"</button>
-                        <a href=clear_url>"清除篩選"</a>
-                    </form>
-                    <p>{filter_description}</p>
-                    <p class="summary-count" aria-live="polite">{count_label}</p>
-                    <table>
-                        <caption>"已提交事件類型明細"</caption>
-                        <thead><tr><th scope="col">"事件類型"</th><th scope="col">"已提交筆數"</th></tr></thead>
-                        <tbody>
-                            {rows.into_iter().map(|(kind, count)| {
-                                let detail_url = format!(
-                                    "/dashboard?source={}&event_type={}",
-                                    filter_source,
-                                    encode_query_component(&kind)
-                                );
-                                view! { <tr>
-                                    <td>
-                                        <a href=detail_url>{kind}</a>
-                                    </td>
-                                    <td>{count}</td>
-                                </tr> }
-                            }).collect_view()}
-                        </tbody>
-                    </table>
-                    <p role="status">
-                        {if total == 0 {
-                            "沒有符合條件的已提交事件"
-                        } else {
-                            "查詢完成"
-                        }}
-                    </p>
-                    <p>"目前僅支援管理員；角色與來源 ACL 尚待接通。"</p>
-                    <form method="post" action="/logout">
-                        <button type="submit">"登出"</button>
-                    </form>
-                </main>
-            </body>
-        </html>
-    }
-}
-
-pub fn render_dashboard_filtered(
-    source_label: &'static str,
-    total: i64,
-    source: &str,
-    event_type: &str,
-) -> String {
-    render_dashboard_with_rows(source_label, total, source, event_type, &[])
-}
-
-/// The real dashboard passes committed PG summary rows; older render callers remain valid.
-pub fn render_dashboard_with_rows(
-    source_label: &'static str,
-    total: i64,
-    source: &str,
-    event_type: &str,
-    rows: &[EventSummary],
-) -> String {
-    let rows: Vec<(String, i64)> = rows
-        .iter()
-        .map(|row| (row.event_type.clone(), row.total))
-        .collect();
-    format!(
-        "<!doctype html>{}",
-        view! {
-            <DashboardView
-                source_label
-                total
-                source=source.to_string()
-                event_type=event_type.to_string()
-                rows
-            />
-        }
-        .to_html()
-    )
-}
-
-// 既有 Day28/29 測試與舊呼叫者仍可使用兩參數版本。
-pub fn render_dashboard(source_label: &'static str, total: i64) -> String {
-    let source = if source_label == "Pet" {
-        "desktop_pet"
-    } else {
-        "web_chat"
-    };
-    render_dashboard_filtered(source_label, total, source, "")
-}
-
-#[component]
-fn LoginView() -> impl IntoView {
-    view! {
-        <html lang="zh-Hant">
-            <head>
-                <meta charset="utf-8"/>
-                <title>"Analytics 帳密登入"</title>
-            </head>
-            <body>
-                <main>
-                    <h1>"Analytics 帳密登入"</h1>
-                    <form method="post" action="/login">
-                        <div>
-                            <label for="analytics-username">"帳號"</label>
-                            <input id="analytics-username" name="username"
-                                autocomplete="username" required/>
-                        </div>
-                        <div>
-                            <label for="analytics-password">"密碼"</label>
-                            <input id="analytics-password" name="password"
-                                type="password" autocomplete="current-password" required/>
-                        </div>
-                        <button type="submit">"登入"</button>
-                    </form>
-                </main>
-            </body>
-        </html>
-    }
-}
-
-pub fn render_login() -> String {
-    format!("<!doctype html>{}", view! { <LoginView/> }.to_html())
-}
-
-#[component]
-fn ChangePasswordView(restricted: bool, csrf: String) -> impl IntoView {
-    let heading = if restricted {
-        "首次登入須更換密碼與Email"
-    } else {
-        "修改密碼與Email"
-    };
-    view! {
-        <html lang="zh-Hant">
-            <head><meta charset="utf-8"/><title>"修改密碼"</title></head>
-            <body><main>
-                <h1>{heading}</h1>
-                <form method="post" action="/change-password">
-                    <input type="hidden" name="csrf" value=csrf/>
-                    <p>
-                        <label for="change-current">"目前密碼"</label>
-                        <input id="change-current" type="password" name="current_password"
-                            autocomplete="current-password" required maxlength="128"/>
-                    </p>
-                    <p>
-                        <label for="change-new">"新密碼"</label>
-                        <input id="change-new" type="password" name="new_password"
-                            autocomplete="new-password" required minlength="8" maxlength="128"/>
-                    </p>
-                    <p>
-                        <label for="change-confirm">"確認新密碼"</label>
-                        <input id="change-confirm" type="password" name="confirm_password"
-                            autocomplete="new-password" required minlength="8" maxlength="128"/>
-                    </p>
-                    <p>
-                        <label for="change-email">"唯一Email"</label>
-                        <input id="change-email" type="email" name="email"
-                            autocomplete="email" required maxlength="254"/>
-                    </p>
-                    <button type="submit">"儲存並重新登入"</button>
-                </form>
-                <p>"成功後所有裝置都須重新登入。Email不驗證；寄信重設尚未接通。"</p>
-                <form method="post" action="/logout">
-                    <button type="submit">"登出"</button>
-                </form>
-            </main></body>
-        </html>
-    }
-}
-
-pub fn render_change_password(restricted: bool, csrf: String) -> String {
-    format!(
-        "<!doctype html>{}",
-        view! {
-            <ChangePasswordView restricted csrf/>
-        }
-        .to_html()
-    )
-}
-
-#[cfg(test)]
-mod day43_source_switch_tests {
-    use super::*;
-    #[test]
-    fn source_navigation_preserves_exact_utf8_filter_and_escapes_markup() {
-        let html = render_dashboard_filtered("Chat", 1, "web_chat", "chat.中文 &");
-        assert!(html.contains("source=desktop_pet&amp;event_type=chat.%E4%B8%AD%E6%96%87%20%26"));
-        assert!(html.contains("source=web_chat&amp;event_type=chat.%E4%B8%AD%E6%96%87%20%26"));
-        assert!(!html.contains("event_type=chat.中文 &"));
-        let all = render_dashboard_filtered("Chat", 0, "web_chat", "");
-        assert!(all.contains("/dashboard?source=desktop_pet"));
-    }
-}
-
-#[cfg(test)]
-mod day42_kind_navigation_tests {
-    use super::*;
-    #[test]
-    fn kind_carow_links_to_real_exact_filter_and_escapes_html() {
-        let html = render_dashboard_with_rows(
-            "Chat",
-            1,
-            "web_chat",
-            "",
-            &[EventSummary {
-                source_app: "web_chat".into(),
-                event_type: "chat.中文 &".into(),
-                total: 1,
-            }],
-        );
-        assert!(html.contains("event_type=chat.%E4%B8%AD%E6%96%87%20%26"));
-        assert!(html.contains("href="));
-        assert!(html.contains("chat.中文 &amp;"));
-        assert!(!html.contains("event_type=chat.中文 &"));
-    }
-}
-
-#[cfg(test)]
-mod day29_tests {
-    use super::*;
-
-    #[test]
-    fn active_source_is_marked_for_accessibility() {
-        let html = render_dashboard("Pet", 3);
-        assert!(html.contains("desktop_pet"));
-        assert!(html.contains("aria-current=\"page\""));
-        assert!(html.contains("全部事件類型：3 筆已提交事件"));
-    }
-}
-
-#[cfg(test)]
-mod day30_tests {
-    #[test]
-    fn login_page_is_server_rendered_with_accessible_fields() {
-        let html = super::render_login();
-        assert_eq!(html.matches("<!doctype html>").count(), 1);
-        assert!(html.contains("action=\"/login\""));
-        assert!(html.contains("name=\"username\""));
-        assert!(html.contains("name=\"password\""));
-        assert!(html.contains("autocomplete=\"current-password\""));
-    }
-}
-
-#[cfg(test)]
-mod day31_tests {
-    #[test]
-    fn change_form_keeps_wire_names_and_escapes_attribute_values() {
-        let html = super::render_change_password(true, "a\" onfocus=\"bad".into());
-        assert_eq!(html.matches("<!doctype html>").count(), 1);
-        for name in [
-            "current_password",
-            "new_password",
-            "confirm_password",
-            "email",
-            "csrf",
-        ] {
-            assert!(html.contains(&format!("name=\"{name}\"")));
-        }
-        assert!(html.contains("&quot;"));
-        assert!(!html.contains("value=\"a\" onfocus="));
-        assert!(html.contains("首次登入須更換密碼與Email"));
-        let ordinary = super::render_change_password(false, "test-csrf".into());
-        assert!(ordinary.contains("修改密碼與Email"));
-        assert!(!ordinary.contains("首次登入須更換密碼與Email"));
-        assert!(!ordinary.contains("lesson-only-password"));
-    }
-}
-
-#[cfg(test)]
-mod day32_filter_tests {
-    #[test]
-    fn dashboard_has_accessible_exact_kind_filter_and_escaped_value() {
-        let html = super::render_dashboard_filtered("Chat", 0, "web_chat", "chat.\" onfocus=\"bad");
-        assert_eq!(html.matches("<!doctype html>").count(), 1);
-        assert!(html.contains("method=\"get\""));
-        assert!(html.contains("action=\"/dashboard\""));
-        assert!(html.contains("name=\"source\" value=\"web_chat\""));
-        assert!(html.contains("for=\"event-type\""));
-        assert!(html.contains("id=\"event-type\""));
-        assert!(html.contains("name=\"event_type\""));
-        assert!(html.contains("&quot;"));
-        assert!(!html.contains("value=\"chat.\" onfocus="));
-        assert!(html.contains("沒有符合條件的已提交事件"));
-        assert!(html.contains("/dashboard?source=web_chat"));
-    }
-}
-
-#[cfg(test)]
-mod day33_summary_caption_tests {
-    #[test]
-    fn count_label_distinguishes_full_and_exact_kind() {
-        assert_eq!(
-            super::summary_count_label("Chat", "", 12),
-            "Chat 全部事件類型：12 筆已提交事件"
-        );
-        assert_eq!(
-            super::summary_count_label("Chat", "chat.message.persisted", 3),
-            "Chat 事件類型「chat.message.persisted」：3 筆已提交事件"
-        );
-        assert_eq!(
-            super::summary_count_label("Pet", "pet.sleep", 0),
-            "Pet 事件類型「pet.sleep」：0 筆已提交事件"
-        );
-    }
-
-    #[test]
-    fn real_leptos_ssr_escapes_filtered_type_in_count_label() {
-        let html =
-            super::render_dashboard_filtered("Chat", 3, "web_chat", "<img src=x onerror=alert(1)>");
-        assert!(html.contains("3 筆已提交事件"));
-        assert!(html.contains("class=\"summary-count\""));
-        assert!(!html.contains("<img src=x onerror=alert(1)>"));
-        assert!(html.contains("&lt;img"));
-    }
-}
-
-#[cfg(test)]
-mod day40_csv_filter_ui_tests {
-    use super::*;
-    #[test]
-    fn dashboard_csv_link_encodes_exact_kind_and_escapes_markup() {
-        assert_eq!(
-            encode_query_component("a &中文"),
-            "a%20%26%E4%B8%AD%E6%96%87"
-        );
-        let html = render_dashboard_filtered("Chat", 1, "web_chat", "x&y");
-        assert!(html.contains("/api/v1/events/web_chat/export.csv?event_type=x%26y"));
-        assert!(!html.contains("event_type=x&y"));
-        let empty = render_dashboard_filtered("Chat", 0, "web_chat", "");
-        assert!(empty.contains("/api/v1/events/web_chat/export.csv"));
-    }
-}
-
-#[cfg(test)]
-mod day41_event_table_tests {
-    use super::*;
-    #[test]
-    fn committed_kind_rows_are_rendered_in_a_real_leptos_table_and_escaped() {
-        let html = render_dashboard_with_rows(
-            "Chat",
-            3,
-            "web_chat",
-            "",
-            &[
-                EventSummary {
-                    source_app: "web_chat".into(),
-                    event_type: "chat.message.persisted".into(),
-                    total: 2,
-                },
-                EventSummary {
-                    source_app: "web_chat".into(),
-                    event_type: "<script>bad</script>".into(),
-                    total: 1,
-                },
-            ],
-        );
-        assert!(html.contains("<table"));
-        assert!(html.contains("已提交事件類型明細"));
-        assert!(html.contains("chat.message.persisted"));
-        assert!(html.contains("&lt;script&gt;bad&lt;/script&gt;"));
-        assert!(!html.contains("<script>bad</script>"));
-        assert!(html.contains("全部事件類型：3 筆已提交事件"));
+    logout_csrf: String,
+) -> impl IntoView {
+    let chat_current = (source_label == "Chat").then_some("page");
+    let pet_current = (source_label == "Pet").then_some("page");
+    let chat_source_url = source_switch_url("web_chat", &event_type);
+    let pet_source_url = source_switch_url("desktop_pet", &event_type);
+    let clear_url = format!("/dashboard?source={source}");
+    let export_url = export_csv_url(&source, &event_type);
+    let filter_description = if event_type.is_empty() {
+        "全部事件類型".to_string()
+    } else {
+        format!("精確事件類型：{event_type}")
+    };
+
+    let count_label = summary_count_label(source_label, &event_type, total);
+    let filter_source = source.clone();
+
+    view! {
+        <html lang="zh-Hant">
+            <head>
+                <meta charset="utf-8"/>
+                <title>"Analytics 管理後台"</title>
+            </head>
+            <body>
+                <main>
+                    <h1>"Analytics 管理後台"</h1>
+                    <nav aria-label="事件來源">
+                        <a href=chat_source_url aria-current=chat_current>
+                            "Chat 事件"
+                        </a>
+                        <a href=pet_source_url aria-current=pet_current>
+                            "Pet 事件"
+                        </a>
+                    </nav>
+                    <a href="/change-password">"修改密碼與 Email"</a>
+                    <a href=export_url>"匯出目前篩選 CSV"</a>
+                    <form method="get" action="/dashboard">
+                        <input type="hidden" name="source" value=source/>
+                        <label for="event-type">"事件類型（精確比對）"</label>
+                        <input
+                            id="event-type"
+                            name="event_type"
+                            value=event_type
+                            maxlength="128"
+                            placeholder="例如 chat.message.persisted"
+                        />
+                        <button type="submit">"篩選"</button>
+                        <a href=clear_url>"清除篩選"</a>
+                    </form>
+                    <p>{filter_description}</p>
+                    <p class="summary-count" aria-live="polite">{count_label}</p>
+                    <table>
+                        <caption>"已提交事件類型明細"</caption>
+                        <thead><tr><th scope="col">"事件類型"</th><th scope="col">"已提交筆數"</th></tr></thead>
+                        <tbody>
+                            {rows.into_iter().map(|(kind, count)| {
+                                let detail_url = format!(
+                                    "/dashboard?source={}&event_type={}",
+                                    filter_source,
+                                    encode_query_component(&kind)
+                                );
+                                view! { <tr>
+                                    <td>
+                                        <a href=detail_url>{kind}</a>
+                                    </td>
+                                    <td>{count}</td>
+                                </tr> }
+                            }).collect_view()}
+                        </tbody>
+                    </table>
+                    <p role="status">
+                        {if total == 0 {
+                            "沒有符合條件的已提交事件"
+                        } else {
+                            "查詢完成"
+                        }}
+                    </p>
+                    <p>"目前僅支援管理員；角色與來源 ACL 尚待接通。"</p>
+                    <form method="post" action="/logout">
+                        <input type="hidden" name="csrf" value=logout_csrf/>
+                        <button type="submit">"登出"</button>
+                    </form>
+                </main>
+            </body>
+        </html>
+    }
+}
+
+pub fn render_dashboard_filtered(
+    source_label: &'static str,
+    total: i64,
+    source: &str,
+    event_type: &str,
+) -> String {
+    render_dashboard_with_rows(source_label, total, source, event_type, &[])
+}
+
+/// The real dashboard passes committed PG summary rows; older render callers remain valid.
+pub fn render_dashboard_with_rows(
+    source_label: &'static str,
+    total: i64,
+    source: &str,
+    event_type: &str,
+    rows: &[EventSummary],
+) -> String {
+    render_dashboard_with_rows_csrf(source_label, total, source, event_type, rows, "")
+}
+
+/// Real authenticated dashboard caller passes a Session-derived logout CSRF.
+/// The old renderer remains for SSR snapshot tests, never as an auth decision.
+pub fn render_dashboard_with_rows_csrf(
+    source_label: &'static str,
+    total: i64,
+    source: &str,
+    event_type: &str,
+    rows: &[EventSummary],
+    logout_csrf: &str,
+) -> String {
+    let rows: Vec<(String, i64)> = rows
+        .iter()
+        .map(|row| (row.event_type.clone(), row.total))
+        .collect();
+    format!(
+        "<!doctype html>{}",
+        view! {
+            <DashboardView
+                source_label
+                total
+                source=source.to_string()
+                event_type=event_type.to_string()
+                rows
+                logout_csrf=logout_csrf.to_string()
+            />
+        }
+        .to_html()
+    )
+}
+
+// 既有 Day28/29 測試與舊呼叫者仍可使用兩參數版本。
+pub fn render_dashboard(source_label: &'static str, total: i64) -> String {
+    let source = if source_label == "Pet" {
+        "desktop_pet"
+    } else {
+        "web_chat"
+    };
+    render_dashboard_filtered(source_label, total, source, "")
+}
+
+#[component]
+fn LoginView() -> impl IntoView {
+    view! {
+        <html lang="zh-Hant">
+            <head>
+                <meta charset="utf-8"/>
+                <title>"Analytics 帳密登入"</title>
+            </head>
+            <body>
+                <main>
+                    <h1>"Analytics 帳密登入"</h1>
+                    <form method="post" action="/login">
+                        <div>
+                            <label for="analytics-username">"帳號"</label>
+                            <input id="analytics-username" name="username"
+                                autocomplete="username" required/>
+                        </div>
+                        <div>
+                            <label for="analytics-password">"密碼"</label>
+                            <input id="analytics-password" name="password"
+                                type="password" autocomplete="current-password" required/>
+                        </div>
+                        <button type="submit">"登入"</button>
+                    </form>
+                </main>
+            </body>
+        </html>
+    }
+}
+
+pub fn render_login() -> String {
+    format!("<!doctype html>{}", view! { <LoginView/> }.to_html())
+}
+
+#[component]
+fn ChangePasswordView(restricted: bool, csrf: String, logout_csrf: String) -> impl IntoView {
+    let heading = if restricted {
+        "首次登入須更換密碼與Email"
+    } else {
+        "修改密碼與Email"
+    };
+    view! {
+        <html lang="zh-Hant">
+            <head><meta charset="utf-8"/><title>"修改密碼"</title></head>
+            <body><main>
+                <h1>{heading}</h1>
+                <form method="post" action="/change-password">
+                    <input type="hidden" name="csrf" value=csrf/>
+                    <p>
+                        <label for="change-current">"目前密碼"</label>
+                        <input id="change-current" type="password" name="current_password"
+                            autocomplete="current-password" required maxlength="128"/>
+                    </p>
+                    <p>
+                        <label for="change-new">"新密碼"</label>
+                        <input id="change-new" type="password" name="new_password"
+                            autocomplete="new-password" required minlength="8" maxlength="128"/>
+                    </p>
+                    <p>
+                        <label for="change-confirm">"確認新密碼"</label>
+                        <input id="change-confirm" type="password" name="confirm_password"
+                            autocomplete="new-password" required minlength="8" maxlength="128"/>
+                    </p>
+                    <p>
+                        <label for="change-email">"唯一Email"</label>
+                        <input id="change-email" type="email" name="email"
+                            autocomplete="email" required maxlength="254"/>
+                    </p>
+                    <button type="submit">"儲存並重新登入"</button>
+                </form>
+                <p>"成功後所有裝置都須重新登入。Email不驗證；寄信重設尚未接通。"</p>
+                <form method="post" action="/logout">
+                    <input type="hidden" name="csrf" value=logout_csrf/>
+                    <button type="submit">"登出"</button>
+                </form>
+            </main></body>
+        </html>
+    }
+}
+
+pub fn render_change_password(restricted: bool, csrf: String) -> String {
+    render_change_password_with_logout(restricted, csrf, String::new())
+}
+
+pub fn render_change_password_with_logout(
+    restricted: bool, csrf: String, logout_csrf: String,
+) -> String {
+    format!(
+        "<!doctype html>{}",
+        view! {
+            <ChangePasswordView restricted csrf logout_csrf/>
+        }
+        .to_html()
+    )
+}
+
+#[cfg(test)]
+mod day43_source_switch_tests {
+    use super::*;
+    #[test]
+    fn source_navigation_preserves_exact_utf8_filter_and_escapes_markup() {
+        let html = render_dashboard_filtered("Chat", 1, "web_chat", "chat.中文 &");
+        assert!(html.contains("source=desktop_pet&amp;event_type=chat.%E4%B8%AD%E6%96%87%20%26"));
+        assert!(html.contains("source=web_chat&amp;event_type=chat.%E4%B8%AD%E6%96%87%20%26"));
+        assert!(!html.contains("event_type=chat.中文 &"));
+        let all = render_dashboard_filtered("Chat", 0, "web_chat", "");
+        assert!(all.contains("/dashboard?source=desktop_pet"));
+    }
+}
+
+#[cfg(test)]
+mod day42_kind_navigation_tests {
+    use super::*;
+    #[test]
+    fn kind_carow_links_to_real_exact_filter_and_escapes_html() {
+        let html = render_dashboard_with_rows(
+            "Chat",
+            1,
+            "web_chat",
+            "",
+            &[EventSummary {
+                source_app: "web_chat".into(),
+                event_type: "chat.中文 &".into(),
+                total: 1,
+            }],
+        );
+        assert!(html.contains("event_type=chat.%E4%B8%AD%E6%96%87%20%26"));
+        assert!(html.contains("href="));
+        assert!(html.contains("chat.中文 &amp;"));
+        assert!(!html.contains("event_type=chat.中文 &"));
+    }
+}
+
+#[cfg(test)]
+mod day29_tests {
+    use super::*;
+
+    #[test]
+    fn active_source_is_marked_for_accessibility() {
+        let html = render_dashboard("Pet", 3);
+        assert!(html.contains("desktop_pet"));
+        assert!(html.contains("aria-current=\"page\""));
+        assert!(html.contains("全部事件類型：3 筆已提交事件"));
+    }
+}
+
+#[cfg(test)]
+mod day30_tests {
+    #[test]
+    fn login_page_is_server_rendered_with_accessible_fields() {
+        let html = super::render_login();
+        assert_eq!(html.matches("<!doctype html>").count(), 1);
+        assert!(html.contains("action=\"/login\""));
+        assert!(html.contains("name=\"username\""));
+        assert!(html.contains("name=\"password\""));
+        assert!(html.contains("autocomplete=\"current-password\""));
+    }
+}
+
+#[cfg(test)]
+mod day31_tests {
+    #[test]
+    fn change_form_keeps_wire_names_and_escapes_attribute_values() {
+        let html = super::render_change_password(true, "a\" onfocus=\"bad".into());
+        assert_eq!(html.matches("<!doctype html>").count(), 1);
+        for name in [
+            "current_password",
+            "new_password",
+            "confirm_password",
+            "email",
+            "csrf",
+        ] {
+            assert!(html.contains(&format!("name=\"{name}\"")));
+        }
+        assert!(html.contains("&quot;"));
+        assert!(!html.contains("value=\"a\" onfocus="));
+        assert!(html.contains("首次登入須更換密碼與Email"));
+        let ordinary = super::render_change_password(false, "test-csrf".into());
+        assert!(ordinary.contains("修改密碼與Email"));
+        assert!(!ordinary.contains("首次登入須更換密碼與Email"));
+        assert!(!ordinary.contains("lesson-only-password"));
+    }
+}
+
+#[cfg(test)]
+mod day32_filter_tests {
+    #[test]
+    fn dashboard_has_accessible_exact_kind_filter_and_escaped_value() {
+        let html = super::render_dashboard_filtered("Chat", 0, "web_chat", "chat.\" onfocus=\"bad");
+        assert_eq!(html.matches("<!doctype html>").count(), 1);
+        assert!(html.contains("method=\"get\""));
+        assert!(html.contains("action=\"/dashboard\""));
+        assert!(html.contains("name=\"source\" value=\"web_chat\""));
+        assert!(html.contains("for=\"event-type\""));
+        assert!(html.contains("id=\"event-type\""));
+        assert!(html.contains("name=\"event_type\""));
+        assert!(html.contains("&quot;"));
+        assert!(!html.contains("value=\"chat.\" onfocus="));
+        assert!(html.contains("沒有符合條件的已提交事件"));
+        assert!(html.contains("/dashboard?source=web_chat"));
+    }
+}
+
+#[cfg(test)]
+mod day33_summary_caption_tests {
+    #[test]
+    fn count_label_distinguishes_full_and_exact_kind() {
+        assert_eq!(
+            super::summary_count_label("Chat", "", 12),
+            "Chat 全部事件類型：12 筆已提交事件"
+        );
+        assert_eq!(
+            super::summary_count_label("Chat", "chat.message.persisted", 3),
+            "Chat 事件類型「chat.message.persisted」：3 筆已提交事件"
+        );
+        assert_eq!(
+            super::summary_count_label("Pet", "pet.sleep", 0),
+            "Pet 事件類型「pet.sleep」：0 筆已提交事件"
+        );
+    }
+
+    #[test]
+    fn real_leptos_ssr_escapes_filtered_type_in_count_label() {
+        let html =
+            super::render_dashboard_filtered("Chat", 3, "web_chat", "<img src=x onerror=alert(1)>");
+        assert!(html.contains("3 筆已提交事件"));
+        assert!(html.contains("class=\"summary-count\""));
+        assert!(!html.contains("<img src=x onerror=alert(1)>"));
+        assert!(html.contains("&lt;img"));
+    }
+}
+
+#[cfg(test)]
+mod day40_csv_filter_ui_tests {
+    use super::*;
+    #[test]
+    fn dashboard_csv_link_encodes_exact_kind_and_escapes_markup() {
+        assert_eq!(
+            encode_query_component("a &中文"),
+            "a%20%26%E4%B8%AD%E6%96%87"
+        );
+        let html = render_dashboard_filtered("Chat", 1, "web_chat", "x&y");
+        assert!(html.contains("/api/v1/events/web_chat/export.csv?event_type=x%26y"));
+        assert!(!html.contains("event_type=x&y"));
+        let empty = render_dashboard_filtered("Chat", 0, "web_chat", "");
+        assert!(empty.contains("/api/v1/events/web_chat/export.csv"));
+    }
+}
+
+#[cfg(test)]
+mod day41_event_table_tests {
+    use super::*;
+    #[test]
+    fn committed_kind_rows_are_rendered_in_a_real_leptos_table_and_escaped() {
+        let html = render_dashboard_with_rows(
+            "Chat",
+            3,
+            "web_chat",
+            "",
+            &[
+                EventSummary {
+                    source_app: "web_chat".into(),
+                    event_type: "chat.message.persisted".into(),
+                    total: 2,
+                },
+                EventSummary {
+                    source_app: "web_chat".into(),
+                    event_type: "<script>bad</script>".into(),
+                    total: 1,
+                },
+            ],
+        );
+        assert!(html.contains("<table"));
+        assert!(html.contains("已提交事件類型明細"));
+        assert!(html.contains("chat.message.persisted"));
+        assert!(html.contains("&lt;script&gt;bad&lt;/script&gt;"));
+        assert!(!html.contains("<script>bad</script>"));
+        assert!(html.contains("全部事件類型：3 筆已提交事件"));
+    }
+}
+
+#[cfg(test)]
+mod day44_logout_csrf_ssr_tests {
+    use super::*;
+    #[test]
+    fn both_leptos_forms_include_distinct_logout_csrf_without_leaking_raw_cookie() {
+        let html = render_dashboard_with_rows_csrf("Chat", 0, "web_chat", "", &[], "logout-hex");
+        let logout = html.split("action=\"/logout\"").nth(1).unwrap();
+        assert!(logout.contains("name=\"csrf\" value=\"logout-hex\""));
+        let change = render_change_password_with_logout(true, "change-hex".into(), "logout-hex".into());
+        assert!(change.contains("name=\"csrf\" value=\"change-hex\""));
+        let logout = change.split("action=\"/logout\"").nth(1).unwrap();
+        assert!(logout.contains("name=\"csrf\" value=\"logout-hex\""));
     }
 }
diff --git a/analytics_apps/analytics_server/src/password_login.rs b/analytics_apps/analytics_server/src/password_login.rs
--- a/analytics_apps/analytics_server/src/password_login.rs
+++ b/analytics_apps/analytics_server/src/password_login.rs
@@ -208,210 +208,220 @@
     let (restricted, raw) = session(&state, &headers).await?;
     let csrf = digest(&format!("change-password:{raw}"));
     Ok(renewed(
-        Html(crate::dashboard_ui::render_change_password(
-            restricted, csrf,
-        )),
-        &raw,
-    ))
-}
-
-#[derive(Deserialize)]
-struct PasswordChange {
-    current_password: String,
-    new_password: String,
-    confirm_password: String,
-    email: String,
-    csrf: String,
-}
-
-async fn change_password(
-    State(state): State<IngestState>,
-    headers: HeaderMap,
-    Form(form): Form<PasswordChange>,
-) -> Result<Response, StatusCode> {
-    if headers.get(header::ORIGIN).and_then(|h| h.to_str().ok()) != Some(ORIGIN) {
-        return Err(StatusCode::FORBIDDEN);
-    }
-    let (_, raw) = session(&state, &headers).await?;
-    if form.csrf != digest(&format!("change-password:{raw}")) {
-        return Err(StatusCode::FORBIDDEN);
-    }
-    let email = form.email.trim().to_ascii_lowercase();
-    let pieces: Vec<_> = email.split('@').collect();
-    if !(8..=128).contains(&form.new_password.len())
-        || form.current_password.is_empty()
-        || form.current_password.len() > 128
-        || form.new_password != form.confirm_password
-        || form.new_password == form.current_password
-        || email.len() > 254
-        || !email.is_ascii()
-        || email
-            .bytes()
-            .any(|b| b.is_ascii_whitespace() || b.is_ascii_control())
-        || pieces.len() != 2
-        || pieces[0].is_empty()
-        || !pieces[1].contains('.')
-        || pieces[1].starts_with('.')
-        || pieces[1].ends_with('.')
-    {
-        return Err(StatusCode::UNPROCESSABLE_ENTITY);
-    }
-    let mut tx = state
-        .pool
-        .begin()
-        .await
-        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
-    // 重驗本次有效 Session，鎖定會員與 Session，不能靠前端或先前 GET 的結果授權。
-    let row: Option<(String, String, String)> = sqlx::query_as(
-        r#"SELECT u.id
-            ,u.user_pd
-            ,s.credential_fingerprint 
-        FROM uac_user u 
-        JOIN uac_login_session s 
-            ON s.user_id=u.id 
-        WHERE s.token_hash=$1 
-            AND s.expires_at>floor(extract(epoch from now()))::bigint 
-            AND u.user_status='active' 
-            AND lower(u.user_acnt)='admin' 
-        FOR UPDATE OF u,s"#,
-    )
-    .bind(digest(&raw))
-    .fetch_optional(&mut *tx)
-    .await
-    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
-    let (id, current, fingerprint) = row.ok_or(StatusCode::UNAUTHORIZED)?;
-    if digest(&current) != fingerprint {
-        return Err(StatusCode::UNAUTHORIZED);
-    }
-    let encoded = tokio::task::spawn_blocking(move || {
-        let parsed = PasswordHash::new(&current).map_err(|_| StatusCode::UNAUTHORIZED)?;
-        Argon2::default()
-            .verify_password(form.current_password.as_bytes(), &parsed)
-            .map_err(|_| StatusCode::UNAUTHORIZED)?;
-        Argon2::default()
-            .hash_password(form.new_password.as_bytes())
-            .map(|h| h.to_string())
-            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)
-    })
-    .await
-    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)??;
-    sqlx::query(
-        r#"UPDATE uac_user 
-            SET user_pd=$1
-                ,user_email=$2
-                ,is_init_pd=FALSE
-                ,updated_at=now() 
-        WHERE id=$3"#,
-    )
-    .bind(encoded)
-    .bind(email)
-    .bind(&id)
-    .execute(&mut *tx)
-    .await
-    .map_err(|e| match e {
-        sqlx::Error::Database(ref d) if d.is_unique_violation() => StatusCode::CONFLICT,
-        _ => StatusCode::SERVICE_UNAVAILABLE,
-    })?;
-    sqlx::query("DELETE FROM uac_login_session WHERE user_id=$1")
-        .bind(id)
-        .execute(&mut *tx)
-        .await
-        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
-    tx.commit()
-        .await
-        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
-    Ok((
-        StatusCode::SEE_OTHER,
-        [
-            (header::LOCATION, "/login"),
-            (
-                header::SET_COOKIE,
-                "analytics_session=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0",
-            ),
-        ],
-    )
-        .into_response())
-}
-
-#[derive(Default, Deserialize)]
-struct DashboardSourceQuery {
-    source: Option<String>,
-    event_type: Option<String>,
-}
-
-async fn dashboard(
-    State(state): State<IngestState>,
-    headers: HeaderMap,
-    Query(query): Query<DashboardSourceQuery>,
-) -> Response {
-    let (restricted, raw) = match session(&state, &headers).await {
-        Ok(v) => v,
-        Err(StatusCode::UNAUTHORIZED) => return Redirect::to("/login").into_response(),
-        Err(e) => return e.into_response(),
-    };
-    if restricted {
-        return renewed(Redirect::to("/change-password"), &raw);
-    }
-
-    let source = query.source.as_deref().unwrap_or("web_chat");
-    let source_label = match source {
-        "web_chat" => "Chat",
-        "desktop_pet" => "Pet",
-        _ => return StatusCode::BAD_REQUEST.into_response(),
-    };
-    let kind = query.event_type.as_deref().unwrap_or("");
-    if !kind.is_empty() && (!crate::valid_event_type(kind) || kind.contains('\0')) {
-        return StatusCode::BAD_REQUEST.into_response();
-    }
-
-    let rows = if kind.is_empty() {
-        summary_for_source(&state.pool, source).await
-    } else {
-        summary_for_source_and_type(&state.pool, source, kind).await
-    };
-    let rows = match rows {
-        Ok(v) => v,
-        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
-    };
-    let Some(total) = crate::summary_total(&rows) else {
-        return StatusCode::SERVICE_UNAVAILABLE.into_response();
-    };
-
-    renewed(
-        Html(crate::dashboard_ui::render_dashboard_with_rows(
-            source_label,
-            total,
-            source,
-            kind,
-            &rows,
-        )),
-        &raw,
-    )
-}
-
-async fn summary(
-    State(state): State<IngestState>,
-    Path(source): Path<String>,
-    headers: HeaderMap,
-) -> Result<Response, StatusCode> {
-    let raw = require_query_session(&state, &headers).await?;
-    if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
-        return Err(StatusCode::BAD_REQUEST);
-    }
-    let rows = summary_for_source(&state.pool, &source)
-        .await
-        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
-    Ok(renewed(Json(rows), &raw))
-}
-
-async fn logout(
-    State(state): State<IngestState>,
-    headers: HeaderMap,
-) -> Result<Response, StatusCode> {
-    if headers.get(header::ORIGIN).and_then(|h| h.to_str().ok()) != Some(ORIGIN) {
-        return Err(StatusCode::FORBIDDEN);
-    }
-    let raw = raw_cookie(&headers).ok_or(StatusCode::UNAUTHORIZED)?;
+        Html(crate::dashboard_ui::render_change_password_with_logout(
+            restricted, csrf, digest(&format!("logout:{raw}")),
+        )),
+        &raw,
+    ))
+}
+
+#[derive(Deserialize)]
+struct PasswordChange {
+    current_password: String,
+    new_password: String,
+    confirm_password: String,
+    email: String,
+    csrf: String,
+}
+
+async fn change_password(
+    State(state): State<IngestState>,
+    headers: HeaderMap,
+    Form(form): Form<PasswordChange>,
+) -> Result<Response, StatusCode> {
+    if headers.get(header::ORIGIN).and_then(|h| h.to_str().ok()) != Some(ORIGIN) {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    let (_, raw) = session(&state, &headers).await?;
+    if form.csrf != digest(&format!("change-password:{raw}")) {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    let email = form.email.trim().to_ascii_lowercase();
+    let pieces: Vec<_> = email.split('@').collect();
+    if !(8..=128).contains(&form.new_password.len())
+        || form.current_password.is_empty()
+        || form.current_password.len() > 128
+        || form.new_password != form.confirm_password
+        || form.new_password == form.current_password
+        || email.len() > 254
+        || !email.is_ascii()
+        || email
+            .bytes()
+            .any(|b| b.is_ascii_whitespace() || b.is_ascii_control())
+        || pieces.len() != 2
+        || pieces[0].is_empty()
+        || !pieces[1].contains('.')
+        || pieces[1].starts_with('.')
+        || pieces[1].ends_with('.')
+    {
+        return Err(StatusCode::UNPROCESSABLE_ENTITY);
+    }
+    let mut tx = state
+        .pool
+        .begin()
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    // 重驗本次有效 Session，鎖定會員與 Session，不能靠前端或先前 GET 的結果授權。
+    let row: Option<(String, String, String)> = sqlx::query_as(
+        r#"SELECT u.id
+            ,u.user_pd
+            ,s.credential_fingerprint 
+        FROM uac_user u 
+        JOIN uac_login_session s 
+            ON s.user_id=u.id 
+        WHERE s.token_hash=$1 
+            AND s.expires_at>floor(extract(epoch from now()))::bigint 
+            AND u.user_status='active' 
+            AND lower(u.user_acnt)='admin' 
+        FOR UPDATE OF u,s"#,
+    )
+    .bind(digest(&raw))
+    .fetch_optional(&mut *tx)
+    .await
+    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    let (id, current, fingerprint) = row.ok_or(StatusCode::UNAUTHORIZED)?;
+    if digest(&current) != fingerprint {
+        return Err(StatusCode::UNAUTHORIZED);
+    }
+    let encoded = tokio::task::spawn_blocking(move || {
+        let parsed = PasswordHash::new(&current).map_err(|_| StatusCode::UNAUTHORIZED)?;
+        Argon2::default()
+            .verify_password(form.current_password.as_bytes(), &parsed)
+            .map_err(|_| StatusCode::UNAUTHORIZED)?;
+        Argon2::default()
+            .hash_password(form.new_password.as_bytes())
+            .map(|h| h.to_string())
+            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)
+    })
+    .await
+    .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)??;
+    sqlx::query(
+        r#"UPDATE uac_user 
+            SET user_pd=$1
+                ,user_email=$2
+                ,is_init_pd=FALSE
+                ,updated_at=now() 
+        WHERE id=$3"#,
+    )
+    .bind(encoded)
+    .bind(email)
+    .bind(&id)
+    .execute(&mut *tx)
+    .await
+    .map_err(|e| match e {
+        sqlx::Error::Database(ref d) if d.is_unique_violation() => StatusCode::CONFLICT,
+        _ => StatusCode::SERVICE_UNAVAILABLE,
+    })?;
+    sqlx::query("DELETE FROM uac_login_session WHERE user_id=$1")
+        .bind(id)
+        .execute(&mut *tx)
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    tx.commit()
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    Ok((
+        StatusCode::SEE_OTHER,
+        [
+            (header::LOCATION, "/login"),
+            (
+                header::SET_COOKIE,
+                "analytics_session=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0",
+            ),
+        ],
+    )
+        .into_response())
+}
+
+#[derive(Default, Deserialize)]
+struct DashboardSourceQuery {
+    source: Option<String>,
+    event_type: Option<String>,
+}
+
+async fn dashboard(
+    State(state): State<IngestState>,
+    headers: HeaderMap,
+    Query(query): Query<DashboardSourceQuery>,
+) -> Response {
+    let (restricted, raw) = match session(&state, &headers).await {
+        Ok(v) => v,
+        Err(StatusCode::UNAUTHORIZED) => return Redirect::to("/login").into_response(),
+        Err(e) => return e.into_response(),
+    };
+    if restricted {
+        return renewed(Redirect::to("/change-password"), &raw);
+    }
+
+    let source = query.source.as_deref().unwrap_or("web_chat");
+    let source_label = match source {
+        "web_chat" => "Chat",
+        "desktop_pet" => "Pet",
+        _ => return StatusCode::BAD_REQUEST.into_response(),
+    };
+    let kind = query.event_type.as_deref().unwrap_or("");
+    if !kind.is_empty() && (!crate::valid_event_type(kind) || kind.contains('\0')) {
+        return StatusCode::BAD_REQUEST.into_response();
+    }
+
+    let rows = if kind.is_empty() {
+        summary_for_source(&state.pool, source).await
+    } else {
+        summary_for_source_and_type(&state.pool, source, kind).await
+    };
+    let rows = match rows {
+        Ok(v) => v,
+        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
+    };
+    let Some(total) = crate::summary_total(&rows) else {
+        return StatusCode::SERVICE_UNAVAILABLE.into_response();
+    };
+
+    renewed(
+        Html(crate::dashboard_ui::render_dashboard_with_rows_csrf(
+            source_label,
+            total,
+            source,
+            kind,
+            &rows,
+            &digest(&format!("logout:{raw}")),
+        )),
+        &raw,
+    )
+}
+
+async fn summary(
+    State(state): State<IngestState>,
+    Path(source): Path<String>,
+    headers: HeaderMap,
+) -> Result<Response, StatusCode> {
+    let raw = require_query_session(&state, &headers).await?;
+    if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
+        return Err(StatusCode::BAD_REQUEST);
+    }
+    let rows = summary_for_source(&state.pool, &source)
+        .await
+        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
+    Ok(renewed(Json(rows), &raw))
+}
+
+#[derive(Deserialize)]
+struct LogoutForm {
+    csrf: String,
+}
+
+async fn logout(
+    State(state): State<IngestState>,
+    headers: HeaderMap,
+    Form(form): Form<LogoutForm>,
+) -> Result<Response, StatusCode> {
+    if headers.get(header::ORIGIN).and_then(|h| h.to_str().ok()) != Some(ORIGIN) {
+        return Err(StatusCode::FORBIDDEN);
+    }
+    let raw = raw_cookie(&headers).ok_or(StatusCode::UNAUTHORIZED)?;
+    if form.csrf != digest(&format!("logout:{raw}")) {
+        return Err(StatusCode::FORBIDDEN);
+    }
     sqlx::query("DELETE FROM uac_login_session WHERE token_hash=$1")
         .bind(digest(&raw))
         .execute(&state.pool)
diff --git a/analytics_apps/analytics_server/tests/day44_logout_csrf.rs b/analytics_apps/analytics_server/tests/day44_logout_csrf.rs
new file mode 100644
--- /dev/null
+++ b/analytics_apps/analytics_server/tests/day44_logout_csrf.rs
@@ -0,0 +1,47 @@
+#![cfg(feature = "postgres-integration")]
+mod support;
+
+use analytics_server::{IngestState, IngestTokens, bootstrap, password_login};
+use axum::{body::Body, http::{Request, StatusCode, header}};
+use std::sync::Arc;
+use support::http::{body_text, response_cookie, send};
+use tower::ServiceExt;
+
+#[sqlx::test(migrations = "../migrations")]
+async fn real_pg_logout_requires_session_bound_form_token(pool: sqlx::PgPool) {
+    bootstrap::initialize(&pool, "lesson-test-password".into()).await.unwrap();
+    let app = password_login::build(IngestState {
+        pool: pool.clone(),
+        tokens: Arc::new(IngestTokens { chat: "chat-fixture".into(), pet: "pet-fixture".into() }),
+    });
+    let login = send(app.clone(), "/login",
+        Some("username=admin&password=lesson-test-password"), None).await;
+    assert_eq!(login.status(), StatusCode::SEE_OTHER);
+    let cookie = response_cookie(&login);
+    let page = send(app.clone(), "/change-password", None, Some(&cookie)).await;
+    assert_eq!(page.status(), StatusCode::OK);
+    let html = body_text(page).await;
+    let logout_form = html.split("action=\"/logout\"").nth(1).unwrap();
+    let csrf = logout_form.split("name=\"csrf\" value=\"").nth(1)
+        .unwrap().split('"').next().unwrap();
+    assert_eq!(csrf.len(), 64);
+    assert_eq!(send(app.clone(), "/logout", Some("csrf=wrong"), Some(&cookie))
+        .await.status(), StatusCode::FORBIDDEN);
+    let still_active: i64 = sqlx::query_scalar("SELECT count(*) FROM uac_login_session")
+        .fetch_one(&pool).await.unwrap();
+    assert_eq!(still_active, 1);
+    let foreign = app.clone().oneshot(Request::post("/logout")
+        .header(header::ORIGIN, "https://foreign.example")
+        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
+        .header(header::COOKIE, &cookie)
+        .body(Body::from(format!("csrf={csrf}"))).unwrap()).await.unwrap();
+    assert_eq!(foreign.status(), StatusCode::FORBIDDEN);
+    let logout = send(app.clone(), "/logout", Some(&format!("csrf={csrf}")), Some(&cookie)).await;
+    assert_eq!(logout.status(), StatusCode::SEE_OTHER);
+    assert_eq!(logout.headers()[header::LOCATION], "/login");
+    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM uac_login_session")
+        .fetch_one(&pool).await.unwrap();
+    assert_eq!(count, 0);
+    assert_eq!(send(app, "/change-password", None, Some(&cookie))
+        .await.status(), StatusCode::UNAUTHORIZED);
+}
```

**斷言映射**：原首次登入受限與密碼更新 Session 撤銷不刪；新增 SSR token 在兩頁的接線測試、PG Router 403／303／Session 計數；`session_cookie::tests` 仍測重複／畸形 Cookie。舊 `admin.rs` legacy route 沒有在此日被宣稱搬層或驗收，後續需單獨清理，避免兩套登入混用。

## ③ 2026-10-30 官方穩定版、manifest、lock 與相容

官方 Rust stable **1.99.1（2026-10-15）**，本機 rustc1.99.1；工具與 lock 詳見本次證據。已核對 `analytics_apps/Cargo.toml`、`analytics_server/Cargo.toml` 與 `analytics_apps/Cargo.lock@831373fcf0824cf5b50e785bf4abe75d3aadc2f8`。以下版本皆由當日官方 Rust release/docs.rs stable 頁確認，預發行排除；無 lock 升級：

| 套件 | manifest → resolved → 最新 stable | 官方來源 |
| --- | --- | --- |
| Rust | stable → 本機1.99.1 → 1.99.1 | https://blog.rust-lang.org/releases/ |
| Axum | ^0.8 → 0.8.9 → 0.8.9 | https://docs.rs/crate/axum/latest |
| Leptos SSR | ^0.8 → 0.8.22 → 0.8.22（0.9 RC 不採用） | https://docs.rs/crate/leptos/latest |
| SQLx PostgreSQL | ^0.9 → 0.9.0 → 0.9.0（MSRV 1.94） | https://docs.rs/crate/sqlx/latest |
| Tokio | ^1.54 → 1.54.1 → 1.54.1 | https://docs.rs/crate/tokio/latest |
| Argon2 | ^0.6 → 0.6.0 → 0.6.0 | https://docs.rs/crate/argon2/latest |
| SHA2 | ^0.11 → 0.11.1 → 0.11.1 | https://docs.rs/crate/sha2/latest |
| UUID | ^1.29 → 1.29.0 → 1.29.0 | https://docs.rs/crate/uuid/latest |
| serde / serde_json | ^1 / ^1 → 1.0.229 / 1.0.154 → 相同 | https://docs.rs/crate/serde/latest · https://docs.rs/crate/serde_json/latest |
| rpassword (bootstrap CLI) | ^7.5 → 7.5.4 → 7.5.4 | https://docs.rs/crate/rpassword/latest |

Rust 2024 edition、SQLx 0.9 MSRV 1.94 在 stable 1.99.1 範圍內；正式 PG、PostgreSQL server 版本／migration、HTTP／SSR、Session Cookie 與 CSRF 需真環境驗證。此日沒有從 metadata 推定建置成功，也未操作正式 DB 或付費 AI。

## ④ 驗證、交付與剩餘缺口

**文件與真套用**：原 Refactor→Green 已真 check／apply；追加 fmt 修補及下節兩個 fixture 修補，保留既有有效斷言。精確 commit／blob／SHA256 於提交後另記；完整官方套件／MSRV Gate 尚未全量重核。

**本地已執行、完整 Gate2 未完成**：最新主分支一般43＋1ignored，真PG完整63通過／0ignored；兩個fixture修正已在4b1008b，最新來源的結果與先前失敗分開記錄。新SHA的真瀏覽器、實際舊PG還原與完整授權旅程未執行。以下重現只能用專用隔離DB：
```bash
git apply --check analytics-day44-refactor.patch
git apply analytics-day44-refactor.patch
git apply --check analytics-day44-green.patch
git apply analytics-day44-green.patch
cargo fmt --manifest-path analytics_apps/Cargo.toml --all -- --check
cargo test --manifest-path analytics_apps/Cargo.toml -p analytics-server
cargo test --manifest-path analytics_apps/Cargo.toml -p analytics-server --features postgres-integration --test day44_logout_csrf -- --include-ignored
cargo check --manifest-path analytics_apps/Cargo.toml --workspace
```
驗證首次改密碼與一般 Dashboard 的登出 token、Origin、重放、閒置 Session、密碼變更撤銷、真 PG commit/rollback、不同來源 token 不得登入。原 Day24/25/26 與完整來源 ACL、帳號角色／稽核、使用者實際舊 PG 備份仍待 Gate2；下一日優先補管理員對檢視者／來源的授權流程。Analytics 目前無 pending FIFO，`last_full_comparison` 不變。核心60分鐘只計 Red/Green 預估15分鐘，Refactor 與交付另計。

## PG fixture 修訂 r3：保留原斷言，補足真實登出前置

`day37_query_cookie` 原空body回415；`day39_export_csv` 原缺CSRF回422。兩測試改成GET真正Dashboard、擷取該Session登出token，再提交form。原重複Cookie拒絕、Session撤銷及CSV斷言不刪。這是同案測試fixture承接，不改伺服器防護、不改產品CI workflow。最新4b1008b已採用下列修正，不要重複套用；7ce重現順序為Refactor→Green→fixture修補→[格式修補](analytics-day44-format-r1.patch)。

[完整附件](analytics-logout-fixture-fix.patch)（本節全文）：

```diff
diff --git a/analytics_apps/analytics_server/tests/day37_query_cookie.rs b/analytics_apps/analytics_server/tests/day37_query_cookie.rs
index 8ced0b6..9823ee0 100644
--- a/analytics_apps/analytics_server/tests/day37_query_cookie.rs
+++ b/analytics_apps/analytics_server/tests/day37_query_cookie.rs
@@ -114,13 +114,27 @@ async fn day37_real_query_routes_reject_ambiguous_session_cookies(pool: sqlx::Pg
         .execute(&pool)
         .await
         .unwrap();
+    let dashboard = query(&app, "/dashboard", &[cookie.clone()], false).await;
+    assert_eq!(dashboard.status(), StatusCode::OK);
+    let html = support::http::body_text(dashboard).await;
+    let csrf = html
+        .split("action=\"/logout\"")
+        .nth(1)
+        .unwrap()
+        .split("name=\"csrf\" value=\"")
+        .nth(1)
+        .unwrap()
+        .split('"')
+        .next()
+        .unwrap();
     let logout = app
         .clone()
         .oneshot(
             Request::post("/logout")
                 .header(header::ORIGIN, "http://127.0.0.1:3100")
                 .header(header::COOKIE, &cookie)
-                .body(Body::empty())
+                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
+                .body(Body::from(format!("csrf={csrf}")))
                 .unwrap(),
         )
         .await
diff --git a/analytics_apps/analytics_server/tests/day39_export_csv.rs b/analytics_apps/analytics_server/tests/day39_export_csv.rs
index 1317796..b7a74a0 100644
--- a/analytics_apps/analytics_server/tests/day39_export_csv.rs
+++ b/analytics_apps/analytics_server/tests/day39_export_csv.rs
@@ -138,13 +138,26 @@ async fn day39_authenticated_admin_exports_only_selected_source_csv(pool: sqlx::
     )
     .await;
     assert_eq!(page.status(), StatusCode::OK);
-    assert!(
-        body_text(page)
-            .await
-            .contains("/api/v1/events/web_chat/export.csv?event_type=chat.message.persisted")
-    );
+    let html = body_text(page).await;
+    assert!(html.contains("/api/v1/events/web_chat/export.csv?event_type=chat.message.persisted"));
+    let csrf = html
+        .split("action=\"/logout\"")
+        .nth(1)
+        .unwrap()
+        .split("name=\"csrf\" value=\"")
+        .nth(1)
+        .unwrap()
+        .split('"')
+        .next()
+        .unwrap();
 
-    let logout = post_form(app.clone(), "/logout", String::new(), Some(&cookie)).await;
+    let logout = post_form(
+        app.clone(),
+        "/logout",
+        format!("csrf={csrf}"),
+        Some(&cookie),
+    )
+    .await;
     assert_eq!(logout.status(), StatusCode::SEE_OTHER);
     assert_eq!(
         get(app, url, Some(&cookie)).await.status(),
```

真PG全套 `--no-fail-fast -- --include-ignored` 修補後63通過。首次link bus error及jobs1成功重試均保存。遠端4b1008b已採用修補，本次只讀快轉並重新驗證PG63通過；當時失敗仍保留在原SHA證據。
