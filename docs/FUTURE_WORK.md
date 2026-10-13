# 未來前進項目

目標依 [P9_TARGET.md](P9_TARGET.md) 與 copilot_use 實際程式，目錄／內容銜接依 rust_ai_chat 最新 main。本清單分開記錄產品缺口、安全修正、參考適配與教學；未核實不標完成。每天從確認缺口與當日差異選三專案合計一小時的小切片，不預排 Day；週六工程與功能驗收另記。

既有開發項目是 P0～P12 階段功能完成條件的唯一追蹤來源；在原項目記錄所屬階段、可驗證功能條件、SHA／測試／CI／套用與驗收證據，未核實者保留未完成。跨階段只在原項目及原 Day 第 2 節後補加，不另開重複項目；完成後可退出活動清單，但原識別與證據須可追溯。P9 產品目標不等於 P9 學習階段。

## 產品缺口

- [ ] Desktop Pet 實際視窗漫遊：移動原生視窗，遵守監視器工作區邊界、負座標／不同尺寸與停滯處理。現有 sprite 動畫不算完成；需 owning module tests 與實際 GUI 功能驗證。
- [ ] FileOrganizer SHA256 重複檔案分組：安全唯讀掃描、尺寸候選、hash 前重新驗證檔案與 root、分組及 IPC／UI 接線；不得自動刪除或跳過確認。需同內容／不同內容／變動檔案／symlink 等回歸。

## 安全修正

- [ ] Chat SSE 完成前重新檢查目前 session／room ACL，再持久化回答；目前 openai::produce 未帶入授權身分，與 Agent 路徑不一致。屬 source 靜態發現，撤權情境尚未回歸；測試用可控 fixture，不要求真實 AI 呼叫。

## 參考適配

- [ ] 明列 Chat Room／作者／ID／category、HTTP 建房 DTO／status、WS 傳訊與通知協定映射；保留成品 ACL、durable realtime／Outbox。topic 不能代替 category。
- [ ] 明列 Pet Sleep／Wake／snapshot revision 與 Action／Speed 的差異、Profile JSON／TOML、ScanPreview／opaque preview 的契約；只作真正需要的適配，不新增未使用同名骨架。
- [ ] 明確決定 pet.cleanup.succeeded 與 file.cleanup.succeeded 的事件映射／版本，不默改既有白名單或把純狀態成功當真操作收據。
- [ ] Analytics 參考 501／記憶體 fake 與成品 PostgreSQL／來源認證／gRPC 的銜接說明；保留實際持久冪等、HTTP＋SSE 能力。

## 教學銜接

以下保留原有學習範圍，checkbox 表示學習／適配未完成，不表示對應成品功能全部未實作。每次依最新 source、所屬測試與 CI 排除已套用項目，再選最小切片。

### 共同主線

- [ ] 採用 rust_ai_chat 新增／微調的目錄、型別、Port、介面與所屬測試，更新 copilot_use 對應及適配理由。
- [ ] 三專案各自產出完整可套用 Day；四節格式、原教學與第 2 節末尾必要實作補充對應一致。
- [ ] 學習倉庫逐步達到產品能力，三專案與跨服務 E2E 共同驗收；獨立記錄學習套用與產品驗收。
- [ ] 每週六完整比對兩邊 source／介面／流程／契約／DB／UI／CI／測試／文件；依共同規則完成必要銜接。

### Chat

- [ ] 訊息／作者／房間模型、MessageRepository Port、SQLite schema／history／備份及持久化後通知。
- [ ] Session／ACL、HTTP／WS 房間隔離、presence／typing／reconnect／queue、durable realtime／Outbox／故障恢復。
- [ ] Leptos SSR／hydration／protected routes、真實 streaming／RAG／MCP／Agent／多 Persona。
- [ ] 同意／撤回、來源事件、OTel 與可審核政策採用／CAS 回退，跨服務 ID 對應與 E2E。

### Desktop Pet

- [ ] 純 Profile／Application Port／Schedule 與 SQLite／HTTP／OS runtime 的邊界。
- [ ] Tauri／Leptos IPC、五欄設定及可選覆寫、多寵物獨立 TOML／資產／備份與可逆封存。
- [ ] 使用者授權、型別化 preview／confirmation／實際 receipt、private Trash／restore、symlink／TOCTOU／journal／OS 安全。
- [ ] AI 外觀／素材與角色一致性、200×200 等比動畫、記憶／回想／複習／提醒、RAG／MCP／Agent。
- [ ] 真實事件至 Analytics、政策確認／回退、三平台 GUI／installer／簽章 updater 與資料保留。

### Product Analytics

- [ ] AI Token／費用與額度統計：按 Provider／Model／來源產品／功能／請求 ID／時間，記錄供應商實際回傳的 input／output／cached input／reasoning／total tokens、價格版本與估算／實際費用；日／月趨勢、預算警示及可取得的 rate limit／quota／remaining／reset，缺失額度顯示「未知」。失敗、重試、串流中斷、缺失 usage 要去重與補帳；Token 不等於配額或帳單；不儲存 API key、Prompt、回應本文。以既有紀錄／fixture／mock 驗證，禁止讀密鑰及付費 AI 呼叫。

- [ ] 教學銜接 gRPC／Protobuf 契約、來源認證／逐事件 ACK／Outbox 重試，以及 HTTP＋SSE 的 commit／rollback／重連；產品實作與驗證入口見 [ANALYTICS_TRANSPORT.md](ANALYTICS_TRANSPORT.md)。

- [ ] 四欄產品事件及來源權限、EventRepository Port、獨立 PostgreSQL migration／交易／持久冪等／衝突與故障恢復。
- [ ] 同服務 Dashboard／Ingest／Query、Taipei 趨勢／明細、可逆封存／同意撤回、備份還原。
- [ ] 聚合快照、可追溯 AI 洞察、人工審核、Chat／Pet 自主確認採用、收據／描述性評估／回退。
- [ ] 事件與 OTel 分離、跨服務 schema／ID／retry／trace、GUI／HTTP／DB／E2E／CI。

## 完成條件

逐項附規格 → 當前檔案／符號／SHA → 真正測試／CI → 學習套用狀態。產品功能驗收、每日教學發布、學習倉庫實際套用各自記錄，適用 CI 通過即可發布每日 Day，不等待週六提交。未完成或未核實項目留在對應分類；完成項目可從活動清單移除，但原項目識別、跨階段補加歷史及完成證據須永久可追溯。新產品需求需更新 P9 目標、實作與重新驗收；不以舊 Release 或教學頁宣稱新需求已完成。
