# Chat Day38 r4｜搜尋失敗顯示在真正的 Leptos 房間列表，保留既有連結

> **2026-10-26 r4修訂**。產品唯讀來源 `9b89656f964b99e0cb89ba421855704a35c5ecb9`。功能Green與本案Refactor已在完整隔離候選驗證；產品checkout未套用。Red類型、測試／DB結果、環境失敗與未驗收項目分列於④及證據檔。

本修訂保留[原版](chat.md)，以當前精確來源提供[功能Green完整patch](chat-r2.patch)。核心60分鐘只計②主Red／Green；同篇本案Refactor獨立段落與其餘作業另計。這是同Day修訂，next_day維持39。

從未改過的來源使用完整patch；若先手動加入Red測試，依Green正文補程式，不重複套入已含測試的patch。來源有本地變更先審查差異，不覆寫。完整patch的SHA256：`fecb9f84591031f0173a598de5ad30857161ce2415b63e9326c38efb95fec745`。

規則精確來源：copilot_use `1e6b8dcfa49d08b4a0af13ee4b602ac7664d2a64`，共用規則blob `34a2847c8628388f5bdb47c64a5f613b80ed3606`；本案Refactor與②Red／Green並列，功能程式與patch沿用r2精確來源。

## ① 學習大綱：承接 Day37 的真實 UI 缺口

[上一篇 Day37 r3](../day-37/chat-r3.md) 在精確來源 648b903 已確認 `apply_room_search` 更新 `status` Signal，卻沒有把 `status.get()` 接到 RoomList DOM；真瀏覽器 HTTP 500 後保留房間列，但顯示「請選擇房間」，而不是錯誤／重試。最新 9b89656f964b99e0cb89ba421855704a35c5ecb9 的 `chat_apps/chat_ui/src/lib.rs` 第 344 行附近仍是 `room_list_feedback(has_snapshot, all.len(), visible.len())`，因此不是重做 Day37 的 reducer。選一個能被使用者看到的 UI 邊界修補：**載入／搜尋中顯示進度，失敗顯示可重試原因，成功顯示空／無符合／請選房；保留先前房間連結**。

正式路徑：`RoomList`（Leptos SSR + hydrate）→ `room_api::search_rooms/list_rooms`（WASM fetch）→ 現有 `chat-server` Axum 房間 API → `chat-application`／`chat-persistence` SQLite → `apply_room_search`／`refresh_result_view` → 真實 `role="status"` DOM。**不新增 API、SQL、Session 或新 crate**；登入／管理文字、首頁首次自動讀取、匿名選房登入回跳、作者／歷史／ACL 是獨立缺口，不能因本篇通過而結案。

## Refactor

### Chat：抽出UI列表狀態模組與去除重複測試

本段只處理Chat自身，與②Red／Green並列，不以當日功能切片或Green完成作為重構邊界。將filter、refresh與feedback移到UI自身的room_list_state模組，讓正式Signal caller與測試共用同一組狀態函式。刪除只有測試呼叫的refreshed_room_list前，先把其有效斷言轉到正式使用的refresh_result_view。這是Chat UI內的責任整理，不涉及其它專案或全庫CI。

### 完整變更

精確產品來源：`9b89656f964b99e0cb89ba421855704a35c5ecb9`。下列patch包含必要的新模組、完整程式／測試與正式caller接線，不是只有目錄或介面骨架。沒有新增Cargo依賴、features、lock、migration或全庫workflow。

[下載原始patch](chat-r4-refactor.patch)（SHA256：`f2ea5c0b259045cac9d649b91ba5c298ea87fc3b54bdbece9c92abfff0da852e`）。請使用原始檔以保留既有CRLF；先檢查再套用，不覆寫本地變更。

