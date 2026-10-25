# Pet Day36 r3｜重啟恢復全部未回籠寵物，而非只恢復最後選取者

> **2026-10-25 實測更新：程式已存在於 [rust_ai_ring/main@5c713553](https://github.com/williechen/rust_ai_ring/commit/5c713553daee4055738239f047d6c774fcc7a7a1)；相關測試已執行通過；完整產品尚未驗收。** 本次只更新文件，產品庫保持唯讀。下方問題分析及 Red／Green 教材以原始 `61ed5930` 為基準；最新 main 已含 Green，請勿重複套用。
>
> 本修訂取代「目前程式未套用／未測試」摘要；[原始文章](pet-r2.md)及其 commit/blob/SHA256、原始交付證據保持不變。Day36 不加號，next_day 維持 37。各命令、原始錯誤與環境復原見[完整實測報告](../../../source-review/2026-10-25-day36-verification-5c713553.md)，[logs／腳本與證據包](../../../source-review/day36-verification-5c713553/evidence.zip)。

## ① 學習大綱：真實多寵物重啟缺口

**2026-10-25；程式來源 `williechen/rust_ai_ring/main@61ed5930bd9ff38e0d5822eea2983121001d8422`；文件庫 `williechen/copilot_use/main`；`next_day.pet=36`。** 接續 [Day35：逐 ID 回籠狀態 SQLx adapter](../day-35/pet.md)。Day35 已在後續來源接通 `pet_persistence::set_caged`，本篇不重複搬移；本日只補**重啟時開啟所有未回籠寵物**。

現行真實鏈：`pet_ui` Leptos CSR → `bridge.rs` Tauri invoke → `pet_server::create_pet`／`select_pet`／`pet_windows::open_saved_pet`／`cage_saved_pet` → `pet_store::PetStore` → `pet_persistence::set_caged`／SQLite `pet_profile.is_default_closure`；Tauri `application().setup` 目前只呼叫 `store.active()`，它讀取 `local_pet_selection.slot=1`，所以即使 A、B、C 都出籠，重啟只恢復最後選取的一隻。原生三窗、逐 ID 睡／醒、回籠與控制窗關閉重開已局部實測，**重啟恢復多隻失敗**（見 [原生驗證](../../../source-review/2026-10-25-pet-native-5d28e96.md)）。本日修復啟動查詢與實際建窗 caller，不建立另一個寵物 Runtime。

完成界線：SQLite 依 owner、未回籠標記列出全部 ID；Tauri 逐一建立各自 `pet-{UUID simple}` 生活窗；回籠者不恢復；單隻建窗失敗不阻止其他隻且不偷偷改 DB；原先「選取一隻」只決定控制台焦點，不再決定重啟開窗清單。**原生 GUI、失敗補償、動態漫遊、單幀單圖與真正重啟驗收仍需實機執行。**

## ② Red → 完整最小 Green → Refactor

### Red：先測出只有 active slot 的錯誤假設

在 `pet_apps/pet_server/src/pet_store.rs` 的既有 `#[cfg(test)] mod day22_tests` 增加下列測試。未新增 `uncaged` 前預期**編譯失敗**；現行 `store.active()` 在選取第三隻後只回第三隻，無法滿足預期的兩個未回籠 ID。SQLx test 使用 `pet_apps/migrations`，不是虛構 in-memory 資料：

~~~rust
#[sqlx::test(migrations = "../migrations")]
async fn day36_restart_restores_all_uncaged_not_just_selected(pool: sqlx::SqlitePool) {
    let store = PetStore { pool: pool.clone() };
    let first = store.create("小橘").await.unwrap();
    let caged = store.create("阿龜").await.unwrap();
    let third = store.create("小白").await.unwrap();
    store.select(&first.id).await.unwrap();
    store.select(&third.id).await.unwrap();
    store.set_caged(&caged.id, true).await.unwrap();
    assert_eq!(store.active().await.unwrap(), Some(third.clone()));

    // 模擬程式重啟：以同一持久化 DB 重新建立 Store，不依賴 Runtime 記憶體。
    let restarted = PetStore { pool: pool.clone() };
    let pets = restarted.uncaged().await.unwrap();
    let ids: std::collections::HashSet<_> = pets.into_iter().map(|p| p.id).collect();
    assert_eq!(ids, [first.id, third.id].into_iter().collect());
    assert!(!ids.contains(&caged.id));
}
~~~

在 `pet_apps/pet_persistence/src/lib.rs` 新增下列 Red，`list_uncaged` 尚不存在預期編譯失敗；測試還驗 owner 範圍、回籠排除及 DB fail closed：

~~~rust
#[cfg(test)]
mod day36_restore_adapter_tests {
    use super::list_uncaged;
    #[sqlx::test(migrations = "../migrations")]
    async fn reads_only_uncaged_local_owner(pool: sqlx::SqlitePool) {
        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','本機')")
            .execute(&pool).await.unwrap();
        for (id, caged) in [("pet-a",false),("pet-b",true),("pet-c",false)] {
            sqlx::query(
                "INSERT INTO pet_profile(id,name,species,owner_id,is_default_closure)
                 VALUES(?1,?2,'tortoise','local-owner',?3)"
            ).bind(id).bind(id).bind(caged).execute(&pool).await.unwrap();
        }
        let rows = list_uncaged(&pool).await.unwrap();
        assert_eq!(rows.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(),
                   vec!["pet-a","pet-c"]);
        pool.close().await;
        assert!(list_uncaged(&pool).await.is_err());
    }
}
~~~

### Green A：真正 SQLx 讀取 adapter（只新增必要的兩個項目）

在 `pet_apps/pet_persistence/src/lib.rs` 的現有 `set_caged` 函式**之後、原有測試 module 之前**新增：

~~~rust
/// 重啟查詢 DTO；不依賴 Tauri 或 server 的 SavedPet。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UncagedPet {
    pub id: String,
    pub name: String,
    pub species: String,
}

