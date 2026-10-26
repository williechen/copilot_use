# 目前功能與驗證狀態

來源：rust_ai_ring `9b89656f964b99e0cb89ba421855704a35c5ecb9`。本頁只列功能、實作與驗證；教學交付版本見各案ENTRY與manifest，工程進度另列。

| 功能 | 來源所見 | 實測範圍與尚缺 |
| --- | --- | --- |
| Chat房間搜尋 | reducer更新Signal且保留失敗前rows；RoomList尚未讀statusSignal | 648b903已有15項／WASM通過；當時瀏覽器HTTP500提示未顯示。9b未重跑瀏覽器；產品仍未套用可見status候選patch；隔離候選實測另見下段 |
| Chat帳號／房間 | 既有Session、建房名稱、搜尋與WS撤銷流程存在 | 首頁登入／管理、自動載入、房名與小字ID、匿名進房回跳、作者／歷史／ACL依原需求核對，不能由UI提示修補結案 |
| Pet生命週期 | 先保存後開窗、補償、逐ID生活窗與全部uncaged重啟迴圈已見 | 648b903曾有17項局部通過；歷史三窗／回籠／重啟GUI只限原SHA，9b未新增原生驗收 |
| Pet漫遊 | 固定pet label與真pet-UUID不符，啟動入口仍為舊全域task | 逐UUID候選方案未套用；靜態測試不能證明真視窗移動或瞬間取消，native／Wayland／X11待驗 |
| Analytics人員與查詢 | 管理員雜湊、首次改密碼、Session與summary Cookie guard已接正式caller | 648b903曾有相關真PG／HTTP40 passed；角色／來源ACL、正式互動頁、SSE與服務短效憑證仍有未完事項 |
| Analytics機器上報 | ingest仍用第一個Authorization值，既有source token與PG寫入保留 | 拒多值patch未套用產品；隔離候選已執行正式Router／PG測試，不以機器token代替人員登入 |
| 產品CI | 9b七個既有workflow均success | 只證明該SHA既有選取，未套用候選程式不在覆蓋內；同一Persistence job責任仍待隔離，不代表全產品GUI已驗收 |

[本次CI讀取證據](../daily/day-38/r2-product-ci-observation.json) · [候選patch靜態檢查](../daily/day-38/r2-static-checks.json) · [工程方案與未套用狀態](../functional-refactoring/PROGRESS.md)。產品checkout維持唯讀；本次Cargo／DB只在隔離候選副本執行，GUI與候選GitHub Actions未執行，不新增產品驗收結論。

## 原始驗證與條件

[648b903增量驗證](../../source-review/2026-10-25-day37-verification-648b903.md) · [5c713553前次功能實測](../../source-review/2026-10-25-day36-verification-5c713553.md) · [Pet原生證據](../../source-review/2026-10-25-pet-native-5d28e96.md) · [afdbb6e驗證](../../source-review/2026-10-25-assertions-afdbb6e.md)。歷次結果只屬原來源；舊回報若已由後續程式接通，仍需匹配SHA的測試與驗收，不能用舊失敗冒稱目前失敗。

[功能待辦與完成條件](../../FUTURE_WORK.md) · [文章現存狀態](ARTICLE_STATUS.md) · [更正對照](ARTICLE_CORRECTIONS.md) · [需求](../../requirements/USER_FLOWS.md) · [人員驗證](../../requirements/AUTHENTICATION.md)。本次整理不結案FIFO、不封存、不更新last_full_comparison。

## Day38 r4 隔離候選驗證

精確9b來源＋三案功能Green＋各案Refactor，並非產品main實測。Chat SSR14項通過、WASM check通過；Pet Application4項與Server／SQLite60項通過；Analytics Router／PG48項通過，1項ignored另跑通過。三案Red另在原始來源＋Red測試驗證：Chat／Pet為預期缺符號編譯失敗；Analytics為重複Authorization錯誤回201的行為失敗。這是來源對照驗證，不追稱先Red後Green的執行時序。

[命令、原始log、來源hash與環境失敗](../daily/day-38/r4-local-validation.json) · [各案完整patch與讀回](../daily/day-38/r4-delivery-verification.json) · [全專案獨立項目](../../engineering/README.md)。WASM保留既有create_room dead_code警告；Pet初次linker Bus error及磁碟quota失敗經任務專用環境調整後通過，未改產品設定。真瀏覽器／Pet生活窗／候選CI未驗收。
