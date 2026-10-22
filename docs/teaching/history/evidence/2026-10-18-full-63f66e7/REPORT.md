# 三專案全面複驗與必要文章更新

記錄時間 2026-10-18T19:23:48.949702+08:00（Asia/Taipei）。最新main `63f66e7742ecd09cd5199a47dbacc9df666af416`；本地 /home/willie/Documents/rust_ai_chat HEAD與GitHub main已核對一致。正式完整功能驗收：**未通過**。

修正候選 `3eace139da323fc906133636fae93e4d54159173` 留在 `codex/full-acceptance-63f66e7` 隔離worktree，parent即本輪main，未推送／合併main。本地程式來源未套候選；驗證後一般執行檔已重新建置為main，避免測試探針或候選覆蓋日常程式。既有資料庫與使用者檔案本輪未重建／刪除。

## 實際結果

主線完整workspace：**126 passed／1 failed／0 ignored**。失敗為UI SSR缺少「歷史載入尚未接通」，不是缺套件或DB未設定。候選同命令：**128 passed／0 failed／0 ignored**。總數含各crate單元、整合及doc-tests；重複覆蓋的HTTP／schema／probe數不可再相加作唯一案例總數。

| 專案或範圍 | 通過 | 失敗或未驗證 |
|---|---|---|
| Chat | SSR/WASM建置、HTTP建房／清單、輸入驗證、真WS七案與SQLite持久化、部分／大小寫／空白搜尋、503保留清單並重試 | main完整名稱3匹配只列2；無相符仍提示成功；SSR歷史限制提示斷言失敗；/signin及/api/messages404；帳號／ACL／歷史／跨瀏覽器產品流程未完成或未驗證 |
| Pet | SQLite正常啟動、兩份migrations與七張業務表、PK/FK約束、單元／整合；真實Tauri座標enabled／disabled／boundary／close四案 | 完整手動GUI、正常啟動漫遊時序、拖曳／睡眠／執行中設定持久化、多螢幕／多寵物未驗證；DB表存在不等於CRUD／PetMachine已持久化 |
| Analytics | 新PG啟動migrations、21項所屬單元／PG／契約、匿名／Origin／CSRF／來源身分分離／query／logout撤銷與冪等 | main活動Cookie續期assert失敗；舊PG31升級與啟動VersionMissing；username/password表單422缺key；帳密／角色／scope等未完成；1800秒browser soak未執行 |
| 既有DB升級 | 新Chat四份、Pet兩份、Stats PG兩份新建成功 | 4198fb1與7917f8b舊Chat DB升級missing91733；舊Stats31改名32後missing31。重建空DB並非兼容升級 |
| 候選修正 | 128通過；瀏覽器完整3筆、正確無相符提示；真實HTTP Cookie=True；短deadline TCP逾時401不復活 | 尚未合併main，未修舊DB兼容或整體帳密／多寵物產品缺口 |

## 命令與原始證據

測試先建立專用PostgreSQL18 cluster，loopback127.0.0.1:55463，fixture角色verify63，DB verify63／verify63_upgrade；SQLite全部在任務work/下，不操作使用者chat.db／pet.db。AI套件已確認只使用MockChatModel，本輪沒有付費AI請求。

主線與候選相同測試命令如下，唯一差異是checkout與source SHA：
```sh
env -u OPENAI_API_KEY \
 ANALYTICS_DATABASE_URL=postgres://verify63@127.0.0.1:55463/verify63 \
 ANALYTICS_TEST_DATABASE_URL=postgres://verify63@127.0.0.1:55463/verify63 \
 DATABASE_URL=postgres://verify63@127.0.0.1:55463/verify63 \
 ANALYTICS_CHAT_TOKEN=fixture-chat ANALYTICS_PET_TOKEN=fixture-pet \
 cargo test --locked --workspace --exclude desktop-pet-frontend \
 --features ui/ssr,stats-server/postgres-integration --no-fail-fast -- --include-ignored
```

