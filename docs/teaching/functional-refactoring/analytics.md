# 查詢Session授權：後續測試重構整理

本篇為功能與測試重構整理，不指定教學日，不是新文章交付或已完成Green。依既有功能整理測試責任；Day37原文與證據保持不動，也不建立其修訂稿。

## 測試責任與重複覆蓋處理

| 舊測試／群組 | 判斷 | 處理與必留斷言 | 目標責任 |
| --- | --- | --- | --- |
| day37_real_query_routes_reject_ambiguous_session_cookies | 正式兩路由共同契約 | 按路由×Cookie情境參數化；不得刪一條路由覆蓋 | 正式Router＋真PG |
| first_admin_login_has_persistent_restricted_session | 首次登入／Session持久化契約 | 與Day37查詢不同，保留restricted／改密碼跳轉與DB斷言 | 正式Router＋真PG |
| 各PG測試重複bootstrap／Router／login準備 | fixture重複候選 | 可抽測試support；每測試自有DB與情境斷言，initialize仍有fixture用途 | analytics-server tests/support（擬議） |

來源 `rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9`。此表是來源核對後的擬議映射，非已遷移清單；完整跨案對照見 [測試覆蓋表](test-coverage-map.csv)。同層同契約才整併；跨層保留接線、SQL、HTTP／IPC或DOM獨有的驗證責任。fixture抽共用準備，不複製業務實作或將情境斷言藏入共用helper。

## 這次後續Red／Green／Refactor須完成什麼

- **Red：** 保留兩查詢路由×Cookie／restricted／logout矩陣；若只搬純授權用例，新Port缺符號可列編譯Red，不能聲稱既有授權契約原本失敗。
- **Green：** 在同篇提供必要完整用例／Port／adapter／caller與測試、Cargo／module／feature／fixture接線；目前仍待撰寫，不能把既有教材Green當成此次測試重構已完成。
- **Refactor：** 逐條核對表中斷言已在新正式路徑保留，再刪失效helper／獨占測試；保留不同邊界值、故障模式與資料隔離。測試以功能／行為命名，Day識別保留在來源對照，不改寫舊文。
- **CI：** 純內層、各案DB、server正式整合、Pet原生與UI分責任；不因刪掉重複測試而漏選正式Router／IPC或PG feature。三案不串接阻擋，必要項失敗仍使整體檢核失敗。


## 適用界線

新用例／Port／adapter／正式caller與完整測試程式仍須在後續適用功能中提供，本次只發布文件，沒有套用產品或跑新測試。版本資料見[當日核對](../../source-review/2026-10-26-versions-and-summary-refresh.md)；換日期或增依賴時重新查核。三案next_day保持38，本次不增減Day或結案FIFO。
