# 三專案共用文章｜學習規則與交付方式

[文件總入口](README.md) · [目前進度](teaching/current/STATUS.md) · [Chat 首頁](teaching/chat/README.md) · [Pet 首頁](teaching/pet/README.md) · [Analytics 首頁](teaching/analytics/README.md)

三案 next_day 以 [learning-state.json](learning-state.json) 即時值為準；教學交付、程式套用與功能驗收分別記錄。本文是教學政策唯一正本；新教學在 [daily](teaching/daily/README.md)，必要驗證完成才移至 `docs/teaching/archived/day-NN/{project}.md`；既有歷史正文與原始證據維持於 [history](teaching/history/README.md)。

### 現行需求與已廢除規則

P9 規則已廢除且與現況不同，不再作為產品目標、選題、待辦優先順序、驗收門檻或全面比對依據。需求以使用者最新確認內容為優先，再依現行使用者流程、人員驗證及 UI 條件核對；實作與驗證以 rust_ai_chat 精確 SHA 為證據。若現行文件仍殘留舊要求，先記錄差異並更新，不從已廢除規則推定必做功能。

2026-10-23 另確認廢除兩項舊工作要求：① 自教學起始階段分批搬移 11 個既有測試檔至指定 crate；② 依 rust_ai_chat 目錄、型別、Port、介面及所屬測試更新教學對應與理由，以及建立平行程式。兩項不再作為活動待辦、Refactor 固定步驟、教學選題或驗收門檻；歷史文章中的描述不恢復其效力。現行程式核對與證據要求仍依本文其他規則，copilot_use 的文件交付分工不變。[廢除紀錄](teaching/history/evidence/2026-10-23-retired-legacy-work-items/REPORT.md)。

### 現況文章的呈現方式

現況文章與專案首頁只說明功能、實作狀態、驗證結果、缺口及來源證據，不在正文列教學日編號或「驗證支持哪些 Day」的對照。例如僅寫「DB 驗證」，再列實際範圍與結果。教學編號、版本、交付、封存與補教追蹤保留於既有狀態檔、manifest、教學索引和原始歷史證據；本規則不改變進度或驗收條件。

## 每日 TDD 教學最小必要規則 v1

規則版本：daily-tdd-minimum-v1（GitHub 交付政策 github-only-v1，2026-10-22）。GitHub <code>docs/COMMON_LEARNING_RULES.md</code> 是現行規則正本，唯一教學交付為 williechen/copilot_use 的 main 分支；不以舊對話或歷史決議覆蓋本版。規則／索引衝突須揭露並核對，不能默認已同步。

### 儲存庫分工（2026-10-23 確認）

- **williechen/copilot_use/main：** 規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本。
- **williechen/rust_ai_chat/main：** 唯一程式、測試、CI 與功能驗收主體。UI／HTTP／IPC／DB／ACL、Cargo／lock／套件、命令及實作缺口均以該庫當時精確 SHA 核對；教學流程保持原始碼唯讀，由使用者套用。
- 兩庫 SHA 分開記錄為「文件交付 SHA」與「程式來源 SHA」。copilot_use 既有程式、scripts、Actions、Release 或本地測試不作主體程式的驗收 Gate，也不能替代 rust_ai_chat 的功能現況或 CI 證據。
- learning-state／manifest 中的舊 validated_program_sha、daily_product_sha、fixed_tag、產品試驗與 weekly acceptance 保留其歷史來源用途；不得據此選取本次程式 checkout 或自動宣稱主體功能已完成。next_day 只按各案即時值與交付證據處理。
- 既有 Day、commit／blob／SHA256、歷史原文及固定發行證據不回寫。此分工更新不增加 Day、不結案補教，也不代表已變更既有雲端任務、GitHub workflow 或排程設定。

### 1. 來源與選題

每次實際重讀 copilot\_use 的 <code>docs/COMMON_LEARNING_RULES.md</code>、<code>docs/learning-state.json</code>、<code>docs/requirements/USER_FLOWS.md</code>、<code>docs/requirements/AUTHENTICATION.md</code>、<code>docs/FUTURE_WORK.md</code>、<code>docs/SYSTEM_ARCHITECTURE.md</code>、<code>docs/UI_ALIGNMENT_REQUIREMENTS.md</code>、<code>docs/teaching/current/manifest.json</code>，以及 GitHub 三專案入口、<code>docs/teaching/daily/README.md</code> 與相關 Day。取得 rust\_ai\_chat/main 的程式來源 SHA 與 copilot\_use/main 的文件交付 SHA；只在 rust\_ai\_chat 核對本日程式、實際呼叫端、測試及差異，在 copilot\_use 核對規則、文章、索引與交付證據。必要來源讀取失敗，只暫停受影響專案，不能以快取冒充最新。

