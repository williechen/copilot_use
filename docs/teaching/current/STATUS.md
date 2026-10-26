# 目前功能與驗證狀態

最新來源 `b0e06b73f5ab37b899c00b2ba08fd4da780e0dbf`；教學候選與最新程式分列，原始產品未被本輪修復patch修改。

| 專案 | 最新來源所見 | 驗證／尚缺 |
| --- | --- | --- |
| Chat | /api/session及首頁Session顯示已接，但既有/account路由遭刪 | 舊登入測試404≠401、整體exit101；候選與補回route的followup另驗；瀏覽器／房間ACL未驗 |
| Pet | b0已補生活窗CloseRequested，Runtime清理、label、IPC destroy存在；3個舊斷言仍缺 | b0原生check及69項通過；恢復3舊斷言候選fmt／server60通過，真多窗／失敗／重啟未驗 |
| Analytics | event_type匯出、URL encode與純csv.rs已接，Session與Persistence沿原路徑 | 真PG／Router與ignored補跑結果見下，角色ACL／瀏覽器／試算表／CI未驗 |

| 範圍 | Chat Rust | Pet Rust | Analytics Rust |
| --- | --- | --- | --- |
| candidate | 59通過／0失敗 | 72通過／0失敗 | 53通過／0失敗 |
| latest | 57通過／2失敗 | 69通過／0失敗 | 53通過／0失敗 |
| followup | 59通過／0失敗 | 60通過／0失敗 | 未執行 |


兩組check與WASM通過；候選fmt三案失敗、最新fmt通過。Pet兩組另各Node6通過。上述是編譯與自動測試範圍，不是完整GUI或Gate2。

[三篇現行教學](../README.md) · [精確命令與原始log](../daily/day-40/local-validation/README.md) · [歷史更正](../daily/day-40/historical-corrections.md) · [Refactor進度](../functional-refactoring/PROGRESS.md) · [原功能待辦](../../FUTURE_WORK.md)。不結案FIFO，不新增Day，next_day=41，last_full_comparison不變；全專案工程另計。

發布前Pet來源刷新至b0並重新驗證；上表latest／followup為ff14初輪紀錄，Chat／Analytics／crates tree未變，測試沿用原SHA且不冒稱重跑。[來源與新測試](../daily/day-40/local-validation/source-refresh.json)。
