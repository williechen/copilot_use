# 功能待辦與工程銜接

> 2026-10-26增量核對：Day40的Session首頁與CSV已見實作，但Chat/account路由回歸、Pet X回籠未接且刪3舊測試。[最新精確來源與命令](teaching/daily/day-40/local-validation/README.md)及[歷史更正](teaching/daily/day-40/historical-corrections.md)優先於下列歷史「目前」摘要；原完成條件／FIFO不變。

本頁保留原功能條件、D01–D09識別及未完成事項；政策不再重複散列。每日功能教學、每日一個全專案工程小工作包、核心60分鐘只計Red／Green，統一依[共用規則](COMMON_LEARNING_RULES.md)。工程交付完整程式／patch與測試方案，產品由使用者套用。

[工程範圍與進度](teaching/functional-refactoring/README.md) · [12組測試對照](teaching/functional-refactoring/test-coverage-map.csv) · [技術責任](SYSTEM_ARCHITECTURE.md) · [當前功能與證據](teaching/current/STATUS.md)

以下日期／SHA的比對與實測摘要保留原來源，不能當成新SHA的通過／失敗；完成條件沿原功能追蹤，不新增平行FIFO。原文與完整歷史可由Git及相應source-review證據追溯。

## 所有問題文章的後續修正（2026-10-25）

本節先前比對來源（歷史）`5c713553daee4055738239f047d6c774fcc7a7a1`；[完整比對與21項差異](source-review/2026-10-25-full-comparison-5c713553.md)。本輪只更新原功能的狀態與證據，不增加Day／FIFO。

[105個目前閱讀版本／111份版本的問題與更正索引](teaching/current/ARTICLE_CORRECTIONS.md)已整併原盤點與最新實測。下表是原功能的集中修正條件，沿用原 D01–D09 及既有項目，不另建任務編號或 FIFO。狀態限各證據來源，不將需求未驗收一律稱為文章錯誤。

<a id="article-api"></a>

### 文章 API／修訂與歷史版本

Chat／Analytics 舊 SSR API、兩個被取代修訂及過時斷言分開記錄；依目前 Leptos API、真實路徑提供後續更正，保留原文。

目前：文章勘誤；部分產品錯誤已修復。完成條件：新補充須編譯並保留有效斷言；不恢復舊 stub 或重複登入頁。

<a id="chat-auth"></a>

### Chat Session／作者／房主與 ACL

2026-10-25 使用者操作回報：已登入首頁仍固定顯示「登入」。最新來源 `dbe00814` 的 HomePage 仍固定 href=/signin。完成條件補充：依伺服器有效 Session 顯示「管理」並進個人管理；未登入／失效顯示「登入」，登入返回、刷新、登出與過期均回歸。[截圖與來源核對](source-review/2026-10-25-chat-homepage-feedback.md)。本項未修復／未驗收。

D05 共用認證保留；WS持續重驗已接並有同SHA局部證據，尚缺原子撤銷競態、匿名進房登入回跳、會員作者及房主／逐房授權。

目前：WS重驗已接／其它需求待完成。完成條件：保留有效WS撤銷，補原房間回跳、真實作者／房主、逐房HTTP／WS授權、活動Session與原子競態驗證。

<a id="chat-history"></a>

### Chat 歷史／migration／稽核

歷史載入未接通；舊 DB checksum／稽核缺口保留原來源，不能用 fresh DB 結案。

目前：歷史實測未通過；升級／稽核待再驗。完成條件：重載還原已提交訊息；舊庫保資料前向升級；要求的稽核真實落庫。

<a id="chat-feedback"></a>

### Chat 房間與訊息回饋

最新648b903增量實測：apply_room_search與真WASM caller已見、UI15項與建置通過；但API500時status未綁DOM，舊清單保留卻無錯誤／重試提示，待修且未驗收。[本次證據](source-review/2026-10-25-day37-verification-648b903.md)。下方5c713553與其他來源診斷保留其歷史範圍。

2026-10-25 使用者操作回報：首頁進入時必須按讀取才有房間；房間頁共用列表也顯示未讀取。最新來源 `dbe00814` 未見首次載入，須補「進入首頁即自動讀取」，保留手動刷新、載入／空／錯誤／重試與舊快照；核對 hydration 重複請求及新舊搜尋競態。[證據與完成條件](source-review/2026-10-25-chat-homepage-feedback.md)。本來源已見 Day37 apply_room_search 與 WASM caller；下段 `5c713553` 診斷保留其歷史範圍，不代表自動載入已完成。