```diff
--- a/chat_apps/chat_ui/src/lib.rs
+++ b/chat_apps/chat_ui/src/lib.rs
@@ -1,3 +1,7 @@
+mod room_list_state;
+use room_list_state::{filter_rooms, room_list_feedback};
+#[cfg(any(target_arch = "wasm32", test))]
+use room_list_state::refresh_result_view;
 mod chat_socket;
 
 pub mod account;
@@ -30,62 +34,13 @@
     }
 }
 
-fn filter_rooms(rows: &[(String, String)], query: &str) -> Vec<(String, String)> {
-    let needle = query.trim().to_lowercase();
-    rows.iter()
-        .filter(|(_, name)| name.to_lowercase().contains(&needle))
-        .cloned()
-        .collect()
-}
-
-fn room_list_feedback(loaded: bool, total: usize, visible: usize) -> &'static str {
-    if !loaded {
-        "請先讀取房間列表"
-    } else if total == 0 {
-        "尚無房間"
-    } else if visible == 0 {
-        "沒有符合條件的房間"
-    } else {
-        "請選擇房間"
-    }
-}
-
-#[cfg(any(target_arch = "wasm32", test))]
-fn refreshed_room_list(
-    previous: Vec<(String, String)>,
-    result: Result<Vec<(String, String)>, String>,
-) -> (Vec<(String, String)>, String) {
-    match result {
-        Ok(value) => {
-            let status = if value.is_empty() {
-                "尚無房間"
-            } else {
-                "請選擇房間"
-            };
-            (value, status.to_owned())
-        }
-        Err(message) => (previous, format!("{message}，請重試")),
-    }
-}
-
-#[cfg(any(target_arch = "wasm32", test))]
-fn refresh_result_view(
-    previous: Vec<(String, String)>,
-    previous_query: String,
-    result: Result<Vec<(String, String)>, String>,
-) -> (Vec<(String, String)>, String, String) {
-    match result {
-        Ok(next) => {
-            let status = if next.is_empty() {
-                "尚無房間"
-            } else {
-                "請選擇房間"
-            };
-            (next, String::new(), status.to_owned())
-        }
-        Err(error) => (previous, previous_query, format!("{error}，請重試")),
-    }
-}
+
+
+
+
+
+
+
 
 #[cfg(any(target_arch = "wasm32", test))]
 fn apply_room_search(
@@ -450,84 +405,7 @@
     }
 }
 
-#[cfg(test)]
-mod day16_room_search_tests {
-    use super::filter_rooms;
-    #[test]
-    fn day19_empty_success_replaces_previous_rows() {
-        let (rows, status) =
-            super::refreshed_room_list(vec![("old".into(), "舊房間".into())], Ok(vec![]));
-        assert!(rows.is_empty());
-        assert_eq!(status, "尚無房間");
-    }
-
-    #[test]
-    fn day19_failure_preserves_complete_rows_and_message() {
-        let old = vec![
-            ("one".into(), "同名房間".into()),
-            ("two".into(), "同名房間".into()),
-        ];
-        let (rows, status) = super::refreshed_room_list(old.clone(), Err("讀取失敗：503".into()));
-        assert_eq!(rows, old);
-        assert_eq!(status, "讀取失敗：503，請重試");
-    }
-
-    #[test]
-    fn day19_refresh_failure_keeps_previous_room_links() {
-        let old = vec![("room_1".into(), "Rust 技術交流".into())];
-        let (rows, status) = super::refreshed_room_list(old.clone(), Err("503".into()));
-        assert_eq!(rows, old);
-        assert!(status.contains("請重試"));
-
-        let (rows, status) =
-            super::refreshed_room_list(old, Ok(vec![("room_2".into(), "生活閒聊".into())]));
-        assert_eq!(rows[0].0, "room_2");
-        assert_eq!(status, "請選擇房間");
-    }
-
-    #[test]
-    fn search_matches_chinese_names_and_preserves_ids() {
-        let rooms = vec![
-            ("room_1".to_owned(), "Rust 技術交流".to_owned()),
-            ("room_2".to_owned(), "生活閒聊".to_owned()),
-        ];
-        assert_eq!(filter_rooms(&rooms, " 技術 "), vec![rooms[0].clone()]);
-        assert_eq!(filter_rooms(&rooms, ""), rooms);
-        assert!(filter_rooms(&rooms, "不存在").is_empty());
-    }
-
-    #[test]
-    fn day18_query_filter_preserves_room_ids() {
-        let rows = vec![
-            ("room_1".to_string(), "Rust 技術交流".to_string()),
-            ("room_2".to_string(), "生活閒聊".to_string()),
-        ];
-        assert_eq!(super::filter_rooms(&rows, " 技術 "), vec![rows[0].clone()]);
-        assert_eq!(super::filter_rooms(&rows, " "), rows);
-    }
-
-    #[test]
-    fn day20_refresh_success_reveals_full_snapshot() {
-        let old = vec![("a".into(), "Rust".into())];
-        let fresh = vec![("a".into(), "Rust".into()), ("b".into(), "Pet".into())];
-        let (rows, query, status) =
-            super::refresh_result_view(old, "Rust".into(), Ok(fresh.clone()));
-        assert_eq!(rows, fresh);
-        assert_eq!(query, "");
-        assert_eq!(status, "請選擇房間");
-        assert_eq!(super::filter_rooms(&rows, &query).len(), 2);
-    }
-
-    #[test]
-    fn day20_refresh_failure_keeps_query_and_rows() {
-        let old = vec![("a".into(), "Rust".into())];
-        let (rows, query, status) =
-            super::refresh_result_view(old.clone(), "Rust".into(), Err("503".into()));
-        assert_eq!(rows, old);
-        assert_eq!(query, "Rust");
-        assert_eq!(status, "503，請重試");
-    }
-}
+
 
 #[cfg(all(test, feature = "ssr"))]
 mod day16_layout_tests {
@@ -599,18 +477,7 @@
     }
 }
 
-#[cfg(test)]
-mod day33_room_list_feedback_tests {
-    use super::room_list_feedback;
-
-    #[test]
-    fn distinguish_not_loaded_empty_filtered_and_found() {
-        assert_eq!(room_list_feedback(false, 0, 0), "請先讀取房間列表");
-        assert_eq!(room_list_feedback(true, 0, 0), "尚無房間");
-        assert_eq!(room_list_feedback(true, 3, 0), "沒有符合條件的房間");
-        assert_eq!(room_list_feedback(true, 3, 2), "請選擇房間");
-    }
-}
+
 
 #[cfg(all(test, feature = "ssr"))]
 mod day34_existing_signin_tests {
--- /dev/null
+++ b/chat_apps/chat_ui/src/room_list_state.rs
@@ -0,0 +1,128 @@
+pub(super) fn filter_rooms(rows: &[(String, String)], query: &str) -> Vec<(String, String)> {
+    let needle = query.trim().to_lowercase();
+    rows.iter()
+        .filter(|(_, name)| name.to_lowercase().contains(&needle))
+        .cloned()
+        .collect()
+}
+
+pub(super) fn room_list_feedback(loaded: bool, total: usize, visible: usize) -> &'static str {
+    if !loaded {
+        "請先讀取房間列表"
+    } else if total == 0 {
+        "尚無房間"
+    } else if visible == 0 {
+        "沒有符合條件的房間"
+    } else {
+        "請選擇房間"
+    }
+}
+
+#[cfg(any(target_arch = "wasm32", test))]
+pub(super) fn refresh_result_view(
+    previous: Vec<(String, String)>,
+    previous_query: String,
+    result: Result<Vec<(String, String)>, String>,
+) -> (Vec<(String, String)>, String, String) {
+    match result {
+        Ok(next) => {
+            let status = if next.is_empty() {
+                "尚無房間"
+            } else {
+                "請選擇房間"
+            };
+            (next, String::new(), status.to_owned())
+        }
+        Err(error) => (previous, previous_query, format!("{error}，請重試")),
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+
+    #[test]
+    fn filter_preserves_ids_and_handles_all_original_queries() {
+        let rooms = vec![
+            ("room_1".into(), "Rust 技術交流".into()),
+            ("room_2".into(), "生活閒聊".into()),
+        ];
+        for query in [" 技術 ", "技術", "RUST"] {
+            assert_eq!(
+                filter_rooms(&rooms, query),
+                vec![rooms[0].clone()],
+                "{query:?}"
+            );
+        }
+        for query in ["", " "] {
+            assert_eq!(filter_rooms(&rooms, query), rooms, "{query:?}");
+        }
+        assert!(filter_rooms(&rooms, "不存在").is_empty());
+    }
+
+    #[test]
+    fn empty_success_replaces_rows_and_clears_query() {
+        let (rows, query, status) = refresh_result_view(
+            vec![("old".into(), "舊房間".into())],
+            "old".into(),
+            Ok(vec![]),
+        );
+        assert!(rows.is_empty());
+        assert_eq!(query, "");
+        assert_eq!(status, "尚無房間");
+    }
+
+    #[test]
+    fn failures_preserve_complete_snapshot_query_and_retry_message() {
+        for (old, error, previous_query) in [
+            (
+                vec![
+                    ("one".into(), "同名房間".into()),
+                    ("two".into(), "同名房間".into()),
+                ],
+                "讀取失敗：503",
+                "同名",
+            ),
+            (
+                vec![("room_1".into(), "Rust 技術交流".into())],
+                "503",
+                "Rust",
+            ),
+            (vec![("a".into(), "Rust".into())], "503", "Rust"),
+        ] {
+            let (rows, query, status) =
+                refresh_result_view(old.clone(), previous_query.into(), Err(error.into()));
+            assert_eq!(rows, old);
+            assert_eq!(query, previous_query);
+            assert_eq!(status, format!("{error}，請重試"));
+            assert!(status.contains("請重試"));
+        }
+    }
+
+    #[test]
+    fn success_replaces_links_and_returns_full_snapshot() {
+        for fresh in [
+            vec![("room_2".into(), "生活閒聊".into())],
+            vec![("a".into(), "Rust".into()), ("b".into(), "Pet".into())],
+        ] {
+            let (rows, query, status) = refresh_result_view(
+                vec![("old".into(), "old".into())],
+                "Rust".into(),
+                Ok(fresh.clone()),
+            );
+            assert_eq!(rows, fresh);
+            assert_eq!(rows[0].0, fresh[0].0);
+            assert_eq!(query, "");
+            assert_eq!(status, "請選擇房間");
+            assert_eq!(filter_rooms(&rows, &query).len(), fresh.len());
+        }
+    }
+
+    #[test]
+    fn distinguish_not_loaded_empty_filtered_and_found() {
+        assert_eq!(room_list_feedback(false, 0, 0), "請先讀取房間列表");
+        assert_eq!(room_list_feedback(true, 0, 0), "尚無房間");
+        assert_eq!(room_list_feedback(true, 3, 0), "沒有符合條件的房間");
+        assert_eq!(room_list_feedback(true, 3, 2), "請選擇房間");
+    }
+}
```

