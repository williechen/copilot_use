# Analytics 傳輸目標與主體核對

現行程式來源與目錄：[rust_ai_ring 四個 workspace 對照](SOURCE_LAYOUT.md)（來源 `0b3e5844`，2026-10-24 靜態核對）。

`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_ring` 是唯一程式、測試、CI 與功能驗收主體。 本頁為需求與驗收方向，不代表 rust_ai_ring 已具備舊庫的 gRPC／Outbox／SSE 實作。

目標保留：Chat／Pet 與 Analytics 的事件 ID／schema、來源服務身分、重試／去重、交易提交後 ACK、HTTP／gRPC／SSE 契約與人員 Session／角色授權分離。實際採用的路由、型別、SQLx Adapter、傳輸與事件名稱須逐項依 rust_ai_ring 精確 SHA 核對，並提供真實 UI／呼叫者、DB／HTTP／故障恢復及該庫 CI 證據。

尚無實際呼叫鏈或匹配測試的目標列為未接通，不以記憶體假成功、舊 501 或 copilot_use 已實作能力當成主體現況。契約變更保留版本與原證據，必要更正按後續 Day 規則處理。

[原 copilot_use 傳輸文件](teaching/history/evidence/2026-10-23-repository-roles/before/docs/ANALYTICS_TRANSPORT.md) 保留固定來源的契約與結果；需求詳見[現行使用者流程](requirements/USER_FLOWS.md)、[UI 驗收條件](UI_ALIGNMENT_REQUIREMENTS.md)、[工作項目](FUTURE_WORK.md)、[服務憑證要求](SERVICE_CREDENTIAL_REQUIREMENTS.md)。
