# 未來前進項目

> 實作進度、精確來源與驗證結果統一見[目前進度](teaching/current/STATUS.md)。本文件保留功能需求／完成條件或架構；歷次測試摘要已移入歷史紀錄，不作現行完成聲明。

目標依 [P9_TARGET.md](P9_TARGET.md) 與 copilot_use 實際程式，目錄／內容銜接依 rust_ai_chat 最新 main。本清單分開記錄產品缺口、安全修正、參考適配與教學；未核實不標完成。每天從確認缺口與當日差異選三專案合計一小時的小切片，不預排 Day；週六工程與功能驗收另記。

既有開發項目是 P0～P12 階段功能完成條件的唯一追蹤來源；在原項目記錄所屬階段、可驗證功能條件、SHA／測試／CI／套用與驗收證據，未核實者保留未完成。跨階段工作留原功能項目追蹤；舊 Day 更正僅於下一個 Day 的 Refactor 後獨立補加，不回填原 Day，不另開重複項目；完成後可退出活動清單，但原識別與證據須可追溯。P9 產品目標不等於 P9 學習階段。

## 當前最高優先順序：先接通可操作的基本流程
這是既有 P0～P12 功能項目的實作順序與驗收 Gate；實際缺口及最新來源見[目前進度](teaching/current/STATUS.md)。每次依真實入口、資料庫與測試調整最小切片，不預排Day，也不以其他分支的舊試作當成已合併／驗收。
- [ ] **Chat：註冊／登入→Session→公開房間清單與進房授權→加入／建立聊天室。** 沿用現有signup／mbr_user，先處理欄位驗證並接通登入授權，不重建平行身分表。基本鏈未完成前不優先擴充歷史訊息或進階UI。
- [ ] **Pet：建立第一隻寵物→名稱／初始外觀→Tauri IPC→SQLite→生活窗→重啟載入。** 沿用現有PetStore、create_pet與pet_profile；補齊逐寵物身分／畫面／載入後再深化多寵物與漫遊。先核對舊DB歷史，以前向migration處理相容性，不清空DB。
- [ ] **Analytics：管理員初始化→持久化帳密登入→首次改密碼→Session／Dashboard授權。** 沿用現有bootstrap，將帳號接到真正登入／改密碼／授權鏈；固定key登入或429回應不等於完成帳密身分流程。
每日仍以Chat→Pet→Analytics各一個可套用的Red→完整最小Green切片推進，未接通部分明示缺口。舊Day更正在本日新實作／Refactor後補充；完整Notion發布讀回才可條件更新next_day，清單整理本身不增加Day。P9功能條件、權限／資料安全、週六工程驗收與其他待辦保留。

## 產品缺口

- [ ] Desktop Pet 實際視窗漫遊：移動原生視窗，正常遵守監視器工作區邊界、負座標／不同尺寸；螢幕移除不立即搬回，提供手動找回，依現行 UI 需求。現有 sprite 動畫不算完成；需 owning module tests 與實際 GUI 功能驗證。
- [ ] FileOrganizer SHA256 重複檔案分組：安全唯讀掃描、尺寸候選、hash 前重新驗證檔案與 root、分組及 IPC／UI 接線；不得自動刪除或跳過確認。需同內容／不同內容／變動檔案／symlink 等回歸。

## 安全修正

- [ ] Chat SSE 完成前重新檢查目前 session／room ACL，再持久化回答；目前 openai::produce 未帶入授權身分，與 Agent 路徑不一致。屬 source 靜態發現，撤權情境尚未回歸；測試用可控 fixture，不要求真實 AI 呼叫。

## 參考適配

- [ ] 明列 Chat Room／作者／ID／category、HTTP 建房 DTO／status、WS 傳訊與通知協定映射；保留成品 ACL、durable realtime／Outbox。topic 不能代替 category。成品 ACL／durable realtime 對應仍待銜接。
- [ ] 明列 Pet Sleep／Wake／snapshot revision 與 Action／Speed 的差異、Profile JSON／TOML、ScanPreview／opaque preview 的契約；只作真正需要的適配，不新增未使用同名骨架。設定／儲存／呈現呼叫端與契約測試須依最新兩庫來源核對。
- [ ] 明確決定 pet.cleanup.succeeded 與 file.cleanup.succeeded 的事件映射／版本，不默改既有白名單或把純狀態成功當真操作收據。
- [ ] 銜接學習庫 HTTP 來源認證／PostgreSQL 與成品 gRPC／逐事件 ACK／SSE／權限模型；驗證必須包含真資料庫與正式憑證契約，不以記憶體去重或歷史CI代替。

## 教學銜接

2026-10-22最新學習來源main a06c038驗證與缺口見[驗證報告](teaching/evidence/2026-10-22-main-a06c038/REPORT.md)。三案Day25封存、next_day=26；Chat共用登入／登出／Session、Pet單圖像動畫與此SHA三窗GUI、Analytics Day24 fixture及各歷史舊庫升級依原項目繼續，未因本輪驗證標完成。這是rust_ai_chat學習來源觀察，不代表copilot_use產品已套用或驗收。

來源、進度與驗證結果統一見[目前進度](teaching/current/STATUS.md)。Day22（含）以前文章封存；以下沿用原功能項追蹤，不代表產品已整合或驗收。

以下保留原有學習範圍，checkbox 表示學習／適配未完成，不表示對應成品功能全部未實作。每次依最新 source、所屬測試與 CI 排除已套用項目，再選最小切片。歷史驗證保留原來源，新一輪只更新統一進度與對應原項目，不重建待辦。

### 共同主線

- [ ] 採用 rust_ai_chat 新增／微調的目錄、型別、Port、介面與所屬測試，更新 copilot_use 對應及適配理由。
- [ ] 三專案各自產出完整可套用 Day；四節格式、原教學與第 2 節末尾必要實作補充對應一致。
- [ ] 學習倉庫逐步達到產品能力，三專案與跨服務 E2E 共同驗收；獨立記錄學習套用與產品驗收。
- [ ] 每週六完整比對兩邊 source／介面／流程／契約／DB／UI／CI／測試／文件；依共同規則完成必要銜接。

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