先前5c713553的成功未清狀態診斷已由後續apply_room_search處理；648b903目前缺口是status未綁DOM，不能將helper通過當可見錯誤／重試已完成。MessageCreated沒有解除等待狀態；D06僅遷refreshed_room_list舊測試，refresh_result_view仍有真caller。

目前：本地候選／待套用；其它待修。完成條件：單一房間狀態；成功回應後送出狀態明確；保留搜尋／重試及真正 helper 回歸。

<a id="pet-lifecycle"></a>

### Pet 多寵物／回籠／重啟

最新648b903：出籠先保存／失敗補償及協調鎖已見，62項測試通過；使用者確認控制窗／生活窗／同ID重複出籠局部正常。native故障注入、併行、直接關窗與重啟仍未驗收，不能沿舊來源再說補償完全未套用。[範圍與證據](source-review/2026-10-25-day37-verification-648b903.md)。

歷史三窗與狀態隔離局部通過；最新setup已逐ID恢復全部未回籠資料。預設資格與回籠未分離、直接關生活窗只清Runtime；出籠已改先保存／建窗失敗補償，真native失敗仍待驗；真程序重啟與失敗補償仍待驗。

目前：恢復迴圈已接／原生與需求模型未完整驗收。完成條件：依現行USER_FLOWS區分預設資格與回籠、回籠不取消預設；直接關窗等同回籠；真程序啟動符合預設政策，開窗／關窗／DB失敗可恢復且不重複開窗。

<a id="pet-roaming"></a>

### Pet 逐 ID 漫遊／桌面例外

2026-10-25 使用者再次回報 pet window not found。來源 `744a75fc` 仍以固定 pet 查找動態 pet-UUID 生活窗，找不到即終止漫遊；須按逐 ID 出／入籠生命週期啟停，三窗與重開驗證。另有 WebKit 內部載入錯誤，未定因，不能當成同一錯誤或靠遮蔽日誌結案。[原始訊息、來源與診斷界線](source-review/2026-10-25-pet-roaming-webkit-feedback.md)。兩項均未修復／未驗收。使用者補充「所有視窗空白、每按下出籠都會重新開啟」；先查共用前端載入，再以同 pet id／label／PID／Destroyed 時序分辨重載或重複建窗，尚未定因。

固定 pet label 不符 pet-UUID；接回原漫遊、拖曳、多螢幕及手動找回需求。

目前：日誌與來源確認／待修。完成條件：真實逐窗漫遊；回籠停活動；拖曳與螢幕變動符合現行 UI 條件。

<a id="pet-display"></a>

### Pet 外觀與生活窗可讀性

透明生活窗白字在淺背景不清楚；單幀單圖 FIFO、素材與初始外觀仍依原項目核對。

目前：截圖所見／部分待驗。完成條件：200×200 名稱／狀態／按鈕可讀可操作；一窗一寵物、每幀單一圖像。

<a id="analytics-auth"></a>

### Analytics 正式人員查詢授權

正式exact-kind已改人員Session，來源token單獨查詢遭拒；POST攝取保留機器token。一般帳號、角色／來源ACL及停用縮權仍屬原需求。

目前：精確查詢Session已修正並局部驗證／角色ACL與SSE待完成。完成條件：Query／管理／SSE經人員Session與角色來源權限；上報機器身分另驗；撤銷立即生效。

<a id="analytics-session"></a>

### Analytics 正式登入／Session 保護

初始化、改密碼、舊 Session 撤銷、重啟保留與 Dashboard 已局部通過；正式登入等待及錯誤回應續期需與 legacy 測試分開。

目前：局部通過／尚缺正式流程驗證。完成條件：真實 password_login 路由驗證等待、活動 Session、閒置、改密碼與權限變更撤銷。

<a id="adapter-cleanup"></a>

### 原 13 項清理／D01–D09

沿用下方原清單：Chat unused API／helper／Cookie，Pet unused extern／global IPC／setter，Analytics 範本／legacy admin／fixture／API 相容；已接 caller 的兩個 persistence 庫不再當空庫。

目前：原項目狀態保留／逐項核對。完成條件：先遷有效測試並盤點框架／外部入口再清；initialize fixture 保留；不新增平行產品。

<a id="ci-coverage"></a>

### CI／產物與測試覆蓋

