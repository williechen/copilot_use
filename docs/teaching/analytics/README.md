# Analytics 首頁

> 本頁需求與教學交付保存在 copilot_use；程式入口、套件、測試、CI 與功能驗收只依 rust_ai_chat 精確 SHA，舊庫產品工具不作主體驗收。

[共用文章](../../COMMON_LEARNING_RULES.md) · [三專案首頁](../README.md)

## 目前狀態

[單一目前進度](../current/STATUS.md) · [即時進度檔](../../learning-state.json)。本文的需求不代表已完成；現況依功能與驗證範圍說明。

## 專案大綱與操作主線

將 Chat 與 Pet 的真實動作轉成可追溯統計，供使用者查詢、篩選與管理。Analytics 提供觀察與建議，Chat／Pet 是否採用仍須各自授權。

主線：真實動作 → 來源認證 → Schema 驗證 → 去重／衝突處理 → PostgreSQL 提交 → Dashboard 查詢／篩選 → 趨勢與明細 → 管理／封存／備份。資料提交與畫面出現是不同步驟，要各有證據。

## 現行需求

[三專案使用者流程](../../requirements/USER_FLOWS.md) · [共用人員驗證](../../requirements/AUTHENTICATION.md) · [UI 詳細驗收](../../UI_ALIGNMENT_REQUIREMENTS.md)。共用條款統一維護，不在各首頁重複。

## 功能目標與完成條件

Analytics 定位為純管理後台，Leptos 提供後台畫面；除登入及必要的身分驗證流程外，所有業務頁面、查詢、管理與即時資料均須登入且具相應權限，不提供匿名 Dashboard。管理者 Session 與 Chat／Pet 服務憑證分開：服務上報／換證驗證機器身分，不以人員登入或來源 token 取代後台登入。

- 攝取與持久化：相同來源／事件 ID 重送不重複計數，內容衝突明確拒絕；失敗不能回成功 ACK，重啟後仍可查。
- Dashboard：選來源與事件類型、載入／空資料／錯誤／重試、數量與明細；不同來源資料與授權隔離，交易未提交不得先通知畫面。
- 通訊與資料：產品攝取目標為 gRPC／Protobuf 與逐事件 ACK，Dashboard 使用 HTTP JSON＋SSE；rust_ai_chat 既有 HTTP 入口依該庫實際契約核對。Analytics 獨立 PostgreSQL，不讀寫 Chat／Pet 的業務 SQLite。
- 管理與洞察：封存／還原／備份、可追溯 AI 洞察、人工審核及採用／回退；再加入 Provider／Model／來源／功能的 Token 用量、成本與實際可取得額度。未知額度明示未知，不能由 Token 數推算。

## 介面與接線要求

#### Leptos Dashboard：帳密主線已接通，完整互動與角色仍待驗收

Analytics 畫面沿用 Leptos 與 Rust 元件，不以獨立手寫 HTML／JavaScript 維護另一套互動介面。先核對現有 UI crate／SSR／hydration 的責任與建置入口再接線，不為三專案檔名一致新建空骨架。

基本元件為來源選擇、事件類型篩選、查詢／重新整理、載入狀態、空資料、錯誤／重試與結果表格；由 Leptos 狀態與非同步請求呼叫現有 HTTP JSON Query API，保留來源授權與 PostgreSQL Adapter。篩選切換時清除過時結果，取消或辨識舊請求，避免晚回應覆蓋新結果。SSE 於基本查詢接通後逐步補齊。

UI 不持有 SQLx／PgPool，不另建平行 total API 或資料來源；正式管理者 Session／唯讀查詢權限與服務憑證分開，不把後端來源密鑰嵌入 WASM／HTML。rust_ai_chat 來源 token 的既有契約保留為過渡事實，不當成正式 Dashboard 授權模型。

驗證須分別包含 native SSR（若採用）、WASM hydration／CSR、元件狀態、HTTP／來源隔離、真實 PostgreSQL 結果與瀏覽器流程；既有 API 或 CI 成功不能代替 Leptos 畫面驗收。完整畫面流程尚未驗收。

#### 服務間交互憑證

[單一服務配對、輪替與撤銷需求](../../SERVICE_CREDENTIAL_REQUIREMENTS.md)。實作／驗收狀態見目前進度。

