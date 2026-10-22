# 三專案共用文章｜學習規則與交付方式

[文件總入口](README.md) · [目前進度](teaching/current/STATUS.md) · [Chat 首頁](teaching/chat/README.md) · [Pet 首頁](teaching/pet/README.md) · [Analytics 首頁](teaching/analytics/README.md)

三案 next_day 以 [learning-state.json](learning-state.json) 即時值為準；教學交付、程式套用與功能驗收分別記錄。共同規則只在本篇維護，歷史正文、封存與原始證據集中於 [歷史資料](teaching/history/README.md)。

## 每日 TDD 教學最小必要規則 v1

規則版本：daily-tdd-minimum-v1（GitHub 交付政策 github-only-v1，2026-10-22）。GitHub <code>docs/COMMON_LEARNING_RULES.md</code> 是現行規則正本，唯一教學交付為 williechen/copilot_use 的 main 分支；不以舊對話或歷史決議覆蓋本版。規則／索引衝突須揭露並核對，不能默認已同步。

### 儲存庫分工（2026-10-23 確認）

- **williechen/copilot_use/main：** 規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本。
- **williechen/rust_ai_chat/main：** 唯一程式、測試、CI 與功能驗收主體。UI／HTTP／IPC／DB／ACL、Cargo／lock／套件、命令及實作缺口均以該庫當時精確 SHA 核對；教學流程保持原始碼唯讀，由使用者套用。
- 兩庫 SHA 分開記錄為「文件交付 SHA」與「程式來源 SHA」。copilot_use 既有程式、scripts、Actions、Release 或本地測試不作主體程式的驗收 Gate，也不能替代 rust_ai_chat 的功能現況或 CI 證據。
- learning-state／manifest 中的舊 validated_program_sha、daily_product_sha、fixed_tag、產品試驗與 weekly acceptance 保留其歷史來源用途；不得據此選取本次程式 checkout 或自動宣稱主體功能已完成。next_day 只按各案即時值與交付證據處理。
- 既有 Day、commit／blob／SHA256、歷史原文及固定發行證據不回寫。此分工更新不增加 Day、不結案補教、不更新 last_full_comparison，也不代表已變更既有雲端任務、GitHub workflow 或排程設定。

### 1. 來源與選題

每次實際重讀 copilot\_use 的 <code>docs/COMMON_LEARNING_RULES.md</code>、<code>docs/learning-state.json</code>、<code>docs/P9_TARGET.md</code>、<code>docs/FUTURE_WORK.md</code>、<code>docs/SYSTEM_ARCHITECTURE.md</code>、<code>docs/UI_ALIGNMENT_REQUIREMENTS.md</code>、<code>docs/teaching/current/manifest.json</code>，以及 GitHub 三專案入口、<code>docs/teaching/daily/README.md</code> 與相關 Day。取得 rust\_ai\_chat/main 的程式來源 SHA 與 copilot\_use/main 的文件交付 SHA；只在 rust\_ai\_chat 核對本日程式、實際呼叫端、測試及差異，在 copilot\_use 核對規則、文章、索引與交付證據。必要來源讀取失敗，只暫停受影響專案，不能以快取冒充最新。

發布每篇前再取 main SHA 及必要規則版本；來源 SHA 改變就重建所有尚未發布草稿，規則改變則重新核對、必要時重建。已完整發布讀回的 Day 不自動撤銷。記錄規則版本、文章 commit／blob SHA、全文 SHA256 與讀回時間、GitHub 規則 blob／commit SHA、兩庫來源 SHA 及讀取時間。rust\_ai\_chat 全庫唯讀，教學程式由使用者套用；copilot\_use 只維護文件與教學交付，不另作產品程式整合或功能驗收。

#### 既定技術分層與 Leptos UI 強制規則（2026-10-22 確認）

本規則是**沿用原本架構、修正教學偏差**，不是重新選型或變更產品架構；與 P9_TARGET、SYSTEM_ARCHITECTURE、UI_ALIGNMENT_REQUIREMENTS 並行，發生矛盾應先核對最新真實程式與規則正本，不得跳過既有呼叫鏈。

