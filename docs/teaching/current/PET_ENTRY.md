# Pet 現行教學入口

[問題文章與後續更正總表](ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

[Day36 r2：重啟恢復全部未回籠寵物，而非只恢復最後選取者](../daily/day-36/pet-r2.md)

**文章已交付／程式未套用／新測試未執行／未驗收**；來源 rust_ai_ring `61ed5930bd9ff38e0d5822eea2983121001d8422`。下一篇 **Day37**；[r2 交付全文讀回](../daily/day-36/pet-r2-readback.json)。

[Day36 r1 原始文章](../daily/day-36/pet.md)（保留原 commit/blob/SHA256，r2 只修正來源報告相對路徑） · [上一篇 Day35：逐寵物回籠狀態](../daily/day-35/pet.md) 已在後續來源套用並完成增量測試，真原生重啟與補償仍待驗；[Day35 原始交付證據](../daily/day-35/pet-readback.json)。

[首頁](../pet/README.md) · [目前功能與驗證現況](STATUS.md) · [進度](../../learning-state.json) · [manifest](manifest.json) · [CI 補充與來源證據](../daily/day-34/source-snapshot.json)

[前次程式重複套用核對](../../source-review/2026-10-25-duplicate-application-review.md)（來源 `3236b94e`；保留原教學與交付證據，進度不變）。

[全部後續調整與清理條件](../../FUTURE_WORK.md)：依本案順序選最小切片，不重做已接通內容；保留現行 Day 與原始證據。

## 舊內容與現存狀態

[全部文章逐篇核對](ARTICLE_STATUS.md) · [機器可讀狀態](article-states.json)

此案現行來源：pet-application 名稱規則保留；pet-persistence::set_caged 已接 PetStore／Tauri 真實 caller，其它 SQL 仍在 server。後端 global runtime／legacy IPC 仍待清理。 教學正文與原 commit/blob/SHA256 保留；局部測試不代表整產品功能驗收。
