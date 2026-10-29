# Pet Day35 r1｜逐寵物回籠狀態搬入 pet-persistence，接回 Tauri 真實呼叫鏈

## ① 學習大綱：最小切片與前置現況

交付日期：2026-10-25（Asia/Taipei）；文件只寫 williechen/copilot_use/main；唯一產品來源 williechen/rust_ai_ring/main@891ff32ae29e1588dc40027cd5c9c99ca91d2378，唯讀。依即時 next_day=35 接續 [Pet Day34](../day-34/pet.md)。上一篇 pet-application::normalize_new_pet_name 已接 Tauri 與 PetStore，不能再以空 application 或名稱未接通當新缺口；pet-persistence/src/lib.rs 目前確實為空，SQLx PetStore 仍在 pet_server。

**本日唯一切片：** 將現有 PetStore::set_caged(id, bool) 的逐 ID SQLite UPDATE 移到 pet-persistence，原有 PetStore 方法變成真實轉接；Tauri cage_saved_pet／open_saved_pet、Leptos CSR、bridge、三窗 Runtime 及 DB schema 不變。新增 adapter 層的三 ID 隔離／不存在 ID／資料庫失敗測試與 server 呼叫者測試。這是「回籠／出籠標記」單一可驗證 SQLx adapter 切片，**不是**聲稱整個 PetStore 已搬完或整個 pet-persistence 完成。無呼叫者的骨架不建立。

真實鏈：pet_ui/src/main.rs 的生活窗「回籠子」→ pet_ui/src/bridge.rs::cage_saved_pet → index.html::cageSavedPet → Tauri pet_server/src/pet_windows.rs::cage_saved_pet → pet_server::pet_store::PetStore::set_caged → **pet_persistence::set_caged** → SQLite pet_profile.is_default_closure；更新成功才關窗並移除該 ID Runtime，關窗失敗則用同一 adapter 回復 false。反向出籠：控制窗 open_saved_pet → pet_windows::open_saved_pet → PetStore::set_caged(false)。UI／IPC／SQLx 不應直接混成一個函式。

前置來源四 workspace manifests/locks 已核對：chat_apps、pet_apps、analytics_apps package version 0.2.34，crates 0.2.32。檔案目錄 pet_server/pet_ui/pet_persistence，Cargo package pet-server/pet-ui/pet-persistence，Rust crate pet_server/pet_ui/pet_persistence。既有 Pet 的 Leptos 0.8 CSR + Tauri 2.12 不改成 SSR；Chat 的 cargo-leptos bin/lib metadata 不套用到 Pet。

## ② Red → 完整最小 Green → Refactor

### Red：先讓新 adapter API 與實際 server 呼叫者暴露缺口

在 pet_apps/pet_persistence/src/lib.rs（目前空檔）**先只放下列完整測試模組**，尚未加 set_caged：預期無此符號的編譯失敗；不是 DB 連線失敗。

~~~rust
#[cfg(test)]
mod day35_cage_adapter_tests {
    use super::set_caged;

    #[sqlx::test(migrations = "../migrations")]
    async fn only_target_pet_changes_and_missing_id_fails(pool: sqlx::SqlitePool) {
        sqlx::query("INSERT INTO pet_owner(id,name) VALUES('local-owner','本機主人')")
            .execute(&pool).await.unwrap();
        for id in ["pet-a", "pet-b", "pet-c"] {
            sqlx::query("INSERT INTO pet_profile(id,name,species,owner_id) VALUES(?1,?2,'tortoise','local-owner')")
                .bind(id).bind(id).execute(&pool).await.unwrap();
        }
        set_caged(&pool, "pet-b", true).await.unwrap();
        for (id, expected) in [("pet-a", 0_i64), ("pet-b", 1), ("pet-c", 0)] {
            let flag: i64 = sqlx::query_scalar(
                "SELECT is_default_closure FROM pet_profile WHERE id=?1"
            ).bind(id).fetch_one(&pool).await.unwrap();
            assert_eq!(flag, expected, "{id}");
        }
        assert!(set_caged(&pool, "missing-id", true).await.is_err());
        set_caged(&pool, "pet-b", false).await.unwrap();
        let flag: i64 = sqlx::query_scalar(
            "SELECT is_default_closure FROM pet_profile WHERE id='pet-b'"
        ).fetch_one(&pool).await.unwrap();
        assert_eq!(flag, 0);
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn failed_database_update_is_not_success(pool: sqlx::SqlitePool) {
        pool.close().await;
        assert!(set_caged(&pool, "pet-a", true).await.is_err());
    }
}
~~~

在 pet_apps/pet_server/src/pet_store.rs 檔尾新增實際 PetStore 轉接測試，沿用 Day34 已存在的 create/name/transaction，不另建假 adapter：

~~~rust
#[cfg(test)]
mod day35_cage_caller_tests {
    use super::PetStore;