| 執行項目 | 實際結果 | 原始證據 |
|---|---|---|
| 上述main完整workspace | exit101；126 passed／1 failed／0 ignored | [main](workspace-tests.log) |
| 上述候選完整workspace | exit0；128 passed／0 failed／0 ignored | [candidate](candidate-workspace-tests.log) |
| main／candidate cargo leptos build --cargo-locked | SSR/WASM皆成功 | [main](main-leptos.log)、[candidate](candidate-leptos.log) |
| main NO_COLOR=true trunk build --locked --release；cargo build --locked -p stats-server -p desktop-pet --features tauri/custom-protocol | Pet前端、原生及Stats皆成功；非安裝包 | [frontend](main-pet-frontend.log)、[native](main-native.log) |
| python3 day18_http.py chat，專用新SQLite | 建房201、清單／SSR200；再加同名／延伸fixture供瀏覽器 | [HTTP](main-chat-http.log)、[rooms](rooms.json)、[AX](main-browser.txt) |
| CUA瀏覽器按查詢；注入GET /api/rooms503後移除並重試 | main3→2失敗；其他搜尋／失敗保留／重試通過；candidate完整3筆及空結果提示通過 | [main failure](main-http-failure.txt)、[main retry](main-http-retry.txt)、[candidate](candidate-browser.txt)、[candidate failure](candidate-http-failure.txt)、[candidate retry](candidate-http-retry.txt)、[network](chat-network.jsonl) |
| Node WebSocket script，兩個同房客戶＋一個另一房 | ping/pong、跨房拒絕、空白拒絕、typing、同ID廣播、另一房無事件、重連七案通過；SQLite同ID已讀回 | [WS](ws.json)、[script](ws63.mjs) |
| schema script，SQLite foreign_keys=ON | 33案：有效新增、缺主鍵／關聯及無效外鍵拒絕；owner delete限制；AI room_id為text | [results](schema.json)、[script](check_full_schema.py) |
| main python3 day18_http.py analytics | Origin／CSRF／來源分離／新PG query／logout通過；Cookie False（基礎腳本exit0不代表續期通過） | [HTTP](main-analytics-http.log) |
| main python3 day18_http.py analytics require-renewal | exit1 AssertionError: valid activity did not renew browser cookie | [Red](main-cookie-red.log) |
| candidate同require-renewal命令 | exit0，Cookie True；完整guards／query／logout回歸通過 | [Green](candidate-analytics-http.log) |
| main真HTTP username/password、房名全空白／超長／NUL | 帳密422缺key；無效房名400 | [HTTP](main-additional-http.json) |
| SQLx舊Chat4198fb1／7917f8b基線先套用，再run最新root migrations | 舊版exit0；新版exit1 applied91733 missing | [419 old](old419-old.log)、[419 new](old419-new.log)、[791 old](old791-old.log)、[791 new](old791-new.log) |
| 舊PG31先套用，再run最新Stats migrations；main binary指向同舊DB | 升級exit1 missing31；啟動exit1 VersionMissing(20261018073931) | [old](pg-upgrade-old.log)、[new](pg-upgrade-new.log)、[startup](main-stats-old-db-startup.log) |
| main Pet binary＋專用SQLite，PET_ENABLE_ROAM=false | 兩份遷移、七張業務表、程序持續運行，沒有PgPool認證錯誤 | [startup](main-pet-startup.log)、[schema](schema.json) |
| candidate acceptance-probe四情境，真實OS座標讀回 | 四案exit0，控制main全程(100,100)，disabled pet固定(300,300)，boundary由(1080,600)向內，close後main不動 | [enabled](pet-enabled.log)、[disabled](pet-disabled.log)、[boundary](pet-boundary.log)、[close](pet-close.log)、[build](probe-build.log) |
| cargo test -p workspace-tests --test analytics_postgres_ingest --features postgres-integration | exit101：workspace-tests未宣告該feature；通常workspace此target跑0 tests，不能算覆蓋。Stats套件自身PG測試已確實執行 | [error](dormant-pg-test.log) |

## 範圍限制與 blocking gaps