### 舊斷言如何承接

| 原測試／責任 | 新位置與保留內容 |
| --- | --- |
| search_matches_chinese_names_and_preserves_ids + day18_query_filter_preserves_room_ids | filter_preserves_ids_and_handles_all_original_queries: Chinese, trim, empty, whitespace, missing and original IDs; adds ASCII case |
| day19_empty_success_replaces_previous_rows | empty_success_replaces_rows_and_clears_query |
| day19_failure_preserves_complete_rows_and_message + day19_refresh_failure_keeps_previous_room_links + day20_refresh_failure_keeps_query_and_rows | failures_preserve_complete_snapshot_query_and_retry_message; all original snapshots/error strings/query retained |
| day19_refresh_failure_keeps_previous_room_links success branch + day20_refresh_success_reveals_full_snapshot | success_replaces_links_and_returns_full_snapshot |
| day33_room_list_feedback_tests | distinguish_not_loaded_empty_filtered_and_found unchanged 4-way contract |
| day37_search_tests | unchanged in lib.rs, real Signal caller boundary retained |

不同故障模式與跨層接線保留，不以測試數減少當覆蓋完成，也不把既有行為重構假稱新功能Red。

### 實際驗證與使用者回歸命令

完整隔離workspace的chat-ui SSR測試14項通過，含原畫面／Signal、新Day38狀態與重構模組測試；hydrate wasm32 cargo check通過，保留原有create_room未使用警告。原始來源加Red測試實際出現room_status_text缺符號編譯Red。真瀏覽器與cargo-leptos完整產物未驗收。

