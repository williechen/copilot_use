# Day24 本地正式驗證：未全部通過

驗證日期2026-10-21（Asia/Taipei），精確來源 `rust_ai_chat/main 08b891a22c1934a2bf0b1edbb103c9b16bcf12de`、workspace0.2.23、Rust/Cargo1.99.1、Linux、PostgreSQL18.6。只驗使用者已推送的main，沒有套用教學或修正patch。167個tracked檔案逐一比對Git blob完全一致；僅在隔離副本另加兩份標明用途的升級探針，不修改應用程式或migration。

Day24三篇Notion正文與封存雜湊逐字一致，GitHub文章／封存不修改；Chat／Pet／Analytics next_day均維持25，last_full_comparison不變，排程未改。教學交付成功與使用者套用後功能驗收分開記錄，本輪結論是**未全部通過**。

## 測試與建置

| 命令／檢查 | 實際結果 | 證據 |
|---|---|---|
| cargo test --locked --workspace --exclude desktop-pet-frontend（SQLite測試URL） | 127通過、0失敗、1 ignored | workspace.log |
| cargo test --locked -p stats-server --features postgres-integration -- --include-ignored | 編譯失敗；Day24測試include_str引用不存在的SQL，沒有執行完整PG套件 | postgres.log |
| PG可執行子集：同feature加 --lib --test day23_password_login --test idempotency_contract --test postgres_event_store | 28通過、0失敗；不包含缺檔的Day24測試 | postgres-available-subset.log |
| cargo fmt --all -- --check | 通過 | fmt.log |
| cargo check --locked -p server --features ssr | 通過 | ssr.log |
| Chat hydrate／Pet frontend wasm32 check | 通過 | chat-wasm.log、pet-wasm.log |
| 三案native build、Pet Trunk真建置 | 通過 | build.log、pet-trunk-refreshed.log |
| strict Clippy --all-targets -- -D warnings | 失敗，4個collapsible_if | clippy.log |

workspace與PG子集有重疊，不相加成不同測試總數。workspace預設feature沒有啟用Day24的PG測試，所以workspace綠燈不代表該測試能編譯。Clippy位置：stats_server/admin.rs:321、desktop_pet/lib.rs:155、156、178。

## Chat：Day24註冊切片通過，完整授權仍未完成

真TCP啟動完成整套fresh SQLite migration；POST /signup後303 /signup/complete、清cookie、完成頁200且沒有自動導頁。大小寫帳號登入、/account、停止並重啟程序後同Session有效；帶原cookie再註冊新會員會撤銷舊Session、仍回完成頁；大小寫重複帳號409，SQLite實有兩筆新會員。

原本缺口仍存在：未登入GET /room/validation-room=200；POST /logout=404。這些不是Day24註冊測試通過就能宣稱完成的登入／登出／聊天室授權。使用者要求的登入、登出與Session共用模組仍是後續重構方向，不在本輪擅自改寫封存內容。

## Pet：多窗能開，但畫面與事件驗收失敗

真Tauri／WebKitWebDriver／Xvfb建立三隻資料後，能同時開三個不同ID的200×200原生生活窗，重複出籠只聚焦；後端三份FSM分離，關一隻不刪DB、不影響另兩隻，重啟載入明確預設身分。

但本輪強制重建WASM並用隔離XDG快取重跑後，**三個生活窗各有2個.pet-face**，截圖可見兩隻烏龜，且按鈕超出可見200px範圍。main在frontend/src/main.rs:135與140重複呼叫PetStatus；封存r2的完整patch原本只有載入身分後的一次，屬目前main的套用差異。

三窗都出現 `event.listen not allowed on window pet-...`：capabilities/default.json只列main、pet，未匹配動態pet-*。後端狀態雖已sleep，畫面仍停待機，不能把後端FSM探針通過冒稱生活窗即時同步成功。首次GUI快取結果只觀測到一個face，已保留initial-cache-*並標作不採信；正式結果採重新編譯及隔離快取後的2個face。

