# 目前功能與驗證狀態

程式來源：`williechen/rust_ai_ring/main@891ff32ae29e1588dc40027cd5c9c99ca91d2378`。本頁只記功能與驗證現況；文章版本／交付與歷史證據另由 [逐篇狀態](ARTICLE_STATUS.md)、manifest 與 learning-state 管理。copilot_use 為文件庫，產品程式維持唯讀，本次沒有套用產品程式；僅新增下述局部實測，不代表全產品驗收完成。

## 現存程式與缺口

| 專案／功能 | 此精確來源所見 | 驗證與尚缺項目 |
| --- | --- | --- |
| Chat 帳號與建房 | 註冊、帳密／Session、完成頁、個人頁建房、公開清單與會員／WS 守衛已見 caller | 歷史固定來源有 SQLite／HTTP／瀏覽器證據；本次未重跑。選房導向、作者、歷史查詢、逐房 ACL、長連線撤銷與共用認證仍有缺口 |
| Chat UI | SignInPage、房間狀態／多行輸入及歷史未接通提示已見；LoginView 已移除，GET /login 導向 /signin | 本次 chat-ui SSR 14 passed、GET /login redirect 1 passed；完整登入／hydration／瀏覽器未重跑；原文 SSR API／過時測試與刷新 helper 仍需銜接，不能把提示當歷史查詢完成 |
| Pet 建立與狀態 | 名稱規則已由 pet-application 共用；PetStore、建立／選取／逐 ID Runtime、控制窗 hide/show 已見 | pet-persistence 未承接 SQLx；舊 global runtime 仍並存。原生三窗、重啟、失敗補償等未在當次重跑 |
| Pet 預覽與產物 | 唯讀掃描／SHA256／摘要、確認取消與 source bridge 已見 | 舊 tracked dist／public／生成 schema 已移除；source 六項 Node 契約通過、Trunk debug 建置成功、新 dist 同六項契約通過；release／native GUI 尚待驗證。確認預覽不是 OS 檔案操作成功，真正清理／receipt／restore 仍未接通 |
| Pet 漫遊 | 開關／幾何 guard 存在，runtime 固定找 pet；動態生活窗採 pet-UUID | 真正逐寵物漫遊、拖曳／多螢幕與跨平台仍需接線／驗證，舊固定字串測試不足以證明完成 |
| Analytics 帳密與 UI | startup／CLI 已共用 ensure_default；正式 password_login、Leptos 登入／改密碼／Dashboard／事件篩選已見 | PostgreSQL／HTTP／瀏覽器未在此次重跑；角色／來源 ACL、登入等待、SSE、明細與服務配對仍有缺口 |
| Analytics 舊碼與範本 | legacy admin 不在 main 路徑；initialize 仍被整合測試 fixture 使用；新 library 為範本或空庫 | 舊限流／續期與 Bearer query helper 的成功不屬目前正式人員路由。本次 library 23 passed／2 failed／1 ignored，兩敗為 Day28／29 舊文案斷言；Day27 DB 測試另有同樣舊文案、尚未重跑；修正斷言時保留原回歸條件 |

## 可引用的測試證據範圍

- [最新來源重掃與測試日誌](../../source-review/2026-10-25-project-rescan.md)：891ff32；四個 workspace metadata 通過，Chat SSR 14 與 redirect 1 通過，Pet source／新 dist 各六項契約通過且 debug 前端建置成功；Analytics library 23 通過／2 失敗／1 忽略。DB 整合／native GUI 未在本輪完整重跑。
- [歷史來源與 JS 契約證據](../../source-review/2026-10-25-duplicate-application-review.md)：Node 實測來源 ff89918d，source 六項通過；tracked dist 同三個 bridge 契約一過兩敗。該結論僅適用歷史來源與相同 blob；891ff32 已移除舊 tracked dist，不能當最新失敗。整套 Rust／GUI 未在最新來源重跑。
- [九筆歷史切片證據](../history/evidence/2026-10-23-browser-sqlite-9c2b7cf/archive-manifest.json) 限 9c2b7cf 原來源；此次原文雜湊核對相符，沒有新增封存通過。
- [歷史正式冷建置摘要](../history/evidence/2026-10-24-formal-f3f8255/SUMMARY.json) 限 f3f8255，保留當時測試／PG／SSR／GUI／Clippy／升級缺口；不能直接推論最新來源通過。
- [舊來源 CI 補充](../daily/day-34/CI_SUPPLEMENT.md) 限 54f5d80a。**2026-10-25 已核對最新 891ff32 的 GitHub Actions push workflows**：Test All [run 40134969908](https://github.com/williechen/rust_ai_ring/actions/runs/40134969908) 失敗（test-analytics-server / Test analytics server）；Persistence [run 40134969909](https://github.com/williechen/rust_ai_ring/actions/runs/40134969909) 失敗（test-persistence / Test analytics server with PostgreSQL）；Rust AI Chat、Rust Desktop Pet、Rust Analytics、Cargo workspaces、Node 五項 workflow 成功。未取得失敗 step 完整日誌，不能臆測根因；CI 僅限原 SHA，不是未套用 Day35 Green 的驗收。
- 本次有上述 Rust、HTTP redirect 與 WASM/debug 產物契約實測；未完整執行 DB、瀏覽器、Tauri GUI，也不是全產品功能驗收。沒有修改產品、migration checksum 或刪除資料庫。

## 後續調整

[同一份後續調整清單](../../FUTURE_WORK.md) 記錄清除、先遷有效測試再清理與保留條件；[現行流程](../../requirements/USER_FLOWS.md) 與 [UI 條件](../../UI_ALIGNMENT_REQUIREMENTS.md) 仍是功能驗收依據。

[本頁改寫前狀態與歷史原文](https://github.com/williechen/copilot_use/blob/5d66ad95b96a7fffd85856eb80676ddd7120baa6/docs/teaching/current/STATUS.md) 保留其原始來源與時間，不以本次文件整理覆蓋舊驗證。
