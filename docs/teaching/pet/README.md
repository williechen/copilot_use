# Pet 首頁

**2026-10-26 功能與測試整理已更新。** [三案整理與12組覆蓋對照](../functional-refactoring/README.md)說明同層重複整併、跨層契約保留及CI責任。Day37不重建、不新增修訂；本次只更新文件，完整新Green與產品測試重構仍待完成，進度不變。

**2026-10-26 現行摘要已核對。** 最新程式來源 `9b89656`（upgrade）；新CI快照為1成功／6執行中，尚未全綠。舊648b903為六成功／Persistence失敗，已配置的Analytics PostgreSQL步驟被略過。當日官方版本、manifest／lock與本機工具差異已補查；未升級產品、未重跑測試、未完成整體驗收。[版本與摘要核對](../../source-review/2026-10-26-versions-and-summary-refresh.md)。

<!-- comparison-5c713553:start -->
[最新三專案程式／文章比對](../../source-review/2026-10-25-full-comparison-5c713553.md)：105個閱讀版本、111份修訂、21項差異；來源 `5c713553`，局部測試與完整驗收分開記錄。
<!-- comparison-5c713553:end -->

[問題文章與後續更正總表](../current/ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

<!-- article-status:start -->
[全部文章現存狀態（105個閱讀版本／111份修訂）](../current/ARTICLE_STATUS.md)：原文、程式所見與歷史／目前驗證分開記錄；已由後續實作取代者勿原樣套回。
<!-- article-status:end -->

<!-- duplicate-review:start -->
[後續調整清單（13 項）](../../FUTURE_WORK.md) · [原始核對報告](../../source-review/2026-10-25-duplicate-application-review.md)。可清除、先遷測試與保留項目分別列明，產品套用／驗收另記。
<!-- duplicate-review:end -->

<!-- daily-delivery:start -->
## 最新交付與下一篇

| 專案 | 最新文章 | 下一篇 | 狀態 |
| --- | --- | --- | --- |
| Chat | [Day38：搜尋失敗顯示在真正的 Leptos 房間列表](../daily/day-38/chat.md) | Day39 | 教學已交付／產品未套用／Red-Green與瀏覽器未執行／未驗收 |
| Pet | [Day37 r3：先保存再開窗，失敗回復原標記](../daily/day-37/pet-r3.md) | Day38 | 程式已見／62項與WASM通過／GUI局部通過／故障注入待驗 |
| Analytics | [Day37 r3：拒絕歧義 Session Cookie，兩種查詢共用 Guard](../daily/day-37/analytics-r3.md) | Day38 | 程式已見／PG45項加略過另1項通過／CI受阻／完整驗收待完成 |

文案修復與最新 PG 結果見 [afdbb6e 驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-assertions-afdbb6e.md)；前次編譯修復見 [bb95a60 驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-compile-bb95a60.md)；前次套用／測試見 [e753c36 更新驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-update-e753c36.md)；原 Day35 正文與交付證據保留，完整產品驗收尚未完成。
<!-- daily-delivery:end -->

2026-10-24 目錄已同步至 [williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)：保留 `chat_apps`、`pet_apps`、`analytics_apps`、`crates` 四個 workspace；[新目錄與套件對照](../../SOURCE_LAYOUT.md)。後續教學依當次來源 SHA 調整程式、Cargo、imports、呼叫端與測試。

> 本頁需求與教學交付保存在 copilot_use；程式入口、套件、測試、CI 與功能驗收只依 rust_ai_ring 精確 SHA，舊庫產品工具不作主體驗收。

[共用文章](../../COMMON_LEARNING_RULES.md) · [三專案首頁](../README.md)

## 目前狀態

[單一目前進度](../current/STATUS.md) · [即時進度檔](../../learning-state.json)。本文的需求不代表已完成；現況依功能與驗證範圍說明。

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

## 功能驗證現況

三個原生生活窗、獨立事件、控制窗關閉再開與profile清單驗證見新報告；舊庫升級與失敗注入仍有缺口。

[完整現況](../current/STATUS.md) · [本輪報告](../history/evidence/2026-10-24-formal-f3f8255/REPORT.md) · [歷史文章與原始證據](../history/README.md)

[Day31教學：控制窗關閉後重開](../daily/day-31/pet.md)（控制窗隱藏／再開已套用並由原生關窗事件實測；完整寵物生命週期仍待補驗。）


## 教學與現存狀態

完整歷史及目前程式／驗證狀態見 [逐篇狀態](../current/ARTICLE_STATUS.md)；最新交付與下一篇仍依頁首及 learning-state。舊文章原文保留當時語境，後續修訂與程式套用另列，不再沿用過時「尚未套用」作為現在的判定。
