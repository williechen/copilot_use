# Chat 專案重構

範圍涵蓋整個專案及共用依賴，依[同一工程進度](PROGRESS.md)逐日接續，不以當日教學切片限定範圍。以下是工程責任清單，產品仍唯讀；尚未交付完整patch的項目只算盤點。

## 專案範圍與相依

| 工程責任 | 需銜接的邊界 |
| --- | --- |
| 認證與Session | Application用例／Session與身份Port；HTTP／WS adapter及組裝，登入／登出／續期契約 |
| 房間與成員 | 房間／成員／加入與ACL用例、SQLite repository，公開清單與授權進房分責任 |
| 訊息與即時傳輸 | 訊息／作者／歷史與WS生命週期，HTTP／WS共用授權邊界與真caller |
| Leptos UI | 房間列表、帳號／管理與進房流程；UI狀態與API client邊界，SSR／hydrate保持一致 |
| 共用依賴／組裝 | 檢視四workspace公開契約、shared、Cargo normal／build／dev／target依賴、features與正式caller；先承接再清舊碼 |

先完成專案caller／Port／Adapter與斷言基線，再依相依選完整包。新功能與未完成需求依FUTURE_WORK另追蹤，不能把表內規劃寫成已存在或已重構。

## 已有局部盤點：保留為專案清單的一部分

| 舊測試／群組 | 判斷 | 處理與必留斷言 | 目標責任 |
| --- | --- | --- | --- |
| search_matches_chinese_names_and_preserves_ids + day18_query_filter_preserves_room_ids | 同層部分重複 | 合併參數化：中文搜尋／前後空白／空字串／純空白／無結果；保留ID | filter_rooms 的功能測試 |
| day19_failure_preserves_complete_rows_and_message + day19_refresh_failure_keeps_previous_room_links | 同層失敗快照斷言重疊 | 遷移舊helper有效斷言，保留同名不同ID／完整順序／錯誤文字／成功替換 | 正式刷新結果處理；再驗UI錯誤可見 |
| day20_refresh_success_reveals_full_snapshot + day20_refresh_failure_keeps_query_and_rows | 不同結果分支 | 保留成功清搜尋與失敗保留搜尋；可表格化但不刪情境 | refresh_result_view 或後續真狀態用例 |
| real_search_completion_updates_the_signals_used_by_the_view | Signal整合；不是DOM驗證 | 保留Signal接線；另補真元件／瀏覽器可見錯誤與自動載入 | UI層與瀏覽器 |

來源 `rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9`。此表是來源核對後的擬議映射，非已遷移清單；完整跨案對照見 [測試覆蓋表](test-coverage-map.csv)。同層同契約才整併；跨層保留接線、SQL、HTTP／IPC或DOM獨有的驗證責任。fixture抽共用準備，不複製業務實作或將情境斷言藏入共用helper。


## 每日接續與驗收

每次列：前次包與產品已套用SHA→本次按專案依賴選的完整包／patch→有效測試與CI去向→實際／未執行驗證→累積狀態與下一包。未套用的前包不能當已存在基準；無本案工程進展就明列原因，不用切片helper整理補數。

已交付的跨案[CI責任隔離](../../engineering/2026-10-26/ci-isolation.md)共用同一份patch及證據，不能各案各算一包；目前未套用／未驗收。下一步及其餘案狀態見[PROGRESS](PROGRESS.md)。

完整包須交付必要用例／Port／DTO／錯誤、Adapter／組裝／正式caller、Cargo／features、測試遷移與CI相容內容；保留不同邊界值、故障模式及隔離斷言。不因測試搬家假造行為Red，也不刪斷言換全綠。規則集中於[共用正本](../../COMMON_LEARNING_RULES.md)，本頁不複製另一套規則。
