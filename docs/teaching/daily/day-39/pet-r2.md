# Pet Day39 r2｜控制窗逐 UUID「找回寵物」到主螢幕可用區

> **2026-10-26 r2 修訂**：Refactor 正文提供逐檔程式、完整測試與斷言承接；本地驗證僅對精確8fc來源＋三案Refactor，未套本篇功能Green。產品checkout保持唯讀。原[首版](pet.md)與[r1](pet-r1.md)保留。

共用規則精確commit `0ccd1a252b3e8fc28150242944b36929760b3cd3`，blob `ba9e6cbd6763d245e60800a02c60888ba6215bd5`。

產品來源 `williechen/rust_ai_ring/main@8fc4b277b6f6f77d97655130036fca47813de56e`；只發布 `williechen/copilot_use/main` 文件。候選Refactor的實測結果列在獨立段落，②功能Green的驗證狀態另列；兩者不互相替代。

## ① 功能目標與真實呼叫鏈

需求明確要求控制窗能找回跑出可視區的某隻生活窗，不改其漫遊設定、不移動控制窗、不影響其他 UUID。現有 `pet_server::roam` 已逐 label 漫遊，`pet_windows::label` 用 `pet-<UUID.simple()>`，但 `pet_ui/src/main.rs` 的多寵物清單沒有找回按鈕。新增每隻的「找回寵物」：Leptos `For` → `bridge::find_saved_pet` → `index.html::findSavedPet` → Tauri `find_saved_pet` → SQLite `PetStore::profile` 存在檢查 → 指定原生生活窗 → 主螢幕 work_area 置中 → show/focus。既有出入籠、選取、SQLite 資料、主控制窗不重建。

核心60分鐘只計② Red／Green，Refactor／版本／驗證／文件提交另計。此功能不是「自動跑回」的全能力，也不處理螢幕拔除策略與拖曳恢復競態；原需求仍待驗。

## Refactor

### 1. 分層問題與本次邊界

Pet 的開窗補償已在 `pet_application::lifecycle::persist_then_open`，回籠補償卻仍寫在 Tauri command 內。資料寫入順序、關窗失敗後恢復標記及雙重失敗訊息是用例責任；如何取得原生視窗與執行 SQLite 則仍由 server Adapter 提供。本次把回籠流程抽成同模組的 `persist_then_close`，使開／關生命週期規則有一致邊界。這項重構不依賴②找回寵物功能。

| 責任 | 修改前 | 修改後 |
| --- | --- | --- |
| 保存→關窗→失敗補償 | command 內嵌 | Application 函式，透過兩個 closure 呼叫外部能力 |
| 身分／profile、UUID label、Tauri window | server | 保留 server，不讓 Application 接觸 Tauri |
| SQLite `set_caged` | server 提供 store | server 以 closure 傳入，Application 不持有 SQLx Pool |
| lifecycle guard、成功後 Runtime remove | server | 原順序保留，Application 失敗直接 `?` 返回 |

### 2. Application：完整新增函式

在 `pet_apps/pet_application/src/lifecycle.rs`，放在既有 `persist_then_open` 後、測試模組前。既有 `pub mod lifecycle;` 已在 `pet_application/src/lib.rs`，不用新增 crate 或依賴。

```rust
pub async fn persist_then_close<W, F, C>(mut write: W, close_window: C) -> Result<(), String>
where
    W: FnMut(bool) -> F,
    F: std::future::Future<Output = Result<(), String>>,
    C: FnOnce() -> Result<(), String>,
{
    write(true).await?;
    if let Err(error) = close_window() {
        return match write(false).await {
            Ok(()) => Err(format!("關閉寵物視窗失敗，已恢復出籠狀態：{error}")),
            Err(rollback) => Err(format!(
                "關閉寵物視窗失敗：{error}；回復資料庫亦失敗：{rollback}"
            )),
        };
    }
    Ok(())
}
```

第一次 `write(true)` 失敗會直接返回，不能呼叫 close。關窗失敗才執行 `write(false)` 補償；補償失敗保留原生與資料庫兩個錯誤。這裡忠實保留原 command 的回復 false 語義，沒有悄悄改成其他持久化政策；直接關窗、預設資格與現行 caged 欄位的分工仍待後續處理。

### 3. Server：替換後完整 command

以此完整函式替換 `pet_apps/pet_server/src/pet_windows.rs::cage_saved_pet`；不要另增第二個 command。`pet-application` 已是 `pet-server` 直接依賴，使用完整 crate 路徑呼叫，原 `persist_then_open` import 保留。