本案patch已在精確來源通過git apply --check及完整修改檔的Rust語法解析。也已在隔離快照實際確認本案Refactor與②功能Green patch可按兩種順序套用；不必假設Green先完成。差異相容性檢查本身不代表編譯；本次另在完整隔離合併候選執行下述Cargo與DB驗證。

```bash
# 在rust_ai_ring儲存庫根目錄檢查乾淨基準；套用由使用者執行。
git apply --check /path/to/chat-r4-refactor.patch
# 套用後在使用者環境重跑；本次相應命令已在隔離候選執行，非原產品checkout。
cd chat_apps
cargo test --locked -p chat-ui --features ssr
cargo check --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown
```

[逐檔來源blob、斷言對照及patch檢查](chat-r4-refactor-evidence.json) · [候選測試結果與範圍](r4-refactor-tests.json)。[完整本地Red／Green與workspace結果](r4-local-validation.json)。所有結果只屬隔離合併候選內容；rust_ai_ring保持唯讀，產品未套用，瀏覽器／GUI與正式CI未驗收。若失敗或patch來源漂移，保留既有程式，重核差異再套用，不把局部通過追認完整產品。

### 下一步

後續依Chat自己的認證／Session、房間／成員、訊息／WS與UI責任逐步盤點及調整；跨workspace共用契約留給全專案獨立項目。 本次有實際程式／測試變更，仍不是本案全部重構完成；依本案累積進度接續，不新增Day、不結案FIFO、不更新last_full_comparison。

