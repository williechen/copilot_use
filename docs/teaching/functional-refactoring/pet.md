# 出籠流程與補償：後續測試重構整理

本篇為功能與測試重構整理，不指定教學日，不是新文章交付或已完成Green。依既有功能整理測試責任；Day37原文與證據保持不動，也不建立其修訂稿。

## 測試責任與重複覆蓋處理

| 舊測試／群組 | 判斷 | 處理與必留斷言 | 目標責任 |
| --- | --- | --- | --- |
| persist_failure_never_opens_or_closes_a_window | 流程契約 | 用例抽出後遷Application，保留先寫DB失敗不碰視窗 | Application fake Port |
| open_failure_restores_the_actual_old_flag | 補償契約 | 保留old_caged=true／false兩情境，不只測固定true | Application fake Port |
| success_writes_once_and_rollback_error_is_visible | 成功與補償再失敗兩責任 | 拆為具名情境；保留寫入次數與兩個錯誤均可見 | Application fake Port |
| lifecycle_lock_excludes_a_second_operation | 並行保護 | 保留真共用鎖接線，不能只把鎖測到用例fake；正式IPC併行另驗 | 組裝／server整合 |
| only_target_pet_changes_and_missing_id_fails + read_flag_and_fail_closed_for_missing_pet | 真SQL不同操作契約 | 保留逐ID隔離、missing、真DB錯誤；不是fake補償測試的重複 | pet-persistence SQLx Adapter |

來源 `rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9`。此表是來源核對後的擬議映射，非已遷移清單；完整跨案對照見 [測試覆蓋表](test-coverage-map.csv)。同層同契約才整併；跨層保留接線、SQL、HTTP／IPC或DOM獨有的驗證責任。fixture抽共用準備，不複製業務實作或將情境斷言藏入共用helper。

## 這次後續Red／Green／Refactor須完成什麼

- **Red：** 抽出純用例後可先記缺符號編譯Red；既有補償測試已通過，不冒稱行為Red。新接線須有正式IPC／故障注入案例，不能只搬closure測試。
- **Green：** 在同篇提供必要完整用例／Port／adapter／caller與測試、Cargo／module／feature／fixture接線；目前仍待撰寫，不能把既有教材Green當成此次測試重構已完成。
- **Refactor：** 逐條核對表中斷言已在新正式路徑保留，再刪失效helper／獨占測試；保留不同邊界值、故障模式與資料隔離。測試以功能／行為命名，Day識別保留在來源對照，不改寫舊文。
- **CI：** 純內層、各案DB、server正式整合、Pet原生與UI分責任；不因刪掉重複測試而漏選正式Router／IPC或PG feature。三案不串接阻擋，必要項失敗仍使整體檢核失敗。


## 適用界線

新用例／Port／adapter／正式caller與完整測試程式仍須在後續適用功能中提供，本次只發布文件，沒有套用產品或跑新測試。版本資料見[當日核對](../../source-review/2026-10-26-versions-and-summary-refresh.md)；換日期或增依賴時重新查核。三案next_day保持38，本次不增減Day或結案FIFO。