最新648b903不是全綠：6成功、Persistence失敗。Pet step已選pet-server＋pet-persistence但缺GLib／GObject native依賴；Analytics server＋persistence完整PG targets已配置，該步被前一步失敗阻擋。本機完整PG45＋略過另1通過不能替代CI；沿原CI項修正runner並重跑，不新增FIFO。[精確job／錯誤](source-review/2026-10-25-day37-verification-648b903.md)。下段舊workflow快照不當作目前配置。

目前七個workflow成功，兩個persistence package已直接選取；Persistence的server PG工作實測命令是analytics-persistence，Test All的analytics-server未啟用postgres-integration。正式server PG targets、新dist契約與實際安裝驗收仍缺。

目前：library 選取已見；完整覆蓋待驗。完成條件：同 SHA 真 PG server＋library、source＋新 dist、native build／安裝更新；不延用舊 SHA 綠燈。

<a id="event-transport"></a>

### 來源事件／Outbox／gRPC／SSE／服務配對

Chat persisted_message_event 與 Pet succeeded_event 尚未證明正式上報 caller；沿原傳輸、短效憑證及投遞項，不因 DTO 存在當完成。

目前：來源接線／需求待完成或待驗。完成條件：真來源事件→可靠投遞→PG commit ACK→人員查詢／SSE，含重試去重、rollback、換證撤銷。

<a id="pet-capabilities"></a>

### Pet 設定／記憶／檔案／AI 與平台能力

模型、唯讀預覽或確認 UI 不等於真 OS 清理／Trash／receipt／restore；五欄設定、素材、記憶與三平台更新保留原條件。

目前：需求／未驗收範圍，非全屬編譯錯誤。完成條件：按原需求逐最小切片提供真 IPC／DB／OS 證據；付費 AI 不在本輪呼叫。

<a id="analytics-capabilities"></a>

### Analytics 報表／Token 費用與跨服務能力

趨勢／明細／備份／同意撤回、Token／額度／費用及 AI 洞察維持原待辦；不由統計 API 的局部成功結案。

目前：需求／未驗收範圍。完成條件：依原功能條件核對真來源與精確驗證；未知額度不可冒稱已取得。

<a id="chat-capabilities"></a>

### Chat 即時／AI／政策與進階 UI

presence／typing／reconnect／queue、streaming／RAG／MCP／Agent／Persona、同意撤回與窄螢幕 UI 等維持既有範圍。

目前：需求／未驗收範圍。完成條件：基本登入授權優先，後續逐切片驗證真 UI／HTTP／WS／DB，不用 helper 冒充完整能力。

<a id="layout-guidance"></a>

### 歷史路徑／庫名／政策與範例判讀

歷史舊庫名、舊路徑、P9 與省略候選只作閱讀提示；後續教學按新目錄給完整必要程式，不恢復廢除搬移計畫。

目前：閱讀注意／套用前核對，非自動認定錯誤。完成條件：四個 workspace／package／lock／features／真 caller 對齊；必要版本在實際新教學交付時核對。

## 後續調整：重複與未使用程式、測試及產物（2026-10-25）

本節維持原 13 項及 D01–D09 識別，下表原13項逐項快照來源為 `williechen/rust_ai_ring@afdbb6e81382e8d82095827a90c52cc3309a3e20`；目前5c713553的變化見本節最新比對入口；每次動手仍須重新核對最新 main 與 caller。[本次重掃報告與 CI／來源一致性證據](source-review/2026-10-25-adjustments-rescan-afdbb6e.md) 保留為原13項的定點核對，歷史原報告與已發布文章保留。

原 13 項中，7 項待清理／測試遷移／相容性盤點，2 項部分完成，2 項程式或建置已處理但驗收未完，1 項明確保留，1 項已結案。已完成的登入頁合併、房名共用、Pet 回籠 adapter、Analytics 精確查詢 adapter、編譯及文案修復不重教；未完成的權限、GUI 與 FIFO 不因此結案。

本次沒有產品變更，沿用同 SHA 的 Analytics 44 passed／0 failed／1 ignored，以及經 git diff 確認來源相同的 Chat／Pet 原範圍結果；沒有冒稱重新跑整套測試。該afdbb6e快照七個push workflows為success；648b903目前為六success／Persistence失敗，不能沿用為全綠。該快照不決定當前教學進度；每日工程依相依性接續，功能教學依即時next_day。

### 清理順序與保留條件