發布每篇前再取 main SHA 及必要規則版本；來源 SHA 改變就重建所有尚未發布草稿，規則改變則重新核對、必要時重建。已完整發布讀回的 Day 不自動撤銷。記錄規則版本、文章 commit／blob SHA、全文 SHA256 與讀回時間、GitHub 規則 blob／commit SHA、兩庫來源 SHA 及讀取時間。rust\_ai\_chat 全庫唯讀，教學程式由使用者套用；copilot\_use 只維護文件與教學交付，不另作產品程式整合或功能驗收。

#### 既定技術分層與 Leptos UI 強制規則（2026-10-22 確認）

本規則是**沿用原本架構、修正教學偏差**，不是重新選型或變更產品架構；與現行使用者流程、SYSTEM_ARCHITECTURE、UI_ALIGNMENT_REQUIREMENTS 並行，發生矛盾應先核對最新真實程式與規則正本，不得跳過既有呼叫鏈。

- **Leptos** 是三案適用的 UI／頁面元件框架：Chat 使用既有 Leptos SSR／Hydration 與 Router；Pet 維持 Leptos CSR + Tauri；Analytics 的正式登入、修改密碼及管理後台頁面應逐步採用 Leptos 元件、表單、Signals／Actions／Resources／Router，而不是以 Axum `Html(...)`／`Html(format!(...))` 完整字串頁面作為新 UI Green。
- **Axum** 負責 HTTP API、WebSocket、Routing／Middleware、Cookie、Session、安全性檢查與 ACL。保留既有可運作的 API／Session／認證契約；逐步將頁面呈現從 Axum HTML Handler 分離。Axum 可處理 HTTP response，但不得藉此取代原有 Leptos UI。
- **Application／Domain** 負責使用案例、流程、業務與領域規則，UI 不直接存取資料庫或偷渡授權邏輯。**SQLx** 在各自的持久化 Adapter 處理資料查詢、交易與 migration；Chat／Pet SQLite 與 Analytics PostgreSQL 維持隔離。Pet 的 Tauri IPC 只負責原生橋接與視窗操作，配合 Leptos CSR。
- **增量式完整呼叫鏈：** 每 Day 只新增一個最小可套用切片，需核對並說明真實 `Leptos UI → Axum HTTP/API 或 Tauri IPC → Application/Domain → SQLx Adapter` 呼叫鏈；**重用已存在的分層，不要求每日每層都修改**。Red → 完整 Green → Refactor 聚焦當日變更與真實呼叫端；僅測 helper/API、假設性畫面不可冒充已完成 UI 流程，未跑測試必須明記。
- **動態 Day：** 每次從 `docs/learning-state.json` 取得各案 `next_day=N`，核對上一 Day 正文、manifest、交付／驗收證據、未完成事項及最新兩庫 SHA，再銜接 N；不得固定指定歷史 Day、跳號、回退或重發。更正使用版本化修訂／後續補教，保留原始 commit／blob／SHA256，不因勘誤增加 Day。
- **推進條件：** 只有確認 Day N 的 GitHub 文章尚未完成交付、獨立提交並通過精確 commit 全文讀回後，才以條件式同步推進 N→N+1；已 N+1 且證據匹配不重複計數。歷史封存及已發文章均不得覆寫原始證據。
- 每日教學 CI 的真實測試來源**只採 `williechen/rust_ai_chat` Actions 精確 workflow／job／SHA／覆蓋**；`copilot_use` 的分支、commit、blob、SHA256 僅驗證規則、文章與進度交付，不得引用其 Actions 冒充教學 Green／產品驗收。保留程式套用、CI、DB／GUI 與功能驗收互相獨立的認定。

#### 基本可操作流程優先

每日選題必須先核對最新 <code>rust_ai_chat/main</code> 的真正入口、可操作 UI、HTTP/IPC、Session/權限、SQLite/PostgreSQL 持久化和該庫 CI；<code>copilot_use</code> 僅讀取需求、教學、進度與原始證據；找出使用者操作流程中最早缺失的必要環節，先教／補最小可套用、已連至真實呼叫者的 Red→Green。不能因 API、helper、單元測試、HTTP 狀態碼、視窗動畫已存在，就跳過前面的註冊、登入或寵物建立。若早期流程尚未可用，不優先教後段的歷史訊息、漫遊細節或登入限流細節；未完成的同一功能跨日延續，依最新程式重新判斷，不預排 Day。

