# 共同學習規則

## 基準與權限

- 目錄與程式銜接以 williechen/rust_ai_chat 最新 main 的實際內容為主。唯讀核對差異、函式／型別、介面、資料流程與所屬測試，再更新 copilot_use 的對應模組；記錄兩邊 SHA 及適配理由。
- 產品目標、能力範圍及未來前進項目以 williechen/copilot_use 的 [P9_TARGET.md](P9_TARGET.md)、實際程式與 [FUTURE_WORK.md](FUTURE_WORK.md) 為主。學習倉庫的套用進度與產品的驗收狀態分開。
- 其他依本文件的共同學習規則執行。rust_ai_chat 的 source、README、CI、PR、branch、commit、tag 與資料庫嚴格唯讀。copilot_use 的每日教學實作在 teaching/rust-ai-chat-alignment；main 整合需匹配驗收，正式 v0.1.0 tag／附件保持固定。
- 兩個 repository 的統一 DB 目標：Chat／Pet 使用可攜式 SQLite；Analytics 使用獨立 PostgreSQL。範圍為本地／Docker／跨平台測試；公開部署不在目前範圍。

## 共同實作與教學規則

1. Domain/Application 不引入 DB、HTTP／gRPC、OS、UI Driver；Repository／Platform Adapter 隔離 SQL、placeholder、migration、交易及平台差異。維持真正使用的 Port，不建立空架構。
2. 每天 08:00 Asia/Taipei 讀取共同主頁、三專案主頁、兩個 repository 最新 SHA／完整 tracked tree、必要 source／tests／Cargo／migration／CI 與歷史 Day。每日先完成選題用的完整差異基線與逐項分類，再從確認缺失中選出 Chat、Pet、Analytics 各自最小、可完成、可驗證的切片；不得只依既定 Day 計畫或產品目標選題。週六另做深入架構、行為與跨服務驗證盤點，不能替代每日選題基線。
3. Dry Run 唯讀核對兩邊 source、必要 CI、DB／migration、介面、測試與歷史 Day，準備完整五節預覽。可修正文案、規則、路徑或教學預覽並讀回；不修改程式碼、測試或 migration，不執行程式修復。無法取得來源或差異分類未完成時，明列未核實／Blocked，不以 hash 不同直接推論功能缺失；未執行測試不得宣稱通過。
4. 正式實作按 Red → 最小 Green → Refactor，保留已有功能與安全不變量；新功能與必要重構均附可追溯路徑、符號、SHA 與相應測試。DDD／事件驅動只用於真實業務邊界、解耦、重試或追蹤需求。
5. 文件階層為共同主頁 → Chat／Desktop Pet／Product Analytics 主頁 → 各專案新 Day。固定順序 Chat → Pet → Analytics，各專案獨立編號。專案主頁固定大綱、最終目標、後續缺口、CI/CD、開發與執行五節。
6. 正式 Day 固定學習大綱、實作內容、套件版本、重構內容、驗證項目五節。第 2／4 節按實際路徑提供完整最終程式碼、繁體中文註解、命令、預期與實際結果；同檔不重複，未變更檔案標 N/A。
7. 每日教學與產品實作／驗收分開記錄。完成每日完整差異基線、確認最小選題及查重後，可依已核實來源發布五節教學，不以先完成 copilot_use 程式銜接、回歸及提交作為發布前置條件。教學程式未實測須標未執行，不能宣稱產品已完成。記錄精確參考 SHA、產品 SHA、檔案／符號；寫後讀回標題、五節、完整程式與父子關係。
8. 測試留在所屬 app／crate／私有模組，例如 apps/server/tests、apps/desktop_pet/tests 或 crates/<模組>/src/tests.rs。禁止根 tests workspace；私有測試不暴露危險 API，外移時保留斷言與覆蓋。
9. 教學、註解與自有 UI 使用繁體中文；第三方識別保留。版本 Gate 查官方 stable／MSRV、安全與 native／WASM 相容性，記錄宣告及 lock 版本、來源與維持／升級理由；未查證不猜。
10. 不改已套用的 migration；Domain/Application 不綁定 SQLx／PgPool／儲存型別。不同 DB Adapter 的原生 UUID／VARCHAR(40)、bind／decode／交易分別驗證。
11. FileOrganizer 非 root，只處理使用者授權 root／opaque ID；預覽與成功分開，破壞性動作須確認並重新驗證 symlink／TOCTOU／權限。產品事件與 OTel 分離，不傳憑證、聊天原文或完整私人路徑。
12. 分別記錄每日差異／選題、Dry Run、Day 教學與程式實作／驗收狀態。最新 rust_ai_chat 的必要 CI 全部 Success，且選題與查重完成，才可推進下一 Day；Failure 則優先安排失敗修復學習；Pending／Skipped／Cancelled／無紀錄不視為成功。現有 Day9 須先重新確認選題與內容，不能僅因 CI 成功跳過。程式實作依變更執行 fmt／Clippy／unit／DB／HTTP／gRPC／SSE／WS／WASM／GUI／E2E／CI；產品與跨服務里程碑仍須實際驗收。教學發布不表示學習倉庫已套用。
13. AI provider 為 OpenAI，model 為 gpt-5.6-luna；需要真實呼叫時讀本機 ~/.bashrc 的 OPENAI_API_KEY，不寫 Git／Notion／報告。AI 邏輯未改動可沿用匹配來源的有效證據；付費、mock、未執行測試分開標示。

