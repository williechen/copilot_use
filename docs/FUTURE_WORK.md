# 未來前進項目

目標依 [P9_TARGET.md](P9_TARGET.md) 及 copilot_use 真正程式，目錄／程式銜接依 rust_ai_chat 最新 main。以下是未來教學／銜接項目，並不表示已驗收的成品功能尚未實作。每天按最新唯讀 source 排除已套用項目，再選一個能完整實作及驗證的跨層切片；不自動新增超出產品目標的功能。

## 共同主線

- [ ] 採用 rust_ai_chat 新增／微調的目錄、型別、Port、介面與所屬測試，更新 copilot_use 對應及適配理由。
- [ ] 三專案各自產出完整可套用 Day；實際程式、重構、套件與回歸對應一致。
- [ ] 學習倉庫逐步達到產品能力，三專案與跨服務 E2E 共同驗收；獨立記錄學習套用與產品驗收。
- [ ] 每週六完整比對兩邊 source／介面／流程／契約／DB／UI／CI／測試／文件；依共同規則完成必要銜接。

## Chat

- [ ] 訊息／作者／房間模型、MessageRepository Port、SQLite schema／history／備份及持久化後通知。
- [ ] Session／ACL、HTTP／WS 房間隔離、presence／typing／reconnect／queue、durable realtime／Outbox／故障恢復。
- [ ] Leptos SSR／hydration／protected routes、真實 streaming／RAG／MCP／Agent／多 Persona。
- [ ] 同意／撤回、來源事件、OTel 與可審核政策採用／CAS 回退，跨服務 ID 對應與 E2E。

## Desktop Pet

- [ ] 純 Profile／Application Port／Schedule 與 SQLite／HTTP／OS runtime 的邊界。
- [ ] Tauri／Leptos IPC、五欄設定及可選覆寫、多寵物獨立 TOML／資產／備份與可逆封存。
- [ ] 使用者授權、型別化 preview／confirmation／實際 receipt、private Trash／restore、symlink／TOCTOU／journal／OS 安全。
- [ ] AI 外觀／素材與角色一致性、200×200 等比動畫、記憶／回想／複習／提醒、RAG／MCP／Agent。
- [ ] 真實事件至 Analytics、政策確認／回退、三平台 GUI／installer／簽章 updater 與資料保留。

## Product Analytics

- [ ] 教學銜接 gRPC／Protobuf 契約、來源認證／逐事件 ACK／Outbox 重試，以及 HTTP＋SSE 的 commit／rollback／重連；產品實作與驗證入口見 [ANALYTICS_TRANSPORT.md](ANALYTICS_TRANSPORT.md)。

- [ ] 四欄產品事件及來源權限、EventRepository Port、獨立 PostgreSQL migration／交易／持久冪等／衝突與故障恢復。
- [ ] 同服務 Dashboard／Ingest／Query、Taipei 趨勢／明細、可逆封存／同意撤回、備份還原。
- [ ] 聚合快照、可追溯 AI 洞察、人工審核、Chat／Pet 自主確認採用、收據／描述性評估／回退。
- [ ] 事件與 OTel 分離、跨服務 schema／ID／retry／trace、GUI／HTTP／DB／E2E／CI。

## 完成條件

逐項附規格 → 當前檔案／符號／SHA → 真正測試／CI → 學習套用狀態。未完成或未核實項目留在本清單；完成項目從活動清單移除，其證據獨立保存。新產品需求需更新 P9 目標、實作與重新驗收；不以舊 Release 或教學頁宣稱新需求已完成。
