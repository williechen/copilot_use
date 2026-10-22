# Day18 正式複驗與文章統整（最新 main 794da6e）

記錄時間：2026-10-18T17:07:46.803625+08:00（Asia/Taipei）。正式完整功能驗收仍為 **未通過**。

來源 main `794da6e57a6490d580a4c49789519998f941b4c2`；工作分支 codex/day18-functional-acceptance 的修正候選 `8e8469989642aa7ca6ab329f0bd95b3ac0fd170d`，未推送／合併遠端 main。教學分支本次更新前 `d9ee7f1994b4b115dc81a34e27bb607c5b139c8e`。

## 實際結果與必要修改

- Chat：最新 main 的 SSR/WASM 乾淨建置已通過，舊 abebbef 的 E0382 已解除。中文含空白、空白查全部通過；但完整房名查詢會先取第一筆完全匹配，API 的 3 筆名稱包含匹配在瀏覽器只列 1 筆。候選改為 list_rooms → 既有 filter_rooms，實測完整保留 3 筆與正確 IDs。SSR 仍有缺少「歷史載入尚未接通」提示的失敗；候選恢復提示，沒有宣稱歷史已實作。
- Pet：main 已選 pet、控制窗操作仍選 main。相同 roam_desktop 核心的真實 Tauri/X11 座標四案再次通過（啟用、停用、邊界、關閉 pet）；main 均維持 (100,100)，邊界自 (1080,600) 向內反彈。普通啟動時序、手動 GUI、拖曳／睡眠／執行中設定／持久化、真實多螢幕與多寵物仍未驗證。
- Analytics：main 的 server touch_session 已套用，但活動回應仍沒有刷新 Cookie。清除 stats-server package 後重建 CLI，main HTTP 為 COOKIE_RENEWED: False；候選重新清除並重建後為 True，Origin／CSRF／來源 token 分離／PG／logout 撤銷通過。帳密、角色／來源 scope、立即撤銷與 1800 秒瀏覽器 soak 未完成或未驗證。
- Migration：794da6e 把 20261018073931 改名為 20261018073932。本輪先在獨立 DB 套用舊 migration，再執行新 migration，實測「migration 20261018073931 was previously applied but is missing in the resolved migrations」。這是既有資料庫升級 blocker；新建 DB 的綠燈不能代替它。候選尚未修復此項，需要恢復歷史 31 檔／checksum 並依 32 是否已套用處理兼容，兩種既有基線仍待實作重測。本次沒有修改正式 DB 的 migration 記錄。

## 命令與證據

| 範圍／命令 | 實際結果 | 原始證據 |
|---|---|---|
| main：UI package clean 後 cargo leptos build --cargo-locked | SSR/WASM 通過 | [log](main-794-leptos.log) |
| main：cargo test --locked -p ui --features ssr -p stats-server -p desktop-pet | 53 passed／1 failed／1 ignored；SSR 3 passed／1 failed | [log](main-794-tests.log) |
| main：新 SQLite fixture，POST/GET rooms、SSR、瀏覽器查詢 | HTTP 201/200；API 有 3 筆匹配，畫面只列 1 筆 | [HTTP](main-794-chat-http.log)、[fixtures](main-794-name-fixtures.log)、[browser](main-794-browser.txt) |
| main：stats-server package clean，cargo build --locked -p stats-server，python3 day18_http.py analytics | 真實 PG／guard／Origin／CSRF／logout 通過；Cookie False | [build](main-794-stats-clean-build.log)、[HTTP](main-794-analytics-http.log) |
| 升級：先 cargo sqlx migrate run --source 舊 migration，再 --source 最新 migration | 舊版套用成功；新版因 missing applied migration 失敗 | [old](migration-old-apply.log)、[upgrade](migration-794-upgrade.log) |
| 候選：cargo test --locked -p ui --features ssr -p stats-server -p desktop-pet -p server -p desktop-pet-frontend -p workspace-tests | 100 passed／0 failed／1 ignored；含 root tests 20 案及 doc-tests | [log](candidate-794-tests.log) |
| 候選：DATABASE_URL／ANALYTICS_TEST_DATABASE_URL=專用 DB，cargo test --locked -p stats-server --features postgres-integration -- --include-ignored | 新建 DB／schema 路徑 22 passed／0 failed／0 ignored；不代表 migration upgrade 通過 | [log](candidate-794-pg.log) |
| 候選：cargo leptos build --cargo-locked | SSR/WASM 通過；相同 fixture 名稱查詢顯示完整 3 筆 | [build](candidate-794-leptos.log)、[browser](candidate-794-browser.txt) |
| 候選：stats-server package clean 後重建，python3 day18_http.py analytics require-renewal | Cookie True；401／403、PG、CSRF、logout／舊 Cookie 拒絕通過 | [build](candidate-794-stats-clean-build.log)、[HTTP](candidate-794-http.log) |
| Pet：原生 acceptance-probe 四情境 | 4 passed；真實 OS 座標讀回，不是只測 label 字串 | [enabled](pet-enabled.log)、[disabled](pet-disabled.log)、[boundary](pet-boundary.log)、[close](pet-close.log) |

