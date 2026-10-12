# 共同學習規則

## 基準與權限

- 目錄與程式銜接以 williechen/rust_ai_chat 最新 main 的實際內容為主。唯讀核對差異、函式／型別、介面、資料流程與所屬測試，再更新 copilot_use 的對應模組；記錄兩邊 SHA 及適配理由。
- 產品目標、能力範圍及未來前進項目以 williechen/copilot_use 的 [P9_TARGET.md](P9_TARGET.md)、實際程式與 [FUTURE_WORK.md](FUTURE_WORK.md) 為主。學習倉庫的套用進度與產品的驗收狀態分開。
- 其他依本文件的共同學習規則執行。rust_ai_chat 的 source、README、CI、PR、branch、commit、tag 與資料庫嚴格唯讀。copilot_use 的每日教學實作在 teaching/rust-ai-chat-alignment；main 整合需匹配驗收，正式 v0.1.0 tag／附件保持固定。
- 兩個 repository 的統一 DB 目標：Chat／Pet 使用可攜式 SQLite；Analytics 使用獨立 PostgreSQL。範圍為本地／Docker／跨平台測試；公開部署不在目前範圍。

## 共同實作與教學規則

1. Domain/Application 不引入 DB、HTTP／gRPC、OS、UI Driver；Repository／Platform Adapter 隔離 SQL、placeholder、migration、交易及平台差異。維持真正使用的 Port，不建立空架構。
2. 每天 08:00 Asia/Taipei 讀取共同主頁、三專案主頁、最新 source／tests／Cargo／migration／CI，唯讀比較 rust_ai_chat 的微調；產品增量與必要重構一起前進。
3. Dry Run 逐檔讀取實際程式、公開／私有介面、斷言及缺口，提出可驗證的最小切片。Dry Run 不寫 Git／Notion，不要求完整程式；未取得 source／結果就標 Blocked，寫入 Gate 為 N/A。
4. 正式實作按 Red → 最小 Green → Refactor，保留已有功能與安全不變量；新功能與必要重構均附可追溯路徑、符號、SHA 與相應測試。DDD／事件驅動只用於真實業務邊界、解耦、重試或追蹤需求。
5. 文件階層為共同主頁 → Chat／Desktop Pet／Product Analytics 主頁 → 各專案新 Day。固定順序 Chat → Pet → Analytics，各專案獨立編號。專案主頁固定大綱、最終目標、後續缺口、CI/CD、開發與執行五節。
6. 正式 Day 固定學習大綱、實作內容、套件版本、重構內容、驗證項目五節。第 2／4 節按實際路徑提供完整最終程式碼、繁體中文註解、命令、預期與實際結果；同檔不重複，未變更檔案標 N/A。
7. 先完成 copilot_use 的實際銜接、必要回歸及提交，再寫對應 Day。記錄精確參考 SHA、產品 SHA、檔案／符號；寫前查重、寫後讀回五節、完整程式及父子關係。排程狀態或文檔不能代替實作。
8. 測試留在所屬 app／crate／私有模組，例如 apps/server/tests、apps/desktop_pet/tests 或 crates/<模組>/src/tests.rs。禁止根 tests workspace；私有測試不暴露危險 API，外移時保留斷言與覆蓋。
9. 教學、註解與自有 UI 使用繁體中文；第三方識別保留。版本 Gate 查官方 stable／MSRV、安全與 native／WASM 相容性，記錄宣告及 lock 版本、來源與維持／升級理由；未查證不猜。
10. 不改已套用的 migration；Domain/Application 不綁定 SQLx／PgPool／儲存型別。不同 DB Adapter 的原生 UUID／VARCHAR(40)、bind／decode／交易分別驗證。
11. FileOrganizer 非 root，只處理使用者授權 root／opaque ID；預覽與成功分開，破壞性動作須確認並重新驗證 symlink／TOCTOU／權限。產品事件與 OTel 分離，不傳憑證、聊天原文或完整私人路徑。
12. 分別記錄 Dry Run source 檢查、Day 教學準備、程式實作／驗收的 Pass／Blocked／N/A。依變更執行 fmt／Clippy／unit／DB／HTTP／gRPC／SSE／WS／WASM／GUI／E2E／CI；三專案及跨服務必要證據齊全才前進共同里程碑。產品成功不代表學習倉庫已套用。
13. AI provider 為 OpenAI，model 為 gpt-5.6-luna；需要真實呼叫時讀本機 ~/.bashrc 的 OPENAI_API_KEY，不寫 Git／Notion／報告。AI 邏輯未改動可沿用匹配來源的有效證據；付費、mock、未執行測試分開標示。

## 每週六完整比對

每週六 08:00 Asia/Taipei 執行完整比對，首次預定 2026-10-17。以 docs/learning-state.json 的到期日與來源為準；失敗不前移完成狀態，到期未完成由每日排程補做，避免同周期重複執行。

1. 唯讀取得兩個 repository 的最新 SHA／完整 tracked tree，核對正式 tag、教學 branch、適用 CI 與文件。
2. 比較全部 tracked 檔案：路徑、內容 hash／差異、各自新增／刪除／搬移；排除未追蹤的 build、私人 DB、key 與快取。
3. 逐模組核對職責、公開介面、Domain／Application／Adapter 依賴、重要資料流程、HTTP／gRPC／SSE／WS／IPC／事件契約、資料模型／migration、UI／建置／CI／測試覆蓋。文字不同不自動判為需要重構。
4. 目錄／程式微調依 rust_ai_chat；目標與前進項目依 copilot_use。分類為保留合理差異、必要銜接、架構違反、產品未完成項目及未核實項目；不覆蓋已驗證功能／安全邊界。
5. 完成必要的 copilot_use 實作與相應回歸；更新當前對照、共同規則／未來清單與來源，不把歷史摘要累積到主頁。無來源、測試或權限時列出實際阻礙。
6. 保存完整比對證據於分離的證據區；整輪完成後才更新來源與下一個週六到期日。僅完成、重要變更、失敗或需要使用者處理時通知；未變且不可行動時保持安靜。

## 文件位置

現行主文件只保存有效規則、操作方式、產品目標及未來工作。歷史敘事與舊 Day 位於分離封存區，不作當前待辦。已套用 migrations、有效 release 證據與 checksum 不刪除。
