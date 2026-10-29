# 既有每日排程提示詞

> 下列 Task ID、排程狀態、提示詞 SHA256 與儲存／讀回敘述是前次操作紀錄，保留作歷史證據；不是本次重新核驗結果。2026-10-23 僅同步 GitHub 文件，未建立、修改或觸發既有排程，亦未變更其模型、頻率或啟用狀態。本頁基底提示詞與文末現行補充共同閱讀；文件同步不代表雲端任務提示詞已同步或實際引擎已核實。

## 前次排程與基底提示詞紀錄（原文保留）

Task ID：`6acd967c15fc8191ade8407954c5b354`。每日06:00 Asia/Taipei，ACTIVE。只原地更新 main 分支和歷史路徑，未新增任務，未改模型或頻率；儲存後重新載入並全文核對一致。

提示詞 SHA256：`1bba61c134cb333c928fbbbafb34bf747b52681da8a26be8457773d741a38f60`。

【GitHub 唯一交付・每 Day 獨立 commit】沿用本既有三專案每日 TDD 教學排程，每日06:00 Asia/Taipei，保留啟用與實際模型設定，不新增任務。每次重讀 williechen/copilot_use 的 main 分支 docs/COMMON_LEARNING_RULES.md、docs/learning-state.json、docs/teaching/current/manifest.json、docs/teaching/daily/README.md、P9_TARGET、FUTURE_WORK、SYSTEM_ARCHITECTURE、UI_ALIGNMENT_REQUIREMENTS及相關Day；GitHub為唯一規則和文章正本。停止所有Notion教學發文、更新與讀回依賴；共同主頁及三案全部教學子頁已依使用者要求完整搬至 docs/teaching/history/evidence/notion-export/2026-10-22/，GitHub逐頁全文讀回後移至Notion可還原垃圾桶，不永久刪除其他資料。
依Chat→Pet→Analytics逐案交付，從各案最新next_day取N，不寫死或跳號。每新增一個專案Day立即獨立commit並推送至上述 main 分支的docs/teaching/daily/day-NN/{chat,pet,analytics}.md，不能累積多案或多天一起commit。文章須含四節：學習大綱與真實現況；真Red/完整可套用最小Green/Refactor及必要FIFO舊Day更正；套件版本；命令、預期/實際、CI/job/SHA與限制。先查重並核對最新兩庫SHA、真實呼叫鏈/UI/HTTP/IPC/DB/ACL與套件；來源變動重建未發草稿。
文章commit推送後，從精確commit SHA讀回全文與全部程式碼，保存blob SHA、SHA256及證據；然後重新讀最新branch/blob SHA，以條件commit原子同步該案next_day N→N+1、manifest及必要索引和有證據的makeup_queue變更，寫後讀回。狀態commit可另提交並引用文章commit。全部gate通過才換下一案；已N+1且證據一致不再增加；衝突先重讀，禁止覆寫。開始與推送前檢查其他執行。文章成功但狀態失敗只補狀態、不重發。單一pending去重，有界重試，同項連續三次失敗轉needs_manual_review，只暫停受影響專案。
Day25含以前GitHub正文/程式/證據不回寫；搬遷新增原文不算新Day。保留makeup_queue各案FIFO，每篇最多一項舊Day補教，有全文讀回證據才結案；修訂/補教/規則/搬遷/CI不增加Day。真實可操作基本流程優先，依最新缺口選最小切片，不預排題目；登入/登出/Cookie/Session共用實作、各系統儲存與政策獨立。Chat先註冊登入/房間流程與ACL；Pet先建立持久寵物/生活窗/重啟，單元件及每幀單一圖像、三個不同id原生GUI分別驗；Analytics先持久管理員帳密/改密碼/Session/角色來源授權和完整postgres-integration。SQLite與PostgreSQL分離；本次Analytics歷史學習庫採使用者刪除重建政策，不要求保留資料升級，未另授權不操作DB。
rust_ai_chat全庫唯讀，不修改main、不讀正式秘密、不使用付費AI。CI只揭露精確SHA與覆蓋，未執行測試/套用/部署/驗收如實標示，教學交付與產品驗收分開。主體學習各案15分鐘加共用15分鐘共60分鐘，其他估時另列，非模型時限。週六00:00全面比對為另一既有工作，勿新建；只有完整成功才更新last_full_comparison，每日不覆蓋。每次回報文章連結、commit、讀回、next_day及未完成事項；提示詞不作模型已切換證據。
現行文件與教學統一以 main 為正本；舊教學分支已合併刪除，不重建。歷史路徑依 docs/teaching/history/PATH_MAP.json 解析。

