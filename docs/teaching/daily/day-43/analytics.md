# Analytics Day43｜切換 Chat／Pet 來源時保留事件類型精確篩選

> 2026-10-29；唯讀產品 [rust_ai_ring@c6708c4](https://github.com/williechen/rust_ai_ring/tree/c6708c42f21ac919e57a379848b4977d27a1c3d6)；文件起點 `copilot_use/main@d6a701e`；發布前 `next_day.analytics=42`（交付後條件同步至 43）。**正式文件教學（Gate1）：產品尚未套用 Day43 變更，Gate2 尚未驗收。**

## ① 目標／現況／真實鏈

Day42 已將 Leptos SSR Dashboard 事件類型明細連到 `/dashboard?source=...&event_type=...`，後端沿管理員 Session → 原 PostgreSQL 聚合查詢；但是頂部「Chat 事件／Pet 事件」導覽使用固定 URL，切來源時丟失使用者的精確 `event_type`。新增來源切換保留當前篩選（含中文、空白、`&`）；無篩選仍維持原 URL，清除篩選仍返回該來源全量。只改已授權後台 Leptos URL，不新增 SQL／API、繞過 Session 或改機器 ingest token；角色／來源 ACL 仍是原未完成條件。

## Refactor｜Analytics 計數標籤純決策搬層（獨立）

修改前 `analytics_server/src/dashboard_ui.rs` 同時負責 Leptos 元件與 `summary_count_label` 純文案決策；修改後新增 `dashboard_summary.rs` 保存原函式，`lib.rs` 宣告 `mod dashboard_summary;`，`dashboard_ui.rs` import `crate::dashboard_summary::summary_count_label`。原 Day33 `count_label_distinguishes_full_and_exact_kind` 與 SSR HTML escaping 斷言保留；新增零筆、中文類型及來源標籤測試。無跨案、shared crates 或全庫 CI 變更。

```rust
pub(crate) fn summary_count_label(source_label: &str, event_type: &str, total: i64) -> String {
    if event_type.is_empty() {
        format!("{source_label} 全部事件類型：{total} 筆已提交事件")
    } else {
        format!("{source_label} 事件類型「{event_type}」：{total} 筆已提交事件")
    }
}
```

完整新增 module、原 caller 與測試在 [Analytics Refactor patch](analytics-day43-refactor.patch)。下一步獨立盤點後台 Session／CSRF 測試與資料邊界，不將全庫契約計入本案。

### Refactor 逐檔完整 patch、原測試與 SSR caller

獨立 Refactor 將既有 `summary_count_label` 完整移至 `dashboard_summary.rs`，由 `lib.rs` 引入並在正式 `dashboard_ui.rs` 重新接線。保留 Day33 計數與 HTML escaping 契約，新增含零筆／中文事件類型的回歸測試。以下是完整變更內容；附件 [Analytics Refactor patch](analytics-day43-refactor.patch)。

```diff
diff --git a/analytics_apps/analytics_server/src/lib.rs b/analytics_apps/analytics_server/src/lib.rs
--- a/analytics_apps/analytics_server/src/lib.rs
+++ b/analytics_apps/analytics_server/src/lib.rs
@@ -2,7 +2,8 @@
 pub mod bootstrap;
 mod csv;
 pub mod dashboard_ui;
-mod dashboard_url;
+mod dashboard_url;
+mod dashboard_summary;
 pub mod export;
 pub mod password_login;
 pub mod pg_store;
diff --git a/analytics_apps/analytics_server/src/dashboard_ui.rs b/analytics_apps/analytics_server/src/dashboard_ui.rs
--- a/analytics_apps/analytics_server/src/dashboard_ui.rs
+++ b/analytics_apps/analytics_server/src/dashboard_ui.rs
@@ -2,15 +2,9 @@
 
 use analytics_persistence::EventSummary;
 
-use crate::dashboard_url::encode_query_component;
+use crate::dashboard_url::encode_query_component;
+use crate::dashboard_summary::summary_count_label;
 
-fn summary_count_label(source_label: &str, event_type: &str, total: i64) -> String {
-    if event_type.is_empty() {
-        format!("{source_label} 全部事件類型：{total} 筆已提交事件")
-    } else {
-        format!("{source_label} 事件類型「{event_type}」：{total} 筆已提交事件")
-    }
-}
 
 #[component]
 fn DashboardView(
diff --git a/analytics_apps/analytics_server/src/dashboard_summary.rs b/analytics_apps/analytics_server/src/dashboard_summary.rs
new file mode 100644
--- /dev/null
+++ b/analytics_apps/analytics_server/src/dashboard_summary.rs
@@ -0,0 +1,17 @@
+pub(crate) fn summary_count_label(source_label: &str, event_type: &str, total: i64) -> String {
+    if event_type.is_empty() {
+        format!("{source_label} 全部事件類型：{total} 筆已提交事件")
+    } else {
+        format!("{source_label} 事件類型「{event_type}」：{total} 筆已提交事件")
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    #[test]
+    fn committed_count_caption_preserves_source_kind_and_zero() {
+        assert_eq!(summary_count_label("Chat", "", 2), "Chat 全部事件類型：2 筆已提交事件");
+        assert_eq!(summary_count_label("Pet", "pet.睡覺", 0), "Pet 事件類型「pet.睡覺」：0 筆已提交事件");
+    }
+}
```

## ② Red → 完整 Green（核心預估 15 分鐘）

**Red：** 新增 `dashboard_ui::day43_source_switch_tests`，以真 `render_dashboard_filtered("Chat", 1, "web_chat", "chat.中文 &")` 產生 Leptos SSR，要求兩個來源連結均包含 `event_type=chat.%E4%B8%AD%E6%96%87%20%26`，HTML attribute 以 `&amp;` escape，原始字串不得直接進入 URL；無篩選時保留舊連結。原 main 來源切換丟失參數，為可見行為 Red；**未實際執行 Red**。原 Day29/32/33/40/41/42 的 SSR、CSV、PG、授權測試不刪。

**Green：** 沿 Day42 的 `dashboard_url::encode_query_component`，不複製 encoder：

```rust
pub(crate) fn source_switch_url(source: &str, event_type: &str) -> String {
    if event_type.is_empty() {
        format!("/dashboard?source={source}")
    } else {
        format!("/dashboard?source={source}&event_type={}", encode_query_component(event_type))
    }
}
```

在 `DashboardView` 先建立 `chat_source_url = source_switch_url("web_chat", &event_type)`、`pet_source_url = source_switch_url("desktop_pet", &event_type)`，將原兩個靜態 href 改成 Leptos `href=chat_source_url`／`href=pet_source_url`。既有 `aria-current`、CSV、清除篩選、表格及 `render_dashboard_with_rows` 保留。完整 [Analytics Green patch](analytics-day43-green.patch) 含 URL helper、兩組 Red 測試與正式 UI caller；無 Cargo、lock、migration 或新 crate。切來源後資料仍由原授權查詢返回，不能由 URL 連結存在推定來源 ACL 已通過。

### Red／Green 逐檔完整修改與測試

新 Red 於原來源驗證來源切換不能保留 `event_type`；Green 沿用既有 `encode_query_component` 並更新實際 Leptos Dashboard 來源連結，補上 UTF-8、`&`、無篩選 SSR 斷言。以下為完整 patch 本文；附件 [Analytics Green patch](analytics-day43-green.patch)。**尚未實測，不推定正式 PG／角色 ACL 已驗收。**

```diff
diff --git a/analytics_apps/analytics_server/src/dashboard_ui.rs b/analytics_apps/analytics_server/src/dashboard_ui.rs
--- a/analytics_apps/analytics_server/src/dashboard_ui.rs
+++ b/analytics_apps/analytics_server/src/dashboard_ui.rs
@@ -2,7 +2,7 @@
 
 use analytics_persistence::EventSummary;
 
-use crate::dashboard_url::encode_query_component;
+use crate::dashboard_url::{encode_query_component, source_switch_url};
 use crate::dashboard_summary::summary_count_label;
 
 
@@ -15,7 +15,9 @@
     rows: Vec<(String, i64)>,
 ) -> impl IntoView {
     let chat_current = (source_label == "Chat").then_some("page");
-    let pet_current = (source_label == "Pet").then_some("page");
+    let pet_current = (source_label == "Pet").then_some("page");
+    let chat_source_url = source_switch_url("web_chat", &event_type);
+    let pet_source_url = source_switch_url("desktop_pet", &event_type);
     let clear_url = format!("/dashboard?source={source}");
     let export_url = if event_type.is_empty() {
         format!("/api/v1/events/{source}/export.csv")
@@ -44,10 +46,10 @@
                 <main>
                     <h1>"Analytics 管理後台"</h1>
                     <nav aria-label="事件來源">
-                        <a href="/dashboard?source=web_chat" aria-current=chat_current>
+                        <a href=chat_source_url aria-current=chat_current>
                             "Chat 事件"
                         </a>
-                        <a href="/dashboard?source=desktop_pet" aria-current=pet_current>
+                        <a href=pet_source_url aria-current=pet_current>
                             "Pet 事件"
                         </a>
                     </nav>
@@ -239,8 +241,22 @@
     )
 }
 
-#[cfg(test)]
-mod day42_kind_navigation_tests {
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
     use super::*;
     #[test]
     fn kind_carow_links_to_real_exact_filter_and_escapes_html() {
diff --git a/analytics_apps/analytics_server/src/dashboard_url.rs b/analytics_apps/analytics_server/src/dashboard_url.rs
--- a/analytics_apps/analytics_server/src/dashboard_url.rs
+++ b/analytics_apps/analytics_server/src/dashboard_url.rs
@@ -10,11 +10,27 @@
     result
 }
 
-#[cfg(test)]
-mod tests {
+/// Keep the exact event-type filter when switching the authenticated dashboard source.
+pub(crate) fn source_switch_url(source: &str, event_type: &str) -> String {
+    if event_type.is_empty() {
+        format!("/dashboard?source={source}")
+    } else {
+        format!("/dashboard?source={source}&event_type={}", encode_query_component(event_type))
+    }
+}
+
+#[cfg(test)]
+mod tests {
     use super::encode_query_component;
-    #[test]
-    fn existing_csv_url_contract_keeps_utf8_and_reserved_characters() {
+    #[test]
+    fn source_switch_preserves_encoded_filter_and_empty_semantics() {
+        assert_eq!(super::source_switch_url("desktop_pet", "a &中文"),
+            "/dashboard?source=desktop_pet&event_type=a%20%26%E4%B8%AD%E6%96%87");
+        assert_eq!(super::source_switch_url("web_chat", ""), "/dashboard?source=web_chat");
+    }
+
+    #[test]
+    fn existing_csv_url_contract_keeps_utf8_and_reserved_characters() {
         assert_eq!(
             encode_query_component("a &中文"),
             "a%20%26%E4%B8%AD%E6%96%87"
```

## ③ 版本（2026-10-29）

Rust stable [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)、Leptos [0.8.22](https://docs.rs/crate/leptos/latest)、Axum [0.8.9](https://docs.rs/crate/axum/latest)、SQLx [0.9.0](https://docs.rs/crate/sqlx/latest)、Tokio [1.54.1](https://docs.rs/crate/tokio/latest)、UUID [1.29.0](https://docs.rs/crate/uuid/latest)、Argon2 [0.6.0](https://docs.rs/crate/argon2/latest)。產品 workspace `0.2.42`，Analytics lock resolved 依 [Day42 最新版本表](../day-42/versions-2026-10-29.md)。實際 PostgreSQL／本機 rustc／遞迴 MSRV 未執行，因此不宣稱產品相容驗收通過；不升級產品。當日官方穩定版與既有 lock／已知限制另見 [Day43 版本基線](versions.md)。

## ④ 驗證／套用／驗收／下一步

已做：讀回精確產品的 `dashboard_ui.rs`、`dashboard_url.rs`、`lib.rs`，確認現行 caller／SSR 斷言與 patch 修改片段唯一。**未做**：真 `git apply --check`、Red、Cargo／SSR、隔離 PostgreSQL、HTTP、瀏覽器與 CI。Gate1 文件交付後依獨立讀回證據結案；Gate2 未執行，不追認 Day42 舊 SHA 測試。隔離 checkout `c6708c42f21ac919e57a379848b4977d27a1c3d6` 後先 Refactor 再 Green：

```bash
git apply --check analytics-day43-refactor.patch && git apply analytics-day43-refactor.patch
git apply --check analytics-day43-green.patch && git apply analytics-day43-green.patch
cargo fmt --all --manifest-path analytics_apps/Cargo.toml -- --check
cargo test --locked --manifest-path analytics_apps/Cargo.toml -p analytics-server
cargo test --locked --manifest-path analytics_apps/Cargo.toml -p analytics-server -p analytics-persistence --features postgres-integration -- --include-ignored
```

人工驗收：管理員登入→選「chat.中文 &」→切 Pet／切回 Chat 均保留精確類型→清除篩選回全量→核對真 PG 總數、CSV 與匿名／無權 401/403。一般檢視者來源 ACL／SSE 仍待獨立驗收。核心 15 分鐘僅 Red/Green，Refactor、版本、驗證與提交另計。Gate1 文件交付／產品套用／驗收分列；本案依已交付證據條件同步 `next_day`，不增加重構 Day；FIFO、`last_full_comparison` 不變。


## 文件交付與來源證據

原 [Day43 候選](../../drafts/day-43/analytics.md) 保留原 commit／blob，六份 patch 的嚴格逐行來源核對詳 [候選證據](../../drafts/day-43/candidate-evidence.json)。先 Refactor 後 Green；記憶體內 patch 重播不代表原生 `git apply`、SSR／PG／HTTP 測試。正式 [版本基線](versions.md) 與 [文章／patch 交付證據](delivery-verification.json) 分別記 Gate1 和 Gate2，不寫入產品程式。
