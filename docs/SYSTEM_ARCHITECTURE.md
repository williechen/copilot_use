# 系統架構與責任

唯一產品為 rust_ai_ring，文件與教學為 copilot_use。現行來源 `2c7ff8b861d32ab6d879bf80c9f2dce50de38097`；目錄與相依見 [SOURCE_LAYOUT](SOURCE_LAYOUT.md)。以下分層是教學／工程約束，不代表現有所有 handler 已完成搬層。

<a id="functional-layering"></a>

| 邊界 | 責任 |
| --- | --- |
| Leptos UI | 元件、表單、可見狀態與 API／IPC 互動；不直接查 DB |
| Axum／Tauri Delivery | 協定、身分上下文、輸入錯誤與強制授權，呼叫用例 |
| Application | 用例、授權決策、交易／補償意圖及 Port／DTO；不依賴 UI／driver |
| Domain | 規則、值物件、狀態機與不變條件 |
| Persistence／原生 Adapter | SQLx、migration、交易、視窗／OS／模型的具體實作 |
| Composition root | 建立 pool／adapter、注入用例並組裝啟動 |

Delivery→Application→Domain；Adapter 實作內層 Port。Chat 已有 Repository／用例與 Session 入口決策；Pet 有 Domain、名稱規則、生命週期及部分 persistence；Analytics 有 PG、登入 Session、Leptos Dashboard 與部分聚合 Adapter。仍有 handler／Store SQL、legacy/global 入口及範本庫待按正式 caller 整理。共用 shared 仍直接依賴 chat-domain，不能宣稱跨 workspace 完全解耦。

測試分純規則／用例、真 DB Adapter、正式 Router／IPC、UI、瀏覽器及 native；不同邊界的有效斷言保留。metadata／check 不替代 DB／GUI；CI 綠燈也不替代產品旅程驗收。待辦見 [FUTURE_WORK](FUTURE_WORK.md)，跨案工作只列 [engineering](engineering/README.md)。

Pet活動模型已在main：目前籠態由pet_activity導出，預設沿用既有profile欄位；Application精確snapshot補償，Native與UI逐ID接線。資料舊標籤forward升級已在2c7ff8b並以主分支重驗112項，不新增欄位。native視覺旅程仍待驗；見[STATUS](teaching/current/STATUS.md)。

[Pet 2c7ff8b採用後最新驗證](source-review/pet-day44-adopted-2026-10-30/README.md)；既有Rust單元結果保留4b1008b來源；本輪2c7ff8b原生／瀏覽器新證據另列，不追認舊測試。

[2026-10-31 Gate2原生／HTTP實測與已確認阻擋](source-review/gate2-2026-10-31/README.md)；目前完整Gate2=false，不以CI全綠替代逐房／角色授權或缺失的舊庫／實機證據。
