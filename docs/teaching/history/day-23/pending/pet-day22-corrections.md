# Day23 第2節追加單元：Day22 更正補充

狀態：pending，尚未發布 Day23。執行前須重讀 main、規則、套件、makeup_queue；預期 next_day=23，但本文件不推進計數。只在本日新 Red／完整 Green／Refactor 結束後追加，不取代本日新實作。依 FIFO，每篇最多一個舊 Day 單元，發布與全文讀回後才可標結案。

來源核對基底 `cabacb75e946b03c67c455ce13a7a4ea85652f8e`，規則 daily-tdd-minimum-v1 ＋ 2026-10-20 封存順序補充。

## 舊 Day 更正補充（程式／套件差異）：Pet Day22

main 已有 PetStore／create_pet 與同 pool 接線，不再照舊文重複新增。記錄 workspace 0.2.22／Tokio1.54.0、SQLx0.9.0／Tauri2.12.4；本輪28個直接依賴已是最新穩定版，不為補教執行 cargo update。

控制台既有 Rust bridge／window.validateNewPetName 目前呼叫 create_pet；舊名稱與建立副作用不一致，後續若更名必須同步 frontend/index.html、frontend/src/bridge.rs extern與wrapper、frontend/src/main.rs呼叫者，不能只改其中一處。此為待實作建議，非本輪已套用。

20261018091733_create_pet.sql 已增加 pet_activity.is_use_ai、pet_study.is_use_ai、pet_life_log.has_use_ai；這是修改舊檔，不是新增 create_pet migration。SQLx probe 從 da515526 歷史升級會 checksum mismatch，sentinel 保留；fresh最新DB成功。正式 DB 歷史未讀取；先確認已套用 checksum與備份，再決定基底並新增前向migration，禁止刪庫／改checksum掩蓋。

本輪 cargo test --locked -p desktop-pet：42 passed；控制台 WASM check通過。不能據此宣稱 native GUI、逐寵物生活窗／外觀／重啟載入已完成；這些是下一主線判斷依據，不以純欄位或 helper 當新功能。

第3節更新套件／lock，第四節揭露 fresh與upgrade範圍。完整補教讀回前仍pending。
