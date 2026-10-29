# Chat｜教學與問題入口

> 文件保存規則：已驗證的 Day43 教學正文依指定 [只留既有 ZIP](../daily/day-43/README.md)；未驗證原稿、規則、入口、狀態及證據保留原檔。Day44 尚無原稿，未補造；其後未驗證原稿不刪除。

> **2026-10-30 SSR 修補追驗：** 產品已更新至 `7ce39f0166a2120539e5908b33c88322a930c35a`，正式 main 已套用 Router imports。隔離原樣測試 UI SSR 21 項、server／SQLite／UI SSR 組合 60 項及 metadata／fmt 通過。[新 SHA 命令與 CI 快照](../../source-review/day43-7ce39f0-2026-10-30/README.md)；遠端 CI 仍執行中，未稱全綠。其他 workspace 與 GUI／WASM／DB 保留各自原 SHA 的證據；next_day 三案 44、FIFO、last_full_comparison 不變，完整 Gate2 未結案。下方較早日期與 SHA 為歷史快照。

> 2026-10-29 驗收增量：[網頁排程已儲存、GUI／CI／舊庫實測](../../source-review/acceptance-2026-10-29/README.md)。Chat 真瀏覽器另發現清除搜尋回歸，修補候選已通過；CI attempt #2 仍失敗，Pet 隔離 A/B 原生回籠生命週期已由人工操作及 SQLite 核對通過，其他原生情境／真實舊 PG 備份待驗。Gate2 未結案。

> 2026-10-29 Day43 本地實測：固定產品 `8b882696c5548accb9d98a32b27db35e11109c74`；[測試矩陣與原始證據](../../source-review/local-validation-2026-10-29-day43/README.md) · [今日交付延遲原因](../../source-review/local-validation-2026-10-29-day43/diagnosis.md)。下列舊 SHA 結果保留原範圍；next_day 維持 44，完整 GUI／CI／舊 DB 驗收未完成。

產品固定來源 `c6708c42f21ac919e57a379848b4977d27a1c3d6`；下一待發布篇 Day44；Day43 文件已正式發布，但產品 Day43 patch 未套用、Gate2 尚未通過。