- <b>Chat 基本功能門檻：</b> 使用者能註冊／登入，帳號資料與 Session 合理持久化，未登入者不能進入需要授權的聊天室；登入後可看見房間清單並加入或建立房間，才能依需要開發歷史訊息、重連及進階 UI。只有首頁 <code>/signin</code> 連結，或僅能透過 <code>/room/:id</code> 開啟頁面，不等於登入與授權鏈完成；須檢查後端、前端與 SQLite 之間的實際接線。
- <b>Pet 基本功能門檻：</b> 使用者可建立第一隻具身分、名稱、初始外觀的寵物，透過 Tauri IPC 真正儲存 SQLite、顯示於生活視窗，重新啟動後能載入。<code>PetMachine::default()</code> 不等於使用者建立寵物；在建立、載入流程可用前，視窗漫遊／拖曳／找回與多寵物進階特性只列後續項目。
- <b>Analytics 基本功能門檻：</b> 建立並持久化管理員帳號，安全的加鹽密碼雜湊、首次登入強制改密碼、有效 Session 與後台頁面授權先接通，再深化角色、帳號級登入等待、報表及來源事件。

主體為各專案每天**一個**可驗證的最小切片，整體能力可跨 Day 漸進完成；每日新內容須接到既有真實呼叫鏈。教學文章完整交付可推進 Day，程式套用與產品驗收獨立判定；產品驗收條件依使用者最新確認內容、[現行使用者流程](requirements/USER_FLOWS.md)與[UI 驗收條件](UI_ALIGNMENT_REQUIREMENTS.md)逐項核對。

### 2. 文章最小格式

沿用四節：①學習大綱＝目標、現況、前置條件與本日差異；②實作內容＝Red → 完整最小 Green → Refactor → 必要的舊 Day 更正補充；③套件與軟體版本＝本日涉及的每項套件、Rust toolchain、框架、資料庫、CLI／工具，均核對**交付當日官方最新穩定版**（排除 nightly／alpha／beta／RC），列明官方來源連結與核對日期、`rust_ai_chat` manifest 宣告版、Cargo.lock resolved 版或已安裝版、版本差異、MSRV／native／WASM／DB 相容性，以及建議採用版本與理由；查核不全要明示，**尚未確認本篇涉及項目的最新穩定版，不得通過文件 Gate 1**。最新穩定版為教學選用的優先目標；不相容時交代阻礙、必要遷移及測試，不得假稱已升級；`rust_ai_chat` 唯讀，不自動修改 lock、執行 `cargo update`、安裝軟體或套用 migration；④驗證項目＝測試、證據、待完成與下一步。

每篇只教一個最小完整切片，提供真實檔案位置、必要完整修改與繁體中文說明，交代入口到資料／畫面的呼叫鏈。Red 必須對應同篇 Green，預期行為失敗、缺少待新增符號的預期編譯失敗、匯入／環境錯誤分別標示；已通過的既有測試不能冒稱 Red。不能以未確認的建窗標籤或介面假設冒稱完整可套用 Green；必要契約未核實先補齊，否則只存草稿。

大功能可跨日；原切片未完成如實保留，不把 API、欄位或純補測當完整新功能。已封存 Day 不回寫；尚未封存草稿的未實作部分可在原頁修訂；狀態不明先列未核實。重大錯誤由人工逐案裁定影響範圍及是否版本化重發，保留原文與版本；更正不自動新增 Day 或倒退進度。

#### 封存與補教

三案 Day25（含）以前正文、程式與證據不回寫；封存不代表產品驗收。每次先核對最新來源、真實呼叫鏈、CI，以及本篇涉及套件和軟體的 manifest／lock／**官方最新穩定版與發行資訊**（依第③節），再將必要更正放在下一個適用原教學第2節的新 Red／完整 Green／Refactor 之後，另列「舊 Day 更正補充」。提供真實路徑、必要完整修改、版本理由及驗證範圍。

