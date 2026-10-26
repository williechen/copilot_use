# Pet Day40 r2｜三個舊斷言已恢復，原生接線保留

2026-10-26；最新來源 `rust_ai_ring@9c767978b20bd56e1c2182dac2f37e6ae9b7d988`。本篇更正[r1](pet-r1.md)在舊SHA的狀態，保留原文、原patch與失敗證據；不新增Day或重構計功。完整原始Red／Green教學仍在r1，不能在已套用程式重貼修復patch。

## ① 最新狀態與修訂範圍

最新來源已恢復兩個Day24多寵物／UUID測試及Day39位置測試，不能繼續寫成「三個舊測試仍缺」。b0補上的生活窗CloseRequested→先保存→destroy與Runtime清理均保留；真native失敗、多窗和重啟仍未驗收。

## Refactor

Day40的既有責任仍由Runtime.remove集中停止目標漫遊，Application保有persist_then_close補償；此次只是恢復原有效斷言，不新增一個重構完成量，也不改其他寵物生命週期。

來源：[`pet_apps/pet_server/src/pet_windows.rs::remove`](https://github.com/williechen/rust_ai_ring/blob/9c767978b20bd56e1c2182dac2f37e6ae9b7d988/pet_apps/pet_server/src/pet_windows.rs#L35)。

```rust
    pub fn remove(&self, id: &str) {
        // Lifecycle cleanup belongs to Runtime, regardless of cage IPC or native Destroyed.
        if let Ok(name) = label(id) {
            crate::roam::stop_for_pet(&name);
        }
        if let Ok(mut pets) = self.machines.lock() {
            pets.remove(id);
        }
    }
```

## ② 恢復後完整測試與對照

以下三個測試逐一回到正式Runtime／label／centered_position；沒有降低斷言或用假的新helper抵消舊測試。

來源：[`pet_apps/pet_server/src/pet_windows.rs::day24_more_than_two_pet_states_are_independent_and_caging_is_local`](https://github.com/williechen/rust_ai_ring/blob/9c767978b20bd56e1c2182dac2f37e6ae9b7d988/pet_apps/pet_server/src/pet_windows.rs#L221)。

```rust
    #[test]
    fn day24_more_than_two_pet_states_are_independent_and_caging_is_local() {
        let runtime = Runtime::default();
        for id in ["one", "two", "three"] {
            runtime
                .machines
                .lock()
                .unwrap()
                .insert(id.into(), PetMachine::default());
        }
        let before_two = runtime.snapshot("two").unwrap();
        let before_three = runtime.snapshot("three").unwrap();
        runtime.dispatch("one", PetCommand::Sleep).unwrap();
        assert_eq!(runtime.snapshot("two").unwrap(), before_two);
        assert_eq!(runtime.snapshot("three").unwrap(), before_three);
        runtime.remove("one");
        assert!(runtime.snapshot("one").is_err());
        assert!(runtime.snapshot("two").is_ok());
        assert!(runtime.snapshot("three").is_ok());
    }
```

來源：[`pet_apps/pet_server/src/pet_windows.rs::day24_window_labels_are_derived_from_each_uuid_not_a_two_window_limit`](https://github.com/williechen/rust_ai_ring/blob/9c767978b20bd56e1c2182dac2f37e6ae9b7d988/pet_apps/pet_server/src/pet_windows.rs#L242)。

```rust
    #[test]
    fn day24_window_labels_are_derived_from_each_uuid_not_a_two_window_limit() {
        let labels: std::collections::HashSet<_> = (0..12)
            .map(|_| label(&uuid::Uuid::new_v4().to_string()).unwrap())
            .collect();
        assert_eq!(labels.len(), 12);
        assert!(label("../invalid").is_err());
    }
```

來源：[`pet_apps/pet_server/src/pet_windows.rs::day39_find_position_respects_primary_work_area_and_negative_origins`](https://github.com/williechen/rust_ai_ring/blob/9c767978b20bd56e1c2182dac2f37e6ae9b7d988/pet_apps/pet_server/src/pet_windows.rs#L251)。

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

舊恢復patch套在b0後，測試內容與最新檔一致；最新檔另有一處impl前空白行，逐bytes不完全相同。移除空白行後各行相同，這只算靜態來源比對，不能說最新SHA已編譯。

原生關窗接線未被本次兩檔變更移除；控制窗仍hide，生活窗CloseRequested先prevent_close，再用生命週期鎖與persist_then_close保存、destroy。完整native修改單元及故障限制見[r1](pet-r1.md)，其原始ff／b0缺口與結果保持原SHA。

## ③ 版本與依賴

無新依賴、feature、migration或lock變更；沿[r1版本核對](r1-versions.json)。未新增產品版本升級或官方版本普查。

## ④ 本輪實際檢查與待驗

三案fmt檢查exit0；已發布Chat／Pet修復patch在乾淨b0副本標準check及實際apply成功，再比對最新兩檔，詳見[命令／hash](source-update-9c76797/verification.json)。暫存副本已刪除。

**本輪未跑cargo check/test、真GUI或CI。** 前輪b0未恢復測試的69通過，以及恢復候選server60通過、fmt通過仍屬前輪；不直接填成最新SHA的72通過。要重新驗收時分別選Application、SQLite、server、WASM／Node及真多窗、直接X、故障／重啟流程。Day41、FIFO及last_full_comparison不變。
