# 未來前進項目

> 實作進度、精確來源與驗證結果統一見[目前進度](teaching/current/STATUS.md)。本文件保留功能需求／完成條件或架構；歷次測試摘要已移入歷史紀錄，不作現行完成聲明。

需求正文依 [P9_TARGET.md](P9_TARGET.md)，實作缺口、目錄／內容、測試／CI 與功能驗收只依 rust_ai_chat 最新 main。本庫保存需求與教學，不另追蹤 copilot_use 產品程式；未核實不標完成。每天從確認缺口與當日差異選三專案合計一小時的小切片，不預排 Day；週六工程與功能驗收另記。

既有開發項目是 P0～P12 階段功能完成條件的唯一追蹤來源；在原項目記錄所屬階段、可驗證功能條件、SHA／測試／CI／套用與驗收證據，未核實者保留未完成。跨階段工作留原功能項目追蹤；舊 Day 更正僅於下一個 Day 的 Refactor 後獨立補加，不回填原 Day，不另開重複項目；完成後可退出活動清單，但原識別與證據須可追溯。P9 產品目標不等於 P9 學習階段。

## 清單使用方式

下列 checkbox 表示產品或教學仍待核對／完成的條件，不能據此推論所有底層程式都不存在。主體程式狀態見 [目前進度](teaching/current/STATUS.md) 的來源與證據範圍；舊 copilot_use token Dashboard、程式或驗收紀錄不作主體缺口判定。每項完成須附目前檔案、來源 SHA、適用測試與 UI／跨層結果；僅更新文件不勾選完成。

[改寫前完整清單及雜湊](teaching/history/evidence/2026-10-22-current-docs/README.md) 永久保留；以下保留原功能識別與待驗收條件，不重排教學 Day。

## 當前最高優先順序：先接通可操作的基本流程
這是既有 P0～P12 功能項目的實作順序與驗收 Gate；實際缺口及最新來源見[目前進度](teaching/current/STATUS.md)。每次依真實入口、資料庫與測試調整最小切片，不預排Day，也不以其他分支的舊試作當成已合併／驗收。
- [ ] **Chat：註冊／登入→Session→公開房間清單與進房授權→加入／建立聊天室。** 沿 rust_ai_chat 的 signup／signin／logout、mbr_user／Session 與房間授權實際來源核對；不引用舊庫 auth.rs／chat_users 介面。先完成欄位驗證與登入授權，不重建平行身分表。基本鏈未完成前不優先擴充歷史訊息或進階UI。
- [ ] **Pet：建立第一隻寵物→名稱／初始外觀→Tauri IPC→SQLite→生活窗→重啟載入。** 沿 rust_ai_chat 的 create_pet／pet_store／pet_profile 與 Tauri command 實際來源核對；補齊逐寵物身分／畫面／載入後再深化多寵物與漫遊。先核對舊DB歷史，以前向migration處理相容性，不清空DB。
- [ ] **Analytics：管理員初始化→持久化帳密登入→首次改密碼→Session／Dashboard授權。** 沿 rust_ai_chat bootstrap／password_login／Session 的最新來源，核對真正登入／改密碼／授權鏈及未完成部分；固定key登入或429回應不等於完成帳密身分流程。
每日仍以Chat→Pet→Analytics各一個可套用的Red→完整最小Green切片推進，未接通部分明示缺口。舊Day更正在本日新實作／Refactor後補充；每新增一個專案Day獨立GitHub commit推送並全文讀回後，才可條件同步next_day及manifest，清單整理本身不增加Day。P9功能條件、權限／資料安全、週六工程驗收與其他待辦保留。

## 既有實作的驗收邊界

[2026-10-16 舊庫 Linux 證據](weekly/2026-10-16/acceptance.json) 保留原來源，不作 rust_ai_chat 的完成證據。原功能是否已實作、是否有新增需求與跨平台缺口，須在主體最新 SHA 核對；不因來源角色調整重教已完成內容。

- [ ] Pet 漫遊後續需求：多寵物獨立生活窗、出入籠與互動、螢幕移除不立即搬回／手動找回；逐項按 [UI 需求](UI_ALIGNMENT_REQUIREMENTS.md) 核對目前來源、所屬測試及真實桌面結果。
- [ ] 漫遊與 SHA256 分組的 Windows／macOS 功能驗收，以及相對原 Linux 證據新增程式的回歸；沒有匹配證據前不標完成。

## 歷史雙庫適配項的判讀

舊 Room／WS DTO、Pet FSM／JSON／TOML、事件名稱、gRPC／ACK 與 SSE 的雙庫差異表[原文保留](teaching/history/evidence/2026-10-23-repository-roles/before/docs/FUTURE_WORK.md)。不再要求把 rust_ai_chat 移植到 copilot_use；必要的 ID／契約／傳輸需求留在原功能項，依 rust_ai_chat 的實際呼叫者與測試判定，不默改既有契約或增加無呼叫者骨架。

