# Chat Day43｜房間搜尋一鍵清除並恢復已載入清單

> 2026-10-29；唯讀產品來源 [rust_ai_ring@c6708c4](https://github.com/williechen/rust_ai_ring/tree/c6708c42f21ac919e57a379848b4977d27a1c3d6)；文件來源起點 copilot_use/main `d6a701eb8de9eaeb04d4145e436ce75485bb967e`；發布前 `next_day.chat=42`（交付後條件同步至 43）。**正式文件教學（Gate1）：產品尚未套用 Day43 變更，Gate2 尚未驗收。**

## ① 目標／現況／前置

Day42 已將房名與錯誤狀態接入正式 Chat UI。現有 `RoomList` 可搜尋、讀取並在失敗時保留上次成功清單，但使用者輸入搜尋詞後，缺少不發送新 HTTP 請求即可回到完整快照的明確操作。新增「清除搜尋」按鈕：載入完成後清空查詢詞、恢復全部已載入房間；載入中禁用，不更動 rows、Session、HTTP/WS/SQLite 或登入 ACL。清除前後均沿用 `room_list_state::filter_rooms` 與 `room_list_feedback`，沒有第二份清單。原始需求仍以公開房間→登入→授權進房為優先；此切片不宣稱完成逐房 ACL。

## Refactor｜Chat Session 入口狀態搬層（與功能 Green 獨立）

修改前 `chat_ui/src/lib.rs` 同時持有 `SessionEntry` 四狀態與 `session_entry_display` 的純導覽決策，`HomePage` 則負責 Leptos signals 和 `room_api::session_status`。修改後將原完整決策搬至新 `chat_ui/src/session_entry.rs`，在 lib.rs 宣告 `mod session_entry;` 並 `use session_entry::{session_entry_display, SessionEntry};`。原 Day40 `entry_does_not_claim_manage_without_confirmed_session` 保留，新增測試確認 Checking、SignedOut、Unavailable 均不冒稱管理，SignedIn 才顯示管理。沒有新增 crate、feature、workflow；不碰 Pet、Analytics 或全庫工程項目。

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum SessionEntry { Checking, SignedOut, SignedIn, Unavailable }

pub(super) fn session_entry_display(state: SessionEntry) -> (&'static str, &'static str, &'static str) {
    match state {
        SessionEntry::Checking => ("/signin", "登入", "確認登入狀態中"),
        SessionEntry::SignedOut => ("/signin", "登入", ""),
        SessionEntry::SignedIn => ("/account", "管理", ""),
        SessionEntry::Unavailable => ("/signin", "登入", "無法確認登入狀態；可使用登入入口"),
    }
}
```

完整 module、呼叫端、原斷言承接與新增測試見 [Refactor patch](chat-day43-refactor.patch)。先套 Refactor；回復時在 Green 之後逆序撤回。下一個 Chat Refactor 應盤點房間 API 重複的 fetch/error 轉換，保留現有 HTTP 契約與真呼叫者。

### Refactor 逐檔完整 patch、caller、原斷言與新增測試

下列是可直接套用到指定產品 SHA 的**完整 Refactor diff**（非僅摘要）。它移動純 Session 入口決策，保留原 Day40 斷言，同時新增 fail-closed 測試；正式 Leptos `HomePage` 呼叫不改。附件為 [Chat Refactor patch](chat-day43-refactor.patch)。

```diff
diff --git a/chat_apps/chat_ui/src/lib.rs b/chat_apps/chat_ui/src/lib.rs
--- a/chat_apps/chat_ui/src/lib.rs
+++ b/chat_apps/chat_ui/src/lib.rs
@@ -5,7 +5,8 @@
 #[cfg(target_arch = "wasm32")]
 mod room_api;
 
-mod room_list_state;
+mod room_list_state;
+mod session_entry;
 
 #[cfg(all(test, feature = "ssr"))]
 mod signin_contract;
@@ -21,7 +22,8 @@
     path,
 };
 