#### 純後台登入與授權：帳密與 Session 已接通，完整管理仍待驗收

Analytics 定位為純管理後台，Leptos 提供後台畫面；除登入及必要的身分驗證流程外，所有業務頁面、查詢、管理與即時資料均須登入且具相應權限，不提供匿名 Dashboard。管理者 Session 與 Chat／Pet 服務憑證分開：服務上報／換證驗證機器身分，不以人員登入或來源 token 取代後台登入。

- 入口：未登入只可進入登入及必要驗證流程，不渲染或傳送業務資料；登入成功後只進入授權頁面。不提供公開業務首頁或匿名統計。
- 服務端強制：Leptos 畫面／SSR／server functions（若採用）、HTTP 查詢／管理 API 及 SSE 均檢查 Session 與操作權限；隱藏按鈕或前台 route guard 不能代替後端授權。未登入的 API 回 401，已登入但權限不足回 403；瀏覽器頁面導向登入，不先輸出敏感 SSR 資料。
- Session 生命週期：登入建立與更新有效工作階段；逾期、登出或管理者撤銷後不能繼續查詢／管理。SSE 在建立、重連及存續期間依授權政策處理失效，不能讓已登出的連線持續收到資料；具體撤銷生效期限在實作時明定與驗證。
- 管理操作：來源配對核准、憑證輪替／撤銷、封存／還原／備份等均須管理者權限與操作稽核；若使用 Cookie Session，狀態變更須有 CSRF 防護。不將來源服務的 ingestion／續期秘密交給後台瀏覽器當登入憑證。
- 機器通道：Chat／Pet 的 HTTP／gRPC 上報、配對兌換與自動換證使用各自服務身分及權限；不能開放匿名，也不要求每次自動上報由人員互動登入。一次性配對須有已登入管理者的核准交易。
- 驗收：直接輸入後台 URL、未登入呼叫 API／建立 SSE、跨權限操作、Session 到期／登出／撤銷、重連及狀態變更防護；確認沒有先洩漏頁面資料或以來源 token 越權登入。機器上報／換證與人員登入分別驗證。完整授權與機器通道條件尚未驗收。

## 驗證範圍

已核驗來源與測試範圍統一見GitHub [目前進度](../current/STATUS.md)。完整功能驗收仍須由真實入口逐段測試；單元測試、資料寫入或建置通過不能代替使用者流程驗收。

## 開發位置

rust_ai_chat 的 apps/stats\_server/src/<code>lib.rs</code> 與正式登入 router 負責 HTTP 路由與授權，pg\_<code>store.rs</code> 負責 PostgreSQL 聚合與持久化；依該庫精確 SHA 核對 migrations 與實際呼叫鏈。copilot_use 舊 shared/proto 與 analytics_transport 僅供歷史追溯，不列雙庫適配工作。

先沿既有 API 接通粗略可操作畫面，再逐步補趨勢、明細與管理。資料安全或基本流程阻礙當步處理，其他深化留在原功能追蹤；詳細程式與驗證仍看相關功能教學。

[產品通訊契約](https://github.com/williechen/copilot_use/blob/main/docs/ANALYTICS_TRANSPORT.md) · [操作設定](https://github.com/williechen/copilot_use/blob/main/docs/DEPLOYMENT.md) · [產品未來清單](https://github.com/williechen/copilot_use/blob/main/docs/FUTURE_WORK.md)

## 功能驗證現況

Leptos登入／改密碼、完整PG與實際HTTP通過；Day32事件類型篩選／清除／來源切換另有本輪真瀏覽器證據。角色與來源權限仍待完成。

[完整現況](../current/STATUS.md) · [本輪報告](../history/evidence/2026-10-24-formal-f3f8255/REPORT.md) · [歷史文章與原始證據](../history/README.md)

[Day31教學：Leptos 改密碼頁](../daily/day-31/analytics.md)（Leptos改密碼頁已套用並通過本輪PG／HTTP驗證；完整後台仍有獨立待辦。）


## 最新教學（Day32）

[Day32：Dashboard 事件類型篩選](../daily/day-32/analytics.md)（程式已套用，PG／HTTP與本輪瀏覽器篩選已驗證；完整產品驗收仍未完成）。下一篇 Day33，以 [即時進度檔](../../learning-state.json) 為準；Day31 與歷史驗證證據保留。
