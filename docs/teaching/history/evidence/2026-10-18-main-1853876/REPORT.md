# Day18 最新程式核對與文章同步

記錄時間 2026-10-18T18:01:40.294174+08:00。最新 main `1853876e0eb1cfe8e4562be8b3c661b55f1bc54b`（add table）。正式完整功能驗收：**未通過**。
修正候選 `80ba01e6d965d0e3ff10bef18a5cbd17f01e8f05` 在 `codex/day18-revalidate-1853876`，parent 為最新 main；未推送／合併 main。

## 這次更新與實測

- Chat：95aff59 把 filter_room 從第一筆完全同名改為全部完全同名；1806326、1853876 未再修改搜尋。API 3 筆包含匹配（2 筆 Rust 技術交流＋1 筆 Rust 技術交流延伸），瀏覽器只顯示 2 筆，較先前 794da6e 的 1 筆改善，但仍漏列延伸名稱。中文含空白查詢顯示 3 筆、空白查全部 5 筆通過。SSR/WASM build 通過，SSR 歷史未接通提示斷言仍失敗。候選 list_rooms → filter_rooms 完整顯示 3 筆，ID 保留。
- Pet：原生單元／整合 34 案通過；真實 X11/Tauri enabled、disabled、boundary、close 四案再次通過，控制 main 固定 (100,100)。實際核心與 Cargo.lock 無差異，使用既有 opt-in 探針（測試啟動等待映射，並非普通啟動驗證）。完整手動 GUI、拖曳／睡眠／執行中設定／持久化、多螢幕／多寵物仍未驗證。
- 新增 schema：root migrations 由 Chat persistence 的 SqlitePool／sqlx::migrate! 使用；新增 pet、AI 人物與 pet_owner，不是 Analytics PG migration。最新全部 SQL 可套用，SQLite／SQLx persistence 5 案通過；但 pet_profile.owner_id 等仍 REFERENCES owner(id)，實際只有 pet_owner。外鍵開啟時 INSERT pet_profile 即使 owner_id=NULL 也回 no such table: main.owner。INSERT pet_owner(name) 成功但 id=NULL，SQLite 的 SERIAL 不會提供 PostgreSQL SERIAL 的自動整數主鍵語意。rg tracked runtime source 無新表 repository/API/UI 呼叫；這是 schema 草稿，沒有證據支持資料管理／帳密／ACL 已完成功能。
- Analytics：main 真實 HTTP 的匿名、Origin、CSRF、來源 token 分離、PG query、logout 撤銷通過，但活動回應 COOKIE_RENEWED: False。舊 PG migration31 套用後升級32，仍回 migration 20261018073931 was previously applied but is missing in the resolved migrations。候選 Cookie True 且未復活逾時 Session；候選未修 migration 升級、pet schema，也未完成帳密／角色與來源 scope。1800 秒瀏覽器 soak 未執行。

## 命令、結果與來源適用性

先實際測試 main 1806326；發布前檢查發現 1853876，只新增 pet_owner migration。逐檔 diff 確認所有 runtime source、依賴／lock 無變；最新1853876 已重跑 SQLite 全部 migrations、SQLx persistence。runtime/build/browser/PG/native 的1806326證據僅在上述無差異範圍適用1853876，沒有冒稱全部命令都在1853876執行。候選在1853876重新建立 commit，完整回歸、建置與真實 HTTP/browser 已再次執行。

| 命令／範圍 | 實際結果 | 證據 |
|---|---|---|
| main180：cargo test --locked -p ui --features ssr -p stats-server -p desktop-pet -p persistence | 58 passed／1 failed／1 ignored；SSR 提示失敗 | [log](baseline180-main-tests.log) |
| main180：UI native/front clean 後 cargo leptos build --cargo-locked | SSR/WASM 通過 | [log](baseline180-leptos.log) |
| main180：新 SQLite、python3 day18_http.py chat、瀏覽器查詢 | HTTP 201/200；完整房名 3→2 失敗；部分／空白通過 | [HTTP](baseline180-chat-http.log)、[browser](baseline180-main-browser.txt) |
| main180：stats-server clean／build、python3 day18_http.py analytics | guard／PG／CSRF／logout 通過；Cookie False | [HTTP](baseline180-analytics-http.log)、[build](baseline180-stats-build.log) |
| main180：DATABASE_URL／ANALYTICS_TEST_DATABASE_URL=專用 DB cargo test --locked -p stats-server --features postgres-integration -- --include-ignored | 新建 DB 21 passed／0 failed／0 ignored；非升級驗收 | [log](baseline180-main-pg-configured.log) |
| main180：ANALYTICS_DATABASE_URL／CHAT_TOKEN／PET_TOKEN=fixture cargo test --locked -p workspace-tests | 20 passed | [log](baseline180-main-root-configured.log) |
| 舊 PG migrations→最新 PG migrations，cargo sqlx migrate run --source | 升級失敗 missing migration31 | [old](baseline180-old-migrations.log)、[upgrade](baseline180-upgrade.log) |
| native PET_ACCEPTANCE_SCENARIO=enabled/disabled/boundary/close | 4 passed；OS 座標讀回 | [enabled](baseline180-pet-enabled.log)、[disabled](baseline180-pet-disabled.log)、[boundary](baseline180-pet-boundary.log)、[close](baseline180-pet-close.log) |
| 最新185：SQLite pragma foreign_keys=on，依序 executescript，INSERT pet_owner／pet_profile | 遷移通過；owner ID NULL；pet INSERT 失敗 | [log](sqlite-migrations.log) |
| 最新185：cargo test --locked -p persistence | 5 passed | [log](persistence.log) |
| 候選185：fixture env cargo test --locked -p ui --features ssr -p stats-server -p desktop-pet -p server -p desktop-pet-frontend -p workspace-tests -p persistence | 105 passed／0 failed／1 ignored | [log](candidate-tests.log) |
| 候選185：cargo leptos build --cargo-locked，瀏覽器完整房名 | SSR/WASM 通過；3 筆全部顯示 | [build](candidate-leptos.log)、[browser](candidate-browser.txt) |
| 候選185：stats-server clean／build、python3 day18_http.py analytics require-renewal | Cookie True；guard、CSRF、logout 通過 | [HTTP](candidate-http.log)、[build](candidate-stats-build.log) |
| 候選180：PG postgres-integration include-ignored | 22 passed；stats source/lock 與候選185完全相同 | [log](baseline180-candidate-pg.log) |