**雙軌補教：** ① 同案上一 Day 第④節及狀態中的未完成事項，於新 Day 第②節當日 Red → Green → Refactor 後增列「上一 Day 未完成事項補充」，標明來源、缺口、必要程式修補、測試命令與預期／實際；未驗收持續追蹤。② 更早的 `makeup_queue` 嚴格 FIFO，每篇最多**結案一個**舊 Day 補教單元。相同問題去重、不能跳過較早項；補充教學或全文讀回本身不代表程式驗收通過。未完成與補教均不新增 Day，不更新封存舊文章。

登入、登出、Cookie 與 Session 採共用實作，各系統透過儲存介面及政策接入；SQLite／PostgreSQL 與登入狀態保持獨立。Pet 每窗單一元件、動畫每幀單一寵物圖像及至少三個不同 pet id 的生活窗各自驗證。Analytics 須驗完整 postgres-integration targets 與全新建庫流程。

### 3. 測試與證據

**Gate 1（文件交付）：** 真實路徑與現有呼叫鏈、完整可套用 Red／Green／Refactor、**第③節官方最新穩定版核對與相容性判斷**、測試設計及預期／實際明列，GitHub 每篇獨立 commit、精確 SHA 全文讀回和狀態同步完成，方可推進 next_day。**Gate 2（程式驗收）：** 使用者在 `rust_ai_chat` 套用後依精確 SHA 進行測試／CI／DB／HTTP／GUI／端到端核對；不直接影響 next_day，僅影響切片驗收及封存。既有 CI Success 不能證明未套用的新 Green 成功；CI Fail/Pending 也不能單獨阻止合格文件發布。CI 須揭露精確來源 SHA、workflow/job、狀態、時間與覆蓋；`copilot_use` CI 不替代主體程式驗收。

每項列命令、預期、實際結果、證據 SHA、覆蓋範圍、未執行／失敗／待修。未執行明示，不捏造 Red／Green 成功；教學發布、使用者套用、產品整合、功能驗收分別記錄。跨 SHA 引用證據須核對相關差異及依賴，不能直接沿用成功結論。寫入／補償成功不等於測試成功，驗證範圍縮小須列剩餘缺口。

### 4. GitHub 發布、獨立 commit 與 Day 推進

GitHub `williechen/copilot_use/main` 為唯一文章交付處：新篇位於 `docs/teaching/daily/day-NN/{chat,pet,analytics}.md`；必要 Gate 2 驗證全部通過後，保持正文 byte-for-byte 不變移至 `docs/teaching/archived/day-NN/{project}.md`。原交付 commit、blob SHA、SHA256 及 Git 歷史不可改寫，manifest 更新封存狀態與實際路徑，更新閱讀索引及舊路徑對照。既存 `history/day-NN` 文章與證據維持原位；封存不代表整體產品完成。

**每新增一個專案 Day 就獨立 commit 並推送**，提交訊息含專案／Day／修訂；不能累積多天或三案後才一起提交。文章 commit 完成後從 GitHub 精確 commit SHA 讀回全文，逐字核對四節、完整 Red／Green／程式碼與來源版本，保存路徑、blob SHA、SHA256、讀回時間及結果；commit 存在或工具寫入成功本身不代表交付成功。

每專案獨立：文章 commit 推送＋全文讀回通過，才重新讀最新 learning-state、manifest、blob／branch SHA，以 expected branch SHA 條件提交原子同步該案 next_day（預期 N→N+1）、manifest 與必要索引／有證據的 makeup_queue 變更，再讀回狀態。狀態提交可另 commit，引用已驗證文章 commit，不合併下一篇原教學。若已 N+1 且證據一致只補缺少資料，不再遞增；值或 SHA 不符先重讀，無法安全合併就暫停受影響更新，禁止強制覆寫。完成一案全部 gate 後再交付下一案。

**原文不可變＋版本化修訂：** 發布後保留原始 commit／blob／SHA256；發現錯誤另存 r2/r3 修訂與勘誤證據，不覆寫原版本的歷史證據、不額外增加 Day，未通過 Gate 2 不得封存。既有 `makeup_queue` 嚴格 FIFO 且每篇最多結案一項，補教失敗不回退 Day；規則整理、驗證、修訂及封存均不增加 Day。

