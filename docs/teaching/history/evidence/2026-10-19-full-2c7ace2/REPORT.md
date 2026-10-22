# Day18 最新主線完整複驗

記錄時間：2026-10-19T00:21:38.348478+08:00。正式完整功能驗收：**未通過**。依原有切片及產品需求核對，未新增規則；三專案next_day仍各19。

來源為williechen/rust_ai_chat/main `2c7ace259ef790cea303d841f5779cd5f022d554`，本地 `/home/willie/Documents/rust_ai_chat` 已fast-forward到此版本。遠端main未寫入。修正分支 `codex/revalidate-f9484f2` 的最終commit `1abcb8aa6bc279e888a057b87069bea788281c7c`，基底為本輪main，未推送或合併。分支名稱保留本輪最初f948來源；檢查中main新增2c7ace2修正search_room／search_rooms名稱，已重新執行最新全量測試／建置／HTTP／瀏覽器，不拿中途f948結果冒充最新。

|Gate|最新main實際結果|修正與剩餘限制|
|---|---|---|
|完整workspace，SSR＋真PG＋include-ignored|126 passed / 1 failed / 0 ignored，exit101|候選131 / 0 / 0，exit0；含5項新增回歸，不是移除失敗測試|
|SSR限制提示|失敗：缺「歷史載入尚未接通」|候選恢復提示，原SSR測試通過；history功能仍未接通|
|參數化SQL LIKE與房名三筆|HTTP12案、瀏覽器三筆與空結果通過|名稱不是room ID；保留DB LIKE，不改回全量client contains|
|重新整理503|失敗：三筆被清空；按鈕可恢復重試|候選移除請求前清空rows、重設query、API結果直接顯示；查詢與重新整理503保留三筆及重試均通過|
|WS／真SQLite持久化|七案通過，另以持久訊息ID直接讀回SQLite|僅ping、typing、room isolation、persist then broadcast及重連；不等於帳號ACL與history驗收|
|Chat SQLite升級|9a95a10→本版通過；fa0888e→本版checksum mismatch；old419／791缺91733，CLI及真啟動均失敗|候選還原22554原檔＋新增130000前向修補，六案通過；已套用改版22554的新DB仍checksum mismatch，非通用升級方案|
|action log主鍵／操作者|新建DB缺少／NULL主鍵拒絕已通過；任意操作者ID仍可寫|沒有Rust寫入呼叫端或已授權操作者綁定，完整稽核未完成|
|Pet普通啟動／獨立SQLite|三份migration／九表、完整性與停用啟動通過|程序由測試timeout停止，非啟動崩潰；PetMachine資料持久化不可由建表推論|
|Pet原生視窗|候選四案實測通過，控制窗不動、停用不動、邊界反彈、關窗不回退main|可選acceptance-probe等待映射再啟動原roam；普通啟動漫遊時序、手動GUI／拖曳、逐寵物持久化、多寵物／多螢幕未驗證|
|Analytics Cookie續期|真HTTP require-renewal通過；另10案成功GET續期、400／401／錯CSRF／login不續期、logout後不復活|main已修，不能再寫Cookie仍失敗；候選額外短deadline真TCP通過，非1800秒browser soak|
|Analytics PG／身分隔離|新庫migration、真PG query、來源認證、冪等、Origin／CSRF／logout通過|舊PG31→32 CLI及啟動仍失敗；username/password表單422缺key，帳密／角色／scope／服務配對換證等原需求尚未完成|
|建置／品質|Leptos SSR＋WASM、Pet release前端及原生建置、fmt通過；嚴格Clippy exit101（admin.rs logout）|候選全部上述建置、fmt／嚴格workspace Clippy及probe feature Clippy通過|
|整體產品、跨平台與AI|未驗收|Chat註冊／管理／ACL／history、Pet完整GUI與持久化、Analytics帳密／角色與升級、服務閉環、跨平台／MSRV仍缺證據；未讀正式秘密、未付費AI呼叫|

## 重現命令、環境與原始結果

