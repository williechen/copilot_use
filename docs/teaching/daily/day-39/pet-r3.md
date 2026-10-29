# Pet Day39 r3｜逐 UUID 找回生活窗：已實作的 IPC、補償用例與驗證

> 2026-10-26：按最新已實作程式及本地驗證修訂；此為前一輪本地修訂，隨本次Day40歷史勘誤提交；下文未push等敘述限原驗證時點。原[初版](pet.md)、[r1](pet-r1.md)、[r2](pet-r2.md)及舊 patch 保留。Day40、FIFO、last_full_comparison 均不變。

產品來源 `williechen/rust_ai_ring@bb259aaf76542554a3a704d9c4f4a3a219d65cbf`；文件編修基底 `d8d9f4ddf46d923c6abcc83ef0b86e5439eee4c4`。本文程式逐段取自此精確來源，[來源片段與 hashes](r3-source-excerpts.json)可核對。共用規則與需求依文件基底版本；本修訂整理既有 Day39，沒有新增一天的功能或重構計功。

## ① 目標、現況與前置條件

控制窗每筆寵物提供「找回寵物」，將既存生活窗移至主螢幕可用區，只操作目標 UUID。最新來源已接 Leptos → JS bridge → Tauri command → 真視窗 API；不存在生活窗時回錯，不另開新窗。回籠補償也已移到 Application，不再說 Tauri caller 未接通。

前置：Linux GTK／WebKitGTK 依賴、WASM target 與 Tauri 2；真正視窗行為仍需原生環境。逐隻漫遊的持久化尚未完成，現行 `PET_ENABLE_ROAM` 全域政策不應被描述為完整需求已達成。

## Refactor

### 回籠補償移至 Application，Driver 留在 Tauri

原 `cage_saved_pet` 同時負責資料寫入與關窗補償。最新 `persist_then_close` 接收 write／close closures，使 Application 不依賴 Tauri 或 SQLx；只有寫入成功才關窗，關窗失敗回復 uncaged，補償亦失敗則保留兩個錯誤。`persist_then_open` 及原測試仍在同檔，沒有刪掉開窗與其他故障模式斷言。

以下是完整用例檔，包含 ready-future fixture 與所有遷移／新增測試；不是以摘要取代測試：