同案先處理使用者可見流程與來源／產物一致性，再做無呼叫者清理及必要分層。下表順序是功能依賴與優先序，並非每日排程或新增一套待辦編號；原有 D01–D09 與既有功能項持續追蹤。

| 專案／順序 | 後續調整與位置 | 清除／保留判準 | 必要驗證與目前狀態 |
| --- | --- | --- | --- |
| Chat 1／D01（已實作／驗收待做） | 單一 SignInPage、GET /login redirect 與 POST /login 已接通；login.rs／多餘模組已移除 | 不再套一次 r2；保留正式登入／Session 契約 | Chat 35＋UI 14 原範圍結果與本 SHA CI 通過；hydration／瀏覽器及完整登入流程驗收仍待完成 |
| Chat 2／未使用函式 | 移除 chat_ui/src/room_api.rs 的 create_room 及僅供它使用的 CreateRoomRequest/CreateRoomResponse、JsValue、Request/RequestInit/RequestMode imports | 私有 room_api 模組內未找到此函式 caller；保留 list_rooms/search_rooms 與 server JSON API、個人頁建房流程 | WASM check、房間查詢／重新整理及個人頁建房；待清理 |
| Chat 3／D06 | 將 refreshed_room_list 的舊測試轉至 RoomList 實際呼叫的 refresh_result_view，再移除舊 helper | 保留空結果、失敗保留清單／搜尋字、成功清除搜尋與重試條件；不直接刪掉回歸斷言 | 測真正 helper 及 RoomList，SSR／WASM 與瀏覽器回歸；待遷移 |
| Chat 4／D05、D07 | D07 建房名稱驗證已移到 chat_application::validate_room_name，JSON／個人頁皆接回；D05 Cookie／digest 共用仍待清理 | 保留 400 JSON／422 HTML、Session／Origin／CSRF；控制字元、200 UTF-8 bytes 邊界與原字串規則已有測試，不重做房名 helper | e753c36 application／server／整合 35 passed，UI 14 passed；共用認證 FIFO 與完整瀏覽器驗收仍未結案 |
| Pet 1／D02（來源清理及 CI build 已過） | 舊 tracked dist／public／gen 已移除，以 source bridge 重建；本 SHA CI release 前端／native build／產物上傳成功 | 不手改雙份程式；不將 build 或上傳當已安裝版本／GUI 驗收 | 原 source／新 debug dist 六契約通過；後續將同契約接到 CI 新 dist，並驗實際 binary／原生三窗、重啟與更新 |
| Pet 2／未使用宣告 | 清除 pet_ui/src/bridge.rs 的 validate_new_pet_name_js extern 宣告 | 未找到 Rust wrapper/caller，現行 index.html 也未提供 validateNewPetName；不因此刪掉 PetStore 的必要名稱驗證 | WASM check、建立寵物的非法名稱不寫 DB／合法名稱儲存；待清理 |
| Pet 3／D03 | 盤點舊 get_pet_state/send_pet_command、global Mutex<PetMachine>、pet://state-changed、舊選單分支；public 原型已刪 | 舊 IPC 仍在 generate_handler 註冊，不是零入口。確認現行／相容客戶端後才連同註冊、state、舊資產與專屬測試清理；新逐 ID Runtime 保留 | 三個不同 ID 的狀態／事件、互動與選單、回籠／重開／重啟，依原 GUI 項目驗證；後端 global state／event／無 ID fallback 仍待清理，不能誤刪已映射逐 ID IPC 的現行 UI wrapper |
| Pet 4／D08 | 移除掃描 handler 內多餘 reset／loading setter | 每次 await／過時 generation 早退仍須正確釋放 busy，不刪除唯一必要清理；未證實有雙重 scan invoke | 預覽成功／失敗／取消／過時回應與重新點選；待清理 |
| Analytics 1／D09 | 清除 analytics_application/domain/ui 中三份 add 函式與 it_works 範本測試 | 三檔是預設範本，未接業務 caller；保留已有 workspace/package，無必要不新增分層骨架，也不將空 library 當功能完成 | Cargo metadata／workspace check；待清理 |
| Analytics 2／D04 | 盤點並淘汰舊 admin.rs 登入碼／memory Session 後台與其專屬測試、Config／模組出口 | main/build_app 目前採 password_login::build；確認無相容 caller 再移除舊公開 API，不刪除正式帳密／Session／PostgreSQL 測試 | 正式登入／首次改密碼／後台／登出與 Session 撤銷；角色／來源 ACL 原缺口保留，待核對／清理 |
| Analytics 3／測試 fixture | 保留 bootstrap::initialize；僅在 fixture 完整遷移後評估縮為測試支援或移除 | day23_password_login、day24_password_change、day25_repeat_password_change、day26_session_contract、day27_dashboard_source 等整合測試仍直接使用；不能因 main 不呼叫就刪掉 | 原測試的自訂初始密碼、重複初始化／hash 保留、Session／Email／權限條件均須保留；目前保留 |
| Analytics 4／測試 handler／正式查詢 | source_summary 仍 cfg(test)；正式精確查詢已接 persistence；bb95a60 測試改用新函式路徑並局部建立 fixture | 五個 E0425 已消除；未新增相容 re-export，舊公開 pg_store 路徑已變更。此列afdbb6e的來源token診斷是歷史；648b903正式查詢已用人員Session，角色／來源ACL仍缺 | 全部 PG 測試目標可編譯；Day35 live Router／來源隔離／commit／rollback 實測通過。仍須對齊查詢授權需求，不能將 Router 成功當成完整權限驗收 |
| Analytics 5／過時斷言（已修復） | afdbb6e 在三檔更新四處「已提交事件總數」舊文字，涵蓋 Day27 Chat／Pet 與 Day28／29 | 保留來源切換、HTTP／Session、連結、可及性、escaping 等既有斷言，未刪測試；原識別與失敗證據保留 | 完整 PG 回歸 44 passed／0 failed／1 ignored、exit 0。此三敗已結案，不重教；其它授權／產品驗收仍待辦 |


