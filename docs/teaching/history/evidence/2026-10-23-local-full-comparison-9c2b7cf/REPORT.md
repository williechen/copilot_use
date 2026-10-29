# 三專案全專案比對｜2026-10-23

**判定：full_comparison_success=false。** 已將兩個儲存庫最新 main 複製到本地，完成全檔案 byte/hash 清冊、三案主要呼叫鏈與全部 18 migrations 的檢查、Cargo.lock 579 個 package 清冊及 31 項依賴／工具官方版本查詢，並實跑 Rust、PostgreSQL、WASM、HTTP／WS 測試。發現實際缺陷且仍缺完整 GUI、跨平台、逐篇語意驗收及 Clippy 證據，故不得稱為「完整成功」。文件交付、來源盤點、程式驗收分開。

本次只新增此審查目錄的報告與證據，不變更程式、需求、教學原文、manifest、補教結案或 learning-state。`next_day` 三案均保留 **31**；`last_full_comparison` 保留 **2026-10-16T22:47:14.399027+08:00**。

## 1. 精確來源與本地副本

| 來源 | main SHA | Git tree SHA |
|---|---|---|
| williechen/rust_ai_chat | `9c2b7cf5a4c60a08254445911916c4ed9b02d7b3` | `46f608515c8300b73064f37a2b436c1ea7953c8d` |
| williechen/copilot_use | `9ad1226ea21bb8c367cae1b091ee1ebe2004bd63` | `603fe068cb6ae0d25b8146289f9d7bee36078799` |

起始透過 GitHub branch API 及本地 git clone 互核。兩庫 recursive tree 均未截斷；**tree entry 含目錄，不能當檔案數**。本地 tracked blobs 分別 **179、2016**，逐檔以 Git blob SHA-1 與 SHA256 核對，全部相符，詳 evidence archive 的 `rust_ai_chat-inventory.json`、`copilot_use-inventory.json`。取得全部 bytes 不等於每個檔案逐行語意審查或產品驗收。

本次副本在本工作目錄的 `work/rust_ai_chat`、`work/copilot_use`。程式庫 push URL 設為 `disabled://read-only`，沒有推送、修改 tracked source、lock 或 migration。編譯使用既有外部 Cargo target cache；SQLx 測試另在 crate 內自動建立 ignored target fixture 目錄，已將本次 fixture 移至外部 `work/sqlx-fixtures`。測試只使用新建本機 SQLite 與隔離 PostgreSQL cluster／database，沒有操作正式資料、讀取正式秘密或呼叫付費 AI。

證據時間採 Asia/Taipei；CI 原始時間保留 UTC。報告提交 SHA 由提交後的 `READBACK.json`／交付回覆提供，不能在本提交內自我引用其尚未生成的 commit SHA。

## 2. 覆蓋與呼叫鏈

| 範圍 | 本次核對 | 判定／限制 |
|---|---|---|
| Chat UI | `crates/ui/src/{lib,chat_socket,room_api}.rs`、CSS、SSR/hydrate features | Leptos 入口、登入、列表、搜尋／刷新、WS adapter 已有；完整 browser E2E 未重跑 |
| Chat HTTP／WS | server 的 lib/state/http/ssr/hub/ws/signup/signin/logout/auth_session/room_auth 與測試 | 真實服務、註冊／登入／登出、匿名 POST、WS Upgrade／送收／跨房檢查已實跑；逐房 ACL 缺失 |
| Chat 純層／SQLx | chat_application、repository、product_event、chat_domain、shared/auth、persistence | 建房／先寫後播接線存在；會員身分未傳入 ChatService；history/outbox/來源上報未接線 |
| Pet | 全部原生 source、frontend bridge/index/main/components/CSS、pet_domain、capabilities／Tauri config | SQLite/FSM/native Rust tests、WASM check 已跑；真 Tauri GUI／OS 失敗注入、三窗、重啟未完整驗收 |
| Analytics | main/bootstrap/password_login/dashboard_ui/pg_store/lib/admin、全部 integration targets | 正式入口使用 password_login::build；舊 admin::build 是不同路徑，不能借用其成功測試代表正式登入限流／事件類型 UI |
| DB | Chat 9、Pet 3、Analytics 6 migrations；歷史 checksum 比對 | 全新測試 DB／migration 及 PG 全 targets 已跑；完整歷史版本升級矩陣未完成 |
| 設定／建置 | 12 workspace manifests、lock、toolchain、6 workflows、deploy placeholders、環境範例、README | 已核對真命令／feature；正式部署已取消，placeholder 不作產品能力 |
| 文件 | COMMON_LEARNING_RULES、P9_TARGET、SYSTEM_ARCHITECTURE、UI_ALIGNMENT_REQUIREMENTS、FUTURE_WORK、USER_FLOWS、AUTHENTICATION、狀態／manifest／三案首頁、Day30 三文 | 需求與實作、交付與驗收分開；歷史／現行 87 文章檔案已建立 byte／source-path 自動清冊，**未逐篇重新執行全部教學 Green、逐段語意驗收** |
| 其他資產 | icon、public 舊頁、vendor／舊 copilot_use 程式 | 全 bytes 清冊；未逐張圖像人工驗收，舊庫程式不作主體產品測試 |

