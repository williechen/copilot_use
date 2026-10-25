# Pet 現行教學入口

[Day35：逐寵物回籠狀態搬入 pet-persistence，接回 Tauri 真實呼叫鏈](../daily/day-35/pet.md)

**已套用／驗證結果**：Pet Day35 已在 e753c36 套用；persistence＋server 單元／整合共 55 項通過。三窗 GUI、故障補償注入、重啟及單幀單圖 FIFO 未驗收。下一篇 **Day36**。 [完整結果與勘誤](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-update-e753c36.md)。

[Day35 交付與讀回證據](../daily/day-35/pet-readback.json) · [上一篇 Day34](../daily/day-34/pet.md)（名稱規則已接 application／Tauri／PetStore）。

[首頁](../pet/README.md) · [目前功能與驗證現況](STATUS.md) · [進度](../../learning-state.json) · [manifest](manifest.json) · [CI 補充與來源證據](../daily/day-34/source-snapshot.json)

[前次程式重複套用核對](../../source-review/2026-10-25-duplicate-application-review.md)（來源 `3236b94e`；保留原教學與交付證據，進度不變）。

[全部後續調整與清理條件](../../FUTURE_WORK.md)：依本案順序選最小切片，不重做已接通內容；保留現行 Day 與原始證據。

## 舊內容與現存狀態

[全部文章逐篇核對](ARTICLE_STATUS.md) · [機器可讀狀態](article-states.json)

此案現行來源：pet-application 名稱規則保留；pet-persistence::set_caged 已接 PetStore／Tauri 真實 caller，其它 SQL 仍在 server。後端 global runtime／legacy IPC 仍待清理。 教學正文與原 commit/blob/SHA256 保留；局部測試不代表整產品功能驗收。