### 目前來源補充：接回既有功能與測試項

- Chat 作者／身分：require_member 驗證後未往 handler 傳 member ID，WebSocket 用例仍建立 AnonymousUser 訊息，建房 repository 未綁真實房主。接回既有身分／作者／ACL／共用認證項，不另建身分表。
- Pet 漫遊仍只查固定 `pet`，實際生活窗為 `pet-{UUID}`；目前setup已逐筆讀uncaged並開窗；預設資格／回籠模型、直接關窗持久化及失敗補償仍未完整。接回原多寵物／漫遊／重啟驗收項，先接真實生命週期，不能以固定 label 或 DB 旗標測試結案。
- Analytics精確查詢已改人員Session；角色／來源ACL及正式登入政策需求落差仍在。已編譯、CI 成功與來源隔離測試通過均不足以驗收人員權限；source_summary 的測試遷移及舊公開 API 路徑相容性仍分別核對。
- Pet set_caged 與 Analytics 精確查詢已有 persistence caller，不再當空庫；其餘 SQL 搬移只按實際切片處理。initialize fixture／InMemoryDeduplicator 契約測試保留用途。
- Chat persisted_message_event 未見 caller，Pet succeeded_event 只有測試 caller；接回既有真實事件／Analytics／Outbox 項，不因無 caller 就當垃圾刪除，也不因有 DTO 宣稱已上報。
- CI覆蓋：兩個persistence library自身測試已直接選取；648b903的persistence.yml已選analytics-server＋analytics-persistence並加postgres-integration；目前需修復前段Pet native依賴後取得該步驟實際通過證據。metadata／dependency feature不等於執行dependency tests。
- Pet Node workflow 仍只讀 source HTML；CI build 後需同測新 dist，接回 D02，不維護平行 bridge 邏輯。

### 每項調整的最小交付

1. 在最新精確 SHA 查清 Rust caller、mod/pub export、Axum route、Tauri generate_handler／menu、WASM/JS bridge、cfg/feature、測試與 CI 入口；跨 workspace 和 tracked dist 也需查看。搜尋不到一般函式呼叫不等於沒有框架或外部入口。
2. 確認可刪才一併清掉函式、專用 imports/dependencies/features、註冊、已失效的獨占測試或範本。有效回歸測試先遷到真實 caller；每個被移除測試要能說明驗證條件已由何處保留，或其測試目標確實已淘汰。
3. 工程提供完整變更內容 的新增／修改／刪除／必要搬移、Cargo/lock、imports、caller 與測試；重用已存在實作。從對應 workspace 驗證 native／WASM、HTTP/IPC、DB/GUI；未執行項目明記。
4. 教學發布與產品套用／驗收分開；未有實測證據不標完成。保留原篇、來源 SHA、commit/blob/SHA256；修訂、清理檢核與本次整理不加 Day、不結案 FIFO、不更新 last_full_comparison。

