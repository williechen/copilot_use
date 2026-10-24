# Pet Day34 r1｜新增寵物名稱規則接入 application 並拒絕控制字元

## ① 學習大綱

交付日期：2026-10-25（Asia/Taipei）。文件基準 `williechen/copilot_use/main@fd426c68facdfdfd97616cf04a8bbc6badfe02cc`；唯一程式來源 `williechen/rust_ai_ring/main@54f5d80a5678e68e164d56e6d9f11c57cdff1f36`，全程唯讀，由使用者套用本文程式。舊名稱 rust_ai_chat 僅供歷史追溯；資源輸出名稱與外部契約依來源保留。

已重新讀取共同規則、SOURCE_LAYOUT、SYSTEM_ARCHITECTURE、TEACHING_ALIGNMENT、FUTURE_WORK、USER_FLOWS、AUTHENTICATION、UI_ALIGNMENT_REQUIREMENTS、learning-state、manifest、三案 ENTRY 及上一篇。即時 next_day 為 34；文章發布、程式套用、功能驗收分開記錄。

本日功能：在 Leptos 控制窗建立第一隻寵物時，正常中文名稱去除頭尾空白，1–32 個 Unicode scalar values 可儲存；含 NUL、換行、Tab 或 DEL 的名稱明確失敗且不寫入 SQLite。用字數不是 UTF-8 bytes，既有名字不改寫；不是檔案名稱安全或完整字素長度政策。

來源 `pet-application`、`pet-persistence` 的 src/lib.rs 均為空。建立流程已存在：`pet_ui/src/main.rs` 新增按鈕 → `bridge::create_pet` → index.html `createPet` → Tauri `pet_server::create_pet` → `pet_store::PetStore::create` → SQLite pet_owner/pet_profile → 原按鈕 select_pet → SQLite local_pet_selection → pet_windows::open → saved_pet_profile → Leptos 生活窗。本文只抽出並強化「建立名稱」規則，完整接回兩個既有呼叫者；SQLx adapter 仍在 server，不宣稱 persistence 已接通。

Day33 的 window.openPetControls 與其契約測試已在 `pet_apps/pet_ui`；不重教其已套用程式。原生多窗／重啟／失敗注入仍未取得新驗收證據。

## ② Red → 完整最小 Green → Refactor

### Red：規則與真實儲存呼叫者分開測

先在空的 `pet_apps/pet_application/src/lib.rs` 加入以下模組。此階段預期 missing normalize_new_pet_name 編譯失敗：
```rust
#[cfg(test)]
mod day34_name_tests {
    use super::normalize_new_pet_name;

    #[test]
    fn normalized_name_preserves_unicode_and_rejects_controls() {
        assert_eq!(normalize_new_pet_name("  小橘  "), Ok("小橘".into()));
        assert_eq!(normalize_new_pet_name(&"寵".repeat(32)), Ok("寵".repeat(32)));
        for invalid in ["", "  ", "小\0橘", "小\n橘", "小\t橘", "\u{7f}"] {
            assert!(normalize_new_pet_name(invalid).is_err(), "{invalid:?}");
        }
        assert!(normalize_new_pet_name(&"寵".repeat(33)).is_err());
    }
}
```

再將下列完整模組加入 `pet_apps/pet_server/src/pet_store.rs` 末端，保留既有 day22_tests。未套 Green 時現有 normalize 會接受內嵌控制字元，預期 store.create 的 is_err 斷言失敗；不是宣稱已執行 Red。從真實 PetStore 進入，避免只測未被產品呼叫的 helper。
```rust
#[cfg(test)]
mod day34_create_name_tests {
    use super::PetStore;

    #[sqlx::test(migrations = "../migrations")]
    async fn day34_bad_name_never_writes_owner_or_profile(pool: sqlx::SqlitePool) {
        let store = PetStore { pool: pool.clone() };
        let before: (i64, i64) = sqlx::query_as(
            "SELECT (SELECT count(*) FROM pet_owner), (SELECT count(*) FROM pet_profile)"
        ).fetch_one(&pool).await.unwrap();
        for name in ["小\0橘", "小\n橘", "小\t橘", "\u{7f}"] {
            assert!(store.create(name).await.is_err());
        }
        let after: (i64, i64) = sqlx::query_as(
            "SELECT (SELECT count(*) FROM pet_owner), (SELECT count(*) FROM pet_profile)"
        ).fetch_one(&pool).await.unwrap();
        assert_eq!(after, before);
        let created = store.create("  小橘  ").await.unwrap();
        assert_eq!(created.name, "小橘");
        assert_eq!(store.profile(&created.id).await.unwrap(), created);
    }
}
```

### Green A：application 完整檔案

