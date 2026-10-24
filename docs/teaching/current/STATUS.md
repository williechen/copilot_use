# 目前功能與驗證狀態

> 2026-10-24 目錄同步：現行程式庫為 `williechen/rust_ai_ring`，本次靜態來源 `0b3e58440bf4b36a398e3698e352e69faf53cfa2`，見 [SOURCE_LAYOUT](../../SOURCE_LAYOUT.md)。下列既有正式驗證仍屬原固定 SHA；目錄更名不將舊測試／GUI／CI 結果轉移至新 SHA，也不推進教學進度。

2026-10-23 依使用者確認，所有涉及 copilot_use 程式開發、移植、同步適配、重構、套用、建置、測試、CI、產品試驗、發布、安裝更新與功能驗收的工作要求均已廢除，不再列為待辦、選題或驗收門檻。copilot_use 僅維護需求、規則、教學文章、進度、索引與交付證據；文件內的教學程式碼用於說明 rust_ai_chat 的實作，不建立第二套產品。程式功能、測試、CI 與驗收只依 rust_ai_ring 精確 SHA。既有 copilot_use 程式、腳本、workflow、Release 及歷史欄位僅供追溯，不代表仍有維護或執行義務。

程式與功能驗收以 `williechen/rust_ai_chat` 為準；`copilot_use` 保存需求、文章與證據。本頁按功能說明現況，教學編號、交付與封存對照集中於索引和原始證據。

## 驗證來源

- 本輪程式／命令／執行精確來源：`f3f8255b0b701a1b57cb8c0a1737c3caa4319f2d`（workspace0.2.32）。先更新本地、清除舊target與測試DB，再從零冷建置；沒有沿用上一輪編譯或跨SHA執行證據。
- **整體正式驗證未全部通過**：單元／DB／SSR／WASM／建置與CI通過，嚴格Clippy仍失敗；歷史庫升級與完整產品缺口分開記錄。
- [清理後正式驗證報告](../history/evidence/2026-10-24-formal-f3f8255/REPORT.md) · [精確摘要](../history/evidence/2026-10-24-formal-f3f8255/SUMMARY.json) · [清理清單](../history/evidence/2026-10-24-formal-f3f8255/reset-cleanup.json) · [全部日誌／工具／截圖](../history/evidence/2026-10-24-formal-f3f8255/evidence.tar.gz)。

## 清理與新資料庫

使用者授權清空本地應用DB；實際未找到.env／DB URL或既有應用表，系統postgres／willie兩庫沒有非系統表，未刪其他DB。清除18個已識別舊測試／產物／WebKit路徑，7.44GiB邏輯大小；原始碼、README／docs修改和歷史報告保留。以全新SQLite與專用PGcluster驗證，沒有宣稱重設未定位的應用庫。驗證後停止服務，清除本輪測試DB與WebKit暫存。

## Chat

| 功能／驗證 | 現況 |
| --- | --- |
| 帳號與建房 | fresh SQLite的註冊、登入、會員建房／CSRF、拒絕匿名、程序重啟與登出撤銷通過；真瀏覽器操作與WS／DB證據見報告。 |
| Day32歷史提示 | 新增明確未接通提示與即時訊息空狀態；UI SSR12/12通過，原先失敗的提示斷言已修復。 |
| 未完成 | 歷史訊息仍未載入；提示存在不等於歷史功能完成。訊息作者、逐房ACL、長連線撤銷、稽核與完整管理仍分別追蹤。 |

## Pet

| 功能／驗證 | 現況 |
| --- | --- |
| 建立、多窗與控制窗 | 真GUI建立三個ID、各單一寵物圖像與獨立事件、原生關閉控制窗再開、單隻回籠及重啟profile清單，依具名結果記錄。 |
| Day32右鍵選取 | 真生活窗右鍵原生選單→選開啟控制窗→SQLite選取正確ID→控制台狀態更新，通過。 |
| 舊庫 | 指定4f0cd00→本輪migration仍VersionMismatch(20261018095626)，資料sentinel保留；空庫通過不代表已修升級。 |
| 待驗證 | DB／關窗／補償失敗注入、預設多寵物出籠、重啟完整生命週期、漫遊／拖曳及跨平台。 |

## Analytics

| 功能／驗證 | 現況 |
| --- | --- |
| 帳號與Session | fresh PG的真TTY初始化、首次改密碼、撤銷／重登、Leptos表單及來源token攝取／冪等通過。 |
| Day32事件類型 | 真production HTTP與瀏覽器篩選／清除／空結果／來源切換，精確結果見本輪報告與觀察；與SSR helper分開。 |
| PG | postgres-integration全部targets37通過，另1項ignored實跑通過；指定舊migration升級探針成功。 |
| 待完成 | 角色與來源ACL、持久登入等待、即時更新、正式服務配對與完整後台。 |

## 共用驗證與限制

- workspace142 passed／0 failed／1 ignored；Server SSR26/26、獨立UI8/8、UI SSR12/12；Node3/3、SSR與兩案WASM通過。
- PostgreSQL37項加另1項ignored通過，與workspace有重疊，不相加為不同測試總數。
- 嚴格Clippy仍有4項：Pet巢狀if；UI set_rows unused及兩個dead_code。未擅修程式或checksum讓結果變綠。
- GitHub同SHA6 workflows／17 jobs成功；CI不涵蓋本輪所有lint／GUI／升級條件，不推論全產品成功。
- 未執行付費AI、真實舊庫修復、跨平台或全面兩庫比對；next_day維持三案各33、last_full_comparison維持2026-10-16T22:47:14.399027+08:00；makeup_queue與原Day正文不改寫或封存。
- [待完成功能](../../FUTURE_WORK.md) · [現行需求](../../requirements/USER_FLOWS.md) · [即時資料](../../learning-state.json) · [歷史證據](../history/README.md)。

## 排程與通知

原排程文章已過時，[目前排程文件](../scheduler-instructions-v1.md)僅提供最新規則與狀態入口，不代表已部署提示詞。既有每日 06:00、週六／週日時間及啟用敘述不能當作即時任務證據；實際時間、提示詞、模型、通知及送達狀態尚未從排程服務讀回。本機未找到對應 automation.toml，本次未修改或觸發任務。每週至少一次全面比對的需求保留；教學交付與功能驗收分開，文字整理不推進進度或結案。
