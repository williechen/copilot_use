# Rust AI Ring｜P0–P12 十三階段詳細規劃

> 2026-10-25 依使用者更正：**13 個正式階段，P0–P12**。合併最新本地程式 `rust_ai_ring@5c713553daee4055738239f047d6c774fcc7a7a1`、文章 `copilot_use@4a3c43187e1ffd7cf4ba8d44cd4f8529f0d9d6d4` 與本地 `learning-state.json.curriculum`。此文件為需求與工作規劃，沒有將階段標為功能驗收完成。

## 合併來源與規劃決策

|來源|保留內容|本次處理|
|---|---|---|
|最新直接指示|13 階段、以本地最新程式與文章合併、只更新 copilot_use 文件|優先於前版「12 階段」文字|
|本地 curriculum|P0–P12、每階段 2 個月、共 26 個月、原日曆與每日 60 分鐘|完整保留，不把 P12 列成衝突|
|前版 P0–P11 Roadmap|P0 非 AI、P1.0 基礎 Logging、P10 完整觀測及中段功能順序|P0–P10 名稱順序沿用；原末段拆為 P11 部署就緒、P12 發布生命週期|
|本地最新程式|現有 routes／IPC／DB adapter／tests／workflow 設定|重新定位 caller 與差距；未重跑 CI／runtime，不追認 Verified|
|本地既有文章|105 閱讀版本、157 份正文／修訂／鏡像及原始雜湊|保留正文，只增分類／補充／階段關係，不遞增 Day 或重開 FIFO|

P11／P12 的細分是本次為完成十三階段而作的規劃決策。本地既有 curriculum 證明十三階段及日曆，但沒有足夠資料證明歷史上 P12 曾採此名稱，故不假稱恢復原名稱。原十二階段文件原文與 SHA256 保存於 [合併來源](source-review/phase-plan-merge-2026-10-25/previous-roadmap.json)。

## 共通原則與狀態

- `copilot_use` 只保存需求、教學、規劃、狀態及證據；唯一程式／測試／CI／產品驗收主體為 `rust_ai_ring`。本輪未修改其來源或執行產品測試。
- P0 完成非 AI 基本旅程；P0 的授權／去重／重啟不可延後。P0 工程 Gate 後立即優先做 P1.0 Common Logging；P1.0 是 P1 工作包，不是第十四階段。
- P10 擴展已有基本日誌為完整觀測；舊 P9_TARGET 已停用，新 P9 是教學代理，不恢復舊規則。
- Domain／Application 與 SQLx／Tauri／模型 SDK／OS Adapter 分開；按真實 caller 與產品需求增量實作，不預建空架構。
- 每階段記錄文件交付、程式接線、測試、CI、runtime／GUI／E2E、學習完成各自狀態。工程狀態用 Not Started→In Progress→Implemented→Verified；沒有足夠同 SHA 證據時記「未查核／待驗證」，不臆判 Not Started 或 Verified。
- 學習日曆到期仍按原政策切換，並標「階段已結束、未完成項目留原追蹤」。日期、Day 數與文章發布不等同工程 Exit Gate；未通過的依賴仍需補齊才能開啟依賴它的功能。
- 不修改原 next_day（各產品 37）、last_full_comparison、原 Day／修訂、既有測試結果或日期。所有後續工作連回 FUTURE_WORK 原功能，不建立重複任務清單。

## 十三階段總覽與原日曆

|階段|正式名稱|台北時間學習區間（迄日為下一階段起點）|工程退出重點|
|---|---|---|---|
|P0|Foundation & MVP｜非 AI 基礎產品|2026-10-12 → 2026-12-12|三條完整旅程均通過；重載／真正程序重啟仍可讀取正確資料；無 P0 blocker。|
|P1|Security & Identity｜含 P1.0 共用 Logging|2026-12-12 → 2027-02-12|P1.0 JSON 可解析、filter 正確、必要欄位齊、注入秘密未輸出，三程序成功／失敗事件可取證。|
|P2|Reliability & Persistence｜可靠性與資料恢復|2027-02-12 → 2027-04-12|相同事件多次並發／重啟後只計一次，不同內容同鍵明確衝突，DB 錯誤不得成功。|
|P3|Real-time Communication｜即時連線與同步|2027-04-12 → 2027-06-12|斷線期間提交的資料可補回且不重複，跨房／來源不洩漏；登出／縮權影響既有串流。|
|P4|AI Integration｜真實模型整合|2027-06-12 → 2027-08-12|真 Provider 成功／失敗／取消／stream end 可辨識，串流與非串流結果相容；敏感值不進日誌／事件。|
|P5|AI Persona & Conversation｜角色與對話控制|2027-08-12 → 2027-10-12|多角色不自我無限呼叫，超預算與取消停止後續發言；跨房／跨角色上下文不洩漏。|
|P6|Memory & RAG｜授權記憶與檢索|2027-10-12 → 2027-12-12|跨房／跨使用者記憶不可讀；撤回資料不再被檢索／回答，重建不復活已刪資料。|
|P7|MCP & Tool Integration｜工具權限與可追溯執行|2027-12-12 → 2028-02-12|未授權／未確認／越界參數不執行，回覆無法升權；重試不重複破壞性副作用。|
|P8|Agent Runtime｜有界任務與恢復|2028-02-12 → 2028-04-12|所有執行有界，取消後不再啟動新工具，已提交副作用有正確結果；崩潰後可繼續或明確終止。|
|P9|Learning & Teaching Agent｜新定義的教學代理|2028-04-12 → 2028-06-12|相同歷程與規則可說明選題；複習間隔與錯答調整可重現，無歷程不捏造學會。|
|P10|Observability & Operations｜完整觀測|2028-06-12 → 2028-08-12|從使用者操作追至必要服務與儲存邊界；敏感值無洩漏，告警能觸發、定位並恢復。|
|P11|Production Readiness & Deployment｜正式部署與營運就緒|2028-08-12 → 2028-10-12|部署可重現、秘密不落庫／日誌／映像；readiness 與依賴故障相符，停機不遺失已確認資料。|
|P12|Release & Lifecycle｜發布、安裝更新與回滾|2028-10-12 → 2028-12-12|每個宣稱支援的平台均完成真實安裝、啟動、保資料更新、中斷恢復與支援回退流程。|