Ubuntu/Linux X11，Rust/Cargo1.99.1、SQLx0.9.0、Leptos0.8.22、Axum0.8.9、Tokio1.53.3、Tauri2.12.4、tauri-build2.7.3、wasm-bindgen0.2.129；cargo-leptos0.3.11、Trunk0.21.14、Node22.22.1、PostgreSQL18.6。Cargo.lock未改，[環境](environment.json)／[lock checksum](lock.sha256)。測試從本任務隔離工作目錄啟動，不載入產品正式dotenv；OPENAI_API_KEY移除，全部token、key與資料均為fixture。workspace用私有PG的verifyf948，真HTTP用verify2c，port55463；未碰使用者5432或原有資料庫。

完整測試（main與候選各實際執行）：
```bash
env -u OPENAI_API_KEY ANALYTICS_DATABASE_URL=postgres://verify63@127.0.0.1:55463/verifyf948 \
 ANALYTICS_TEST_DATABASE_URL=postgres://verify63@127.0.0.1:55463/verifyf948 \
 DATABASE_URL=postgres://verify63@127.0.0.1:55463/verifyf948 \
 ANALYTICS_CHAT_TOKEN=fixture-chat ANALYTICS_PET_TOKEN=fixture-pet \
 cargo test --locked --workspace --exclude desktop-pet-frontend \
 --features ui/ssr,stats-server/postgres-integration --no-fail-fast -- --include-ignored
```
候選的PG名稱實際為verify2c；詳細見[runbook](RUNBOOK.md)。main [126／1原始log](workspace-tests.log)，候選[131／0原始log](candidate-workspace-tests.log)。workspace-tests.analytics_postgres_ingest需要的postgres-integration未在該package宣告，另執行啟用命令exit101，見[原始錯誤](workspace-pg-feature.log)；該target零測試不算覆蓋。stats-server自己的PG integration已開啟並實際執行。

```bash
cargo leptos build --cargo-locked
(cd apps/desktop_pet/frontend && NO_COLOR=true trunk build --locked --release)
cargo build --locked -p stats-server -p desktop-pet --features tauri/custom-protocol
cargo fmt --all -- --check
cargo clippy --locked --workspace --exclude desktop-pet-frontend --all-targets \
 --features ui/ssr,stats-server/postgres-integration -- -D warnings
```
主線[建置退出碼](build-summary.log)、[WASM／SSR](main-leptos.log)、[Pet前端](main-pet-frontend.log)、[原生](main-native-build.log)、[Clippy失敗](clippy.log)；候選[Leptos](candidate-leptos.log)、[原生](candidate-native-build.log)、[workspace Clippy](candidate-clippy.log)、[probe Clippy](candidate-probe-clippy.log)。共享target在切換來源前清除對應package，凍結各SHA執行檔後才切換，未將舊快取當新版本。

真HTTP：`python3 http2c.py --seed`須空隔離DB；候選`candidate-http2c.py --seed`同樣另建空DB；各12案。[主線HTTP](http.json)／[候選HTTP](candidate-http.json)。`python3 full_http.py analytics require-renewal`兩者exit0，[主線](main-analytics-http.log)／[候選](candidate-analytics-http.log)；`python3 refresh-2c.py`的[10項Cookie邊界](refresh-guards.json)。HTTP服務固定3000／3100，fixture proxy3002僅注入GET503且不變更產品程式。[瀏覽器網路](network.jsonl)記錄真正帶q的請求。主線[三筆畫面](main-browser.txt)／[503清空失敗](main-refresh-failure.txt)；候選[三筆](candidate-browser.txt)／[503保留](candidate-refresh-failure.txt)／[查詢503保留](candidate-query-failure.txt)／[重試](candidate-retry.txt)／[空結果](candidate-empty.txt)。

`node ws2c.mjs`：七案[WS](ws.json)及[SQLite讀回](ws-persistence.json)。`python3 schema2c.py`33案、`python3 new-schema2c.py`27案，合計60案，[schema](schema.json)／[protocol](new-schema.json)。新建action log主鍵與任意操作者[實測](action-log-main.json)。`sqlx migrate run --source migrations --database-url sqlite://FIXTURE`的[主線升級原錯誤](upgrade-main.json)／[真啟動錯誤](upgrade-main-startup.json)、[9a升級](upgrade9a.json)；PG舊31[CLI](pg-old31-upgrade.log)／[啟動](pg-old31-startup.log)。候選[六案前向修補](action-schema-green.json)及[兩種checksum歷史相容性](candidate-upgrade-compatibility.json)。不刪原DB、不改_sqlx_migrations checksum、不聲稱所有歷史均可升級。

