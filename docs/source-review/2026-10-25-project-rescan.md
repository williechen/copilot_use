# 專案重掃與後續調整核對

核對日期：2026-10-25（Asia/Taipei）。產品來源：`williechen/rust_ai_ring/main@891ff32ae29e1588dc40027cd5c9c99ca91d2378`。需求／待辦來源：[FUTURE_WORK.md](../FUTURE_WORK.md) 的「後續調整：重複與未使用程式、測試及產物」及 D01–D09 歷史報告。

本次重新取得 main，盤點 202 個追蹤檔案、78 個 Rust 檔與四個 workspace；比對原清單並追蹤呼叫端、route、Tauri 註冊、JS bridge、測試與 CI。全庫雜湊清單見 [inventory](rescan-891ff32/project-rescan-inventory.json)。這是全庫盤點與重點程式審查，並非每個功能的完整驗收。本次發布重掃報告、原待辦與統一狀態頁，附測試日誌及雜湊；產品程式、已發布 Day 正文、next_day／FIFO／last_full_comparison 保留原狀。

## 應先更正的舊狀態

1. **D01 登入表單合併已套用。** `chat_ui/src/login.rs` 與 `pub mod login` 已不存在；`signin.rs:35` 的 GET `/login` 會 redirect 至 `/signin`，POST `/login` 保留。單一 SignInPage 與 HTTP redirect 測試均已實測通過。仍需完整登入／Session／SQLite／hydration／瀏覽器回歸，不能直接結案整個認證功能。
2. **D02 舊 tracked dist 已移除。** 相較 `3aeab095`，最新來源刪除 dist 的 HTML、JS、WASM、CSS、四份生成 schema，以及 `pet_server/public/index.html`、`pet.js`，並新增忽略 gen 的規則。差異合計 11 個路徑（10 刪除、1 修改），產品 Rust／來源 bridge 無變更。舊 dist 二敗只能當歷史紀錄；最新乾淨 checkout 沒有該輸入，不能繼續寫「目前 tracked dist 二敗」。本次 debug 新建置與六個 dist 契約已通過；release、打包與 GUI 仍待驗證。
3. **D03 舊 public 原型已刪，但後端 legacy 通道仍在。** `get_pet_state`、`send_pet_command` 仍在 generate_handler；`Mutex<PetMachine>`、`pet://state-changed`、無 ID 選單 fallback 仍存在。刪掉頁面不等於已清完後端。注意 UI 的同名 Rust wrapper 會透過 JS 呼叫 `get_saved_pet_state`／`send_saved_pet_command`，不可誤刪正常通道。
4. **Analytics 過時文案斷言已實際重現兩敗。** 不再僅標成「未重跑／疑似過時」。更新測試應保留來源連結、可及性、資料隔離與 escaping 的原斷言。

## 原清單逐項結果

以下路徑均相對產品庫；行號對應本次精確來源。

| 原項目 | 最新核對結果 | 後續調整／保留條件 |
| --- | --- | --- |
| Chat 1／D01 | 已合併；`chat_server/src/signin.rs:35`、`chat_ui/src/lib.rs:145` | 排除重複套用 r2；保留正式 POST 與認證驗收 |
| Chat 2／未使用函式 | `chat_ui/src/room_api.rs:6` 的 create_room 仍在；私有 room_api 模組無呼叫該函式之處 | 可清函式、專屬 DTO imports／Request 類 imports；保留 list_rooms/search_rooms、server API、個人頁建房。依剩餘使用情況縮減 web-sys features |
| Chat 3／D06 | `refreshed_room_list:53` 只被舊測試使用；RoomList 呼叫 `refresh_result_view:70` | Day20 已測成功清搜尋與失敗保留清單；先把 Day19 的空結果／完整保留／重試條件映射到正式 helper，再刪舊 helper |
| Chat 4／D05、D07 | signin 的 digest、signup 的 Cookie/SHA256、Analytics 的 digest/raw_cookie 仍重複；建房名稱規則仍分置 `http.rs:25`、`signin.rs:148` | 重用既有 shared::auth；Analytics shared 目前僅 dev-dependency，改正式引用須同步 Cargo。兩種建房傳輸與各自回應碼保留 |
| Pet 1／D02 | 舊 dist 與 public 已不追蹤；Tauri frontendDist 與 beforeBuildCommand 仍指向現行 Trunk 流程 | 改追蹤新建置、同一 bridge 契約、JS/WASM/CSS 及實際 binary；不能把刪產物當成功建置 |
| Pet 2／未使用宣告 | `pet_ui/src/bridge.rs:67` 的 validate_new_pet_name_js 無 wrapper/caller；index.html 無 validateNewPetName | 可移除孤立 extern；server 的 validate_new_pet_name 仍是已註冊 IPC，不可混為同一個未使用項目。PetStore 名稱規則保留 |
| Pet 3／D03 | 舊後端 IPC／global state／event／menu 仍在；public 原型已刪 | 確認相容入口後一併移除 legacy 註冊與專屬測試；保留逐 ID Runtime、現行 wrapper、三隻狀態隔離與開關窗回歸 |
| Pet 4／D08 | `pet_ui/src/main.rs:298,302,306,315,335,340,343,351` 仍有同 handler 重複 reset／loading setter | 保留 await 後、過時 generation 早退的 busy 釋放；未觀察到此 handler 重複 invoke scan |
| Analytics 1／D09 | application/domain/ui 三份 lib.rs 仍逐位相同的 add／it_works；server 未依賴這三包 | 可清範本，保留 workspace 成員；空 library 不代表完成分層。persistence 仍空 |
| Analytics 2／D04 | `pub mod admin`、955 行 admin.rs 及 memory Session 測試仍存在；main/build_app 只用 password_login | 待確認公開 API 相容性後清 legacy Config／module／專屬測試；不能宣稱 main 同時運行兩套後台 |
| Analytics 3／fixture | Day23–27 等整合測試仍直接用 bootstrap::initialize；startup／CLI 已呼叫 ensure_default | initialize 保留。不能因 main 不呼叫而刪除；自訂初始密碼與保留 hash 契約要持續驗證 |
| Analytics 4／測試 handler | lib.rs 的 cfg(test) source_summary/source_kind_summary 及測試仍在 | 純 SQL 測試可移至 pg_store；正式授權應經 password_login HTTP/Session。保留來源隔離、交易與空結果驗證 |
| Analytics 5／過時斷言 | `dashboard_ui.rs:207`、`lib.rs:673` 兩個 library 測試已重現失敗；`tests/day27_dashboard_source.rs:139` 另有相同舊文案 | 修改語意斷言後重跑 library 與全部 postgres-integration targets；本次未跑 DB target，不宣稱第三處已實測失敗 |

