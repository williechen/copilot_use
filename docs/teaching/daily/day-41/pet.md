# Pet Day41｜控制台逐隻回籠，其他生活窗不受影響

> 2026-10-27；產品唯讀 SHA `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`，文件預期 parent `a27c17eadb49e2bd15e70c12e208699979af03f6`。依 Pet next_day=41；[Day40 r2](../day-40/pet-r2.md) 已處理生活窗 Native X 持久回籠，本篇不重教。**方案未套用／未驗收。**

## ① 功能目標／現況／前置

控制台已可對每隻寵物「出籠／找回」，但沒有「回籠這隻」按鈕；原生活窗的「回籠子」只針對自身。今日在**控制台清單**加逐 UUID 回籠操作，呼叫既有 `pet_windows::cage_saved_pet`（生命週期鎖 → PetStore/SQLite `set_caged` → 只銷毀對應 Tauri WebviewWindow → Runtime/漫遊清理），不重建另一套 cage handler；若回籠的是目前選取寵物，控制台清除過時選取／快照，其他生活窗維持不動。失敗時保留狀態並顯示錯誤，不能假稱成功。預設出籠資格與回籠狀態分離仍是後續功能。

真呼叫鏈：`pet_ui::App` 控制台 `For` 每列 → `bridge::cage_saved_pet_by_id` → `index.html::cageSavedPetById` → Tauri `invoke("cage_saved_pet", {id})` → 已註冊 `pet_windows::cage_saved_pet` → `pet_application::lifecycle::persist_then_close` → `pet_persistence::set_caged`（SQLite）→ 目標 Window destroy／Runtime remove。無新增未使用 Port 或空 library。

## Refactor｜Pet 自身 UI 快照次序與 fixture 責任（獨立於回籠功能）

原 `pet_apps/pet_ui/src/component/mod.rs` 混合 module exports 與 `apply_snapshot` 的 revision 過時事件決策；控制台與生活窗兩種事件／IPC 都依賴它，若異步回應晚到可能覆寫新狀態。**完整移動**原函式至新 `pet_apps/pet_ui/src/component/snapshot.rs`，`mod.rs` 只保留 `pub mod activity/chat/control; mod snapshot; pub use activity::*; pub use chat::*; pub use control::*; pub use snapshot::apply_snapshot;`；原 `crate::component::apply_snapshot` caller 不改。

```rust
use leptos::prelude::*;
use pet_domain::PetSnapshot;

fn accepts_revision(current: Option<u64>, incoming: u64) -> bool {
    current.map(|revision| incoming >= revision).unwrap_or(true)
}

pub fn apply_snapshot(
    incoming: PetSnapshot,
    snapshot: ReadSignal<Option<PetSnapshot>>,
    set_snapshot: WriteSignal<Option<PetSnapshot>>,
) {
    if accepts_revision(snapshot.get_untracked().map(|s| s.revision), incoming.revision) {
        set_snapshot.set(Some(incoming));
    }
}
```

**斷言承接**：新增同檔 `snapshot_order_preserves_existing_ipc_and_event_assertions`，明確檢查無快照、相同 revision、較新接受、較舊拒絕；既有 `PetMachine` 狀態、Day24 三隻隔離、Day39 找回、Day40 Native 關窗、Node bridge 原測試全部保留。此 Refactor 不變更 DB／IPC 協定、其他專案、共用 crates 或全庫 workflow。[完整 Refactor patch](pet-day41-refactor.patch) 含搬移前後與測試全文。下一步再核對 Pet UI 的 stale callback／owner lifecycle 與 native failure fixture。

## ② Red → 完整最小 Green

### Red｜先用正式 bridge 失敗斷言

新增 `pet_apps/pet_ui/tests/day41-cage-by-id.test.mjs`，沿 Day33 真 `index.html` script 的 Node `vm` fixture 執行，斷言 `window.cageSavedPetById("pet-b")` **只**呼叫 `invoke("cage_saved_pet",{id:"pet-b"})`，不先切換選取或碰其他 UUID；native reject 必須原樣傳回。舊 script 缺函式，這是明確的缺新符號 Red。真 SQLite `pet_store::day35_cage_caller_tests::live_store_cages_only_one_saved_pet` 原斷言保留，並非本篇新增的 Red；不得把 Node 測試冒充 native Tauri GUI。Red 尚未實際執行。

### Green｜完整檔案、呼叫端、測試

[完整 Green patch](pet-day41-green.patch) 修改 **3 個既有檔**並新增 **1 個 Node 測試檔**，全部源於精確產品 SHA：