另需分清使用者指出的「動畫內容」：封存動畫字串本身含🐢💤、🐢z等複合符號，即使只渲染一個元件也不等於單一寵物圖像。這是後續單圖像動畫需求；本次main因事件權限錯誤，睡眠畫面沒有可靠更新，不能偽稱已通過動畫內容驗收。未在驗證中修正動畫、重複渲染或capability。

## Analytics：首次改密碼可用，但測試與後續改密碼有缺口

專用PG fresh庫以真TTY初始化管理員；真HTTP受限登入→報表403→GET真表單／CSRF→POST改密碼與Email→303/login並清cookie→舊cookie401。停止並重啟程序後舊密碼401、新密碼登入303/dashboard、dashboard200，代表已保存新密碼並維持首次改密碼要求。

但再次GET /change-password實際303/dashboard，不是表單200：password_login.rs:228的非restricted分支直接導回後台，與dashboard「修改密碼與Email」連結形成不可用入口。原完整HTTP腳本在這裡失敗；另跑followup逐項保存401／303／200／303證據，沒有把整條HTTP測試標成成功。

完整PG測試在day24_password_change.rs:245與261引用 `20261021071000_unique_account_email.sql`，main沒有此檔；唯一Email index卻被加入歷史20261018073932_create_auth_user.sql。封存教學提供的是新增migration，不能以改歷史SQL取代。

## 舊庫升級：兩案阻擋，資料未被刪除

使用精確main 4f0cd00各案migration先建立舊庫與一筆寵物／管理員，再用08b891a原始Migrator升級：Pet得到VersionMismatch(20261018095626)，Analytics得到VersionMismatch(20261018073932)。兩份探針的「測試通過」是成功重現預期缺陷，不代表升級通過；原資料筆數保持1。

不改checksum、不刪資料、不回寫封存教學來讓結果變綠；真實舊庫修復需先核對各DB歷史，再驗證升級方案。本輪只報告已重現的阻擋。

## CI同SHA快照

- [Persistence](https://github.com/williechen/rust_ai_chat/actions/runs/39595880327)：completed/success；2026-10-21T12:01:53Z；jobs test-persistence=success
- [Rust Dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39595880453)：completed/success；2026-10-21T12:01:02Z；jobs check-dashboard=success, build-dashboard=success
- [Rust Desktop Pet](https://github.com/williechen/rust_ai_chat/actions/runs/39595880622)：completed/success；2026-10-21T12:06:40Z；jobs check-pet=success, build-pet=success
- [Node](https://github.com/williechen/rust_ai_chat/actions/runs/39595880309)：completed/failure；2026-10-21T11:58:04Z；jobs test-node=failure
- [Rust AI Chat](https://github.com/williechen/rust_ai_chat/actions/runs/39595880201)：completed/success；2026-10-21T12:11:22Z；jobs build-chat=success, check-chat=success
- [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39595880206)：completed/success；2026-10-21T11:59:43Z；jobs test-chat-application=success, test-desktop-pet=success, test-ui=success, test-pet-domain=success, test-shared=success, test-stats-server=success, test-server=success, test-chat-domain=success, test-ai-core=success

CI Test All成功也不代表postgres-integration feature或本輪原生GUI已涵蓋；Node失敗與本地Rust測試分開看，原始job／時間／連結在ci.json。

## 保存與清理

完整log、三窗截圖、HTTP腳本、外加升級探針、來源逐檔雜湊、Notion封存核對與DB清理均隨本報告保存。本次專用55439 PostgreSQL已停止／刪除，隔離SQLite與本輪WebKit快取已清除；既有5432與copilot_use資料不動。儲存空間不足時只清除可重建的incremental編譯快取，未刪產品資料或原始碼。

來源repo原README dirty diff仍43新增／227刪除，既有docs/保留。新問題各案去重放入Day25待補清單；未修產品、未改Day24正文、未增加Day，也未更新last_full_comparison或恢復排程。尚未重測跨平台、付費AI、正式TLS或真實既有開發庫，不宣稱完整產品驗收。
