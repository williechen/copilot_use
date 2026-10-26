# 各專案Refactor進度

本頁集中各案自己的實際變更；[全專案項目](../../engineering/README.md)另列。正文提供詳細程式與測試，patch供套用；產品保持唯讀，候選驗證與產品驗收分開。

| 專案 | 詳細正文與實際變更 | 本次隔離Refactor驗證 | 產品套用／驗收 | 下一步 |
| --- | --- | --- | --- | --- |
| [Chat](../daily/day-39/chat-r2.md#refactor) | 兩組登入SSR契約遷至signin_contract.rs；完整新檔、cfg接線與兩個舊測試斷言逐一比對 | SSR 14項通過，兩組測試內容僅排版改變 | 未套用／GUI未驗 | 登入／Session與房間權限caller責任 |
| [Pet](../daily/day-39/pet-r2.md#refactor) | persist_then_close完整用例、真cage caller、四個新增測試；保留Runtime／mutex邊界 | Application 8項、真SQLite 4項通過；Tauri server與真窗未驗 | 未套用／未驗收 | 直接關窗、caged持久化與Runtime |
| [Analytics](../daily/day-39/analytics-r2.md#refactor) | 完整來源SQL移Persistence、正式import、相容re-export、真PG測試 | Server／Persistence 49項通過、1項ignored；原SQL除空白外相同 | 未套用／GUI未驗 | Session／角色／查詢caller與fixture |

來源為精確8fc；本次只套三案Refactor，沒有套Day39功能Green。[命令、原始log與已測檔案hash](../daily/day-39/r2-refactor-validation.json)。三案與各自Green前後兩種順序皆git apply檢查通過；原Chat Refactor與Chat／Pet Green換行不符的失敗及修正紀錄均保留。功能Green、WASM、Pet原生GUI、Analytics CSV與候選GitHub Actions不由上述結果追認通過。

[本次獨立提交與讀回](../daily/day-39/r2-delivery-verification.json)。各案累積接續，不以當日Green決定重構順序，未套用方案不能當產品現況。全專案G-01／G-02維持獨立；不新增Day、不改FIFO或last_full_comparison。Day38隔離結果仍沿[原證據](../daily/day-38/r4-local-validation.json)追溯。