- **Leptos** 是三案適用的 UI／頁面元件框架：Chat 使用既有 Leptos SSR／Hydration 與 Router；Pet 維持 Leptos CSR + Tauri；Analytics 的正式登入、修改密碼及管理後台頁面應逐步採用 Leptos 元件、表單、Signals／Actions／Resources／Router，而不是以 Axum `Html(...)`／`Html(format!(...))` 完整字串頁面作為新 UI Green。
- **Axum** 負責 HTTP API、WebSocket、Routing／Middleware、Cookie、Session、安全性檢查與 ACL。保留既有可運作的 API／Session／認證契約；逐步將頁面呈現從 Axum HTML Handler 分離。Axum 可處理 HTTP response，但不得藉此取代原有 Leptos UI。
- **Application／Domain** 負責使用案例、流程、業務與領域規則，UI 不直接存取資料庫或偷渡授權邏輯。**SQLx** 在各自的持久化 Adapter 處理資料查詢、交易與 migration；Chat／Pet SQLite 與 Analytics PostgreSQL 維持隔離。Pet 的 Tauri IPC 只負責原生橋接與視窗操作，配合 Leptos CSR。
- 往後每一 Day 的 Red → 完整最小 Green → Refactor 須覆蓋真實 `Leptos UI → Axum HTTP/API 或 Tauri IPC → Application/Domain → SQLx Adapter` 的該日最小可操作切片。沒有真實 UI 呼叫端、只有 Axum 回傳 HTML、API/helper 單獨成功或假設性頁面者，不能宣稱完成 Leptos UI 功能；測試未跑與未驗收仍須明記。
- **先核對已發布 Day27，再從最新 next_day 銜接，不跳號**：下次每日教學先逐案讀回 Day27 正文、最新 `rust_ai_chat/main`、`copilot_use/main`、規則、manifest、索引及實際 UI／HTTP／IPC／DB 呼叫鏈，辨識 Chat／Analytics Axum HTML 與既有 Leptos 的架構偏差及 Pet 現況。必要更正須在後續對應 Day 的 Red／完整 Green／Refactor 之後記錄，或走保留原 commit／blob／SHA256 的版本化勘誤流程；不得覆寫、抹除 Day27 原始 GitHub 證據，亦不得將核對／修訂自動計為新 Day。對原文已完成的功能不能假裝尚未完成或重教。
- 新文章 Day **僅**取 `docs/learning-state.json` 當時各案 `next_day`（本次規則修改時三案均為 28，僅為核對快照、不是寫死後續 Day）；確認 N 的文章與同步狀態沒有完成證據才依既有獨立 commit／全文讀回／條件同步流程推進 N→N+1。絕不因「從 Day27 銜接」而回退至 27 或跳過 28。Day25（含）以前封存、Day26／Day27 已發布文章與歷史證據不因本規則更新回寫。
- 每日教學 CI 的真實測試來源**只採 `williechen/rust_ai_chat` Actions 精確 workflow／job／SHA／覆蓋**；`copilot_use` 的分支、commit、blob、SHA256 僅驗證規則、文章與進度交付，不得引用其 Actions 冒充教學 Green／產品驗收。保留程式套用、CI、DB／GUI 與功能驗收互相獨立的認定。

#### 基本可操作流程優先

每日選題必須先核對最新 <code>rust_ai_chat/main</code> 的真正入口、可操作 UI、HTTP/IPC、Session/權限、SQLite/PostgreSQL 持久化和該庫 CI；<code>copilot_use</code> 僅讀取需求、教學、進度與原始證據；找出使用者操作流程中最早缺失的必要環節，先教／補最小可套用、已連至真實呼叫者的 Red→Green。不能因 API、helper、單元測試、HTTP 狀態碼、視窗動畫已存在，就跳過前面的註冊、登入或寵物建立。若早期流程尚未可用，不優先教後段的歷史訊息、漫遊細節或登入限流細節；未完成的同一功能跨日延續，依最新程式重新判斷，不預排 Day。

- <b>Chat 基本功能門檻：</b> 使用者能註冊／登入，帳號資料與 Session 合理持久化，未登入者不能進入需要授權的聊天室；登入後可看見房間清單並加入或建立房間，才能依需要開發歷史訊息、重連及進階 UI。只有首頁 <code>/signin</code> 連結，或僅能透過 <code>/room/:id</code> 開啟頁面，不等於登入與授權鏈完成；須檢查後端、前端與 SQLite 之間的實際接線。
- <b>Pet 基本功能門檻：</b> 使用者可建立第一隻具身分、名稱、初始外觀的寵物，透過 Tauri IPC 真正儲存 SQLite、顯示於生活視窗，重新啟動後能載入。<code>PetMachine::default()</code> 不等於使用者建立寵物；在建立、載入流程可用前，視窗漫遊／拖曳／找回與多寵物進階特性只列後續項目。
- <b>Analytics 基本功能門檻：</b> 建立並持久化管理員帳號，安全的加鹽密碼雜湊、首次登入強制改密碼、有效 Session 與後台頁面授權先接通，再深化角色、帳號級登入等待、報表及來源事件。