通過數有重複覆蓋，不相加當成唯一案例數。重現命令在 [REPRODUCTION](REPRODUCTION.md)，完整候選改動在 [patch](day18-functional-fixes.patch)。這個 patch 沒有修復 migration 升級兼容，也不代表完整產品通過。

## 環境、版本與範圍

Ubuntu 26.04.1／Linux x86_64；Rust 1.99.1／Cargo 1.99.1；PostgreSQL 18.6；Node 22.22.1；cargo-leptos 0.3.11；Trunk 0.21.14。鎖定 Axum 0.8.9、Leptos 0.8.22、SQLx 0.9.0、Tauri 2.12.4、Tokio 1.53.3、wasm-bindgen 0.2.129。使用專用 cluster loopback 55418、day18_revalidation／day18_upgrade、新 SQLite 與合成憑證。未讀正式秘密、未呼叫付費 AI、未操作私人檔案。

Pet 單一 X11 1280×800，工作區 (0,25,1280,775)，映射後 pet 200×200。探針等待映射後再定位／啟動原 roam_desktop，因此沒有冒充普通啟動時序或手動 GUI 通過；負座標是單元測試，真實多螢幕未測。原核心與依賴自 abebbef 至 794da6e 未變，已核對差異。

候選 TCP 測試跨初始 2 秒期限仍 200，再設測試專用 30ms idle deadline 並真實等 60ms，逾時回 401、不刷新／不復活。這不是 1800 秒瀏覽器 soak。新建 PG 測試與實際升級分列；帳密 admin/admin 輸入仍 422（缺 key），不能把登入碼原型當成帳密／角色系統。

初次共用 target 曾重用另一 checkout 的產物，未歸屬的結果已排除；本報告採相關套件 clean 後、明確 checkout 重建及可驗證行為的結果。這是執行紀錄，沒有增加規則。abebbef 的 E0382／CI 失敗與 794da6e 的編譯通過分開保存，不沿用舊失敗作為目前 blocker。

## 文章、索引與規則核對

既有 3 篇 Day18、3 篇 Day17 的歷史適用範圍、3 個專案入口、共同總頁現況，共 10 個 Notion 頁已更新至 794da6e 並全文讀回。原頁 ID／子頁保存；Day18 維持四節。GitHub 同步三篇 Day18 鏡像、README、manifest、learning-state 的實測紀錄及四份相關產品文件現況。Day15／Day16 原文、既有封存與歷史未執行紀錄未改成當時通過。

v1 規則正文在 Notion 與 GitHub 均逐字核對未變；沒有增加規則、沒有改排程。next_day 維持三專案各 19，last_full_comparison 未改。所有改動是實作說明、候選程式、測試結果、來源與原有產品缺口。讀回時間／標題／子頁保存檢查見 [notion-readback.json](notion-readback.json)，修訂前文章在 *-before.md。