主體鏈：Chat Leptos → Axum → chat_application/domain → SQLite Adapter 已部分接通；會員認證 SQL 仍在 server module，沒有完整共用人員驗證 Application Port。Pet Leptos → JS bridge → Tauri → PetStore／PetMachine → SQLite 存在，但建窗／DB 補償仍混合於 command workflow。Analytics 正式 HTTP handler 直接結合 SQLx、認證與頁面，pg_store 獨立處理事件；尚無完整角色／Application 分層。不得因目錄／trait 同名認定架構全面符合。

## 3. 已確認問題、證據與追蹤

以下路徑均指 rust_ai_chat 上列固定 SHA，行號為該版。P1 表示優先處理的授權／資料一致性問題，P2 為功能、測試與文件缺口；本輪沒有修改程式。

| ID | 優先 | 證據／觸發 | 結果與必要後續 | 對應既有需求／教學 |
|---|---|---|---|---|
| C01 | P1 | `apps/server/src/lib.rs:62`，真 HTTP 無 Cookie POST /api/rooms | 實際 **201** 且 SQLite 新增；預期 401。只保護 POST，保留公開 GET；另驗 Session、CSRF、ownership | Chat Day30 r3／Session ACL |
| C02 | P1 | `room_auth.rs:16` 只驗 active_member；`ws.rs:27` 只查房間；mbr_room/mbr_exclude 無呼叫 | 有效會員不等於該房成員，缺逐房 ACL／exclude／私人房判斷；不能把房間 ID 一致性當成 membership | Chat USER_FLOWS、FUTURE_WORK ACL |
| C03 | P2 | `chat_application/src/lib.rs:20` 固定 AnonymousUser；真 WS 發訊後讀 messages | 已登入訊息存 **anonymous_user / NULL**，無法辨識作者與正確稽核；需把認證身分接入 use case | Chat 作者模型、action log |
| C04 | P1 | WS session 只在 Upgrade 前驗；登出後原 socket Ping 仍 Pong，重新連線 401 | 撤銷只影響新握手；長連線存續期間無再驗。需定義撤銷期限並驗發訊／停用／過期，不把本次 Ping 當完整發訊矩陣 | Session／即時連線生命週期 |
| C05 | P2 | UI `lib.rs:483` SSR test 與 ChatPage | 「歷史載入尚未接通」提示缺失；ui/ssr 測試失敗；history Port/API 亦不存在 | Chat Day16／Day28 SSR 回歸 |
| C06 | P2 | UI RoomList 與 room_api::create_room，signin/account | create_room browser helper 沒有 UI 呼叫者；個人頁僅登出。未登入點房間為 401，未提供保留房間目標的登入導向；沒有登入／管理自動切換 | Chat 首頁→個人頁→建房 |
| C07 | P2 | signin.rs、signup.rs、shared/auth | 有 salted Argon2、hash-only Session、logout CSRF；仍缺帳號持久失敗等待、活動 Session 政策、管理者初始化、改密碼／Email reset、完整共用 auth，不能重教「完全沒有登入」 | AUTHENTICATION／FIFO shared-auth |
| P01 | P1 | `pet_windows.rs:123-125` | DB 設回籠後 window.close 同步失敗直接返回，無 DB 補償；Day30 r2 尚未套用。未原生注入此失敗，不宣稱 GUI defect 已實測 | Pet Day29／30 |
| P02 | P2 | `roam.rs:12` 固定 label "pet"；`pet_windows.rs:35` 為 pet-{uuid.simple} | 原生漫遊尋找不存在的固定窗，不能移動動態生活窗。現有 helper test 仍斷言 "pet"，不覆蓋真建窗 | Pet 漫遊原項 |
| P03 | P2 | lib.rs 的 Destroyed handler、open_control_window；tauri.conf | Destroyed 僅移除 Runtime，不持久化回籠；main 控制窗關閉未攔截成 hide，open_control_window 只取既有 main，關掉後不能重建。需真 GUI 驗證 | Pet 直接關窗／重開控制窗 |
| P04 | P2 | local_pet_selection slot=1；PetStore::active／select | 只能持久化一隻 active 選取，與「可多預設、回籠不取消預設資格」模型不一致；FSM／動畫不持久。三 id Runtime 單測不等於三窗重啟 | Pet 多寵物／預設出籠 |
| P05 | P2 | frontend tests 3 檔＋index.html | CI 與本地原版 **3/3 fail**：VM 未注入 URLSearchParams，尚未到 IPC 斷言。獨立副本加入 `URLSearchParams`、`location`、`window.location` 後 **3/3 pass**，未套用原庫 | Node 契約測試 |
| P06 | P2 | file_organizer.rs::revalidate_scanned_file → sha256_file | revalidate 與 File::open 分離，存在 TOCTOU 競態面；目錄遞迴 likewise path-based。只是唯讀 preview，無 Trash／restore／receipt；不得宣稱破壞性清理已安全完成 | FileOrganizer 原項 |
| P07 | P2 | Pet UI/component、capabilities、config | 單 img＋六姿態 class 存在；五欄設定保存／六功能區／每寵漫遊／右鍵坐下／find-back／installer/updater 未完成；bundle.active=false；app 0.2.16 與 workspace 0.2.28 不同 | Pet UI／封裝／FIFO single-image |
| A01 | P2 | password_login.rs:53、change_page；真 GET /login | 正式登入頁手寫 Html，實際 **2 個 doctype**；改密碼頁亦手寫；Dashboard 才是 Leptos SSR。Day30 r2 尚未套用 | Analytics UI alignment |
| A02 | P2 | password_login::build:479-486；lib::source_kind_summary cfg(test)；admin::build 舊路徑 | 正式 `/api/v1/events/{source}/summary/{event_type}` 實際 **404**，舊／helper tests 成功不能替代；正式 UI 只有來源總數，缺互動事件篩選／SSE | Day14／Dashboard Query |
| A03 | P1 | 正式 login/session 僅 admin；舊 admin.rs attempts 使用另一 router | 帳號／角色／來源 ACL、持久帳號等待、管理／權限稽核、短效服務配對輪替缺失。來源 token 與人員 Session 有分離，不能把舊登入碼限流當正式帳密限流 | Analytics AUTHENTICATION／ACL |
| A04 | P1 | `20261020154237_password_login_session.sql` 先建表；`20261022070000_session_contract.sql:1` IF NOT EXISTS | 全新 PG 跑完六 migration，pg_constraint **沒有 session_hash_length／session_fingerprint_length**。需前向 ALTER／資料檢查 migration，保留歷史檔 | DB schema／Session 契約 |
| A05 | P2 | bootstrap.rs:4 要求 8..128 bytes；正式 login 亦同 | 文件 admin/admin 首次初始化無法原樣成立（admin 5 bytes）；CLI 自訂強密碼存在且已實測。屬明確規格差異，不能自行降低密碼規則 | USER_FLOWS 初始管理者 |
| X01 | P1 | 18 migration 歷史比對，9 個 current path 的內容較首次提交不同 | 過去已套用 DB 有 checksum／升級風險；新建庫成功不能代表舊庫可升級。需保資料的全歷史升級矩陣，禁止清空 DB／改 migration checksum 假裝修復 | migrations 不可變政策 |
| X02 | P2 | persistence/src/lib.rs:351 | long-id 測試訊息說不應接受，實際 `assert!(!result.is_err())` 要求成功。SQLite VARCHAR(40) 無此長度限制；綠燈不能代表拒絕 41 字元 | 資料契約／測試可信度 |
| X03 | P2 | README、deploy、telemetry；程式全樹 | README 仍稱 Chat PostgreSQL、首頁可建房、Stats 無儲存／501，均落後實作。deploy 三檔及 telemetry 為空／placeholder。Chat/Pet 事件 mapping 無上報呼叫者，gRPC/Outbox/OTel/RAG/Agent 不存在完整鏈 | P9／架構／文件 |
| D01 | P2 | learning-state／FUTURE_WORK／COMMON_RULES | 規則週日08:00，state 還 Saturday／next_full_comparison=10/24、FUTURE_WORK 週六。state 部分舊 copilot_use 來源措辭不能作新程式驗收 | 週比對排程與狀態 |
| D02 | P2 | Day30 三文／manifest／daily README／首頁 | Day30 版本節明記 lock/MSRV 未查，manifest 只是 article readback；Chat 正文 r3，但 daily 表仍標 r2；首頁前一篇 Day28 與 Day29 已發布不一致；舊產品端 pet_runtime/proto 描述殘留。此報告不回寫原文／歷史證據 | 文件 Gate1／索引 |

