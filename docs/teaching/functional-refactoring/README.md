# 功能重構與測試整理

2026-10-26整理。**Day37不重建、不新增修訂**；原文章、commit／blob／SHA256與驗證證據保留。這裡按功能整理後續程式、測試及CI責任，不指定教學日，不是完整Green、新Day交付或產品驗收。

程式來源 [`rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9`](https://github.com/williechen/rust_ai_ring/tree/9b89656f964b99e0cb89ba421855704a35c5ecb9)；文件整理基準 `c67044f59a161fb2d131771c0db3d09cd80e6e05`。產品程式與workflow唯讀。本次未執行產品測試、未修改DB／lock／migration，三案next_day仍38，不變更FIFO或last_full_comparison。

## 三案功能整理

| 專案 | 內容與保留條件 |
| --- | --- |
| [Chat：房間查詢與畫面回饋](chat.md) | 合併同層filter／刷新重疊斷言；保留空白、同名不同ID、完整快照與錯誤條件。Signal接線與DOM可見行為分別驗證。 |
| [Pet：出籠流程與補償](pet.md) | 抽用例時同步遷移純補償測試；SQLite、共用鎖、原生Adapter與正式IPC保留各自覆蓋。 |
| [Analytics：查詢Session授權](analytics.md) | 正式兩路由的Cookie矩陣參數化；首次登入restricted與Session落庫契約保留，fixture只共用穩定準備。 |

[12組測試覆蓋對照](test-coverage-map.csv)逐項列舊測試、重複判斷、保留／合併／遷移方案與目標責任；目前全部未套用／未新跑。這是本切片的靜態盤點，不宣稱全庫測試已掃完或重構完成。

## 規則與驗證責任

- [共用規則：test-refactor-v1](../../COMMON_LEARNING_RULES.md)是測試重構政策正本。
- [架構責任](../../SYSTEM_ARCHITECTURE.md)區分Domain、Application、SQLx、Delivery／native與UI測試。
- [教學交付檢核](../../TEACHING_ALIGNMENT.md)要求同篇提供必要完整程式、測試、fixture、Cargo／feature與CI接線。
- [既有功能後續條件](../../FUTURE_WORK.md)沿原項目追蹤，不新增平行FIFO。
- [CI責任分析](../../source-review/2026-10-26-ci-layer-boundaries.md)要求三案獨立、資料與原生分開，保留失敗訊號；補系統套件不等於已完成分層。
- [當日版本與差異](../../source-review/2026-10-26-versions-and-summary-refresh.md)保留各次來源與核對日期；後續換日期或新增依賴須重查。

## 完成界線

此次完成的是文件整理與GitHub發布。新用例／Port／Adapter、正式caller及完整測試重構Green仍未實作，沒有新增DB／HTTP／IPC／GUI驗收。下一次按即時進度與最新來源選最小功能切片，不把本頁當成已交付的新教學日。

[現行狀態](../current/STATUS.md) · [來源與整理紀錄](evidence.json)
