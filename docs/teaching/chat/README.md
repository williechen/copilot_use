# Chat 首頁

<!-- comparison-5c713553:start -->
[最新三專案程式／文章比對](../../source-review/2026-10-25-full-comparison-5c713553.md)：105個閱讀版本、111份修訂、21項差異；來源 `5c713553`，局部測試與完整驗收分開記錄。
<!-- comparison-5c713553:end -->

[問題文章與後續更正總表](../current/ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

<!-- article-status:start -->
[全部文章現存狀態（105個閱讀版本／111份修訂）](../current/ARTICLE_STATUS.md)：原文、程式所見與歷史／目前驗證分開記錄；已由後續實作取代者勿原樣套回。
<!-- article-status:end -->

<!-- duplicate-review:start -->
[後續調整清單（13 項）](../../FUTURE_WORK.md) · [原始核對報告](../../source-review/2026-10-25-duplicate-application-review.md)。可清除、先遷測試與保留項目分別列明，產品套用／驗收另記。
<!-- duplicate-review:end -->

<!-- daily-delivery:start -->
## 最新交付與下一篇

| 專案 | 最新文章 | 下一篇 | 狀態 |
| --- | --- | --- | --- |
| Chat | [Day37 r2：房間搜尋完成時更新真實畫面狀態](../daily/day-37/chat-r2.md) | Day38 | 教學已交付／程式未套用／新測試未執行／產品未驗收 |
| Pet | [Day37 r2：先保存再開窗，失敗回復原標記](../daily/day-37/pet-r2.md) | Day38 | 教學已交付／程式未套用／新測試未執行／產品未驗收 |
| Analytics | [Day36 r2：精確事件統計 API 改用人員 Session，不再接受來源 token 查詢](../daily/day-36/analytics-r2.md) | Day37 | 程式已存在／相關實測通過／完整產品未驗收 |

文案修復與最新 PG 結果見 [afdbb6e 驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-assertions-afdbb6e.md)；前次編譯修復見 [bb95a60 驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-compile-bb95a60.md)；前次套用／測試見 [e753c36 更新驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-update-e753c36.md)；原 Day35 正文與交付證據保留，完整產品驗收尚未完成。
<!-- daily-delivery:end -->

2026-10-24 目錄已同步至 [williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)：保留 `chat_apps`、`pet_apps`、`analytics_apps`、`crates` 四個 workspace；[新目錄與套件對照](../../SOURCE_LAYOUT.md)。後續教學依當次來源 SHA 調整程式、Cargo、imports、呼叫端與測試。

[共用文章](../../COMMON_LEARNING_RULES.md) · [三專案首頁](../README.md)

## 目前狀態

[單一目前進度](../current/STATUS.md) · [即時進度檔](../../learning-state.json)。本文的需求不代表已完成；現況依功能與驗證範圍說明。

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

先讀相關功能教學的「學習大綱」確認前置能力，再按第 2 節套用與 Refactor，最後照第 4 節從真實入口驗證。已實作的舊篇用來追溯，不照順序把所有舊程式再貼一次。

[產品操作設定](https://github.com/williechen/copilot_use/blob/main/docs/DEPLOYMENT.md) · [產品未來清單](https://github.com/williechen/copilot_use/blob/main/docs/FUTURE_WORK.md)；兩者是文件庫的需求／操作入口；主體程式、命令與驗收只依 rust_ai_ring 當時 Cargo／README／精確 SHA。

## 功能驗證現況

個人頁建房與訊息廣播沿真實入口驗證；Day32歷史未接通提示與空狀態已加入，UI SSR12/12通過。歷史讀回及作者身分仍未完成。

[完整現況](../current/STATUS.md) · [本輪報告](../history/evidence/2026-10-24-formal-f3f8255/REPORT.md) · [歷史文章與原始證據](../history/README.md)

[Day31教學：個人頁建房](../daily/day-31/chat.md)（個人頁建房切片已套用並通過本輪瀏覽器操作；整體驗證仍未全部通過。）


## 教學與現存狀態

完整歷史及目前程式／驗證狀態見 [逐篇狀態](../current/ARTICLE_STATUS.md)；最新交付與下一篇仍依頁首及 learning-state。舊文章原文保留當時語境，後續修訂與程式套用另列，不再沿用過時「尚未套用」作為現在的判定。