```rust
#[tauri::command]
pub async fn cage_saved_pet(
    id: String,
    app: tauri::AppHandle,
    store: State<'_, PetStore>,
) -> Result<(), String> {
    let _operation = lifecycle_guard().await;
    store.profile(&id).await?;
    let window = app.get_webview_window(&label(&id)?);
    pet_application::lifecycle::persist_then_close(
        |flag| store.set_caged(&id, flag),
        || match window {
            Some(window) => window.close().map_err(|e| e.to_string()),
            None => Ok(()),
        },
    )
    .await?;
    app.state::<Runtime>().remove(&id);
    Ok(())
}
```

`lifecycle_guard` 涵蓋整段 await；找不到 profile 或 UUID 不合法時不執行寫入；沒有現存生活窗時，close closure 回成功，仍保存回籠並清除 Runtime。只有保存與關窗全部成功，才抵達 Runtime remove。Tauri Destroyed 事件可能另外清理 Runtime 的既有行為保持原狀；純測試不能證明真視窗事件時序。

### 4. Application：完整新增測試

下列四個測試全部放入既有 `lifecycle.rs::tests`，沿用該模組的 `Cell`、`RefCell`、`ready` 與 `complete_ready`。`complete_ready` 只支援 ready fixture，遇 Pending 會明確失敗；它不是產品 executor。舊開窗測試全部保留。

```rust
#[test]
    fn caging_does_not_close_when_db_write_fails() {
        complete_ready(async {
            let closed = Cell::new(false);
            let writes = RefCell::new(Vec::new());
            let result = persist_then_close(
                |flag| {
                    writes.borrow_mut().push(flag);
                    ready(Err("db".into()))
                },
                || {
                    closed.set(true);
                    Ok(())
                },
            )
            .await;
            assert!(result.is_err());
            assert!(!closed.get());
            assert_eq!(*writes.borrow(), vec![true]);
        });
    }

    #[test]
    fn native_close_failure_restores_uncaged_or_reports_rollback_failure() {
        complete_ready(async {
            for fail_rollback in [false, true] {
                let writes = RefCell::new(Vec::new());
                let result = persist_then_close(
                    |flag| {
                        writes.borrow_mut().push(flag);
                        ready(if !flag && fail_rollback {
                            Err("rollback".into())
                        } else {
                            Ok(())
                        })
                    },
                    || Err("native".into()),
                )
                .await
                .unwrap_err();
                assert_eq!(*writes.borrow(), vec![true, false]);
                assert!(result.contains("native"));
                assert_eq!(result.contains("rollback"), fail_rollback);
            }
        });
    }

    #[test]
    fn successful_cage_writes_once_then_closes_once() {
        complete_ready(async {
            let closed = Cell::new(0);
            let writes = RefCell::new(Vec::new());
            persist_then_close(
                |flag| {
                    writes.borrow_mut().push(flag);
                    ready(Ok(()))
                },
                || {
                    closed.set(closed.get() + 1);
                    Ok(())
                },
            )
            .await
            .unwrap();
            assert_eq!(*writes.borrow(), vec![true]);
            assert_eq!(closed.get(), 1);
        });
    }

    #[test]
    fn close_runs_after_persist_and_rollback_runs_after_failed_close() {
        complete_ready(async {
            let events = RefCell::new(Vec::new());
            let error = persist_then_close(
                |flag| {
                    events.borrow_mut().push(if flag {
                        "persist-caged"
                    } else {
                        "restore-uncaged"
                    });
                    ready(Ok(()))
                },
                || {
                    events.borrow_mut().push("close");
                    Err("native".into())
                },
            )
            .await
            .unwrap_err();
            assert_eq!(
                *events.borrow(),
                vec!["persist-caged", "close", "restore-uncaged"]
            );
            assert_eq!(error, "關閉寵物視窗失敗，已恢復出籠狀態：native");
        });
    }
```

### 5. 斷言與原生邊界對照

| 邊界／故障 | 本次測試或保留位置 | 能證明的事 |
| --- | --- | --- |
| 首次 DB 寫入失敗 | `caging_does_not_close_when_db_write_fails` | 只嘗試 true 一次，close 未呼叫 |
| close 失敗，補償成功／失敗 | `native_close_failure_restores_uncaged_or_reports_rollback_failure` | 寫入 `[true,false]`、原生錯誤必在、回復錯誤不得吞掉 |
| 正常回籠 | `successful_cage_writes_once_then_closes_once` | 一次保存、一次 close |
| 順序與原錯誤文字 | `close_runs_after_persist_and_rollback_runs_after_failed_close` | 保存先於close、補償在後，精確錯誤字串維持 |
| 逐 ID／缺 ID／SQLite 失敗 | 原 `pet_persistence::day35_cage_adapter_tests` | 仍由真 SQLite Adapter 測試；不搬到 fake |
| Runtime 隔離與真 lifecycle mutex | 原 `pet_windows::tests`／`lifecycle_lock_tests` | 不刪有效斷言，不能由 Application 測試代替 |
| 視窗存在／不存在、Destroyed、重入與重開 | Tauri 真 GUI | 仍需手動原生驗證，不能因 closure 測試通過宣稱已驗收 |

