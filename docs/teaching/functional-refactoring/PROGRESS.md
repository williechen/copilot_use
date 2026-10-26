# 各專案Refactor進度

最新來源 `ff14ca6224f7c5a65c799bf4a82a743fa79c880c`。教學候選、產品已接線部分與驗收分列，不以測試數降低宣稱重構完成。

| 專案 | 已在來源的獨立Refactor | 本次結果／限制 | 下一步 |
| --- | --- | --- | --- |
| [Chat](../daily/day-40/chat-r1.md#refactor) | signin統一shared token_digest | 最新/account被刪造成舊登入回歸；完整候選與補route方案另驗 | 保留既有授權caller與回歸 |
| [Pet](../daily/day-40/pet-r1.md#refactor) | Runtime.remove集中逐label停止漫遊 | 最新缺nativeX分支且移除3舊斷言；恢復patch另附，不能算完整回籠 | 原生生命週期與故障補償 |
| [Analytics](../daily/day-40/analytics-r1.md#refactor) | csv.rs承接serializer及原測試；export相容re-export | 真PG／Router及完整候選已驗；GUI／試算表未驗 | Session／角色／查詢caller |

[原始命令與兩組結果](../daily/day-40/local-validation/README.md) · [歷史勘誤](../daily/day-40/historical-corrections.md)。不新增Day、FIFO、last_full_comparison不變；[G-01／G-02](../../engineering/README.md)不因當案測試計功。
