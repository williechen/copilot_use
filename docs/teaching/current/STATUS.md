# 目前功能與驗證狀態

程式來源：`williechen/rust_ai_ring/main@afdbb6e81382e8d82095827a90c52cc3309a3e20`。本頁只記功能與驗證現況；文章版本／交付與歷史證據另由 [逐篇狀態](ARTICLE_STATUS.md)、manifest 與 learning-state 管理。copilot_use 為文件庫，產品程式維持唯讀，本次沒有套用產品程式；僅新增下述局部實測，不代表全產品驗收完成。

## 現存程式與缺口

| 專案／功能 | 此精確來源所見 | 驗證與尚缺項目 |
| --- | --- | --- |
| Chat 帳號與建房 | Day35 validate_room_name 已共用於 JSON／個人頁，拒絕控制字元；原 Session／CSRF 與 transport 契約保留 | 本次 application／server／整合共 35 passed，包含新名稱／400／422／DB 不寫入；作者、歷史、逐房 ACL、共用認證與完整瀏覽器仍待驗收 |
| Chat UI | SignInPage 與 GET /login redirect 保留；RoomList 的舊 helper 仍待測試遷移 | 本次 SSR 14 passed；redirect 測試包含在 server 通過項。未重跑 hydration／真瀏覽器，不追認歷史與權限功能完成 |
| Pet 建立與狀態 | 名稱規則共用 pet-application；Day35 set_caged 已移到 pet-persistence 並由 PetStore／Tauri caller 使用；其它 SQL 仍在 server | persistence＋server 單元／整合共 55 passed；舊 global runtime 仍並存，原生三窗／關窗補償注入／重啟與單幀單圖 FIFO 待驗 |
| Pet 預覽與產物 | 唯讀預覽／摘要／source bridge 保留；舊產物已清，本 SHA CI release 前端／native build／上傳 steps 成功 | 新 dist 契約尚未接 CI，兩個新 persistence library 的自身單元測試也未直接被 CI 選取；實際 binary／GUI、真正清理／receipt／restore 仍待驗 |
| Pet 漫遊 | roam.rs 只查固定 pet；實際生活窗採 pet-UUID。startup 只依 active slot=1 開一窗 | 逐 ID 漫遊／所有未回籠寵物重啟／三窗互動仍有接線與驗收缺口；固定 label 單測或資料層旗標成功不能代替 |
| Analytics 帳密與 UI | ensure_default、正式 password_login／Leptos Dashboard 保留；精確類型查詢已接 analytics-persistence；新增來源 token API | persistence 真 PG 2 passed；HTTP oneshot 證實新 API 無管理者 Cookie、僅來源 token 可讀統計；與業務查詢須人員登入需求有落差。瀏覽器／角色 ACL／登入等待／SSE／服務配對仍待驗收 |
| Analytics 舊碼與範本 | bb95a60 修復五個 E0425；afdbb6e 更新四處舊文案斷言，保留來源／連結／可及性檢查；legacy admin／範本仍保留 | 最新完整 PostgreSQL 回歸 44 passed／0 failed／1 ignored、exit 0；Day27／28／29 全過。查詢授權需求落差、其它功能與 GUI／瀏覽器仍未驗收 |

## 可引用的測試證據範圍

- [本次 13 項重掃與 CI 快照](../../source-review/2026-10-25-adjustments-rescan-afdbb6e.md)：本 SHA 的七個 push workflows 均 success，包含 Test All／Persistence／Pet release build。仍缺 persistence libraries 自身測試的 CI package 選取、新 dist 契約與 GUI／需求驗收；CI 全綠不代表功能全部符合需求。
- [同 SHA 的本機 PostgreSQL 回歸](../../source-review/2026-10-25-assertions-afdbb6e.md)：44 passed／0 failed／1 ignored；編譯與舊文案失敗均已修復。本輪來源未變，沒有重複執行相同測試。人員查詢授權落差保留。
- [e753c36 增量測試與 Router 實證](../../source-review/2026-10-25-update-e753c36.md)：Chat 35＋UI 14、Pet 55 原範圍結果可引用，本輪 git diff 證明兩案全目錄與本 SHA 無差異；原 Analytics 失敗僅屬歷史，已由後續修正。HTTP probe 的授權實作未再變更。
- [891ff32 前次重掃／debug 產物契約](../../source-review/2026-10-25-project-rescan.md) 與 [更早 JS 差異報告](../../source-review/2026-10-25-duplicate-application-review.md) 保留各自輸入及來源；舊 tracked dist 兩敗不再當目前失敗。沒有宣稱原 debug 產物就是本次安裝 binary。
- [歷史正式冷建置摘要](../history/evidence/2026-10-24-formal-f3f8255/SUMMARY.json)、[九筆歷史切片證據](../history/evidence/2026-10-23-browser-sqlite-9c2b7cf/archive-manifest.json) 與 [舊 CI 補充](../daily/day-34/CI_SUPPLEMENT.md) 都限其原 SHA，不能當成最新完成證據。
- 本輪只重新讀來源、盤點追蹤檔與核對 CI；沒有產品修改、新 runtime 測試、GUI／瀏覽器或全產品功能驗收。既有測試證據、文章／進度與歷史雜湊保留。

## 後續調整

[同一份後續調整清單](../../FUTURE_WORK.md) 記錄清除、先遷有效測試再清理與保留條件；[現行流程](../../requirements/USER_FLOWS.md) 與 [UI 條件](../../UI_ALIGNMENT_REQUIREMENTS.md) 仍是功能驗收依據。

[本頁改寫前狀態與歷史原文](https://github.com/williechen/copilot_use/blob/5d66ad95b96a7fffd85856eb80676ddd7120baa6/docs/teaching/current/STATUS.md) 保留其原始來源與時間，不以本次文件整理覆蓋舊驗證。
