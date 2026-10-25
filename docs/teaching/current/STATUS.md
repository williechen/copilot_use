# 目前功能與驗證狀態

程式來源：`williechen/rust_ai_ring/main@afdbb6e81382e8d82095827a90c52cc3309a3e20`。本頁只記功能與驗證現況；文章版本／交付與歷史證據另由 [逐篇狀態](ARTICLE_STATUS.md)、manifest 與 learning-state 管理。copilot_use 為文件庫，產品程式維持唯讀，本次沒有套用產品程式；僅新增下述局部實測，不代表全產品驗收完成。

## 現存程式與缺口

| 專案／功能 | 此精確來源所見 | 驗證與尚缺項目 |
| --- | --- | --- |
| Chat 帳號與建房 | Day35 validate_room_name 已共用於 JSON／個人頁，拒絕控制字元；原 Session／CSRF 與 transport 契約保留 | 本次 application／server／整合共 35 passed，包含新名稱／400／422／DB 不寫入；作者、歷史、逐房 ACL、共用認證與完整瀏覽器仍待驗收 |
| Chat UI | SignInPage 與 GET /login redirect 保留；RoomList 的舊 helper 仍待測試遷移 | 本次 SSR 14 passed；redirect 測試包含在 server 通過項。未重跑 hydration／真瀏覽器，不追認歷史與權限功能完成 |
| Pet 建立與狀態 | 名稱規則共用 pet-application；Day35 set_caged 已移到 pet-persistence 並由 PetStore／Tauri caller 使用；其它 SQL 仍在 server | persistence＋server 單元／整合共 55 passed；舊 global runtime 仍並存，原生三窗／關窗補償注入／重啟與單幀單圖 FIFO 待驗 |
| Pet 預覽與產物 | 唯讀掃描／SHA256／摘要、確認取消與 source bridge 保留；本次未改 UI／bridge | 前次 891ff32 source／新 debug dist 各六契約通過；e753c36 本輪未重建 release／GUI。確認預覽不是 OS 操作成功，真正清理／receipt／restore 仍未接通 |
| Pet 漫遊 | 開關／幾何 guard 存在，runtime 固定找 pet；動態生活窗採 pet-UUID | 真正逐寵物漫遊、拖曳／多螢幕與跨平台仍需接線／驗證，舊固定字串測試不足以證明完成 |
| Analytics 帳密與 UI | ensure_default、正式 password_login／Leptos Dashboard 保留；精確類型查詢已接 analytics-persistence；新增來源 token API | persistence 真 PG 2 passed；HTTP oneshot 證實新 API 無管理者 Cookie、僅來源 token 可讀統計；與業務查詢須人員登入需求有落差。瀏覽器／角色 ACL／登入等待／SSE／服務配對仍待驗收 |
| Analytics 舊碼與範本 | bb95a60 修復五個 E0425；afdbb6e 更新四處舊文案斷言，保留來源／連結／可及性檢查；legacy admin／範本仍保留 | 最新完整 PostgreSQL 回歸 44 passed／0 failed／1 ignored、exit 0；Day27／28／29 全過。查詢授權需求落差、其它功能與 GUI／瀏覽器仍未驗收 |

## 可引用的測試證據範圍

- [afdbb6e 舊文案斷言修復驗證](../../source-review/2026-10-25-assertions-afdbb6e.md)：analytics-server／analytics-persistence 啟用 postgres-integration，所有測試目標與 doc tests 執行結果 44 passed／0 failed／1 ignored、exit 0；三個先前失敗測試已恢復通過。僅本機回歸，不代表新 CI 或全產品功能驗收；查詢授權需求落差保留。

- [bb95a60 前次編譯修復與 PG 回歸](../../source-review/2026-10-25-compile-bb95a60.md)：no-run 編譯 exit 0；no-fail-fast 實跑 41 passed／3 failed／1 ignored；五個 E0425 已消除，三敗均為舊 Dashboard 文案。Day35 真實 Router／資料層測試已通過；來源 token 查詢與人員 Session 的需求落差未解。Chat／Pet 本提交無變更，沿用前次原範圍證據。

- [e753c36 前次增量驗證](../../source-review/2026-10-25-update-e753c36.md)：Chat 35＋UI 14、Pet 55、Analytics persistence 2 通過；Analytics library 兩敗、server PG target 編譯失敗。Test All／Persistence CI 原始日誌已對上相同原因；其它 workflow 完成／進行中狀態見報告 CI 快照。新 API 授權落差亦已用合成事件／真 DB 實測。三案 Day35 已有來源實作，完整功能驗收維持未完成。


- [前次來源重掃與測試日誌](../../source-review/2026-10-25-project-rescan.md)：891ff32；四個 workspace metadata 通過，Chat SSR 14 與 redirect 1 通過，Pet source／新 dist 各六項契約通過且 debug 前端建置成功；Analytics library 23 通過／2 失敗／1 忽略。DB 整合／native GUI 未在本輪完整重跑。
- [歷史來源與 JS 契約證據](../../source-review/2026-10-25-duplicate-application-review.md)：Node 實測來源 ff89918d，source 六項通過；tracked dist 同三個 bridge 契約一過兩敗。該結論僅適用歷史來源與相同 blob；891ff32 已移除舊 tracked dist，不能當最新失敗。整套 Rust／GUI 未在最新來源重跑。
- [九筆歷史切片證據](../history/evidence/2026-10-23-browser-sqlite-9c2b7cf/archive-manifest.json) 限 9c2b7cf 原來源；此次原文雜湊核對相符，沒有新增封存通過。
- [歷史正式冷建置摘要](../history/evidence/2026-10-24-formal-f3f8255/SUMMARY.json) 限 f3f8255，保留當時測試／PG／SSR／GUI／Clippy／升級缺口；不能直接推論最新來源通過。
- [舊來源 CI 補充](../daily/day-34/CI_SUPPLEMENT.md) 限 54f5d80a。**2026-10-25 已核對最新 891ff32 的 GitHub Actions push workflows**：Test All [run 40134969908](https://github.com/williechen/rust_ai_ring/actions/runs/40134969908) 失敗（test-analytics-server / Test analytics server）；Persistence [run 40134969909](https://github.com/williechen/rust_ai_ring/actions/runs/40134969909) 失敗（test-persistence / Test analytics server with PostgreSQL）；Rust AI Chat、Rust Desktop Pet、Rust Analytics、Cargo workspaces、Node 五項 workflow 成功。未取得失敗 step 完整日誌，不能臆測根因；CI 僅限原 SHA，不是未套用 Day35 Green 的驗收。
- 本次有上述 Rust、SQLite／PostgreSQL 與真 HTTP Router 實測；先前 WASM/debug 產物結果限其原 SHA；未完整執行 DB、瀏覽器、Tauri GUI，也不是全產品功能驗收。沒有修改產品、migration checksum 或刪除資料庫。

## 後續調整

[同一份後續調整清單](../../FUTURE_WORK.md) 記錄清除、先遷有效測試再清理與保留條件；[現行流程](../../requirements/USER_FLOWS.md) 與 [UI 條件](../../UI_ALIGNMENT_REQUIREMENTS.md) 仍是功能驗收依據。

[本頁改寫前狀態與歷史原文](https://github.com/williechen/copilot_use/blob/5d66ad95b96a7fffd85856eb80676ddd7120baa6/docs/teaching/current/STATUS.md) 保留其原始來源與時間，不以本次文件整理覆蓋舊驗證。