Pet候選：`cargo build --locked -p desktop-pet --features acceptance-probe,tauri/custom-protocol`後，用合成SQLite及PET_ACCEPTANCE_SCENARIO依序enabled／disabled／boundary／close，disabled配PET_ENABLE_ROAM=false，其他true。真OS座標每案16筆，單螢幕1280×800、工作區(0,25,1280,775)、pet200×200，main一直(100,100)。[enabled](native-enabled.log)／[disabled](native-disabled.log)／[boundary](native-boundary.log)／[close](native-close.log)。close時runtime回報webview已關閉並停止，探針確認沒有改動main；不將此預期關窗訊息藏掉。CUA原生控制不可用、inventory apps=[]，所以未宣稱手動GUI或正常啟動時序已通過。

## 精確main CI快照

SHA `2c7ace259ef790cea303d841f5779cd5f022d554`，查核時間 `2026-10-18T16:19:10.199186+00:00`。16 jobs success、0 failed、1 unfinished；pending不阻擋文件同步。[原始runs／jobs](ci.json)含workflow、SHA及時間，沒有候選遠端CI。Test All/test-ui沒有SSR，test-stats-server沒開PG feature／include-ignored；Persistence CI只測PG store。CI success不能代替本輪失敗SSR／升級或未完成GUI。

|job|狀態|UTC起訖|證據|
|---|---|---|---|
|build-chat|in_progress / pending|2026-10-18T16:06:10Z → None|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143471023/job/117722443663)|
|check-chat|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:14:25Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143471023/job/117722443980)|
|test-node|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:06:15Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143471017/job/117722443431)|
|test-stats-server|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:07:46Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470988/job/117722443330)|
|test-chat-application|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:06:44Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470988/job/117722443488)|
|test-desktop-pet|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:08:50Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470988/job/117722443636)|
|test-pet-domain|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:06:42Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470988/job/117722443645)|
|test-server|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:07:45Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470988/job/117722443675)|
|test-chat-domain|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:06:41Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470988/job/117722443679)|
|test-shared|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:06:38Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470988/job/117722443815)|
|test-ui|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:07:16Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470988/job/117722443830)|
|test-ai-core|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:06:51Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470988/job/117722443847)|
|build-dashboard|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:09:09Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470956/job/117722443600)|
|check-dashboard|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:07:38Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470956/job/117722443704)|
|test-persistence|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:09:05Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470959/job/117722443900)|
|build-pet|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:15:13Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470951/job/117722443516)|
|check-pet|completed / success|2026-10-18T16:06:10Z → 2026-10-18T16:14:42Z|[job](https://github.com/williechen/rust_ai_chat/actions/runs/39143470951/job/117722443920)|

## 文章同步與程式修正邊界

共同總頁、三專案入口、三篇Day18與三篇Day17適用性說明共10既有Notion頁已讀回。Day17原98f30d8 Rust code完整保留；Day15／16封存未改。Day18保留四節與完整Green，已套用的SQL／Cookie明示不能重複新增，舊失敗與新結果按SHA分列。Notion曾拒絕一次跨code的Day17連續替換，改小段落後成功，見[操作失敗與恢復](document-sync-errors.json)。v1正文未改、next_day各19與makeup_queue不變，last_full_comparison不變。本次不是週六全庫比較，沒有冒稱完整產品驗收。

[完整候選patch](functional-fixes.patch)以2c7ace2為基底，含SSR提示、刷新保留、直接API結果、logout條件、SQL／HTTP／Session回歸及可選Pet探針；migration部分只修原fa歷史，不可盲套已採改版22554的DB。分支commit與bundle保留本地，main未更改。copilot_use產品78d5796未在本輪重測、未合併，前輪註冊與SQL證據保留原SHA；本輪參考庫測試不等於產品通過。後續先完成DB歷史兼容及原有產品gate，再重測；不以增訂規則取代修程式。
