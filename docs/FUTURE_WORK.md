# 未來前進項目

現行程式來源與目錄：[rust_ai_ring 四個 workspace 對照](SOURCE_LAYOUT.md)（來源 `0b3e5844`，2026-10-24 靜態核對）。

> 實作進度、精確來源與驗證結果統一見[目前進度](teaching/current/STATUS.md)。本文件保留功能需求／完成條件或架構；歷次測試摘要已移入歷史紀錄，不作現行完成聲明。

需求依使用者最新確認內容及[現行使用者流程](requirements/USER_FLOWS.md)、[人員驗證需求](requirements/AUTHENTICATION.md)、[UI 驗收條件](UI_ALIGNMENT_REQUIREMENTS.md)，實作缺口、目錄／內容、測試／CI 與功能驗收只依 rust_ai_ring 最新 main。本庫保存需求與教學，不另追蹤 copilot_use 產品程式；未核實不標完成。每天從確認缺口與當日差異選三專案合計一小時的小切片，不預排 Day；週六工程與功能驗收另記。

P9 規則已廢除。本清單中僅源自舊規則、尚未依最新需求確認的項目只作候選，不自動作必做功能或驗收門檻。已確認的開發項目在原項目記錄可驗證功能條件、SHA／測試／CI／套用與驗收證據，未核實者保留未完成。跨階段工作留原功能項目追蹤；舊 Day 更正僅於下一個 Day 的 Refactor 後獨立補加，不回填原 Day，不另開重複項目；完成後可退出活動清單，但原識別與證據須可追溯。學習階段不代表產品功能已驗收。

## 舊內容與現存狀態（2026-10-25）

[99 篇目前閱讀版本與舊修訂逐篇狀態](teaching/current/ARTICLE_STATUS.md) 已核對原文、現存程式與歷史證據。程式已見、已被後續取代、legacy 專用與原文待勘誤分開追蹤；本次不新增完成 checkbox 或改變既有 FIFO。Pet 名稱規則、控制窗 hide/show、Analytics Leptos 帳密與 ensure_default 已見 caller，不再以原文「未套用」重教；最新 3aeab095 已見 Chat r2 去重與測試；歷史／作者／ACL 等缺口保留，當次驗收未重跑。

## 後續調整：重複與未使用程式、測試及產物（2026-10-25）

本節統整本輪全部核對與使用者「檢核清除不使用函式與測試」的要求，作為後續調整單一清單。最新增量驗證基準為 `williechen/rust_ai_ring/main@afdbb6e81382e8d82095827a90c52cc3309a3e20`；每次動手前重讀最新來源與 caller，不固定套用此 SHA。最新 [afdbb6e 斷言修復驗證](source-review/2026-10-25-assertions-afdbb6e.md) 已確認舊文案三敗消除，完整 PG 回歸 44 passed／0 failed／1 ignored；[bb95a60 編譯修復](source-review/2026-10-25-compile-bb95a60.md) 保留歷史；Day35 前次套用、測試及待修證據見 [e753c36 更新驗證](source-review/2026-10-25-update-e753c36.md)；前次逐項結果見 [2026-10-25 專案重掃](source-review/2026-10-25-project-rescan.md)；原 [D01–D09 報告](source-review/2026-10-25-duplicate-application-review.md) 保留其歷史來源，不再以舊 dist 失敗描述最新 main。

本次完成來源重掃及部分實測，下列程式清理未由本次任務套用；已有來源實作的 Pet 名稱規則、Analytics ensure_default 不重教。Chat 既有 r2 已在最新來源接通，先核對驗證／排除已套部分，後續由各案即時 next_day 選一個最小切片，仍依 Chat→Pet→Analytics 交付，不預排 Day，不要求每日改完所有項目。各項完成須附實際變更 SHA、caller／註冊與相應測試證據。

### 清理順序與保留條件

