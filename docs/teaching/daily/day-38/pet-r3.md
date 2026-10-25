# Pet Day38 r3｜多隻生活窗各自漫遊：逐 UUID 啟動、取代舊任務並停止固定 pet label

> **2026-10-26 教學交付**。唯一程式來源 [rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9](https://github.com/williechen/rust_ai_ring/tree/9b89656f964b99e0cb89ba421855704a35c5ecb9)；產品唯讀，程式由使用者套用。以下 Red／Green／原生 GUI／CI 均**尚未執行**；文件交付不是產品驗收。沿用 [合併後共用規則](../../../COMMON_LEARNING_RULES.md)。

本修訂保留[原版](pet.md)，以當前精確來源提供[完整可套用patch](pet-r2.patch)。核心60分鐘只計②主Red／Green；同篇專案Refactor獨立段落與其餘作業另計。這是同Day修訂，next_day維持39。

從未改過的來源使用完整patch；若先手動加入Red測試，依Green正文補程式，不重複套入已含測試的patch。來源有本地變更先審查差異，不覆寫。完整patch的SHA256：`3272c80e9f32df8edefebee31dff3f5f60c7fa8fadb5dd55183b02fb4ffe56d7`。

規則精確來源：copilot_use `92913e7aae67b317fe25dab75f6ec56721042d35`，共用規則blob `492b50b2e16f1d8997ab55028bfc3c4d072af363`；專案Refactor與②Red／Green並列，功能程式與patch沿用r2精確來源。

## ① 功能、上一篇缺口與正式呼叫鏈

[Day37 r3](../day-37/pet-r3.md) 已在最新來源看到 `open_saved_pet` 的先保存再開窗、失敗補償與生命週期 lock；本日**不重建**那一段。現行 `pet_apps/pet_server/src/lib.rs::application().setup` 只在啟動時 `PET_ENABLE_ROAM` 為 true 就啟一個 `roam_desktop(app)`，而 `pet_apps/pet_server/src/roam.rs::roaming_window_label()` 永遠回 `"pet"`。真實生活窗由 `pet_windows::label` 產生 `pet-{UUID.simple()}`；因此即使重啟載入多隻生活窗，漫遊工作仍找不到窗口。Day18 舊測試還在斷言固定 `"pet"`，應遷移而非持續維護錯誤實作。

**本篇最小功能切片**：只要全域 opt-in `PET_ENABLE_ROAM=true/1`，每次成功建立一隻新的 `pet-{UUID}` 生活窗，便為該窗啟動**唯一**漫遊任務；同 ID 關閉又重開時替換舊 token，舊任務於取消檢查點退出；已在途的位移可能完成，非原子停移；關閉後任務自然退出，不能移動 `main` 控制窗。既有邊界反彈／意外外部位移／停止判斷不改。**逐隻持久化漫遊開關、坐下／拖曳暫停、跨螢幕回程、直接關窗 DB 保存**尚未完成，不能把這次全域 opt-in 當逐寵物政策完成。

真鏈：`pet_ui` Leptos CSR 的 `open_saved_pet` IPC／控制窗選取 → `pet_server::pet_windows::open_saved_pet` → `PetStore`／`pet_persistence` SQLite → `pet_windows::open` 建立 UUID Tauri 原生生活窗 → `roam::spawn_for_pet` → `roam_desktop` 對**同一 label**位移。啟動時 `store.uncaged()` 也經既有 `pet_windows::open`，所以無須第二套 startup 漫遊路徑。UI／DB／Pet Domain 原樣重用，不預建空 Application Port。

## 專案 Refactor

本節在同篇與②Red／Green並列，按Pet整體架構、共用依賴、測試與CI的累積進度每日調整；不是Green完成後才開始的步驟，也不限定於當日功能切片。另計時間，不增加Day。

### 專案目標與累積現況

| 工程責任 | 需銜接的邊界 |
| --- | --- |
| 生命週期用例 | 建立／出籠／回籠／選取／重啟載入，Persistence與Window Port、補償和相容錯誤 |
| 儲存與狀態 | SQLite Adapter與Row／DTO、Domain狀態機、Runtime／逐ID鎖與交易界線 |
| 原生與UI | Tauri IPC／視窗／漫遊／資源Adapter，Leptos CSR、原生產物與正式組裝 |
| 測試與原生CI | Application fake Port、SQLite真Adapter、正式IPC及native／GUI分層；保留所有有效故障與並行斷言 |
| 共用依賴／組裝 | 檢視四workspace公開契約、shared、Cargo normal／build／dev／target依賴、features與正式caller；先承接再清舊碼 |

先完成專案caller／Port／Adapter與斷言基線，再依相依選完整包。新功能與未完成需求依FUTURE_WORK另追蹤，不能把表內規劃寫成已存在或已重構。

### 既有測試遷移基線（專案清單的一部分）

| 舊測試／群組 | 判斷 | 處理與必留斷言 | 目標責任 |
| --- | --- | --- | --- |
| persist_failure_never_opens_or_closes_a_window | 流程契約 | 用例抽出後遷Application，保留先寫DB失敗不碰視窗 | Application fake Port |
| open_failure_restores_the_actual_old_flag | 補償契約 | 保留old_caged=true／false兩情境，不只測固定true | Application fake Port |
| success_writes_once_and_rollback_error_is_visible | 成功與補償再失敗兩責任 | 拆為具名情境；保留寫入次數與兩個錯誤均可見 | Application fake Port |
| lifecycle_lock_excludes_a_second_operation | 並行保護 | 保留真共用鎖接線，不能只把鎖測到用例fake；正式IPC併行另驗 | 組裝／server整合 |
| only_target_pet_changes_and_missing_id_fails + read_flag_and_fail_closed_for_missing_pet | 真SQL不同操作契約 | 保留逐ID隔離、missing、真DB錯誤；不是fake補償測試的重複 | pet-persistence SQLx Adapter |

來源 `rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9`。此表是來源核對後的擬議映射，非已遷移清單；完整跨案對照見 [測試覆蓋表](test-coverage-map.csv)。同層同契約才整併；跨層保留接線、SQL、HTTP／IPC或DOM獨有的驗證責任。fixture抽共用準備，不複製業務實作或將情境斷言藏入共用helper。



### 本次完整調整：CI責任隔離

原Persistence workflow將Chat→Pet server/native→Analytics PG串在同一job，前段故障會使後段不執行。pet-sqlite只選pet-persistence而不帶Tauri；pet-native保留pet-server與原測試，安裝必要Linux依賴。server測試不再冒充純SQLite證據，原生編譯仍不是GUI驗收。

下面是取代產品來源 `.github/workflows/persistence.yml` 的完整YAML，三案共用同一份內容及證據。本r3把先前已交付的方案納入同篇獨立段落，不重複計為新增工程成果；其餘Application／Domain／Adapter的完整重構仍列待辦。

```yaml
name: Persistence
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
permissions:
  contents: read
concurrency:
  group: persistence-${{ github.ref }}
  cancel-in-progress: true
env:
  CARGO_TERM_COLOR: always
jobs:
  chat-sqlite:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Chat SQLite adapter tests
        working-directory: chat_apps
        env:
          DATABASE_URL: "sqlite::memory:"
        run: cargo test --locked -p chat-persistence

  pet-sqlite:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Pet SQLite adapter tests (no Tauri)
        working-directory: pet_apps
        env:
          DATABASE_URL: "sqlite::memory:"
        run: cargo test --locked -p pet-persistence

  pet-native:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Tauri native dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y build-essential pkg-config curl wget file \
            libssl-dev libxdo-dev librsvg2-dev libwebkit2gtk-4.1-dev \
            libayatana-appindicator3-dev
      - uses: dtolnay/rust-toolchain@stable
      - name: Pet native/server tests (not GUI acceptance)
        working-directory: pet_apps
        env:
          DATABASE_URL: "sqlite::memory:"
        run: cargo test --locked -p pet-server

  analytics-postgres:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:18
        env:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: analytics_ci
        ports: ["5432:5432"]
        options: >-
          --health-cmd "pg_isready -U postgres -d analytics_ci"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Analytics SQLx adapter + real Router integration
        working-directory: analytics_apps
        env:
          DATABASE_URL: postgres://postgres:postgres@localhost:5432/analytics_ci
          ANALYTICS_DATABASE_URL: postgres://postgres:postgres@localhost:5432/analytics_ci
          ANALYTICS_CHAT_TOKEN: local-ci-chat-token
          ANALYTICS_PET_TOKEN: local-ci-pet-token
        run: cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration
```

### 測試／CI承接與套用條件

| 原選取 | 新責任入口 | 必留範圍 |
| --- | --- | --- |
| chat-persistence | chat-sqlite | SQLite Adapter原斷言全部保留 |
| pet-persistence | pet-sqlite | SQLite Adapter原斷言全部保留，不帶Tauri |
| pet-server | pet-native | 原server／IPC與native測試保留，GUI另驗 |
| analytics-server＋analytics-persistence | analytics-postgres | postgres-integration、真PG與正式Router／資料層測試 |

四job無互相needs、無continue-on-error，不刪測試換全綠。若required checks仍指定舊test-persistence，使用者同步調整新job名稱；本次未改分支規則。PG18是擬議服務major，實際minor及DB結果須套用後讀回。其它workflow保留；純內層／HTTP／WS／IPC／UI覆蓋仍須完整基線核對，不能從此workflow推論全專案覆蓋完整。

### 實際驗證、回復與下一步

方案來源為 `9b89656f964b99e0cb89ba421855704a35c5ecb9`。原方案已通過精確來源的patch套用檢查與YAML解析；產品未套用，Actions、DB、native與GUI未執行。本r3未新增執行或套用結果。

使用者套用後，以隔離分支的Pet native可控失敗確認其餘job仍執行且整體失敗，恢復後重跑全部原斷言。回復時還原該workflow及相應required checks，不回退其它產品變更。

下一步補齊生命週期、Runtime／鎖、SQLite、Tauri／視窗／漫遊、CSR與產物的caller、Port／Adapter、有效斷言及CI選取基線，再依真相依提供完整用例與測試遷移；前案未套用不得假設已存在。這些未完成項不能由本篇Green或CI方案代替，也不新增FIFO或更新last_full_comparison。

[原完整差異patch](../../../engineering/2026-10-26/ci-isolation.patch) · [原驗證設計](../../../engineering/2026-10-26/ci-isolation.md) · [專案盤點](../../functional-refactoring/pet.md) · [跨日累積進度](../../functional-refactoring/PROGRESS.md) · [r2原文](pet-r2.md)。

## ② 功能實作：Red → 完整 Green

### Red：取代固定 label 的錯誤斷言，驗證每 ID 任務隔離

在 `pet_apps/pet_server/src/roam.rs` 的現有 `#[cfg(test)] mod tests` 中，**刪除只驗證舊錯誤契約**的 `day18_roaming_selects_pet_life_window`，以以下兩個完整測試替換；其它位移／邊界測試保留。新增的 `register_task`／`finish_task` 尚不存在，預期先為缺符號編譯 Red；舊 `"pet"` 斷言本來通過，不可宣稱它是新 Red。兩測試使用獨立 UUID 避免並行污染。

```rust
    #[test]
    fn day38_window_label_is_per_pet_not_control_or_fixed_pet() {
        let id = uuid::Uuid::new_v4().to_string();
        let label = crate::pet_windows::label(&id).unwrap();
        assert_eq!(label, format!("pet-{}", uuid::Uuid::parse_str(&id).unwrap().simple()));
        assert_ne!(label, "pet");
        assert_ne!(label, "main");
    }

    #[test]
    fn day38_reopen_cancels_only_previous_task_for_same_label() {
        use std::sync::atomic::Ordering;
        let a = format!("pet-{}", uuid::Uuid::new_v4().simple());
        let b = format!("pet-{}", uuid::Uuid::new_v4().simple());
        let first = register_task(&a).unwrap();
        let other = register_task(&b).unwrap();
        let second = register_task(&a).unwrap();
        assert!(!first.load(Ordering::Acquire));
        assert!(second.load(Ordering::Acquire));
        assert!(other.load(Ordering::Acquire));
        finish_task(&a, &first);
        assert!(roam_tasks().lock().unwrap().contains_key(&a));
        finish_task(&a, &second);
        finish_task(&b, &other);
        assert!(!roam_tasks().lock().unwrap().contains_key(&a));
        assert!(!roam_tasks().lock().unwrap().contains_key(&b));
    }
```

### Green A：在原生 Adapter 維持每個 label 的可取消任務

`pet_apps/pet_server/src/roam.rs`：保留 `use tauri::{Manager, PhysicalPosition, PhysicalRect, PhysicalSize};` 及所有 `bounded_next_position` 等既有函式。**刪除**原本 `PET_WINDOW_LABEL` 常數與 `roaming_window_label()` 函式，將下列 imports／狀態／函式加在檔案開頭 imports 之後：

```rust
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
};

fn roam_tasks() -> &'static Mutex<HashMap<String, Arc<AtomicBool>>> {
    static TASKS: OnceLock<Mutex<HashMap<String, Arc<AtomicBool>>>> = OnceLock::new();
    TASKS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn register_task(label: &str) -> Option<Arc<AtomicBool>> {
    let mut tasks = roam_tasks().lock().ok()?;
    let active = Arc::new(AtomicBool::new(true));
    if let Some(previous) = tasks.insert(label.to_owned(), active.clone()) {
        previous.store(false, Ordering::Release);
    }
    Some(active)
}

fn finish_task(label: &str, active: &Arc<AtomicBool>) {
    if let Ok(mut tasks) = roam_tasks().lock() {
        if tasks.get(label).is_some_and(|current| Arc::ptr_eq(current, active)) {
            tasks.remove(label);
        }
    }
}

pub(crate) fn spawn_for_pet(app: tauri::AppHandle, label: String) {
    if !crate::roam_enabled() {
        return;
    }
    let Some(active) = register_task(&label) else {
        eprintln!("desktop roaming unavailable: task registry poisoned");
        return;
    };
    tauri::async_runtime::spawn(async move {
        roam_desktop(app, &label, &active).await;
        finish_task(&label, &active);
    });
}
```

同檔**完整取代**原 `pub async fn roam_desktop(app: tauri::AppHandle)` 的**完整函式**：

```rust
async fn roam_desktop(app: tauri::AppHandle, label: &str, active: &AtomicBool) {
    let mut dx = ROAM_INITIAL_DX;
    let mut dy = ROAM_INITIAL_DY;
    let mut stalled_attempts = 0_u8;

    loop {
        if !active.load(Ordering::Acquire) {
            break;
        }
        let window = match app.get_webview_window(label) {
            Some(window) => window,
            None => {
                eprintln!("desktop roaming unavailable: pet window not found: {label}");
                break;
            }
        };
                let before = match window.outer_position() {
            Ok(position) => position,
            Err(error) => { eprintln!("desktop roaming unavailable: {error}"); break; }
        };
        let window_size = match window.outer_size() {
            Ok(size) => size,
            Err(error) => { eprintln!("desktop roaming unavailable: {error}"); break; }
        };
        let monitor = match window.current_monitor() {
            Ok(Some(monitor)) => monitor,
            Ok(None) => { eprintln!("desktop roaming unavailable: no monitor found"); break; }
            Err(error) => { eprintln!("desktop roaming unavailable: {error}"); break; }
        };
        let requested =
            bounded_next_position(before, window_size, monitor.work_area(), &mut dx, &mut dy);
        // 取消後不可再送出新的位移；不持有 registry Mutex 跨 await。
        if !active.load(Ordering::Acquire) { break; }
        if let Err(error) = window.set_position(requested) {
            eprintln!("desktop roaming unavailable: {error}");
            break;
        }
        if no_roaming_space(dx, dy) {
            eprintln!("desktop roaming stopped: pet window exceeds available work area");
            break;
        }
        tokio::time::sleep(ROAM_VERIFY_INTERVAL).await;
        if !active.load(Ordering::Acquire) { break; }
        let after = match window.outer_position() {
            Ok(position) => position,
            Err(error) => {
                eprintln!("desktop roaming verification unavailable: {error}");
                break;
            }
        };
        if unexpected_displacement(requested, after) {
            eprintln!("desktop roaming stopped: unexpected window displacement");
            break;
        }
        if after == before {
            stalled_attempts += 1;
            if stalled_attempts >= ROAM_STALL_LIMIT {
                eprintln!(
                    "desktop roaming unavailable: window position did not change after                      {ROAM_STALL_LIMIT} attempts"
                );
                break;
            }
        } else {
            stalled_attempts = 0;
        }
        tokio::time::sleep(ROAM_INTERVAL).await;
    }
}
```

**注意：** `register_task`／`finish_task` 不複製任何業務狀態到 Domain；這是 Tauri 原生移動 task 的 OS adapter lifecycle。每個 label 的 token 只在任務結束時移除；舊任務結束時用 `Arc::ptr_eq` 防止清掉重開後的新 token。被取消的 task 可能已送出當下位移，因此本切片不承諾同步原子停移；真正拖曳暫停另設互動協定。

### Green B：接回真實建立／啟動呼叫端，移除舊全域入口

`pet_apps/pet_server/src/pet_windows.rs`：原 `fn label(id: &str) -> Result<String,String>` 改成 `pub(crate) fn label(...)`，**函式本體完全保留**。在 `pub fn open(app,pet)` 最後這段：

```rust
    if let Err(e) = result {
        machines.remove(&pet.id);
        return Err(e.to_string());
    }
    Ok(())
```

完整替換成：

```rust
    if let Err(e) = result {
        machines.remove(&pet.id);
        return Err(e.to_string());
    }
    // 僅真正建立新生活窗後啟動；既有窗 show/focus 早退不重複 spawn。
    crate::roam::spawn_for_pet(app.clone(), name);
    Ok(())
```

`pet_apps/pet_server/src/lib.rs`：將 `use crate::{chat::AiState, roam::roam_desktop};` 改成 `use crate::chat::AiState;`。在 `application().setup` 尾端**完整刪除**舊區塊：

```rust
            if roam_enabled() {
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    roam_desktop(app_handle).await;
                });
            }
```

`roam_enabled`／`roam_enabled_value` 保留供新 `spawn_for_pet` 與原測試使用。現在 startup 的 `store.uncaged() → pet_windows::open` 與手動 `open_saved_pet → pet_windows::open` 都使用同一任務入口；`select_pet` 的 `open` 亦不會為既有窗重啟任務。保留 `pet_windows` 既有生命週期 lock、SQLx、Runtime、200×200 Tauri 設定、IPC 註冊與 Leptos CSR 的真實 caller。不要移動 DB adapter 來湊齊目錄。

### Green 回歸檢查：測試與依賴

| 舊入口與測試 | 本次處理 | 有效斷言去向 |
| --- | --- | --- |
| `roam::tests::day18_roaming_selects_pet_life_window` | 確認舊固定 label 契約已失效，替換 | Day38 UUID label 測試，不能仍期待 `"pet"` |
| `roam::tests` 其餘位移／工作區測試 | 全部保留 | 越界、單軸、外部拖移、停滯與極值 |
| `roam_setting_tests` | 保留 | `PET_ENABLE_ROAM` opt-in，不誤認已持久逐寵物 |
| 新任務註冊／取代測試 | 新增 | 相同 ID 重開取消舊任務、不同 ID 隔離、清理不誤刪 |
| 原生三窗與重啟 GUI | 必須另驗；未執行 | 真實 Tauri 生活窗可移動、控制窗不移動、回籠停止 |

四 workspace Cargo.toml/Cargo.lock 已核對；只改 `pet_apps/pet_server/src/{roam,pet_windows,lib}.rs`，Cargo package `pet-server`、crate `pet_server`，`pet-persistence` 已有真 caller，`pet-application` 名稱規則已接通，無需修改。現有 `tauri=2.12`、`tokio=1.54/full`、`uuid=1.29/v4` 與 `std::sync` 足夠，**不新增 Cargo normal/build/dev/target dependencies、features、imports 以外的檔案、migrations、Leptos metadata、CI workflow**。Pet UI `pet-ui` 仍是 Leptos CSR + Tauri；其 Rust imports 不變。

## ③ 當日官方穩定版本與採用判斷

2026-10-26 台北日期，crates/Rust於 `2026-10-25T23:01:56.157211+00:00` 重查；排除預發行與 yanked。[完整宣告／lock／官方網址／原始回應SHA](r2-versions.json)。來源9b與648外部宣告及resolved一致；9b內部package為0.2.37。

| 套件／工具 | manifest 宣告 | lock resolved／已安裝 | 官方穩定版／MSRV |
| --- | --- | --- | --- |
| tauri | `見package/target表` | 2.12.5 | [2.12.5](https://crates.io/crates/tauri/2.12.5)／1.90 |
| tauri-build | `見package/target表` | 2.7.4 | [2.7.4](https://crates.io/crates/tauri-build/2.7.4)／1.90 |
| leptos | `0.8` | 0.8.22 | [0.8.22](https://crates.io/crates/leptos/0.8.22)／1.88 |
| tokio | `1.54` | 1.54.1 | [1.54.1](https://crates.io/crates/tokio/1.54.1)／1.85 |
| uuid | `1.29` | 1.29.0 | [1.29.0](https://crates.io/crates/uuid/1.29.0)／1.89.0 |
| sqlx | `0.9` | 0.9.0 | [0.9.0](https://crates.io/crates/sqlx/0.9.0)／1.94.0 |
| serde | `1.0` | 1.0.229 | [1.0.229](https://crates.io/crates/serde/1.0.229)／1.56 |
| serde_json | `1.0` | 1.0.154 | [1.0.154](https://crates.io/crates/serde_json/1.0.154)／1.71 |

Rust官方stable 1.99.1，[官方channel](https://static.rust-lang.org/dist/channel-rust-stable.toml)，本機rustc/cargo同版。最高所列MSRV不高於本機Rust，但不等於SSR／WASM／native／DB建置通過。相符resolved維持；若有差異優先以最新穩定版驗證遷移，本包不改lock、安裝工具或升級。

工具／native：Tauri CLI已安裝2.12.3，官方2.12.4（MSRV1.90）；Trunk已安裝0.21.14，官方0.22.0（MSRV1.96.1），建議各自測試升級後固定相容版本，本包未升級。Linux WebKitGTK官方2.54.2／本機2.52.6；GTK3官方3.24.52；GLib官方2.90.1／本機2.88。SQLite官方3.54.1，CLI未裝；libsqlite3-sys resolved0.37.0對官方0.39.1，不能假稱bundled引擎已更新。
[Tauri官方Linux prerequisites](https://v2.tauri.app/start/prerequisites/) · [WebKitGTK](https://webkitgtk.org/) · [GTK3官方索引](https://download.gnome.org/sources/gtk/3.24/) · [GLib](https://download.gnome.org/sources/glib/)；須另驗WebKit/CSR資源、原生視窗、Wayland與X11能力，套件新版不保證視窗可程式移動。

資料庫／native當日官方來源、已安裝探測與採用理由見[版本詳細核對](../../../source-review/2026-10-26-versions-and-summary-refresh.md)，不能以metadata宣稱功能已驗收。
## ④ 驗證命令、預期／實際與上一 Day 更正

```bash
cd rust_ai_ring/pet_apps
# 原生開發套件須先依 Tauri v2 官方 prerequisite 安裝。
cargo test --locked -p pet-server roam::tests
cargo test --locked -p pet-server
cargo test --locked -p pet-persistence
cargo check --locked -p pet-ui --target wasm32-unknown-unknown
# 另用 Tauri 真實桌面：PET_ENABLE_ROAM=1；建立／出籠三隻，
# 檢查每個 pet-{UUID} 自己位移、main 不移動、重複開窗不重複 task、
# 回籠／重開／重啟與手動拖曳情況，觀察原生 log。
```

| Gate | 預期 | 本次實際 |
| --- | --- | --- |
| Red | 新函式缺符號；舊固定 `pet` label 找不到真生活窗 | 未執行；後者為已知來源缺口 |
| Green／原位移回歸 | Day38 task 測試與原工作區測試皆通過 | 未執行 |
| Pet SQLite／CSR | 原有 persistence 與 Leptos UI 不回歸 | 未執行 |
| 原生三窗 GUI | 各窗位移、取消與重開、控制窗不位移 | 未執行 |
| CI／Gate2 | 套用後重跑相關原生與UI驗證 | 未套用、未驗收 |

**上一 Day 補充：** Day37 先保存再開窗與補償已在本次來源；不把其 closure 測試當真 native fault injection。舊 Day18 固定 label 斷言是過時測試，只有在正式 caller 改成 UUID 後才移除；其餘斷言全保留。後續逐寵物持久漫遊、直接關窗 DB 同步與 UI 控制仍屬獨立切片。本篇不修改 rust_ai_ring、舊文章、`last_full_comparison`。

沿用r2的靜態驗證：在相同精確來源快照，`git apply --check`通過；完整修改檔以rustfmt解析通過。本r3重組正文，未重跑以上檢查或任何產品測試。[逐檔來源blob與結果](r2-static-checks.json)。這兩項不代表編譯、Red失敗、Green通過、DB、native、GUI或CI；上述仍未執行。產品倉庫未套用本候選內容。
