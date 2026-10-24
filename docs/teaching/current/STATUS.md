# 目前功能與驗證狀態

> 2026-10-24 目錄同步：現行程式庫為 `williechen/rust_ai_ring`，本次靜態來源 `0b3e58440bf4b36a398e3698e352e69faf53cfa2`，見 [SOURCE_LAYOUT](../../SOURCE_LAYOUT.md)。下列既有正式驗證仍屬原固定 SHA；目錄更名不將舊測試／GUI／CI 結果轉移至新 SHA，也不推進教學進度。

2026-10-23 依使用者確認，所有涉及 copilot_use 程式開發、移植、同步適配、重構、套用、建置、測試、CI、產品試驗、發布、安裝更新與功能驗收的工作要求均已廢除，不再列為待辦、選題或驗收門檻。copilot_use 僅維護需求、規則、教學文章、進度、索引與交付證據；文件內的教學程式碼用於說明 rust_ai_ring 的實作，不建立第二套產品。程式功能、測試、CI 與驗收只依 rust_ai_ring 精確 SHA。既有 copilot_use 程式、腳本、workflow、Release 及歷史欄位僅供追溯，不代表仍有維護或執行義務。

程式與功能驗收以 `williechen/rust_ai_ring` 為準；`copilot_use` 保存需求、文章與證據。本頁按功能說明現況，教學編號、交付與封存對照集中於索引和原始證據。

## 重複套用與本次可確認結果（2026-10-25）

精確來源 `williechen/rust_ai_ring/main@3236b94e95555c20e4709b5db40b0a8688b5762e`。[程式與呼叫鏈核對](../../source-review/2026-10-25-duplicate-application-review.md) 是重複篩查及選定 JS 契約實測，未做全面比對或完整產品驗收。

- Chat：SignInPage 與 LoginView 兩份登入表單仍在；合併修正尚未在此來源。另有只由舊測試呼叫的刷新 helper，signin/signup 與 shared 的認證 helper 重複。
- Pet：名稱規則已由 Tauri/PetStore 共用 pet-application，不再是空 library；全域單寵物與逐 ID 狀態通道仍並存。來源 bridge 的六個 Node 測試通過；tracked dist 的同三個契約測試一過兩敗，顯示舊產物缺先選取與失敗傳遞。
- JS 測試實際來源為 ff89918d；發布前新來源只改 Analytics 四個檔案，已確認整個 pet_apps 無差異才引用該結果。沒有宣稱在新來源重跑全部程式。
- Analytics：初始化與登入銜接已出現在 startup/CLI；正式 main 使用 password_login::build，舊 admin::build 尚未掛入 main。新 library 仍為預設範本，角色／來源 ACL 等缺口保留。
- 本次沒有產品程式修改、Rust／資料庫／HTTP／Tauri GUI 實測或 CI 全面核實；原正式驗收結果與 CI 仍各限其來源。重複程式尚待依真實 caller 清理，不因核對報告發布視為驗收。

## 先前來源的靜態與 CI 觀察（保留原證據範圍）

先前 Chat 核對來源為 `williechen/rust_ai_ring/main@d241cb04765002df797c614049149c3a0a3f7630`；下列其他功能與 CI 觀察保留其原來源 54f5d80a，不直接轉移到新來源。本次只核對四個 workspace、相關呼叫者與測試、GitHub Actions；未套用程式、未在此環境執行 Rust／DB／HTTP／GUI 驗收，未做全面比對。