## 每週六完整比對

每週六 08:00 Asia/Taipei 執行完整深入比對，首次預定 2026-10-17。在每日完整差異基線之外，盤點逐模組語意、架構邊界、資料流程、測試與跨服務證據，完成必要適配／回歸並記錄差異決策。以 docs/learning-state.json 的到期日與來源為準；失敗不前移完成狀態，到期未完成由每日排程補做，避免同周期重複執行。每日選題基線完成不等於本輪週六完整驗收完成。

1. 唯讀取得兩個 repository 的最新 SHA／完整 tracked tree，核對正式 tag、教學 branch、適用 CI 與文件。
2. 比較全部 tracked 檔案：路徑、內容 hash／差異、各自新增／刪除／搬移；排除未追蹤的 build、私人 DB、key 與快取。
3. 逐模組核對職責、公開介面、Domain／Application／Adapter 依賴、重要資料流程、HTTP／gRPC／SSE／WS／IPC／事件契約、資料模型／migration、UI／建置／CI／測試覆蓋。文字不同不自動判為需要重構。
4. 目錄／程式微調依 rust_ai_chat；目標與前進項目依 copilot_use。分類為保留合理差異、必要銜接、架構違反、產品未完成項目及未核實項目；不覆蓋已驗證功能／安全邊界。
5. 完成必要的 copilot_use 實作與相應回歸；更新當前對照、共同規則／未來清單與來源，不把歷史摘要累積到主頁。無來源、測試或權限時列出實際阻礙。
6. 保存完整比對證據於分離的證據區；整輪完成後才更新來源與下一個週六到期日。僅完成、重要變更、失敗或需要使用者處理時通知；未變且不可行動時保持安靜。

## 文件位置

現行主文件只保存有效規則、操作方式、產品目標及未來工作。歷史敘事與舊 Day 位於分離封存區，不作當前待辦。已套用 migrations、有效 release 證據與 checksum 不刪除。

## 每日選題與 Day9 狀態

每日比較全部 tracked 路徑及內容，逐項分類為已實作且已驗證、已實作未驗證、缺失需新增、需要重構、僅屬未來目標或不適用；涵蓋 API、Domain/Application/Adapter、DB／migration、UI／IPC／HTTP／WS／gRPC／SSE、測試、CI 與文件，記錄涵蓋率及無法取得項目。hash 比對只構成檔案差異證據，不能代替語意分類。從確認缺失中按依賴與風險選最小切片，明列來源、理由、涉及檔案與不包含範圍。

既有 Day9 與 DAY9_PLAN.md 僅為候選範圍，均待差異確認、重新選題與內容驗收；不新增重複 Day9，不直接推進 Day10。
