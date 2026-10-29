# 三案重複套用與殘留程式核對

> 歷史報告：下文保留原 `3236b94e` 核對與測試結果。最新 `891ff32` 已套用 Chat r2，且移除 Pet 舊 tracked dist／public 原型；Chat 定向測試及新 Pet debug 產物契約已通過。後端 legacy 清理與 Analytics 兩個舊文案斷言失敗仍待處理。請先讀 [最新專案重掃](2026-10-25-project-rescan.md) 與 [後續調整清單](../FUTURE_WORK.md)。


核對日期：2026-10-25（Asia/Taipei）。程式來源：`williechen/rust_ai_ring/main@3236b94e95555c20e4709b5db40b0a8688b5762e`；文件基準：`williechen/copilot_use/main@9650b170218dc115f33aca5f7603ee353cace1ab`。產品程式與 README 均未修改。

## 結果

確認有重複與新舊殘留；最優先為 **Chat 兩份登入表單**，以及 **Pet 來源 bridge 與 tracked dist 不一致**。其餘要分開看：Pet 舊狀態通道仍註冊；Analytics 舊後台尚未掛入現行主流程，不能說同一 main 正在跑兩套登入。

這是使用者要求的重複套用核對，不發布新的每日教學、不增加或回退 Day。Chat r2 仍是現行修訂，產品尚未套用；Pet 名稱規則於本來源已移入 pet-application 並接回 caller，不再把它列成空 library。三案 next_day 維持35，last_full_comparison 不更新。最新來源已將 Analytics ensure_default 接到 startup 與 CLI，屬已套用、未在本輪驗收；舊 initialize 僅為不同政策的相容 API／測試，不能再說主啟動缺初始化，也不推斷兩個函式在每次啟動都執行。

## 範圍與方法

- 對精確 commit 的追蹤檔匯出唯讀快照，篩查 133 個 Rust／JS／HTML／SQL／TOML／Node 測試文字檔（其中79個 Rust），另記錄四份 Cargo.lock。包括 gitignore 隱藏但已追蹤的 dist，沒有只靠 rg 預設檔案清單。
- 比對同內容檔案與12個非空、非註解行的重複候選；再人工追蹤路由、mod/use、Cargo dependency、Tauri generate_handler、事件、frontendDist、真正 UI caller 與測試。字串或函式名稱相同本身不是判定。
- 確認四個 workspace 的 manifests/locks；Analytics 已有5個成員，但新增三個 library 尚為預設範本。
- 靜態篩查不能保證已找出所有語意重複；未跑 Rust／Cargo／DB／GUI，這也不是全產品全面比對或功能驗收。

## 已確認項目

| ID | 專案 | 優先 | 分類 | 發現 |
| --- | --- | --- | --- | --- |
| D01 | Chat | 高 | 現行兩個入口各自渲染重複功能 | 兩份 Leptos 登入表單仍同時存在 |
| D02 | Pet | 高 | 來源與已追蹤建置產物分歧；已重現行為差異 | tracked dist 仍含舊 bridge |
| D03 | Pet | 中 | 新舊狀態通道並存；舊 IPC 仍註冊 | 單寵物全域狀態與逐 ID Runtime 共存 |
| D04 | Analytics | 中 | 舊實作仍公開編譯；未掛入現行 main router | 舊登入碼後台與正式帳密後台共存 |
| D05 | Chat / Analytics / shared | 中 | 重複邏輯；不是重複登入請求 | Cookie 解析與 SHA256 helper 尚未完全共用 |
| D06 | Chat | 中 | 新舊 helper 並存；舊版僅測試使用 | 列表刷新舊 helper 與正式 helper 重複 |
| D07 | Chat | 低 | 兩種合法傳輸入口的規則重複 | 建房名稱檢查複寫在 JSON API 與個人頁 |
| D08 | Pet | 低 | 同一事件處理器內的重複 setter | 掃描／重複檔預覽重複清除及 loading 寫入 |
| D09 | Analytics | 資訊 | 重複 Cargo 預設範本；沒有功能呼叫者 | 三個新 library 都仍是 add(2,2) 範本 |