主體為各專案每天 <b>一個</b>可驗證的最小 Green 切片，功能可跨日，真正使用者流程尚未可用須在文章與進度明示缺口；以最新程式與資料庫契約優先，不強制當天一次完成註冊／登入或整套寵物建立。原 P9 產品目標、功能 Gate、歷史 Day、四節文章、GitHub commit 推送及全文讀回、條件式更新 <code>next_day</code>、CI 覆蓋範圍與每日／週六各自驗收規則全部保留。

### 2. 文章最小格式

沿用四節：①學習大綱＝目標、現況、前置條件與本日差異；②實作內容＝Red → 完整最小 Green → Refactor → 必要的舊 Day 更正補充；③套件版本＝本日涉及的 Cargo／lock、stable／MSRV／native／WASM 核對，未查明示；④驗證項目＝測試、證據、待完成與下一步。

每篇只教一個最小完整切片，提供真實檔案位置、必要完整修改與繁體中文說明，交代入口到資料／畫面的呼叫鏈。Red 必須對應同篇 Green，預期行為失敗、缺少待新增符號的預期編譯失敗、匯入／環境錯誤分別標示；已通過的既有測試不能冒稱 Red。不能以未確認的建窗標籤或介面假設冒稱完整可套用 Green；必要契約未核實先補齊，否則只存草稿。

大功能可跨日；原切片未完成如實保留，不把 API、欄位或純補測當完整新功能。已封存 Day 不回寫；尚未封存草稿的未實作部分可在原頁修訂；狀態不明先列未核實。重大錯誤由人工逐案裁定影響範圍及是否版本化重發，保留原文與版本；更正不自動新增 Day 或倒退進度。

#### 封存與補教

三案 Day25（含）以前正文、程式與證據不回寫；封存不代表產品驗收。每次先核對最新來源、真實呼叫鏈、CI 與涉及套件的 manifest／lock／官方穩定版本，再將必要更正放在下一個適用原教學第2節的新 Red／完整 Green／Refactor 之後，另列「舊 Day 更正補充」。提供真實路徑、必要完整修改、版本理由及驗證範圍。

各專案 makeup\_queue 採 FIFO、每篇最多一個舊 Day 單元；新差異去重入列，完整發布讀回才結案，不因補教、文件整理或驗證增加 Day。最新依賴不直接等於適合升級，不自動修改來源 lock。

登入、登出、Cookie 與 Session 採共用實作，各系統透過儲存介面及政策接入；SQLite／PostgreSQL 與登入狀態保持獨立。Pet 每窗單一元件、動畫每幀單一寵物圖像及至少三個不同 pet id 的生活窗各自驗證。Analytics 須驗完整 postgres-integration targets 與全新建庫流程。

### 3. 測試與證據

CI 必須核對並揭露精確 SHA、workflow／job、狀態、時間與連結；Success／Failure／Pending／Skipped／Cancelled／無紀錄均不單獨阻擋教學發布或 next\_day。CI Success 只證明實際執行的範圍，不能代替 DB、HTTP／WS、SSR／WASM、GUI 或端到端驗收；只有 rust\_ai\_chat 的精確 SHA／workflow／job 作主體程式 CI 證據；copilot\_use 的交付讀回或文件檢查只證明文件一致性，既有程式 CI 不納入主體驗收。

每項列命令、預期、實際結果、證據 SHA、覆蓋範圍、未執行／失敗／待修。未執行明示，不捏造 Red／Green 成功；教學發布、使用者套用、產品整合、功能驗收分別記錄。跨 SHA 引用證據須核對相關差異及依賴，不能直接沿用成功結論。寫入／補償成功不等於測試成功，驗證範圍縮小須列剩餘缺口。

### 4. GitHub 發布、獨立 commit 與 Day 推進

GitHub williechen/copilot_use 的 main 為唯一教學文章交付處。新文章一律放在 <code>docs/teaching/daily/day-NN/{chat,pet,analytics}.md</code>；依專案＋Day＋修訂查重，不複製已交付文章。既有 Day26 已歸入 daily/day-26；Day25（含）以前正文與原始證據統一歸入 history/day-NN，由三案首頁與歷史索引連結，搬遷不新增 Day。

