# Pet Day42｜控制台逐隻查詢 SQLite 回籠狀態

> 2026-10-28；產品唯讀來源 `williechen/rust_ai_ring@0b2288e4996b6d80b00e7a77e2cd023835306528`；`next_day.pet=42`。Day41 已見控制台逐隻回籠與 Native cage caller，本篇不重建回籠命令。**方案已提交，未套用／未編譯／未完成原生 GUI 驗收。**

## ① 目標、現況、前置

目前控制台可列出寵物、逐隻出籠、回籠及找回，但無法直接查詢 SQLite 目前儲存的回籠旗標，使用者無法辨識 DB 讀回狀態與單純視窗顯示。新增每列「查詢回籠狀態」按鈕，將指定 UUID 傳入既有 `PetStore::read_caged`，成功顯示「已回籠／已出籠」，資料庫錯誤必須顯示失敗，不得當成 false。

正式鏈：`pet_ui::App` → `bridge::pet_cage_status_by_id` → `index.html::petCageStatusById` → `Tauri invoke("pet_cage_status_by_id", {id})` → `pet_server::pet_cage_status_by_id` → `PetStore::read_caged` → `pet_persistence::read_caged` → SQLite。這是控制台的讀取功能，不會改 DB、選取、視窗、漫遊或預設資格。下一個功能需另將「啟動預設出籠」與「目前回籠」兩個持久化欄位分開，**本篇不冒稱已解決啟動預設政策**。

## Refactor｜Pet 自身漫遊啟用政策歸屬

修改前 `pet_server/src/lib.rs` 的 `roam_enabled_value` 決定是否允許漫遊，`roam.rs` 負責漫遊任務、視窗位置、停止及清理。修改後把純政策函式移到 `pet_server/src/roam.rs`，在 `lib.rs` 透過 `use roam::roam_enabled_value;` 讓既有 `roam_enabled` 和原測試仍使用同一實作。保留環境變數 `PET_ENABLE_ROAM` 的語意；只有精確 `true` 或 `1` 開啟，缺值、`TRUE`、`false` 與空值都維持關閉。

```rust
pub(crate) fn roam_enabled_value(value: Option<&str>) -> bool {
    matches!(value, Some("true" | "1"))
}
```

新增 `day42_roam_policy_refactor_tests` 覆蓋 7 個輸入；原 `roam_setting_tests`、逐 UUID task、停止、視窗邊界、失敗保護及其他 Pet 測試不刪。完整修改單元、原 caller 與新增斷言在 [Pet Refactor patch](pet-day42-refactor.patch)。這是 Pet 專案自身責任整理，未改 `crates`、Chat、Analytics 或全庫 workflow，不計入 G-01/G-02。下一步再核對 native failure fixture、逐寵物漫遊持久化與任務關閉競態。

## ② Red → 完整 Green（核心預估 15 分鐘）

### Red：真實 JS bridge 的 ID 與失敗傳遞

新增 `pet_apps/pet_ui/tests/day42-cage-status.test.mjs`，讀取**真實** `index.html` script，使用 Node `vm` 模擬 Tauri invoke，驗證兩隻不同 ID 的回籠狀態分別為 false／true，且呼叫名稱精確 `pet_cage_status_by_id`、參數只含指定 `id`；native reject 時 Promise 必須 reject。舊來源不存在 `window.petCageStatusById`，屬缺符號 Red；**未實際執行 Red**。既有 Day41 `day41-cage-by-id.test.mjs`、Day35/37 SQLite 斷言不移除。

```javascript
test("each status query reaches the correct saved UUID and native result", async () => {
  const { window, calls } = boot();
  assert.equal(await window.petCageStatusById("pet-a"), false);
  assert.equal(await window.petCageStatusById("pet-b"), true);
  assert.deepEqual(calls, [
    ["pet_cage_status_by_id", { id: "pet-a" }],
    ["pet_cage_status_by_id", { id: "pet-b" }]
  ]);
});
```

### Green：完整正式 caller 與 UI