/// 只查本機 owner 的未回籠寵物；slot=1 不是出籠清單。
pub async fn list_uncaged(pool: &sqlx::SqlitePool) -> Result<Vec<UncagedPet>, String> {
    let rows: Vec<(String, String, String)> = sqlx::query_as(
        "SELECT id, name, species
         FROM pet_profile
         WHERE owner_id='local-owner'
           AND COALESCE(is_default_closure, 0)=0
         ORDER BY name, id"
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "無法讀取出籠寵物".to_owned())?;
    Ok(rows.into_iter().map(|(id,name,species)| UncagedPet {
        id, name, species
    }).collect())
}
~~~

`pet_apps/pet_server/src/pet_store.rs` 的既有 `impl PetStore` 中，在 `set_caged` 前**新增**，保留 `SavedPet` 既有序列化契約：

~~~rust
pub async fn uncaged(&self) -> Result<Vec<SavedPet>, String> {
    let rows = pet_persistence::list_uncaged(&self.pool).await?;
    Ok(rows.into_iter().map(|row| SavedPet {
        id: row.id,
        name: row.name,
        species: row.species,
    }).collect())
}
~~~

### Green B：實際 Tauri 啟動 caller（不是只加 helper）

`pet_apps/pet_server/src/lib.rs` `application` 內原 `.setup(|app| {` 的第一個 `tauri::async_runtime::spawn`，**從 `let handle` 起至該 spawn 的 `});` 完整替換**為：

~~~rust
let handle = app.handle().clone();
let store = app.state::<pet_store::PetStore>().inner().clone();
tauri::async_runtime::spawn(async move {
    match store.uncaged().await {
        Ok(pets) => {
            for pet in pets {
                // 每個 UUID 由既有 open() 建立獨立原生視窗與 Runtime。
                // 單隻失敗不應讓其他未回籠寵物無法恢復。
                if let Err(error) = pet_windows::open(&handle, &pet) {
                    eprintln!("重啟恢復寵物 {} 失敗：{error}", pet.id);
                }
            }
        }
        Err(error) => {
            // DB 失敗時不可誤認沒有寵物，更不可將其標成回籠。
            eprintln!("重啟讀取出籠寵物失敗：{error}");
        }
    }
});
~~~

原 `.on_window_event` 的 Destroyed 清理、`Runtime`、`invoke_handler`、`select_pet`、`open_saved_pet`、`cage_saved_pet`、`roam_desktop` 與 `pet_ui` JS bridge **不變**。`open` 已依 UUID 產生唯一 label、重複呼叫會 show/focus，建窗失敗會移除 Runtime；本日沿用這些真實呼叫者，不另建視窗管理器。Tauri UI 仍用 `index.html?view=pet&pet_id=...`，Leptos metadata／capability 不變。

### Refactor、Cargo 與上一 Day／舊 Day 補充

重啟名單的資料責任由 `pet_persistence` 查詢，`PetStore` 僅轉為既有 `SavedPet`，Tauri 僅建窗。**本日沒有新增 crate、Cargo dependency/feature、Rust import 或 Cargo.lock 變更**：`pet_persistence` 已依賴 `sqlx.workspace`；`pet_server` 已依賴 `pet-persistence`；兩者 package 名稱分別 `pet-persistence`、`pet-server`，Rust import 是 `pet_persistence`。`pet_application` 已有名稱規則，不是空 crate；其餘仍留在 server 的 SQLx 不宣稱全部搬移。`analytics_persistence` 與本日無關。

Day35 的 `set_caged` 寫入成功才回傳，故本篇不重貼。Day28 的 `active` 仍有用途：控制台目前選取，不可因「只恢復一隻」而刪除；Day25 三窗／單幀單圖仍 pending，DB 單測不能結案。重啟時部分視窗失敗會記錄但不修改 DB，使用者可由既有控制台再出籠；更嚴格的開窗失敗補償、崩潰恢復、關窗失敗注入屬後續切片。漫遊固定 `main` label 與動態 UUID 不相符，保留為已知缺口。