`pet_apps/pet_application/src/lib.rs` 的完整內容為以下函式加上剛才的 day34_name_tests 模組，測試只保留一份：
```rust
/// 建立寵物的輸入規則；不持有 Tauri、SQLx 或 OS driver。
pub fn normalize_new_pet_name(input: &str) -> Result<String, String> {
    // 必須在 trim 前檢查；避免尾端換行被當一般空白悄悄吃掉。
    if input.chars().any(char::is_control) {
        return Err("寵物名稱不得含控制字元".into());
    }
    let name = input.trim();
    if name.is_empty() || name.chars().count() > 32 {
        return Err("寵物名稱須為 1 到 32 字元".into());
    }
    Ok(name.to_owned())
}
```

### Green B：接回既有 Tauri 與 SQLx

在 `pet_apps/pet_server/Cargo.toml` 的 [dependencies] 新增：
```toml
pet-application = { path = "../pet_application" }
```

在 `pet_apps/pet_server/src/lib.rs` 刪除**整個舊的** `fn normalize_new_pet_name(...)` 函式，並在模組最上層 imports 新增：
```rust
use pet_application::normalize_new_pet_name;
```

因此原有 `validate_new_pet_name` command 與既有名稱測試會引用同一實作；原 command 完整保留為：
```rust
#[tauri::command]
fn validate_new_pet_name(name: String) -> Result<String, String> {
    normalize_new_pet_name(&name)
}
```

在 `pet_apps/pet_server/src/pet_store.rs::PetStore::create` 內，只將第一行替換為：
```rust
let name = pet_application::normalize_new_pet_name(input)?;
```

原 UUID、transaction、owner/profile INSERT、commit、讀回與錯誤分支全部保留。規則必須在 begin 前執行；即使繞過 UI 的 validate command，直接呼叫 create_pet 也不能寫入非法名稱。`create_pet` 與 Tauri generate_handler 的註冊不變：
```rust
#[tauri::command]
async fn create_pet(
    name: String,
    store: State<'_, pet_store::PetStore>,
) -> Result<pet_store::SavedPet, String> {
    store.create(&name).await
}
```

UI 的既有 `Err(error) => set_new_pet_feedback.set(format!("建立寵物失敗：{error}"))` 會顯示 application 的錯誤；成功時才走選取／出籠。Leptos 顯示文字沿原元件跳脫，不自行拼 HTML。故不必新增 UI skeleton、SQL schema 或 Tauri command，也不把 DB driver 加到 application。

### Green C：lock 與建置邊界

`pet_apps/Cargo.lock` 的既有 `[[package]] name = "pet-server"`、version 0.2.33 的 dependencies 清單中，在 `"pet-domain"` 前新增 `"pet-application",`，如下局部 diff；其他 lock 內容不變，pet-application package 已存在，不手填 checksum：
```diff
@@ pet-server dependencies
  "dotenv",
+ "pet-application",
  "pet-domain",
```

套用後先在 pet_apps 執行 `cargo metadata --locked --format-version 1` 確認依賴圖；若 Cargo 要更新 lock，先檢查是否漏掉上述唯一新增邊，不使用 cargo update 升級整個圖。application 的既有 serde/serde_json/thiserror/uuid 依賴本日不新增或刪除；未使用的歷史依賴清理不混入切片。Rust import 用 `pet_application`，package 用 `pet-application`，目錄用 `pet_application`。

Pet 採既有 Leptos CSR + Tauri，沒有新增 SSR/hydration metadata；不複製 Chat 的 cargo-leptos 設定。`pet_persistence` 仍空，沒有宣稱它接管 PetStore；其後若需要搬 adapter，須另一個完整 caller/test 切片。

### Refactor

刪除 server 內重複規則，所有建立入口共用同一 application 函式；SQLx transaction 留在原 adapter，純輸入規則可在無 WebKit/GTK 的環境單測。此為必要規則切片，不把建立、選取或原生開窗冒稱純 application 已全面分層完成。

### 上一 Day 未完成事項及舊 Day 更正補充

Day33 路徑 `apps/desktop_pet/frontend` 現為 `pet_apps/pet_ui`，`-p desktop-pet`／`desktop-pet-frontend` 現為 `pet-server`／`pet-ui`；命令工作目錄是 pet_apps。上一篇 JS Bridge 已存在仍須 GUI/DB 失敗注入、同窗聚焦及重啟驗證，未因程式存在結案。Day25 單圖 FIFO pending：本篇不新增動畫骨架、不以建立名稱測試代替三個不同 ID 的原生單圖驗收；原文與雜湊保留。

## ③ 套件與軟體版本

2026-10-25 重新核對官方發布頁／套件原始文件；最新穩定與 lock 分開，未以 nightly 的 docs.rs 建置工具鏈當 MSRV，也不把 RC 當正式版。本日保留來源 lock，不做無關升級。