### D01：兩份 Leptos 登入表單仍同時存在

GET /signin 經 App/SignInPage，GET /login 經 signin::login_page → chat_ui::login::render_login。兩表單都 POST /login，沒有需要第三份元件的缺口。r1 已套用；文件 r2 的合併尚未出現在本次來源。

證據：[chat_apps/chat_ui/src/lib.rs:146](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/chat_apps/chat_ui/src/lib.rs#L146) · [chat_apps/chat_ui/src/login.rs:4](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/chat_apps/chat_ui/src/login.rs#L4) · [chat_apps/chat_server/src/signin.rs:35](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/chat_apps/chat_server/src/signin.rs#L35)

必要處理：沿已交付 Chat r2 保留 SignInPage，GET /login 導向 /signin，移除 login.rs、pub mod login 與要求 GET /login 回表單的舊測試；保留 POST /login、原 Session 與帳密回歸。

### D02：tracked dist 仍含舊 bridge

來源 openPetControls 會先 select_pet；dist 只 open_control_window。dist 的原生選單也缺少帶 pet ID 的開啟控制窗項。tauri.conf.json 的 frontendDist 指向 ../pet_ui/dist，beforeBuildCommand 會執行 trunk build；正常重建可能消除差異，但未實跑，不能斷言已安裝 binary 正在使用此舊 dist。

證據：[pet_apps/pet_ui/index.html:48](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_ui/index.html#L48) · [pet_apps/pet_ui/dist/index.html:42](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_ui/dist/index.html#L42) · [pet_apps/pet_server/tauri.conf.json:9](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_server/tauri.conf.json#L9)

必要處理：以 pet_ui/index.html 作唯一 bridge 編輯來源，不手改兩份。重建後對 dist 執行同一套 bridge 契約測試，再建立 Tauri 產物並核對來源 SHA；同步確認 WASM/JS 一起更新，不只替換 index.html。

### D03：單寵物全域狀態與逐 ID Runtime 共存

舊 Mutex<PetMachine>、get_pet_state/send_pet_command、pet://state-changed 仍被管理及註冊，另有舊無 ID 原生選單分支；現行 Leptos bridge 使用 get_saved_pet_state/send_saved_pet_command 與 saved-pet-state/逐 ID Runtime。public/index.html + pet.js 使用舊命令，但目前 tauri build 指向 pet_ui/dist，未找到 public 作現行 frontend 的設定。

證據：[pet_apps/pet_server/src/lib.rs:16](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_server/src/lib.rs#L16) · [pet_apps/pet_server/src/lib.rs:184](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_server/src/lib.rs#L184) · [pet_apps/pet_server/src/lib.rs:274](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_server/src/lib.rs#L274) · [pet_apps/pet_server/src/pet_windows.rs:7](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_server/src/pet_windows.rs#L7) · [pet_apps/pet_ui/index.html:28](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_ui/index.html#L28) · [pet_apps/pet_server/public/pet.js:14](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_server/public/pet.js#L14)

必要處理：以逐 ID Runtime 作正常互動通道；逐一盤點／移除舊 command 註冊、global manage、舊選單 fallback 及 public 原型的使用者。不能只刪 HTML 而讓舊 IPC 保留，也不能將所有狀態合併為單一全域寵物。此處未證實一次點擊會同時 dispatch 兩遍。

### D04：舊登入碼後台與正式帳密後台共存

admin.rs 仍提供登入碼、記憶體 Session、Dashboard/Query 與 public build；password_login 提供 DB 帳密、Session、改密碼與 Leptos 後台。main 及 build_app 都採 password_login::build，未發現目前同一 main 同時 merge admin::build；舊模組仍有自己的測試。

證據：[analytics_apps/analytics_server/src/admin.rs:192](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/analytics_apps/analytics_server/src/admin.rs#L192) · [analytics_apps/analytics_server/src/admin.rs:450](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/analytics_apps/analytics_server/src/admin.rs#L450) · [analytics_apps/analytics_server/src/password_login.rs:431](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/analytics_apps/analytics_server/src/password_login.rs#L431) · [analytics_apps/analytics_server/src/main.rs:24](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/analytics_apps/analytics_server/src/main.rs#L24) · [analytics_apps/analytics_server/src/lib.rs:157](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/analytics_apps/analytics_server/src/lib.rs#L157)

必要處理：正式教學以 password_login 路徑為準，不再補第二套 admin 頁面。若決定淘汰舊公開 API，須同步 lib.rs 模組宣告、舊 key 設定、測試及外部相容性；不能把舊 memory Session 測試當正式 PostgreSQL Session 的通過證據。

### D05：Cookie 解析與 SHA256 helper 尚未完全共用

shared::auth 已提供 cookie_token/token_digest；Chat auth_session/logout 已使用，但 signin 又宣告相同 token_digest、signup 另寫 Cookie/雜湊。Analytics 正式 password_login 與舊 admin 也各有同型解析／雜湊。這是重複計算規則，未觀察到一次登入被插入兩筆 Session 的證據。

證據：[crates/shared/src/auth.rs:3](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/crates/shared/src/auth.rs#L3) · [chat_apps/chat_server/src/signin.rs:13](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/chat_apps/chat_server/src/signin.rs#L13) · [chat_apps/chat_server/src/signup.rs:158](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/chat_apps/chat_server/src/signup.rs#L158) · [analytics_apps/analytics_server/src/password_login.rs:15](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/analytics_apps/analytics_server/src/password_login.rs#L15) · [analytics_apps/analytics_server/src/password_login.rs:26](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/analytics_apps/analytics_server/src/password_login.rs#L26) · [analytics_apps/analytics_server/src/admin.rs:27](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/analytics_apps/analytics_server/src/admin.rs#L27)

必要處理：Chat 可重用已有正常 dependency shared::auth，保留呼叫端回應碼／交易；Analytics 的 shared 目前只在 dev-dependencies，若改正式 imports 必須先加入 normal dependencies 並核對 lock。資料庫、Cookie 名稱及各系統 Session 仍獨立，不能順便合併；shared 本身依賴 chat-domain 的耦合亦需評估。

### D06：列表刷新舊 helper 與正式 helper 重複

RoomList 真正呼叫 refresh_result_view（保留／清除搜尋字）；refreshed_room_list 只被舊測試呼叫。兩者都維護相同成功/錯誤提示，舊 helper 測試可能綠但實際 UI 路徑仍退化。

證據：[chat_apps/chat_ui/src/lib.rs:54](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/chat_apps/chat_ui/src/lib.rs#L54) · [chat_apps/chat_ui/src/lib.rs:71](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/chat_apps/chat_ui/src/lib.rs#L71) · [chat_apps/chat_ui/src/lib.rs:401](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/chat_apps/chat_ui/src/lib.rs#L401) · [chat_apps/chat_ui/src/lib.rs:448](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/chat_apps/chat_ui/src/lib.rs#L448)

必要處理：將舊測試改以 refresh_result_view 驗證原先的清單保留／空結果／重試條件及 query，確認沒有 production caller 後移除舊 helper。不是新增第三個 refresh API，也不刪掉重要斷言。

### D07：建房名稱檢查複寫在 JSON API 與個人頁

create_room JSON handler 與 account_create_room 表單 handler 都檢查 trim/200 UTF-8 bytes/NUL，並呼叫同一 chat_application::create_room。這兩個 endpoint 用途不同，不代表建立時一定寫兩次 DB。

證據：[chat_apps/chat_server/src/http.rs:21](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/chat_apps/chat_server/src/http.rs#L21) · [chat_apps/chat_server/src/signin.rs:136](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/chat_apps/chat_server/src/signin.rs#L136) · [chat_apps/chat_application/src/lib.rs:35](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/chat_apps/chat_application/src/lib.rs#L35)

必要處理：可在 application 共用名稱規則，兩個 transport 保留各自 JSON/HTML 與 400/422 契約及 CSRF；不因重複驗證就刪掉可操作的個人頁或 API。

### D08：掃描／重複檔預覽重複清除及 loading 寫入

scan_status.set(None)、duplicate_status.set(None) 在各自同一 handler 重複；loading=false 在 await 後及 match 後重複。未看到同一 handler 內重複 invoke scan 的程式。

證據：[pet_apps/pet_ui/src/main.rs:298](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_ui/src/main.rs#L298) · [pet_apps/pet_ui/src/main.rs:302](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_ui/src/main.rs#L302) · [pet_apps/pet_ui/src/main.rs:306](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_ui/src/main.rs#L306) · [pet_apps/pet_ui/src/main.rs:315](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_ui/src/main.rs#L315) · [pet_apps/pet_ui/src/main.rs:335](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_ui/src/main.rs#L335) · [pet_apps/pet_ui/src/main.rs:340](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_ui/src/main.rs#L340) · [pet_apps/pet_ui/src/main.rs:343](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_ui/src/main.rs#L343) · [pet_apps/pet_ui/src/main.rs:351](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/pet_apps/pet_ui/src/main.rs#L351)

必要處理：保留一次 reset；generation 過時分支也必須能釋放正確 busy 狀態，再去除尾端多餘 setter。不可直接把 await 後的 loading=false 移到早退之後而令畫面卡住。

### D09：三個新 library 都仍是 add(2,2) 範本

三檔 byte-for-byte 相同，都是 pub fn add 與 it_works；server 的 Cargo.toml 沒有正常依賴這三個 package。analytics-persistence 仍空，不能把 workspace 成員增加或範本測試通過當成業務接線完成。

證據：[analytics_apps/analytics_application/src/lib.rs:1](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/analytics_apps/analytics_application/src/lib.rs#L1) · [analytics_apps/analytics_domain/src/lib.rs:1](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/analytics_apps/analytics_domain/src/lib.rs#L1) · [analytics_apps/analytics_ui/src/lib.rs:1](https://github.com/williechen/rust_ai_ring/blob/3236b94e95555c20e4709b5db40b0a8688b5762e/analytics_apps/analytics_ui/src/lib.rs#L1)

必要處理：不要從空範本重建第二套頁面／Domain。日後確有切片需求時搬移既有實作並接回 server caller、Cargo、tests；本次不預先建立新骨架。

## 沒有判為重複套用的部分

- Pet 名稱規則：server 根模組 import pet_application::normalize_new_pet_name；PetStore::create 也呼叫同一 application 函式，沒有再保留 server 版函式定義。多個 caller／測試重用同一實作不是重複套用。
- Chat/Pet 的 SQLite 與 Analytics PostgreSQL、各自 migration／Session／Cookie 必須維持獨立；相似 DDL 不能直接刪成單一產品資料庫。
- 四個 workspace 的 toolchain.toml 同內容及各自 lock 是明確 workspace 需求，不是應刪除的副本。
- file_organizer 的兩個 ensure_non_root_process 分別受 cfg(unix)／cfg(not(unix)) 控制，單一目標只編入一個，不能當成函式重複宣告。
- Pet server SavedPet 與 WASM bridge 的反序列化 DTO 是傳輸邊界；可評估共用契約，但不代表建立或選取會執行兩次。
- HTTP/API／表單與 native menu／按鈕可以是不同操作入口；必須核對其是否共用用例，不能只因都能觸發同功能就任意移除。

## 實際測試與來源／產物差異

執行環境 Node v24.22.1；Rust/Cargo 未安裝，相關測試未執行。只執行現有 JS 契約與輸入檔差異重現，沒有啟動 Tauri、資料庫或讀取正式秘密。

| 驗證 | 命令／方法 | 實際結果 |
| --- | --- | --- |
| 來源 bridge | 在程式快照根執行 `node --test pet_apps/pet_ui/tests/*.mjs` | 6 passed／0 failed，exit 0 |
| tracked dist | 沿用原 day33-open-controls-selects-origin.test.mjs 三個測試，只把 HTML 輸入路徑換成 tracked dist/index.html；在 scratch 暫存測試執行 | 1 passed／2 failed，exit 1 |
| dist 第一失敗 | 來源預期先 select_pet 再 open_control_window | actual calls=1、expected=2（`1 !== 2`） |
| dist 第二失敗 | 選取失敗必須 reject，不得假報開窗成功 | `Missing expected rejection.` |

Node 實測來源為 ff89918df8e6da0480308497abc81ba5188703c9。發布前 main 更新到 3236b94e95555c20e4709b5db40b0a8688b5762e；git diff 確認只改 Analytics 四個檔案，整個 pet_apps 無差異，並逐檔保留 blob 證據，因此這兩項 JS 結果可用於相同 Pet 檔案，沒有宣稱在新來源重跑整套測試。原測試本身與產品程式沒有修改；臨時測試僅用於讀取另一個輸入 HTML。完整 dist 原始輸出見 [失敗日誌](duplicate-review-3236b94/pet-dist-test.log)，檔案 blob/SHA256 見 [來源清單](duplicate-review-3236b94/inventory.json)。VM/mocks 證實該 script 的契約差異，不等於完整 WebView/WASM/原生 GUI 已驗證。

來源與產物都包含 bridge script 是建置的正常現象；本次缺陷是兩者版本分歧，不是僅因檔名出現兩次。tauri beforeBuildCommand 預期會重建 dist，因此不能從 git 內舊檔直接推定使用者已安裝的 binary 也舊；仍需對本次實際產物測試與綁定 SHA。

## 套用銜接與驗證

Chat 依 [既有修訂](../teaching/daily/day-34/chat-r2.md) 套用，不再建立 LoginView。其餘項目留在原功能缺口追蹤，按最小切片處理；本次不在產品 repo 自動刪程式、不建立另一套產品、不預排每日 Day。

Pet 重建須從正確工作目錄（rust_ai_ring 根目錄）執行；以下僅為使用者後續驗證命令，本輪未安裝 Trunk/Tauri 或執行建置：

```bash
cd pet_apps/pet_ui
trunk build --release
node --test tests/*.mjs
# 再以相同3個bridge契約測試讀取新dist/index.html，通過後才建Tauri產物。
cd ../pet_server
cargo tauri build
```

Tauri beforeBuildCommand 已設為在 ../pet_ui 執行 trunk build --release；不得另外維護手改 dist 的平行程式。Trunk/Tauri CLI 版本與 OS SDK 相容性本輪未新核對，以上不當成已驗證可部署命令；既有專案設定與工具依使用者實際安裝環境確認。

對 Chat／Analytics 清理 legacy 時，先對正式 caller 路徑保留 HTTP、Session、DB 與權限測試；不要把舊 router 的 helper／記憶體測試當現行入口的驗收。對 Pet 移除舊狀態時，要驗三個不同 ID 的窗與事件隔離、IPC 註冊及原生選單。

## 交付界線

只向 copilot_use/main 寫文件及證據；原教學正文、來源 SHA、原 commit/blob/SHA256 保留。learning-state／manifest 只增加核對索引，next_day／makeup_queue／last_full_comparison 不變。這份核對不是三案全部功能或整套程式的通過聲明；CI未在本輪作全面重跑或完整核實。