## 現行規則補充：既定架構與 Day27 核對（2026-10-23）

本節將 GitHub main 的 [共用規則](../COMMON_LEARNING_RULES.md) 同步至本文件；執行時仍須重讀最新正本及 [系統架構](../SYSTEM_ARCHITECTURE.md)、[UI 驗收要求](../UI_ALIGNMENT_REQUIREMENTS.md)。前述提示詞 SHA256 僅對應前次保存的提示詞，不是本文件加入補充後的雜湊；本次未寫入或讀回雲端排程。

1. 沿用既定架構修正教學偏差：**Leptos** 負責 UI／頁面元件與互動（Chat SSR／hydration／Router、Pet CSR + Tauri、Analytics 正式登入／修改密碼／管理後台）；**Axum** 負責 HTTP/API、WebSocket、Routing／Middleware、Cookie／Session、安全性檢查及服務端 ACL，保留既有 API／認證契約。不得以 Axum `Html(...)`／`Html(format!(...))` 完整字串頁面取代既有 Leptos UI，或冒稱新 UI Green。
2. **Application／Domain** 負責使用案例、流程與業務／領域規則，UI 不直接存取 DB 或偷渡授權；**SQLx 持久化 Adapter** 處理查詢、交易與 migration，Chat／Pet SQLite 與 Analytics PostgreSQL 隔離。**Tauri IPC** 配合 Pet Leptos CSR，僅處理原生橋接與視窗操作。
3. 每 Day 的 Red → 完整最小 Green → Refactor 核對真實 `Leptos UI → Axum HTTP/API 或 Tauri IPC → Application/Domain → SQLx Adapter` 當日最小可操作切片。只有 Axum HTML、API/helper 或假設性頁面成功，不能宣稱完成 Leptos UI；未執行測試、未套用與未驗收須各自明示。
4. **先逐案核對已發布 Day27，再從最新 next_day 接續**：讀回 Day27 正文、最新 `rust_ai_chat/main`、`copilot_use/main`、共用規則、manifest、索引與真實 UI／HTTP／IPC／DB／ACL 呼叫鏈，辨識 Chat／Analytics 的 Axum HTML 偏差與 Pet 現況。新文章 N 僅取當時 `docs/learning-state.json` 各案 `next_day`，不寫死 27、28 或任何後續編號，不回退、不跳號。確認文章／狀態尚無完整交付證據後，才依既有每案獨立 commit、全文讀回及條件同步流程推進 N→N+1；已交付者只補缺少狀態，不重發、不重教已完成功能。
5. 必要更正放在後續適用 Day 的 Red／完整 Green／Refactor 後，或依保留原 commit／blob／SHA256 的版本化勘誤流程處理；保留 Day25（含）以前封存、Day26／Day27 正文及原始歷史證據。核對、修訂、規則／文件同步不增加 Day，不自動結案 FIFO 補教。
6. 每日教學 CI 真實測試只引用 `williechen/rust_ai_chat` Actions 的精確 workflow／job／SHA／覆蓋；`copilot_use` 的 commit／blob／SHA256 只證明規則、文章與進度交付，不能用其 Actions 冒充教學 Green 或完整產品驗收。程式套用、CI、DB／GUI 與功能驗收分開。