來源：[`pet_apps/pet_application/src/lifecycle.rs`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/pet_apps/pet_application/src/lifecycle.rs#L1)。

```rust
/// Persistence and window closures are the boundary; no Tauri/SQLx types enter Application.
pub async fn persist_then_open<W, F, O>(
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cell::{Cell, RefCell},
        future::{Future, ready},
        task::{Context, Poll, Waker},
    };
    // These fixtures use ready futures only; fail explicitly on Pending rather than pretending
    // this is a production executor.
    fn complete_ready<F: Future>(future: F) -> F::Output {
        let mut future = std::pin::pin!(future);
        match future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
        {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("fixture unexpectedly yielded"),
        }
    }
    #[test]
    fn persist_failure_never_opens_or_closes_a_window() {
        complete_ready(async {
            let opened = Cell::new(false);
            let writes = RefCell::new(Vec::new());
            let result = persist_then_open(
                true,
                |flag| {
                    writes.borrow_mut().push(flag);
                    ready(Err("DB".into()))
                },
                || {
                    opened.set(true);
                    Ok(())
                },
            )
            .await;
            assert!(result.is_err());
            assert!(!opened.get());
            assert_eq!(*writes.borrow(), vec![false]);
        });
    }

    #[test]
    fn open_failure_restores_the_actual_old_flag() {
        complete_ready(async {
            for old in [true, false] {
                let writes = RefCell::new(Vec::new());
                let result = persist_then_open(
                    old,
                    |flag| {
                        writes.borrow_mut().push(flag);
                        ready(Ok(()))
                    },
                    || Err("native".into()),
                )
                .await;
                assert!(result.unwrap_err().contains("native"));
                assert_eq!(*writes.borrow(), vec![false, old]);
            }
        });
    }

    #[test]
    fn success_writes_once_and_rollback_error_is_visible() {
        complete_ready(async {
            let writes = RefCell::new(Vec::new());
            persist_then_open(
                true,
                |flag| {
                    writes.borrow_mut().push(flag);
                    ready(Ok(()))
                },
                || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(*writes.borrow(), vec![false]);
            let calls = Cell::new(0);
            let error = persist_then_open(
                true,
                |_| {
                    calls.set(calls.get() + 1);
                    ready(if calls.get() == 1 {
                        Ok(())
                    } else {
                        Err("rollback".into())
                    })
                },
                || Err("native".into()),
            )
            .await
            .unwrap_err();
            assert!(error.contains("native") && error.contains("rollback"));
        });
    }

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
}
```

`pet_application/src/lib.rs` 保留 `pub mod lifecycle;`。正式 Tauri caller 負責生命週期 mutex、profile／UUID、真視窗與 Runtime：

來源：[`pet_apps/pet_server/src/pet_windows.rs::cage_saved_pet`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/pet_apps/pet_server/src/pet_windows.rs#L133)。

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

斷言承接：首次 DB 失敗不關窗；close 失敗的 `[true,false]` 補償順序、補償錯誤文字、成功只寫／關一次都在 Application；逐 ID SQLite 寫入及 Runtime／mutex 邊界仍由原 Adapter／Server 測試驗證。下一步是直接關窗、回籠持久化與重啟政策，不能由 closure 測試結案真 native 故障。

## ② Red → 現行 Green 與正式呼叫鏈

### Red 邊界

原 Red 是缺 `centered_position` 的編譯失敗，以及控制窗沒有找回按鈕的使用者行為；本次沒有移除實作重跑 Red。以下測試在最新來源均實際通過。

### Green：置中與真 command

`pet_windows.rs` 的既有 imports 提供 `Manager`、`PhysicalRect`、`PhysicalSize`、`PhysicalPosition`、`State`；沿既有 `PetStore`／`Runtime`，不新建第二套 Store。

來源：[`pet_apps/pet_server/src/pet_windows.rs::centered_position`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/pet_apps/pet_server/src/pet_windows.rs#L155)。

```rust
fn centered_position(
    area: &PhysicalRect<i32, u32>,
    size: PhysicalSize<u32>,
) -> PhysicalPosition<i32> {
    let x = i64::from(area.position.x) + i64::from(area.size.width.saturating_sub(size.width) / 2);
    let y =
        i64::from(area.position.y) + i64::from(area.size.height.saturating_sub(size.height) / 2);
    PhysicalPosition {
        x: x.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
        y: y.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
    }
}
```

來源：[`pet_apps/pet_server/src/pet_windows.rs::find_saved_pet`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/pet_apps/pet_server/src/pet_windows.rs#L168)。

```rust
#[tauri::command]
pub async fn find_saved_pet(
    id: String,
    app: tauri::AppHandle,
    store: State<'_, PetStore>,
) -> Result<(), String> {
    let _operation = lifecycle_guard().await;
    store.profile(&id).await?;
    let name = label(&id)?;
    let window = app.get_webview_window(&name).ok_or("寵物尚未出籠")?;
    let monitor = app
        .primary_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("找不到主螢幕")?;
    let size = window.outer_size().map_err(|e| e.to_string())?;
    let position = centered_position(monitor.work_area(), size);
    crate::roam::stop_for_pet(&name);
    let result = window
        .set_position(position)
        .and_then(|_| window.show())
        .and_then(|_| window.set_focus());
    crate::roam::spawn_for_pet(app.clone(), name);
    result.map_err(|e| format!("找回寵物失敗：{e}"))
}
```

置中使用 i64 中間值、saturating_sub 與 clamp；視窗大於可用區時釘在工作區原點。取 monitor 或 size 失敗發生在取消漫遊之前；移動／show／focus 的 Result 以原錯誤回傳，之後仍呼叫原漫遊啟動政策。舊任務取消是旗標，不等於已 await 結束，因此真視窗取消／重啟競態仍待測。

完整新位置測試：

來源：[`pet_apps/pet_server/src/pet_windows.rs::day39_find_position_respects_primary_work_area_and_negative_origins`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/pet_apps/pet_server/src/pet_windows.rs#L227)。

```rust
    #[test]
    fn day39_find_position_respects_primary_work_area_and_negative_origins() {
        let area = PhysicalRect {
            position: PhysicalPosition { x: -1920, y: -100 },
            size: PhysicalSize {
                width: 1920,
                height: 1080,
            },
        };
        assert_eq!(
            centered_position(
                &area,
                PhysicalSize {
                    width: 200,
                    height: 200
                }
            ),
            PhysicalPosition { x: -1060, y: 340 }
        );
        assert_eq!(
            centered_position(
                &area,
                PhysicalSize {
                    width: 2500,
                    height: 1200
                }
            ),
            PhysicalPosition { x: -1920, y: -100 }
        );
    }
```

逐 label 的停止函式與隔離測試：

來源：[`pet_apps/pet_server/src/roam.rs::stop_for_pet`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/pet_apps/pet_server/src/roam.rs#L44)。

```rust
pub(crate) fn stop_for_pet(label: &str) {
    if let Ok(tasks) = roam_tasks().lock() {
        if let Some(active) = tasks.get(label) {
            active.store(false, Ordering::Release);
        }
    }
}
```

來源：[`pet_apps/pet_server/src/roam.rs::day39_find_cancels_only_target_pet_roaming`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/pet_apps/pet_server/src/roam.rs#L327)。

```rust
    #[test]
    fn day39_find_cancels_only_target_pet_roaming() {
        let a = format!("pet-{}", uuid::Uuid::new_v4().simple());
        let b = format!("pet-{}", uuid::Uuid::new_v4().simple());
        let first = register_task(&a).unwrap();
        let other = register_task(&b).unwrap();
        stop_for_pet(&a);
        assert!(!first.load(Ordering::Acquire));
        assert!(other.load(Ordering::Acquire));
        finish_task(&a, &first);
        finish_task(&b, &other);
    }
```

### UI、bridge、command 註冊

`pet_server/src/lib.rs` 既有 `tauri::generate_handler!` 已包含 `pet_windows::find_saved_pet`。`index.html` 既有 script 的 `invoke` 來自 Tauri core，增加的正式 JS 入口如下：

```javascript
window.findSavedPet = (id) => invoke("find_saved_pet", { id });
```

`bridge.rs` 在既有 wasm_bindgen extern block 內的宣告及 async wrapper：

來源：[`pet_apps/pet_ui/src/bridge.rs`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/pet_apps/pet_ui/src/bridge.rs#L81)。

```rust
    #[wasm_bindgen(js_namespace=window, js_name=findSavedPet)]
    fn find_saved_pet_js(id: &str) -> Promise;
```

來源：[`pet_apps/pet_ui/src/bridge.rs::find_saved_pet`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/pet_apps/pet_ui/src/bridge.rs#L246)。

```rust
pub async fn find_saved_pet(id: &str) -> Result<(), String> {
    JsFuture::from(find_saved_pet_js(id))
        .await
        .map_err(js_error)?;
    Ok(())
}
```

列表用 pet.id 為 key，逐筆 clone `find_id`，pending 時停用，成功／失敗都解除 busy 並顯示結果。以下保留整個 For 修改單元與原本選取／出籠按鈕：

來源：[`pet_apps/pet_ui/src/main.rs`](https://github.com/williechen/rust_ai_ring/blob/bb259aaf76542554a3a704d9c4f4a3a219d65cbf/pet_apps/pet_ui/src/main.rs#L231)。

```rust
                    <For each=move || saved_pets.get() key=|pet| pet.id.clone()
                        children=move |pet| {
                            let id=pet.id.clone(); let open_id=id.clone(); let find_id=id.clone();
                            view! { <li>{format!("{}／{}（{}）", pet.name, pet.species, pet.id)}
                                <button type="button" disabled=move || pets_busy.get()
                                    on:click=move |_| {
                                        if pets_busy.get_untracked() { return; }
                                        set_pets_busy.set(true); let id=id.clone();
                                        spawn_local(async move {
                                            match bridge::select_pet(&id).await {
                                                Ok(pet) => { set_active_pet.set(Some(pet)); set_new_pet_feedback.set("生活窗選取已儲存".into()); },
                                                Err(message) => set_new_pet_feedback.set(message),
                                            }
                                            set_pets_busy.set(false);
                                        });
                                    }>"選為生活窗寵物"</button>
                                <button type="button" disabled=move || pets_busy.get()
                                    on:click=move |_| {
                                        if pets_busy.get_untracked() {return;}
                                        set_pets_busy.set(true);let id=open_id.clone();
                                        spawn_local(async move {
                                            match bridge::open_saved_pet(&id).await {Ok(())=>set_new_pet_feedback.set("已出籠；重複操作只聚焦既有生活窗".into()),Err(e)=>set_new_pet_feedback.set(e)}
                                            set_pets_busy.set(false);
                                        });
                                    }>"出籠"</button>
                                    <button type="button" disabled=move || pets_busy.get()
                                        on:click=move |_| {
                                            if pets_busy.get_untracked() { return; }
                                            set_pets_busy.set(true);
                                            let id = find_id.clone();
                                            spawn_local(async move {
                                                match bridge::find_saved_pet(&id).await {
                                                    Ok(()) => set_new_pet_feedback.set("已找回該隻生活窗".into()),
                                                    Err(e) => set_new_pet_feedback.set(e),
                                                }
                                                set_pets_busy.set(false);
                                            });
                                        }>"找回寵物"</button>
                            </li> }
                        } />
```

無新依賴／feature／migration／lock。Rust package 為 `pet-server`、`pet-ui`、`pet-application`、`pet-persistence`；JS bridge／WASM wrapper 名稱與 command 註冊完整相接。

## ③ 來源版本與執行環境

| 項目 | 精確來源或已執行環境 |
| --- | --- |
| Rust / Cargo | 實測 1.99.1；edition 2024 |
| leptos | Cargo.lock resolved `0.8.22` |
| sqlx | Cargo.lock resolved `0.9.0` |
| tauri | Cargo.lock resolved `2.12.5` |
| tauri-build | Cargo.lock resolved `2.7.4` |
| tokio | Cargo.lock resolved `1.54.1` |
| uuid | Cargo.lock resolved `1.29.0` |
| Native | GTK 3.24.52、WebKitGTK/JavaScriptCore 2.52.6、libsoup 3.6.6；已過原生編譯 |

四個 workspace 的 manifest／lock 已重新讀取；沒有改版本、依賴或 features。本文列的是來源 resolved 及當次實測版本，不宣稱本輪另做官方「最新穩定版」查核或升級；r2 的官方查核表保留其原日期與範圍。

## ④ 實測命令、證據與未完成

最新精確 SHA 實測：**70 項 Rust（Application 8＋SQLite 4＋Server 58）及 6 項 Node 通過；原生 server／WASM UI check 通過**；`cargo fmt --all -- --check` exit 0。這些已於前一輪驗證實際執行，本輪編修只重核來源與保存證據，沒有重跑或重複計功。

| 命令 | exit code | 原始 log |
| --- | --- | --- |
| `cargo fmt --all -- --check` | 0 | [log](r3-local-validation/logs/latest-pet-fmt.log) |
| `cargo test --offline --locked -p pet-application` | 0 | [log](r3-local-validation/logs/latest-pet-test-application.log) |
| `cargo test --offline --locked -p pet-persistence` | 0 | [log](r3-local-validation/logs/latest-pet-test-sqlite.log) |
| `cargo check --offline --locked -p pet-server` | 0 | [log](r3-local-validation/logs/latest-pet-check-server.log) |
| `cargo test --offline --locked -p pet-server` | 0 | [log](r3-local-validation/logs/latest-pet-test-server.log) |
| `cargo check --offline --locked -p pet-ui --target wasm32-unknown-unknown` | 0 | [log](r3-local-validation/logs/latest-pet-check-wasm.log) |
| `node --test pet_ui/tests/day33-open-controls-selects-origin.test.mjs pet_ui/tests/duplicate-preview-contract.test.mjs pet_ui/tests/scan-preview-contract.test.mjs pet_ui/tests/scan-preview-no-path.test.mjs` | 0 | [log](r3-local-validation/logs/latest-pet-node.log) |

當次工作目錄：`/home/willie/Documents/Codex/2026-10-26/referenced-chatgpt-conversation-this-is-an/work/latest/pet_apps`。完整工作目錄、環境覆寫、耗時及 log SHA256 見[逐命令紀錄](r3-local-validation/commands.jsonl)。表中 Cargo 以 `--offline --locked` 執行；快取命中不代表跳過測試，log 保留實際測試名稱與結果。

教學舊基底 `8fc4b27` 另有隔離候選：Refactor → Green 的兩份 patch 均先 `git apply --check` exit 0，再 `git apply` exit 0，之後實際 Cargo check/test 通過；但三案候選 fmt 均 exit 1，Chat/Pet 的 diff whitespace 檢查亦失敗。這些舊 patch 不適合重貼最新來源；最新來源正向套用檢查全部失敗，且不等同舊 patch 逐位元結果。新程式直接以最新 SHA 驗證。

尚未執行：真三寵物視窗、主／副螢幕切換、負座標桌面、find 後其他寵物不動、漫遊取消／重新啟動競態、native API 故障注入、直接關窗／重啟與完整設定持久化。Node 6 項是既有 bridge 回歸，不能代替新 command 的真 GUI 驗收。

[完整兩組驗證摘要](r3-local-validation/README.md)保留初次工作目錄被刪除與空間處理中斷；中斷不計成功。`rust_ai_ring` 本輪文件編修保持唯讀；未 push、未執行新 CI、未封存／宣稱 Gate 2 全部完成。