同案先處理使用者可見流程與來源／產物一致性，再做無呼叫者清理及必要分層。下表順序是功能依賴與優先序，並非每日排程或新增一套待辦編號；原有 D01–D09 與既有功能項持續追蹤。

| 專案／順序 | 後續調整與位置 | 清除／保留判準 | 必要驗證與目前狀態 |
| --- | --- | --- | --- |
| Chat 1／D01 | [Day34 r2](teaching/daily/day-34/chat-r2.md) 登入頁合併已套用：SignInPage 保留，GET /login 導向 /signin，login.rs 與多餘模組已移除 | POST /login 保留；排除重複套用 r2，原合併不是零 caller 刪除 | 891ff32 實測 chat-ui SSR 14 passed、HTTP redirect 1 passed；完整登入／Session／SQLite、hydration 與瀏覽器回歸仍待驗收 |
| Chat 2／未使用函式 | 移除 chat_ui/src/room_api.rs 的 create_room 及僅供它使用的 CreateRoomRequest/CreateRoomResponse、JsValue、Request/RequestInit/RequestMode imports | 私有 room_api 模組內未找到此函式 caller；保留 list_rooms/search_rooms 與 server JSON API、個人頁建房流程 | WASM check、房間查詢／重新整理及個人頁建房；待清理 |
| Chat 3／D06 | 將 refreshed_room_list 的舊測試轉至 RoomList 實際呼叫的 refresh_result_view，再移除舊 helper | 保留空結果、失敗保留清單／搜尋字、成功清除搜尋與重試條件；不直接刪掉回歸斷言 | 測真正 helper 及 RoomList，SSR／WASM 與瀏覽器回歸；待遷移 |
| Chat 4／D05、D07 | D07 建房名稱驗證已移到 chat_application::validate_room_name，JSON／個人頁皆接回；D05 Cookie／digest 共用仍待清理 | 保留 400 JSON／422 HTML、Session／Origin／CSRF；控制字元、200 UTF-8 bytes 邊界與原字串規則已有測試，不重做房名 helper | e753c36 application／server／整合 35 passed，UI 14 passed；共用認證 FIFO 與完整瀏覽器驗收仍未結案 |
| Pet 1／D02 | 舊 tracked dist、public 原型及生成 schema 已移除；以 pet_ui/index.html 為正本核對新建置 WASM／JS／CSS 與 Tauri 實際產物 | source/dist 是正常建置關係，不手改兩份程式；舊 dist 二敗只屬歷史紀錄，不代表最新 binary | 891ff32 source 六個 Node 測試通過；Trunk debug 建置成功，新 dist 沿用原六個契約全過。release、打包／原生 GUI 尚待驗收 |
| Pet 2／未使用宣告 | 清除 pet_ui/src/bridge.rs 的 validate_new_pet_name_js extern 宣告 | 未找到 Rust wrapper/caller，現行 index.html 也未提供 validateNewPetName；不因此刪掉 PetStore 的必要名稱驗證 | WASM check、建立寵物的非法名稱不寫 DB／合法名稱儲存；待清理 |
| Pet 3／D03 | 盤點舊 get_pet_state/send_pet_command、global Mutex<PetMachine>、pet://state-changed、舊選單分支；public 原型已刪 | 舊 IPC 仍在 generate_handler 註冊，不是零入口。確認現行／相容客戶端後才連同註冊、state、舊資產與專屬測試清理；新逐 ID Runtime 保留 | 三個不同 ID 的狀態／事件、互動與選單、回籠／重開／重啟，依原 GUI 項目驗證；後端 global state／event／無 ID fallback 仍待清理，不能誤刪已映射逐 ID IPC 的現行 UI wrapper |
| Pet 4／D08 | 移除掃描 handler 內多餘 reset／loading setter | 每次 await／過時 generation 早退仍須正確釋放 busy，不刪除唯一必要清理；未證實有雙重 scan invoke | 預覽成功／失敗／取消／過時回應與重新點選；待清理 |
| Analytics 1／D09 | 清除 analytics_application/domain/ui 中三份 add 函式與 it_works 範本測試 | 三檔是預設範本，未接業務 caller；保留已有 workspace/package，無必要不新增分層骨架，也不將空 library 當功能完成 | Cargo metadata／workspace check；待清理 |
| Analytics 2／D04 | 盤點並淘汰舊 admin.rs 登入碼／memory Session 後台與其專屬測試、Config／模組出口 | main/build_app 目前採 password_login::build；確認無相容 caller 再移除舊公開 API，不刪除正式帳密／Session／PostgreSQL 測試 | 正式登入／首次改密碼／後台／登出與 Session 撤銷；角色／來源 ACL 原缺口保留，待核對／清理 |
| Analytics 3／測試 fixture | 保留 bootstrap::initialize；僅在 fixture 完整遷移後評估縮為測試支援或移除 | day23_password_login、day24_password_change、day25_repeat_password_change、day26_session_contract、day27_dashboard_source 等整合測試仍直接使用；不能因 main 不呼叫就刪掉 | 原測試的自訂初始密碼、重複初始化／hash 保留、Session／Email／權限條件均須保留；目前保留 |
| Analytics 4／測試 handler／正式查詢 | source_summary 仍 cfg(test)；正式精確查詢已接 persistence；bb95a60 測試改用新函式路徑並局部建立 fixture | 五個 E0425 已消除；未新增相容 re-export，舊公開 pg_store 路徑已變更。新正式查詢仍只用來源 token，與人員 Session 需求有落差 | 全部 PG 測試目標可編譯；Day35 live Router／來源隔離／commit／rollback 實測通過。仍須對齊查詢授權需求，不能將 Router 成功當成完整權限驗收 |
| Analytics 5／過時斷言（已修復） | afdbb6e 在三檔更新四處「已提交事件總數」舊文字，涵蓋 Day27 Chat／Pet 與 Day28／29 | 保留來源切換、HTTP／Session、連結、可及性、escaping 等既有斷言，未刪測試；原識別與失敗證據保留 | 完整 PG 回歸 44 passed／0 failed／1 ignored、exit 0。此三敗已結案，不重教；其它授權／產品驗收仍待辦 |