以上區間沿用本地既有安排；每日 Chat／Pet／Analytics 各 15 分鐘，加共用彈性 15 分鐘。只計主體 Red／Green；Refactor、文章閱讀、工程／GUI 驗證另計，不把本文全部交付塞入一天。

## 三產品適用範圍

|階段|Chat|Pet|Analytics|
|---|---|---|---|
|P0|Chat：主體|Pet：主體|Analytics：主體|
|P1|Chat：Logging／人員與房間授權|Pet：Logging／原生 capability／來源身分|Analytics：Logging／角色、來源與配對管理|
|P2|Chat：訊息／migration／事件|Pet：OS 與 DB 一致性／事件|Analytics：PG 冪等／備份恢復|
|P3|Chat：即時主體|Pet：逐窗 IPC 同步|Analytics：授權即時後台|
|P4|Chat：AI 對話入口|Pet：AI 對話／素材按需求|Analytics：usage 接收；AI 洞察另按需求|
|P5|Chat：多角色對話|Pet：人格與互動|Analytics：usage／費用統計；Persona N/A|
|P6|Chat：房間／個人記憶|Pet：獨立寵物記憶|Analytics：品質／治理；個人記憶 N/A|
|P7|Chat：可授權工具|Pet：可選 OS 工具|Analytics：工具稽核查詢；MCP 執行 N/A 除非另需|
|P8|Chat：可見的任務|Pet：可選背景任務|Analytics：執行稽核與使用量|
|P9|Chat：教學互動|Pet：可選學習陪伴|Analytics：經授權的學習成效分析|
|P10|Chat：請求／WS／模型|Pet：原生／工具／模型|Analytics：ingest／query／營運觀測|
|P11|Chat：Web／服務部署|Pet：桌面組態與服務相容|Analytics：PG／後台部署|
|P12|Chat：服務版本與資產發布|Pet：原生 installer／updater|Analytics：服務／schema 發布與回復|

N/A 僅在該產品不需要該能力時可用，須記理由；不將 N/A 算未完成，也不可用它跳過共同安全／資料義務。

## P0 — Foundation & MVP｜非 AI 基礎產品

### Scope

- Chat：公開清單→選房→登入保留目標並核權→兩人聊天→成功持久化→重載／程序重啟後讀回→登出撤銷；首頁登入回個人頁，註冊完成保持登出且清除選房目標。
- Pet：建立／列表／選取、多寵物獨立生活窗與狀態；回籠及直接關生活窗都停止該隻活動而保留資料；控制窗關閉後可恢復。
- Pet 啟動依 USER_FLOWS 的「預設寵物」政策，可多隻；預設資格與當次回籠狀態分離。不可把 Day36 的所有未回籠恢復直接當此需求完成。基本移動／睡醒／逐 ID 事件須實際可用。
- Analytics：初始化管理員→首次改密碼與唯一 Email→登入→授權機器事件寫入／同來源去重→人員 Session 查統計及篩選→重啟一致→登出拒絕。
- 最低安全條件：伺服器核對身分與資源權限、敏感操作拒絕未授權、密碼加鹽雜湊、資料庫分離；P1 將能力系統化，不能用後續階段作不安全基本流程的豁免。

### Non-goals

- 不要求真 AI、RAG、MCP、Agent、完整 OTel、正式發布；FileOrganizer 破壞性操作不作 P0 Gate。
- 產品基本程式已存在的切片只補真缺口及驗收，不重做已接通的 Day36 撤銷／統計／恢復查詢。

### Prerequisites

使用最新需求及精確 rust_ai_ring SHA，具隔離 SQLite／PostgreSQL、瀏覽器及 Tauri 原生環境。

### Deliverables

- 三產品可操作基本旅程與正／負向案例表；每個失敗步驟連回原 FUTURE_WORK 功能。
- 前後端、DB、原生 binary／JS／WASM 的一致來源紀錄，以及 fresh DB 與需支援舊 DB 的邊界。

### Acceptance Criteria / Exit Gate

- 三條完整旅程均通過；重載／真正程序重啟仍可讀取正確資料；無 P0 blocker。
- Chat 跨房授權／作者身分及登出後既有連線不得越權；Pet 不串錯 ID／不重複窗且符合預設政策；Analytics 人員與機器身分互不替代。
- 必要同 SHA 測試與適用 CI 可追溯；未執行或環境不支援須明列，不能標 Verified。