-use room_list_state::{filter_rooms, room_status_text};
+use room_list_state::{filter_rooms, room_status_text};
+use session_entry::{session_entry_display, SessionEntry};
 
 #[cfg(any(target_arch = "wasm32", test))]
 use room_list_state::refresh_result_view;
@@ -107,22 +109,6 @@
     }
 }
 
-#[derive(Clone, Copy, PartialEq, Eq, Debug)]
-enum SessionEntry {
-    Checking,
-    SignedOut,
-    SignedIn,
-    Unavailable,
-}
-
-fn session_entry_display(state: SessionEntry) -> (&'static str, &'static str, &'static str) {
-    match state {
-        SessionEntry::Checking => ("/signin", "登入", "確認登入狀態中"),
-        SessionEntry::SignedOut => ("/signin", "登入", ""),
-        SessionEntry::SignedIn => ("/account", "管理", ""),
-        SessionEntry::Unavailable => ("/signin", "登入", "無法確認登入狀態；可使用登入入口"),
-    }
-}
 
 #[component]
 fn HomePage() -> impl IntoView {
diff --git a/chat_apps/chat_ui/src/session_entry.rs b/chat_apps/chat_ui/src/session_entry.rs
new file mode 100644
--- /dev/null
+++ b/chat_apps/chat_ui/src/session_entry.rs
@@ -0,0 +1,29 @@
+#[derive(Clone, Copy, PartialEq, Eq, Debug)]
+pub(super) enum SessionEntry {
+    Checking,
+    SignedOut,
+    SignedIn,
+    Unavailable,
+}
+
+pub(super) fn session_entry_display(state: SessionEntry) -> (&'static str, &'static str, &'static str) {
+    match state {
+        SessionEntry::Checking => ("/signin", "登入", "確認登入狀態中"),
+        SessionEntry::SignedOut => ("/signin", "登入", ""),
+        SessionEntry::SignedIn => ("/account", "管理", ""),
+        SessionEntry::Unavailable => ("/signin", "登入", "無法確認登入狀態；可使用登入入口"),
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    #[test]
+    fn session_navigation_is_fail_closed_and_preserves_existing_contract() {
+        assert_eq!(session_entry_display(SessionEntry::Checking).0, "/signin");
+        assert_eq!(session_entry_display(SessionEntry::SignedOut).1, "登入");
+        assert_eq!(session_entry_display(SessionEntry::Unavailable).0, "/signin");
+        assert!(session_entry_display(SessionEntry::Unavailable).2.contains("無法確認"));
+        assert_eq!(session_entry_display(SessionEntry::SignedIn), ("/account", "管理", ""));
+    }
+}
```

## ② Red → 完整 Green（核心預估 15 分鐘）

**Red：** 新增 `room_list_state::day43_clear_room_search_tests`：兩個已載入房間、搜尋 Rust 僅一筆；清除後查詢為空且兩筆仍在；未載入時仍提示「請先讀取房間列表」。另在現有 SSR Router fixture 的 `signin_contract.rs` 新增 `day43_room_list_has_clear_search_control`，要求真實 `RoomList` 有「清除搜尋」按鈕。原 main 缺 `clear_room_search` 與按鈕，屬缺符號／可見行為 Red；**未實際執行 Red**。Day37/38/39/40/42 的搜尋、錯誤、Session、房名斷言全部保留。

**Green：** `room_list_state.rs` 增加完整純決策單元：

```rust
pub(super) fn clear_room_search(loaded: bool, rows: &[(String, String)]) -> (String, String) {
    (String::new(), room_list_feedback(loaded, rows.len(), rows.len()).to_owned())
}
```

`chat_ui/src/lib.rs::RoomList` 在兩個既有查詢按鈕後新增以下真 Leptos 操作；沿用 `rows`、`query`、`has_snapshot`、`loading`、`status` signals：

```rust
<button
    type="button"
    disabled=move || loading.get() || query.get().is_empty()
    on:click=move |_| {
        let (cleared, feedback) =
            clear_room_search(has_snapshot.get_untracked(), &rows.get_untracked());
        set_query.set(cleared);
        set_status.set(feedback);
    }
>"清除搜尋"</button>
```

完整 [Green patch](chat-day43-green.patch) 包含 module import、按鈕、純函式及兩組新測試；不修改 Cargo.toml、Cargo.lock、migration 或服務端。既有查詢錯誤時保留快照的斷言不刪；清除只重置本地顯示，不代表重新成功查詢或新增授權。

### Red／Green 逐檔完整變更及測試

以下是新增的完整 Green diff，含可驗證的 Red test source（原分支因缺符號／控制項為 Red）、正式 `RoomList` caller 與既有測試保留接線。**未實際執行，不宣稱測試已綠。** 附件 [Chat Green patch](chat-day43-green.patch)。

```diff
diff --git a/chat_apps/chat_ui/src/lib.rs b/chat_apps/chat_ui/src/lib.rs
--- a/chat_apps/chat_ui/src/lib.rs
+++ b/chat_apps/chat_ui/src/lib.rs
@@ -22,7 +22,7 @@
     path,
 };
 
-use room_list_state::{filter_rooms, room_status_text};
+use room_list_state::{clear_room_search, filter_rooms, room_status_text};
 use session_entry::{session_entry_display, SessionEntry};
 
 #[cfg(any(target_arch = "wasm32", test))]
@@ -433,7 +433,17 @@
                     { set_loading.set(false); }
 
             }>"讀取／重新整理房間"</button>
