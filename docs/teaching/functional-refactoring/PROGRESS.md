# 每日全專案工程進度

依[共用規則](../../COMMON_LEARNING_RULES.md)，工程按專案整體架構、測試與CI的相依每日接續一個完整小包、與功能教學分開計時，不增加 Day。rust_ai_ring 唯讀，使用者套用。

| 工作包 | 方案交付 | 產品套用 | 驗收 |
| --- | --- | --- | --- |
| [2026-10-26：CI責任隔離](../../engineering/2026-10-26/ci-isolation.md) | 完整 YAML／patch、斷言去向與驗證方案；套用檢查與 YAML 解析通過 | 未套用 | 未執行 Actions、required checks、DB、native／GUI |
| [三案分層與測試盤點](README.md) | 現況／依賴與12組測試映射已整理；尚待完整 caller 與斷言基線 | 無新增產品變更 | 不視為重構完成 |

## 三案累積接續

| 專案 | 當前工程基線 | 下一步 | 套用／驗收 |
| --- | --- | --- | --- |
| [Chat](chat.md) | 房間查詢局部測試盤點僅為子集；CI跨案包已交付 | 補齊認證／Session、房間／成員、訊息／WS、UI及shared的caller／Port／Adapter與斷言基線 | 無新增套用／驗收 |
| [Pet](pet.md) | 出籠補償局部盤點僅為子集；CI跨案包已交付 | 補齊生命週期、Runtime／鎖、SQLite、Tauri／視窗／漫遊與產物的責任／斷言基線 | 無新增套用／驗收 |
| [Analytics](analytics.md) | 查詢Cookie局部盤點僅為子集；CI跨案包已交付 | 補齊帳號／Session、查詢／ACL、攝取／憑證、PG與UI的caller／斷言基線 | 無新增套用／驗收 |

三案共用CI工作包不重複計功。此表是實際盤點接續，尚未交付新的產品重構patch；各篇Refactor引用本案累積進度，不能只寫「本切片」處理或把局部整理充作工程完成。

下一個小包：以最新已套用 SHA 完成四 workspace caller／Port／Adapter／fixture／CI 執行入口的完整基線，再依相依順序逐功能提供完整程式與測試遷移。若前包未套用，後包不得假定其已存在。既有功能與 FIFO 均沿原紀錄，無平行清單。