1. `pet_apps/pet_server/src/lib.rs`：新增 Tauri command，使用既有 `PetStore` 讀取資料，並在原 `generate_handler!` 登記；不建平行 SQL。
2. `pet_apps/pet_ui/index.html`：新增 `window.petCageStatusById = (id) => invoke("pet_cage_status_by_id", { id });`；既有出籠、回籠與選取橋接保留。
3. `pet_apps/pet_ui/src/bridge.rs`：增加 `#[wasm_bindgen(js_namespace=window, js_name=petCageStatusById)]` extern Promise 與 wrapper；`JsValue` 不是 boolean 時拒絕，不能把格式錯誤解讀為回籠。
4. `pet_apps/pet_ui/src/main.rs`：在原 `<For each=saved_pets>` 每列加入「查詢回籠狀態」，與 `pets_busy` 排他；成功／錯誤透過既有 `new_pet_feedback` 呈現，finally 路徑恢復 busy，其他寵物不變。
5. 新增真實 Node bridge 測試 `day42-cage-status.test.mjs`，兩個測試涵蓋 ID 隔離及 DB failure 傳遞。

```rust
#[tauri::command]
async fn pet_cage_status_by_id(
    id: String,
    store: State<'_, pet_store::PetStore>,
) -> Result<bool, String> {
    store.read_caged(&id).await
}
```

```rust
pub async fn pet_cage_status_by_id(id: &str) -> Result<bool, String> {
    let value = JsFuture::from(pet_cage_status_by_id_js(id)).await.map_err(js_error)?;
    value.as_bool().ok_or_else(|| "回籠狀態回應格式錯誤".into())
}
```

[完整 Pet Green patch](pet-day42-green.patch) 含上述所有逐檔新增／修改與完整 Node 測試。先套 Refactor 後套 Green，回復逆序。無 Cargo package／feature、lock、migration 變更；不觸碰 `PetStore::set_caged`、`select_pet` 或 startup restore。

## ③ 當日版本

2026-10-28 Rust 官方 stable **1.99.1**（2026-10-15，[官方 release notes](https://doc.rust-lang.org/releases.html)）。workspace manifest 宣告 `tauri` 由 `pet-server` 直接指定、共用 `leptos 0.8`、`sqlx 0.9`、`tokio 1.54`、`uuid 1.29`；官方查核入口 [Tauri](https://docs.rs/crate/tauri/latest)、[Leptos](https://docs.rs/crate/leptos/latest)、[SQLx](https://docs.rs/crate/sqlx/latest)。前篇 lock 曾記錄 Tauri 2.12.5；本輪未完成最新 `Cargo.lock` resolved、Tauri CLI／Node／native 系統套件與 MSRV/WASM 相容核對，**版本 Gate 尚未全部通過**，未升級產品。

## ④ 驗證、限制、下一步

**已執行**：最新產品來源五檔精確讀取、目標修改片段唯一比對、unified diff 生成、兩份 patch 精確 GitHub 提交後讀回。**未執行**：`git apply --check`、Node、Cargo、SQLite、Tauri native、GUI、CI；不把 JS mock 當真原生驗收。

```bash
# 隔離副本 checkout 0b2288e4996b6d80b00e7a77e2cd023835306528
git apply --check pet-day42-refactor.patch
git apply pet-day42-refactor.patch
git apply --check pet-day42-green.patch
git apply pet-day42-green.patch
node --test pet_apps/pet_ui/tests/*.test.mjs
cargo fmt --all --manifest-path pet_apps/Cargo.toml -- --check
cargo test --locked --manifest-path pet_apps/Cargo.toml -p pet-server
cargo test --locked --manifest-path pet_apps/Cargo.toml -p pet-persistence
cargo check --locked --manifest-path pet_apps/Cargo.toml -p pet-ui --target wasm32-unknown-unknown
```

原生手動驗收：A/B 兩隻出籠→回籠 B→控制台分別查 A、B，SQLite 實際 `is_default_closure` 與顯示一致；再測 DB 關閉、缺 ID、重啟、重複查詢及其他視窗不受影響。**預設出籠資格與回籠旗標仍混用**，必須後續獨立 migration／用例／UI／重啟測試解決；本篇只提供讀回入口。核心 Red/Green 15 分鐘，其餘另計；不增加 Refactor Day、不動 FIFO／`last_full_comparison`。
