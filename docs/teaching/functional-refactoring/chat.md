# 房間查詢與畫面回饋：後續測試重構整理

本篇為功能與測試重構整理，不指定教學日，不是新文章交付或已完成Green。依既有功能整理測試責任。

## 測試責任與重複覆蓋處理

| 舊測試／群組 | 判斷 | 處理與必留斷言 | 目標責任 |
| --- | --- | --- | --- |
| search_matches_chinese_names_and_preserves_ids + day18_query_filter_preserves_room_ids | 同層部分重複 | 合併參數化：中文搜尋／前後空白／空字串／純空白／無結果；保留ID | filter_rooms 的功能測試 |
| day19_failure_preserves_complete_rows_and_message + day19_refresh_failure_keeps_previous_room_links | 同層失敗快照斷言重疊 | 遷移舊helper有效斷言，保留同名不同ID／完整順序／錯誤文字／成功替換 | 正式刷新結果處理；再驗UI錯誤可見 |
| day20_refresh_success_reveals_full_snapshot + day20_refresh_failure_keeps_query_and_rows | 不同結果分支 | 保留成功清搜尋與失敗保留搜尋；可表格化但不刪情境 | refresh_result_view 或後續真狀態用例 |
| real_search_completion_updates_the_signals_used_by_the_view | Signal整合；不是DOM驗證 | 保留Signal接線；另補真元件／瀏覽器可見錯誤與自動載入 | UI層與瀏覽器 |

來源 `rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9`。此表是來源核對後的擬議映射，非已遷移清單；完整跨案對照見 [測試覆蓋表](test-coverage-map.csv)。同層同契約才整併；跨層保留接線、SQL、HTTP／IPC或DOM獨有的驗證責任。fixture抽共用準備，不複製業務實作或將情境斷言藏入共用helper。

## 這次後續Red／Green／Refactor須完成什麼

- **Red：** 新增瀏覽器Red：進首頁不按按鈕即讀取；500時看到錯誤、可重試、保留舊快照。既有filter／Signal通過案例只列回歸。
- **Green：** 在同篇提供必要完整用例／Port／adapter／caller與測試、Cargo／module／feature／fixture接線；目前仍待撰寫，不能把既有教材Green當成此次測試重構已完成。
- **Refactor：** 逐條核對表中斷言已在新正式路徑保留，再刪失效helper／獨占測試；保留不同邊界值、故障模式與資料隔離。測試以功能／行為命名，Day識別保留在來源對照，不改寫舊文。
- **CI：** 純內層、各案DB、server正式整合、Pet原生與UI分責任；不因刪掉重複測試而漏選正式Router／IPC或PG feature。三案不串接阻擋，必要項失敗仍使整體檢核失敗。


## 適用界線

新用例／Port／adapter／正式caller與完整測試程式仍須在後續適用功能中提供，本次只發布文件，沒有套用產品或跑新測試。版本資料見[當日核對](../../source-review/2026-10-26-versions-and-summary-refresh.md)；換日期或增依賴時重新查核。三案next_day保持38，本次不增減Day或結案FIFO。
