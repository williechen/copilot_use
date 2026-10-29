# Pet Day32 r1｜從哪隻寵物右鍵開控制窗，就選取那隻

## ① 學習大綱：功能解說與精確來源

今日小切片：桌面上同時有「小橘」「阿龜」兩隻生活窗，先在控制台選小橘，再從阿龜的生活窗右鍵選「開啟控制窗」。**預期恢復同一個控制窗，且控制台選取阿龜**；不建立第二個控制窗、不把其他寵物回籠、不重設 Runtime 狀態。這是使用者已確認的多寵物操作流程，而不是只驗證選單字串。

規則正本 `williechen/copilot_use/main`；唯一程式來源唯讀 `williechen/rust_ai_chat/main@be0494a2a70be91eb74b2b726970a23fc3e5c044`；接續 `next_day.pet=32`，版本 `daily-tdd-minimum-v1 + github-only-v1`。Day31 已在 `apps/desktop_pet/src/lib.rs` 攔截主窗 CloseRequested，隱藏 `main` 而不銷毀；`open_control_window` 能 show/focus 原主窗。但 `frontend/index.html` 的原生選單使用固定 id `open_control`，`app.on_menu_event` 收到後只呼叫 `open_control_window`，**沒有**傳送是哪隻寵物；所以即使重開控制窗，選取仍停在前一隻。

真實鏈：生活窗 `index.html?view=pet&pet_id=<uuid>` → `currentPetId()` → Tauri JS `Menu.new` → Rust `app.on_menu_event` → `PetStore::select`（SQLite `local_pet_selection` slot=1 + `pet_profile`）→ `pet-profile-changed` → Leptos CSR `listen_pet_profile` → `open_control_window` 顯示／聚焦原 `main`。本篇不新建寵物、資料表、Tauri command 或第二套控制台；`pet_windows::Runtime` 不受選取事件重建。使用者在自己的 rust_ai_chat 工作副本套用，**不**向 copilot_use 寫產品程式。

## ② Red → 完整可套用 Green → Refactor

### Red A：測試選單必須攜帶 UUID

在 `apps/desktop_pet/src/lib.rs` 末尾加以下單元測試。Green 前 `control_menu_pet_id` 不存在，預期是 **missing function 編譯失敗**；這是先定契約的 Red，不是本次實測。測試要求拒絕其他選單 id、格式錯誤、跨指令後綴，防止隨便把原生事件當成合法寵物。

```rust
#[cfg(test)]
mod day32_control_selection_tests {
    use super::control_menu_pet_id;

    #[test]
    fn open_control_menu_is_scoped_to_one_valid_pet() {
        let id = "123e4567-e89b-42d3-a456-426614174000";
        assert_eq!(
            control_menu_pet_id(&format!("pet:{id}:open_control")),
            Some(id.to_string())
        );
        for invalid in [
            "open_control",
            "pet:other:open_control",
            "pet:123e4567-e89b-42d3-a456-426614174000:sleep",
            "pet:123e4567-e89b-42d3-a456-426614174000:open_control:extra",
        ] {
            assert_eq!(control_menu_pet_id(invalid), None);
        }
    }
}
```

### Red B：真實原生視窗行為

隔離 SQLite 建立小橘、阿龜兩隻並同時出籠；控制台選小橘→按 X 隱藏控制台→在阿龜生活窗右鍵開控制台。現行固定 `open_control` 不含阿龜 id，預期控制台雖恢復卻仍選小橘。**本次未執行 GUI**，是由來源推得的預期失敗；不能用 mock window、單元測試通過或 CI build success 取代原生操作。

### Green A：在 JS 選單 id 保留生活窗寵物身分

在 `apps/desktop_pet/frontend/index.html` 的 `getPetContextMenu()` 中，**只替換**原本這一行：

```javascript
{ id: "open_control", text: "開啟控制窗" },
```

為下列完整項目（原 `const id = await currentPetId();` 已存在；其餘互動／睡／醒、`menu.popup` 不動）：

```javascript
{ id: `pet:${id}:open_control`, text: "開啟控制窗" },
```

如此每隻生活窗建立的 native menu 都持有自己的 UUID，不依賴控制台目前的 slot。既有 `window.openPetControls()` 仍可由其他按鈕直接聚焦控制台，不影響這個右鍵流程。

### Green B：Rust 驗證 id、選取 SQLite、通知 Leptos、再聚焦

在 `apps/desktop_pet/src/lib.rs`、既有 `open_control_window` 之前，新增下列完整函式；直接使用原專案已存在的 `uuid` 依賴：

