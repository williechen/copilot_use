# 系統架構與責任

唯一產品為 rust_ai_ring，文件與教學為 copilot_use。現行來源 `6b34c42f74fc1585b1b864fb7dd6969318270e0f`；目錄與相依見 [SOURCE_LAYOUT](SOURCE_LAYOUT.md)。以下分層是教學／工程約束，不代表現有所有 handler 已完成搬層。

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

Pet活動r3在隔離候選將現在籠態移到既有pet_activity；Application保留純snapshot補償，Adapter提供精確活動／選取還原，Native與UI逐ID接線。這是已實測文件方案，不能當main已完成搬層；[目前矩陣](teaching/current/STATUS.md)區分兩者。
