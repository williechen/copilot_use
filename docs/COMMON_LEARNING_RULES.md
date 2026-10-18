# Rust AI 三專案：教學總綱與實作對照

> 現行規則正本：[Notion 共同總頁](https://app.notion.com/p/3f2e51d4509381ddb8a8ce7cb4cd3e60)。本文件為 GitHub 鏡像。

## 每日 TDD 教學最小必要規則 v1

規則版本：daily-tdd-minimum-v1／2026-10-18。這是正式規則變更，取代舊版每日流程；產品需求、安全邊界、既有 Day 與證據仍保留。Notion 共同總頁是現行規則正本，GitHub `COMMON_LEARNING_RULES.md` 為鏡像；不以舊對話或歷史決議覆蓋本版。規則／索引衝突須揭露並核對，不能默認已同步。

### 1. 來源與選題

每次實際重讀 copilot_use 的 `docs/COMMON_LEARNING_RULES.md`、`docs/learning-state.json`、`docs/P9_TARGET.md`、`docs/FUTURE_WORK.md`、`docs/SYSTEM_ARCHITECTURE.md`、`docs/UI_ALIGNMENT_REQUIREMENTS.md`、`docs/teaching/current/manifest.json`，以及 Notion 共同總頁、三專案入口與相關 Day。取得 rust_ai_chat/main 與 copilot_use 教學分支最新完整 SHA，核對本日相關程式、實際呼叫端、測試及差異。必要來源讀取失敗，只暫停受影響專案，不能以快取冒充最新。

發布每篇前再取 main SHA 及必要規則版本；來源 SHA 改變就重建所有尚未發布草稿，規則改變則重新核對、必要時重建。已完整發布讀回的 Day 不自動撤銷。記錄規則版本、Notion page_last_edited_at、GitHub 規則 blob／commit SHA、兩庫來源 SHA 及讀取時間。rust_ai_chat 全庫唯讀，教學程式由使用者套用；copilot_use 產品整合與教學分開。

### 2. 文章最小格式

沿用四節：①學習大綱＝目標、現況、前置條件與本日差異；②實作內容＝Red → 完整最小 Green → Refactor → 必要的舊 Day 更正補充；③套件版本＝本日涉及的 Cargo／lock、stable／MSRV／native／WASM 核對，未查明示；④驗證項目＝測試、證據、待完成與下一步。

每篇只教一個最小完整切片，提供真實檔案位置、必要完整修改與繁體中文說明，交代入口到資料／畫面的呼叫鏈。Red 必須對應同篇 Green，預期行為失敗、缺少待新增符號的預期編譯失敗、匯入／環境錯誤分別標示；已通過的既有測試不能冒稱 Red。不能以未確認的建窗標籤或介面假設冒稱完整可套用 Green；必要契約未核實先補齊，否則只存草稿。

大功能可跨日；原切片未完成如實保留，不把 API、欄位或純補測當完整新功能。已實作 Day 封存，未實作部分可在原頁修訂；狀態不明先列未核實。重大錯誤由人工逐案裁定影響範圍及是否版本化重發，保留原文與版本；更正不自動新增 Day 或倒退進度。

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

### 7. 91 項決議的適用分類

教學必要規則：來源／規則／SHA、最小完整 TDD、CI 範圍揭露、Notion 發布讀回、逐專案條件推進與封存。可靠性輔助：去重、條件寫入、狀態讀回、單層 pending／有界重試與人工處理、FIFO 補教。歷史第13項「CI 成功即相關功能已驗證」以本版第3節的實際覆蓋範圍解釋，不能泛化為整體驗收。

歷史 37～91 的每次失敗另開補償、獨立多層佇列、cycle、補償彼此補償、事後依賴風險、追蹤完整性風險與復發統計等複雜機制暫緩實作，也不作每日發文 Gate。不是整段刪除驗證原則：證據適用性、缺口揭露、去重、條件寫入、讀回、不可把持久化當驗證、不可偽造完成仍依上文強制保留。既有歷史狀態不刪除、不批次改 resolved／verified。

91 項原文與本次變更前規則保留於 docs/teaching/archive/rules-before-minimum-v1/；第17項原文無編號，依第16與18項間順序對應並註明。封存資料僅供追溯，不再產生下一個衝突討論，也不自動成為執行規則。


## 目前進度（最新程式再次核對）
最新 main `2eddd6428cda6c81c53b1e261d0846ebcf56e6a0`。既有 Day18 已發布，next_day 各19，本次未遞增；正式完整驗收維持未通過。
## 當前功能狀態與證據入口
- chat：新 migration 20261018100114 使用 SQLite 不支援的 MODIFY COLUMN。SQLx 新建與從1853876升級均失敗；清除 persistence 套件後 0 passed／5 failed。Chat 啟動亦受 migration 阻擋，尚不能驗收房間功能。搜尋三筆只顯示兩筆與 SSR 提示失敗仍是未修的1853876基線；本輪沒有宣稱最新服務已可供瀏覽器驗證。
- pet：本輪新增 migration 嘗試把 owner_id 對齊 pet_owner，但開頭 MODIFY COLUMN 在實際 SQLite 執行失敗，尚未到欄位／外鍵修正；pet_owner 實際欄位仍為 id，不存在 pet_owner_id。先前 missing owner／SERIAL NULL 不能宣告解除。漫遊核心、設定與依賴無變，1853876的四案原生幾何證據仍適用該核心；本輪未重跑完整 GUI。
- analytics：本輪只更動 root SQLite migration，Analytics PG程式及其migrations／lock未變；先前 Cookie 未續期、PG migration31→32既有庫升級失敗仍未修。新 SQLite migration 本身又在 MODIFY 語法失敗，不能作帳密／角色或資料管理功能已完成的證據。本輪未重跑相同 HTTP／PG 流程，沿用前輪時明確保留1853876來源。
[最新實測與原始錯誤](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-2eddd64/REPORT.md)。候選80ba01e以1853876為基底，沒有包含本次 migration，不是最新 main 的 Green；需先修遷移並重跑。v1正文、原需求與歷史證據保留。
SQLite 修正候選 `e33971f508e9612770ad72960b62a8b090efd53e`（工作分支，未合併main）：以重建表取代不支援的ALTER，保留有效資料／ID，owner_id對齊pet_owner_id並強制外鍵。新建／有效資料升級／缺少owner ID／孤兒owner四案通過，後兩者須拒絕並rollback。最新main仍失敗；此候選只修SQLite migration，未包含之前Chat搜尋與Analytics Cookie候選。完整SQL與重現命令見 [修正與證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-2eddd64/REPORT.md)。
## 文件入口
產品能力與未來清單、詳細架構、操作設定、功能對照與工程證據仍保留各自用途；本篇集中教學方法、格式及三專案大綱，其他教學入口改為引用。原Day及程式保留，舊規則／來源／失效教法只作歷史，不再套用為目前指令。
[聊天室](https://app.notion.com/p/3f3e51d45093807da638c336e857543a) · [桌面寵物](https://app.notion.com/p/3f3e51d450938067923ae5a424d30950) · [統計儀錶板｜聊天室 × 桌面寵物](https://app.notion.com/p/3f3e51d45093817eafaee0cfc9ea4cc8)。功能與實作對照的詳細歷史保留工程紀錄；本篇不再追加歷次討論或驗證摘要。
## Day索引入口
[聊天室](https://app.notion.com/p/3f3e51d45093807da638c336e857543a)
[統計儀錶板｜聊天室 × 桌面寵物](https://app.notion.com/p/3f3e51d45093817eafaee0cfc9ea4cc8)
[桌面寵物](https://app.notion.com/p/3f3e51d450938067923ae5a424d30950)
[文件與教學封存（非現行規則）](https://app.notion.com/p/3f7e51d4509381faa894d3660c3252f7)