### 本次重掃補充

- e753c36 已接通 Pet set_caged 與 Analytics 精確查詢 persistence，不再當成空 library；Pet 55 項本機測試通過，Analytics persistence 2 項 PG 測試通過。完整 GUI／瀏覽器與其它 SQL 搬移仍未完成。


- Chat product_event::persisted_message_event 未見本庫 caller；Pet cleanup_event::succeeded_event 僅見契約測試。沿既有真實事件／Analytics／Outbox 項目核對接線，不把公開契約直接當垃圾刪除，也不因型別存在宣稱上報完成。
- EventIdentity／InMemoryDeduplicator 有內部與整合測試用途；正式 ingest 走 pg_store。記憶體契約成功不代表持久化冪等驗收。
- Pet Node workflow 目前只驗來源 HTML，建議 build 後同測新 dist。本次本機 debug 產物契約已通過，不等於 release／安裝 binary／原生 GUI 通過。

### 每項調整的最小交付

1. 在最新精確 SHA 查清 Rust caller、mod/pub export、Axum route、Tauri generate_handler／menu、WASM/JS bridge、cfg/feature、測試與 CI 入口；跨 workspace 和 tracked dist 也需查看。搜尋不到一般函式呼叫不等於沒有框架或外部入口。
2. 確認可刪才一併清掉函式、專用 imports/dependencies/features、註冊、已失效的獨占測試或範本。有效回歸測試先遷到真實 caller；每個被移除測試要能說明驗證條件已由何處保留，或其測試目標確實已淘汰。
3. 完整提供當次最小 Green 的新增／修改／刪除／必要搬移、Cargo/lock、imports、caller 與測試；重用已存在實作。從對應 workspace 驗證 native／WASM、HTTP/IPC、DB/GUI；未執行項目明記。
4. 教學發布與產品套用／驗收分開；未有實測證據不標完成。保留原篇、來源 SHA、commit/blob/SHA256；修訂、清理檢核與本次整理不加 Day、不結案 FIFO、不更新 last_full_comparison。