### Tests

- Chat 真 HTTP＋雙瀏覽器／WS＋SQLite 讀回，含匿名／越權／撤銷／重啟。
- Pet 真三窗、逐 ID 操作、回籠／直接關窗／控制窗恢復、預設與非預設重啟；DB 及窗失敗另記。
- Analytics 真 PG＋HTTP／Browser：stored／duplicate／conflict、登入／改密碼／登出與重新啟動。

### Evidence

需求版本＋rust_ai_ring 精確 commit＋實際 caller／route／IPC／migration＋測試命令／環境／結果＋同 SHA CI run 與 target／feature＋適用的 Browser／GUI／程序重啟證據；失敗及未執行原樣保存。

原追蹤：[chat-auth](FUTURE_WORK.md#chat-auth) · [chat-history](FUTURE_WORK.md#chat-history) · [pet-lifecycle](FUTURE_WORK.md#pet-lifecycle) · [pet-roaming](FUTURE_WORK.md#pet-roaming) · [analytics-auth](FUTURE_WORK.md#analytics-auth) · [analytics-session](FUTURE_WORK.md#analytics-session) · [ci-coverage](FUTURE_WORK.md#ci-coverage)。本階段不另開 Day／FIFO。

### Status

詳細規劃已記錄；來源所見限 [本輪本地基準](source-review/phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)。測試／CI run／runtime 本輪未重驗，工程 Gate 未由本文件宣告通過。

## P1 — Security & Identity｜含 P1.0 共用 Logging

### Scope

- 第一工作包 P1.0：三程序共用 tracing／tracing-subscriber 初始化、可切 JSON／文字、RUST_LOG、timestamp／level／target／service.name／event_name、request_id；明確敏感值 allowlist。
- 各產品接啟停、auth failure、HTTP／WS 生命週期、Pet 建立／選取／DB 拒絕、Analytics stored／duplicate／conflict 與管理員登入；補三程序 runtime smoke。
- Chat／Analytics 共用 Cookie／Session 政策和 Port，但各自 SQLite／PG 與登入狀態獨立；活動續期、閒置後台設定、帳號級登入等待、改密碼／Email 重設及所有 Session 撤銷。
- Chat 房主／資源 ACL／管理入口；Analytics 管理者及唯讀角色、來源範圍、停用／縮權立即作用於既有連線，管理動作低敏稽核。
- 機器來源管理者核准配對、短效資格、自動續期輪替、撤銷與重放防護；協定依本地 SERVICE_CREDENTIAL_REQUIREMENTS 選型，不預定 JWT 或 mTLS。

### Non-goals

- P1.0 不含 OTel Collector／exporter／distributed traces／metrics Dashboard；P10 負責完整觀測。
- 日誌不是業務事件投遞；機器服務憑證不能取代人員後台 Session。

### Prerequisites

P0 基本旅程已完成工程 Gate；P1.0 隨即優先。已知授權漏洞須即時修復，不等待日曆切點。

### Deliverables

- 共用 Logging 初始化與欄位／redaction 契約、stdout/stderr 收集／rotation／retention 操作文件。
- 人員、角色、資源、來源及機器憑證的權限矩陣與撤銷生效界線；找回密碼合成郵件測試流程。

### Acceptance Criteria / Exit Gate

- P1.0 JSON 可解析、filter 正確、必要欄位齊、注入秘密未輸出，三程序成功／失敗事件可取證。
- 跨角色／房間／來源、CSRF、過期／撤銷資格均拒絕；活躍 Session 與閒置政策符合 AUTHENTICATION。
- 帳號等待 15 秒起倍增的既有需求、刷新／IP 無法繞過、等待中不重新起算；輪替／回應遺失／撤銷後無法續期可驗。

### Tests

- logger unit＋subprocess smoke、秘密哨兵與 JSON schema 檢查。
- 真 HTTP／WS／SSE（若已有）負向授權、並發撤銷、fake clock 等待／閒置、密碼重設一次性與過期。
- 合成服務配對／換證／重放／重啟測試及管理畫面狀態一致性，不操作正式秘密。

### Evidence

需求版本＋rust_ai_ring 精確 commit＋實際 caller／route／IPC／migration＋測試命令／環境／結果＋同 SHA CI run 與 target／feature＋適用的 Browser／GUI／程序重啟證據；失敗及未執行原樣保存。

原追蹤：[chat-auth](FUTURE_WORK.md#chat-auth) · [analytics-auth](FUTURE_WORK.md#analytics-auth) · [analytics-session](FUTURE_WORK.md#analytics-session) · [event-transport](FUTURE_WORK.md#event-transport) · [ci-coverage](FUTURE_WORK.md#ci-coverage)。本階段不另開 Day／FIFO。

### Status

詳細規劃已記錄；來源所見限 [本輪本地基準](source-review/phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)。測試／CI run／runtime 本輪未重驗，工程 Gate 未由本文件宣告通過。

## P2 — Reliability & Persistence｜可靠性與資料恢復

### Scope

- 交易邊界、同鍵同內容冪等／不同內容衝突、並發重送及程序重啟；持久化失敗不廣播、不回成功 ACK。
- Chat／Pet 真來源事件→可靠佇列或 outbox（確有跨交易需求才採）→傳輸→Analytics PG commit ACK；HTTP／gRPC 的必要契約依 ANALYTICS_TRANSPORT 逐項選型。
- 不可改寫已交付 migration；fresh／支援舊庫前向升級、失敗保資料；Pet DB／開關窗／Runtime 的部分失敗補償。
- 隔離備份、還原、資料保留／撤回需求及可核對的恢復目標，數值以產品需求決定。

### Non-goals

- 不用無界重試或記憶體去重冒充跨程序保證；不用空 Port 或空資料表作完成。

### Prerequisites

P0 儲存流程與 P1 必要安全、Logging 契約可用；傳輸選型先有實際來源 caller。

### Deliverables

- 寫入與 ACK／事件發送的一致性設計、重試上限／錯誤分類、migration 支援矩陣。
- 故障注入與備份還原 runbook、重啟／並發／去重原始證據。

### Acceptance Criteria / Exit Gate

- 相同事件多次並發／重啟後只計一次，不同內容同鍵明確衝突，DB 錯誤不得成功。
- 升級及失敗回復保留舊資料；還原後可驗資料數與語義；部分 OS 失敗有可見錯誤與可恢復狀態。

### Tests

- 真 SQLite／PG 交易、rollback、斷線、程序 kill/restart、重放／並發去重。
- 支援版本舊 DB fixture 升級、checksum 保護、backup→restore→產品讀回及 Pet 視窗故障補償。

### Evidence

需求版本＋rust_ai_ring 精確 commit＋實際 caller／route／IPC／migration＋測試命令／環境／結果＋同 SHA CI run 與 target／feature＋適用的 Browser／GUI／程序重啟證據；失敗及未執行原樣保存。

原追蹤：[chat-history](FUTURE_WORK.md#chat-history) · [pet-lifecycle](FUTURE_WORK.md#pet-lifecycle) · [event-transport](FUTURE_WORK.md#event-transport) · [analytics-capabilities](FUTURE_WORK.md#analytics-capabilities)。本階段不另開 Day／FIFO。

### Status

詳細規劃已記錄；來源所見限 [本輪本地基準](source-review/phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)。測試／CI run／runtime 本輪未重驗，工程 Gate 未由本文件宣告通過。

## P3 — Real-time Communication｜即時連線與同步

### Scope

- Chat reconnect、presence、typing、歷史增量同步與斷線補訊；以穩定訊息 ID／游標避免重複及順序歧義。
- Pet 動態 ID 事件訂閱、窗口生命週期／關窗釋放、重開／重連同步；本機 IPC 不強制新增網路服務。
- Analytics 即時查詢／SSE 的 commit 後通知、rollback 無事件、Last-Event-ID／補查、慢消費者及授權續查。

### Non-goals

- 不將 typing／presence 寫成永久聊天訊息；不因使用 WS／SSE 就宣稱可靠送達。

### Prerequisites

P0 歷史基本讀回、P1 撤銷與來源授權、P2 穩定 ID／持久化事件邊界。

### Deliverables

- 連線狀態機、重連／補查／backpressure 契約、UI 載入／離線／錯誤狀態。
- 長連線撤銷、訂閱釋放及漏訊補回端到端證據。

### Acceptance Criteria / Exit Gate

- 斷線期間提交的資料可補回且不重複，跨房／來源不洩漏；登出／縮權影響既有串流。
- 慢接收端、重連風暴、頁面卸載／原生關窗不造成無界累積或幽靈訂閱。

### Tests

- 網路中斷／重連、並發房間、游標過期、慢消費者、取消訂閱。
- SSE 真 PG commit／rollback、已撤銷 Session、多 Pet ID 事件隔離與原生重開。

### Evidence

需求版本＋rust_ai_ring 精確 commit＋實際 caller／route／IPC／migration＋測試命令／環境／結果＋同 SHA CI run 與 target／feature＋適用的 Browser／GUI／程序重啟證據；失敗及未執行原樣保存。

原追蹤：[chat-capabilities](FUTURE_WORK.md#chat-capabilities) · [event-transport](FUTURE_WORK.md#event-transport) · [pet-lifecycle](FUTURE_WORK.md#pet-lifecycle) · [analytics-auth](FUTURE_WORK.md#analytics-auth)。本階段不另開 Day／FIFO。

### Status

詳細規劃已記錄；來源所見限 [本輪本地基準](source-review/phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)。測試／CI run／runtime 本輪未重驗，工程 Gate 未由本文件宣告通過。

## P4 — AI Integration｜真實模型整合

### Scope

- 透過 Model Port 接真 Provider；Responses API 等供應商 adapter 的版本／契約以實作當時官方文件與 SDK 為準。
- HTTP／SSE 串流、structured output、function calling 的編解碼與錯誤分類；function calling 尚不等於可以任意執行工具。
- 逾時、取消、rate limit、重試及 usage 缺失；Analytics 接最小 usage schema，未知 quota／成本明示未知。

### Non-goals

- MockChatModel 僅作測試替身；不宣稱已接真 AI。未經授權不執行付費或正式帳號呼叫。
- 不在此階段加入完整 RAG／MCP／Agent 或多角色自治循環。

### Prerequisites

P1 安全保管／Logging、P2 請求可靠性、P3 串流與取消邊界；先明確模型與成本上限。

### Deliverables

- provider adapter、模型設定、串流 UI、錯誤與取消行為、usage schema／估算標示。
- 固定 fixture 契約測試與另列、獲授權的真 Provider smoke。

### Acceptance Criteria / Exit Gate

- 真 Provider 成功／失敗／取消／stream end 可辨識，串流與非串流結果相容；敏感值不進日誌／事件。
- usage 缺漏不作 0、重試不重複計費；有上限且能停，真實呼叫證據與 mock 測試分開。

### Tests

- 假 provider／HTTP replay：分段 SSE、schema 失配、429／5xx、timeout／cancel。
- 獲授權的少量真實 smoke；精確模型／版本／請求結果／usage，原文以低敏證據保存。

### Evidence

需求版本＋rust_ai_ring 精確 commit＋實際 caller／route／IPC／migration＋測試命令／環境／結果＋同 SHA CI run 與 target／feature＋適用的 Browser／GUI／程序重啟證據；失敗及未執行原樣保存。

原追蹤：[chat-capabilities](FUTURE_WORK.md#chat-capabilities) · [pet-capabilities](FUTURE_WORK.md#pet-capabilities) · [analytics-capabilities](FUTURE_WORK.md#analytics-capabilities)。本階段不另開 Day／FIFO。

### Status

詳細規劃已記錄；來源所見限 [本輪本地基準](source-review/phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)。測試／CI run／runtime 本輪未重驗，工程 Gate 未由本文件宣告通過。

## P5 — AI Persona & Conversation｜角色與對話控制

### Scope

- Persona、系統規則、context window／摘要與多回合持續；使用者輸入與工具回覆不得越過安全政策。
- 多角色 turn-taking、cooldown、token／費用／回合預算、停止／取消與循環偵測。
- Pet 五項 Profile 與可選外觀設定接到實際 AI 行為；非 AI 漫遊／基本睡醒仍按 P0／既有原需求追蹤。

### Non-goals

- 不把 PetProfile 值物件或角色列舉當成完整 AI 對話；不要求 Analytics 模擬人格。

### Prerequisites

P4 真模型與 usage／取消可用；P1 權限和 P2 對話狀態保存邊界。

### Deliverables

- 角色與上下文政策、對話／回合狀態、預算設定、UI 角色識別。
- 預算耗盡／角色衝突／使用者停止的可重現案例。

### Acceptance Criteria / Exit Gate

- 多角色不自我無限呼叫，超預算與取消停止後續發言；跨房／跨角色上下文不洩漏。
- Profile 變更有可解釋的行為效果，前端顯示與實際使用角色一致。

### Tests

- 固定模型替身＋seed 的回合／預算／上下文單測，實際 UI 多回合／中斷。
- 角色輸入注入、輪流競態、usage 缺失保守處理與新舊 Profile 相容。

### Evidence

需求版本＋rust_ai_ring 精確 commit＋實際 caller／route／IPC／migration＋測試命令／環境／結果＋同 SHA CI run 與 target／feature＋適用的 Browser／GUI／程序重啟證據；失敗及未執行原樣保存。

原追蹤：[chat-capabilities](FUTURE_WORK.md#chat-capabilities) · [pet-capabilities](FUTURE_WORK.md#pet-capabilities) · [analytics-capabilities](FUTURE_WORK.md#analytics-capabilities)。本階段不另開 Day／FIFO。

### Status

詳細規劃已記錄；來源所見限 [本輪本地基準](source-review/phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)。測試／CI run／runtime 本輪未重驗，工程 Gate 未由本文件宣告通過。

## P6 — Memory & RAG｜授權記憶與檢索

### Scope

- 對話記憶保存、來源／owner／room 範圍、embedding／retriever adapter、chunk／metadata 版本與可管理生命週期。
- 查詢前授權、刪除／撤回／更新同步於原文與索引；跨程序重啟後可取回正確記憶。
- RAG 引文、找不到證據的回答策略與檢索品質資料集；依本地需求選儲存方案，不預先指定向量 DB。

### Non-goals

- 模型 context 不是持久記憶；相似度不等於授權；不因資料表存在就標 RAG 完成。

### Prerequisites

P4／P5 對話與上下文、P1 資源授權、P2 資料保留與刪除／備份語義。

### Deliverables

- 記憶／Retriever Port、索引與重建工作、管理 UI、查詢與引用 trace。
- 授權、刪除、跨重啟與品質基準。

### Acceptance Criteria / Exit Gate

- 跨房／跨使用者記憶不可讀；撤回資料不再被檢索／回答，重建不復活已刪資料。
- 重啟能查回，引用能回到實際來源；品質門檻先定測試集再量測，不臆造通過率。

### Tests

- 真儲存與索引 integration、ACL／刪除／重建、空查詢與 provider embedding 錯誤。
- 固定資料集檢索評估及 UI 引文可追溯驗證。

### Evidence

需求版本＋rust_ai_ring 精確 commit＋實際 caller／route／IPC／migration＋測試命令／環境／結果＋同 SHA CI run 與 target／feature＋適用的 Browser／GUI／程序重啟證據；失敗及未執行原樣保存。

原追蹤：[chat-capabilities](FUTURE_WORK.md#chat-capabilities) · [pet-capabilities](FUTURE_WORK.md#pet-capabilities) · [analytics-capabilities](FUTURE_WORK.md#analytics-capabilities)。本階段不另開 Day／FIFO。

### Status

詳細規劃已記錄；來源所見限 [本輪本地基準](source-review/phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)。測試／CI run／runtime 本輪未重驗，工程 Gate 未由本文件宣告通過。

## P7 — MCP & Tool Integration｜工具權限與可追溯執行

### Scope

- MCP client、能力協商、Tool Registry、參數 schema 與結果映射；實際採用 transport／版本於實作當時確認。
- 依使用者／來源／工具／操作授權，必要的破壞性或外部副作用確認；工具回覆視為非可信資料。
- Pet FileOrganizer 作可選工具 adapter：授權根目錄、opaque preview、確認與真 filesystem receipt／restore 分開，依既有需求逐項完成。

### Non-goals

- 現有 Tauri IPC／掃描預覽不等於 MCP；不把工具回傳的文字當成新授權。
- FileOrganizer 尚未進核心 Gate 的部分不強制前移 P0。

### Prerequisites

P1 工具／資源權限與稽核、P4 function calling 契約、P2 副作用冪等與回復。

### Deliverables

- 工具目錄／schema／權限矩陣、拒絕與確認 UI、timeout／取消／錯誤映射。
- 合成 MCP server 與實際副作用 receipt 的分層證據。

### Acceptance Criteria / Exit Gate

- 未授權／未確認／越界參數不執行，回覆無法升權；重試不重複破壞性副作用。
- 工具失敗與取消可回復，稽核記對象與結果但不含秘密／私密檔內容。

### Tests

- 協商／工具列舉／呼叫／斷線、schema 不符、惡意工具輸出／路徑 symlink。
- 隔離目錄 preview→確認→操作→receipt→restore；未執行 OS 僅能標模型測試。

### Evidence

需求版本＋rust_ai_ring 精確 commit＋實際 caller／route／IPC／migration＋測試命令／環境／結果＋同 SHA CI run 與 target／feature＋適用的 Browser／GUI／程序重啟證據；失敗及未執行原樣保存。

原追蹤：[pet-capabilities](FUTURE_WORK.md#pet-capabilities) · [chat-capabilities](FUTURE_WORK.md#chat-capabilities) · [event-transport](FUTURE_WORK.md#event-transport)。本階段不另開 Day／FIFO。

### Status

詳細規劃已記錄；來源所見限 [本輪本地基準](source-review/phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)。測試／CI run／runtime 本輪未重驗，工程 Gate 未由本文件宣告通過。

## P8 — Agent Runtime｜有界任務與恢復

### Scope

- 任務 state machine、execution／turn boundary、步數／時間／費用預算、cancel／pause 的明確語義。
- checkpoint、程序重啟 recovery、工具副作用記錄與重試；人員介入／必要確認不跨過授權邊界。
- UI 顯示任務狀態與可核對的進度，失敗不冒充成功，重試仍追蹤原任務。

### Non-goals

- PetMachine 的存在不等於 Agent Runtime；不預建無用途的多 Agent 框架。

### Prerequisites

P4–P7 所需 provider／記憶／工具已具能力；P1 權限、P2 恢復必須存在。

### Deliverables

- 任務／執行／checkpoint 資料契約、取消／恢復入口、可追溯事件。
- 跨程序恢復及工具副作用不重做的驗收案例。

### Acceptance Criteria / Exit Gate

- 所有執行有界，取消後不再啟動新工具，已提交副作用有正確結果；崩潰後可繼續或明確終止。
- 權限縮減影響待執行步驟，checkpoint 不攜帶可重用秘密。

### Tests

- 固定 clock／模型／工具替身的狀態轉移、budget、取消競態。
- 真程序 kill／restart、重複訊息、工具回應遺失與副作用去重 integration。

### Evidence

需求版本＋rust_ai_ring 精確 commit＋實際 caller／route／IPC／migration＋測試命令／環境／結果＋同 SHA CI run 與 target／feature＋適用的 Browser／GUI／程序重啟證據；失敗及未執行原樣保存。

原追蹤：[chat-capabilities](FUTURE_WORK.md#chat-capabilities) · [pet-capabilities](FUTURE_WORK.md#pet-capabilities) · [event-transport](FUTURE_WORK.md#event-transport)。本階段不另開 Day／FIFO。

### Status

詳細規劃已記錄；來源所見限 [本輪本地基準](source-review/phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)。測試／CI run／runtime 本輪未重驗，工程 Gate 未由本文件宣告通過。

## P9 — Learning & Teaching Agent｜新定義的教學代理

### Scope

- learner state、能力目標／證據、active recall、spaced repetition、可解釋的下題選擇。
- 保留原 Day／修訂／來源與原始作答；學習完成、文章交付與產品 Gate 獨立。
- 依學習者授權管理資料、人工更正／撤回與重新計算；教學建議不自動改寫歷史文章或工程結案。

### Non-goals

- 新 P9 不恢復已廢除 P9_TARGET；目前 learning-state.json 與排程文件只是規劃來源，不等於產品代理已實作。

### Prerequisites

P6 授權記憶、P8 有界執行及 P4–P5 對話；先有可核對的學習資料模型。

### Deliverables

- learner／recall／review 契約、複習排程與解釋、教師／使用者調整入口。
- 固定題庫與合成學習歷程驗證資料；資料刪除／更正證據。

### Acceptance Criteria / Exit Gate

- 相同歷程與規則可說明選題；複習間隔與錯答調整可重現，無歷程不捏造學會。
- 學習狀態重啟保留且權限隔離，調整不覆寫歷史證據或把 CI 當學習完成。

### Tests

- 固定 clock 的 recall／複習／時區邊界與冷啟動資料缺漏。
- 真儲存、撤回／更正、人工介入、跨使用者隔離與 UI 解釋驗收。

### Evidence

需求版本＋rust_ai_ring 精確 commit＋實際 caller／route／IPC／migration＋測試命令／環境／結果＋同 SHA CI run 與 target／feature＋適用的 Browser／GUI／程序重啟證據；失敗及未執行原樣保存。

原追蹤：[chat-capabilities](FUTURE_WORK.md#chat-capabilities) · [pet-capabilities](FUTURE_WORK.md#pet-capabilities) · [analytics-capabilities](FUTURE_WORK.md#analytics-capabilities)。本階段不另開 Day／FIFO。

### Status

詳細規劃已記錄；來源所見限 [本輪本地基準](source-review/phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)。測試／CI run／runtime 本輪未重驗，工程 Gate 未由本文件宣告通過。

## P10 — Observability & Operations｜完整觀測

### Scope

- 擴展 P1.0：跨 HTTP／WS／工具／模型／事件鏈的 context propagation、traces／metrics、exporter／collector 與營運查詢。
- 錯誤率／延遲／佇列／資料庫／模型 usage 與成本；模型 quota 未提供時標未知，產品統計與觀測分流。
- 告警、sampling／cardinality、低敏欄位、retention 與敏感資料刪除／遮蔽。

### Non-goals

- 不把 Analytics 業務 event-id 當 trace ID，也不以 request_id 宣稱跨服務追蹤。
- 不等到本階段才有基本日誌；不由零散 tracing import 宣稱完整 OTel。

### Prerequisites

P1.0 Logging 和 P1 安全已可用；選已存在的端到端旅程，其他階段新增能力增量納入。

### Deliverables

- 觀測欄位與傳播契約、Dashboard／alerts、collector/export 操作與故障降級。
- 端到端 trace／metrics 範例與告警演練。

### Acceptance Criteria / Exit Gate

- 從使用者操作追至必要服務與儲存邊界；敏感值無洩漏，告警能觸發、定位並恢復。
- 觀測後端不可用不拖垮產品；負載與高 cardinality 仍受控，費用／quota 狀態來源清楚。

### Tests

- 合成 collector 斷線、抽樣／propagation、跨服務錯誤與 secret sentinel。
- 受控負載與告警演練、restore 後觀測、固定資料集成本去重與未知欄位。

### Evidence

需求版本＋rust_ai_ring 精確 commit＋實際 caller／route／IPC／migration＋測試命令／環境／結果＋同 SHA CI run 與 target／feature＋適用的 Browser／GUI／程序重啟證據；失敗及未執行原樣保存。

原追蹤：[ci-coverage](FUTURE_WORK.md#ci-coverage) · [event-transport](FUTURE_WORK.md#event-transport) · [analytics-capabilities](FUTURE_WORK.md#analytics-capabilities)。本階段不另開 Day／FIFO。

### Status

詳細規劃已記錄；來源所見限 [本輪本地基準](source-review/phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)。測試／CI run／runtime 本輪未重驗，工程 Gate 未由本文件宣告通過。

## P11 — Production Readiness & Deployment｜正式部署與營運就緒

### Scope

- 部署拓撲、組態／秘密注入、HTTPS／可信對端、DB 權限、連線池、備份與 disaster-recovery 演練。
- staging→production 的變更流程、health/readiness、graceful shutdown、資源限制與容量基準。
- 建立服務維運、診斷、告警／事故處理 runbook；Desktop 與後端的版本／通訊相容及安全連線。

### Non-goals

- 部署腳本存在或本地 build 成功不等於正式部署；不自動操作實際正式環境。
- 安裝包發行、updater 信任鏈、跨版升級／回滾由 P12 驗收。

### Prerequisites

擬部署的功能 Gate 及 P1 安全／P2 備份恢復／P10 必要觀測已具證據；N/A 有理由。

### Deliverables

- 環境與秘密契約、可重建部署設定、容量／備份恢復目標、運維 runbook。
- 隔離 staging 的 clean install、DB restore、依賴不可用、shutdown 演練結果。

### Acceptance Criteria / Exit Gate

- 部署可重現、秘密不落庫／日誌／映像；readiness 與依賴故障相符，停機不遺失已確認資料。
- 還原演練符合先確認的恢復目標；責任人能依 runbook 定位與恢復。

### Tests

- 隔離部署 smoke／負載、秘密輪替、DB／服務故障、TLS 設定、關閉／重啟。
- backup→restore→三產品關鍵旅程；環境未備妥時只交付計畫，不能標部署完成。

### Evidence

需求版本＋rust_ai_ring 精確 commit＋實際 caller／route／IPC／migration＋測試命令／環境／結果＋同 SHA CI run 與 target／feature＋適用的 Browser／GUI／程序重啟證據；失敗及未執行原樣保存。

原追蹤：[ci-coverage](FUTURE_WORK.md#ci-coverage) · [event-transport](FUTURE_WORK.md#event-transport) · [analytics-capabilities](FUTURE_WORK.md#analytics-capabilities)。本階段不另開 Day／FIFO。

### Status

詳細規劃已記錄；來源所見限 [本輪本地基準](source-review/phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)。測試／CI run／runtime 本輪未重驗，工程 Gate 未由本文件宣告通過。

## P12 — Release & Lifecycle｜發布、安裝更新與回滾

### Scope

- 以精確 tag／commit 產生可追溯版本、產物雜湊、release notes 與相依／授權資訊。
- Pet 依目標平台產生 installer、簽章／信任與更新檢查；平台能力與實測分列，不以 Linux 通過代表 Windows／macOS。
- 升級／中斷恢復／回滾與 DB 相容政策；不可降級的 migration 明確拒絕或前向修復，不以程式退版保證資料可回復。
- 跨產品版本相容、release channel、可控制的更新推出與回退；卸載／重裝的使用者資料保留與刪除政策。

### Non-goals

- 不重建第二個 copilot_use 產品或 CI；舊 copilot_use tag／三平台記錄僅歷史，不移作 rust_ai_ring 驗收。
- 發布到真實使用者或正式環境仍須具該操作授權，本輪只發布規劃文件。

### Prerequisites

P11 部署與支援矩陣、P2 migration／restore、P1 更新來源與簽章信任、P10 故障可觀測。

### Deliverables

- 版本／tag／產物清單、平台測試矩陣、installer／updater 契約、upgrade／rollback runbook。
- 舊版→新版→允許回退版本的可重現資料與流程證據；不能回退時有前向恢復方案。

### Acceptance Criteria / Exit Gate

- 每個宣稱支援的平台均完成真實安裝、啟動、保資料更新、中斷恢復與支援回退流程。
- 錯誤簽章／來源／舊版本拒絕；tag、產物、文件與已部署版本可互相核對，未完成平台標未驗收。

### Tests

- clean install／upgrade／repair／uninstall／reinstall，真實平台與 updater 換版／中斷／拒絕。
- 版本相容、簽章負向、migration 不可回退、資料保留、release smoke 與復原演練。

### Evidence

需求版本＋rust_ai_ring 精確 commit＋實際 caller／route／IPC／migration＋測試命令／環境／結果＋同 SHA CI run 與 target／feature＋適用的 Browser／GUI／程序重啟證據；失敗及未執行原樣保存。

原追蹤：[ci-coverage](FUTURE_WORK.md#ci-coverage) · [pet-capabilities](FUTURE_WORK.md#pet-capabilities)。本階段不另開 Day／FIFO。

### Status

詳細規劃已記錄；來源所見限 [本輪本地基準](source-review/phase-plan-merge-2026-10-25/SOURCE_BASELINE.md)。測試／CI run／runtime 本輪未重驗，工程 Gate 未由本文件宣告通過。

## 當前執行優先序與文章銜接

1. 先完成 P0 真缺口：Chat 房間／作者授權與歷史讀回；Pet 預設資格／回籠／直接關窗／逐 ID 漫遊；Analytics 正式人員／角色來源授權及完整管理旅程。每次先重讀最新 caller，Day36 已接通的 WS 重驗、精確查詢 Session、未回籠恢復查詢不重做。
2. 為 P0 補整條真實旅程與必要 CI 範圍；保留既有局部測試。程式已見但未驗者先驗，確定缺口才實作。
3. P0 Gate 通過後先 P1.0，再按本階段安全項及跨階段依賴分解。嚴重安全問題不等待階段切點。
4. P2 持久可靠性與 P3 即時補訊按真依賴拆分；P4→P5→P6→P7→P8→P9 為功能路線，不要求每產品機械地全部實作。P10 的觀測工作隨功能增量準備，最後做完整端到端驗收。
5. P11 驗部署／營運就緒；P12 驗版本、installer／updater／升級回滾與支援平台。二者各自有退出條件，不合併成一次「build 成功」。

[逐篇分類與缺口](source-review/p0-p11-local-scan-2026-10-25/README.md) 的資料夾名稱為原交付識別，內容已按十三階段更正；[合併紀錄](source-review/phase-plan-merge-2026-10-25/README.md) 說明本地來源、原日曆與新增設計的界線。

## 證據記錄模板

|欄位|內容|
|---|---|
|需求／階段|本文件階段＋原 FUTURE_WORK anchor＋使用者流程版本|
|程式|rust_ai_ring SHA、caller／route／IPC、DB／migration、binary／前端建置來源|
|測試|命令、target／feature、環境、測試總數／失敗／忽略、原始 log|
|CI|同 SHA run URL、workflow、實際 target／feature、失敗與未覆蓋項|
|runtime／GUI／E2E|真旅程、使用者觀察或自動化來源、程序重啟／故障步驟、適用平台|
|結果|文件／Implemented／Verified／學習完成分列，失敗回原功能追蹤|

## 需求來源

[使用者流程](requirements/USER_FLOWS.md) · [人員驗證](requirements/AUTHENTICATION.md) · [UI 條件](UI_ALIGNMENT_REQUIREMENTS.md) · [服務憑證](SERVICE_CREDENTIAL_REQUIREMENTS.md) · [傳輸目標](ANALYTICS_TRANSPORT.md) · [原功能清單](FUTURE_WORK.md) · [現行狀態](teaching/current/STATUS.md)。