缺少 fixture env 的初次 root／PG 執行曾報 NotPresent，已保留 *-configured 前的原始失敗 log；補齊專用環境後的成功另列，沒有算成產品 defect，也沒有隱藏執行失敗。重複覆蓋的數字不能相加當唯一案例數。

## 必需修改與重測缺口

1. Chat：套用候選完整 rows 搜尋及歷史未接通提示，再於 main 重測三筆同名／延伸名稱與 SSR。候選通過不等於 main 通過。帳號／個人管理／ACL、保留目標房間、歷史、跨瀏覽器 WS、行動／鍵盤功能仍待原產品驗證。
2. SQLite schema：依既有使用者／寵物主人模型確認 owner_id 契約，對齊 pet_owner 與所有外鍵；SQLite 主鍵改用符合 domain 的 INTEGER PRIMARY KEY 或明確 TEXT ID，補有 owner、無 owner、多筆 ID、關聯與升級資料回歸。已有部署時不得直接改歷史 migration checksum，需前進 migration；候選尚未實作，不能推定需求語意。
3. Analytics：候選修正已驗證 server expiry／Cookie 同步；主線仍待套用。保留 migration31 的歷史 checksum，兼容31、32兩種既有基線，再測新建與升級。帳密、Email、角色／來源 scope、撤銷與1800秒瀏覽器 soak 未完成或未驗證。
4. Pet：完成原有完整 GUI／普通啟動／拖曳與睡眠／執行中啟停及持久化／多螢幕與多寵物 gate。原生四案的成功範圍不擴張。

## 文章同步、環境與版本

既有 Day18三篇、Day17三篇的當前適用性、專案入口三頁、共同總頁現況共10頁已修訂並全文讀回；原頁ID、子頁及四節保存。GitHub 三篇鏡像依 Notion讀回正文產生，manifest／learning-state與四份產品文件同步。v1規則正文逐字未變；next_day各19、makeup_queue及last_full_comparison均不推進。先前基線與證據另留原資料夾，未改寫歷史失敗。

Ubuntu26.04.1、Rust/Cargo1.99.1、PostgreSQL18.6、Node22.22.1；cargo-leptos0.3.11、Trunk0.21.14。Cargo.lock Axum0.8.9／Leptos0.8.22／SQLx0.9.0／Tauri2.12.4／Tokio1.53.3／wasm-bindgen0.2.129。單X11 1280×800，workarea(0,25,1280,775)、pet200×200；專用PG127.0.0.1:55418、新SQLite、合成token。未使用正式秘密、未呼叫付費AI、未更改遠端main。

完整修正見 [patch](day18-functional-fixes.patch)，本機可恢復bundle另附。探針與HTTP重現見 [REPRODUCTION](REPRODUCTION.md)。短deadline TCP測試為跨初始2秒活動後仍200，再30ms idle＋等60ms回401，不是1800秒soak。

## 最新 SHA CI

CI僅列實際workflow/job覆蓋；不代替上方功能結果。完整API快照見 [ci.json](ci.json)。

| workflow/job | 狀態／結論 | 證據 |
|---|---|---|
| check-pet | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332711/job/117645671158) |
| build-pet | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332711/job/117645671300) |
| test-persistence | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332765/job/117645671488) |
| check-chat | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332741/job/117645671199) |
| build-chat | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332741/job/117645671457) |
| test-node | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332671/job/117645671190) |
| build-dashboard | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332682/job/117645671159) |
| check-dashboard | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332682/job/117645671348) |
| test-chat-application | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332672/job/117645671271) |
| test-pet-domain | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332672/job/117645671358) |
| test-ai-core | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332672/job/117645671425) |
| test-chat-domain | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332672/job/117645671434) |
| test-stats-server | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332672/job/117645671452) |
| test-ui | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332672/job/117645671467) |
| test-server | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332672/job/117645671509) |
| test-desktop-pet | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332672/job/117645671527) |
| test-shared | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117332672/job/117645671582) |

CI觀察時間 2026-10-18T09:59:51.720757+00:00，SHA `1853876e0eb1cfe8e4562be8b3c661b55f1bc54b`。尚在執行者保持 pending，候選未推送沒有遠端CI。