1. main名稱查詢與SSR提示仍失敗，修正集中於本輪候選與Chat Day18；main未套用不能借用候選綠燈。
2. main Session server touch與Cookie失同步，已由真實assert失敗確認。候選把續期放在成功的受保護GET回應層，回應前重查仍有效且未撤銷，再同步Set-Cookie。候選真TCP測試跨初始2秒期限仍200，再設測試30ms deadline、實際等60ms後401不刷新／不復活。這不是1800秒瀏覽器soak；該長時測試未執行。
3. 舊Chat已套用Pet migrations的相容性：保留歷史Chat migration內容／checksum，另設Pet新DB遷移與資料搬移／保留ID關係回歸；不能靠搬走舊檔或刪_sqlx_migrations通過。舊Stats31／32也需兩種基線兼容。本輪候選未修這些資料升級，沒有刪除或重建正式DB。
4. Pet幾何採與main相同漫遊核心的opt-in測試儀器，先等映射再定位／啟動；不宣稱普通啟動時序或手動拖曳通過。單X11工作區(0,25,1280,775)、pet200×200；負座標僅單元測試，真實多螢幕未測。`cua.getState`原生apps實際為空，完整原生GUI控制不可用；手動UI／逐寵物持久化等維持未驗證。
5. 原產品帳號／權限／歷史、多寵物與管理完整流程沒有因schema／CI／文章發布而完成。HTTP帳密422、登入路由404與0-test feature缺口均如實保留。file organizer測試只用自有fixture，沒有掃描私人檔案。

本次檢查已涵蓋目前可執行的完整workspace、DB、HTTP／WS、瀏覽器與原生核心；尚有失敗／不可驗證gate，**不正式標記完整功能驗收**。沒有新增驗收規則、沒有以口頭認定取代證據。

## 版本與交付

Ubuntu26.04.1；Rust／Cargo1.99.1；PostgreSQL18.6；SQLx0.9.0；Axum0.8.9；Leptos0.8.22；Tauri2.12.4／tauri-build2.7.3；Tokio1.53.3；wasm-bindgen0.2.129；cargo-leptos0.3.11；Trunk0.21.14。Node WS測試使用bundled24.19.0。實際命令輸出、鎖版與來源適用性按檔保留，不使用過去另一個checkout的未歸屬binary結果。

候選完整差異在 [functional-fixes.patch](functional-fixes.patch)，parent即本次main，分支commit已保留；可恢復bundle與瀏覽器截圖另附本地完整證據包。一般本地執行檔最後以main重新建置，未把probe留作日常程式；候選實際HTTP使用獨立複製執行檔與candidate site，跟main分開。

原Day18三篇已依本輪結果重寫四節、提供完整Green與範圍；Day17三篇保留原98f30d8教學／未執行歷史，更新當前適用性；三專案入口與共同現況，共10頁皆發布並全文讀回。Notion原ID／子頁保存，v1正文逐字不變。GitHub鏡像取自讀回正文，manifest／learning-state僅更新測量結果與來源；next_day各19、makeup_queue、last_full_comparison均保持。Day15／Day16封存未改。copilot_use產品不因參考rust_ai_chat結果自動驗收。

