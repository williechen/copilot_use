# Rust AI 三專案：教學總綱與實作對照

> 現行規則正本：[Notion 共同總頁](https://app.notion.com/p/3f2e51d4509381ddb8a8ce7cb4cd3e60)。本文件為 GitHub 鏡像。

## 每日 TDD 教學最小必要規則 v1

規則版本：daily-tdd-minimum-v1／2026-10-18。這是正式規則變更，取代舊版每日流程；產品需求、安全邊界、既有 Day 與證據仍保留。Notion 共同總頁是現行規則正本，GitHub `COMMON_LEARNING_RULES.md` 為鏡像；不以舊對話或歷史決議覆蓋本版。規則／索引衝突須揭露並核對，不能默認已同步。

### 1. 來源與選題

每次實際重讀 copilot_use 的 `docs/COMMON_LEARNING_RULES.md`、`docs/learning-state.json`、`docs/P9_TARGET.md`、`docs/FUTURE_WORK.md`、`docs/SYSTEM_ARCHITECTURE.md`、`docs/UI_ALIGNMENT_REQUIREMENTS.md`、`docs/teaching/current/manifest.json`，以及 Notion 共同總頁、三專案入口與相關 Day。取得 rust_ai_chat/main 與 copilot_use 教學分支最新完整 SHA，核對本日相關程式、實際呼叫端、測試及差異。必要來源讀取失敗，只暫停受影響專案，不能以快取冒充最新。

發布每篇前再取 main SHA 及必要規則版本；來源 SHA 改變就重建所有尚未發布草稿，規則改變則重新核對、必要時重建。已完整發布讀回的 Day 不自動撤銷。記錄規則版本、Notion page_last_edited_at、GitHub 規則 blob／commit SHA、兩庫來源 SHA 及讀取時間。rust_ai_chat 全庫唯讀，教學程式由使用者套用；copilot_use 產品整合與教學分開。

**2026-10-19 補充：基本可操作流程優先（沿用 daily-tdd-minimum-v1，不改 Day 計數規則）**
每日選題必須先核對最新 `rust_ai_chat/main` 與對應 `copilot_use` 真正入口、可操作 UI、HTTP/IPC、Session/權限、SQLite/PostgreSQL 持久化和對應 CI；找出使用者操作流程中最早缺失的必要環節，先教／補最小可套用、已連至真實呼叫者的 Red→Green。不能因 API、helper、單元測試、HTTP 狀態碼、視窗動畫已存在，就跳過前面的註冊、登入或寵物建立。若早期流程尚未可用，不優先教後段的歷史訊息、漫遊細節或登入限流細節；未完成的同一功能跨日延續，依最新程式重新判斷，不預排 Day。
- **Chat 基本功能門檻：** 使用者能註冊／登入，帳號資料與 Session 合理持久化，未登入者不能進入需要授權的聊天室；登入後可看見房間清單並加入或建立房間，才能依需要開發歷史訊息、重連及進階 UI。只有首頁 `/signin` 連結，或僅能透過 `/room/:id` 開啟頁面，不等於登入與授權鏈完成；須檢查後端、前端與 SQLite 之間的實際接線。
- **Pet 基本功能門檻：** 使用者可建立第一隻具身分、名稱、初始外觀的寵物，透過 Tauri IPC 真正儲存 SQLite、顯示於生活視窗，重新啟動後能載入。`PetMachine::default()` 不等於使用者建立寵物；在建立、載入流程可用前，視窗漫遊／拖曳／找回與多寵物進階特性只列後續項目。
- **Analytics 基本功能門檻：** 建立並持久化管理員帳號，安全的加鹽密碼雜湊、首次登入強制改密碼、有效 Session 與後台頁面授權先接通，再深化角色、帳號級登入等待、報表及來源事件；現行固定 64 字元登入碼與 429 `Retry-After` 不等於帳密登入已完成。
主體為各專案每天 **一個**可驗證的最小 Green 切片，功能可跨日，真正使用者流程尚未可用須在文章與進度明示缺口；以最新程式與資料庫契約優先，不強制當天一次完成註冊／登入或整套寵物建立。原 P9 產品目標、功能 Gate、歷史 Day、四節文章、Notion 正式發布及全文讀回、條件式更新 `next_day`、CI 覆蓋範圍與每日／週六各自驗收規則全部保留。

### 2. 文章最小格式

沿用四節：①學習大綱＝目標、現況、前置條件與本日差異；②實作內容＝Red → 完整最小 Green → Refactor → 必要的舊 Day 更正補充；③套件版本＝本日涉及的 Cargo／lock、stable／MSRV／native／WASM 核對，未查明示；④驗證項目＝測試、證據、待完成與下一步。

每篇只教一個最小完整切片，提供真實檔案位置、必要完整修改與繁體中文說明，交代入口到資料／畫面的呼叫鏈。Red 必須對應同篇 Green，預期行為失敗、缺少待新增符號的預期編譯失敗、匯入／環境錯誤分別標示；已通過的既有測試不能冒稱 Red。不能以未確認的建窗標籤或介面假設冒稱完整可套用 Green；必要契約未核實先補齊，否則只存草稿。

大功能可跨日；原切片未完成如實保留，不把 API、欄位或純補測當完整新功能。已實作 Day 封存，未實作部分可在原頁修訂；狀態不明先列未核實。重大錯誤由人工逐案裁定影響範圍及是否版本化重發，保留原文與版本；更正不自動新增 Day 或倒退進度。

#### Day 22（含）以前文章封存與後續修正順序（2026-10-20）
依使用者確認，Chat／Pet／Analytics 的 Day 22（含）以前所有既有文章均已封存；封存範圍不只 Day 22 三篇。保留原文、程式、既有版本勘誤及歷史證據，不再直接修訂正文；即使舊文存在未實作、未驗證或錯誤，也只在下一 Day 的新實作／Refactor 後提出更正，不因這些狀態重新開放舊文。一般「未實作 Day 可原頁修訂」條款僅適用尚未封存的新草稿，不能套用到 Day 22（含）以前。封存本身不代表程式已套用或功能已驗收。每次下一 Day 前核對最新 rust_ai_chat/main、真實呼叫鏈、CI 與本日涉及套件的 manifest／lock／官方最新穩定版本；有差異時，在下一 Day 第2節的新 Red／完整 Green／Refactor 之後，另加「舊 Day 更正補充（程式／套件差異）」，提供真實路徑、必要完整修改、版本理由與驗證／未驗證範圍。原教學與補教分開，既有 makeup_queue 採 FIFO、每篇最多一個舊 Day 單元；新發現差異去重排入，不預先宣稱補教完成、不為修正新增 Day 或遞增 next_day。最新依賴不等於適合直接升級，不自動更新產品或來源 lock；跨主要版本需先核實 API 與相容性。

#### Day23（含）以前文章封存（2026-10-21）
使用者已確認三案Day23完成並封存；Day23（含）以前正文、程式與歷史證據全部保留，不再回寫。新差異只在Day24或之後的新Red／Green／Refactor後補充。封存與使用者完成回報不代替本地正式檢驗；next_day仍為24，未新增排程。

#### Day24（含）以前文章封存（2026-10-21）
使用者要求先封存三案Day24，包含既有程式與動畫內容。Day24（含）以前正文、程式及歷史證據保留，不再回寫；Cookie共用工具整理、Pet重複渲染與動畫多圖像的後續方案，僅在Day25或之後的新Red／Green／Refactor後補充。封存不代表完整產品驗收；next_day維持三案25，正式排程未恢復、未新增。

#### Day25（含）以前文章封存與需求覆核（2026-10-22 同步）

三案 Day25 已依既有封存 manifest 封存；本輪重新讀回三篇 Notion 全文，與 GitHub 封存正文 SHA256 逐字一致。Chat／Pet／Analytics 的 next_day 均為26，這是教學發布進度，不能當成功能驗收。Day25（含）以前正文、程式及歷史證據不回寫；後續差異按 FIFO 放在下一 Day 新 Red／完整 Green／Refactor 之後，不因驗證、文件同步或補教另增 Day。

登入、登出、Cookie 與 Session 必須採共用實作，各系統透過儲存介面及政策接入；SQLite／PostgreSQL 與各系統登入狀態仍獨立。Pet 至少以三個不同 pet id 的生活窗驗證獨立狀態；每窗單一元件與動畫每幀單一寵物圖像分別驗證，設定字串測試不能代替 GUI／事件授權實測。Analytics 需核對完整 postgres-integration targets、歷史 migration checksum 與舊資料保留，單一 Day25 target 不代表全部測試通過。

### 3. 測試與證據

CI 必須核對並揭露精確 SHA、workflow／job、狀態、時間與連結；Success／Failure／Pending／Skipped／Cancelled／無紀錄均不單獨阻擋教學發布或 next_day。CI Success 只證明實際執行的範圍，不能代替 DB、HTTP／WS、SSR／WASM、GUI 或端到端驗收；copilot_use CI 僅是產品證據。

每項列命令、預期、實際結果、證據 SHA、覆蓋範圍、未執行／失敗／待修。未執行明示，不捏造 Red／Green 成功；教學發布、使用者套用、產品整合、功能驗收分別記錄。跨 SHA 引用證據須核對相關差異及依賴，不能直接沿用成功結論。寫入／補償成功不等於測試成功，驗證範圍縮小須列剩餘缺口。

### 4. Notion 發布與 Day 推進

按專案＋Day＋修訂版查現有頁與索引，重用既有頁，不重複建同一教學。先完整發布 Notion，再讀回全文，核對四節、Red／完整 Green、程式碼、來源版本與索引；部分失敗保留失敗紀錄，重建同一文章的完整內容，成功讀回前不得推進。

每專案獨立處理：原教學 Day N 完整發布且讀回成功，才可將該專案 next_day 從預期 N 更新為 N+1。更新前重新讀取 learning-state 及 blob／branch SHA，依 Notion 證據於單次條件提交更新進度及既有 makeup_queue，保留其他欄位，寫後讀回；manifest 同步目前索引。若現值已為 N+1 且證據一致，視為已補更新，不再遞增。併發衝突重新讀取，不能安全合併就暫停該狀態更新。

文章修訂、規則同步、補教、CI 成功與週六比對都不自行推進 next_day。原教學與補教分開；已存在的 makeup_queue 採 FIFO，每篇最多一個舊 Day 補教單元，完整讀回才結案，不因補教失敗回退已確認的原教學進度。未完成原教學不冒充已發布；隔日新內容須標明原教學與待補內容，不能跳號掩蓋失敗。

### 5. 最小失敗恢復

Notion 已完整讀回但 GitHub 狀態寫入／讀回失敗，保留頁面 ID、專案／Day／修訂、來源版本、預期狀態及錯誤，下次先讀 Notion 與 GitHub，只補缺少的狀態，不重發、不重複加 Day。狀態不明只暫停受影響專案，其他來源完整者繼續。

證據檔保存失敗另記 pending 與錯誤；原教學已完整讀回時可依上節推進，不能聲稱證據已保存或測試已驗證。重試前先讀取去重，沿用同一筆待辦；同項連續三次失敗轉 needs_manual_review、停止相同自動寫入，人工重新啟用才恢復，保留歷史。若失敗紀錄也無法持久保存，回報明示。只保留必要失敗紀錄，不新增多層補償／cycle／復發風險系統。

### 6. 時程與邊界

每日 06:00 Asia/Taipei，依 Chat → Pet → Analytics；主體 Red／最小 Green 學習合計最多 60 分鐘，各預留 15 分鐘，加共用彈性 15 分鐘。保留既有主體學習計時口徑：Refactor、必要補加／補教及第1、3、4節另列估時，不隱藏成本，也不為填滿時間擴大切片。超出主體預算的內容留待後續，不以缺少 Green 的文章冒稱完成；執行遇到時間上限先保存可恢復進度。

週六 00:00 Asia/Taipei 獨立完整比對 tracked 程式、介面、呼叫流程、契約、DB／migration、UI、設定、測試、CI 與文件；只有完整成功才更新 last_full_comparison 並讀回。每日做增量核對，不覆蓋完整比對時間。週六失敗不自動阻擋每日教學，須揭露並重新讀取必要來源。既有排程原地更新，不建立重複任務；指令保存不等於排程已更新或實際執行成功。

保留原生環境、Domain／Application 與 Adapter 邊界、不同 DB 分別驗證、既有 migration／固定 tag／證據不可改寫、權限與檔案操作確認。不得讀正式秘密或執行付費 AI；週六功能驗收不冒稱 AI 已重測。P9 產品目標、P0–P12 學習階段及實際驗收分開。

### 排程模型設定（2026-10-22）

使用者指定模型 `gpt-6.1-sol`（GPT-6.1 Sol），推理強度 `medium`（中）。既有「三專案每日 TDD 教學」雲端任務提示詞已原地更新，重新載入後全文比對一致；每日06:00、Asia/Taipei、原啟用狀態保留，沒有建立替代任務。排程每次須實際讀取本共同主頁與最新 GitHub 來源，依現行項目執行。

實際模型選單目前只列 GPT-6 與 GPT-5.6 Sol，未提供 `gpt-6.1-sol`；原對話推理強度為「中」。因此本次僅確認模型要求已寫入提示詞，實際引擎切換仍受介面可選項目限制，不能宣稱已改為 GPT-6.1 Sol，也不保證後續排程執行結果。

### 7. 91 項決議的適用分類

教學必要規則：來源／規則／SHA、最小完整 TDD、CI 範圍揭露、Notion 發布讀回、逐專案條件推進與封存。可靠性輔助：去重、條件寫入、狀態讀回、單層 pending／有界重試與人工處理、FIFO 補教。歷史第13項「CI 成功即相關功能已驗證」以本版第3節的實際覆蓋範圍解釋，不能泛化為整體驗收。

歷史 37～91 的每次失敗另開補償、獨立多層佇列、cycle、補償彼此補償、事後依賴風險、追蹤完整性風險與復發統計等複雜機制暫緩實作，也不作每日發文 Gate。不是整段刪除驗證原則：證據適用性、缺口揭露、去重、條件寫入、讀回、不可把持久化當驗證、不可偽造完成仍依上文強制保留。既有歷史狀態不刪除、不批次改 resolved／verified。

91 項原文與本次變更前規則保留於 docs/teaching/archive/rules-before-minimum-v1/；第17項原文無編號，依第16與18項間順序對應並註明。封存資料僅供追溯，不再產生下一個衝突討論，也不自動成為執行規則。


## 目前進度（2026-10-22｜main aaa9194 增量驗證）

最新已核驗來源 rust_ai_chat/main `aaa9194a966aefae1ef4724f5d2b576b4289c550`，教學基準 `9716c24f7b3c87a1337f91a82ad10635db598591`。隔離副本170個tracked檔案與Git blob逐位元組一致，產品原始碼與遠端未修改。相較前次a06c038，只改Analytics測試及Persistence workflow；production handler、UI、migration與Cargo／lock未變。

**最新SHA：預設stats-server測試20通過／1ignored；隔離PostgreSQL加 --include-ignored 後31通過、0失敗、0ignored。** Day24的mail／email與兩處SQL引號錯誤已修正並實跑通過。183a058／13a60ed曾因取消ignore而在缺DB URL時失敗；aaa9194恢復ignore後預設測試已通過，不能再把先前的NotPresent失敗寫成最新結果。被略過的PG測試實際讀DATABASE_URL，雖然ignore提示仍保留舊變數名。

最新同SHA Test All九個job均success；Dashboard亦success，Node failure，其他workflow擷取時尚在執行者保留in_progress。Persistence workflow已擴大到全部PG targets，但沒有 --include-ignored，因此仍會略過上述登入／查詢／登出PG測試；若要宣稱CI全部31項執行，需在已配置PG的job加上該旗標或等效命令。本輪只記錄覆蓋範圍，未改來源workflow。

本輪未重跑workspace、fmt／Clippy、WASM、真HTTP、Pet GUI或舊庫升級。前次a06c038的137通過／1ignored不冒稱新SHA整套結果；相關程式與依賴未變的HTTP／UI／DB觀察只保留原SHA與範圍，見[前次報告](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-22-main-a06c038/REPORT.md)。

保留缺口：Chat共用登入／登出／Cookie／Session與API／WS房間ACL；Pet單圖像動畫及此SHA三窗GUI；Analytics角色／來源授權及歷史庫checksum。舊庫migration未改：前次Pet自4f0cd00升級阻擋、自Day24成功；Analytics自4f0cd00成功、自Day24阻擋，均非本輪新實測。

三案Day25（含）以前正文與程式封存，next_day均維持26，last_full_comparison不變。後續補教按各專案既有佇列FIFO，在下一個適用原教學的新Red／完整Green／Refactor後加入，每篇最多一個舊Day單元；不預先指定新題目、不回寫封存原文、不因補教或文件同步增加Day。copilot_use產品未重新驗收，不宣稱完整產品功能通過。

- **Analytics Day24：pending。** fixture已修正並驗證；仍須核對實際歷史migration／checksum，提出並驗證保留資料的升級方案，再完成正式補教發布、全文讀回及狀態同步，才能整筆結案。
- **Chat Day25：pending。** 共用登入／登出／Cookie／Session要求列入Chat後續FIFO補教；不能以房間頁Gate測試通過當作共用實作已完成。
- **Pet Day25：pending。** 單圖像動畫及對應三窗原生GUI驗證列入Pet後續FIFO補教；每窗單一元件、每幀單一圖像與事件同步分別驗證。

[本輪完整驗證與原始證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-22-main-aaa9194/REPORT.md)。依使用者說明排程內容已更新，本輪未再修改、啟停、新增或觸發排程，亦未重新驗證實際模型設定。2026-10-22依使用者要求開啟通知，已實際檢查ChatGPT設定→通知→任務：推播與電子郵件皆已勾選開啟。這是帳戶的「任務」通知分類；查驗時本來即為開啟，本輪無須切換、未變更其他通知分類。此實際介面觀察取代先前依使用者回報的「關閉」快照。未測試真實通知送達，結果仍以任務紀錄、Notion文章與GitHub證據讀回確認。

## 文件入口
產品能力與未來清單、詳細架構、操作設定、功能對照與工程證據仍保留各自用途；本篇集中教學方法、格式及三專案大綱，其他教學入口改為引用。原Day及程式保留，舊規則／來源／失效教法只作歷史，不再套用為目前指令。
[聊天室](https://app.notion.com/p/3f3e51d45093807da638c336e857543a) · [桌面寵物](https://app.notion.com/p/3f3e51d450938067923ae5a424d30950) · [統計儀錶板｜聊天室 × 桌面寵物](https://app.notion.com/p/3f3e51d45093817eafaee0cfc9ea4cc8)。功能與實作對照的詳細歷史保留工程紀錄；本篇不再追加歷次討論或驗證摘要。
## Day索引入口
[聊天室](https://app.notion.com/p/3f3e51d45093807da638c336e857543a)
[統計儀錶板｜聊天室 × 桌面寵物](https://app.notion.com/p/3f3e51d45093817eafaee0cfc9ea4cc8)
[桌面寵物](https://app.notion.com/p/3f3e51d450938067923ae5a424d30950)
[文件與教學封存（非現行規則）](https://app.notion.com/p/3f7e51d4509381faa894d3660c3252f7)
