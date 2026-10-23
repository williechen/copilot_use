# 目前功能與驗證狀態

2026-10-23 依使用者確認，所有涉及 copilot_use 程式開發、移植、同步適配、重構、套用、建置、測試、CI、產品試驗、發布、安裝更新與功能驗收的工作要求均已廢除，不再列為待辦、選題或驗收門檻。copilot_use 僅維護需求、規則、教學文章、進度、索引與交付證據；文件內的教學程式碼用於說明 rust_ai_chat 的實作，不建立第二套產品。程式功能、測試、CI 與驗收只依 rust_ai_chat 精確 SHA。既有 copilot_use 程式、腳本、workflow、Release 及歷史欄位僅供追溯，不代表仍有維護或執行義務。

程式與功能驗收以 `williechen/rust_ai_chat` 為準；`copilot_use` 保存需求、文章與證據。本頁按功能說明現況，教學編號、交付與封存對照集中於索引和原始證據。

## 驗證來源

- 本輪最新目標：`be0494a2a70be91eb74b2b726970a23fc3e5c044`；**正式驗證未全部通過**。
- 完整建置、DB、HTTP、瀏覽器與原生GUI來源：`7b016106058084de7c1d0e19eec061b17ceda063`。be0494a只調整Day31 UI測試SSR feature gate，生產程式、依賴與migrations未變；已核對差異，並在新版重跑fmt／UI／workspace。
- [本輪完整報告](../history/evidence/2026-10-23-formal-be0494a/REPORT.md) · [精確摘要](../history/evidence/2026-10-23-formal-be0494a/SUMMARY.json) · [日誌、工具與截圖](../history/evidence/2026-10-23-formal-be0494a/evidence.tar.gz)。本輪結果取代前次對「尚未套用」的現況敘述，原日誌與文章當時事實保留。

## Chat

| 功能／驗證 | 現況 |
| --- | --- |
| 帳號、Session與建房 | 新註冊一般會員登入、匿名建房401、個人頁表單CSRF拒絕403／成功201、API建房201、重啟讀回Session與房間、登出撤銷通過。 |
| Day31個人頁UI | 真瀏覽器登入→輸入名稱→建立→顯示進房連結→點擊進房→登出通過；不再列為未套用。 |
| 訊息 | 雙頁WS廣播與SQLite恰1筆寫入通過；重新整理後沒有載入既有訊息，登入者發送仍保存為anonymous_user。 |
| SSR回歸 | 最新版10通過、1失敗：缺少「歷史載入尚未接通」提示。 |
| 待完成 | 作者身分、逐房ACL、長連線撤銷、歷史／重連、完整管理與歷史DB升級等仍獨立追蹤。 |

## Pet

| 功能／驗證 | 現況 |
| --- | --- |
| 建立與多窗 | 真控制台建立三個ID、SQLite三筆；三個200×200生活窗各單一寵物圖像，睡眠事件只更新所選ID，另兩窗保持待機。 |
| Day31控制窗生命週期 | 真原生WM_DELETE_WINDOW關閉控制台後，從生活窗點「控制台」可恢復；已實測。 |
| 回籠與重啟 | 回籠只關閉該生活窗；App重啟讀回三筆profile清單通過，未證明生活窗自動恢復／出籠標記全部一致。 |
| 舊庫升級 | 從指定4f0cd00 migrations升級，仍VersionMismatch(20261018095626)；合成sentinel保留，不能列為升級通過。 |
| 待驗證 | DB／關窗／補償失敗注入、預設出籠與重啟生命週期、漫遊／拖曳及跨平台安裝仍未完成。 |

## Analytics

| 功能／驗證 | 現況 |
| --- | --- |
| Leptos登入與改密碼 | Day31改密碼頁已套用；新CLI初始化、首次限制、Leptos表單與CSRF、交易改密碼、撤銷與新密碼重登、日後改密碼頁200等真HTTP通過。 |
| DB與攝取 | PostgreSQL全部targets36通過，另1項ignored實跑通過；Chat／Pet分別token攝取201、重送200通過。 |
| 指定舊庫升級 | 4f0cd00→本輪migrations成功，合成sentinel保留；不等於所有歷史cohort都已驗收。 |
| 瀏覽器與權限 | 本輪未重跑Analytics瀏覽器；原瀏覽器證據仍依原SHA。角色／來源ACL、登入等待、即時更新等仍待完成。 |

## 共用驗證與限制

- 最新be0494a workspace139通過／0失敗／1 ignored；UI CI同命令8通過／0失敗；UI SSR10通過／1失敗。PG、SSR與workspace有重疊覆蓋，不相加為獨立測試總數。
- 原始7b01610：workspace140通過／1 ignored、Server SSR26通過；Node3/3、SSR／兩案WASM compile、Chat與Analytics release、Pet frontend release與native debug build通過。
- 嚴格Clippy仍失敗：Pet巢狀if，以及UI unused變數與dead_code。Pet歷史migration阻擋與Chat歷史缺口仍存在，整體Gate未通過。
- 相同SHA CI快照見[報告](../history/evidence/2026-10-23-formal-be0494a/REPORT.md)；舊7b01610的test-ui失敗與新版feature gate修正分開判定，不把執行中的run記為成功。
- 未執行付費AI、真實舊庫修復、跨平台或全面兩庫比對；next_day維持三案各32，last_full_comparison維持2026-10-16T22:47:14.399027+08:00，makeup_queue與原Day正文不改寫。
- [待完成功能](../../FUTURE_WORK.md) · [現行需求](../../requirements/USER_FLOWS.md) · [即時資料](../../learning-state.json) · [歷史證據](../history/README.md)。

## 排程與通知

原排程文章已過時，[目前排程文件](../scheduler-instructions-v1.md)僅提供最新規則與狀態入口，不代表已部署提示詞。既有每日 06:00、週六／週日時間及啟用敘述不能當作即時任務證據；實際時間、提示詞、模型、通知及送達狀態尚未從排程服務讀回。本機未找到對應 automation.toml，本次未修改或觸發任務。每週至少一次全面比對的需求保留；教學交付與功能驗收分開，文字整理不推進進度或結案。
