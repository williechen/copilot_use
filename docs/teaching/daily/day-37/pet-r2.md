# Pet Day37 r2｜先保存再開窗，失敗回復原標記並序列化生命週期

2026-10-25（Asia/Taipei）；文件基準 `copilot_use/main@cc4d5c8ecf283680d34a48f65a52a0af9fc40b2b`；程式來源 [`rust_ai_ring/main@5c713553daee4055738239f047d6c774fcc7a7a1`](https://github.com/williechen/rust_ai_ring/tree/5c713553daee4055738239f047d6c774fcc7a7a1)，全庫唯讀。規則 `daily-tdd-minimum-v1 / github-only-v1`；[規則正本](../../../COMMON_LEARNING_RULES.md)。本修訂新增檔案，保留 [Day37 原稿](pet.md) 的原始 commit/blob/全文；接續 [上一篇](../day-36/pet-r3.md)。**本篇程式尚未套用；新 Red/Green、CI、DB、HTTP、瀏覽器／原生 GUI 均未執行，不宣稱產品驗收。** 交付 commit/blob/SHA256 與時間以同目錄 `pet-r2-readback.json` 為準，文件 Gate 1 須在精確讀回及條件同步後才完成。

## ① 學習大綱

現行建立／命名／SQLite／多生活窗與 uncaged 啟動恢復已有真 caller。本日從 `pet_apps/pet_server/src/pet_windows.rs::open_saved_pet` 的一致性缺口接續：現行先 `open` 再 `set_caged(false)`，DB 失敗時已開窗。原稿以預先 existed 判斷關窗補償，存在同 ID 併行誤關窗問題。

本修訂改為**DB 寫入成功後才呼叫 open**；open 失敗時回復真正讀到的舊值，回復失敗則明確回錯。同程序所有會影響出入籠與選取的入口共用 async lock，避免另一個成功操作的 flag 被補償覆寫。全域 lock 比逐 ID map 簡單且避免無界 key；目前互動量小，代價是不同寵物的生命週期操作依序執行，活動／動畫不持此鎖。

真鏈：`pet_apps/pet_ui/src/main.rs` Leptos CSR → 既有 `open_saved_pet` IPC／回應錯誤顯示 → Tauri command → Store → `pet-persistence` SQLx SQLite；`open` → UUID label／200×200 生活窗／Runtime。沒有第二套產品、沒有新增 IPC 名稱、沒有新畫面或資料模型。本篇只處理受控命令的失敗與補償；預設資格與回籠欄位仍混用，完整重啟政策尚未完成。

## ② Red → 完整 Green → Refactor

### Red：測試生產 caller 將共用的操作順序

在 `pet_apps/pet_server/src/pet_windows.rs` 尾端加完整模組。來源尚無 `persist_then_open`／`lifecycle_guard`／`LIFECYCLE`，預期先為缺符號編譯 Red；是明列的待新增介面，非已通過的回歸測試。Green 後同一函式會被正式 IPC 使用；fake closure 只驗證順序／錯誤契約，不能當 Tauri GUI 實測。

```rust
#[cfg(test)]
mod day37_open_tests {
    use super::*;
    use std::{cell::{Cell, RefCell}, future::ready};

    #[tokio::test]
    async fn persist_failure_never_opens_or_closes_a_window() {
        let opened = Cell::new(false);
        let writes = RefCell::new(Vec::new());
        let result = persist_then_open(true,
            |flag| { writes.borrow_mut().push(flag); ready(Err("DB".into())) },
            || { opened.set(true); Ok(()) }).await;
        assert!(result.is_err());
        assert!(!opened.get());
        assert_eq!(*writes.borrow(), vec![false]);
    }

    #[tokio::test]
    async fn open_failure_restores_the_actual_old_flag() {
        for old in [true, false] {
            let writes = RefCell::new(Vec::new());
            let result = persist_then_open(old,
                |flag| { writes.borrow_mut().push(flag); ready(Ok(())) },
                || Err("native".into())).await;
            assert!(result.unwrap_err().contains("native"));
            assert_eq!(*writes.borrow(), vec![false, old]);
        }
    }

    #[tokio::test]
    async fn success_writes_once_and_rollback_error_is_visible() {
        let writes = RefCell::new(Vec::new());
        persist_then_open(true,
            |flag| { writes.borrow_mut().push(flag); ready(Ok(())) },
            || Ok(())).await.unwrap();
        assert_eq!(*writes.borrow(), vec![false]);
        let calls = Cell::new(0);
        let error = persist_then_open(true,
            |_| { calls.set(calls.get()+1);
                ready(if calls.get()==1 { Ok(()) } else { Err("rollback".into()) }) },
            || Err("native".into())).await.unwrap_err();
        assert!(error.contains("native") && error.contains("rollback"));
    }

    #[tokio::test]
    async fn lifecycle_lock_excludes_a_second_operation() {
        let first = lifecycle_guard().await;
        assert!(LIFECYCLE.try_lock().is_err());
        drop(first);
        assert!(LIFECYCLE.try_lock().is_ok());
    }
}
```

在 `pet_apps/pet_persistence/src/lib.rs` 尾端加入以下 SQLite 測試；原來源缺 `read_caged`，也是預期缺符號 Red，非環境失敗。

```rust
#[cfg(test)]
mod day37_open_state_tests {
    use super::*;
    #[sqlx::test(migrations = "../migrations")]
    async fn read_flag_and_fail_closed_for_missing_pet(pool: sqlx::SqlitePool) {
        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','本機')")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id,is_default_closure)
            VALUES('a','A','tortoise','local-owner',1)")
            .execute(&pool).await.unwrap();
        assert!(read_caged(&pool,"a").await.unwrap());
        set_caged(&pool,"a",false).await.unwrap();
        assert!(!read_caged(&pool,"a").await.unwrap());
        assert!(read_caged(&pool,"missing").await.is_err());
        pool.close().await;
        assert!(read_caged(&pool,"a").await.is_err());
    }
}
```

### Green A：讀取原標記、保存前置操作

在 `pet_apps/pet_persistence/src/lib.rs` 的 `set_caged` 後加入完整 adapter；缺資料、讀取失敗、非 0/1 均不可猜成某個舊狀態。原欄位 BOOLEAN DEFAULT FALSE 允許 NULL；遇歷史 NULL 解碼錯誤也 fail closed，不把未知值猜成 false。

```rust
pub async fn read_caged(pool: &sqlx::SqlitePool, id: &str) -> Result<bool, String> {
    let value: Option<i64> = sqlx::query_scalar(
        "SELECT is_default_closure FROM pet_profile
         WHERE id=?1 AND owner_id='local-owner'"
    ).bind(id).fetch_optional(pool).await
        .map_err(|_| "無法讀取回籠狀態".to_owned())?;
    match value {
        Some(0) => Ok(false),
        Some(1) => Ok(true),
        Some(_) => Err("回籠狀態資料不合法".into()),
        None => Err("找不到寵物".into()),
    }
}
```

在 `pet_apps/pet_server/src/pet_store.rs` 的 `impl PetStore` 內新增：

```rust
pub async fn read_caged(&self, id: &str) -> Result<bool, String> {
    pet_persistence::read_caged(&self.pool, id).await
}
```

在 `pet_apps/pet_server/src/pet_windows.rs` 的 imports 後新增以下完整 coordinator；只管操作順序，既有 SQLx adapter／Tauri `open` 透過真 closure 接入。

```rust
// 產品目前單程序；全域序列化所有出／入籠、選取與啟動恢復。
// 不持有 std::sync::MutexGuard 跨 await；Tauri command future 仍可 Send。
static LIFECYCLE: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

pub(crate) async fn lifecycle_guard() -> tokio::sync::MutexGuard<'static, ()> {
    LIFECYCLE.lock().await
}

async fn persist_then_open<W, F, O>(
    old_caged: bool,
    mut write: W,
    open_window: O,
) -> Result<(), String>
where
    W: FnMut(bool) -> F,
    F: std::future::Future<Output = Result<(), String>>,
    O: FnOnce() -> Result<(), String>,
{
    write(false).await?;
    if let Err(error) = open_window() {
        return match write(old_caged).await {
            Ok(()) => Err(format!("開窗失敗，已恢復原回籠值：{error}")),
            Err(rollback) => Err(format!("開窗失敗：{error}；DB 補償失敗：{rollback}")),
        };
    }
    Ok(())
}
```

以以下完整函式取代同檔 `open_saved_pet`：

```rust
#[tauri::command]
pub async fn open_saved_pet(
    id: String,
    app: tauri::AppHandle,
    store: State<'_, PetStore>,
) -> Result<(), String> {
    let _operation = lifecycle_guard().await;
    let pet = store.profile(&id).await?;
    let old_caged = store.read_caged(&id).await?;
    persist_then_open(old_caged,
        |flag| store.set_caged(&id, flag),
        || open(&app, &pet)).await
}
```

DB 失敗時 `open` 根本未執行，既有窗不被關閉、Runtime 不被移除。新窗 build 失敗時原 `open` 已清理當次 machines insert；既有窗 show／focus 失敗時保留既有窗，只回復舊 DB 值。若補償 SQL 失敗，回傳兩個錯誤，不能聲稱回復成功。此補償不是跨 DB／OS 原子交易；程序在 DB commit 與開窗之間崩潰仍可能留下 flag，需另作啟動恢復／狀態模型設計。

### Green B：所有現行狀態寫入入口接到同一鎖

只鎖 `open_saved_pet` 不足以防回籠／選取／startup 交錯。以下四處精確新增，原函式其它內容全部保留；不能少做任一處，也不能在 Store 方法再鎖造成非重入死鎖。

1. 同檔 `cage_saved_pet` 函式開頭（在 `store.profile(&id).await?;` 前）新增：

```rust
let _operation = lifecycle_guard().await;
```

2. `pet_apps/pet_server/src/lib.rs::select_pet` 開頭（在 `let saved = store.select(&id).await?;` 前）新增：

```rust
let _operation = pet_windows::lifecycle_guard().await;
```

3. 同檔 `.setup` 內，啟動恢復的 `tauri::async_runtime::spawn(async move {` 後、`match store.uncaged().await` **前**新增（鎖須包含讀出清單及完整開窗迴圈，避免舊清單在回籠成功後又開窗）：

```rust
let _operation = pet_windows::lifecycle_guard().await;
```

4. 同檔 `app.on_menu_event` 的 `if let Some(id) = control_menu_pet_id(...)` 分支，該分支 `spawn(async move {` 後、`match store.select(&id).await` 前新增：

```rust
let _operation = pet_windows::lifecycle_guard().await;
```

靜態核對該 SHA：以上是生產中 `store.select`、`set_caged` 及 `pet_windows::open` 的全部入口；Store 內其它同名呼叫位於測試。原生 Destroyed 只清 Runtime，不 await、不取得此鎖；不應在視窗事件中 blocking_lock。直接關窗的持久化、延遲 Destroyed 與重新開窗的事件識別尚屬既有缺口，須另驗；本切片不宣稱該流程完成或全面 race-free。select／menu 等入口只新增協調鎖，既有 select 失敗補償不在本次完成範圍。

### Refactor、上一 Day 未完成事項與原稿更正

原稿的 `existed`／關窗補償整段不採用，**不修改原稿**；`read_caged` 與補償是本修訂內容。`pet_persistence` 無 Tauri 依賴，Store 僅橋接。`pet-application` 既有名稱規則保持真 caller，不建立空層。Day36 的 Store 重建測試不代表 native 程序重啟；三窗重啟、直接關窗、預設資格／出籠分離與動態 ID 漫遊仍按 FUTURE_WORK 既有項目追蹤，FIFO 不結案。

Cargo：`pet_apps` workspace；`pet-server`／`pet-persistence` package 與 `pet_server`／`pet_persistence` crate 分清。既有 tokio `full` 已含 sync／macros／runtime，tauri 2.12、sqlx sqlite／macros 已存在。此文使用 std::future 與完整 tokio path，不新增 imports／dependencies／features／migrations，四個 lock 不改；Pet CSR 的 Trunk／Tauri metadata、既有 command 註冊與 UI 錯誤顯示原樣。

## ③ 套件與軟體版本

交付日 2026-10-25，直接讀官方 crates.io API 的 versions，排除預發行與 yanked；不是以 Cargo.lock 猜測最新版。逐項官方網址、發行日期與 MSRV 原始回應見 [共用版本證據](versions-r2.json)。四個 workspace 的 manifest／lock 已讀；下表為本篇使用範圍。

| 項目 | manifest 宣告 | lock resolved | 當日官方最新穩定 | MSRV |
| --- | --- | --- | --- | --- |
| tauri | `2.12` | 2.12.5 | [2.12.5](https://crates.io/crates/tauri/2.12.5) | 1.90 |
| tauri-build | `2.7` | 2.7.4 | [2.7.4](https://crates.io/crates/tauri-build/2.7.4) | 1.90 |
| sqlx | `0.9` | 0.9.0 | [0.9.0](https://crates.io/crates/sqlx/0.9.0) | 1.94.0 |
| tokio | `1.54 / full` | 1.54.1 | [1.54.1](https://crates.io/crates/tokio/1.54.1) | 1.85 |
| leptos | `0.8` | 0.8.22 | [0.8.22](https://crates.io/crates/leptos/0.8.22) | 1.88 |
| wasm-bindgen | `0.2` | 0.2.129 | [0.2.129](https://crates.io/crates/wasm-bindgen/0.2.129) | 1.81 |
| wasm-bindgen-futures | `0.4` | 0.4.79 | [0.4.79](https://crates.io/crates/wasm-bindgen-futures/0.4.79) | 1.81 |
| web-sys | `0.3` | 0.3.106 | [0.3.106](https://crates.io/crates/web-sys/0.3.106) | 1.81 |
| serde | `1.0` | 1.0.229 | [1.0.229](https://crates.io/crates/serde/1.0.229) | 1.56 |
| serde_json | `1.0` | 1.0.154 | [1.0.154](https://crates.io/crates/serde_json/1.0.154) | 1.71 |
| uuid | `1.29` | 1.29.0 | [1.29.0](https://crates.io/crates/uuid/1.29.0) | 1.89.0 |
| trunk | `CI 0.21.14` | 工具，不在產品 lock | [0.22.0](https://crates.io/crates/trunk/0.22.0) | 1.96.1 |

Rust 官方 stable manifest 為 **1.99.1**（2026-10-15），[官方公告](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)；本機 `rustc 1.99.1`、`cargo 1.99.1`，Edition 2024，workspace 未固定 rust-version。所有上列已宣告 MSRV 均不高於本機 toolchain。建議採表列當日穩定版本；產品 lock 已相符者保留，無需 cargo update；相同 lock 的 native／WASM 相容仍須第④節實測，不能從 MSRV 推定成功。sha2 0.10.9 若見於 lock 屬並存的傳遞依賴，不任意強制升成 0.11。

SQLite 官方最新 [3.54.1](https://www.sqlite.org/chronology.html)；本次未查產品 runtime `sqlite_version()`，不能把 Python 3.46.1 當產品；SQLx 0.9.0／libsqlite3-sys 隨 lock 保留，本篇查詢只用原有欄位，不新增 DB 功能。Trunk 本機／CI 0.21.14 落後官方 0.22.0；建議新環境評估 0.22.0（MSRV 1.96.1 可滿足），現有交付先保留已鎖 CI 以避免混入工具升級，須跑 CSR build／Tauri 資源路徑與真 GUI 後才能更換。沒有宣稱已升級或測過 0.22.0。

Tauri Linux native 需 WebKitGTK 4.1 ABI、GTK3、libsoup3 等；本機 pkg-config GTK 3.24.52／WebKitGTK 2.52.6 只記錄環境，並非任務要升級的套件或最新上游版宣告。新 coordinator 不變更 native API／ABI；實際 link、GUI 及跨平台仍待驗。
## ④ 驗證項目與交付界線

使用者完整套用所有段落後，在 `pet_apps` 執行：

```bash
cargo test --locked -p pet-persistence day37_open_state_tests
cargo test --locked -p pet-server day37_open_tests
cargo test --locked -p pet-server
cargo check --locked -p pet-server
cargo check --locked -p pet-ui --target wasm32-unknown-unknown
cd pet_ui
env NO_COLOR=true trunk build --locked
```

預期：Red 缺新增符號；Green 後新增測試與原測試通過。Tauri 原生 runner 再注入①第一次 DB 寫入失敗（新窗／既有窗均不變）；②build／show／focus 失敗（舊值 true/false 均精確回復）；③補償 SQL 失敗（兩錯誤可見）；④同 UUID 雙出籠、出籠對回籠、select、menu、startup 交錯（lock 不遺漏）；⑤不同 UUID 三窗互不誤關、snapshot 不重設；⑥延遲 Destroyed、直接關窗、真正重啟與 preset 缺口。fake closure 不涵蓋 GUI，必須從 Leptos 按鈕到 SQLite／OS 結果逐項驗。

實際：新 Red/Green、DB、WASM、Trunk、原生測試**均未執行**，未修改產品。來源 SHA 的 [Rust Desktop Pet](https://github.com/williechen/rust_ai_ring/actions/runs/40174572112)（check-pet／build-pet，08:05:52Z）與 [Test All](https://github.com/williechen/rust_ai_ring/actions/runs/40174572101) 的 test-pet-server（workflow 07:57:53Z 完成）success，只涵蓋原版 build／測試，不含本修訂；CI 沒有本篇 native 故障注入。文件交付與產品驗收分開、不封存、不更新 last_full_comparison。