### 明確保留與不誤刪範圍

- Rust #[test]/#[tokio::test]/#[sqlx::test] 由測試框架收集；是否執行也受 cfg/feature/target 影響，沒有一般 caller 不是刪除理由。
- bootstrap::initialize 與整合測試 fixture 有真實用途；主啟動／CLI 使用 ensure_default 亦已接通，兩種入口用途分開記錄，不重教初始化。
- Pet application 的名稱規則已由 Tauri/PetStore 共用，保留該規則及非法輸入的 DB 回歸；server/bridge 傳輸 DTO 相似也不能當成重複寫入。
- 各產品 SQLite／PostgreSQL、Session／Cookie、migration 維持獨立；四個 workspace 的 manifest/lock/toolchain 依原架構保留。
- cfg(unix)/cfg(not(unix)) 的互斥平台函式、真實 JSON/HTML 操作入口、Query／Ingest 不因函式名稱或資料形狀相似而刪除。
- 本清單沒有授權直接改寫 rust_ai_ring/main；維持先前分工，產品調整由後續教學與使用者套用。本次只向 copilot_use/main 整理規則、缺口與入口。

## 清單判讀

Checkbox代表原功能尚待完成／核實，不表示每個底層都未實作。以最新產品來源與證據去重追蹤；工程、教學與產品驗收分列，規則依[共用正本](COMMON_LEARNING_RULES.md)。

## 當前最高優先順序：先接通可操作的基本流程

舊文章與目前程式的核對以 [最新逐篇狀態](teaching/current/ARTICLE_STATUS.md) 為準；舊84篇矩陣、固定來源 cold-build／瀏覽器／GUI 證據保留各自來源，不直接當成最新通過或失敗。已見程式的切片不重建，原文版本／路徑及 SSR 勘誤、實際功能與未驗收缺口分開調整。
這是目前已確認功能的實作順序與驗收條件；實際缺口及最新來源見[目前進度](teaching/current/STATUS.md)。每次依真實入口、資料庫與測試調整最小切片，不預排Day，也不以其他分支的舊試作當成已合併／驗收。
- [ ] **Chat：註冊／登入→Session→公開房間清單與進房授權→加入／建立聊天室。** 沿 rust_ai_ring 的 signup／signin／logout、mbr_user／Session 與房間授權實際來源核對；不引用舊庫 auth.rs／chat_users 介面。先完成欄位驗證與登入授權，不重建平行身分表。基本鏈未完成前不優先擴充歷史訊息或進階UI。
- [ ] **Pet：建立第一隻寵物→名稱／初始外觀→Tauri IPC→SQLite→生活窗→重啟載入。** 沿 rust_ai_ring 的 create_pet／pet_store／pet_profile 與 Tauri command 實際來源核對；補齊逐寵物身分／畫面／載入後再深化多寵物與漫遊。先核對舊DB歷史，以前向migration處理相容性，不清空DB。
- [ ] **Analytics：管理員初始化→持久化帳密登入→首次改密碼→Session／Dashboard授權。** 沿 rust_ai_ring bootstrap／password_login／Session 的最新來源，核對真正登入／改密碼／授權鏈及未完成部分；固定key登入或429回應不等於完成帳密身分流程。
每日仍以Chat→Pet→Analytics各一個可套用的Red→完整最小Green切片推進，未接通部分明示缺口。舊Day更正在本日新實作／Refactor後補充；每新增一個專案Day獨立GitHub commit推送並全文讀回後，才可條件同步next_day及manifest，清單整理本身不增加Day。已確認功能條件、權限／資料安全、週六工程驗收與其他待辦保留。

## 既有實作的驗收邊界

[2026-10-16 舊庫 Linux 證據](weekly/2026-10-16/acceptance.json) 保留原來源，不作 rust_ai_ring 的完成證據。原功能是否已實作、是否有新增需求與跨平台缺口，須在主體最新 SHA 核對；不因來源角色調整重教已完成內容。

- [ ] Pet 漫遊後續需求：多寵物獨立生活窗、出入籠與互動、螢幕移除不立即搬回／手動找回；逐項按 [UI 需求](UI_ALIGNMENT_REQUIREMENTS.md) 核對目前來源、所屬測試及真實桌面結果。
- [ ] 漫遊與 SHA256 分組的 Windows／macOS 功能驗收，以及相對原 Linux 證據新增程式的回歸；沒有匹配證據前不標完成。

## 教學銜接