```rust
fn control_menu_pet_id(menu_id: &str) -> Option<String> {
    let id = menu_id
        .strip_prefix("pet:")?
        .strip_suffix(":open_control")?;
    uuid::Uuid::parse_str(id).ok().map(|id| id.to_string())
}
```

再於 `application` 的 `app.on_menu_event(|app_handle, event| { ... })` closure **最前面**插入以下完整分支，放在舊的 `if event.id().0.as_str() == "open_control"` 之前。其他 `pet:{id}:sleep/wake/interact`、generic menu、`invoke_handler`、`.setup` 及 `Runtime` 全部保留：

```rust
                // Day32：由生活窗原生選單傳入該隻寵物的 UUID。
                if let Some(id) = control_menu_pet_id(event.id().0.as_str()) {
                    let handle = app_handle.clone();
                    let store = app_handle
                        .state::<pet_store::PetStore>()
                        .inner()
                        .clone();
                    tauri::async_runtime::spawn(async move {
                        match store.select(&id).await {
                            Ok(saved) => {
                                // 控制窗原有 Leptos 監聽此事件，選取成功才通知。
                                if let Err(error) = handle.emit("pet-profile-changed", &saved) {
                                    eprintln!("Day32 選取通知失敗：{error}");
                                    return;
                                }
                                if let Err(error) = open_control_window(handle) {
                                    eprintln!("Day32 無法開啟控制窗：{error}");
                                }
                            }
                            Err(error) => eprintln!("Day32 選取寵物失敗：{error}"),
                        }
                    });
                    return;
                }
```

既有 `PetStore::select(&id)` 已在 SQLite transaction 中查 owner、更新 slot=1、解除該隻的回籠旗標並 commit；本篇**重用**它而不重寫 SQL。此右鍵只會出現在已出籠生活窗，故不需再呼叫 `pet_windows::open`；若 DB 寫入失敗，分支不送事件、不顯示「已選取成功」，保留原控制台及其原狀。舊固定 `open_control` branch 仍作舊入口相容，但新選單不再走它。

### Green C：測試持久化選取（沿用既有 migration）

在 `apps/desktop_pet/src/pet_store.rs` 的既有 `#[cfg(test)] mod day22_tests` **內**追加此整合測試；不新建測試 DB schema，不動 `Cargo.toml`。它驗證 Green B 所重用的 adapter，不能代替 native menu 的驗收：

```rust
    #[sqlx::test(migrations = "./migrations")]
    async fn day32_selecting_other_pet_changes_only_active_slot(pool: sqlx::SqlitePool) {
        let store = PetStore { pool: pool.clone() };
        let orange = store.create("小橘").await.unwrap();
        let turtle = store.create("阿龜").await.unwrap();
        store.select(&orange.id).await.unwrap();
        assert_eq!(store.active().await.unwrap(), Some(orange.clone()));
        store.select(&turtle.id).await.unwrap();
        assert_eq!(store.active().await.unwrap(), Some(turtle.clone()));
        assert_eq!(store.list().await.unwrap().len(), 2);
        let selected: String =
            sqlx::query_scalar("SELECT pet_id FROM local_pet_selection WHERE slot=1")
                .fetch_one(&pool).await.unwrap();
        assert_eq!(selected, turtle.id);
    }
```

### Refactor：一個身分來源、一個控制窗、一個持久化選取

選單 id 由生活窗的 URL UUID 生成，Rust 端重新 parse UUID，再交由 `PetStore::select` 核對 owner；不信任 JS 自行聲稱的控制台選取。先 commit DB 再發 `pet-profile-changed`，避免前端先顯示未提交的選取；`open_control_window` 只 show/focus `main`，不重新 mount Leptos、不移動其他寵物。若 `emit` 或 `show` 失敗，已提交的選取仍會持久保存，UI 可能尚未更新：**這是需明列的補償／重試缺口**，不可宣稱跨 SQLite／GUI 的原子交易。長期應讓控制台顯示時重讀 active slot 並提供錯誤回饋。

### 上一 Day 未完成事項補充

Day31 的主窗 hide/show 程式在來源已可核對，真實原生 X→隱藏→重開、三隻生活窗獨立、草稿保留與跨平台差異仍需精確 SHA GUI 驗收；本日不重教 Day31，也不把 CI success 冒稱 GUI 成功。

### 舊 Day 更正補充（FIFO）

最早未結案 Pet `pet-day25-single-image-20261022`：需確認單圖動畫（不是單元件）及三隻不同 pet-id 的真實原生視窗、影格、裁切、獨立事件。此補教保留 **pending**；本日不改 Day25 歷史正文、不以本篇選取功能替代單圖／三窗驗收，也不把舊項目轉為 copilot_use 程式工作。

