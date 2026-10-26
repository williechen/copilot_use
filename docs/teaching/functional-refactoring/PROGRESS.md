# 各專案Refactor進度

本頁只列各案自己的實際變更；[全專案項目](../../engineering/README.md)另列。方案交付、產品套用與驗收分開，產品保持唯讀。

| 專案 | 本次程式／測試變更 | 實際候選驗證 | 產品套用／驗收 | 下一步 |
| --- | --- | --- | --- | --- |
| [Chat](../daily/day-38/chat-r4.md#refactor) | room_list_state模組、正式caller imports、去重並遷移有效斷言 | 精確來源patch／兩種順序通過；完整SSR14項與WASM check通過 | 未套用；真瀏覽器／完整產物與CI未驗收 | 依Chat自身認證／Session、房間與訊息邊界接續 |
| [Pet](../daily/day-38/pet-r4.md#refactor) | 補償流程／純測試移入Application，正式IPC import接線；真鎖留server | patch／兩種順序通過；Application4項、server／persistence60項通過；配額錯誤恢復留證 | 未套用；真生活窗GUI與CI未驗收 | 依Pet自身回籠、Runtime／鎖與Adapter接續 |
| [Analytics](../daily/day-38/analytics-r4.md#refactor) | 三組PG測試共用support，保留每情境全部斷言與DB隔離 | patch／兩種順序通過；完整server／PG48項，ignored1項另跑通過 | 未套用；真瀏覽器與CI未驗收 | 依Analytics自身Session／查詢／PG／UI與fixture接續 |

以上皆為精確來源＋功能Green／Refactor的完整隔離候選測試，不能追認產品main已套用或GUI／CI已驗收。[本地完整證據](../daily/day-38/r4-local-validation.json)分列原始環境錯誤、恢復與Red類型。每案累積接續，不以當日Green決定重構順序；未套用方案不假設已存在。全專案G-01的全庫CI不在三案計功。保留既有原文、斷言與FIFO，不增加Day、不更新last_full_comparison。