| 既有頁面 | 最後讀回版本 UTC | 連結 |
|---|---|---|
| 2026-10-18｜P0 Chat Day 18｜查詢房間完成 HTTP 接線與錯誤恢復 | 2026-10-18T09:02:50.201Z | [Notion](https://app.notion.com/p/3fde51d45093816bb36dd9099190d907?pvs=204) |
| 2026-10-18｜P0 Pet Day 18｜漫遊只移動生活窗並核對真實標籤 | 2026-10-18T09:02:48.536Z | [Notion](https://app.notion.com/p/3fde51d4509381bfa106ea4d9b190480?pvs=204) |
| 2026-10-18｜P0 Analytics Day 18｜有效活動延長伺服器 Session 並拒絕過期 | 2026-10-18T09:02:46.837Z | [Notion](https://app.notion.com/p/3fde51d4509381c2a0d5f60041137619?pvs=204) |
| 聊天室 | 2026-10-18T09:02:51.914Z | [Notion](https://app.notion.com/p/3f3e51d45093807da638c336e857543a?pvs=204) |
| 桌面寵物 | 2026-10-18T09:02:53.912Z | [Notion](https://app.notion.com/p/3f3e51d450938067923ae5a424d30950?pvs=204) |
| 📊 統計儀錶板｜聊天室 × 桌面寵物 | 2026-10-18T09:02:55.790Z | [Notion](https://app.notion.com/p/3f3e51d45093817eafaee0cfc9ea4cc8?pvs=204) |
| 🦀 Rust AI 三專案｜共同學習規則與前進目標 | 2026-10-18T09:02:57.748Z | [Notion](https://app.notion.com/p/3f2e51d4509381ddb8a8ce7cb4cd3e60?pvs=204) |
| 2026-10-18｜P0 Chat Day 17｜查詢房間接通 HTTP | 2026-10-18T09:03:59.859Z | [Notion](https://app.notion.com/p/3fce51d45093818b913bebb8dab5ab65?pvs=204) |
| 2026-10-18｜P0 Pet Day 17｜漫遊指向寵物生活窗 | 2026-10-18T09:04:02.307Z | [Notion](https://app.notion.com/p/3fce51d4509381ca83e3e825f0313698?pvs=204) |
| 2026-10-18｜P0 Analytics Day 17｜活動延長管理員 Session | 2026-10-18T09:04:04.788Z | [Notion](https://app.notion.com/p/3fce51d4509381ea871ec9c92da37c1b?pvs=204) |

## 最新 SHA 的 CI 快照

觀察時間 2026-10-18T17:05:44.215028+08:00，來源 `794da6e57a6490d580a4c49789519998f941b4c2`；候選沒有遠端 CI。CI 成功範圍與功能結果分列，原始完整內容見 [ci-latest.json](ci-latest.json)。

| Job | 狀態／結論 | 開始／結束 UTC | 證據 |
|---|---|---|---|
| test-node | completed／success | 2026-10-18T08:52:50Z／2026-10-18T08:52:55Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150673/job/117633352219) |
| build-chat | in_progress／None | 2026-10-18T08:52:50Z／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150668/job/117633352185) |
| check-chat | completed／success | 2026-10-18T08:52:50Z／2026-10-18T09:01:10Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150668/job/117633352286) |
| test-pet-domain | completed／success | 2026-10-18T08:52:50Z／2026-10-18T08:53:21Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150682/job/117633352393) |
| test-shared | completed／success | 2026-10-18T08:52:50Z／2026-10-18T08:53:17Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150682/job/117633352627) |
| test-desktop-pet | completed／success | 2026-10-18T08:52:50Z／2026-10-18T08:54:59Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150682/job/117633352628) |
| test-stats-server | completed／success | 2026-10-18T08:52:50Z／2026-10-18T08:54:00Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150682/job/117633352629) |
| test-chat-domain | completed／success | 2026-10-18T08:52:50Z／2026-10-18T08:53:16Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150682/job/117633352648) |
| test-ui | completed／success | 2026-10-18T08:52:50Z／2026-10-18T08:53:58Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150682/job/117633352653) |
| test-ai-core | completed／success | 2026-10-18T08:52:50Z／2026-10-18T08:53:22Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150682/job/117633352663) |
| test-server | completed／success | 2026-10-18T08:52:50Z／2026-10-18T08:54:39Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150682/job/117633352674) |
| test-chat-application | completed／success | 2026-10-18T08:52:50Z／2026-10-18T08:53:20Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150682/job/117633352703) |
| test-persistence | completed／success | 2026-10-18T08:52:50Z／2026-10-18T08:55:39Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150680/job/117633352223) |
| check-dashboard | completed／success | 2026-10-18T08:52:50Z／2026-10-18T08:53:55Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150701/job/117633352268) |
| build-dashboard | completed／success | 2026-10-18T08:52:50Z／2026-10-18T08:55:06Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150701/job/117633352464) |
| build-pet | completed／success | 2026-10-18T08:52:50Z／2026-10-18T09:02:12Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150756/job/117633352608) |
| check-pet | completed／success | 2026-10-18T08:52:51Z／2026-10-18T09:00:48Z | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39113150756/job/117633352902) |
