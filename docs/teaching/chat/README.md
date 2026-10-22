# Chat 首頁

[共用文章](../../COMMON_LEARNING_RULES.md) · [三專案首頁](../README.md)

## 目前狀態

[單一目前進度](../current/STATUS.md) · [即時進度檔](../../learning-state.json)。本文的需求不代表已完成；不在首頁複製過期的 Day 或測試快照。

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

學習庫：apps/server 負責 HTTP／WS，crates/ui 負責畫面，chat\_domain／chat\_application 負責規則與 Port，crates/persistence 負責 SQLite／migration；資料庫差異留在 Adapter。

先讀目前 Day 的「學習大綱」確認前置能力，再按第 2 節套用與 Refactor，最後照第 4 節從真實入口驗證。已實作的舊篇用來追溯，不照順序把所有舊程式再貼一次。

[產品操作設定](https://github.com/williechen/copilot_use/blob/main/docs/DEPLOYMENT.md) · [產品未來清單](https://github.com/williechen/copilot_use/blob/main/docs/FUTURE_WORK.md)；兩者是文件庫的需求／操作入口；主體程式、命令與驗收只依 rust_ai_chat 當時 Cargo／README／精確 SHA。


## 目前教學

- **最新已發布：** [Chat Day 27](../daily/day-27/chat.md)（教學文章與程式套用／驗收分開）
- 前一篇：[Chat Day 26](../daily/day-26/chat.md)
- **下一篇 Day：** 以 [learning-state.json](../../learning-state.json) 的 `next_day.chat` 即時值為準；不依首頁快照或直接重發 Day27。

<details>
<summary>歷史文章、修訂與原始封存證據</summary>

| Day | 文章與原始資料 |
| --- | --- |
| 25 | [文章／版本／證據](../history/day-25/chat/README.md) |
| 24 | [文章／版本／證據](../history/day-24/chat/README.md) |
| 23 | [文章／版本／證據](../history/day-23/chat/README.md) |
| 22 | [文章／版本／證據](../history/day-22/chat/README.md) |
| 21 | [文章／版本／證據](../history/day-21/chat/README.md) |
| 20 | [文章／版本／證據](../history/day-20/chat/README.md) |
| 19 | [文章／版本／證據](../history/day-19/chat/README.md) |
| 18 | [文章／版本／證據](../history/day-18/chat/README.md) |
| 17 | [文章／版本／證據](../history/day-17/chat/README.md) |
| 16 | [文章／版本／證據](../history/day-16/chat/README.md) |
| 15 | [文章／版本／證據](../history/day-15/chat/README.md) |
| 14 | [文章／版本／證據](../history/day-14/chat/README.md) |
| 13 | [文章／版本／證據](../history/day-13/chat/README.md) |
| 12 | [文章／版本／證據](../history/day-12/chat/README.md) |
| 11 | [文章／版本／證據](../history/day-11/chat/README.md) |
| 10 | [文章／版本／證據](../history/day-10/chat/README.md) |
| 9 | [文章／版本／證據](../history/day-09/chat/README.md) |
| 8 | [文章／版本／證據](../history/day-08/chat/README.md) |
| 7 | [文章／版本／證據](../history/day-07/chat/README.md) |
| 6 | [文章／版本／證據](../history/day-06/chat/README.md) |
| 5 | [文章／版本／證據](../history/day-05/chat/README.md) |
| 4 | [文章／版本／證據](../history/day-04/chat/README.md) |
| 3 | [文章／版本／證據](../history/day-03/chat/README.md) |
| 2 | [文章／版本／證據](../history/day-02/chat/README.md) |

</details>