    #[sqlx::test(migrations = "../migrations")]
    async fn live_store_cages_only_one_saved_pet(pool: sqlx::SqlitePool) {
        let store = PetStore { pool: pool.clone() };
        let first = store.create("小橘").await.unwrap();
        let second = store.create("阿龜").await.unwrap();
        store.set_caged(&second.id, true).await.unwrap();
        let first_flag: i64 = sqlx::query_scalar(
            "SELECT is_default_closure FROM pet_profile WHERE id=?1"
        ).bind(&first.id).fetch_one(&pool).await.unwrap();
        let second_flag: i64 = sqlx::query_scalar(
            "SELECT is_default_closure FROM pet_profile WHERE id=?1"
        ).bind(&second.id).fetch_one(&pool).await.unwrap();
        assert_eq!((first_flag, second_flag), (0, 1));
        assert!(store.set_caged("not-a-pet", true).await.is_err());
    }
}
~~~

若只加新測試，persistence 測試因缺 set_caged 無法編譯；若只加 server 測試，現行 set_caged 已可通過，屬舊實作回歸保護，**不能**冒稱此測試原本必敗。Red 的編譯／行為與環境錯誤必須區分。本次未執行。

### Green A：真正 SQLx adapter（完整 pet_persistence/src/lib.rs）

在前述測試模組**上方**加入以下完整函式；與測試模組合起來即為本檔完整內容，測試不要重複貼兩份：

~~~rust
/// 只負責逐寵物回籠標記；Tauri 視窗／Runtime 不屬於資料層。
pub async fn set_caged(
    pool: &sqlx::SqlitePool,
    id: &str,
    caged: bool,
) -> Result<(), String> {
    let result = sqlx::query(
        "UPDATE pet_profile SET is_default_closure=?1,
         updated_at=CURRENT_TIMESTAMP
         WHERE id=?2 AND owner_id='local-owner'",
    )
    .bind(caged)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|_| "無法保存回籠狀態".to_owned())?;
    if result.rows_affected() != 1 {
        return Err("找不到可回籠的寵物".into());
    }
    Ok(())
}
~~~

SQL 條件、bind 順序、rows_affected 與原 PetStore 相同；不把所有寵物一起 UPDATE、不建立新表、不在資料層操作 OS 視窗。錯誤文字保留原 IPC 契約。

### Green B：依賴、Cargo.lock、真正 caller

在 pet_apps/pet_persistence/Cargo.toml 的 [dependencies] 加入（其餘 serde/serde_json/thiserror/uuid 保留，不清理未證實用途）：

~~~toml
sqlx.workspace = true
~~~

在 pet_apps/pet_server/Cargo.toml 的 [dependencies] 加入：

~~~toml
pet-persistence = { path = "../pet_persistence" }
~~~

在 pet_apps/pet_server/src/pet_store.rs **只替換**原本完整 set_caged 方法（原方法第 151–166 行），保持同一個公開簽名：

~~~rust
pub async fn set_caged(&self, id: &str, caged: bool) -> Result<(), String> {
    pet_persistence::set_caged(&self.pool, id, caged).await
}
~~~

使用全限定 Rust crate 路徑 pet_persistence，無額外 use。pet_windows.rs 原 cage_saved_pet／open_saved_pet 已呼叫 PetStore::set_caged，不修改 Tauri command 簽名、generate_handler!、PetStore state 或 UI bridge；既有 day28_caged_pet_is_not_active 與選取／重啟測試都繼續走真實 adapter。原本 pet_store.rs 的其他 create/list/profile/select SQLx 留在 server，不能宣稱整庫已分層。

**Cargo.lock 必須同步：** pet_apps/Cargo.lock 中 [[package]] name="pet-persistence" version="0.2.34" 的 dependencies 加入 "sqlx"；[[package]] name="pet-server" version="0.2.34" 的 dependencies 加入 "pet-persistence"（在 pet-domain 之後）。兩處的完整新增行如下，其他既有 dependency/checksum 保留：

~~~diff
@@ [[package]] pet-persistence / dependencies
  "serde",
  "serde_json",
+ "sqlx",
  "thiserror 2.0.21",
@@ [[package]] pet-server / dependencies
  "pet-application",
  "pet-domain",
+ "pet-persistence",
  "serde",
~~~

從 pet_apps 執行 cargo metadata --locked 與 cargo check --locked；若 Cargo 提示 lock 不一致，確認上述兩條新增邊，不能以 cargo update 全庫升級取代必要 lock 修正。pet-persistence 沒有新 feature；SQLx sqlite/runtime-tokio/macros 已由 pet_apps workspace 提供。Pet UI Leptos CSR、Tauri capability、JS bridge、migration、schema、資源與 build.rs 均不變。

### Refactor、上一篇缺口與舊 Day 補充

