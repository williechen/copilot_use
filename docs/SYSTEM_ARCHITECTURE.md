# 系統架構與責任

唯一產品為 rust_ai_ring，文件與教學為 copilot_use。現行來源 `f6a04015188da2155af6c264d0e6326e6a922dca`；目錄與相依見 [SOURCE_LAYOUT](SOURCE_LAYOUT.md)。以下分層是教學／工程約束，不代表現有所有 handler 已完成搬層。

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

Pet活動模型已在main：目前籠態由pet_activity導出，預設沿用既有profile欄位；Application精確snapshot補償，Native與UI逐ID接線。資料舊標籤forward升級已在2c7ff8b並以主分支重驗112項，不新增欄位。隔離native25步已在2c7ff8b通過，實體桌面仍待驗；見[STATUS](teaching/current/STATUS.md)。

[Pet 2c7ff8b採用後最新驗證](source-review/pet-day44-adopted-2026-10-30/README.md)；既有Rust單元結果保留4b1008b來源；本輪2c7ff8b原生／瀏覽器新證據另列，不追認舊測試。

[2026-10-31 Gate2原生／HTTP實測與已確認阻擋](source-review/gate2-2026-10-31/README.md)；目前完整Gate2=false，不以CI全綠替代逐房／角色授權或缺失的舊庫／實機證據。

[64eff52 SQL更新後最新回歸](source-review/gate2-sql-2026-10-31/README.md)：Pet112／Node12／WASM及四workspace metadata通過；舊native／browser證據仍屬2c7ff8b，完整Gate2未通過。

## Analytics ACL 階層契約

`uac_role`／`uac_user_role` 負責角色分類與指派，`uac_acl` 以既有 `acl_parent_id` 構成 category→page→action。三個category為會員管理、資訊分析、認證管理；action判斷CRUD。`uac_role_acl`為角色預設，`uac_user_acl`提供個人獨立加減；page可見性與CRUD操作分別檢查。[完整需求](requirements/ANALYTICS_ACL.md)。目前僅交付候選patch，不能宣稱產品main已具此分層或權限管理UI。

目前63b1f93只新增Analytics ACL schema：analytics-token認證管理、密碼page歸會員管理、security管理憑證page；其餘runtime仍沿原來源，page／CRUD Adapter尚未採用。[採用證據與限制](source-review/analytics-adoption-2026-10-31-63b1f93/README.md)。舊測試保留原SHA，不追認新來源。

最新f6a0401只調整schema角色預設，管理憑證page grant已補；自身密碼ACL page保留，首次／後續登入預設頁與共用實作須依更正契約核對。[採用核對](source-review/analytics-adoption-2026-10-31-f6a0401/README.md)。歷史測試保留原SHA。