### 6. 完整 patch、驗證與回復

[完整 Refactor patch](pet-day39-refactor-r2.patch) 包含上面函式、server 接線與全部測試；無 Cargo／feature／lock／migration 變更。與②找回功能 patch 前後兩種套用順序都已實際檢查。

```bash
git apply --check /path/to/pet-day39-refactor-r2.patch
git apply /path/to/pet-day39-refactor-r2.patch
cd pet_apps
cargo test --offline --locked -p pet-application
# 原生／資料層的後續驗證入口，不等於已執行結果
cargo test --offline --locked -p pet-server -p pet-persistence
```

**本次實測**：隔離精確8fc來源＋Refactor；`cargo test --offline --locked -p pet-application` exit 0，8項通過、0項失敗、0項ignored；另跑`pet-persistence`真SQLite 4項通過。[SQLite原始log](r2-validation/pet-sqlite.log)。[原始log](r2-validation/pet-application.log)；此結果不包含未套用的Day39功能Green。

若本地有同區域修改，先三方比對；乾淨完整候選的回復先 `git apply -R --check`，再 `git apply -R`。下一步處理直接關窗、資料庫標記與 Runtime 的責任；需要另外定義持久化策略，不在本次以抽函式偷偷改行為。全庫 CI 仍屬全專案獨立項目。

## ② 功能實作：Red → 完整 Green

### Red｜主螢幕置中及多寵物隔離

先於 `pet_apps/pet_server/src/pet_windows.rs::tests` 加入下列測試；缺少 `centered_position` 時屬**缺符號編譯 Red**，並非已跑失敗。另在原生三窗環境測「找回寵物」按鈕，目前 UI 不存在（尚未實測）。

```rust
#[test]
fn day39_find_position_respects_primary_work_area_and_negative_origins() {
    let area = PhysicalRect {
        position: PhysicalPosition { x: -1920, y: -100 },
        size: PhysicalSize { width: 1920, height: 1080 },
    };
    assert_eq!(centered_position(&area, PhysicalSize { width: 200, height: 200 }),
        PhysicalPosition { x: -1060, y: 340 });
    assert_eq!(centered_position(&area, PhysicalSize { width: 2500, height: 1200 }),
        PhysicalPosition { x: -1920, y: -100 });
}
```

### Green｜真實 UI→IPC→native caller，完整 patch

[pet-day39-green-r2.patch](pet-day39-green-r2.patch) 含 **六個完整檔案變更**：`pet_server/src/pet_windows.rs`（置中演算法＋真 Tauri command＋測試）、`pet_server/src/roam.rs`（只取消目標 UUID 任務＋測試）、`pet_server/src/lib.rs`（generate_handler）、`pet_ui/index.html`（JS invoke）、`pet_ui/src/bridge.rs`（wasm_bindgen import＋async wrapper）、`pet_ui/src/main.rs`（逐 ID 找回按鈕、loading、結果回饋）。程式路徑均位於 `pet_apps`；Cargo package `pet-server`、`pet-ui`，Rust crate `pet_server`、`pet_ui`，無新增依賴或 features。

```rust
#[tauri::command]
pub async fn find_saved_pet(id: String, app: tauri::AppHandle,
    store: State<'_, PetStore>) -> Result<(), String> {
    let _operation = lifecycle_guard().await;
    store.profile(&id).await?;
    let name = label(&id)?;
    let window = app.get_webview_window(&name).ok_or("寵物尚未出籠")?;
    let monitor = app.primary_monitor().map_err(|e| e.to_string())?.ok_or("找不到主螢幕")?;
    let size = window.outer_size().map_err(|e| e.to_string())?;
    let position = centered_position(monitor.work_area(), size);
    crate::roam::stop_for_pet(&name);
    let result = window.set_position(position).and_then(|_| window.show()).and_then(|_| window.set_focus());
    crate::roam::spawn_for_pet(app.clone(), name);
    result.map_err(|e| format!("找回寵物失敗：{e}"))
}
```

只操作現存生活窗，未出籠拒絕，不偷偷開新窗；移動前停止該 label 的 roam task，移動後按原設定重新啟動（現行來源仍由全域 `PET_ENABLE_ROAM` 決定，**逐寵物漫遊持久化尚未完成**，不得冒稱滿足完整政策）。主螢幕取得失敗時保留原窗，不動資料。需真 GUI 驗證舊任務取消與新任務競態。

