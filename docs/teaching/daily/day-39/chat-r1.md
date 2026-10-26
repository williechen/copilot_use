# Chat Day39 r1｜首頁與房間側欄首次進入自動載入公開房間

> **r1 勘誤**：保留原 [Day39 初版](chat.md) 的 commit/blob/斷言；修正 Green 在 native SSR 非測試模式的 unused import／dead_code 警告：`should_auto_load` 及其 import 分別以 `cfg(any(wasm32,test))`／`cfg(wasm32)` 限定，功能語義不變。原 Refactor patch 仍適用。

> **Gate 1 候選方案**（2026-10-26）；來源唯讀 `williechen/rust_ai_ring/main@8fc4b277b6f6f77d97655130036fca47813de56e`，文件 `williechen/copilot_use/main`。前篇 [Day38 r4](../day-38/chat-r4.md) 已交付搜尋失敗可見回饋與 room_list_state 拆分；本篇沿已套用至最新來源的模組重新核對。方案／套用／驗收：**方案待提交／產品尚未套用本篇／未驗收**。不新增第二套產品。

## ① 目標、現況與範圍

使用者進入首頁 `/` 或房間頁 `/room/:room_id` 時，不按按鈕就應看到公開房間。現有 `chat_apps/chat_ui/src/lib.rs::RoomList` 只在「查詢房間」「讀取／重新整理房間」按鈕呼叫 `room_api::search_rooms`；初次掛載不發請求，`HomePage` 與 `RoomPage` 都重用它。Day38 已修正錯誤可見回饋；這次不重建 API、不宣稱房間 ACL 完成。

**切片完成條件**：hydrate 後每次 RoomList 掛載只發起一次初始 `/api/rooms?q=`；請求進行中不重入；手動搜尋／刷新仍有效；成功、空清單、失敗、舊快照及房間連結維持既有契約。SSR 不在伺服器執行瀏覽器 fetch。核心60分鐘僅計② Red／Green，Refactor、來源、版本、驗證與提交另計。

## Refactor｜Chat 自身登入表單契約測試遷移（不屬② Green 整理）

**累積相依**：前篇已將房間列表純函式及斷言移至 `room_list_state.rs`；本次獨立處理 **Chat UI 的 Day28/Day34 重疊測試模組**，不動 Pet、Analytics、shared 或全庫 workflow。把兩組既有 SSR 登入表單測試完整搬至 `chat_ui/src/signin_contract.rs`，原函式名稱與所有 assert 原文保留，`lib.rs` 只宣告 `#[cfg(all(test, feature="ssr"))] mod signin_contract;`；沒有把兩個情境刪成一個。這是測試目錄責任整理，不是假造新功能 Red。

**完整可套用 Refactor patch**：[chat-day39-refactor.patch](chat-day39-refactor.patch)。原 `day28_signin_tests::day28_leptos_signin_posts_to_existing_login_handler` 與 `day34_existing_signin_tests::existing_signin_is_the_single_login_form` 皆轉入同一模組，原 POST 路徑、username/password/autocomplete、唯一 form、signup link、label/id 斷言仍各自保留；未動正式 `SignInPage` 或 Cargo。先套此 patch 再套 Green，或反向，兩者僅在 lib.rs 不相交區域變更。

```bash
# rust_ai_ring 根目錄；只在使用者工作樹檢查與套用
 git apply --check /path/to/chat-day39-refactor.patch
 git apply /path/to/chat-day39-refactor.patch
 cd chat_apps
 cargo test --locked -p chat-ui --features ssr signin_contract
```

**風險／回復**：若本地已修改相同兩個測試模組，停止套用並人工三方比對；`git apply -R` 僅在乾淨且完整套用的候選分支回復。未執行 Cargo/瀏覽器，不能宣稱遷移測試已通過。下一包依 Chat Session／授權真 caller 與既有斷言逐步搬移，不跨案計功。

## ② 功能實作：Red → 完整 Green

### Red｜首次載入守衛及實際瀏覽器行為

先將下列測試加在 `chat_apps/chat_ui/src/room_list_state.rs` 的 `mod tests`：

```rust
#[test]
fn day39_initial_load_is_once_and_does_not_interrupt_pending_request() {
    assert!(should_auto_load(false, false));
    assert!(!should_auto_load(false, true));
    assert!(!should_auto_load(true, false));
    assert!(!should_auto_load(true, true));
}
```

此時 `should_auto_load` 尚未定義，屬**缺符號編譯 Red**，不是實測的失敗行為。另在**尚未套 Green 的產品瀏覽器**記錄 `GET /api/rooms?q=` 次數：初次載入預期 1，現行 0（待實測，非本次已跑 Red）。不得以舊搜尋回歸測試當新 Red。

### Green｜完整最小修改與正式 caller