## 4. 本次實測

所有 Cargo 命令均 `--locked --offline`，使用已存在依賴 cache；沒有 cargo update／軟體安裝。全部命令、退出碼與日誌在 evidence archive。

| 命令／流程 | 本次結果 | 範圍 |
|---|---|---|
| cargo test -p server -p ui -p chat-application -p chat-domain -p persistence -p shared -p ai-core -p pet-domain --features server/ssr,ui/ssr --no-fail-fast | **66 passed / 1 failed / 0 ignored**，exit101 | 失敗為 ui SSR 歷史提示；其餘所列 targets 繼續跑完 |
| cargo test -p desktop-pet --no-fail-fast | **49 passed / 0 failed** | 含 SQLite pool reopen／create/select、FSM／原生 helper；非真 GUI |
| DATABASE_URL=隔離PG cargo test -p stats-server --features postgres-integration --no-fail-fast -- --include-ignored | **35 passed / 0 failed / 0 ignored** | 所有 lib/bin/integration targets；含舊 admin ignored test、bootstrap、改密碼 rollback、Session pool reopen、來源與冪等；不同 router 覆蓋分開 |
| cargo check -p ui --features hydrate --target wasm32-unknown-unknown | exit0 | WASM check；未重建／驗瀏覽器資產 |
| cargo check -p desktop-pet-frontend --target wasm32-unknown-unknown | exit0 | WASM check；非原生 IPC／畫面 |
| cargo build -p server --features ssr -p stats-server | exit0 | 本次真 HTTP／WS 使用此新建 binary |
| cargo fmt --all -- --check | exit0 | 全 workspace 格式 |
| cargo clippy -p server -p stats-server -p desktop-pet --features server/ssr --all-targets -- -D warnings | **未完成**，中止 exit255 | 編譯期間可用磁碟降至約432 MiB，主動停止；不是通過，也不能記成程式 lint 失敗 |
| node --test（原版；本地 Node22.22.1） | **0 passed / 3 failed** | 與 CI Node24 相同 URLSearchParams 缺失 |
| node --test（獨立 fixture，只補 VM globals） | **3 passed / 0 failed** | 修正建議可行，不是 rust_ai_chat 已修復 |
| Chat 真 HTTP／WS＋SQLite | signup/login/logout 303、公開GET200、匿名建房201、WS 匿名／偽造401、有效101、訊息落DB、跨房 room_mismatch、撤銷後新握手401、原連線Pong | 詳 runtime.json；沒有測完整過期矩陣／瀏覽器／所有入口 |
| Analytics 真 HTTP＋隔離 PG | 匿名dashboard303、kind summary404、login雙doctype；真 CLI bootstrap 後帳密303→強制改密碼→303→重登→dashboard200 | 詳 analytics-runtime.json；第一次 Python PTY bootstrap 缺 controlling terminal exit1，造成初次401，已用真正 PTY CLI 成功更正，**初次401不是產品缺陷** |
| 新 PG schema 查詢 | 六 migrations success；Session長度約束缺失 | pg-schema.log |

