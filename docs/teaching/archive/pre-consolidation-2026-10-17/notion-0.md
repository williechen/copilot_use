Here is the result of "fetch" for the Page with URL https://app.notion.com/p/3f2e51d4509381ddb8a8ce7cb4cd3e60 as of 2026-10-16T16:53:54.198Z:
<page url="https://app.notion.com/p/3f2e51d4509381ddb8a8ce7cb4cd3e60" icon="🦀">
<ancestor-path></ancestor-path>
<properties>
{"title":"Rust AI 三專案｜共同學習規則與前進目標"}
</properties>
<iconMetadata>{"type":"emoji","emoji":"🦀"}</iconMetadata>
<content>
## 基準與權限
- **每日教學唯一實作起點為 rust_ai_chat 最新 main 已存在的架構、原始碼、Port、Adapter、呼叫端及測試。** copilot_use 已實作功能只提供最終功能行為及驗收方向的比對，不是要照搬的另一套架構或程式模板。先核對 rust_ai_chat 已有能力，再與 copilot_use 比對尚缺的功能；僅在 rust_ai_chat 原有程式上作最小必要擴充與重構，優先重用現有 MessageRepository 等型別；沒有來源證據不得預先創造重複的 DurableMessageRepository、平行 Port、服務或空示範架構。記錄兩庫 SHA、實際函式、呼叫端、測試、合理差異及適配理由。 核對與同步判定依「文章要求 → rust_ai_chat 實作 → copilot_use 對應實作 → 驗證結果」逐項追溯：記錄文章頁面／Day、兩庫精確 SHA、實作路徑與符號、實際呼叫端、對應測試及匹配來源的驗證結果；介面不同須說明合理差異與適配理由。缺少證據標示未核實，只有定義／單元測試而未接通流程標示部分完成，未執行標示未執行；不得以讀過文章、更新 SHA、同名符號或歷史 CI 判定同步完成。
- 產品目標、能力範圍及未來前進項目以 copilot_use 的 [P9_TARGET.md](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/P9_TARGET.md)、實際程式與 [FUTURE_WORK.md](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/FUTURE_WORK.md) 為主。學習倉庫的套用進度與產品的驗收狀態分開。
- 其他依本文件的共同學習規則執行。rust_ai_chat 嚴格唯讀。copilot_use 的每週實作與每日教學文件在 teaching/rust-ai-chat-alignment；main 整合需匹配驗收，正式 v0.1.0 tag／附件保持固定。
- 兩個 repository 的統一 DB 目標：Chat／Pet 使用可攜式 SQLite；Analytics 使用獨立 PostgreSQL。實際套用以 source／測試為準；rust_ai_chat 保持唯讀。範圍為本地／Docker／跨平台測試；公開部署不在目前範圍。
## 共同實作與教學規則
1. Domain/Application 不引入 DB、HTTP／gRPC、OS、UI Driver；Repository／Platform Adapter 隔離 SQL、placeholder、migration、交易及平台差異。維持真正使用的 Port，不建立空架構。
2. 每天 06:00 Asia/Taipei 讀取共同主頁、三專案主頁、兩個 repository 最新 SHA、適用 CI 與歷史 Day。以最近一次週六完整比對的缺口清單為基線，對照上次每日差異檢查 SHA，讀取新增／修改／刪除的檔案、符號、呼叫端及所屬測試，產生每日教學。每日不重做全量比對，不以週驗收未完成阻擋已通過最新必要 CI 的每日教學；尚無週六基線時，沿用已核實差異並明列未核實範圍，不冒稱全量完成。
3. Dry Run 僅允許修改使用者提出問題所涉及的文案、規則、路徑敘述或教學文件，修改後讀回核對。程式碼、測試與 migration 保持唯讀，不執行程式修復；其他文件問題先列出，不自行擴大修改範圍。來源或測試證據不足須明列未核實／未執行。發現教學執行偏差時，先條列「既有規定／實際行為／衝突原因／修正結果」並優先修正教學內容；規則正確時不新增同義規則，僅在文件確實互相衝突或必要規範缺失時做最小修改。
4. 每日教學與正式實作均以當前階段尚未完成的真實功能為主，保留差異驅動與最小切片，採 TDD Red → 最小 Green → Refactor → 驗證；必要測試與重構隨功能提供，重大缺陷、阻塞或必要安全驗證在原學習實作內容後補加，不以純測試取代功能推進。正式實作按 Red → 最小 Green → Refactor，保留已有功能與安全不變量；新功能與必要重構均附可追溯路徑、符號、SHA 與相應測試。DDD／事件驅動只用於真實業務邊界、解耦、重試或追蹤需求。
5. 文件階層為共同主頁 → Chat／Desktop Pet／Product Analytics 主頁 → 各專案新 Day。固定順序 Chat → Pet → Analytics，各專案獨立編號。專案主頁固定大綱、最終目標、後續缺口、CI/CD、開發與執行五節。
6. 每日教學與正式 Day 統一四節：1. 學習大綱、2. 實作內容、3. 套件版本、4. 驗證項目。不設獨立重構章；完整程式、繁體中文註解及必要實作補充集中在第 2 節。原教學文字、程式與來源保留，必要實作只補在第 2 節末尾，同檔不重複，未變檔 N/A。第 4 節記錄教學相關命令、預期、實際結果與未執行項目。
7. 每日適用 CI 通過即可產生四節教學；教學與週六工程工作分開。既有教學保留，追加的必要實作放第 2 節末尾，不以新內容覆蓋原教學。週六完整比對、程式整合、測試、提交及功能驗收另存工程證據，不加入教學內容；必要功能 Gate 通過後才記錄正式驗收狀態。寫前查重、寫後讀回四節、完整程式、來源 SHA 與父子關係。
8. 測試留在所屬 app／crate／私有模組；禁止在 workspace 根目錄建立共用 tests；私有測試不暴露危險 API，外移時保留斷言與覆蓋。 目前教學已到 Day 14，既有根 tests 的整合列為 Day 15 起可分批進行的 Refactor；每次隨當日相關功能搬移對應測試、保留斷言並核對測試探索及執行結果，不回填 Day 13、不計新功能。過渡期間保留尚未搬完的測試套件與 CI，全部歸位後才移除 workspace-tests、根 tests 與舊 CI 引用；由使用者套用 rust_ai_chat。
9. 教學、註解與自有 UI 使用繁體中文；第三方識別保留。版本 Gate 查官方 stable／MSRV、安全與 native／WASM 相容性，記錄宣告及 lock 版本、來源與維持／升級理由；未查證不猜。
10. 資料庫 Adapter 與 Migration 規範：已套用的 migration 不得修改；不同資料庫 Adapter 須分別驗證。SQLite 與 PostgreSQL 的差異由各自 Adapter 處理，不得滲入 Domain/Application。
11. FileOrganizer 非 root，只處理使用者授權 root／opaque ID；預覽與成功分開，破壞性動作須確認並重新驗證 symlink／TOCTOU／權限。產品事件與 OTel 分離，不傳憑證、聊天原文或完整私人路徑。
12. 每日推進條件為 rust_ai_chat 最新 Commit 的適用必要 CI 全部 Success；Failure／Pending／Skipped／Cancelled／無紀錄不視為通過。每日不要求額外實作、全量比對或週驗收。週六必要功能實作、功能測試與提交未通過先修正，通過才記錄正式驗收；AI 項目不屬於週六 Gate。每日教學、週六工程／正式驗收、學習倉庫套用分別記錄，週六工程紀錄不進教學。
13. AI provider 為 OpenAI；每日教學與週六工程均禁止讀取 API key／密鑰及執行付費 AI 呼叫，不得將密鑰寫入 Git／Notion／報告。AI 邏輯未改動可沿用匹配來源的有效證據；付費、mock、未執行測試分開標示。
## 每週六完整比對
每週六 08:00 Asia/Taipei 完整比對，首次預定 2026-10-17。核對全部 tracked 路徑／內容、架構、介面、資料流程、DB／migration、UI／傳輸、測試、CI 與文件，完成必要整合／功能測試／提交，工程證據獨立保存，不加入教學。功能 Gate 未通過先修正；成功才前移週驗收狀態與到期日，每日不補做週工程。
週六整合只驗證功能：依變更執行 source／fmt／Clippy／unit／DB／HTTP／gRPC／SSE／WS／WASM／GUI／E2E／CI 等適用功能檢查及提交。功能 Gate 全部通過即記為功能驗收通過；不執行或要求 AI 對話／串流、生圖、RAG、MCP 等 AI 項目驗證，不讀取 API key、不重複付費呼叫，也不以 AI 項目未驗證阻擋功能通過。AI 只保留既有證據或標示本週未驗證，不宣稱本次功能通過代表 AI 重測通過。
1. 唯讀取得兩個 repository 的最新 SHA／完整 tracked tree，核對正式 tag、教學 branch、適用 CI 與文件。
2. 比較全部 tracked 檔案：路徑、內容 hash／差異、各自新增／刪除／搬移；排除未追蹤的 build、私人 DB、key 與快取。
3. 逐模組核對職責、公開介面、Domain／Application／Adapter 依賴、重要資料流程、HTTP／WS／IPC／事件契約、資料模型／migration、UI／建置／CI／測試覆蓋。文字不同不自動判為需要重構。
4. 目錄／程式微調依 rust_ai_chat；目標與前進項目依 copilot_use。分類為保留合理差異、必要銜接、架構違反、產品未完成項目及未核實項目；不覆蓋已驗證功能／安全邊界。
5. 完成必要的 copilot_use 實作與相應回歸；更新當前對照、共同規則／未來清單與來源，不把歷史摘要累積到主頁。無來源、測試或權限時列出實際阻礙。
6. 保存完整比對證據於分離的證據區；整輪完成後才更新來源與下一個週六到期日。僅完成、重要變更、失敗或需要使用者處理時通知；未變且不可行動時保持安靜。
## 文件位置
現行主文件只保存有效規則、操作方式、產品目標及未來工作。歷史敘事與舊 Day 位於分離封存區，不作當前待辦。已套用 migrations、有效 release 證據與 checksum 不刪除。
## 目前系統架構
[元件／框架／職責／DB／資料流程表格](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/SYSTEM_ARCHITECTURE.md)
## 專案入口
<page url="https://app.notion.com/p/3f3e51d45093807da638c336e857543a">聊天室</page>
<page url="https://app.notion.com/p/3f3e51d45093817eafaee0cfc9ea4cc8">統計儀錶板｜聊天室 × 桌面寵物</page>
<page url="https://app.notion.com/p/3f3e51d450938067923ae5a424d30950">桌面寵物</page>
<page url="https://app.notion.com/p/3f7e51d4509381faa894d3660c3252f7">文件與教學封存（非現行規則）</page>
## 最新來源驗證最終判定（2026-10-16）
**目前狀態：test-workspace 的 Analytics 測試環境缺口已修正，最新 GitHub CI 全部通過。** `rust_ai_chat/main@d05438950b96e03c9c6c6f8f2c7e3573cc191a25` 的 6／6 workflows、18／18 checks 均為 Success；[Test All／test-workspace 成功紀錄](https://github.com/williechen/rust_ai_chat/actions/runs/38765474606/job/116527415647)。最新 test.yml 已提供獨立 PostgreSQL、`ANALYTICS_DATABASE_URL` 與兩個合成來源 token。「缺少 Analytics 資料庫與 token」是舊提交 `a57e532` 的失敗原因，已不再是當前阻礙。
本地正式驗證仍精確對應 `a57e532c125f5f18d7fd2a250401efde0c9d2612`：107 項 Rust、3 項 Node、fmt、嚴格 Clippy、SSR／兩個 WASM、Linux 原生／Chat Leptos／Pet 前端 Release，以及 14 項實際 HTTP／WS／DB 檢查通過。已唯讀比對 `a57e532..d054389`，只有 `test.yml` 與 `persistence.yml` 改動，應用程式／測試／依賴鎖均未變；因此保留原本地證據，不冒稱已對新 SHA 重跑全部本地驗證。[精確來源、CI 與可攜式證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/source-review/rust-ai-chat-validation.json)。
Day 12 已由使用者套用；原撰寫基底 `5a0a8ce` 與全部原教學程式區塊保留。Analytics HTTP 已接 PostgreSQL，Typing／來源認證／持久冪等／併發／重啟／DB 故障測試通過。Pet 實際 GUI 移動、跨平台安裝包、gRPC／SSE 銜接及正式憑證安全審查仍未由本次驗證完成；不將 CI 全綠當作全部 Day／P9／週六或學習階段完成。未讀取私人 .env／AI key，未執行付費 AI，未寫入 rust_ai_chat。
## 選題、學習時程與 Day 狀態
不預排 Day 或固定每日題目。從每週完整比對已確認的缺失與當日差異中，按依賴與風險挑各專案最小切片；每日教學從 rust_ai_chat 最新 main 已存在的架構、型別、Port、函式、呼叫端與測試延伸，copilot_use 提供功能行為與目標方向，不照搬平行架構。rust_ai_chat repository 仍嚴格唯讀；教學中的程式是供使用者套用的內容，不代表已寫入學習倉庫。先查既有 Day 避免重複，CI 通過後推進每日教學編號；在既有 Day 區分「教學已發布、程式尚待套用、功能已實作、功能正式驗收通過」，CI 不等於功能完成。P9 產品目標不等於 P9 學習階段。
學習路線為 P0–P12，共 13 階段；每階段 2 個月，總計 26 個月。起始日期 2026-10-12，P0→P1 預定切點 2026-12-12，P12 預定 2028-10-12 開始、總時程預定 2028-12-12 結束。每日 06:00 Asia/Taipei 產生教學。學習時數只計第 2 節的主體實作（Red 與最小 Green）；Refactor／必要補加不計入學習時數，第 1、3、4 節亦不計入。三專案每日主體實作學習時數合計最多 60 分鐘。階段切點通知一次；「進入下一階段」、「學習實際完成」、「功能驗收通過」分開記錄。到達階段最晚切點必須結束當前階段並切換下一階段，不可延後；功能未完成時標記「階段已結束、學習未完成」，在原開發項目及原 Day 第 2 節後補加，跨階段持續開發，完成後回填原階段完成日期及證據；正式驗收獨立。各階段完成條件對應 `FUTURE_WORK.md` 既有項目，不另建重複清單。提前完成須有實際學習證據，CI 通過不代表學習階段已完成。
</content>
</page>