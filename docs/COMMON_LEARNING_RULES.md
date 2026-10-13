# 共同學習規則

## 基準與權限

- 目錄與程式銜接以 williechen/rust_ai_chat 最新 main 的實際內容為主。唯讀核對差異、函式／型別、介面、資料流程與所屬測試，再更新 copilot_use 的對應模組；記錄兩邊 SHA 及適配理由。
- 產品目標、能力範圍及未來前進項目以 williechen/copilot_use 的 [P9_TARGET.md](P9_TARGET.md)、實際程式與 [FUTURE_WORK.md](FUTURE_WORK.md) 為主。學習倉庫的套用進度與產品的驗收狀態分開。
- 其他依本文件的共同學習規則執行。rust_ai_chat 的 source、README、CI、PR、branch、commit、tag 與資料庫嚴格唯讀。copilot_use 的每週實作與每日教學文件在 teaching/rust-ai-chat-alignment；main 整合需匹配驗收，正式 v0.1.0 tag／附件保持固定。
- 兩個 repository 的統一 DB 目標：Chat／Pet 使用可攜式 SQLite；Analytics 使用獨立 PostgreSQL。範圍為本地／Docker／跨平台測試；公開部署不在目前範圍。

## 共同實作與教學規則

1. Domain/Application 不引入 DB、HTTP／gRPC、OS、UI Driver；Repository／Platform Adapter 隔離 SQL、placeholder、migration、交易及平台差異。維持真正使用的 Port，不建立空架構。
2. 每天 06:00 Asia/Taipei 讀取共同主頁、三專案主頁、兩個 repository 最新 SHA、適用 CI 與歷史 Day。以最近一次週六完整比對的缺口清單為基線，對照上次每日差異檢查 SHA，讀取新增／修改／刪除的檔案、符號、呼叫端及所屬測試，產生每日教學。每日不重做全量比對，不以週驗收未完成阻擋已通過最新必要 CI 的每日教學；尚無週六基線時，沿用已核實差異並明列未核實範圍，不冒稱全量完成。
3. Dry Run 僅允許修改使用者提出問題所涉及的文案、規則、路徑敘述或教學文件，修改後讀回核對。程式碼、測試與 migration 保持唯讀，不執行程式修復；其他文件問題先列出，不自行擴大修改範圍。來源或測試證據不足須明列未核實／未執行。
4. 正式實作按 Red → 最小 Green → Refactor，保留已有功能與安全不變量；新功能與必要重構均附可追溯路徑、符號、SHA 與相應測試。DDD／事件驅動只用於真實業務邊界、解耦、重試或追蹤需求。
5. 文件階層為共同主頁 → Chat／Desktop Pet／Product Analytics 主頁 → 各專案新 Day。固定順序 Chat → Pet → Analytics，各專案獨立編號。專案主頁固定大綱、最終目標、後續缺口、CI/CD、開發與執行五節。
6. 正式 Day 固定學習大綱、實作內容、套件版本、重構內容、驗證項目五節。第 2／4 節按實際路徑提供完整最終程式碼、繁體中文註解、命令、預期與實際結果；同檔不重複，未變更檔案標 N/A。
7. 每日產生五節學習內容，標記「每日教學：CI 通過／週驗收待完成」，不宣稱已完成正式實作。每週六才執行 copilot_use 的必要實作、測試、回歸及提交；全部必要 Gate 通過後，將對應內容發布或更新為正式 Day，附實作 SHA 與實測證據。先完成實作、必要測試及提交再發布正式 Day，這是每週工作，不是每日發布學習內容的前置條件。兩種狀態使用同一 Day 頁更新，不另建重複頁；寫前查重、寫後讀回五節、完整程式、SHA 與父子關係。
8. 測試留在所屬 app／crate／私有模組，例如 apps/server/tests、apps/desktop_pet/tests 或 crates/<模組>/src/tests.rs。禁止根 tests workspace；私有測試不暴露危險 API，外移時保留斷言與覆蓋。
9. 教學、註解與自有 UI 使用繁體中文；第三方識別保留。版本 Gate 查官方 stable／MSRV、安全與 native／WASM 相容性，記錄宣告及 lock 版本、來源與維持／升級理由；未查證不猜。
10. 不改已套用的 migration；Domain/Application 不綁定 SQLx／PgPool／儲存型別。不同 DB Adapter 的原生 UUID／VARCHAR(40)、bind／decode／交易分別驗證。
11. FileOrganizer 非 root，只處理使用者授權 root／opaque ID；預覽與成功分開，破壞性動作須確認並重新驗證 symlink／TOCTOU／權限。產品事件與 OTel 分離，不傳憑證、聊天原文或完整私人路徑。
12. 每日推進條件為 rust_ai_chat 最新 Commit 的適用必要 CI 全部 Success；Failure／Pending／Skipped／Cancelled／無紀錄不視為通過。每日僅核對已有 CI 證據，不要求先完成額外本地實作、測試或提交；選題與查重是內容品質工作，不能變成每日全量比對或週驗收 Gate。每週正式 Day／共同里程碑須完成必要實作與 fmt／Clippy／unit／DB／HTTP／gRPC／SSE／WS／WASM／GUI／E2E／CI／提交等適用 Gate；未通過先修正，不發布正式 Day。每日教學、每週正式驗收與學習倉庫實際套用分別記錄。
13. AI provider 為 OpenAI，model 為 gpt-5.6-luna；需要真實呼叫時讀本機 ~/.bashrc 的 OPENAI_API_KEY，不寫 Git／Notion／報告。AI 邏輯未改動可沿用匹配來源的有效證據；付費、mock、未執行測試分開標示。

