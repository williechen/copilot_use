# Day23封存與本地正式檢驗（2026-10-21 Asia/Taipei）

Day23三篇已依使用者要求封存；正文、程式碼與GitHub原稿未改，Notion只標記標題並逐篇讀回確認正文不變。此封存是政策凍結，不聲稱設定Notion權限鎖。三案next_day維持24，last_full_comparison與排程不變。

## 檢驗版本與範圍

實測 `rust_ai_chat/main 8b9d59110349609c940e540ee9d5b42883d52b3e`，workspace0.2.23，rustc/cargo1.99.1，Linux原生、PostgreSQL18.6。這是使用者已推送的六個Phase2 day23提交；不是上輪f38360c候選，也沒有再套Day23 patch。本地main已安全fast-forward；原README.md未提交差異仍43新增／227刪除、既有docs/保留，不覆寫未提交變更。所有新增測試資料與執行副本在本聊天work目錄，來源庫程式唯讀；copilot_use只寫教學封存與工程證據。

## 實際結果

| 檢查 | 結果 | 證據 |
|---|---|---|
| cargo test --locked --workspace --exclude desktop-pet-frontend（DATABASE_URL=sqlite::memory:） | 122通過／0失敗／1 ignored | workspace.log |
| cargo test --locked -p stats-server --features postgres-integration -- --include-ignored（兩個PG環境變數皆指專用cluster） | 28通過／0失敗／0 ignored | postgres.log |
| cargo fmt --all -- --check | 通過 | fmt.log |
| cargo check --locked -p server --features ssr | 通過，有既有UI warnings | ssr.log |
| Chat UI hydrate與Pet frontend wasm32 check | 兩者通過 | chat-wasm.log／pet-wasm.log |
| 三案native build | 通過 | build.log |
| Pet Trunk前端資產 | 通過 | pet-trunk-online.log |
| cargo clippy --locked --workspace --exclude desktop-pet-frontend --all-targets -- -D warnings | 失敗：兩處collapsible_if | clippy.log |

workspace與PG的重疊測試不得相加當不同測試數。未把ignored算通過；PG另外確實執行原需獨立DB的測試。

嚴格Clippy未過位置：`apps/stats_server/src/admin.rs:321`、`apps/desktop_pet/src/lib.rs:134`。依只檢驗main的授權，本輪沒有修產品程式。Leptos native直接啟動也出現缺LEPTOS_OUTPUT_NAME的設定警告；HTTP實測仍成功，但未以此宣稱Chat完整hydration瀏覽器驗收。

## 真實程序與原生GUI

Chat：真TCP GET /signup=200，POST /signup=303，大小寫帳號登入=303；/account未登入401、有效cookie200；停止並重啟server程序後同cookie仍可存取。另測未登入GET /room/formal-unauthed仍200，**房間頁面尚無授權防護，不能視為完整聊天室安全驗收**。

Analytics：先在全新專用PG資料庫，以真TTY兩次輸入教學密碼執行bootstrap_admin，exit0。新main在未設舊admin-key下可啟動：舊key表單422、帳密登入303、受限頁200、受限報表403；真正停止再啟動stats-server後cookie仍可讀受限頁；登出303，舊cookie401。首次改密碼提交仍未提供，完整初始化→改密碼→報表主線尚未完成。

Pet：Trunk建出最新WASM，由真Tauri WebView與Tauri driver啟動；控制台初始讀取清單、建立寵物→真IPC→SQLite、重啟原生應用後讀回同一ID、重新載入按鈕，四項均通過。截圖pet-created.png。此驗證尚未涵蓋逐寵物生活窗／外觀／預設出籠與完整多寵物流程。

工具處理：初次Trunk被環境NO_COLOR=1解析阻擋，清除此變數後離線模式仍缺對應wasm-bindgen工具；允許工具下載後建置成功。初次GUI harness沿用舊訊息「已儲存」，但當前程式顯示「已建立寵物」；依真實source修正斷言，在全新測試DB重跑後四項通過，沒有修改產品程式。

## DB清除範圍與讀回

使用者明確指定：只清空rust_ai_chat三案開發DB，保留copilot_use。盤點未找到三案有效.env或DB環境設定、未找到來源repo內既有SQLite開發檔。既有5432只有postgres／willie兩個非template DB，兩者均無應用資料表，因此沒有刪除整個服務或無關資料庫。

實際清除3個SQLite測試DB與2個專用PostgreSQL測試cluster（包含本聊天前輪留下的cluster）；先停止所有測試程序，再移除資料。work下已無DB檔；3000、3100、1420、55439與driver ports均已釋放。既有5432服務與copilot_use資料保持不動。詳細逐檔路徑見db-cleanup.json。這不等於聲稱找到了未設定位置的正式開發DB並清空；未提供位置的DB不能憑名稱猜測刪除。

## 同SHA CI快照

- [Node / 39456179654](https://github.com/williechen/rust_ai_chat/actions/runs/39456179654)：completed／success；2026-10-20T16:18:00Z UTC。
- [Rust AI Chat / 39456179774](https://github.com/williechen/rust_ai_chat/actions/runs/39456179774)：in_progress／None；2026-10-20T16:18:03Z UTC。
- [Rust Dashboard / 39456179626](https://github.com/williechen/rust_ai_chat/actions/runs/39456179626)：completed／success；2026-10-20T16:21:03Z UTC。
- [Test All / 39456179730](https://github.com/williechen/rust_ai_chat/actions/runs/39456179730)：completed／success；2026-10-20T16:20:28Z UTC。
- [Persistence / 39456179885](https://github.com/williechen/rust_ai_chat/actions/runs/39456179885)：completed／success；2026-10-20T16:21:58Z UTC。
- [Rust Desktop Pet / 39456179638](https://github.com/williechen/rust_ai_chat/actions/runs/39456179638)：completed／success；2026-10-20T16:27:49Z UTC。

CI只記實際workflow狀態，不替代本輪本地結果或未涵蓋GUI／跨平台／正式DB升級。

## 結論

封存、最新main的本地檢驗與已識別測試DB清除均已完成。測試與重啟接線通過，但嚴格Clippy失敗及上述功能缺口仍存在，因此不標記完整產品功能驗收通過。已封存Day23不回寫；需要修正時在Day24或之後處理。本輪不推進Day、不更新last_full_comparison、不建立或更改排程。