-            <div class="room-results" aria-live="polite">
+            <button
+                type="button"
+                disabled=move || loading.get() || query.get().is_empty()
+                on:click=move |_| {
+                    let (cleared, feedback) =
+                        clear_room_search(has_snapshot.get_untracked(), &rows.get_untracked());
+                    set_query.set(cleared);
+                    set_status.set(feedback);
+                }
+            >"清除搜尋"</button>
+            <div class="room-results" aria-live="polite">
             {move || {
                 let all = rows.get();
                 let visible = filter_rooms(&all, &query.get());
diff --git a/chat_apps/chat_ui/src/room_list_state.rs b/chat_apps/chat_ui/src/room_list_state.rs
--- a/chat_apps/chat_ui/src/room_list_state.rs
+++ b/chat_apps/chat_ui/src/room_list_state.rs
@@ -51,8 +51,13 @@
     room_list_feedback(loaded, total, visible).to_owned()
 }
 
-#[cfg(test)]
-mod tests {
+/// Reset the local search without discarding the last successful room snapshot.
+pub(super) fn clear_room_search(loaded: bool, rows: &[(String, String)]) -> (String, String) {
+    (String::new(), room_list_feedback(loaded, rows.len(), rows.len()).to_owned())
+}
+
+#[cfg(test)]
+mod tests {
     use super::*;
 
     #[test]
@@ -149,8 +154,23 @@
     }
 }
 
-#[cfg(test)]
-mod day42_status_refactor_tests {
+#[cfg(test)]
+mod day43_clear_room_search_tests {
+    use super::*;
+    #[test]
+    fn clear_restores_all_visible_rooms_without_discarding_snapshot() {
+        let rows = vec![("a".into(), "Rust".into()), ("b".into(), "Pet".into())];
+        assert_eq!(filter_rooms(&rows, "Rust").len(), 1);
+        let (query, feedback) = clear_room_search(true, &rows);
+        assert_eq!(query, "");
+        assert_eq!(feedback, "請選擇房間");
+        assert_eq!(filter_rooms(&rows, &query), rows);
+        assert_eq!(clear_room_search(false, &[]).1, "請先讀取房間列表");
+    }
+}
+
+#[cfg(test)]
+mod day42_status_refactor_tests {
     use super::room_status_text;
     #[test]
     fn loading_error_empty_and_found_contract_survives_move() {
diff --git a/chat_apps/chat_ui/src/signin_contract.rs b/chat_apps/chat_ui/src/signin_contract.rs
--- a/chat_apps/chat_ui/src/signin_contract.rs
+++ b/chat_apps/chat_ui/src/signin_contract.rs
@@ -27,8 +27,19 @@
     assert!(html.contains("/signup"));
 }
 
-#[test]
-fn existing_signin_is_the_single_login_form() {
+#[test]
+fn day43_room_list_has_clear_search_control() {
+    let owner = Owner::new();
+    let html = owner.with(|| {
+        provide_context(RequestUrl::new("http://localhost/"));
+        view! { <Router><crate::RoomList /></Router> }.to_html()
+    });
+    assert!(html.contains("清除搜尋"));
+    assert!(html.contains("aria-label=\"搜尋房間\""));
+}
+
+#[test]
+fn existing_signin_is_the_single_login_form() {
     let html = render_signin();
     assert_eq!(html.matches("<form").count(), 1);
     for marker in [
```

## ③ 版本與相容性（2026-10-29）

官方 Rust stable [1.99.1（2026-10-15）](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)，Leptos [0.8.22](https://docs.rs/crate/leptos/latest)、Leptos Router [0.8.17](https://docs.rs/crate/leptos_router/latest)、Axum [0.8.9](https://docs.rs/crate/axum/latest)、SQLx [0.9.0](https://docs.rs/crate/sqlx/latest)、Tokio [1.54.1](https://docs.rs/crate/tokio/latest)、UUID [1.29.0](https://docs.rs/crate/uuid/latest)。來源四 workspace version `0.2.42`；Chat lock resolved 見 [Day42 最新 lock](../day-42/versions-2026-10-29.md)。本輪沒有讀回實際 rustc、全遞迴 MSRV 或 SSR/WASM 編譯；本地編譯與遞迴 MSRV 屬未驗收項，不能冒充相容通過；不自動升級，官方版號與既有 lock／已知 MSRV 見 [Day43 版本基線](versions.md)。

## ④ 驗證／套用／驗收與下一步

已做：GitHub 精確產品來源、正式 UI caller、SSR fixture、規則及前篇讀回；patch 原始片段唯一比對與逐行 hunk 建構。**未做**：真實 `git apply --check`、Red、`cargo fmt/check/test`、SSR/WASM、瀏覽器、HTTP/SQLite/CI；不得宣稱 Green 通過。隔離副本於 `c6708c42f21ac919e57a379848b4977d27a1c3d6` 先套 Refactor 再 Green，執行：

```bash
git apply --check chat-day43-refactor.patch && git apply chat-day43-refactor.patch
git apply --check chat-day43-green.patch && git apply chat-day43-green.patch
cargo fmt --all --manifest-path chat_apps/Cargo.toml -- --check
cargo test --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --features ssr
cargo check --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --features hydrate --target wasm32-unknown-unknown
```

人工驗收：首頁首次自動載入→輸入 Rust 看到部分房間→按清除恢復原清單→重試錯誤後保留上次快照→載入中按鈕禁用→匿名點房仍需登入與 ACL。核心 15 分鐘只計 Red/Green；Refactor、版本、補教、提交、驗收另計。**Gate1 文件已交付／產品未套用／Gate2 未執行**；本案按獨立證據條件同步 `next_day`，FIFO 與 `last_full_comparison` 不變。


## 文件交付與來源證據

本頁內容直接取自原 [Day43 候選](../../drafts/day-43/chat.md)，保留候選原始 commit 與 blob；六份 patch 的嚴格上下文／增刪行數檢查見 [候選證據](../../drafts/day-43/candidate-evidence.json)，但未執行原生 `git apply` 或 Cargo。先 Refactor 後 Green。發布只建立 `copilot_use` 文檔／patch，未修改 `rust_ai_ring`；完整版本與 Gate1/Gate2 分工見 [版本基線](versions.md)，正式 commit/blob/SHA256 見 [Day43 交付證據](delivery-verification.json)。
