# Notion 教學與兩庫程式對照

核對日期：2026-10-17（Asia/Taipei）。唯讀基準：rust_ai_chat `3e58e650bd63bcd21ba58e7aabf751542401ef80`、copilot_use `9f632c76ee432d77c1d20f46da9485d982d49c9e`。

目前判定：已有部分功能整合與不同介面的對應實作，不能說完全未同步；也不能宣稱所有文章、程式與測試已逐項同步驗收完成。

| 教學 | rust_ai_chat 實際程式與測試 | copilot_use 對應程式 | 本次判定 |
|---|---|---|---|
| [Chat Day14：HTTP 讀回房間](https://app.notion.com/p/3fbe51d450938149af5dd68abc6109bd) | apps/server/src/http.rs::get_room／RoomDetails、lib.rs GET /api/rooms/{id}、room_details_tests 已存在；crates/ui/src/room_api.rs 僅有 create_room，未呼叫新讀取 API | apps/server/src/main.rs::list_rooms、RoomItem、GET /api/v1/rooms 提供有權限的房間清單；crates/ui/src/management.rs 有清單呼叫 | 功能方向有對應，介面與授權不同，不能以函式名不同判為缺失。參考庫 Day14 實作已存在，前台讀取接線仍待補；本輪沒有執行該庫測試 |
| [Pet Day14：主動掃描與摘要](https://app.notion.com/p/3fbe51d4509381e484f4eb9e1f98fc68) | frontend main.rs 按鈕→bridge.rs→Tauri scan_file_preview；file_scan_view.rs 有數量與隱私測試；載入互斥已存在 | frontend main.rs::Organizer→scan_file_preview；desktop_pet/src/main.rs::scan_file_preview→AuthorizedRoot::scan；另有 SHA256 預覽及 duplicates.rs 測試 | 兩庫都有可操作掃描流程，但回應契約不同：參考庫三項摘要，成品檔案清單。不能宣稱逐欄相同或未整合掃描功能；成品 SHA256 已存在，舊 interface-catalog 的「尚未對應」敘述已過時 |
| [Analytics Day14：事件類型篩選](https://app.notion.com/p/3fbe51d4509381f2a9a2e4b608706eea) | lib.rs::source_kind_summary→pg_store.rs::summary_for_source_and_type，以來源憑證及 SQL bind 查詢；source_kind_summary_tests 已存在 | EventStore::summary 依來源／類型聚合；GET /api/v1/events/recent 可帶來源／類型，summary 採獨立查詢 Token | 聚合／來源類型資料已有對應，Day14 的單一來源類型 summary URL 並未照搬。是否需相同 API 必須按產品需求判定，不可直接複製來源認證模型。參考庫新增 DB 單測受 postgres-integration feature 控制，Persistence CI 只跑 postgres_event_store target，不能當作這組單測已跑 |

## 另查到的流程缺口

rust_ai_chat 的 persisted_message_event 與 succeeded_event 已有定義和契約測試，但查得的呼叫端限測試，未接到實際跨服務上報流程。copilot_use 另有 Outbox／傳輸實作；不能用成品的成功證明學習庫已完成接線。

## 盤點範圍及限制

已取得三個專案主頁連結的 24 篇文章，以及封存頁列出的 Day2–8 共21篇，共45篇；取得全文並建立文章宣告符號對應來源檔案／行號。這份符號索引是定位輔助，不能代替逐項行為、呼叫端、斷言或執行驗證。Day1 未在這些索引中取得；不宣稱所有歷史／已取消連結頁均已核對。

本報告深入核對上表 Day14 的實作、實際呼叫及測試位置；其他歷史篇目前僅完成全文及符號索引，不宣稱45篇全數語意與執行驗收通過。舊篇章的函式名、測試去重與後續替代行為仍須逐項判斷。

本地 current 教學鏡像仍是 Day13，與 Notion Day14 不同步；learning-state next_day 為15，但 next_formal_day 仍為9，其欄位語意與使用端尚待核實，不能直接拿舊欄位排課。此次未修改 Notion 教學、任一庫原始碼、測試或 migration，也未重新執行測試或遠端 CI。

後續核對必須保持：文章要求 → rust_ai_chat 實作與呼叫端／測試 → copilot_use 對應實作與差異 → 精確來源驗證。測試搬移屬 Day15 起可分批納入的 Refactor，不是新功能，也不回填 Day13。