學習來源現況與驗證範圍見[目前進度](teaching/current/STATUS.md)。Chat／Pet 各自依 FIFO 補教尚未完成的部分；Analytics Day24 已結案。下一 Day 即時值只引用 [learning-state](learning-state.json)，不在此複製固定 Day。主體程式套用與功能驗收分開追蹤。

來源、進度與驗證結果統一見[目前進度](teaching/current/STATUS.md)。Day25（含）以前文章封存；以下沿用原功能項追蹤，不代表產品已整合或驗收。

以下保留原有學習範圍，checkbox 表示學習／適配未完成，不表示對應成品功能全部未實作。每次依最新 source、所屬測試與 CI 排除已套用項目，再選最小切片。歷史驗證保留原來源，新一輪只更新統一進度與對應原項目，不重建待辦。

### 共同主線

2026-10-23 依使用者確認，舊「分批搬移 11 個測試檔」及「依目錄、型別、Port、介面與所屬測試更新教學對應及理由／建立平行程式」兩項已廢除，已移出活動待辦；不作重構計畫、教學選題或驗收要求。[廢除紀錄與原文](teaching/history/evidence/2026-10-23-retired-legacy-work-items/REPORT.md)僅供追溯。


- [ ] 三專案各自產出完整可套用 Day；四節格式、原教學與第 2 節末尾必要實作補充對應一致。
- [ ] 每週六完整核對 rust_ai_ring 的 source／介面／流程／契約／DB／UI／CI／測試，再核對 copilot_use 文件、教學與證據的對應；既有執行流程尚待調整見 [同步範圍](REPOSITORY_ROLE_TRANSITION.md)。

### Chat

原有 Session／ACL 與 UI 項目納入共同規則本輪確認的公開清單、房名登入導向、個人管理、註冊獨立完成頁、admin/admin 首次改密碼及 Email、帳號錯誤等待逐次加倍無上限、活動 Session／後台閒置設定及密碼變更撤銷。需求已確認，實作／驗收待逐項核對。

- [ ] 訊息／作者／房間模型、MessageRepository Port、SQLite schema／history／備份及持久化後通知。
- [ ] Session／ACL、HTTP／WS 房間隔離、presence／typing／reconnect／queue、durable realtime／Outbox／故障恢復。
- [ ] Leptos SSR／hydration／protected routes、真實 streaming／RAG／MCP／Agent／多 Persona。 首頁僅聊天室清單，點選後進房；建立／管理放個人管理頁。房間查詢須發出真實請求，並驗證載入、失敗恢復／重試與清單保留。房間頁採房間欄／訊息氣泡／多行輸入與真實操作回饋，窄螢幕可收合房間欄；個人與系統管理路由分開。驗收條件見 [畫面銜接需求](UI_ALIGNMENT_REQUIREMENTS.md)。
- [ ] 同意／撤回、來源事件、OTel 與可審核政策採用／CAS 回退，跨服務 ID 對應與 E2E。

### Desktop Pet

原有多寵物／漫遊／UI 項目納入獨立狀態與生活窗、多預設出籠、回籠停活動、右鍵坐下／控制窗選取、習性速度、打鬧吵醒、拖曳與多螢幕例外／手動找回。詳細依共同規則及 UI 需求；不提供固定速度設定，不以單 pet 共用狀態取代多寵物需求。未重新實作／驗收。

- [ ] 純 Profile／Application Port／Schedule 與 SQLite／HTTP／OS runtime 的邊界。
- [ ] Tauri／Leptos IPC、五欄設定及可選覆寫、多寵物獨立 TOML／資產／備份與可逆封存。rust_ai_ring 畫面需求：200×200 寵物顯示與可縮放／捲動的控制視窗分離；六區為互動、設定、記憶、檔案、管理、更新，未接通能力明示，反覆開啟只聚焦既有視窗。漫遊只移動寵物生活窗，控制窗保留手動拖曳；需以完整原生GUI／IPC驗收。詳細條件見 [畫面銜接需求](UI_ALIGNMENT_REQUIREMENTS.md)。
- [ ] 使用者授權、型別化 preview／confirmation／實際 receipt、private Trash／restore、symlink／TOCTOU／journal／OS 安全。
- [ ] AI 外觀／素材與角色一致性、200×200 等比動畫、記憶／回想／複習／提醒、RAG／MCP／Agent。
- [ ] 真實事件至 Analytics、政策確認／回退、三平台 GUI／installer／簽章 updater 與資料保留。

