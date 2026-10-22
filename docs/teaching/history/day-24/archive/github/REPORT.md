# Day24 完整發布與條件式進度同步驗證

三篇皆以正式完整文章寫入 Notion，包含四節與完整適用 patch；並非簡短測試內容。全文比對僅容許 Notion 行內格式／語言標籤與換行序列化差異，程式內容另外逐字核對。

| 專案 | 修訂 | 文章字元 | 測試 | next_day |
|---|---|---:|---|---|
| Chat | r1 | 19,347 | 5 通過、SSR通過 | 25 |
| Pet | r2 | 32,546 | 46 通過、三隻原生生活窗／獨立狀態／單隻回籠 | 25 |
| Analytics | r1 | 26,240 | 30 PostgreSQL測試、真HTTP與程序重啟 | 25 |

- [Chat Day24](https://app.notion.com/p/3ffe51d4509381ee9326ce6cc7f1c8a1)
- [Pet Day24 r2](https://app.notion.com/p/3ffe51d450938186b15fd1c014df8edd)
- [Analytics Day24](https://app.notion.com/p/3ffe51d4509381c88f26f4a90c2958fc)

Pet依UUID動態建窗，沒有兩隻硬上限；三窗同時實測，另以12個UUID測標籤分離。重複出籠只聚焦，單隻睡覺／回籠不影響另外兩隻，SQLite資料保留。r1原文／patch／證據保留在revisions/pet-r1，同一頁r2修訂沒有增加Day。尚未完成多個啟動預設、每隻持久FSM／漫遊、完整素材與跨平台；目前重啟只自動載入一隻明確預設。

來源：Chat c965681eeb2b4791dba8d157624424b7f274b2fd；Pet／Analytics 4f0cd00b8d571bdaf120298efebf0d62b795498f。中途main僅更新root AI migration，Pet依新來源重驗，Chat保留原交付SHA。Chat舊來源的fresh migration失敗如實留在該篇，不能當成新版main仍失敗或已重新全面驗收。

發布gate共13項測試，拒絕短文、截斷、程式內容差異、索引缺漏與進度衝突；逐案寫後已從遠端讀回25，對真實state重試保持不變。Day23三篇全文及程式比對保留，Day23以前未修改；makeup_queue、last_full_comparison與其他既有state欄位保持原值。

rust_ai_chat來源唯讀，原README dirty diff仍43新增／227刪除、未追蹤docs保留；本次只安全fast-forward main。教學程式在隔離副本實作，完整修改發布copilot_use，未套用產品原始碼。使用者套用、產品整合與完整驗收仍與教學交付分開。

本次專用PostgreSQL已停止並刪除，Pet隔離SQLite已刪除；系統5432與copilot_use資料保留。正式排程未恢復、未新增。精確逐篇讀回與條件同步結果見delivery-verification.json及三份completion/readback JSON；原始測試／GUI／CI／版本資料在各案validation。
