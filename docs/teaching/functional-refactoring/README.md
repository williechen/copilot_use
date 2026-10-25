# 全專案架構、測試與CI調整

2026-10-26確認：重構是全專案累積工程，每天逐步調整，放在同篇獨立專案Refactor段落，不作當日功能教學主題。範圍涵蓋Chat、Pet、Analytics與共用crates／跨workspace依賴；不新增平行產品或固定教學日。

目前狀態：**範圍與驗收整理，產品重構尚未執行。** 來源為 `rust_ai_ring@9b89656f964b99e0cb89ba421855704a35c5ecb9`；此次只改文件與每日任務提示詞，未改產品、測試、DB、lock或workflow，教學進度保持原值。

## 全專案工程範圍

| 範圍 | 工作責任 |
| --- | --- |
| [Chat](chat.md) | 用例／認證／Session與SQLx邊界、正式HTTP／WS／UI、房間狀態與有效測試接線 |
| [Pet](pet.md) | 生命週期協調、儲存／視窗Port、SQLite／Tauri Adapter、Runtime／鎖／補償與原生IPC |
| [Analytics](analytics.md) | Session／授權／查詢用例、PG Adapter與DTO、正式Router／Leptos、角色與來源ACL缺口分別追蹤 |
| 共用crates與依賴 | shared跨workspace依賴、公開契約／DTO歸屬、normal／build／dev／target依賴與組裝位置 |
| 測試與fixture | [12組既有測試對照](test-coverage-map.csv)作工程盤點起點；擴充全專案契約映射，同層去重、跨層責任保留，避免fixture複製業務實作 |
| CI與驗收 | 純內層、三案真DB、正式HTTP／IPC、Pet原生、UI／產物分責任；無實際相依不串接，失敗仍反映於整體結果 |

## 每日工程節奏

先核對三案各自進度與專案整體依賴，每天完成一個完整小工作包，接續前日未完成項；不隨當日切片換題。跨案包共用證據且逐案標記影響，不重複計功；各案未推進則如實記錄；獨立記錄程式／測試／CI變更內容與驗證，方案交付、產品套用及產品驗收分列，不占用核心教學60分鐘。受阻時保留原工作包與原因，選可獨立推進的工程項；不能漏掉當日三案教學。

[工程進度與接續位置](PROGRESS.md)維護同一工程的狀態，不另建教學Day或平行FIFO。

## 工程順序與完成條件

1. 盤點全專案來源、公開契約、依賴與caller、測試／fixture及CI覆蓋，保留現行有效基準。
2. 依相依性確定Port／DTO／錯誤與相容入口，再重接Adapter、組裝與正式caller，避免循環依賴或兩套平行實作。
3. 同步遷移／整併測試與fixture；有效斷言先有承接才清舊實作。不能以測試數下降或刪測試換全綠。
4. 同步CI job、features、services／native依賴、trigger與required checks；分別記錄精確SHA的內層／DB／HTTP／IPC／native／UI結果。
5. 列實際變更、相容性、風險、回復方式、未驗證與失敗。文件規劃不是工程驗收，單一crate通過不代表全專案完成。

這是同一全專案工程的相依順序；每日選一個可完成的小工作包，不是教學日分配或新FIFO。資料升級須保資料並另具驗證，不能用fresh DB取代舊DB相容性證據。

## 與每日教學分工

每日功能教學提供可操作功能的Red／完整Green與直接相關測試。Refactor接續各專案整體工程：同篇設與Red／Green並列的獨立段落，提供累積狀態、本次完整程式／patch、測試遷移、實際驗證與下一步；共用附件與跨日進度仍集中本入口，不強制另拆文章；切片小整理併Green說明，不能抵算工程。若功能依賴工程變更須明列。工程本身不增加Day、不改last_full_comparison；保留已發布文章與證據。

[共用規則](../../COMMON_LEARNING_RULES.md) · [架構責任](../../SYSTEM_ARCHITECTURE.md#functional-layering) · [教學與工程銜接](../../TEACHING_ALIGNMENT.md) · [既有功能項](../../FUTURE_WORK.md) · [CI分析](../../source-review/2026-10-26-ci-layer-boundaries.md) · [版本紀錄](../../source-review/2026-10-26-versions-and-summary-refresh.md)
