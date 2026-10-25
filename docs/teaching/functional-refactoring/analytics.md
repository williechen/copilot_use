# Analytics 專案重構

範圍涵蓋整個專案及共用依賴，依[同一工程進度](PROGRESS.md)逐日接續，不以當日教學切片限定範圍。以下是工程責任清單，產品仍唯讀；尚未交付完整patch的項目只算盤點。

## 專案範圍與相依

| 工程責任 | 需銜接的邊界 |
| --- | --- |
| 帳號與Session | bootstrap／登入／改密碼／撤銷用例、身份與Session Port，正式Router與Leptos接線 |
| 查詢與權限 | 統計查詢／角色／來源ACL用例、DTO與授權責任，PG Adapter不承擔HTTP決策 |
| 攝取與事件 | 機器憑證／事件驗證／冪等與持久化邊界，與人員Session分開 |
| 測試與PG CI | 隔離PG fixture、正式Router／Adapter與用例測試，功能斷言與準備流程分離；獨立PG選取 |
| 共用依賴／組裝 | 檢視四workspace公開契約、shared、Cargo normal／build／dev／target依賴、features與正式caller；先承接再清舊碼 |

先完成專案caller／Port／Adapter與斷言基線，再依相依選完整包。新功能與未完成需求依FUTURE_WORK另追蹤，不能把表內規劃寫成已存在或已重構。

## 已有局部盤點：保留為專案清單的一部分

| 舊測試／群組 | 判斷 | 處理與必留斷言 | 目標責任 |
| --- | --- | --- | --- |
| day37_real_query_routes_reject_ambiguous_session_cookies | 正式兩路由共同契約 | 按路由×Cookie情境參數化；不得刪一條路由覆蓋 | 正式Router＋真PG |
| first_admin_login_has_persistent_restricted_session | 首次登入／Session持久化契約 | 與Day37查詢不同，保留restricted／改密碼跳轉與DB斷言 | 正式Router＋真PG |
| 各PG測試重複bootstrap／Router／login準備 | fixture重複候選 | 可抽測試support；每測試自有DB與情境斷言，initialize仍有fixture用途 | analytics-server tests/support（擬議） |

來源 `rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9`。此表是來源核對後的擬議映射，非已遷移清單；完整跨案對照見 [測試覆蓋表](test-coverage-map.csv)。同層同契約才整併；跨層保留接線、SQL、HTTP／IPC或DOM獨有的驗證責任。fixture抽共用準備，不複製業務實作或將情境斷言藏入共用helper。


## 每日接續與驗收

每次列：前次包與產品已套用SHA→本次按專案依賴選的完整包／patch→有效測試與CI去向→實際／未執行驗證→累積狀態與下一包。未套用的前包不能當已存在基準；無本案工程進展就明列原因，不用切片helper整理補數。

已交付的跨案[CI責任隔離](../../engineering/2026-10-26/ci-isolation.md)共用同一份patch及證據，不能各案各算一包；目前未套用／未驗收。下一步及其餘案狀態見[PROGRESS](PROGRESS.md)。

完整包須交付必要用例／Port／DTO／錯誤、Adapter／組裝／正式caller、Cargo／features、測試遷移與CI相容內容；保留不同邊界值、故障模式及隔離斷言。不因測試搬家假造行為Red，也不刪斷言換全綠。規則集中於[共用正本](../../COMMON_LEARNING_RULES.md)，本頁不複製另一套規則。