### Product Analytics

原有後台與來源認證項目納入管理者建帳（無自行註冊）、首次改密碼／唯一 Email、與 Chat 相同但獨立累計的登入保護、活動 Session、管理者／唯讀角色及 Chat／Pet 資料範圍、立即停用／縮權與不含秘密的稽核。獨立服務配對／短效憑證／自動輪替撤銷沿用原項目。實作／驗收待核對。

- [ ] AI Token／費用與額度統計：按 Provider／Model／來源產品／功能／請求 ID／時間，記錄供應商實際回傳的 input／output／cached input／reasoning／total tokens、價格版本與估算／實際費用；日／月趨勢、預算警示及可取得的 rate limit／quota／remaining／reset，缺失額度顯示「未知」。失敗、重試、串流中斷、缺失 usage 要去重與補帳；Token 不等於配額或帳單；不儲存 API key、Prompt、回應本文。以既有紀錄／fixture／mock 驗證，禁止讀密鑰及付費 AI 呼叫。

- [ ] 教學銜接 gRPC／Protobuf 契約、來源認證／逐事件 ACK／Outbox 重試，以及 HTTP＋SSE 的 commit／rollback／重連；產品實作與驗證入口見 [ANALYTICS_TRANSPORT.md](ANALYTICS_TRANSPORT.md)。
  來源認證延伸為服務間核准配對、短效存取、自動交互換證、輪替與撤銷；Leptos 提供管理狀態，實際換證由來源與後端交互完成。驗收條件見 [交互憑證需求](SERVICE_CREDENTIAL_REQUIREMENTS.md)，既有固定 token 僅為過渡實作，未完成此需求。

- [ ] 四欄產品事件及來源權限、EventRepository Port、獨立 PostgreSQL migration／交易／持久冪等／衝突與故障恢復。
- [ ] 同服務 Dashboard／Ingest／Query、Taipei 趨勢／明細、可逆封存／同意撤回、備份還原。Analytics 後台首次初始化 `admin/admin`，帳密登入後可修改密碼，持久保存加鹽雜湊且重啟不重設；服務 token 分開。除登入／必要驗證外的頁面、Query／管理 API／SSE 均須人員 Session 與權限；服務上報／換證另用機器身分，禁止匿名 Dashboard。詳細條件見 [畫面銜接需求](UI_ALIGNMENT_REQUIREMENTS.md)。畫面沿用 Leptos 元件與 Rust 狀態，呼叫既有 HTTP JSON API；不維護獨立 HTML／JavaScript 互動頁。Day22（含）以前均封存，後續修正於下一Day新實作／Refactor後提出；條件見 [畫面銜接需求](UI_ALIGNMENT_REQUIREMENTS.md)。
- [ ] 聚合快照、可追溯 AI 洞察、人工審核、Chat／Pet 自主確認採用、收據／描述性評估／回退。
- [ ] 事件與 OTel 分離、跨服務 schema／ID／retry／trace、GUI／HTTP／DB／E2E／CI。

## 完成條件

逐項附規格 → 當前檔案／符號／SHA → rust_ai_ring 真正測試／CI → 程式套用狀態。rust_ai_ring 產品功能驗收、每日教學發布、主體程式實際套用各自記錄，適用 CI 未全部 Success 仍可發布每日 Day，但須揭露失敗與待修問題，不宣稱驗證通過，不等待週六提交。未完成或未核實項目留在對應分類；完成項目可從活動清單移除，但原項目識別、跨階段補加歷史及完成證據須永久可追溯。新產品需求需同步現行需求文件、實作與重新驗收；不以舊 Release 或教學頁宣稱新需求已完成。

### Chat瀏覽器與SQLite最新實證

[2026-10-23後續驗收](teaching/history/evidence/2026-10-23-browser-sqlite-9c2b7cf/REPORT.md)：列表／中文搜尋／刷新故障保留及重試已實機通過；不再列為單純「待瀏覽器驗收」。9c2b7cf當時尚缺個人頁建房UI；最新be0494a對應生產程式已接通，真瀏覽器個人頁建房與進房通過。這補足操作入口，但不改寫Day15原文或追認該篇其他Gate全部通過。原SQLite／稽核項仍未完成：歷史migration升級checksum不符（資料保留），created_user_id未綁真實身分，註冊登入與發訊沒有action log寫入。保留原待辦與歷史正文，不結案整個Chat產品。