## 額外掃描發現與邊界

- `chat_application/src/product_event.rs:12` 的公開 `persisted_message_event` 本庫未找到 caller；Pet `cleanup_event::succeeded_event` 只有契約測試。兩者應接回既有「真實事件至 Analytics／Outbox」功能缺口核對，不能直接列成可刪垃圾。沒有正式發送呼叫鏈，也不能把映射型別存在當作產品已上報。
- Analytics `EventIdentity`／`InMemoryDeduplicator` 有內部與整合測試使用；正式 ingest 則走 pg_store::persist_event。記憶體去重測試不是持久化冪等證據；保留其契約用途，除非完成測試遷移。
- 全檔相同掃描還找到四份 toolchain 及多組平台 icon；它們是工作區與打包資源，不因內容相同就刪除。
- Node workflow 目前只測來源 index.html；Desktop Pet build job 先 Trunk 再 native build，未在新 dist 上執行同一 bridge 契約。建議把產物契約核對放在 build 後。尚未驗證使用者已安裝的 binary。
- GitHub connector 對本精確 SHA 的 workflow run 查詢回傳空清單；這不足以證明 CI 通過或未執行。本報告僅陳述本機實測。

## 本次驗證

| 驗證 | 結果 |
| --- | --- |
| 四個 workspace `cargo metadata --locked --no-deps --format-version 1` | 全部 exit 0；成員數 crates 3、Chat 5、Pet 5、Analytics 5；不等於完整 check/build |
| `node --test pet_apps/pet_ui/tests/*.mjs` | 6 passed／0 failed；Node v22.22.1 |
| Chat `cargo test --locked -p chat-ui --features ssr` | 14 passed／0 failed；包含唯一登入表單、正式與舊 refresh helper 測試 |
| Analytics `cargo test --locked -p analytics-server --lib` | exit 101；23 passed／2 failed／1 ignored；失敗為上述 Day28／Day29 舊文案 |
| Chat HTTP redirect 定向測試 | `cargo test --locked -p chat-server --features ssr --lib day34_login_redirect_tests`：1 passed／0 failed（另 19 filtered out） |
| Pet `env NO_COLOR=true trunk build --locked` | exit 0；debug 前端建置成功；原有四個 Node 測試檔僅替換 HTML 輸入為新 dist，共 6 passed／0 failed |

工具：rustc 1.99.1、Trunk 0.21.14；本次 Trunk 使用 debug profile，沒有把它當成 release／原生 GUI 驗收。初次 Trunk 啟動受本機 `NO_COLOR=1` 解析影響，改以命令層 `NO_COLOR=true` 後執行；未修改專案。

原生日常操作、完整 Chat／Analytics 登入與資料庫、PostgreSQL 整合測試、Tauri 打包與 GUI、Windows／macOS 未在此輪完整執行。後續優先處理 Analytics 已重現的紅測試，接著驗證 Pet 新產物，再依 Chat→Pet→Analytics 原清單做最小清理切片。既有 Day、FIFO 與功能驗收狀態不因本次盤點自動結案。

測試原始日誌：[Analytics library](rescan-891ff32/analytics-lib-test.log)、[Chat UI](rescan-891ff32/chat-ui-test.log)、[Chat redirect](rescan-891ff32/chat-login-test.log)、[Pet Trunk](rescan-891ff32/pet-trunk-build.log)、[新 dist 契約](rescan-891ff32/pet-dist-contract-test.log)。dist 測試沿用來源測試的全部斷言，只改讀取 HTML 的 URL；這是 VM/mock 契約測試，不是原生 GUI 結果。產物雜湊見 [Pet build manifest](rescan-891ff32/pet-build-manifest.json)。
