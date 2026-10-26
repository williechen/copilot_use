# Chat Day39 r2｜首頁與房間側欄首次進入自動載入公開房間

> **2026-10-26 r2 修訂**：Refactor 正文提供逐檔程式、完整測試與斷言承接；本地驗證僅對精確8fc來源＋三案Refactor，未套本篇功能Green。產品checkout保持唯讀。原[首版](chat.md)與[r1](chat-r1.md)保留。

共用規則精確commit `0ccd1a252b3e8fc28150242944b36929760b3cd3`，blob `ba9e6cbd6763d245e60800a02c60888ba6215bd5`。

產品來源 `williechen/rust_ai_ring/main@8fc4b277b6f6f77d97655130036fca47813de56e`；只發布 `williechen/copilot_use/main` 文件。候選Refactor的實測結果列在獨立段落，②功能Green的驗證狀態另列；兩者不互相替代。

## ① 目標、現況與範圍

使用者進入首頁 `/` 或房間頁 `/room/:room_id` 時，不按按鈕就應看到公開房間。現有 `chat_apps/chat_ui/src/lib.rs::RoomList` 只在「查詢房間」「讀取／重新整理房間」按鈕呼叫 `room_api::search_rooms`；初次掛載不發請求，`HomePage` 與 `RoomPage` 都重用它。Day38 已修正錯誤可見回饋；這次不重建 API、不宣稱房間 ACL 完成。

**切片完成條件**：hydrate 後每次 RoomList 掛載只發起一次初始 `/api/rooms?q=`；請求進行中不重入；手動搜尋／刷新仍有效；成功、空清單、失敗、舊快照及房間連結維持既有契約。SSR 不在伺服器執行瀏覽器 fetch。核心60分鐘僅計② Red／Green，Refactor、來源、版本、驗證與提交另計。

## Refactor

### 1. 本次責任與修改前後

Chat 的 `lib.rs` 同時承載正式元件與多個歷史 Day 測試模組；登入表單同一契約散在 Day28 與 Day34，讀者不易辨認哪一組才是現行契約。本次把這兩組完整移入依功能命名的 `signin_contract.rs`，使登入表單測試集中維護。這一步只整理測試責任；`SignInPage` 元件仍在原處，沒有宣稱登入用例或整個 Chat 已完成分層。

Day28 測 POST／欄位／密碼 autocomplete；Day34 另外檢查只有一個 form、label/id 配對及 signup href，因此仍保留兩個測試。重疊標記目前保留以維持原斷言，不以刪除測試數冒稱去重完成。這項工作接續 Chat 自身的功能目錄整理，與②首次自動讀取房間的 Green 無相依。

| 檔案 | 本次動作 | 不可遺漏的接線 |
| --- | --- | --- |
| `chat_apps/chat_ui/src/lib.rs` | 移除兩個舊測試模組，新增一個條件模組宣告 | 保留正式 `SignInPage`、其他 SSR／Signal／列表測試 |
| `chat_apps/chat_ui/src/signin_contract.rs` | 新檔，集中兩個完整 SSR 契約測試 | `use super::*` 使用真元件與既有 Leptos imports，不複製表單 HTML |

### 2. lib.rs：模組接線與舊碼移除

在既有 `mod room_list_state;` 後加入以下宣告；`all(test, feature="ssr")` 只在 SSR 測試編譯時載入，WASM 正式產物不編譯此測試模組。

```rust
#[cfg(all(test, feature = "ssr"))]
mod signin_contract;
```

從 `lib.rs` 整塊移除 `#[cfg(all(test, feature="ssr"))] mod day28_signin_tests { ... }` 及 `mod day34_existing_signin_tests { ... }`，只移除這兩塊；內部斷言完整承接到下列新檔。不是在舊模組仍留著時再複製一份。

### 3. signin_contract.rs：完整新檔

以下是新檔全部內容，可直接與 patch 對照。兩次 SSR 都渲染正式 `SignInPage`；不是對 fixture 字串做自我驗證。

```rust
use super::*;

#[test]
fn day28_leptos_signin_posts_to_existing_login_handler() {
    let html = view! { <SignInPage /> }.to_html();
    assert!(html.contains("<form"));
    assert!(html.contains("method=\"post\""));
    assert!(html.contains("action=\"/login\""));
    assert!(html.contains("name=\"username\""));
    assert!(html.contains("name=\"password\""));
    assert!(html.contains("type=\"password\""));
    assert!(html.contains("autocomplete=\"current-password\""));
    assert!(html.contains("/signup"));
}

#[test]
fn existing_signin_is_the_single_login_form() {
    let html = view! { <SignInPage/> }.to_html();
    assert_eq!(html.matches("<form").count(), 1);
    for marker in [
        "method=\"post\"",
        "action=\"/login\"",
        "for=\"login-username\"",
        "id=\"login-username\"",
        "name=\"username\"",
        "for=\"login-password\"",
        "id=\"login-password\"",
        "name=\"password\"",
        "type=\"password\"",
        "autocomplete=\"current-password\"",
        "href=\"/signup\"",
    ] {
        assert!(html.contains(marker), "missing {marker}");
    }
}
```

