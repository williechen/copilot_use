# Pet Day43 候選｜控制台逐隻顯示回籠狀態，不再覆蓋全域訊息

> 2026-10-29；唯讀產品 [rust_ai_ring@c6708c4](https://github.com/williechen/rust_ai_ring/tree/c6708c42f21ac919e57a379848b4977d27a1c3d6)；文件起點 `copilot_use/main@d6a701e`；`next_day.pet=42`。**新 Day43 候選草稿；未發布 Gate1，未套用產品。**

## ① 目標／前置與正式呼叫鏈

Day42 已由 `pet_ui::main::App` 每列「查詢回籠狀態」呼叫 `bridge::pet_cage_status_by_id` → `index.html::petCageStatusById` → Tauri `pet_server::pet_cage_status_by_id` → `PetStore::read_caged` → SQLite。但結果寫到共用 `new_pet_feedback`，查詢 B 會覆蓋 A 的可見訊息，不能同時比較兩隻。新增以 UUID 為 key 的控制台顯示狀態，成功／錯誤只更新該列；重新載入清單時清空舊快照，選取／出籠／回籠成功時清除該隻過期查詢結果。**不更改** DB 回籠旗標、預設出籠政策、Tauri command、native window 或漫遊。

## Refactor｜Pet 原生選單 UUID 解析歸屬（獨立）

修改前 `pet_server/src/lib.rs` 同時包含 Tauri setup、事件處理與純 `control_menu_pet_id` 字串解析；修改後把解析移到 `pet_server/src/menu_scope.rs`，在 lib.rs 宣告 `mod menu_scope;`、`use crate::menu_scope::control_menu_pet_id;`，保持正式 `app.on_menu_event` caller、原 `open_control_menu_is_scoped_to_one_valid_pet` 斷言及錯誤路徑不變。新增不同 UUID、非法 UUID、錯 action 的 fail-closed 測試；不觸及跨 workspace、共用 crates、全庫 CI。

```rust
pub(crate) fn control_menu_pet_id(menu_id: &str) -> Option<String> {
    let id = menu_id.strip_prefix("pet:")?.strip_suffix(":open_control")?;
    uuid::Uuid::parse_str(id).ok().map(|id| id.to_string())
}
```

完整新增 module、caller 和測試見 [Pet Refactor patch](pet-day43-refactor.patch)。下一步盤點 native menu event 與 Runtime 逐 ID 鎖的錯誤注入；未執行 native 測試不視為重構驗收。

## ② Red → 完整 Green（核心預估 15 分鐘）

**Red：** 在 `pet_ui/src/cage_status.rs` 新增 `two_pets_keep_independent_status_and_failure_is_not_false`，依序寫入 A=出籠、B=回籠、A=SQLite 失敗；要求 B 仍為回籠且 A 顯示錯誤，不得把失敗轉 false。原程式沒有逐 UUID map 與顯示單元，是缺符號／可見行為 Red；**未實際執行**。Day41/42 Node bridge、PetStore SQLite、native lifecycle 舊斷言全部保留。

**Green：** `pet_ui/src/cage_status.rs` 的完整純函式：

```rust
pub(crate) fn record_cage_status(
    statuses: &mut std::collections::HashMap<String, String>,
    id: &str,
    result: Result<bool, String>,
) {
    let text = match result {
        Ok(true) => "已回籠".to_owned(),
        Ok(false) => "已出籠".to_owned(),
        Err(error) => format!("查詢回籠狀態失敗：{error}"),
    };
    statuses.insert(id.to_owned(), text);
}
```

`pet_ui/src/main.rs` 宣告 `mod cage_status;`、import `HashMap`／`record_cage_status`，在 `App` 新增 `let cage_status = RwSignal::new(HashMap::<String,String>::new());`。原逐列查詢 closure 改為：

```rust
let result = bridge::pet_cage_status_by_id(&id).await;
cage_status.update(|statuses| record_cage_status(statuses, &id, result));
```

在同一個 `<li>` 的查詢按鈕後顯示 `<p role="status" aria-live="polite">{move || cage_status.get().get(&display_id).cloned().unwrap_or_default()}</p>`。重新載入清單清空 map；出籠、回籠、選取成功只移除該 UUID 的舊狀態。完整 [Pet Green patch](pet-day43-green.patch) 包含所有 UI 接線、純函式及測試。沒有新 dependency、Cargo feature、migration 或 JS bridge。原 `pets_busy` 排他保留；SQLite 錯誤明示，不冒稱 native 成功。

## ③ 版本（2026-10-29）

Rust stable [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)、Tauri [2.12.5](https://docs.rs/crate/tauri/latest)、Leptos [0.8.22](https://docs.rs/crate/leptos/latest)、SQLx [0.9.0](https://docs.rs/crate/sqlx/latest)、Tokio [1.54.1](https://docs.rs/crate/tokio/latest)、UUID [1.29.0](https://docs.rs/crate/uuid/latest)；Tauri CLI 官方文件目前可見 [2.12.0](https://docs.rs/tauri-cli/latest/tauri_cli/)，本機 CLI 與 native WebKitGTK/OS 套件未核對。產品四 workspace `0.2.42`，Pet lock resolved 依 [Day42 最新版本表](../../daily/day-42/versions-2026-10-29.md)。本輪無完整 MSRV/native/WASM 驗證，不升級產品。

## ④ 驗證、套用、驗收與下一步

已做：精確 GitHub 來源、正式 Tauri caller／Leptos 清單、既有 Node bridge、原斷言讀回；生成逐檔 patch 與來源片段唯一比對。**未做**：真 `git apply --check`、Red、Cargo、Node、SQLite、Tauri GUI／WebKit、CI；Gate1／Gate2 均未通過。隔離 checkout `c6708c42f21ac919e57a379848b4977d27a1c3d6` 後先 Refactor 再 Green：

```bash
git apply --check pet-day43-refactor.patch && git apply pet-day43-refactor.patch
git apply --check pet-day43-green.patch && git apply pet-day43-green.patch
cargo fmt --all --manifest-path pet_apps/Cargo.toml -- --check
cargo test --locked --manifest-path pet_apps/Cargo.toml -p pet-server
cargo test --locked --manifest-path pet_apps/Cargo.toml -p pet-ui
cargo check --locked --manifest-path pet_apps/Cargo.toml -p pet-ui --target wasm32-unknown-unknown
node --test pet_apps/pet_ui/tests/*.test.mjs
```

人工驗收：A/B 出籠→分別查狀態→B 回籠→A 訊息不受影響→重新查 B→SQLite 故障只顯示 B 錯誤→重載清單清除舊顯示；核對原生視窗、重啟與預設資格仍為既有待辦。核心 15 分鐘只計 Red/Green；Refactor 與其餘另計。候選方案／產品套用／驗收分列；不推進 Day、FIFO、`last_full_comparison`。