PG 最初啟動失敗是本工作目錄 Unix socket 路徑超長，改用 TCP-only 後成功；保留 pg-server.log，不把基座錯誤當產品錯誤。測試結束已停止此隔離 cluster。

## 5. GitHub CI 精確 SHA／job 覆蓋

查核 `head_sha=9c2b7cf...` 的全部六次 runs，均 created_at `2026-10-23T02:51:29Z`，17 jobs（16 success、1 failure），非他庫或其他 SHA。run/job 明細與時間見 github-ci.json。

| Workflow | Run | jobs | 結論 |
|---|---|---|---|
| Node | [39891849761](https://github.com/williechen/rust_ai_chat/actions/runs/39891849761) | 120213214766 test-node | failure；完整 log 已讀、存檔 |
| Test All | [39891849769](https://github.com/williechen/rust_ai_chat/actions/runs/39891849769) | 9 個各 crate tests | success；ui job120213215355 命令無 ssr，只跑8 tests |
| Rust Dashboard | [39891849841](https://github.com/williechen/rust_ai_chat/actions/runs/39891849841) | check/build-dashboard | success |
| Persistence | [39891849758](https://github.com/williechen/rust_ai_chat/actions/runs/39891849758) | 120213214555 | success；PG feature 有跑，但1 ignored；本次已另補跑 |
| Rust Desktop Pet | [39891849805](https://github.com/williechen/rust_ai_chat/actions/runs/39891849805) | check/build-pet | success；編譯不是 GUI |
| Rust AI Chat | [39891849782](https://github.com/williechen/rust_ai_chat/actions/runs/39891849782) | check/build-chat | success；SSR check 不等於 SSR tests |

CI 的 postgres:latest、stable、cargo install 工具未全部 pin；部分 Pet workflow workspace build/check 無 --locked。保留 workflow 原樣，只記錄重現性風險。既有 CI 成功不替代 Day30 未套用提案的 Green，也不代表全產品通過。

## 6. 官方版本／相容性

查核日期2026-10-23。`versions.json` 對 **31項直接／建置依賴與工具**，逐項列 manifest requirement、lock 中同名 resolved（可能含舊 transitive major）、registry 最新未撤回 stable、發布日、宣告 MSRV、官方 API URL。排除 prerelease；沒有以 manifest major/minor 冒充精確版。579個 lock package 在 lock-packages.json；未逐一宣稱所有 transitive dependency 都是最新。

主要直接 resolved 均含當日最新 stable：Leptos0.8.22／leptos_axum0.8.10／router0.8.17、Axum0.8.9、Tauri2.12.5／tauri-build2.7.4、SQLx0.9.0、Argon2 0.6.0、Tokio1.54.1、uuid1.28.0、sha2 0.11.1。官方資料與MSRV見各 [crates.io API](https://crates.io/api/v1/crates/sqlx) 欄位，SQLx宣告1.94.0、Tauri1.90、Leptos1.88；本機Rust1.99.1高於這些要求，native tests／兩WASM checks 已通過相應編譯，但完整功能仍如上失敗／缺證。any_spawner、dotenv、serde-wasm-bindgen 的 registry MSRV 空值保留未知，不能填成零或猜測。

| 工具／DB | 本次環境／來源 | 官方最新 stable | 判讀 |
|---|---|---|---|
| Rust／Cargo | 1.99.1；repo channel=stable | Rust1.99.1，[官方公告](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)／[channel manifest](https://static.rust-lang.org/dist/channel-rust-stable.toml) | stable 未 pin，未安裝升級 |
| PostgreSQL | 原生18.6隔離cluster | [18.6](https://www.postgresql.org/support/versioning/)；19RC1排除 | 新建庫與PG tests已跑 |
| SQLite | Python probe3.46.1（**不是SQLx runtime版本**） | [3.54.1](https://www.sqlite.org/changes.html) | SQLx 實際 SQLite runtime版本未直接量測，不能以Python版本代替；仍是完整版本驗證缺口 |
| cargo-leptos | 0.3.11 | 0.3.11，crates.io | 同版；本次未重跑全資產release build |
| Trunk | 0.21.14 | 0.22.0，MSRV1.96.1 | 本機非最新，未升級；WASM cargo check不能替代最新Trunk整包驗證 |
| SQLx CLI | 本次未使用／未確認安裝 | 0.9.0，MSRV1.94.0 | DB 由程式sqlx migration／tests驗證 |
| Node | 本地22.22.1；CI24.21.0 | [26.12.1 Current、24.22.1 LTS](https://nodejs.org/dist/index.json) | 本次未下載升級；CI測試基座問題與版本差異分開 |
| Native GUI libs | GTK3.24.52、WebKitGTK2.52.6、GLib2.88.0 | 未完成各上游最新穩定版全面核對 | 本機 native test 可連結，不代表三平台／最新版相容 |

## 7. Node VM 修正建議（僅建議／副本驗證）

三個 `apps/desktop_pet/frontend/tests/*.test.mjs` 都要加入 import，並以以下取代原 `vm.runInNewContext(script, { window });`。不需安裝 npm 套件。官方：[VM context](https://nodejs.org/api/vm.html#vmruninnewcontextcode-contextobject-options)、[URLSearchParams](https://nodejs.org/api/url.html#class-urlsearchparams)。

```js
import { URLSearchParams } from "node:url";

// 放在既有 window mock 建立之後。
const location = { search: "" };
window.location = location;
vm.runInNewContext(script, { window, location, URLSearchParams });
```

獨立 fixture 3/3 pass，仍須由使用者套用後以新 rust_ai_chat commit 跑 CI；沒有修改原庫。

## 8. 完整成功的剩餘門檻

1. 修正並由使用者提交主體程式缺口，特別是C01／C02／P01／A04，重新固定新SHA；唯讀審查不能替使用者套用。
2. 全套SSR測試、Node CI、Clippy通過；補真WS過期／停用／撤銷存續與授權矩陣、全部HTTP／GUI E2E，而非只有helper。
3. 三個不同pet id的真原生視窗／每幀單圖／回籠失敗注入／控制窗重開／App重啟／多螢幕與Windows/macOS；本次未以模擬web頁冒充原生GUI。
4. 所有歷史DB版本的資料保留升級／checksum、rollback／備份恢復，並直接量測SQLx SQLite runtime。
5. 87篇歷史與現行教學逐段語意、Green套用與相應Gate重新驗證；本次article inventory只驗檔案與source path，不是逐篇完成聲明。現有歷史84篇矩陣只作其原範圍證據，未冒充本輪重跑。
6. 剩餘上游native版本／未宣告MSRV與工具鏈相容性；P9上層能力逐項規格→呼叫者→實作→測試映射。P9產品全部實作與週比對「查核完成」不同，但未審完範圍不可記完整成功。

報告提交成功只代表審查證據交付。只有所有必要查核已完成且有真實全面證據，才另以最新branch SHA條件更新成功時間；本次不更新。

## 9. 證據索引與提交保護

- `evidence.tar.gz`：logs下完整原始測試／CI／runtime／PG schema、兩庫逐檔SHA清冊、579 lock package、18 migration history、87 article inventory、版本／環境，以及本次檢查腳本。未含資料庫、正式秘密或編譯cache。
- `SUMMARY.json`：精確來源、結果、不可變狀態及證據archive SHA256。
- 發布前重取兩庫main；若程式SHA或文件base變動，停止直接發布並重新核對；以copilot_use expected_sha lease、force=false更新main，禁止覆蓋他人變更。
- 發布後從GitHub精確新commit重新讀取全部報告bytes及blob SHA，核對SHA256；另fetch新commit比較parent與changed paths。讀回證據在本地READBACK.json，交付回覆列明精確commit及結果。