## 教學銜接

學習來源現況與驗證範圍見[目前進度](teaching/current/STATUS.md)。Chat／Pet 各自依 FIFO 補教尚未完成的部分；Analytics Day24 已結案。下一 Day 即時值只引用 [learning-state](learning-state.json)，不在此複製固定 Day。主體程式套用與功能驗收分開追蹤。

來源、進度與驗證結果統一見[目前進度](teaching/current/STATUS.md)。Day25（含）以前文章封存；以下沿用原功能項追蹤，不代表產品已整合或驗收。

以下保留原有學習範圍，checkbox 表示學習／適配未完成，不表示對應成品功能全部未實作。每次依最新 source、所屬測試與 CI 排除已套用項目，再選最小切片。歷史驗證保留原來源，新一輪只更新統一進度與對應原項目，不重建待辦。

### 共同主線

- [ ] 現有測試歸位重構：自Day15起分批搬移11個非空測試檔，Chat兩檔歸chat-application、Server一檔歸server、Pet五檔歸pet-domain、Analytics三檔歸stats-server。整合步驟及文章位置見[教學總文章](COMMON_LEARNING_RULES.md)的Refactor；參考庫由使用者套用，尚未搬移或執行搬移後驗證。

- [ ] 依 rust_ai_chat 最新目錄、型別、Port、介面與所屬測試更新教學對應及理由；不在 copilot_use 建立平行程式。
- [ ] 三專案各自產出完整可套用 Day；四節格式、原教學與第 2 節末尾必要實作補充對應一致。
- [ ] 學習倉庫逐步達到產品能力，三專案與跨服務 E2E 共同驗收；獨立記錄學習套用與產品驗收。
- [ ] 每週六完整核對 rust_ai_chat 的 source／介面／流程／契約／DB／UI／CI／測試，再核對 copilot_use 文件、教學與證據的對應；既有執行流程尚待調整見 [同步範圍](REPOSITORY_ROLE_TRANSITION.md)。

### Chat

原有 Session／ACL 與 UI 項目納入共同規則本輪確認的公開清單、房名登入導向、個人管理、註冊獨立完成頁、admin/admin 首次改密碼及 Email、帳號錯誤等待逐次加倍無上限、活動 Session／後台閒置設定及密碼變更撤銷。需求已確認，實作／驗收待逐項核對。

- [ ] 訊息／作者／房間模型、MessageRepository Port、SQLite schema／history／備份及持久化後通知。
- [ ] Session／ACL、HTTP／WS 房間隔離、presence／typing／reconnect／queue、durable realtime／Outbox／故障恢復。
- [ ] Leptos SSR／hydration／protected routes、真實 streaming／RAG／MCP／Agent／多 Persona。 首頁僅聊天室清單，點選後進房；建立／管理放個人管理頁。房間查詢須發出真實請求，並驗證載入、失敗恢復／重試與清單保留。房間頁採房間欄／訊息氣泡／多行輸入與真實操作回饋，窄螢幕可收合房間欄；個人與系統管理路由分開。驗收條件見 [畫面銜接需求](UI_ALIGNMENT_REQUIREMENTS.md)。
- [ ] 同意／撤回、來源事件、OTel 與可審核政策採用／CAS 回退，跨服務 ID 對應與 E2E。

### Desktop Pet

原有多寵物／漫遊／UI 項目納入獨立狀態與生活窗、多預設出籠、回籠停活動、右鍵坐下／控制窗選取、習性速度、打鬧吵醒、拖曳與多螢幕例外／手動找回。詳細依共同規則及 UI 需求；不提供固定速度設定，不以單 pet 共用狀態取代多寵物需求。未重新實作／驗收。

- [ ] 純 Profile／Application Port／Schedule 與 SQLite／HTTP／OS runtime 的邊界。
- [ ] Tauri／Leptos IPC、五欄設定及可選覆寫、多寵物獨立 TOML／資產／備份與可逆封存。學習庫畫面適配：200×200 寵物顯示與可縮放／捲動的控制視窗分離；六區為互動、設定、記憶、檔案、管理、更新，未接通能力明示，反覆開啟只聚焦既有視窗。漫遊只移動寵物生活窗，控制窗保留手動拖曳；需以完整原生GUI／IPC驗收。詳細條件見 [畫面銜接需求](UI_ALIGNMENT_REQUIREMENTS.md)。
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

逐項附規格 → 當前檔案／符號／SHA → 真正測試／CI → 學習套用狀態。產品功能驗收、每日教學發布、學習倉庫實際套用各自記錄，適用 CI 未全部 Success 仍可發布每日 Day，但須揭露失敗與待修問題，不宣稱驗證通過，不等待週六提交。未完成或未核實項目留在對應分類；完成項目可從活動清單移除，但原項目識別、跨階段補加歷史及完成證據須永久可追溯。新產品需求需更新 P9 目標、實作與重新驗收；不以舊 Release 或教學頁宣稱新需求已完成。