Refactor 後由 PetStore 保留 application-facing 方法，持久化 adapter 只接 SQLx，Tauri 負責視窗關閉／補償，Leptos 仍以 IPC 收錯誤。關窗失敗的反向 set_caged(false) 也走同一 adapter，不漏改第二個 caller；已存在的單 ID Runtime 測試保留。若日後搬整個 PetStore，必須再核對 SavedPet DTO、SQLx migrations、state、tests 與 Tauri caller，不在本篇建立無呼叫者 trait。

Day34 的名稱控制字元規則已在 pet-application 並接真實 caller，保留；Day33 的控制窗 openPetControls 也已存在，不重貼。Day25 單幀單圖與三個不同 pet-id 的原生生活窗 FIFO 仍 pending：本日 DB 逐 ID 測試**不能**取代三窗 GUI、獨立事件、動作影格、拖曳／重啟的驗收。pet_ui/src/bridge.rs 的 validate_new_pet_name_js extern 無 Rust caller，先確認生成 JS 契約再清理，不因本篇重複套用。原 Day 與雜湊均保留。

## ③ 套件與軟體版本（2026-10-25 官方最新穩定）

| 項目 | 來源宣告／Cargo.lock | 官方最新穩定版 |
| --- | --- | --- |
| Rust/Cargo | rust-toolchain=stable，安裝版未量測 | [1.99.1（10/15）](https://blog.rust-lang.org/releases/) |
| SQLx SQLite | 0.9 + runtime-tokio/macros/sqlite → 0.9.0 | [0.9.0](https://docs.rs/crate/sqlx/latest) |
| Tauri / tauri-build | 2.12 → 2.12.5；2.7 → 2.7.4 | [Tauri](https://docs.rs/crate/tauri/latest)／[tauri-build](https://docs.rs/crate/tauri-build/latest) |
| Leptos CSR | pet-ui 0.8/csr → 0.8.22 | [0.8.22](https://docs.rs/crate/leptos/latest) |
| Tokio | 1.54/full → 1.54.1 | [1.54.1](https://docs.rs/crate/tokio/latest) |
| Serde / UUID / thiserror | 1.0 → 1.0.229；1.29 → 1.29.0；2.0 → 2.0.21 | [Serde](https://docs.rs/crate/serde/latest)／[UUID](https://docs.rs/crate/uuid/latest)／[thiserror](https://docs.rs/crate/thiserror/latest) |
| SQLite engine | 未量測 SQLx 內建／系統 runtime | [3.54.1（10/16）](https://www.sqlite.org/download.html) |

來源四份 lock blob：chat 8fe9db93e078e8633f283834fd01e6805d97f9b3、pet b38668fac8b3d8cd105d54d3baf9e3a8f0b3e1a5、analytics 9f6a1720ec999263d162a1e8c90c0fbc1b3dd6a7、crates 5321bfdaaaa24bb796c40092e39a4aa8a27f2d8f。此次新增的依賴只用已鎖定的 SQLx，建議保持原版本；Rust edition 2024 至少需要 1.85，建議已核對 stable 1.99.1。native Tauri 仍依 OS 的 WebKit/GTK 或 Windows WebView2 等環境；未測平台依賴與完整 MSRV。不要把最新版 SQLite CLI 視為應用程式實際 sqlite_version()；需另查。

## ④ 驗證：正確工作目錄、預期／實際與剩餘風險

在使用者的 rust_ai_ring 產品工作樹套 Red，再套完整 Green，**命令從 pet_apps 執行**：

~~~bash
cd pet_apps
cargo metadata --locked --format-version 1 --no-deps
cargo test --locked -p pet-persistence day35_cage_adapter_tests
cargo test --locked -p pet-server day35_cage_caller_tests
cargo test --locked -p pet-server day28_caged_pet_is_not_active
cargo test --locked -p pet-server --all-targets
cargo check --locked -p pet-ui --target wasm32-unknown-unknown
cargo fmt --all -- --check
# 在隔離 SQLite 連線查詢實際引擎：SELECT sqlite_version();
~~~

Red：新 adapter API 缺符號編譯失敗；Green：三 ID 只更新目標、未知 ID 不成功、closed pool 回錯誤，server caller／原有出入籠與選取回歸應通過。實機另驗 Leptos CSR → JS bridge → Tauri → SQLite → UI：建立三個不同 ID、出籠／回籠任一隻不影響另兩隻；關窗失敗時標記恢復；重啟後保留每隻標記，控制窗仍可重開。Tauri GUI 需平台原生依賴與可操作桌面，不以單元測試代替。

**實際結果：** 產品來源唯讀，未套用 Green、未跑 Rust/SQLite/Tauri/WASM/GUI/CI；文件已準備可套用的 Red、Green、Cargo/lock、caller 與測試，但 Gate 2 未驗收。Day25 單圖 FIFO、三窗原生事件、關窗失敗注入、SQLite 重啟與剩餘 PetStore adapter 搬移仍待後續；不更新 last_full_comparison。文章 commit、blob/SHA256、讀回與七首頁／ENTRY/進度另由交付證據追蹤。