### 明確保留與不誤刪範圍

- Rust #[test]/#[tokio::test]/#[sqlx::test] 由測試框架收集；是否執行也受 cfg/feature/target 影響，沒有一般 caller 不是刪除理由。
- bootstrap::initialize 與整合測試 fixture 有真實用途；主啟動／CLI 使用 ensure_default 亦已接通，兩種入口用途分開記錄，不重教初始化。
- Pet application 的名稱規則已由 Tauri/PetStore 共用，保留該規則及非法輸入的 DB 回歸；server/bridge 傳輸 DTO 相似也不能當成重複寫入。
- 各產品 SQLite／PostgreSQL、Session／Cookie、migration 維持獨立；四個 workspace 的 manifest/lock/toolchain 依原架構保留。
- cfg(unix)/cfg(not(unix)) 的互斥平台函式、真實 JSON/HTML 操作入口、Query／Ingest 不因函式名稱或資料形狀相似而刪除。
- 本清單沒有授權直接改寫 rust_ai_ring/main；維持先前分工，產品調整由後續教學與使用者套用。本次只向 copilot_use/main 整理規則、缺口與入口。

## 新目錄銜接的後續教學要求

- [ ] 每次後續教學依 rust_ai_ring 最新目錄提供必要的程式調整；依實際情況同步檔案搬移、Cargo dependency / feature、Rust imports、呼叫端與測試。application / persistence 依本次來源核對接線；未接通的 library 不因目錄存在記為完成，已接通的 Pet 名稱規則不重做。
- [ ] 發布前核對最新來源與尚未發布草稿，保留已發布文章及歷史證據；不因更名額外新增 Day，也不在 copilot_use 建立平行產品。

## 清單使用方式

2026-10-23 依使用者確認，所有涉及 copilot_use 程式開發、移植、同步適配、重構、套用、建置、測試、CI、產品試驗、發布、安裝更新與功能驗收的工作要求均已廢除，不再列為待辦、選題或驗收門檻。copilot_use 僅維護需求、規則、教學文章、進度、索引與交付證據；文件內的教學程式碼用於說明 rust_ai_ring 的實作，不建立第二套產品。程式功能、測試、CI 與驗收只依 rust_ai_ring 精確 SHA。既有 copilot_use 程式、腳本、workflow、Release 及歷史欄位僅供追溯，不代表仍有維護或執行義務。

下列 checkbox 表示產品或教學仍待核對／完成的條件，不能據此推論所有底層程式都不存在。主體程式狀態見 [目前進度](teaching/current/STATUS.md) 的來源與證據範圍；舊 copilot_use token Dashboard、程式或驗收紀錄不作主體缺口判定。每項完成須附目前檔案、來源 SHA、適用測試與 UI／跨層結果；僅更新文件不勾選完成。

[改寫前完整清單及雜湊](teaching/history/evidence/2026-10-22-current-docs/README.md) 永久保留；以下保留原功能識別與待驗收條件，不重排教學 Day。

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

## 歷史雙庫適配項的判讀

舊 Room／WS DTO、Pet FSM／JSON／TOML、事件名稱、gRPC／ACK 與 SSE 的雙庫差異表[原文保留](teaching/history/evidence/2026-10-23-repository-roles/before/docs/FUTURE_WORK.md)。不再要求把 rust_ai_ring 移植到 copilot_use；必要的 ID／契約／傳輸需求留在原功能項，依 rust_ai_ring 的實際呼叫者與測試判定，不默改既有契約或增加無呼叫者骨架。

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
