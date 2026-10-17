# Rust AI 三專案：教學總綱與實作對照

> 唯一現行規則來源：[Notion｜Rust AI 三專案共同學習規則與前進目標](https://app.notion.com/p/3f2e51d4509381ddb8a8ce7cb4cd3e60)。本文件是 GitHub 同步鏡像；有差異時以 Notion 最新總頁為準。

本篇集中教學方法、文章格式及 Chat／Desktop Pet／Analytics 大綱。其他教學入口只連到本篇及既有 Day，不再重述方法。原文章與程式保留於 Day／歷史封存，統整不代表完成程式同步或功能驗收。
## 目前進度與兩庫分工
三專案目前已發布到 Day 15，下一篇均為 Day 16；各專案各有一套連續 Day 編號。套用、流程接通與驗證分別記錄，不能從 Day 編號推算。舊「正式 Day 9」編號已移入歷史。
教學從 rust_ai_chat 最新 main 的既有架構、程式、呼叫端與測試延伸，由使用者套用；助手對該庫的 source、README、CI、branch、PR、commit、tag及資料庫保持唯讀。copilot_use 依產品目標整合必要功能與適配，保留已有能力、安全邊界及合理差異，不按教學 Day 等待產品開發，也不為檔名一致創造平行 Port 或空架構。產品目標 P9 與學習階段 P9 分開。
每個功能用同一條對照呈現：**文章要求 → rust_ai_chat 實作 → copilot_use 對應實作 → 驗證結果**。附文章／Day、兩庫精確 SHA、路徑／符號、實際呼叫端、對應測試及匹配來源的結果。介面不同說明適配理由；缺證據列未核實、未接通列部分完成、未執行明示，不能以更新 SHA、同名符號、文章發布或歷史 CI 宣稱同步完成。
## 教學方法與學習時程
先建立粗略但可操作的基本功能，再接通前台、後台及資料流程，逐步加入細節與例外。每天可延續同一基本流程，交代已完成哪一段、可驗證成果與下一段銜接；允許跨日，不把中間的 API、欄位、狀態碼或單純補測另計完整新功能。阻擋基本流程或資料安全的問題當步處理，其餘深化留在原功能下。
每天06:00 Asia/Taipei，按 Chat→Pet→Analytics 產出；三專案主體學習合計最多60分鐘，只計第2節 Red與最小Green，Refactor／必要補加及第1、3、4節不計入主體時數。按已確認功能缺口及當日程式差異選題，不固定預排未來各Day題目；測試歸位自Day15起可隨相關功能分批做。
學習為P0–P12共13階段、每階段2個月、合計26個月；起始2026-10-12，P0下一切點2026-12-12，預定2028-12-12結束。階段到期照時切換、通知一次；未完成項目留在原功能追蹤，階段切換不等於學習或功能驗收完成。提前完成以實際學習證據判定。
## 每篇 Day 的四節格式
### 1. 學習大綱
說明本次使用者能多完成哪一步、前置能力、目前程式與文章差異、對應功能及來源。先排除已套用內容，接續已有流程，不重教已完成切片。
### 2. 實作內容
按 **Red → 最小 Green → Refactor → 舊 Day 更正補充** 排列。提供本次必要修改的完整程式、實際檔案位置及繁體中文註解，同檔不重複，未變檔N/A。實作要說明入口、呼叫鏈、資料處理及畫面如何接通。
Refactor 整理當步實作及現有測試；例如將根 tests 的現有測試逐批歸回所屬 app／crate／私有模組，同批維護 Cargo、lock及CI，保留斷言與覆蓋。未搬完的套件／target／CI保留運作，全部歸位並驗證後才移除根tests與workspace-tests；不為測試公開危險私有API。
未實作 Day 可在既有原頁重整，更新修訂版與來源、同步同一索引及鏡像，不另建相同專案／Day。已實作內容即封存，保留原文、程式及證據；修改方案只在下一 Day 的 Refactor 後「舊 Day 更正補充」提出。部分已實作時保留已實作段落，只重整尚未實作部分；實作狀態不明時先標未核實，不當作未實作覆寫。文章修訂不新增 Day，也不推進 next_day。
### 3. 套件版本
記錄本次涉及的Cargo宣告、lock、官方stable／MSRV／安全及native／WASM相容性查核，只使用最新的穩定版或長期維護版，若版本涉及功能改寫，需在實作內容後面補增加需變更的內容。
### 4. 驗證項目
列與本次功能對應的命令、預期、實際結果及未執行範圍；從真實入口驗證到資料與畫面結果。handler單測、DB、router/socket、GUI及跨服務流程的證據分別交代。文章發布、使用者套用、流程接通與功能驗收各自依證據判定。
## 三專案功能大綱
### Chat：房間與聊天流程
基本主線：註冊／登入 → 房間列表選房 → 個人管理建立／管理自己的房間 → 進房聊天 → 重進讀回歷史 → 操作紀錄送達Analytics並可查詢，功能進展逐步補充，不列為一次完成要求。
後續能力包括Session／ACL、SQLite歷史與備份、WS presence／typing／reconnect／queue、Outbox、Leptos SSR／hydration、Streaming／RAG／MCP／Agent、多Persona、同意／撤回及政策回退。
### Desktop Pet：啟動、互動及安全操作
基本主線：啟動既有寵物 → 控制介面即時調整 → 設定／記憶持久保存 → 授權目錄 → 掃描及SHA256預覽 → 明確確認 → 真實操作結果／還原 → Analytics紀錄。
介面使用 Leptos，寵物顯示與控制視窗分離：200×200 寵物視窗保留動畫／拖曳／右鍵入口，控制視窗可縮放與捲動，分互動、設定、記憶、檔案、管理、更新；只接真實可用 IPC，未接通能力明示。控制面板不因開啟或切換而新增寵物。
逐步完成五欄設定（名字／種類／喜好／個性／興趣）、選用外觀覆寫、每隻寵物獨立資料與素材、200×200等比動畫、多寵物、手動拖移與可選的自動漫遊、記憶／回想／複習／提醒、RAG／MCP／Agent、備份／還原及installer／簽章updater。安裝後的更新採啟動時檢查或手動檢查正式GitHub Release，也不要求重新安裝。
安全檔案流程保持授權root／opaque ID、non-root、確認、symlink／TOCTOU／權限重驗、journal／receipt／Trash／restore；預覽或Domain成功狀態不能冒充filesystem成功憑證。
### Analytics：真實事件、查詢與管理
服務間來源認證採「首次核准配對 → 短效憑證 → 來源端自動交互續期／輪替 → 撤銷／重新配對」；Chat／Pet 與 Analytics 互驗對方身分，不以 source_app 自我宣告或固定 token 無限換證。Leptos 管理畫面負責配對核准、期限／輪替政策、狀態與稽核；後端及來源端執行自動換證。此為已確認需求、未實作／未驗證，完整方案於後續 Day 提出，不回改 Day15。
基本主線：接收Chat／Pet真實動作 → 來源認證 → 去重並持久提交 → Dashboard查詢／篩選 → 趨勢與明細 → 管理紀錄 → 封存／還原／備份。
Dashboard 畫面使用 Leptos 元件與 Rust 狀態管理，沿用 HTTP JSON＋SSE／來源授權／PostgreSQL Adapter；不另維護獨立 HTML／JavaScript 互動頁，不把服務密鑰嵌入前台。既有 Day 的教法更正依封存規則在下一 Day 提出。
產品上報目標採gRPC／Protobuf及逐事件ACK，Dashboard保留HTTP JSON＋SSE；學習庫已有HTTP相容入口的演進事實保留。獨立PostgreSQL、來源／事件ID冪等與衝突、commit後通知、重試／重連／故障恢復分別驗證。Chat／Pet各自SQLite，不共享業務DB。
後續包括可追溯AI洞察、人工審核、Chat／Pet自主授權採用／回退，以及Provider／Model／來源／功能／請求／時間的Token使用量、成本及實際可取得額度。額度未知明示未知，不由Token推算配額；用既有紀錄或合成資料驗證。
## 當前功能狀態與證據入口
核對日期：2026-10-17（Asia/Taipei）。參考庫來源 `3e58e650bd63bcd21ba58e7aabf751542401ef80`；成品對應程式核對來源 `9f632c76ee432d77c1d20f46da9485d982d49c9e`。以下為程式／呼叫端／測試位置的靜態核對結果，本輪未重新執行測試或遠端CI，不能解讀為完整同步或功能驗收通過。
- **Chat：部分接通。** Day14房間讀取API與測試已存在，尚未被學習庫前台呼叫；成品已有具權限的房間清單流程，介面差異仍需對照。證據：[參考庫房間入口與測試](https://github.com/williechen/rust_ai_chat/blob/3e58e650bd63bcd21ba58e7aabf751542401ef80/apps/server/src/http.rs)、[工程對照紀錄](https://app.notion.com/p/3f7e51d4509381faa894d3660c3252f7)。
- **Desktop Pet：掃描入口已接通，整體流程未完整驗證。** Day14主動掃描到摘要及測試已存在；成品也有掃描與SHA256分組，資料契約不同。證據：[參考庫前台入口](https://github.com/williechen/rust_ai_chat/blob/3e58e650bd63bcd21ba58e7aabf751542401ef80/apps/desktop_pet/frontend/src/main.rs)、[工程對照紀錄](https://app.notion.com/p/3f7e51d4509381faa894d3660c3252f7)。
- **Analytics：查詢API已實作，Dashboard流程未接通。** Day14來源／類型查詢及測試已存在；成品有對應聚合／篩選能力，新增DB單測的實際執行證據仍需核對。證據：[參考庫查詢入口與測試](https://github.com/williechen/rust_ai_chat/blob/3e58e650bd63bcd21ba58e7aabf751542401ef80/apps/stats_server/src/lib.rs)、[工程對照紀錄](https://app.notion.com/p/3f7e51d4509381faa894d3660c3252f7)。
本節只列當前狀態與證據入口；歷次結果、來源變化及完整日誌留在工程紀錄，不在總文章累加歷史摘要。
## 實作、發布及驗證的分工
每日教學核對共同總綱、相關Day、兩庫最新來源、差異、所屬測試及適用CI。每日教學發布條件僅檢視 rust_ai_chat 最新提交的 CI：即使未全部 Success 或已有失敗，教學仍可發布；須列明 rust_ai_chat 當前 CI 狀態、失敗與待修問題，不宣稱驗證通過。copilot_use 的 CI 不屬於每日教學發布 Gate；其 CI 結果只供 copilot_use 產品實作、整合及功能驗收各自判定。每日發布不等待週六工程驗收，也不把既有CI當新教學程式已驗證。
週六00:00 Asia/Taipei完整比較tracked程式、介面、流程、DB／migration、UI、設定與測試，必要的copilot_use整合與功能回歸另存工程證據；每日沿用已確認缺口，不重做全量。工程完成、每日發布及學習庫套用各自記錄。固定正式tag及附件、已套用migration和既有證據保持可追溯，新成果不得覆蓋成歷史通過。
本機採原生環境，不使用Docker或隔離runtime。Domain／Application不依賴DB／HTTP／OS／UI Driver，SQL、DB方言、交易與平台差異留在Adapter；不同DB分別驗證，已套用migration不改寫。PostgreSQL UUID或VARCHAR(40)由Adapter適配，不滲入Domain契約。
Dry Run只處理使用者指出的文件／規則／路徑問題，程式、測試與migration唯讀；來源證據不足明示。教學、註解與自有UI用繁體中文。AI provider既定OpenAI；不讀API key、私人環境檔或\~/.bashrc、不執行付費AI、不把密鑰寫入Git／Notion／報告。週六功能Gate不重測AI對話、串流、生圖、RAG、MCP等AI能力；沿用有效證據或標示未驗證，不將功能通過當成AI已重測。
## 文件入口
產品能力與未來清單、詳細架構、操作設定、功能對照與工程證據仍保留各自用途；本篇集中教學方法、格式及三專案大綱，其他教學入口改為引用。原Day及程式保留，舊規則／來源／失效教法只作歷史，不再套用為目前指令。
[聊天室](https://app.notion.com/p/3f3e51d45093807da638c336e857543a) · [桌面寵物](https://app.notion.com/p/3f3e51d450938067923ae5a424d30950) · [統計儀錶板｜聊天室 × 桌面寵物](https://app.notion.com/p/3f3e51d45093817eafaee0cfc9ea4cc8)。功能與實作對照的詳細歷史保留工程紀錄；本篇不再追加歷次討論或驗證摘要。
## Day索引入口
[聊天室](https://app.notion.com/p/3f3e51d45093807da638c336e857543a)
[統計儀錶板｜聊天室 × 桌面寵物](https://app.notion.com/p/3f3e51d45093817eafaee0cfc9ea4cc8)
[桌面寵物](https://app.notion.com/p/3f3e51d450938067923ae5a424d30950)
[文件與教學封存（非現行規則）](https://app.notion.com/p/3f7e51d4509381faa894d3660c3252f7)
