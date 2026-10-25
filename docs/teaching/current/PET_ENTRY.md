# Pet 現行教學入口

**2026-10-25 後續功能分層要求已確認。** 後續教學按功能提供用例／Port／Adapter／UI／正式caller與測試的完整必要重構，沿原功能項追蹤；本次僅更新規則，未重構產品、未新增Day或驗收。[架構正本](../../SYSTEM_ARCHITECTURE.md#functional-layering) · [來源評估與範圍](../../source-review/2026-10-25-functional-layering.md)。

[Day37 r3：驗證更新](../daily/day-37/pet-r3.md)

**程式已見／62項與WASM通過／GUI局部通過／故障注入待驗**；來源 `648b90323af7b381d6a79dc2fd08a53b63173a43`，下一篇仍為 **Day38**。[精確交付讀回](../daily/day-37/pet-r3-readback.json) · [本次完整驗證／錯誤與恢復](../../source-review/2026-10-25-day37-verification-648b903.md)。下方先前來源、交付及「未套用／未測試」摘要僅保留其當時範圍，不代表目前狀態。

**2026-10-25 Pet 錯誤回報：尚未修復／未驗收。** 最新來源 `744a75fc` 的漫遊仍查固定 pet，與 pet-UUID 生活窗不符；使用者補充所有視窗空白、每次出籠重新開啟；WebKit／共用前端載入原因待查，尚未證實同 ID 重複建窗。[原始錯誤與來源核對](../../source-review/2026-10-25-pet-roaming-webkit-feedback.md)。本來源已見 Day37 出籠補償／協調函式；舊交付摘要限原時點，不代表目前完全未套用。本次未做產品修改或 GUI 驗收，進度不變。

[Day37 r2：先保存再開窗，失敗回復原標記](../daily/day-37/pet-r2.md)

**教學已交付／程式未套用／新測試未執行／產品未驗收**；下一篇 **Day38**。[精確 commit 全文讀回、blob 與 SHA256](../daily/day-37/pet-r2-readback.json)。程式來源 `5c713553daee4055738239f047d6c774fcc7a7a1`；以下保留前次來源／驗證摘要，不能當成新 Green 成功。


<!-- comparison-5c713553:start -->
[最新三專案程式／文章比對](../../source-review/2026-10-25-full-comparison-5c713553.md)：105個閱讀版本、111份修訂、21項差異；來源 `5c713553`，局部測試與完整驗收分開記錄。

啟動已逐ID恢復未回籠資料；預設資格／回籠分離、直接關窗保存、出籠DB失敗補償及動態漫遊仍待處理。
<!-- comparison-5c713553:end -->

[問題文章與後續更正總表](ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

[Day36 r3：重啟恢復全部未回籠寵物，而非只恢復最後選取者](../daily/day-36/pet-r3.md)

**程式已存在／相關實測通過／完整產品未驗收**；實測來源 rust_ai_ring `5c713553daee4055738239f047d6c774fcc7a7a1`，原始教學基準 `61ed5930`。當時下一篇 **Day37**（現行進度見上方）；[r3 交付全文讀回](../daily/day-36/pet-r3-readback.json)。

Server 54、Day36 adapter／Store 測試與 WASM 通過。Store 重建不是原生程序重啟；Tauri 多窗重啟及建窗失敗注入待驗。 [完整實測報告](../../source-review/2026-10-25-day36-verification-5c713553.md)。原版文章及交付證據均保留。

[Day36 r1 原始文章](../daily/day-36/pet.md)（保留原 commit/blob/SHA256，r2 只修正來源報告相對路徑） · [上一篇 Day35：逐寵物回籠狀態](../daily/day-35/pet.md) 已在後續來源套用並完成增量測試，真原生重啟與補償仍待驗；[Day35 原始交付證據](../daily/day-35/pet-readback.json)。

[首頁](../pet/README.md) · [目前功能與驗證現況](STATUS.md) · [進度](../../learning-state.json) · [manifest](manifest.json) · [CI 補充與來源證據](../daily/day-34/source-snapshot.json)

[前次程式重複套用核對](../../source-review/2026-10-25-duplicate-application-review.md)（來源 `3236b94e`；保留原教學與交付證據，進度不變）。

[全部後續調整與清理條件](../../FUTURE_WORK.md)：依本案順序選最小切片，不重做已接通內容；保留現行 Day 與原始證據。

## 舊內容與現存狀態

[全部文章逐篇核對](ARTICLE_STATUS.md) · [機器可讀狀態](article-states.json)

此案現行來源：pet-application 名稱規則保留；pet-persistence::set_caged／list_uncaged已接PetStore／Tauri與啟動caller，其它 SQL 仍在 server。後端 global runtime／legacy IPC 仍待清理。 教學正文與原 commit/blob/SHA256 保留；局部測試不代表整產品功能驗收。