| 閱讀目的 | 入口 |
|---|---|
| **最新 Day43：房間搜尋一鍵清除／Session 入口決策搬層** | [正式文章](../daily/day-43/chat-archive.md) · [完整 Refactor / Green patch](../daily/day-43/README.md) |
| Day42 原始教學與舊驗證 | [原始 Day42](../daily/day-42/chat.md) · [修訂索引](../daily/day-42/README.md) |
| 本案問題與原完成條件 | [問題追蹤](../../FUTURE_WORK.md#chat-auth) |
| 目前測試及驗收限制 | [功能狀態](../current/STATUS.md) · [本輪原始命令與結果](../../source-review/local-validation-2026-10-28-c6708c4/README.md) |
| 文章勘誤與舊文判讀 | [問題文章索引](../current/ARTICLE_CORRECTIONS.md) |

新 SHA：server／SQLite／SSR 57 項通過（含 UI 18）。

## 專案大綱與操作主線

建立多人、多房間的聊天系統，先讓使用者能建立房間、找到房間、選房並聊天，再逐步接上身分、權限、歷史與管理。

前台主線：公開房間清單 → 點房名 → 必要時登入 → 伺服器核對房間權限 → 進房收發 → 重進讀歷史 → Analytics。首頁登入／管理另導向個人頁；註冊完成維持登出，從獨立訊息頁按回首頁後重新選房。這是分段完成方向。

後台主線：系統管理者管理 AI 聊天者、每人可開房間數、公告與管理者帳號。房間管理與系統管理分開，不能把房主當成系統管理者。

## 現行需求

[三專案使用者流程](../../requirements/USER_FLOWS.md) · [共用人員驗證](../../requirements/AUTHENTICATION.md) · [UI 詳細驗收](../../UI_ALIGNMENT_REQUIREMENTS.md)。共用條款統一維護，不在各首頁重複。

## 功能目標與完成條件

- 基本聊天：從真實首頁進入房間，兩個瀏覽器可在同房收發；不同房間不串訊息。SQLite 寫入失敗不能先發送成功廣播。
- 房間管理：只操作自己有權管理的房間；成員停用／恢復限定該房間。房主交接後失去管理權並保留一般聊天資格，AI 設定仍由系統管理者管理。
- 歷史與帳號：重啟後可讀回已保存訊息；刪除帳號前處理房主交接或封存，保留應保留的聊天紀錄。
- 後續能力：Session／ACL、presence／typing／reconnect、Outbox、備份，接著才逐步加入 streaming、RAG／MCP／Agent、多 Persona 與同意／撤回。

## 介面與接線要求

#### 聊天前台與管理入口

聊天前台使用 Leptos，採左側房間欄、中央訊息區、底部輸入區；個人管理與系統後台使用各自頁面／路由及後端權限，不塞進聊天訊息區。

- 首頁只顯示聊天室清單與必要狀態；建立／管理房間放個人管理頁，點選清單後進入聊天頁。
- 房間欄：顯示可存取房間與目前選取狀態，沿用既有列表 API；窄螢幕時可收合，切換不應把草稿送往錯誤房間。
- 訊息區：房間名稱與連線狀態固定可見，訊息以氣泡與適當寬度呈現；辨識自己、其他使用者、AI／系統。作者、時間與日期分隔只使用實際資料，缺少時明示或省略，不編造頭像、線上人數或歷史紀錄。
- 輸入區：多行輸入、送出與清楚的鍵盤操作，處理中文輸入法組字；空白／送出中禁用，保留失敗草稿。傳送中、失敗／重試等狀態須由真實協定與回應支持，不將本地送出成功冒稱伺服器已持久化。
- 使用回饋：載入、空房間引導、錯誤及重新連線狀態清楚可辨；歷史載入或重連若尚無完整後端能力，明示未接通，不顯示沒有作用的按鈕。
- 視覺與可用性：柔和背景、清楚留白與一致字體；訊息區獨立捲動，輸入區不被長訊息擠出，長文字／網址可換行。不能只靠顏色區分作者；鍵盤焦點、對比、窄螢幕與系統縮放均需驗證。

驗收從真實房間列表進房，測同房收發／跨房隔離、切房與草稿、長訊息、空／錯誤狀態、斷線、中文多行輸入及窄螢幕；無訊息確認或重試冪等能力時不能宣稱傳送狀態已完整實作。完整瀏覽器互動仍須按上述條件驗收。

## 驗證範圍

已核驗來源與測試範圍統一見GitHub [目前進度](../current/STATUS.md)。完整功能驗收仍須由真實入口逐段測試；單元測試、資料寫入或建置通過不能代替使用者流程驗收。

## 開發位置

`rust_ai_ring/chat_apps/chat_server` 負責 HTTP／WS，`chat_ui` 負責 Leptos 畫面，`chat_domain`／`chat_application` 負責規則與 Port，`chat_persistence` 負責 SQLite Adapter；migration 位於 `chat_apps/migrations`。Cargo package 使用 `chat-server`、`chat-ui`、`chat-persistence` 等實際名稱。

先讀相關功能教學的「學習大綱」確認前置能力；第2節提供Red／Green，獨立Refactor段落提供Chat自身累積重構，分別套用與驗證，再依第4節檢查真實入口。已實作的舊篇用來追溯，不照順序把所有舊程式再貼一次。

[產品操作設定](https://github.com/williechen/copilot_use/blob/main/docs/DEPLOYMENT.md) · [產品未來清單](https://github.com/williechen/copilot_use/blob/main/docs/FUTURE_WORK.md)；兩者是文件庫的需求／操作入口；主體程式、命令與驗收只依 rust_ai_ring 當時 Cargo／README／精確 SHA。

## 歷史與工程

[每日文章總目錄](../daily/README.md) · [歷史文章與修訂](../current/ARTICLE_STATUS.md) · [各案 Refactor 進度](../functional-refactoring/PROGRESS.md) · [跨案工程](../../engineering/README.md)

歷史測試維持原 SHA；跨案工程不混入本案 Refactor。