### 4. 測試遷移與有效斷言

| 舊測試 | 新位置 | 承接的完整契約 |
| --- | --- | --- |
| `day28_signin_tests::day28_leptos_signin_posts_to_existing_login_handler` | `signin_contract::day28_leptos_signin_posts_to_existing_login_handler` | form、POST、`/login`、username／password 名稱、password type、current-password、signup |
| `day34_existing_signin_tests::existing_signin_is_the_single_login_form` | `signin_contract::existing_signin_is_the_single_login_form` | 恰好一個 form、POST/action、兩組 label for／input id、欄位名稱與密碼屬性、signup href |
| 房間狀態、Day37 真 Signal caller、Day38 可見回饋、帳號與版面測試 | 原位置 | 不搬移、不刪除、不以登入表單測試取代 |

兩個函式的斷言內容已按去除排版空白後逐一比對；只改檔案位置及排版，未移除斷言。`Cargo.toml`、features、lock 與正式 UI caller 均未變更。

### 5. 完整 patch、順序、驗證與回復

[完整 Refactor patch](chat-day39-refactor-r2.patch) 可單獨套在精確8fc來源；不用先套②Green。原 LF patch 無法套到來源的 CRLF `lib.rs`；本修訂保留來源換行格式，修正實際套用失敗，而不是要求使用者先整檔轉換。舊 patch 與失敗紀錄保留。

```bash
# 在自己的乾淨候選分支，從 rust_ai_ring 根目錄執行
git apply --check /path/to/chat-day39-refactor-r2.patch
git apply /path/to/chat-day39-refactor-r2.patch
cd chat_apps
cargo test --offline --locked -p chat-ui --features ssr signin_contract
cargo test --offline --locked -p chat-ui --features ssr
```

**本次實測**：隔離精確8fc來源＋Refactor；`cargo test --offline --locked -p chat-ui --features ssr` exit 0，14項通過、0項失敗、0項ignored。[原始log](r2-validation/chat-ssr.log)；此結果不包含未套用的Day39功能Green。

兩個 patch 的前後順序皆已用 `git apply --check` 實際確認；這項套用檢查不等於②功能測試通過。只要本地改過相同區域就先三方比對，不覆寫。回復時在未混入後續修改的候選分支先 `git apply -R --check`，再 `git apply -R`。下一步沿登入／Session 與房間權限的真 caller 整理責任；本次沒有抽出空 Application 或移除不同故障模式測試。

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

[chat-day39-green-r2.patch](chat-day39-green-r2.patch) 是以精確來源生成的完整 diff，含 `room_list_state.rs` 守衛／測試、`lib.rs` 的正式 `RoomList` import 與 WASM-only `Effect::new` 接線；保留 `RoomPage`／`HomePage` 真 caller、`room_api::search_rooms`、`apply_room_search`、Leptos `hydrate` 入口與舊測試。**沒有新增依賴、feature、migration 或 lock**；`should_auto_load` import 只在 WASM 編譯，守衛函式只在 WASM 或 test 編譯，避免 native SSR release warnings。workspace metadata 仍為 `bin-package=chat-server`、`lib-package=chat-ui`，Rust imports 仍使用底線 crate 名。

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
 git apply --check /path/to/chat-day39-green-r2.patch
 git apply /path/to/chat-day39-green-r2.patch
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

## ④ 驗證、證據與未完成

本r2新增的實測限於獨立Refactor：完整來源8fc的隔離副本套入三案Refactor，沒有套②功能Green。各案精確命令、通過／ignored數與原始log見本案Refactor段落及[驗證紀錄](r2-refactor-validation.json)。產品checkout與遠端main均未套用。

三案Refactor各自可直接套在原始8fc；也已實際檢查與各自Green前後兩種套用順序。原Chat Refactor及Chat／Pet Green存在換行格式不符的git apply失敗，本r2只修正Green patch換行格式，功能設計及斷言不變；原始失敗與修正後檢查都保留在[套用紀錄](r2-composition-checks.json)及[修正前紀錄](r2-composition-checks-before-eol-fix.json)。這些套用檢查不是Green功能測試。

**未執行**：②新功能Red／Green的Rust／WASM／Node／原生／瀏覽器測試與候選GitHub Actions；Pet Tauri server編譯及真生活窗事件亦未驗。Chat初次自動讀取、Pet找回視窗、Analytics CSV下載／試算表匯入各自依②的真入口驗收步驟執行，不能由Refactor回歸測試替代。

本次版本表保留同日r1的官方查核與來源lock紀錄，沒有改依賴或聲稱重新升級。Gate1依[本案精確commit全文讀回與blob／SHA256](chat-r2-readback.json)記錄；Gate2仍是使用者套用後，以新產品SHA跑功能／DB／GUI與CI。獨立段落提供的是可套用重構程式與證據，不是產品main已重構或已驗收。

既有原文、FIFO與有效斷言保留；Refactor不新增Day，next_day維持40，last_full_comparison不變。各案後續責任見本案Refactor最後一段；全專案G-01／G-02另列[工程項目](../../../engineering/README.md)，不以本案測試通過抵算。