| 項目 | 宣告／解析或環境 | 官方最新穩定與來源 |
| --- | --- | --- |
| Rust/Cargo | 各 workspace rust-toolchain.toml = stable；未量測安裝版 | [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) |
| Leptos | 0.8 / 0.8.22 | [0.8.22（排除 0.9 RC）](https://github.com/leptos-rs/leptos/releases) |
| SQLx | 0.9 / 0.9.0 | [0.9.0](https://github.com/transact-rs/sqlx/discussions/4271) |
| Tokio | 1.54 / 1.54.1 | [1.54.1](https://github.com/tokio-rs/tokio/releases) |
| Serde | 1.0 + derive / 1.0.229 | [1.0.229](https://docs.rs/crate/serde/latest) |
| UUID | 1.29 + serde,v4,js / 1.29.0 | [1.29.0](https://docs.rs/crate/uuid/latest) |
| Tauri | pet-server 2.12 / 2.12.5 | [2.12.5](https://github.com/tauri-apps/tauri/releases) |
| tauri-build | 2.7 / 2.7.4 | [2.7.4](https://github.com/tauri-apps/tauri/releases) |
| SQLite | libsqlite3-sys 0.37.0；runtime 未量測 | [3.54.1](https://www.sqlite.org/changes.html) |

相容性：各 workspace edition 2024；Tokio 1.54 系列官方 MSRV 1.85，edition 2024 亦至少需要 1.85。本文建議已核對的 stable 1.99.1，未宣稱在每個依賴的最低 MSRV 測試。Leptos 0.8 API 沿用 `view!`／`IntoView`／`.to_html()`；native SSR 與 WASM 各自驗證。SQLx 0.9 既有 runtime-tokio/macros 保留；Chat/Pet 使用 sqlite，Analytics 使用 postgres。實際 SQLite 以 `SELECT sqlite_version()`、PostgreSQL 以 `SHOW server_version` 讀取，未在此環境測量 runtime。CLI 不增安裝步驟；使用隨 Rust toolchain 的 Cargo/rustfmt/clippy。沒有執行付費 AI 或讀取正式秘密。

crates.io API 的瀏覽讀取回報「not accessible via this tool」，已改用上述官方 release／套件文件；該錯誤不當成版本證據。其餘未變更依賴仍由四份原始 lock 固定；完整 manifests/lock blob 證據見本目錄 `source-snapshot.json`。

## ④ 驗證步驟與結果

以下命令從使用者的 `rust_ai_ring` 根目錄進入所屬 workspace，先只套 Red，再套 Green 後重跑。不得在 copilot_use 執行產品建置。

```bash
cd pet_apps
rustc -Vv
cargo -V
cargo metadata --locked --format-version 1 > /tmp/pet-day34-metadata.json
cargo test --locked -p pet-application day34_name_tests
cargo test --locked -p pet-server day34_create_name_tests
cargo test --locked -p pet-server
cargo check --locked -p pet-ui --target wasm32-unknown-unknown
cargo fmt --all -- --check
```

Red／Green 的預期見第②節；本次只交付文件，**未套用產品、未執行上述 Rust 測試／DB／瀏覽器或 GUI**，沒有冒稱 Red 已失敗或 Green 已通過。文件的 JSON、相對連結、精確 commit 全文與雜湊另做交付驗證。

實機流程：原生控制窗輸入中文名稱→建立→清單與生活窗顯示同 ID/name→重啟載入；再以 IPC 注入含 NUL/換行名稱，畫面顯示失敗且 DB owner/profile 數量不增加。成功後故障須沿既有「已儲存勿重送」回饋，不把視窗失敗變成第二次 INSERT。保留其他寵物狀態；Tauri 原生測試須有既有 WebKitGTK/系統 SDK。

CI 來源限定 `54f5d80a5678e68e164d56e6d9f11c57cdff1f36`。[該 SHA Actions](https://github.com/williechen/rust_ai_ring/actions?query=branch%3Amain) 查詢時，connector 的 PR-only workflow_runs 回空，combined statuses 也回空；這不能證明 push workflows 不存在或全部成功。後續已由 Actions REST 核實 push workflows：[Test All](https://github.com/williechen/rust_ai_ring/actions/runs/40101549290) 的 test-analytics-server（Test analytics server）失敗；[Persistence](https://github.com/williechen/rust_ai_ring/actions/runs/40101549347) 的 test-persistence（Test analytics server with PostgreSQL）失敗；Node、四個 workspace metadata、三案 check/build workflows 成功。完整 job/step 與時間見 [來源與 CI 附件](source-snapshot.json) 及 [補充說明](CI_SUPPLEMENT.md)，CI 並非全綠；尚未取得失敗完整日誌，不猜測根因；待修項包含舊報告的 Clippy／migration checksum 及本次切片待執行驗證。不得清空既有資料庫或改舊 migration checksum 迴避問題。

尚待完成：三窗獨立單圖 FIFO、原生 close/hide/聚焦、SQLite 失敗注入、重啟驗收；pet-persistence 尚空。本篇未結案 FIFO、未封存、未更動 last_full_comparison；每日局部核對不等於全面比對。文章 commit 全文讀回通過後才可同步 next_day 34→35；Gate 2 仍未通過。
