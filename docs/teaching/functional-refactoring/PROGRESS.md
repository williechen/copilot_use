# 各專案Refactor進度

本頁集中各案自己的實際變更；[全專案項目](../../engineering/README.md)另列。正文提供詳細程式與測試，patch供套用；產品保持唯讀，候選驗證與產品驗收分開。

| 專案 | 詳細正文與實際變更 | 本次隔離Refactor驗證 | 產品套用／驗收 | 下一步 |
| --- | --- | --- | --- | --- |
| [Chat](../daily/day-40/chat.md#refactor) | Day39 SSR 契約保留；Day40 登入 token_digest 統一使用 shared::auth，完整 caller 見 Day40 | Day39 SSR 14項屬舊來源證據；Day40 尚未執行 Cargo | 未套用／GUI未驗 | Session 與房間權限 caller 責任 |
| [Pet](../daily/day-40/pet.md#refactor) | Day39 persist_then_close 保留；Day40 Runtime.remove 統一逐 UUID 漫遊停止、真 Destroyed/cage caller | Day39 舊來源 Application 8項／SQLite 4項；Day40 未跑 Cargo／native | 未套用／未驗收 | 直接關窗套用與故障補償驗收、逐 UUID Runtime |
| [Analytics](../daily/day-40/analytics.md#refactor) | Day39 SQL Adapter 保留；Day40 CSV serializer 搬至同案 csv.rs、相容 re-export 與舊測試完整承接 | Day39 舊來源 Server／PG 49項通過、1 ignored；Day40 未執行 Cargo／PG | 未套用／GUI未驗 | Session／角色／查詢 caller 與 fixture |

來源為精確8fc；本次只套三案Refactor，沒有套Day39功能Green。[命令、原始log與已測檔案hash](../daily/day-39/r2-refactor-validation.json)。三案與各自Green前後兩種順序皆git apply檢查通過；原Chat Refactor與Chat／Pet Green換行不符的失敗及修正紀錄均保留。功能Green、WASM、Pet原生GUI、Analytics CSV與候選GitHub Actions不由上述結果追認通過。

[本次獨立提交與讀回](../daily/day-39/r2-delivery-verification.json)。各案累積接續，不以當日Green決定重構順序，未套用方案不能當產品現況。全專案G-01／G-02維持獨立；不新增Day、不改FIFO或last_full_comparison。Day38隔離結果仍沿[原證據](../daily/day-38/r4-local-validation.json)追溯。
