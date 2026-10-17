# Pet 控制介面與 Analytics Leptos 畫面銜接

本文件是既有 FUTURE_WORK 項目的詳細驗收條件，不另立 Day 或開發清單。狀態：需求已統整，程式未實作，新增驗證未執行。

## 來源與 Day15 封存

2026-10-17 已唯讀核對最新 main `8bbf884ed73038adbe9f7f51030bbb676ae4789f`（Phase2 day15 red）：三批舊 tests 已搬移，Chat 新增 list_rooms Red 測試。此為 source 套用證據，未重新執行測試／CI，不能判定完整 Green 或功能驗收。依使用者指示，三篇 Day15 原文與鏡像保持封存，不重建、不回填；畫面與教法修改留在專案缺口，下一 Day 才提出完整方案，next_day 保持 16。

精確增量：Chat Red 測試加入 apps/server/src/http.rs；測試搬移到 apps/server/tests/root_hub_contract.rs、crates/pet_domain/tests/cleanup_contract.rs、apps/stats_server/tests/idempotency_contract.rs，原 target 自 tests/Cargo.toml 移除。Git compare 顯示三份搬移內容不變。原教學的 room_hub_contract.rs 與實際 root_hub_contract.rs 命名差異保留於來源紀錄，後續命令以實際路徑為準。只有 Red／測試搬移不能宣稱三篇完整功能已套用。

## Desktop Pet

#### 控制介面調整：後續銜接需求，未套用
寵物視窗與控制視窗分工：寵物視窗保留 200×200 等比動畫、拖曳與右鍵入口；控制視窗使用 Leptos，可調整大小並可捲動，不把掃描結果、設定表單與所有按鈕堆進寵物小視窗。重複開啟只顯示／聚焦既有控制視窗，關閉控制視窗不停止寵物，不建立新寵物。
控制視窗分為六個功能區：
- 互動：狀態、互動／結束／睡覺／醒來、已有聊天入口；漫遊／速度只在有對應 runtime 與保存能力時開放。
- 設定：五欄與選用外觀；必須能讀回、保存及重啟保留，缺後端時明示未接通。
- 記憶：列表、查詢與管理接既有資料服務；尚無對應 API 時只顯示能力狀態，不放無作用的操作按鈕。
- 檔案：授權狀態 → 掃描／SHA256 → 結果 → 確認／取消 → 操作結果／收據 → 還原。只呈現可用 IPC；預覽確認與真實操作分開，未有操作 IPC 時明示未接通。
- 管理：多寵物、備份／還原及封存；依實作能力逐步接通。
- 更新：版本與啟動／手動檢查，依安裝與簽章更新能力接線。
視窗間共用後端寵物狀態，由既有 snapshot／事件同步；分頁切換不新增寵物、不自行掃描或觸發檔案動作。掃描、確認、收據與錯誤保留在對應功能區，長結果可換行／捲動且操作按鈕可到達。
驗收包含：控制視窗縮至可用小尺寸及放大、系統縮放、鍵盤焦點與捲動、長清單／錯誤文案、六區切換、反覆開啟不重複建立、關閉後寵物仍運作；專用目錄實測 UI → bridge → Tauri → OS／資料層 → 回應 → 畫面，無後端的功能不得假報成功。上述新介面尚未實作或實測；後續 Day 才提供完整程式，Day15 不修改。

來源畫面診斷：3e58e650 的 tauri.conf.json 是固定 320×360、resizable=false；style.css 對 html／body 設 overflow:hidden；所有互動與掃描結果集中 frontend/src/main.rs::App。比較至 8bbf884e，這些檔案未變。此為靜態裁切風險診斷，未冒稱本輪重現 GUI。產品端 copilot_use 已有 main／pet 視窗及控制分頁，其既有能力作適配參考，不將本文件解讀為產品必須重建相同功能。

## Product Analytics

#### Leptos Dashboard：後續更正方向，未套用
Analytics 畫面沿用 Leptos 與 Rust 元件，不以獨立手寫 HTML／JavaScript 維護另一套互動介面。先核對現有 UI crate／SSR／hydration 的責任與建置入口再接線，不為三專案檔名一致新建空骨架。
基本元件為來源選擇、事件類型篩選、查詢／重新整理、載入狀態、空資料、錯誤／重試與結果表格；由 Leptos 狀態與非同步請求呼叫現有 HTTP JSON Query API，保留來源授權與 PostgreSQL Adapter。篩選切換時清除過時結果，取消或辨識舊請求，避免晚回應覆蓋新結果。SSE 於基本查詢接通後逐步補齊。
UI 不持有 SQLx／PgPool，不另建平行 total API 或資料來源；正式管理者 Session／唯讀查詢權限與服務憑證分開，不把後端來源密鑰嵌入 WASM／HTML。來源 token 的既有學習契約保留為過渡事實，不當成正式 Dashboard 授權模型。
Day15 的 HTML／JavaScript 提案保留歷史；改採 Leptos 的完整變更與原因，在下一 Day 的 Refactor 後「舊 Day 更正補充」提出，不回填 Day15。這次只記錄方向與缺口，不新增 Day 或宣稱已接通。
驗證須分別包含 native SSR（若採用）、WASM hydration／CSR、元件狀態、HTTP／來源隔離、真實 PostgreSQL 結果與瀏覽器流程；既有 API 或 CI 成功不能代替 Leptos 畫面驗收。本輪未執行上述新增驗證。


## 驗證與交付界線

只更新共同規則、專案入口、原功能缺口及 learning-state 的當前來源觀察；不改 Day15 原文／鏡像，不新增 Day16，不增加 next_day，不改 rust_ai_chat 原始碼。Day15 文內的未套用標示是發布當時快照，後續套用狀態由入口及 learning-state.latest_source_observation 記錄。
