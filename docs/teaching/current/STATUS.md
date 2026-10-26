# 目前功能與驗證狀態

最新產品 `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`。本輪只作快轉、來源比對及三案fmt檢查；沒有重新執行cargo check/test、GUI或CI。

| 專案 | 最新來源狀態 | 證據與尚缺 |
| --- | --- | --- |
| Chat | /account已補回，與/api/session並存 | 修改檔與前輪補route候選bytes相同，fmt通過；前輪59通過不寫成新SHA已重跑；真瀏覽器／ACL待驗 |
| Pet | 3個舊斷言已恢復，b0的native CloseRequested保留 | 舊恢復patch對照只差空白行，fmt通過；新SHA的Rust／多窗／故障／重啟未驗 |
| Analytics | 本次產品變更未涉及其程式／crates／lock | 前輪真PG53通過保持原SHA；本輪fmt通過，未重跑PG／瀏覽器／試算表 |

## 歷史結果的適用範圍

ff14ca6的Chat57通過／2失敗及b0的Pet69通過／缺3舊測試屬當時結果；相關回歸和刪除已由17238f6／9c76797修正，不能在首頁再寫成最新缺口。前輪Chat修復候選59通過、Pet恢復候選server60通過（另有原Application／SQLite結果）、Analytics53通過仍保存原始log，不挪作本輪重跑。

[來源／命令／exit及hash](../daily/day-40/source-update-9c76797/verification.json) · [原完整Day40實測](../daily/day-40/local-validation/README.md) · [歷史更正](../daily/day-40/historical-corrections.md) · [Refactor](../functional-refactoring/PROGRESS.md)。Day41、FIFO、last_full_comparison不變，完整Gate2未宣稱通過。