| Notion頁面 | 最後編輯版本 | 讀回 |
|---|---|---|
| 🦀 Rust AI 三專案｜共同學習規則與前進目標 | 2026-10-18T11:18:31.653Z | [原頁](https://app.notion.com/p/3f2e51d4509381ddb8a8ce7cb4cd3e60?pvs=204) |
| 聊天室 | 2026-10-18T11:18:24.719Z | [原頁](https://app.notion.com/p/3f3e51d45093807da638c336e857543a?pvs=204) |
| 桌面寵物 | 2026-10-18T11:18:26.053Z | [原頁](https://app.notion.com/p/3f3e51d450938067923ae5a424d30950?pvs=204) |
| 📊 統計儀錶板｜聊天室 × 桌面寵物 | 2026-10-18T11:18:27.175Z | [原頁](https://app.notion.com/p/3f3e51d45093817eafaee0cfc9ea4cc8?pvs=204) |
| 2026-10-18｜P0 Chat Day 18｜查詢房間完成 HTTP 接線與錯誤恢復 | 2026-10-18T11:23:43.067Z | [原頁](https://app.notion.com/p/3fde51d45093816bb36dd9099190d907?pvs=204) |
| 2026-10-18｜P0 Pet Day 18｜漫遊只移動生活窗並核對真實標籤 | 2026-10-18T11:23:44.770Z | [原頁](https://app.notion.com/p/3fde51d4509381bfa106ea4d9b190480?pvs=204) |
| 2026-10-18｜P0 Analytics Day 18｜有效活動延長伺服器 Session 並拒絕過期 | 2026-10-18T11:23:47.679Z | [原頁](https://app.notion.com/p/3fde51d4509381c2a0d5f60041137619?pvs=204) |
| 2026-10-18｜P0 Chat Day 17｜查詢房間接通 HTTP | 2026-10-18T11:18:28.498Z | [原頁](https://app.notion.com/p/3fce51d45093818b913bebb8dab5ab65?pvs=204) |
| 2026-10-18｜P0 Pet Day 17｜漫遊指向寵物生活窗 | 2026-10-18T11:18:29.359Z | [原頁](https://app.notion.com/p/3fce51d4509381ca83e3e825f0313698?pvs=204) |
| 2026-10-18｜P0 Analytics Day 17｜活動延長管理員 Session | 2026-10-18T11:18:30.277Z | [原頁](https://app.notion.com/p/3fce51d4509381ea871ec9c92da37c1b?pvs=204) |

## 最新main精確CI快照

觀察時間 2026-10-18T11:19:01.857281+00:00，source `63f66e7742ecd09cd5199a47dbacc9df666af416`。CI只證明實際workflow/job的範圍，不代替功能gate。完整原始資料含workflow/run、開始／結束時間在[ci.json](ci.json)。

| job | 狀態／結論 | 開始／結束UTC | 證據 |
|---|---|---|---|
| build-chat | completed／success | 2026-10-18T10:55:59Z／2026-10-18T11:10:05Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189735/job/117656965220) |
| check-chat | completed／success | 2026-10-18T10:56:07Z／2026-10-18T11:05:27Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189735/job/117656965267) |
| check-pet | completed／success | 2026-10-18T10:55:58Z／2026-10-18T11:03:24Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189742/job/117656965200) |
| build-pet | completed／success | 2026-10-18T10:55:58Z／2026-10-18T11:05:02Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189742/job/117656965389) |
| test-node | completed／success | 2026-10-18T10:55:58Z／2026-10-18T10:56:05Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189734/job/117656964965) |
| test-ai-core | completed／success | 2026-10-18T10:55:58Z／2026-10-18T10:56:25Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189716/job/117656965202) |
| test-chat-domain | completed／success | 2026-10-18T10:55:59Z／2026-10-18T10:56:27Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189716/job/117656965317) |
| test-pet-domain | completed／success | 2026-10-18T10:55:59Z／2026-10-18T10:56:38Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189716/job/117656965339) |
| test-desktop-pet | completed／success | 2026-10-18T10:55:59Z／2026-10-18T10:58:15Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189716/job/117656965354) |
| test-chat-application | completed／success | 2026-10-18T10:55:59Z／2026-10-18T10:56:33Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189716/job/117656965374) |
| test-server | completed／success | 2026-10-18T10:55:59Z／2026-10-18T10:57:31Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189716/job/117656965375) |
| test-ui | completed／success | 2026-10-18T10:55:59Z／2026-10-18T10:57:02Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189716/job/117656965383) |
| test-shared | completed／success | 2026-10-18T10:55:59Z／2026-10-18T10:56:30Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189716/job/117656965387) |
| test-stats-server | completed／success | 2026-10-18T10:55:58Z／2026-10-18T10:57:33Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189716/job/117656965422) |
| build-dashboard | completed／success | 2026-10-18T10:55:59Z／2026-10-18T10:58:18Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189757/job/117656965196) |
| check-dashboard | completed／success | 2026-10-18T10:55:59Z／2026-10-18T10:57:25Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189757/job/117656965474) |
| test-persistence | completed／success | 2026-10-18T10:55:58Z／2026-10-18T10:59:05Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39121189727/job/117656965015) |

17個jobs目前皆success。Test All/test-ui實際只跑cargo test --locked -p ui，未啟用SSR；build-chat只建置，因此不包含本輪失敗的SSR斷言。test-stats-server未開postgres-integration及include-ignored。workflow原文見 [test](ci-test-workflow.yml) 與 [chat](ci-chat-workflow.yml)。

教學來源commit `6794641e728ad2b03af6cd3db7f256ae16bab9ab`；v1規則blob `8a36e736b7d056d04462afaf5682499285372342`、正文SHA256 `65e6602567330638a702db5fa35e91906a2e748ddbbfb16c16c9e110162c9081`。原生工具限制見 [native-ui-capability.json](native-ui-capability.json)。
