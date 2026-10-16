# 提前執行週六工程：2026-10-16

使用者要求提前執行 2026-10-17 的週六工程，並把整合版升級為 v0.2.0。
來源參考 `rust_ai_chat/main` 為 `3e58e650bd63bcd21ba58e7aabf751542401ef80`；嚴格唯讀。
本機程式基線為 `de22e95f22e5a84dc45c60a5c854166272bb5c63`；本輪提交與最終 Gate 另存驗收紀錄。
原 v0.1.0 的 tag、附件及歷史四份 release approval 保持固定，不用舊證據宣稱新版通過。

## 全量來源核對

本輪比對完整 tracked tree、逐檔 SHA256 與全部自有 Rust 宣告，233 份 Rust 可解析、零語法錯誤；自有參考 54 檔、產品 71 檔歸入 38 個既有職責群組，沒有重複或漏列。
第三方 GLib backport 原樣保留；不宣稱重新完成第三方全量安全審計。
未變模組承接 2026-10-14 的完整接口／資料流程核對，覆核其後所有 reference source 變更及本輪必要產品變更，沒有以文字差異要求不必要重構。
詳細 mapping／決策見 module-review.json，逐檔身分見 tracked-content.json。

## 已處理的必要功能

- Chat 完成保存：先重新驗證 session／CSRF；一般使用者的有效 session、身分及房間成員資格，在 BEGIN IMMEDIATE 交易中與訊息、Outbox、realtime 保存一起確認。撤權、過期或 CSRF 失敗不保存，也不傳 done；未宣稱撤權可抹除先前已串流的文字。
- Pet 真實漫遊：實際移動 pet 原生視窗；純座標規則支援負座標、邊界反彈、過大視窗及極端整數。預設關閉，需明確啟用並選 Moving，依速度設定移動；停滯或平台失敗時停用，不只是 sprite 動畫。
- 重複檔預覽：尺寸候選後 SHA256；透過已授權 root descriptor／handle 讀檔，前後複核 root 與檔案身份。Unix 額外核對 ctime，拒絕 symlink、非本人／不安全權限與變動檔案。每檔 64 MiB、每次 256 MiB 的讀取上限；不遞迴到未列出的子目錄。UI 只接 opaque ID／檔名／大小，仍逐檔確認隔離，不自動刪除。
- v0.2.0：workspace、前端、Tauri config 與四份 lock 同步；測試更新版本由正式版推導為下一 patch，CI 安裝包檔名同步推導，不固定為 0.1.x。
- 原生桌面：系統 GTK／WebKit runtime 由使用者以 sudo 完成安裝。正常啟動不使用 Docker、Xvfb 或 WebKit 路徑轉接；安裝器核對 binary --version，保留舊版本與原有資料，不寫 autostart。

## 保留合理差異與明確適配

| 契約 | 決策 |
|---|---|
| Chat 建房 | 參考 name-only／自動 ID 與產品指定 ID 保留；兩邊目前均回 201。category 不改名為 topic，不更動既有 schema。 |
| WS | 參考已新增持久寫入與房間隔離 TypingChanged；產品保留 HTTP 寫入＋durable 通知、ACL、queue／resync，不直接複製 tagged DTO。 |
| Pet FSM／Profile | Sleep/Wake／revision 與產品 Action/Speed 不冒稱等同；JSON 與 TOML 透過各自既有 codec，保留設定及投影語意。 |
| Cleanup 事件 | 產品 schema v1 正式輸出保留 file.cleanup.succeeded。參考 pet.cleanup.succeeded 不是產品入口的自動別名；教學適配只能在真實操作成功收據後建立產品事件，不以純狀態成功冒充。 |
| Analytics | 參考目前已是來源認證＋PG 攝取／去重／查詢，舊 501 stub 敘述不再適用。產品仍保留事件白名單、gRPC 逐事件 ACK、HTTP 管理、SSE、權限及交易能力。 |
| Migration／平台 | 不修改既有 migration；SQLite／PG Adapter 的差異留在 Adapter，Windows／macOS 新行為沒有本機執行證據時不冒稱已通過。 |

## 驗證紀錄

第一輪基線完整原生檢查成功，包含 69 項 integration／故障恢復檢查。
既有腳本第一輪含 5 項本地 MCP protocol 檢查；已加 --functional-only，後續週六 Gate 排除這些項目，不把它們列為 AI 重測證據。
未讀取 provider API key、未付費呼叫；Chat 授權修正以本地文字 fixture 驗證。

三組新增回歸均已先觀察失敗再通過：session／ACL 撤權，視窗座標邊界，重複群組與變動檔案／root。
初次 Linux GUI 功能測試已通過；本輪最終來源及正式版的原生功能／GUI／建置結果，須以後續 acceptance.json 與同來源報告為準。
參考庫此 SHA 的六個 GitHub workflow 全部 success；新產品本機提交的遠端 CI／GitHub Release 狀態與本機 Gate 分開。

本紀錄不是每日教學，不發布新 Day，也不改學習完成狀態。
