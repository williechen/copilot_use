# Pet 首頁

每日文章另設獨立 Refactor 段落，交付該篇專案的實際重構程式、測試承接與驗證；全專案重構另列獨立項目。核心60分鐘只計Red／Green；rust_ai_ring唯讀，使用者自行套用。

[共用規則](../../COMMON_LEARNING_RULES.md) · [功能與驗證](../current/STATUS.md) · [工程進度](../functional-refactoring/PROGRESS.md) · [學習進度](../../learning-state.json) · [manifest](../current/manifest.json)

| 專案 | 現行教學 | 下一篇 | 驗證範圍 |
| --- | --- | --- | --- |
| Chat | [Day39 r2：首頁與房間側欄首次進入自動載入公開房間](../daily/day-39/chat-r2.md) | Day40 | Refactor SSR 14項通過；功能Green／GUI／候選CI未驗 |
| Pet | [Day39 r2：控制窗逐 UUID 找回寵物到主螢幕可用區](../daily/day-39/pet-r2.md) | Day40 | Refactor Application 8項＋SQLite 4項通過；Tauri／功能Green／GUI未驗 |
| Analytics | [Day39 r2：管理後台依 Session 匯出指定來源統計 CSV](../daily/day-39/analytics-r2.md) | Day40 | Refactor Server／PG 49項通過、1項ignored；CSV Green／GUI／候選CI未驗 |

## 專案大綱與操作主線

建立可啟動、互動與持久保存設定的桌面寵物，再逐步提供受控的檔案整理、學習記憶及 AI 協作。

基本主線：啟動寵物 → 即時調整互動／設定 → 保存設定與記憶 → 授權目錄 → 掃描／SHA256 預覽 → 確認或取消 → 受控操作 → 成功收據／還原 → Analytics 紀錄。各段分別實作與驗證；確認預覽與實際檔案操作分別判定。

## 現行需求

[三專案使用者流程](../../requirements/USER_FLOWS.md) · [共用人員驗證](../../requirements/AUTHENTICATION.md) · [UI 詳細驗收](../../UI_ALIGNMENT_REQUIREMENTS.md)。共用條款統一維護，不在各首頁重複。

## 功能目標與完成條件

- 寵物與控制：五欄設定（名字、種類、喜好、個性、興趣），選用外觀覆寫；每隻寵物的資料與素材獨立。設定能即時生效並在重啟後保留。
- 顯示與移動：200×200 範圍內等比動畫，手動拖曳與明確啟用的自動漫遊，多寵物逐步加入；移動偏好不能因重開控制面板而意外變更。
- 安全整理：只處理授權 root，前台使用 opaque ID；確認綁定目前掃描結果，操作前重驗權限／symlink／檔案變更，並有 journal、隔離收據與還原。預覽、確認及 Domain 成功狀態都不能代替真實 OS 結果。
- 後續能力：記憶／回想／複習／提醒、RAG／MCP／Agent、備份／還原、安裝包與簽章更新；更新由啟動或手動檢查觸發。

## 介面與接線要求

#### 控制介面目標：雙窗骨架已套用，其餘能力分段驗收

一個後台控制窗管理多隻寵物，各出籠寵物有獨立原生生活窗，不採放大同一張混合頁面的方式。前台寵物保留 200×200 等比動畫、拖曳、漫遊與右鍵入口；後台控制視窗承載六區功能並可縮放／捲動。關閉控制視窗後寵物繼續運作，反覆開啟只聚焦既有視窗，透過後端 snapshot／事件同步。以下為完整功能要求，仍需分段驗收。

寵物視窗與控制視窗分工：寵物視窗保留 200×200 等比動畫、拖曳與右鍵入口；控制視窗使用 Leptos，可調整大小並可捲動，不把掃描結果、設定表單與所有按鈕堆進寵物小視窗。重複開啟只顯示／聚焦既有控制視窗，關閉控制視窗不停止寵物，不建立新寵物。

控制視窗分為六個功能區：

- 互動：狀態、互動／結束／睡覺／醒來、已有聊天入口；漫遊逐隻保存；右鍵「坐下」關閉漫遊；動作速度依種類習性，不提供固定速度設定。
- 設定：五欄與選用外觀；必須能讀回、保存及重啟保留，缺後端時明示未接通。
- 記憶：列表、查詢與管理接既有資料服務；尚無對應 API 時只顯示能力狀態，不放無作用的操作按鈕。
- 檔案：授權狀態 → 掃描／SHA256 → 結果 → 確認／取消 → 操作結果／收據 → 還原。只呈現可用 IPC；預覽確認與真實操作分開，未有操作 IPC 時明示未接通。
- 管理：多寵物、備份／還原及封存；依實作能力逐步接通。
- 更新：版本與啟動／手動檢查，依安裝與簽章更新能力接線。

控制窗依寵物識別同步各生活窗的獨立後端狀態；分頁切換不新增寵物、不自行掃描或觸發檔案動作。掃描、確認、收據與錯誤保留在對應功能區，長結果可換行／捲動且操作按鈕可到達。

驗收包含：控制視窗縮至可用小尺寸及放大、系統縮放、鍵盤焦點與捲動、長清單／錯誤文案、六區切換、反覆開啟不重複建立、關閉後寵物仍運作；專用目錄實測 UI → bridge → Tauri → OS／資料層 → 回應 → 畫面，無後端的功能不得假報成功。雙窗骨架已有實作；上述完整 GUI／IPC／檔案流程仍需後續按功能分段驗收。

## 驗證範圍

已核驗來源與測試範圍統一見GitHub [目前進度](../current/STATUS.md)。完整功能驗收仍須由真實入口逐段測試；單元測試、資料寫入或建置通過不能代替使用者流程驗收。

## 開發位置

`rust_ai_ring/pet_apps/pet_server`（package `pet-server`）負責 Tauri／OS 入口，`pet_ui` 負責 Leptos 畫面與 bridge，`pet_domain` 負責純規則，migration 位於 `pet_apps/migrations`。`pet_application` 已承接建立名稱規則並由 Tauri／PetStore 呼叫；`pet_persistence` 仍未承接 SQLx Adapter。其餘分層依最新 caller 與測試逐切片調整。copilot_use 舊 pet_runtime 僅供歷史追溯。

本機 IPC 可用於測試實際能力，使用全新測試資料與授權目錄；本入口不要求重跑付費 AI。實作與驗證步驟看對應功能教學，不將專用測試收據當成私人檔案已處理。

[產品操作設定](https://github.com/williechen/copilot_use/blob/main/docs/DEPLOYMENT.md) · [產品未來清單](https://github.com/williechen/copilot_use/blob/main/docs/FUTURE_WORK.md)


[現行文章入口](../current/PET_ENTRY.md) · [歷史及修訂總表](../current/ARTICLE_STATUS.md)。實作與驗收僅在現況頁集中維護。

[全專案重構獨立項目](../../../docs/engineering/README.md) · [候選本地驗證](../../../docs/teaching/daily/day-39/r2-refactor-validation.json)。

> Day39 三案 Refactor 只計各案；全庫 G-01／G-02 保留在 [工程首頁](../../engineering/README.md)，未重複計功。正文已補完整重構程式與測試；隔離Refactor驗證通過，產品未套用，功能Green與GUI仍待驗。
