# 三專案共用規則

現行分工：每日功能教學與每日漸進重構並行，分開交付、計時及驗收。本文統一教學與工程規則，架構細節見[系統架構](SYSTEM_ARCHITECTURE.md)，實際目錄見[SOURCE_LAYOUT](SOURCE_LAYOUT.md)。

## 1. 儲存庫、來源與進度

- `williechen/copilot_use/main`維護需求、規則、教學、完整工程程式／patch、測試方案與交付證據，不建立第二套產品。
- `williechen/rust_ai_ring/main`是唯一產品／測試／CI來源，保持唯讀；完整變更由使用者套用。方案已交付、程式已套用、產品已驗收分別記錄，不能互相替代。
- 每次重讀本規則、[使用者流程](requirements/USER_FLOWS.md)、[人員驗證](requirements/AUTHENTICATION.md)、[UI條件](UI_ALIGNMENT_REQUIREMENTS.md)、[架構](SYSTEM_ARCHITECTURE.md)、[待辦](FUTURE_WORK.md)、[即時進度](learning-state.json)、[manifest](teaching/current/manifest.json)、現行入口與上一篇；取得兩庫精確SHA，核對四workspace的manifests／lock、正式caller與相關測試。
- 教學及patch以當時真實程式為基準，不能假設尚未套用的重構已存在。發布前重查來源、規則與branch／blob；變動則重核尚未發布內容。必要來源失敗只暫停受影響項，不以快取冒充最新。
- 原固定搬移11個測試、兩庫平行產品與舊P9規則不恢復；P0–P12規劃、原功能／D01–D09及既有FIFO保留，不新增重複清單。

## 2. 每日功能教學

依Chat→Pet→Analytics，各交付一個可操作功能的最小完整切片。先處理使用者流程最早缺口，不以重構、純補測、helper或狀態碼取代功能。基本入口未可用前，不先深化後段功能：

| 專案 | 基本流程優先 |
| --- | --- |
| Chat | 註冊／登入與Session、公開房間清單、授權進房／加入／建房，再深化歷史與進階即時功能 |
| Pet | 具身分／名稱／初始外觀的寵物，正式IPC寫SQLite、生活窗顯示與重啟載入，再深化漫遊等能力 |
| Analytics | 管理員持久化、加鹽密碼雜湊、首次改密碼、Session與後台授權，再深化角色、報表與來源功能 |

沿現有Leptos UI、Axum HTTP／WS或Tauri IPC、用例／Domain、SQLx Adapter的真呼叫鏈交付。Chat維持SSR／hydrate／Router，Pet為CSR＋Tauri，Analytics正式頁面採Leptos，不以完整Axum HTML字串頁面作新UI Green。前台隱藏不等於伺服器授權；三案DB與Session隔離。

文章保留①目標／現況／前置條件、②Red→完整Green與補教、③版本、④驗證／證據／未完成，並在①後設同篇的獨立二級段落「專案Refactor」，與②並列。Refactor不放進Green小節、不作Green接續的局部整理，也不因獨立段落就要求另拆文章；內容按專案整體架構、共用依賴、測試及CI的累積進度安排，不綁當日功能切片。段落內交代專案目標／現況、本次完整調整程式或patch、測試／CI遷移、實際驗證與下一步；大型共用附件可連結，但不能只列規則或連結取代交付。功能Green仍須有完整函式／檔案、Cargo／features、imports、組裝／caller與直接相關測試；切片小整理屬Green說明，不能抵算專案重構。

Red須對應同篇Green；已通過測試只算回歸。預期行為Red、缺新符號編譯Red、環境／匯入錯誤分開。完整Green未準備好只能存草稿；未跑DB／native／GUI不得宣稱功能驗收。

## 3. 專案Refactor的每日接續

[工程入口與進度](teaching/functional-refactoring/README.md)統整三案、共用crates及跨workspace依賴。先讀Chat、Pet、Analytics各自工程進度，再從專案整體依賴選擇每天一個完整小工作包，接續前日未完成處；不因教學換題重設重構範圍。跨案包共用同一份證據，逐案列受影響邊界，不重複計功；無工程推進的專案如實列續辦／受阻與原因；不作當日功能教學主題，不增加Day。同篇專案Refactor獨立段落呈現本次完整內容，工程入口統整跨日進度與共用附件。產品保持唯讀，完整程式／patch與測試方案由使用者套用。

工作包須有來源SHA、範圍／相依、完整程式與Port／DTO／錯誤、Adapter／組裝／正式caller、Cargo及必要相容處理；同步測試／fixture與CI，列實際／未執行驗證、風險、回復方式和下一步。以最新已套用程式作下一包基準；相依patch標清順序，不能拿未套用方案冒充主分支狀態。

