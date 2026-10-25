# Pet 專案重構

範圍涵蓋整個專案及共用依賴，依[同一工程進度](PROGRESS.md)逐日接續，不以當日教學切片限定範圍。以下是工程責任清單，產品仍唯讀；尚未交付完整patch的項目只算盤點。

## 專案範圍與相依

| 工程責任 | 需銜接的邊界 |
| --- | --- |
| 生命週期用例 | 建立／出籠／回籠／選取／重啟載入，Persistence與Window Port、補償和相容錯誤 |
| 儲存與狀態 | SQLite Adapter與Row／DTO、Domain狀態機、Runtime／逐ID鎖與交易界線 |
| 原生與UI | Tauri IPC／視窗／漫遊／資源Adapter，Leptos CSR、原生產物與正式組裝 |
| 測試與原生CI | Application fake Port、SQLite真Adapter、正式IPC及native／GUI分層；保留所有有效故障與並行斷言 |
| 共用依賴／組裝 | 檢視四workspace公開契約、shared、Cargo normal／build／dev／target依賴、features與正式caller；先承接再清舊碼 |

先完成專案caller／Port／Adapter與斷言基線，再依相依選完整包。新功能與未完成需求依FUTURE_WORK另追蹤，不能把表內規劃寫成已存在或已重構。

## 已有局部盤點：保留為專案清單的一部分

| 舊測試／群組 | 判斷 | 處理與必留斷言 | 目標責任 |
| --- | --- | --- | --- |
| persist_failure_never_opens_or_closes_a_window | 流程契約 | 用例抽出後遷Application，保留先寫DB失敗不碰視窗 | Application fake Port |
| open_failure_restores_the_actual_old_flag | 補償契約 | 保留old_caged=true／false兩情境，不只測固定true | Application fake Port |
| success_writes_once_and_rollback_error_is_visible | 成功與補償再失敗兩責任 | 拆為具名情境；保留寫入次數與兩個錯誤均可見 | Application fake Port |
| lifecycle_lock_excludes_a_second_operation | 並行保護 | 保留真共用鎖接線，不能只把鎖測到用例fake；正式IPC併行另驗 | 組裝／server整合 |
| only_target_pet_changes_and_missing_id_fails + read_flag_and_fail_closed_for_missing_pet | 真SQL不同操作契約 | 保留逐ID隔離、missing、真DB錯誤；不是fake補償測試的重複 | pet-persistence SQLx Adapter |

來源 `rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9`。此表是來源核對後的擬議映射，非已遷移清單；完整跨案對照見 [測試覆蓋表](test-coverage-map.csv)。同層同契約才整併；跨層保留接線、SQL、HTTP／IPC或DOM獨有的驗證責任。fixture抽共用準備，不複製業務實作或將情境斷言藏入共用helper。


## 每日接續與驗收

每次列：前次包與產品已套用SHA→本次按專案依賴選的完整包／patch→有效測試與CI去向→實際／未執行驗證→累積狀態與下一包。未套用的前包不能當已存在基準；無本案工程進展就明列原因，不用切片helper整理補數。

已交付的跨案[CI責任隔離](../../engineering/2026-10-26/ci-isolation.md)共用同一份patch及證據，不能各案各算一包；目前未套用／未驗收。下一步及其餘案狀態見[PROGRESS](PROGRESS.md)。

完整包須交付必要用例／Port／DTO／錯誤、Adapter／組裝／正式caller、Cargo／features、測試遷移與CI相容內容；保留不同邊界值、故障模式及隔離斷言。不因測試搬家假造行為Red，也不刪斷言換全綠。規則集中於[共用正本](../../COMMON_LEARNING_RULES.md)，本頁不複製另一套規則。