## ② 功能實作：Red → 完整 Green

### Red：先讓「訊號正確但畫面錯誤」有可失敗的契約

在 `chat_apps/chat_ui/src/lib.rs` 檔尾新增測試。現有 Day37 Signal 測試是**已通過的回歸基準**，不能冒充本篇新 Red。以下先因 `room_status_text` 尚不存在而編譯失敗；真正的行為 Red 則是前篇 HTTP 500 瀏覽器實證，必須在套用前後重新操作驗證。原r2未跑；本r4已驗證原來源的缺符號編譯Red，見④。

```rust
#[cfg(test)]
mod day38_room_status_tests {
    use super::room_status_text;

    #[test]
    fn request_progress_and_failure_override_stale_room_feedback() {
        assert_eq!(room_status_text("查詢中…", true, 2, 1), "查詢中…");
        assert_eq!(room_status_text("載入中…", true, 2, 1), "載入中…");
        assert_eq!(room_status_text("伺服器：500，請重試", true, 2, 1),
                   "伺服器：500，請重試");
        assert_eq!(room_status_text("離線，請重試", false, 0, 0),
                   "離線，請重試");
    }

    #[test]
    fn success_and_local_filter_show_one_consistent_status() {
        assert_eq!(room_status_text("請選擇房間", false, 0, 0),
                   "請先讀取房間列表");
        assert_eq!(room_status_text("請選擇房間", true, 0, 0),
                   "尚無房間");
        assert_eq!(room_status_text("請選擇房間", true, 2, 0),
                   "沒有符合條件的房間");
        assert_eq!(room_status_text("請選擇房間", true, 2, 1),
                   "請選擇房間");
    }
}
```

### Green A：唯一的可見狀態決策（同檔）

在既有 `room_list_feedback` 函式**之後**加入完整函式；直接重用原本的成功／本地篩選規則，不複製另一份房間演算法。只有目前 reducer 產生的兩種進度與 `，請重試` 結尾的錯誤需要覆蓋舊快照回饋。錯誤內容仍由 API 產生並以 Leptos 文字節點 escape，不以 `inner_html` 注入。

```rust
fn room_status_text(
    status: &str,
    loaded: bool,
    total: usize,
    visible: usize,
) -> String {
    if matches!(status, "查詢中…" | "載入中…")
        || status.ends_with("，請重試")
    {
        return status.to_owned();
    }
    room_list_feedback(loaded, total, visible).to_owned()
}
```

### Green B：修改正式 RoomList 元件，而非只修改 helper

在同檔 `#[component] fn RoomList()` 的 `<div class="room-results" aria-live="polite">` 中，保留原本 `rows`、`filter_rooms`、`<ul>`、`href`、`aria-current`、按鈕與兩條非同步 caller，**只替換**原本三行：