## 每週六完整比對

每週六 08:00 Asia/Taipei 執行完整比對，首次預定 2026-10-17。覆蓋兩個 repository 全部 tracked 路徑／內容、模組職責、公開介面、架構邊界、資料流程、DB／migration、UI／傳輸契約、測試、CI 與文件，確認缺失並選最小切片；完成必要 copilot_use 實作、適用測試、回歸與提交後，才發布正式 Day。失敗不前移週驗收完成狀態或到期日，留待每週工作修正；每日只提示待辦，不補做全量比對或程式修復，不以此阻擋 CI 通過的每日教學。

1. 唯讀取得兩個 repository 的最新 SHA／完整 tracked tree，核對正式 tag、教學 branch、適用 CI 與文件。
2. 比較全部 tracked 檔案：路徑、內容 hash／差異、各自新增／刪除／搬移；排除未追蹤的 build、私人 DB、key 與快取。
3. 逐模組核對職責、公開介面、Domain／Application／Adapter 依賴、重要資料流程、HTTP／gRPC／SSE／WS／IPC／事件契約、資料模型／migration、UI／建置／CI／測試覆蓋。文字不同不自動判為需要重構。
4. 目錄／程式微調依 rust_ai_chat；目標與前進項目依 copilot_use。分類為保留合理差異、必要銜接、架構違反、產品未完成項目及未核實項目；不覆蓋已驗證功能／安全邊界。
5. 完成必要的 copilot_use 實作與相應回歸；更新當前對照、共同規則／未來清單與來源，不把歷史摘要累積到主頁。無來源、測試或權限時列出實際阻礙。
6. 保存完整比對證據於分離的證據區；整輪完成後才更新來源與下一個週六到期日。僅完成、重要變更、失敗或需要使用者處理時通知；未變且不可行動時保持安靜。

## 文件位置

現行主文件只保存有效規則、操作方式、產品目標及未來工作。歷史敘事與舊 Day 位於分離封存區，不作當前待辦。已套用 migrations、有效 release 證據與 checksum 不刪除。

## 選題、學習時程與 Day 狀態

選題以每週完整比對確認的缺失為基礎，再按依賴與風險挑各專案最小切片；每日差異比對更新該清單並調整當日內容。分類為已實作且已驗證、已實作未驗證、缺失需新增、需要重構、僅屬未來目標或不適用；hash 差異不等於功能缺失。明列來源、理由、涉及檔案及不包含範圍，保留已驗證功能與安全邊界。

學習時程共 12 個階段，每階段 2 個月，總計 24 個月；每天合計 1 小時，三專案共用這一小時，不是各一小時。每日安排最小切片，功能新增／銜接與必要重構教學並行：第 2 節功能實作與第 4 節架構、介面、套件版本相容性重構須互相銜接；第 3 節比對現行版本與官方最新 stable 的 API、行為、安全、MSRV、native／WASM 相容差異，再判斷必要重構、可選優化或 N/A。不為重構而重構，不因有新版就強制升級。每日僅規劃及提供完整教學範例，不修改程式、執行額外測試或新增 CI Gate；可依當日依賴調整三專案時間，總時數不超過 60 分鐘。階段主題與驗收切片依完整比對的實際缺口及 P9 目標安排，不因時程到期宣稱驗收完成；尚未確認的起始日、當前階段與主題不自行猜定。

每日不依預先編排的 Day 計畫或固定功能清單執行；先讀取 rust_ai_chat 最新 main 與 copilot_use 教學分支的實際原始碼、Cargo、測試、呼叫端及相關 CI，排除已實作且有證據的功能，再依真實增量差異、未解缺口、相依性和風險動態選擇最小功能切片，與該切片必要的重構、套件版本差異並行教學。產品目標與 FUTURE_WORK 僅界定需求範圍，不是每天必做的既定課表；當日無必要重構則標 N/A 及理由。先核對既有 Day，避免重複內容或頁面；依最新必要 CI Success 推進每日教學編號，不等待週驗收，正式驗收編號分開。