## ③ 版本（2026-10-24 官方穩定版）

來源 `apps/desktop_pet/Cargo.toml` 宣告 `tauri="2.12"`、`tauri-build="2.7"`；`frontend/Cargo.toml` 使用 Leptos 0.8 CSR、wasm-bindgen 0.2、web-sys 0.3；資料層共用 SQLx 0.9。來源 `Cargo.lock@1b637de69d62d6413631b11b3e1af2bc85caa40d` 與官方最新穩定版分開記錄：

| 元件 | 最新穩定 | 來源宣告／lock | 官方核對 |
| --- | --- | --- | --- |
| Rust | **1.99.1**（2026-10-15） | toolchain=`stable`；實際安裝未核 | [Rust Release](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) |
| Tauri | **2.12.5** | `2.12`／2.12.5 | [Tauri crate](https://docs.rs/crate/tauri/latest)；3.0 alpha 不算穩定 |
| tauri-build | **2.7.4** | `2.7`／2.7.4 | [Tauri build crate](https://docs.rs/crate/tauri-build/latest) |
| Leptos | **0.8.22** | `0.8`／0.8.22 | [Leptos crate](https://docs.rs/crate/leptos/latest) |
| SQLx | **0.9.0** | `0.9`／0.9.0 | [SQLx crate](https://docs.rs/crate/sqlx/latest) |
| uuid | **1.29.0** | `1.28`／1.28.0 | [uuid crate](https://docs.rs/crate/uuid/latest)；本篇只用既有 `parse_str`，不要求升級 |
| wasm-bindgen／web-sys | **0.2.129／0.3.106** | `0.2`／0.2.129、`0.3`／0.3.106 | [wasm-bindgen](https://docs.rs/crate/wasm-bindgen/latest)、[web-sys](https://docs.rs/crate/web-sys/latest) |
| SQLite engine／CLI | **3.54.1**（2026-10-16） | runtime/CLI 未核 | [SQLite 版本史](https://www.sqlite.org/chronology.html)；實際 `SELECT sqlite_version()` 待執行 |

**不新增依賴、不變更 lock**；Rust native 與 WASM/Trunk 構建須分別確認，Linux 原生 WebKitGTK、Tauri runtime 與 OS 套件版本未核，不能聲稱全平台相容。Day31 的 Tauri `main` 隱藏邏輯為必要前置條件。

## ④ 驗證命令、預期與實際結果

1. **Red**：`cargo test --locked -p desktop-pet day32_control_selection_tests -- --nocapture`；預期 Green 前找不到 `control_menu_pet_id`。**實際：未執行**，來源唯讀。
2. **Green**：貼上 A/B/C 後 `cargo test --locked -p desktop-pet day32_ -- --nocapture`，預期 parser 與 SQLite selection 均通過；`cargo test --locked -p desktop-pet` 全回歸、`cargo clippy -p desktop-pet --all-targets -- -D warnings`，預期不回歸。**實際：未執行**。
3. **WASM／原生**：`cd apps/desktop_pet/frontend && trunk build`；再 `cd .. && cargo tauri dev`（須本機安裝對應 Tauri CLI、Trunk、平台依賴），預期選單新 id 被 native event 收到，`main` 不重建。**實際：未執行**。
4. **GUI 驗收**：用隔離 SQLite 建兩隻、同時出籠，控制台先選小橘→按 X 隱藏→從阿龜右鍵開控制窗→控制台顯示阿龜，兩生活窗仍存在；重複五次仍僅一個控制窗；`SELECT pet_id FROM local_pet_selection WHERE slot=1` 等於阿龜 id；重新啟動後 selection 一致。另測不存在 UUID／DB 錯誤／控制台顯示失敗。**實際：未執行**。
5. **來源 CI 參考**：`rust_ai_chat@be0494a2a70be91eb74b2b726970a23fc3e5c044` 的 [Rust Desktop Pet](https://github.com/williechen/rust_ai_chat/actions/runs/39977648515) 於 2026-10-23 結論 success；該 CI 不含本篇尚未套用的程式，也不代表 GUI 驗收。

**交付與驗收邊界**：本篇教學可依明確插入點套用，GitHub 文件發文與產品套用分離。rust_ai_chat 無寫入；Red／Green、SQLite、native GUI、WASM／跨平台驗收皆**未執行／未驗收**。剩餘：事件通知失敗補償、真實多窗選取與焦點、漫遊逐隻設定／跨螢幕、單圖動畫及 Day25 FIFO。不要因 Day32 發布便標記產品完成。