1. `pet_apps/pet_ui/index.html`：在既有 `window.cageSavedPet`（生活窗由 `currentPetId()` 取自身）旁新增 `window.cageSavedPetById = (id) => invoke("cage_saved_pet", { id });`。不動舊橋接函式，不建立新 Tauri command。
2. `pet_apps/pet_ui/src/bridge.rs`：原 `#[wasm_bindgen] extern "C"` 增加 `#[wasm_bindgen(js_namespace=window, js_name=cageSavedPetById)] fn cage_saved_pet_by_id_js(id: &str) -> Promise;`，並加下列完整 Rust wrapper，錯誤沿既有 `js_error` 傳遞：

```rust
pub async fn cage_saved_pet_by_id(id: &str) -> Result<(), String> {
    JsFuture::from(cage_saved_pet_by_id_js(id))
        .await
        .map_err(js_error)?;
    Ok(())
}
```

3. `pet_apps/pet_ui/src/main.rs`：控制台 `<For each=saved_pets ...>` 每列另 clone `cage_id`；在「出籠」與「找回寵物」之間加「回籠這隻」按鈕，與既有 `pets_busy` 排他。成功後僅當 `active_pet.id == id` 時清除 `active_pet` 與 `snapshot`，顯示「該隻已回籠；其他寵物保持原狀」；錯誤顯示「回籠失敗」且釋放 busy。完整按鈕閉包及所有保留 caller 均在 patch。
4. `pet_apps/pet_ui/tests/day41-cage-by-id.test.mjs`：新增兩個正式 script 行為測試（精確 ID、不更改選取；native failure 傳遞），舊 Day33 Node 測試保留。

Cargo/feature/Leptos metadata：沿 `pet-ui` CSR + Tauri `pet-server`、`pet-application`、`pet-persistence` 現有 package；沒有新依賴、feature、migration、lock 或 `generate_handler!` 修改。**此切片不改 startup default，避免把尚未實作政策冒稱完成。**

## ③ 版本核對（2026-10-27）

Rust stable **1.99.0**（2026-10-01，https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/）；Tauri stable **2.12.5**（2026-10-21，https://docs.rs/crate/tauri/latest，3.0.0-alpha 排除）；Leptos **0.8.22**（https://docs.rs/crate/leptos/latest）；SQLx **0.9.0**（https://docs.rs/crate/sqlx/latest）；Tokio **1.54.1**（https://docs.rs/crate/tokio/1.54.1）；UUID **1.29.0**（https://docs.rs/crate/uuid/latest）。manifest `tauri = "2.12"`、`leptos = "0.8"`、`sqlx = "0.9"`；精確 Pet Cargo.lock resolved `tauri 2.12.5`、`tauri-build 2.7.4`、`leptos 0.8.22`、`sqlx 0.9.0`、`tokio 1.54.1`、`uuid 1.29.0`。未核對本機 native 系統庫／WASM 版本與全部 MSRV，不升級產品。

## ④ 實際驗證／未完成／下一步

**已執行**：精確 SHA 來源四檔讀取、修改 preimage 文字匹配、完整 unified patch 生成；**未執行** git apply、Node、Cargo、SQLite、native GUI、CI。

```bash
# cwd: rust_ai_ring 根；隔離副本 checkout 9c767978b20bd56e1c2182dac2f37e6ae9b7d988
git apply --check /path/to/pet-day41-refactor.patch
git apply /path/to/pet-day41-refactor.patch
git apply --check /path/to/pet-day41-green.patch
git apply /path/to/pet-day41-green.patch
node --test pet_apps/pet_ui/tests/*.test.mjs
cd pet_apps
cargo fmt --all -- --check
cargo test --locked -p pet-ui
cargo test --locked -p pet-application -p pet-persistence
cargo test --locked -p pet-server
cargo check --locked -p pet-server
cargo check --locked --target wasm32-unknown-unknown -p pet-ui
```

原生驗收：兩隻出籠，控制台按 B 回籠，確認 B SQLite caged=1／B 視窗消失／B roam 停止，A 視窗與狀態不變；B 重複操作、DB 寫入失敗／destroy 失敗、重啟及手動開窗。Native 故障注入、startup-default 政策、三窗 GUI 尚未驗收。**方案／使用者套用／Gate2 分列**。核心 Red/Green 15 分鐘，Refactor 與其餘另計；不增加重構 Day、不動 FIFO/`last_full_comparison`。回復逆 Green 再逆 Refactor。
