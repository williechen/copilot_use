# 各專案Refactor進度

最新來源 `b0e06b73f5ab37b899c00b2ba08fd4da780e0dbf`。教學候選、產品已接線部分與驗收分列，不以測試數降低宣稱重構完成。

| 專案 | 已在來源的獨立Refactor | 本次結果／限制 | 下一步 |
| --- | --- | --- | --- |
| [Chat](../daily/day-40/chat-r1.md#refactor) | signin統一shared token_digest | 最新/account被刪造成舊登入回歸；完整候選與補route方案另驗 | 保留既有授權caller與回歸 |
| [Pet](../daily/day-40/pet-r1.md#refactor) | Runtime.remove集中逐label停止漫遊 | b0已補nativeX，原產品仍少3舊斷言；恢復候選fmt／server60通過，真GUI未驗 | 原生生命週期與故障補償 |
| [Analytics](../daily/day-40/analytics-r1.md#refactor) | csv.rs承接serializer及原測試；export相容re-export | 真PG／Router及完整候選已驗；GUI／試算表未驗 | Session／角色／查詢caller |

[原始命令與兩組結果](../daily/day-40/local-validation/README.md) · [歷史勘誤](../daily/day-40/historical-corrections.md)。不新增Day、FIFO、last_full_comparison不變；[G-01／G-02](../../engineering/README.md)不因當案測試計功。