測試與CI責任集中如下，不再逐篇重複一套規則：

- 建立舊測試→保留／合併／參數化／遷移／確認淘汰→新斷言與執行入口的映射。同層同契約且無額外價值才整併；不同邊界值、故障模式與跨層契約保留。
- 有效斷言先接到真用例／Adapter／Router／IPC／元件，再移除舊helper與獨占測試；不刪斷言換全綠，不以測試數升降當覆蓋完成。fixture只共用穩定準備，不複製業務實作或令純內層依賴server。
- 純Domain／Application、各案真DB、正式HTTP／IPC、Pet原生與UI驗證分責任；三案無實際相依不串接，matrix避免取消無關案。保留server／PG feature／整合測試，不以continue-on-error吞失敗；彙總檢核需反映failure、cancelled與非預期skipped。
- 拆CI時同步job名稱、package／features、services／native依賴、trigger與required checks。補系統套件不是分層完成，單選library不代表依賴crate的測試已執行。

## 4. 版本與驗證證據

每篇涉及的套件、Rust、框架、DB及CLI／native工具均核對交付當日官方最新穩定版，排除預發行／yanked；列官方來源、日期、manifest宣告、lock resolved或已安裝版、差異、MSRV／native／WASM／DB相容性及建議採用理由。最新穩定版優先，不相容須明列阻礙與遷移驗證；未查完整不得通過文件Gate。不能由版本metadata推定build／DB／GUI通過，也不能假稱已升級；不自動改lock、安裝或跑產品migration。

程式測試及CI僅認rust_ai_ring精確SHA、workflow／job、覆蓋與時間；候選patch的語法／套用檢查另標，不當產品測試。copilot_use的CI不代替產品驗收。未套用、未執行、失敗、ignored與部分通過逐項如實列出。

## 5. GitHub交付、修訂與同步

**Gate1是文件交付：** 完整可套用內容、真實路徑／caller、版本與驗證設計齊備，每案文章獨立commit到main，從精確commit全文讀回，保存blob／SHA256、來源／規則SHA及時間。只有新Day N完成證據後，才以當時branch／blob及expected next_day=N條件同步N→N+1；已N+1且證據一致不重複加一。

每篇交付後同步七首頁（根、docs、teaching、daily及三案首頁）、對應ENTRY、manifest／learning-state；先讀最新內容再條件提交，精確commit讀回並驗證連結。STATUS只列功能／實作／驗證，不把發布當產品驗收。同步失敗記錄實際路徑與原始錯誤，標「文章已交付／索引未同步」，繼續其餘案，不強制覆寫。

**Gate2是產品驗收：** 使用者套用後，以精確產品SHA核對測試／CI／DB／HTTP／IPC／GUI，與文件發布分開。原CI成功不能證明未套用Green成功，CI失敗／pending本身也不阻止合格文件發布；實作、封存與功能驗收依真證據判定。

保留已發布正文及原始commit／blob／SHA256。需要更正時用版本化修訂與獨立讀回，保存原版，不新增或倒退Day。Day25（含）以前已封存原文不回寫；未通過產品條件不冒稱已封存／驗收。現況頁正文以功能和證據為主，Day與交付版本留在教學索引及狀態檔。

每次寫入記錄repo／branch／path、預期parent、實際commit和原始錯誤。每案Day／revision保留單一待續紀錄、先去重再重試；連續三次相同失敗須人工檢視，不能空報成功。

## 6. 補教、計時與全面比對

- 上一Day未完成事項於下一篇新Red／Green後補充；更早makeup_queue依同案FIFO，每篇最多結案一個舊Day單元。補教、修訂、工程與封存均不增加Day；全文交付不等於產品驗收。
- **核心60分鐘只計第②節主要Red／Green**：三案各15分鐘，加彈性15分鐘。局部Refactor、全專案工程、補教、版本／來源核對、①③④節、提交／讀回／同步及產品驗收一律另計。不得因時限省略完整Green，超額與受阻如實記錄。
- 每日作當次功能／工程的增量核對；每週至少一次全專案重新比對，包含產品、需求、架構、UI、DB、測試／CI及文件對應。只在實際完整成功後更新last_full_comparison；日常工程或文章整理不更新，也不因週比對推進Day。
- 排程指令見[現行文件與雲端核對](teaching/scheduler-instructions-v1.md)，不從舊時間文字推定實際任務設定。學習階段按[P0–P12規劃](ROADMAP_P0_P12.md)追蹤，日曆、學習完成與產品驗收分開。
- 不讀正式秘密或執行付費AI；產品驗收不可冒稱重跑，舊DB需保資料相容驗證，不能以fresh DB代替升級證據。