```rust
let feedback = room_list_feedback(
    has_snapshot.get(), all.len(), visible.len()
);
```

為以下完整替換（在原有 `let visible = filter_rooms(&all, &query.get());` 後）：

```rust
let feedback = room_status_text(
    &status.get(),
    has_snapshot.get(),
    all.len(),
    visible.len(),
);
```

原本緊接的 `view! { <p class="room-result-feedback" role="status">{feedback}</p> <ul>...</ul> }` **保留不變**。這一個 `status.get()` 是關鍵的 Leptos reactive dependency：`set_status` 更新後，正式 DOM 必須重新計算；不另建第二個提示、不要把舊 `room_list_feedback` 整段刪掉。失敗時 `apply_room_search`／`refresh_result_view` 已保留舊 rows，無需重建 reducer 或 API。按鈕 `查詢房間`、`讀取／重新整理房間` 本身即重試入口；失敗提示不得宣稱有新的「重試」按鈕。

### Green 回歸檢查：測試與既有邊界

| 舊測試／入口 | 處置 | 保留的獨立斷言 |
| --- | --- | --- |
| `day37_search_tests::real_search_completion_updates_the_signals_used_by_the_view` | 保留回歸，不複製 | Signal 更新、失敗保留 rows、loading 釋放 |
| `day16_room_search_tests` | 保留 | 中文搜尋、ID、清單失敗與刷新 |
| 新 `day38_room_status_tests` | 新增兩個可失敗情境 | UI 顯示選擇規則、錯誤優先於舊快照 |
| `RoomList` 真瀏覽器 HTTP500／恢復 | 必須重新驗證，尚未執行 | DOM 可見錯誤、連結保留、按鈕重試、恢復後清除錯誤 |

本切片只有 UI presentation responsibility，Axum delivery、Application／Domain 與 SQLite Adapter **原樣重用**，不把 `chat-persistence` 改成空殼或建立無 caller 的用例。Leptos `RoomList` 是正式呼叫者；本篇測試函式不能代替真瀏覽器／hydrate 驗證。既有測試按功能保留，不因歷史 Day 名稱直接刪除。

### Cargo／imports／metadata／工作目錄

四個 workspace 根為 `chat_apps`、`pet_apps`、`analytics_apps`、`crates`，各自有 Cargo.toml/Cargo.lock，repo 根無 Cargo.toml。此切片只改 `chat_apps/chat_ui/src/lib.rs`：`chat-ui` Cargo package、`chat_ui` Rust crate；既有 `use leptos::prelude::*`、`chat-domain`、`shared`、`leptos_router` 皆保留。**Cargo.toml、Cargo.lock、normal/build/dev/target dependencies、features、Rust imports、migrations、CI YAML 均無需新增或變更**。Leptos workspace metadata 保留 `bin-package="chat-server"`、`lib-package="chat-ui"`、`name="rust_ai_chat"`（資源名稱不是舊 repo）；SSR 與 hydrate 不改。不要在 copilot_use 新增產品程式。

## ③ 當日官方穩定版本與採用判斷

2026-10-26 台北日期，crates/Rust於 `2026-10-25T23:01:56.157211+00:00` 重查；排除預發行與 yanked。[完整宣告／lock／官方網址／原始回應SHA](r2-versions.json)。來源9b與648外部宣告及resolved一致；Chat／Pet／Analytics三workspace為0.2.37，共用crates維持0.2.35。

