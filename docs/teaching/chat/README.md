# Chat 首頁

每日文章另設獨立 Refactor 段落，交付該篇專案的實際重構程式、測試承接與驗證；全專案重構另列獨立項目。核心60分鐘只計Red／Green；rust_ai_ring唯讀，使用者自行套用。

[共用規則](../../COMMON_LEARNING_RULES.md) · [功能與驗證](../current/STATUS.md) · [工程進度](../functional-refactoring/PROGRESS.md) · [學習進度](../../learning-state.json) · [manifest](../current/manifest.json)

| 專案 | 現行教學 | 下一篇 | 驗證範圍 |
| --- | --- | --- | --- |
| Chat | [Day38 r4：搜尋失敗可見狀態與既有房間連結](../daily/day-38/chat-r4.md) | Day39 | 候選 SSR 14項通過＋WASM check；產品未套用，GUI／候選CI未驗 |

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


[現行文章入口](../current/CHAT_ENTRY.md) · [歷史及修訂總表](../current/ARTICLE_STATUS.md)。實作與驗收僅在現況頁集中維護。

[全專案重構獨立項目](../../../docs/engineering/README.md) · [候選本地驗證](../../../docs/teaching/daily/day-38/r4-local-validation.json)。