**文件職責：** 本文唯一管理教學流程、TDD、交付、補教及封存；[現行使用者流程](requirements/USER_FLOWS.md)與[人員驗證需求](requirements/AUTHENTICATION.md)管理已確認需求，[SYSTEM_ARCHITECTURE](SYSTEM_ARCHITECTURE.md) 管架構分層，[UI_ALIGNMENT_REQUIREMENTS](UI_ALIGNMENT_REQUIREMENTS.md) 管 UI 契約，[FUTURE_WORK](FUTURE_WORK.md) 管待辦；`learning-state.json` 管即時進度，`manifest.json` 管文章與版本索引。實際程式現況只以 `rust_ai_chat/main` 精確 SHA 為準。不同文件發生矛盾先記錄核對，不擅改產品要求；根 README 僅導覽。

### 5. 最小失敗恢復

文章已 commit 並全文讀回但狀態寫入／讀回失敗，保留文章路徑、commit／blob SHA、SHA256、專案／Day／修訂、來源 SHA、預期狀態與原始錯誤；下次先重讀 GitHub，只補缺少狀態，不重發、不重複加 Day。未保存全文讀回證據不得宣稱 gate 完成。狀態不明只暫停受影響專案，其餘來源完整者繼續。

**重試以 `project/day/revision/operation` 獨立計數**：article_publish、article_readback、state_sync、manifest_sync、archive 各自去重記錄；同一操作連續三次失敗才標 `needs_manual_review`，成功即將該操作連續失敗數歸零，只暫停該操作與依賴步驟。狀態不明先從 GitHub 讀回辨認結果，不能盲目重送；保存原始錯誤、期望 SHA 與實際結果，失敗紀錄無法保存須明示。推送前核對其他任務與分支 SHA，以 expected_sha 條件提交，不得強制覆寫。

### 6. 時程與邊界

每日 06:00 Asia/Taipei，依 Chat → Pet → Analytics。**核心學習 60 分鐘**：三案各 15 分鐘均包含 Red → Green → Refactor，加共用彈性 15 分鐘。前一 Day 未完成補教、FIFO 舊補教、來源／套件版本核對、文章①③④節、GitHub commit／全文讀回／同步及 Gate 2 測試／整合驗收一律**額外計時**，不得因時間壓力省略最小完整 Green；超額工作記錄為待完成，不假裝驗收通過。

**雙週期來源核對：** 每日增量核對只檢查當日教學切片、真實呼叫鏈、相關測試、版本、上一 Day 與 pending，來源不足只暫停受影響專案。**每週至少一次全專案重新比對**，現行每週日 08:00 Asia/Taipei（彈性執行，排程啟用）；逐案核對 `rust_ai_chat/main` 全部實際程式、Leptos UI、Axum／Tauri IPC、Domain／Application、SQLx、DB／migration、認證與 ACL、配置、測試／CI，與 `copilot_use/main` 的現行使用者需求、架構、UI 契約、文章、狀態、manifest、待辦及版本證據比對。記錄兩庫精確 SHA、差異與證據，去重追蹤並供每日選題使用；不以週比對取代每日增量。只有全面核對**實際完整成功**才更新 `last_full_comparison`；失敗或證據不足不得更新，且不因比對推進 Day 或結案補教。兩庫分工不變，`rust_ai_chat` 始終唯讀。

保留原生環境、Domain／Application 與 Adapter 邊界、各 DB 獨立驗證、migration／固定 tag／原始證據不可改寫、權限與檔案操作確認。不讀正式秘密、不執行付費 AI；產品驗收不得冒稱本次已重跑。學習階段與實際產品驗收分開，已廢除的規則不作驗收依據。

### 文件整理與歷史追溯

**封存門檻與固定路徑：** 全部已發布文章逐篇驗證；只有本篇 Gate 2 所要求的實測證據全部成立才把正文**原樣**移至 `docs/teaching/archived/day-NN/{project}.md`。沒有實際程式套用、測試、DB／HTTP／GUI 證據不可因文件讀回通過而封存。manifest 記錄已驗收狀態、新路徑、來源 SHA、原始 commit／blob／SHA256 與驗證證據；原始 Git 歷史不改寫、舊連結維護路徑對照；修訂、封存不推進 Day。

**索引與歷史：** 共用文章與三案首頁是正式閱讀入口；`daily/day-NN/{project}.md` 為尚待驗收的現行文章，`archived/day-NN/{project}.md` 為新政策下已驗收的原文。過往已存入 `history/day-NN` 的文章與原始封存證據維持其實際位置，不做批次強制搬移；跨 Day 報告留在 `history/evidence`，舊路徑以 [PATH_MAP](teaching/history/PATH_MAP.json) 與 Git commit 追溯。
