# 各專案Refactor進度

本頁只列各案自己的實際變更；[全專案項目](../../engineering/README.md)另列。方案交付、產品套用與驗收分開，產品保持唯讀。

| 專案 | 本次程式／測試變更 | 實際候選驗證 | 產品套用／驗收 | 下一步 |
| --- | --- | --- | --- | --- |
| [Chat](../daily/day-39/chat-r1.md#refactor) | Day28/34 登入 SSR 契約測試遷至 signin_contract.rs；保留所有原斷言與正式 SignInPage | Day39 patch 已交付；本次 Cargo/SSR/WASM 未執行 | 未套用／未驗收 | Chat Session／登入導向與房間權限 |
| [Pet](../daily/day-39/pet.md#refactor) | cage_saved_pet 補償移至 pet_application::lifecycle::persist_then_close；server 真 caller 接回並新增純測試 | Day39 patch 已交付；本次 Cargo/native GUI 未執行 | 未套用／未驗收 | 直接關窗持久化與 Runtime／鎖 |
| [Analytics](../daily/day-39/analytics-r1.md#refactor) | summary_for_source PG 聚合搬至 analytics_persistence；dashboard 真 caller、舊 API re-export 與測試承接 | Day39 patch 已交付；本次 PG／CI 未執行 | 未套用／未驗收 | Session／角色 ACL／查詢 UI 與 fixture |

Day39 三案均為完整候選 patch，**未執行本次測試／GUI／CI**，不能追認產品 main 已套用或驗收。Day38 的隔離候選測試屬舊來源，仍按歷史保存，不挪作 Day39 實測。[本地完整證據](../daily/day-38/r4-local-validation.json)分列原始環境錯誤、恢復與Red類型。每案累積接續，不以當日Green決定重構順序；未套用方案不假設已存在。全專案G-01的全庫CI不在三案計功。保留既有原文、斷言與FIFO，不增加Day、不更新last_full_comparison。