## ③ 原始教材套件版本表（官方最新宣稱本次未重新核實）

本次已量測 Rust/Cargo 1.99.1、native/WASM target，並核對四 workspace 的 manifests／locks；所有 Cargo.toml 與 Cargo.lock 相對 `61ed5930` 未變更。以下版本表保留原始教學查核範圍，不據此宣稱本次重新查證了所有官方最新版本；SQLx 連結的 SQLite engine 未量測。

| 項目 | manifest → Cargo.lock（來源 61ed5930） | 官方最新穩定／相容性 |
| --- | --- | --- |
| Rust/Cargo | 各 workspace `rust-toolchain.toml` stable，安裝版未量測 | [Rust 1.99.1（10/15）](https://blog.rust-lang.org/releases/)；edition 2024 最低 1.85 |
| Tauri / tauri-build | `pet_server` 2.12 / 2.7 → **2.12.5 / 2.7.4** | [Tauri](https://docs.rs/crate/tauri/latest)／[tauri-build](https://docs.rs/crate/tauri-build/latest)；保留現有 native builder |
| SQLx SQLite | `pet_apps` 0.9/sqlite/macros/runtime-tokio → **0.9.0** | [SQLx](https://docs.rs/crate/sqlx/latest)，使用現有 query_as |
| Leptos CSR | `pet_ui` 0.8 → **0.8.22** | [Leptos](https://docs.rs/crate/leptos/latest)；本篇不動 UI |
| Tokio / Serde / UUID | 1.54 / 1.0 / 1.29 → **1.54.1 / 1.0.229 / 1.29.0** | [Tokio](https://docs.rs/crate/tokio/latest)／[Serde](https://docs.rs/crate/serde/latest)／[UUID](https://docs.rs/crate/uuid/latest) |
| SQLite engine | 執行時未量測 | [3.54.1](https://www.sqlite.org/download.html)，不可推定等於 SQLx 實際連結版本 |

四份 lock blob：Chat `384bcb23a13bc0f4e876f29e3037f2181ebbc3d5`、Pet `7097f943da5c88be851979cec74641fa35e6e613`、Analytics `44abb16f1ec3f74fa181c69d46e8bd984a090f21`、Crates `c5ec8079981d59244ce5482258424cd8ff57f9a3`。全部 manifest/lock 以來源精確 SHA 讀取；最新穩定版不等於已升級或安裝，排除 3.0 alpha、RC。Tauri Linux WebKit/GTK、Windows WebView2、macOS native 需求未在此環境測試；維持現有相容版本，無新 migration。


## ④ 已實際執行的驗證與剩餘缺口

精確來源：`5c713553daee4055738239f047d6c774fcc7a7a1`。使用 archive 解出的無 `.git` 隔離副本；原本兩個本地 repo 及產品遠端均未改動。下列命令於 `pet_apps` 執行，全部最終 exit 0。環境為 `CARGO_TARGET_DIR=/tmp/day36-verify-target`、`CARGO_BUILD_JOBS=2`、dev/test debug=0、`CARGO_INCREMENTAL=0`；沒有更新 lock、依賴或產品程式。

~~~bash
cargo metadata --locked --offline --format-version 1 --no-deps
cargo fmt --all -- --check
cargo test --locked --offline -p pet-persistence day36_restore_adapter_tests
cargo test --locked --offline -p pet-server day36_restart_restores_all_uncaged_not_just_selected
cargo test --locked --offline -p pet-server --all-targets
cargo check --locked --offline -p pet-ui --target wasm32-unknown-unknown
~~~

Day36 persistence adapter **1 passed**，重建 Store 清單 **1 passed**，完整 server **54 passed**，UI WASM check 通過。真實 Tauri setup → uncaged → open caller 已編譯接通。在另一份 `61ed5930` 只加入 adapter Red，`list_uncaged` 缺符號 E0432／exit 101，符合預期；Store 缺符號 Red 未另外執行。

**測試界線：**「重啟」單測只用同一 pool 重建 PetStore，不是結束／重開原生程序。adapter fixture 只有 local-owner，尚無異 owner fixture 排除斷言；程式 SQL 的 owner filter 已靜態核對。

**尚未驗收：** 真 Tauri 重啟後 A/C 各自開窗、B 回籠不出現；單隻建窗失敗時其他視窗仍恢復且 DB 不變；逐寵物睡醒、動態漫遊、單幀單圖及實際 bundle/dist。不能把 DB 單測與 native 編譯成功當多窗重啟驗收。

原始錯誤如實保留：首次 native 編譯遇暫存配額 os error 122，調整本次自建快取後重跑成功。Chat 探測曾誤用同帳號作控制組，Analytics 探測初版誤用 /health；修正後的成功 log 才作結論，原失敗另存證據包。獨立 PG 與 server 均已停止，本次快取已清理。

**判定：程式在上述精確 main 已存在，測試已執行；完整產品仍未驗收，不封存、不推進 Day、不更新 last_full_comparison。**