[chat-day39-green-r1.patch](chat-day39-green-r1.patch) 是以精確來源生成的完整 diff，含 `room_list_state.rs` 守衛／測試、`lib.rs` 的正式 `RoomList` import 與 WASM-only `Effect::new` 接線；保留 `RoomPage`／`HomePage` 真 caller、`room_api::search_rooms`、`apply_room_search`、Leptos `hydrate` 入口與舊測試。**沒有新增依賴、feature、migration 或 lock**；`should_auto_load` import 只在 WASM 編譯，守衛函式只在 WASM 或 test 編譯，避免 native SSR release warnings。workspace metadata 仍為 `bin-package=chat-server`、`lib-package=chat-ui`，Rust imports 仍使用底線 crate 名。

```rust
/// chat_apps/chat_ui/src/room_list_state.rs
#[cfg(any(target_arch = "wasm32", test))]
pub(super) fn should_auto_load(has_snapshot: bool, loading: bool) -> bool {
    !has_snapshot && !loading
}

// chat_apps/chat_ui/src/lib.rs::RoomList，query_room closure 宣告之後、view! 之前
#[cfg(target_arch = "wasm32")]
Effect::new(move |_| {
    if should_auto_load(has_snapshot.get_untracked(), loading.get_untracked()) {
        query_room(); // existing real search_rooms("") -> GET /api/rooms?q=
    }
});
```

Effect 內只有 `get_untracked`，不因 loading/status/rows 更新重跑；每個新 RoomList 掛載各有一次初始化，SSR 不觸發 fetch。這不替代服務端房間可見性／進房 ACL；同頁手動查詢仍走既有程式。使用者套用前，這只是候選方案。

```bash
# rust_ai_ring 根目錄
 git apply --check /path/to/chat-day39-green-r1.patch
 git apply /path/to/chat-day39-green-r1.patch
 cd chat_apps
 cargo test --locked -p chat-ui --features ssr
 cargo check --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown
 cargo check --locked -p chat-server --features ssr
 # cargo leptos watch --bin-features ssr --lib-features hydrate
```

### 舊斷言承接與互動驗證

Day37 `apply_room_search` 的成功／空／錯誤 signal 斷言、Day38 `room_status_text`、`room_list_state` 的 ID／搜尋／舊快照斷言皆原封保留。以 DevTools Network 驗證首頁首次 1 次 GET、進房側欄首次 1 次 GET、手動刷新再多 1 次、快速連點 loading 不重入、500 後「請重試」仍可見並保留舊房間連結、窄螢幕／中文房名可點。hydration 是否重複、快速切頁的舊回應競態須真瀏覽器確認，不能靠純函式測試冒稱通過。

## ③ 當日官方穩定版本核對

| 依賴／工具 | 本來源宣告／Lock resolved | 2026-10-26 官方穩定 | 決策 |
| --- | --- | --- | --- |
| Rust | edition 2024；本次未讀到執行環境 rustc | 1.99.1（10/15，Rust 官方） | 不自動更新；在產品機器核對 MSRV／WASM |
| Leptos | 0.8／0.8.22 | 0.8.22（10/05；0.9 RC 非穩定） | 保持 0.8 hydrate/SSR |
| Axum | 0.8／0.8.9 | 0.8.9（04/14） | 無路由依賴變更 |
| SQLx | 0.9／0.9.0 | 0.9.0（05/21） | SQLite 無 migration |
| Tokio | 1.54／1.54.1 | 1.54.1（10/21） | 無依賴調整 |
| UUID | 1.29／1.29.0 | 1.29.0（10/23） | 保留現有 room ID 驗證 |

官方索引：[Rust](https://blog.rust-lang.org/releases/) · [Leptos](https://docs.rs/crate/leptos/latest) · [Axum](https://docs.rs/crate/axum/latest) · [SQLx](https://docs.rs/crate/sqlx/latest) · [Tokio](https://docs.rs/crate/tokio/latest) · [UUID](https://docs.rs/crate/uuid/latest)。四 workspace Cargo.lock 已讀，Chat lock blob `7629c7d06fe92ce55c33c939b189f279dd970506`。

## ④ 驗證、證據、未完成與下一步

- **已實際執行**：GitHub 精確來源檔、四 Cargo.lock、正式 caller、前篇與需求靜態讀取；官方版本頁查詢。尚未執行候選 Rust 編譯、`git apply --check`、Red/Green 測試、WASM、GUI 或產品 CI。
- **文件 Gate1**：需文章獨立 commit、精確 commit 全文讀回與 blob/SHA256、條件同步 state／manifest／七首頁／入口；本段的狀態在交付證據中更新。
- **Gate2**：使用者套用後以新產品 SHA 跑 SSR/WASM/瀏覽器/CI；未驗收，不追認前篇候選測試。
- **FIFO**：本篇只銜接上一 Day 失敗可見狀態；更早項目依原隊列保留，未完成項不假結案。下一 Day 依實際套用進度優先修復已登入首頁仍顯示「登入」及管理入口的 Session 導向，並驗證匿名進房登入回跳。