**每新增一個專案 Day 就獨立 commit 並推送**，提交訊息含專案／Day／修訂；不能累積多天或三案後才一起提交。文章 commit 完成後從 GitHub 精確 commit SHA 讀回全文，逐字核對四節、完整 Red／Green／程式碼與來源版本，保存路徑、blob SHA、SHA256、讀回時間及結果；commit 存在或工具寫入成功本身不代表交付成功。

每專案獨立：文章 commit 推送＋全文讀回通過，才重新讀最新 learning-state、manifest、blob／branch SHA，以 expected branch SHA 條件提交原子同步該案 next_day（預期 N→N+1）、manifest 與必要索引／有證據的 makeup_queue 變更，再讀回狀態。狀態提交可另 commit，引用已驗證文章 commit，不合併下一篇原教學。若已 N+1 且證據一致只補缺少資料，不再遞增；值或 SHA 不符先重讀，無法安全合併就暫停受影響更新，禁止強制覆寫。完成一案全部 gate 後再交付下一案。

文章修訂、搬遷、規則同步、補教、CI 或週六比對均不增加 Day。各專案 makeup_queue 採 FIFO，每篇最多一項舊 Day 單元，有完整發布讀回證據才結案；補教失敗不回退已確認原教學。教學交付、程式套用、產品整合與功能驗收分開記錄。

停止 Notion 教學建立、更新、發布與每日讀回依賴。共同主頁及三案教學子頁依使用者確認，先逐頁原文搬回 GitHub、全文讀回驗證，再移至 Notion 垃圾桶（可還原）；不永久刪除、不影響其他 Notion 資料。原文、頁面 metadata、父子關係及 GitHub 讀回證據保存於 <code>docs/teaching/history/evidence/notion-export/2026-10-22/</code>。Day25（含）以前 GitHub 封存檔不改寫；原文搬遷只新增保存，不重新發文或改進度。

### 5. 最小失敗恢復

文章已 commit 並全文讀回但狀態寫入／讀回失敗，保留文章路徑、commit／blob SHA、SHA256、專案／Day／修訂、來源 SHA、預期狀態與原始錯誤；下次先重讀 GitHub，只補缺少狀態，不重發、不重複加 Day。未保存全文讀回證據不得宣稱 gate 完成。狀態不明只暫停受影響專案，其餘來源完整者繼續。

每專案／Day／修訂單一 pending，重試先讀取去重；同項連續三次失敗轉 needs_manual_review，停止相同自動寫入，人工重新啟用才恢復。失敗紀錄無法保存須明示；不新增多層補償系統。開始及推送前核對其他執行中任務與分支 SHA，使用條件提交避免併發覆寫。

### 6. 時程與邊界

每日 06:00 Asia/Taipei，依 Chat → Pet → Analytics；主體 Red／最小 Green 學習合計最多 60 分鐘，各預留 15 分鐘，加共用彈性 15 分鐘。保留既有主體學習計時口徑：Refactor、必要補加／補教及第1、3、4節另列估時，不隱藏成本，也不為填滿時間擴大切片。超出主體預算的內容留待後續，不以缺少 Green 的文章冒稱完成；執行遇到時間上限先保存可恢復進度。

週六 00:00 Asia/Taipei 獨立核對 rust_ai_chat 的 tracked 程式、介面、呼叫流程、契約、DB／migration、UI、設定、測試與 CI，再核對 copilot_use 文件／教學與該程式來源的對應；只有完整成功才更新 last\_full\_comparison 並讀回。每日做增量核對，不覆蓋完整比對時間。週六失敗不自動阻擋每日教學，須揭露並重新讀取必要來源。既有排程原地更新，不建立重複任務；指令保存不等於排程已更新或實際執行成功。

保留原生環境、Domain／Application 與 Adapter 邊界、不同 DB 分別驗證、既有 migration／固定 tag／證據不可改寫、權限與檔案操作確認。不得讀正式秘密或執行付費 AI；週六功能驗收不冒稱 AI 已重測。P9 產品目標、P0–P12 學習階段及實際驗收分開。

### 文件整理與歷史追溯

共用文章與三案首頁是正式閱讀入口。同一 Day／專案的正文、修訂與原始封存證據集中在 history/day-NN；跨 Day 的完整比對或來源證據集中 history/evidence。同內容原文可按雜湊去重，但保留每個來源的路徑、SHA256 與新位置。舊證據內記錄的原始路徑、SHA、結果不改寫，透過 [路徑對照](teaching/history/PATH_MAP.json) 解析；歷史版本也可在對應 Git commit 讀回。整理不新增 Day、不變更補教結案或完整比對時間。