- 現行帳密登入：`/signin` 已有 SignInPage，最新來源另新增 `/login` 的 LoginView，兩者為重複 Leptos 登入表單。教學已改為重用 SignInPage、GET `/login` 重新導向與移除 LoginView；產品尚未套用此去重修正，HTTP／SQLite／瀏覽器回歸未執行。選房導向、逐房 ACL、作者／歷史與共用認證仍有缺口。
- 寵物建立：既有 IPC、PetStore 與 SQLite 呼叫鏈存在；pet-application／pet-persistence 仍為空 library。名稱規則分層與控制字元拒絕尚未套用，原生多窗、故障注入與重啟仍待驗證。
- 管理員初始化：現有 bootstrap 自訂密碼至少 8 bytes，主啟動未初始化；預設 admin/admin 初始化與登入銜接尚未套用。角色、來源 ACL、SSE 與明細仍未驗收；analytics-persistence 尚空。
- 舊來源 54f5d80a 的 [Test All](https://github.com/williechen/rust_ai_ring/actions/runs/40101549290) 的 test-analytics-server 失敗；[Persistence](https://github.com/williechen/rust_ai_ring/actions/runs/40101549347) 的 test-persistence／PostgreSQL 步驟失敗。完整錯誤日誌根因未確認，不能宣稱現行 PG 測試通過。
- 舊來源 54f5d80a 的 Node、Cargo workspaces 四個 metadata jobs、Chat／Pet／Analytics check/build workflows 成功；此覆蓋不包含未套用程式或完整產品驗收。[精確 run/job/step 證據](../daily/day-34/source-snapshot.json)。

先前 d241cb04 的 CI workflow/job 本次尚未取得可確認結果，標記未知；未將修訂文件發布視為程式驗收。

## 歷史正式驗證來源（以下結果限原固定來源）

- 歷史正式驗證的程式／命令／執行精確來源：`f3f8255b0b701a1b57cb8c0a1737c3caa4319f2d`（workspace0.2.32）。先更新本地、清除舊target與測試DB，再從零冷建置；沒有沿用上一輪編譯或跨SHA執行證據。
- **整體正式驗證未全部通過**：單元／DB／SSR／WASM／建置與CI通過，嚴格Clippy仍失敗；歷史庫升級與完整產品缺口分開記錄。
- [清理後正式驗證報告](../history/evidence/2026-10-24-formal-f3f8255/REPORT.md) · [精確摘要](../history/evidence/2026-10-24-formal-f3f8255/SUMMARY.json) · [清理清單](../history/evidence/2026-10-24-formal-f3f8255/reset-cleanup.json) · [全部日誌／工具／截圖](../history/evidence/2026-10-24-formal-f3f8255/evidence.tar.gz)。

## 清理與新資料庫

使用者授權清空本地應用DB；實際未找到.env／DB URL或既有應用表，系統postgres／willie兩庫沒有非系統表，未刪其他DB。清除18個已識別舊測試／產物／WebKit路徑，7.44GiB邏輯大小；原始碼、README／docs修改和歷史報告保留。以全新SQLite與專用PGcluster驗證，沒有宣稱重設未定位的應用庫。驗證後停止服務，清除本輪測試DB與WebKit暫存。

## Chat

| 功能／驗證 | 現況 |
| --- | --- |
| 帳號與建房 | fresh SQLite的註冊、登入、會員建房／CSRF、拒絕匿名、程序重啟與登出撤銷通過；真瀏覽器操作與WS／DB證據見報告。 |
| 歷史載入提示 | 新增明確未接通提示與即時訊息空狀態；UI SSR12/12通過，原先失敗的提示斷言已修復。 |
| 未完成 | 歷史訊息仍未載入；提示存在不等於歷史功能完成。訊息作者、逐房ACL、長連線撤銷、稽核與完整管理仍分別追蹤。 |

## Pet

| 功能／驗證 | 現況 |
| --- | --- |
| 建立、多窗與控制窗 | 真GUI建立三個ID、各單一寵物圖像與獨立事件、原生關閉控制窗再開、單隻回籠及重啟profile清單，依具名結果記錄。 |
| 生活窗右鍵選取 | 真生活窗右鍵原生選單→選開啟控制窗→SQLite選取正確ID→控制台狀態更新，通過。 |
| 舊庫 | 指定4f0cd00→本輪migration仍VersionMismatch(20261018095626)，資料sentinel保留；空庫通過不代表已修升級。 |
| 待驗證 | DB／關窗／補償失敗注入、預設多寵物出籠、重啟完整生命週期、漫遊／拖曳及跨平台。 |

## Analytics

| 功能／驗證 | 現況 |
| --- | --- |
| 帳號與Session | fresh PG的真TTY初始化、首次改密碼、撤銷／重登、Leptos表單及來源token攝取／冪等通過。 |
| Dashboard 事件類型 | 真production HTTP與瀏覽器篩選／清除／空結果／來源切換，精確結果見本輪報告與觀察；與SSR helper分開。 |
| PG | postgres-integration全部targets37通過，另1項ignored實跑通過；指定舊migration升級探針成功。 |
| 待完成 | 角色與來源ACL、持久登入等待、即時更新、正式服務配對與完整後台。 |

## 共用驗證與限制

- workspace142 passed／0 failed／1 ignored；Server SSR26/26、獨立UI8/8、UI SSR12/12；Node3/3、SSR與兩案WASM通過。
- PostgreSQL37項加另1項ignored通過，與workspace有重疊，不相加為不同測試總數。
- 嚴格Clippy仍有4項：Pet巢狀if；UI set_rows unused及兩個dead_code。未擅修程式或checksum讓結果變綠。
- GitHub同SHA6 workflows／17 jobs成功；CI不涵蓋本輪所有lint／GUI／升級條件，不推論全產品成功。
- 歷史該輪未執行付費 AI、真實舊庫修復、跨平台或全面兩庫比對；原驗證結果僅適用上述固定來源，不能當作現行來源通過證據。
- [待完成功能](../../FUTURE_WORK.md) · [現行需求](../../requirements/USER_FLOWS.md) · [即時資料](../../learning-state.json) · [歷史證據](../history/README.md)。


本頁舊版與完整原始敘述可由 [原文件 commit](https://github.com/williechen/copilot_use/blob/e46733542aadb81f05f6bf1699cf899bcda405b7/docs/teaching/current/STATUS.md) 追溯；教學交付索引與產品驗收各自記錄。