```bash
# rust_ai_ring 根目錄，僅在使用者自己的工作樹
 git apply --check /path/to/pet-day39-refactor-r2.patch
 git apply /path/to/pet-day39-refactor-r2.patch
 git apply --check /path/to/pet-day39-green-r2.patch
 git apply /path/to/pet-day39-green-r2.patch
 cd pet_apps
 cargo test --locked -p pet-application
 cargo test --locked -p pet-persistence
 cargo test --locked -p pet-server day39
 cargo check --locked -p pet-server
 cargo check --locked -p pet-ui --target wasm32-unknown-unknown
 node --test pet_ui/tests/*.test.mjs
```

### 有效斷言、手動驗收與下一步

Day24/35/36/37/38 的逐 ID 狀態、關窗補償、恢復清單、roam task registry 斷言全保留。原生三窗驗證：A、B、C 同時出籠，移 A 到副螢幕外後只找回 A，B/C 位置及狀態不變；重複找回不開新窗；回籠 A 後找回報錯；控制窗不移動；禁用漫遊時找回不重啟；主螢幕不存在時錯誤可見；負座標及高 DPI 正確。直接關窗尚未持久化，需下一 Day 獨立功能切片處理。

## ③ 2026-10-26 官方穩定版核對

| 工具 | 來源 manifest／lock | 官方穩定版與日期 | 相容處置 |
| --- | --- | --- | --- |
| Rust | edition 2024，執行環境未讀 | 1.99.1（10/15） | 機器上另驗 rustc、native/WASM |
| Tauri | `2.12`／`2.12.5` | 2.12.5（10/21）；3 alpha 不採 | API `primary_monitor`、`set_position` 官方文件核對；Linux WebKit 另驗 |
| Leptos | `0.8`／`0.8.22` | 0.8.22（10/05） | CSR 不改 |
| SQLx | `0.9`／`0.9.0` | 0.9.0（05/21） | SQLite schema 未改 |
| Tokio | `1.54`／`1.54.1` | 1.54.1（10/21） | 無新 feature |
| UUID | `1.29`／`1.29.0` | 1.29.0（10/23） | 逐 UUID label 保持 |

官方來源：[Rust](https://blog.rust-lang.org/releases/) · [Tauri](https://docs.rs/crate/tauri/latest) · [Tauri API](https://docs.rs/tauri/latest/tauri/struct.AppHandle.html) · [Leptos](https://docs.rs/crate/leptos/latest) · [SQLx](https://docs.rs/crate/sqlx/latest) · [Tokio](https://docs.rs/crate/tokio/latest) · [UUID](https://docs.rs/crate/uuid/latest)。Pet lock blob `d607731dc9af032fe7a6998b806d95b325a4f589`，四 workspace lock 已讀。

## ④ 驗證、證據與未完成

本r2新增的實測限於獨立Refactor：完整來源8fc的隔離副本套入三案Refactor，沒有套②功能Green。各案精確命令、通過／ignored數與原始log見本案Refactor段落及[驗證紀錄](r2-refactor-validation.json)。產品checkout與遠端main均未套用。

三案Refactor各自可直接套在原始8fc；也已實際檢查與各自Green前後兩種套用順序。原Chat Refactor及Chat／Pet Green存在換行格式不符的git apply失敗，本r2只修正Green patch換行格式，功能設計及斷言不變；原始失敗與修正後檢查都保留在[套用紀錄](r2-composition-checks.json)及[修正前紀錄](r2-composition-checks-before-eol-fix.json)。這些套用檢查不是Green功能測試。

**未執行**：②新功能Red／Green的Rust／WASM／Node／原生／瀏覽器測試與候選GitHub Actions；Pet Tauri server編譯及真生活窗事件亦未驗。Chat初次自動讀取、Pet找回視窗、Analytics CSV下載／試算表匯入各自依②的真入口驗收步驟執行，不能由Refactor回歸測試替代。

本次版本表保留同日r1的官方查核與來源lock紀錄，沒有改依賴或聲稱重新升級。Gate1依[本案精確commit全文讀回與blob／SHA256](pet-r2-readback.json)記錄；Gate2仍是使用者套用後，以新產品SHA跑功能／DB／GUI與CI。獨立段落提供的是可套用重構程式與證據，不是產品main已重構或已驗收。

既有原文、FIFO與有效斷言保留；Refactor不新增Day，next_day維持40，last_full_comparison不變。各案後續責任見本案Refactor最後一段；全專案G-01／G-02另列[工程項目](../../../engineering/README.md)，不以本案測試通過抵算。
