# 主體程式呼叫鏈核對

`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_chat` 是唯一程式、測試、CI 與功能驗收主體。 本頁規範核對方法，不把舊庫程式的靜態對照當 rust_ai_chat 已接通或已驗收證據。

| 使用者起點 | 每次須在 rust_ai_chat 核對的實際鏈 |
| --- | --- |
| Chat 註冊／登入／登出／進房 | Leptos 元件／路由 → Axum Handler／Cookie／Session／ACL → 使用案例／Port → SQLx SQLite → UI 回饋 |
| Chat 訊息／即時操作 | 真 UI 呼叫 → HTTP／WS → 房間／作者權限 → 持久化與通知；逐項檢查錯誤、隔離及重連 |
| Pet 建立／載入／互動 | Leptos CSR → Tauri command → pet id 與使用案例 → pet_store／SQLite／原生窗 → 回應／事件與重新載入 |
| Pet 檔案操作 | 授權／preview → 明確確認 → 真 OS 操作 → receipt／狀態與持久化；純狀態成功不當真操作成功 |
| Analytics 登入／改密碼／管理 | UI → Axum HTTP／Session／操作權限 → 使用案例／SQLx PostgreSQL → 重啟／撤銷／UI 回饋 |
| 跨案事件 | 只依主體實際 HTTP／傳輸、ID／schema、來源驗證與交易／去重；尚無真實呼叫者的目標列未接通 |

[系統架構](SYSTEM_ARCHITECTURE.md) 提供定點來源入口；執行時重讀最新 source。Leptos、Axum、Application／Domain、SQLx 與 Tauri 邊界要求依 [共用規則](COMMON_LEARNING_RULES.md)。本次文件同步未執行上述流程的新增測試。

舊 copilot_use 模組表[原文保留](teaching/history/evidence/2026-10-23-repository-roles/before/docs/SEMANTIC_ARCHITECTURE_REVIEW.md)；[完整雙庫比較](FULL_SOURCE_REVIEW.md) 只保留原固定 SHA 的歷史事實，不作現行產品適配任務或主體功能結論。