| 套件／工具 | manifest 宣告 | lock resolved／已安裝 | 官方穩定版／MSRV |
| --- | --- | --- | --- |
| leptos | `0.8` | 0.8.22 | [0.8.22](https://crates.io/crates/leptos/0.8.22)／1.88 |
| leptos_router | `0.8` | 0.8.17 | [0.8.17](https://crates.io/crates/leptos_router/0.8.17)／1.88 |
| wasm-bindgen | `0.2` | 0.2.129 | [0.2.129](https://crates.io/crates/wasm-bindgen/0.2.129)／1.81 |
| wasm-bindgen-futures | `0.4` | 0.4.79 | [0.4.79](https://crates.io/crates/wasm-bindgen-futures/0.4.79)／1.81 |
| web-sys | `0.3` | 0.3.106 | [0.3.106](https://crates.io/crates/web-sys/0.3.106)／1.81 |
| axum | `0.8` | 0.8.9 | [0.8.9](https://crates.io/crates/axum/0.8.9)／1.80 |
| sqlx | `0.9` | 0.9.0 | [0.9.0](https://crates.io/crates/sqlx/0.9.0)／1.94.0 |
| tokio | `1.54` | 1.54.1 | [1.54.1](https://crates.io/crates/tokio/1.54.1)／1.85 |
| uuid | `1.29` | 1.29.0 | [1.29.0](https://crates.io/crates/uuid/1.29.0)／1.89.0 |
| serde | `1.0` | 1.0.229 | [1.0.229](https://crates.io/crates/serde/1.0.229)／1.56 |
| serde_json | `1.0` | 1.0.154 | [1.0.154](https://crates.io/crates/serde_json/1.0.154)／1.71 |

Rust官方stable 1.99.1，[官方channel](https://static.rust-lang.org/dist/channel-rust-stable.toml)，本機rustc/cargo同版。最高所列MSRV不高於本機Rust，但不等於SSR／WASM／native／DB建置通過。相符resolved維持；若有差異優先以最新穩定版驗證遷移，本包不改lock、安裝工具或升級。

工具：cargo-leptos已安裝0.3.11＝官方0.3.11；SQLx CLI未安裝（官方0.9.0），WASM工具須與lock一致。SQLite CLI未安裝，官方SQLite3.54.1；libsqlite3-sys lock0.37.0對官方0.39.1，bundled實際引擎不能從CLI或registry推定，升級須SQLx相容／保資料測試。本UI修補不動DB依賴。

資料庫／native當日官方來源、已安裝探測與採用理由見[版本詳細核對](../../../source-review/2026-10-26-versions-and-summary-refresh.md)，不能以metadata宣稱功能已驗收。
## ④ 驗證步驟、結果與舊 Day 更正

```bash
cd rust_ai_ring/chat_apps
cargo test --locked -p chat-ui --features ssr day38_room_status_tests
cargo test --locked -p chat-ui --features ssr
cargo check --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown
cargo leptos build --cargo-locked
# 使用隔離 SQLite、測試帳號啟動 Chat；真瀏覽器輸入搜尋，
# 模擬房間 API HTTP 500 → status 顯示錯誤、舊房間連結仍可見，
# 恢復 API → 點原有查詢／重新整理 → status 清除且房間列更新。
```

| Gate | 預期 | 本次實際 |
| --- | --- | --- |
| Red（先新增測試） | `room_status_text` 缺符號；舊 DOM HTTP500 錯誤不可見 | 本次已觀察缺符號編譯Red；真瀏覽器屬舊來源缺陷，未新跑 |
| Green unit／SSR | 新狀態與既有畫面／Signal及重構回歸 | 隔離合併候選14項通過 |
| WASM hydrate check | wasm32型別與接線 | 通過；cargo-leptos完整產物未執行 |
| 真瀏覽器 | 載入、500、舊連結、重試、成功／空結果可見 | 未執行 |
| CI／產品 Gate2 | rust_ai_ring 正式來源套用後重新跑適用 CI／瀏覽器 | 未套用／未驗收 |

**舊 Day37 更正補充：** reducer 更新 Signal 不是可見 UI；舊 r3 已明示這個缺口，本篇在下一 Day 修正，不回寫 r2／r3。Day36 WebSocket 登出、作者與歷史、房間 ACL、首頁登入管理與自動讀取仍分別追蹤。此篇只表示**文件已交付**；產品原始碼仍唯讀，`last_full_comparison` 不變。

本r4已在完整精確來源的隔離副本套用本案Refactor與功能Green，實際完成④所列驗證。原產品checkout與遠端main未套用；未把候選測試追認為瀏覽器／GUI或GitHub Actions驗收。[完整本地證據](r4-local-validation.json)保留原始錯誤與恢復，未更新last_full_comparison。
