# Day43 本地重新驗證｜2026-10-30

產品來源 `d2612fcd154d2132d850086d83a48578abbb07fa`，文件起點 `38337e6852fff68bdc23e453a5e0ad56409fdeea`。兩庫 main 起始均乾淨、未追蹤檔案清單為空；fetch 後產品由 `8b88269` fast-forward 到 `d2612fc`，文件 main 未變。產品只讀、沒有提交／推送；所有測試在隔離 clone，未改正式 DB、lock 或私有檔案。[起始狀態](initial-state.json)。

## 四 workspace／三案測試矩陣

| 範圍 | metadata locked no-deps | fmt | check／clippy | Rust 測試 | 額外驗證 |
| --- | --- | --- | --- | --- | --- |
| 共用 crates | 通過 | 通過 | 重試通過 | 8 passed | 首輪因已刪 target 的連結目標不存在，exit 101；建立目標後重跑三項 exit 0 |
| Chat | 通過 | 通過 | 通過 | 一般 workspace 60 passed | SSR 原樣 exit 101；hydrate WASM 通過；隔離 import 補丁後 UI SSR 21 passed |
| Pet | 通過 | 通過 | 通過 | 99 passed | WASM 通過；Node 10 passed；native/server 單元與 SQLite 測試不等於原生 GUI |
| Analytics | 通過 | 通過 | 通過 | 一般 workspace 41 passed／1 ignored | 隔離 PG postgres-integration、include-ignored：60 passed／0 ignored |

各命令獨立計數，不將不同命令的重疊測試相加當覆蓋量；0 tests 不算功能測試通過。Clippy 命令沒有 `-D warnings`，exit 0 不表示零警告。[計數](test-counts.json) · [原樣命令](commands.json) · [crates 重試](followup-commands.json)。每筆命令有 cwd、exit code、原始輸出檔與時間；工具版本見 008–014.log。Cargo 既有依賴快取可重用，測試命令本輪真正執行，不是沿用舊二進位結果。

## Patch 重現與 Chat 新阻塞

在原文基準 `c6708c42f21ac919e57a379848b4977d27a1c3d6` 的新 clone，按 Chat→Pet→Analytics、各案 Refactor→Green 真正執行六份原 patch 的 `git apply --check` 與 `git apply`，全部 exit 0。原候選 Chat／Pet／Analytics fmt 仍 exit 1，共用 crates fmt 通過；本輪未在原候選重跑完整 Cargo／Red。不可將套用通過說成原附件完整驗收。

最新 main 已含 Day43 實作與先前 Chat 正式清除搜尋修補；不重複正向套用原 patch。新 Router fixture 漏匯入 `Route`、`Routes`、`path`：原樣 SSR 編譯 exit 101，與新 SHA 遠端 CI 相同。一般 default-feature 測試成功不覆蓋 SSR 編譯。

[最小 import 補丁](chat-fix/chat-ssr-imports-d2612fc.patch) 只改 Chat 測試檔，保留全部既有斷言；在另一隔離 clone 真正 check／apply，SSR UI 21 項與 fmt 通過。[補丁命令](chat-fix/commands.json)。僅交付補丁，不修改產品、不建立產品 PR，遠端 CI 尚未套用這個新補丁。

## DB／HTTP／GUI 範圍

[DB 命令](db/commands.json)：Chat／Pet 以前兩份 migration 建立有資料舊 schema，再用本次來源完整 migrations 升級；資料保留、再次執行冪等、checksum 被竄改時拒絕且資料仍保留，均 exit 0。Analytics 相同合成舊 schema 流程保留管理員雜湊與 2 事件，另真實 pg_dump／pg_restore 到獨立 DB 後升級，exit 0。[探針來源](probes/upgrade_probe.rs)。所有資料庫位於隔離 work；PG 在專用 55482 埠，本輪啟動並停止。

這些是有資料的合成舊 schema，並非使用者實際舊庫。本輪沒有重跑使用者本地 DB 備份升級；使用者尚無實際舊 PG 備份，該項仍待驗。HTTP／WS／授權涵蓋範圍以 server 的本輪測試輸出為準；未另外啟動產品服務做外部 HTTP smoke 或真瀏覽器操作。

2026-10-29 Pet 人工 A/B 原生回籠生命週期驗收維持 [當時 8b88269 的證據](../pet-native-2026-10-29/README.md)，本輪未重跑原生 GUI、瀏覽器或 release 安裝，不追認到新 SHA。未跑完整版本升級／遞迴 MSRV Gate；本次沒有升級相依或更動 lock。

## 精確來源與 CI

[78 個 manifest／lock／入口／migration／CI／Refactor 模組的 blob 及 SHA256](source-inventory.json) · [八份 Cargo manifest／lock SHA256](manifest-hashes.json) · [來源增量](product-diff-since-previous.patch)。來源只改 Chat UI／SSR fixture 與 Pet UI 格式，四 workspace、三案 DB 與 CI 分工不變。

GitHub connector 的 commit workflow 查詢回空，因此另讀 GitHub 公開 API；不將空結果認作沒有 CI。[本輪 CI 快照](ci-final.json)：5 success、Test All failure、Rust AI Chat in_progress。失敗為 [Test All / test-ui](https://github.com/williechen/rust_ai_ring/actions/runs/40930806704/job/123648409646)，缺少上述 imports；[完整 job log](ci-test-ui.log)。這不是昨天 attempt #2 的失敗紀錄。本輪只讀 CI，未 rerun。

## 清理與進度界線

第一批只刪隔離 `work/latest/chat_apps/target/front`、`work/latest/crates/target`、`work/__pycache__`，配置大小共 1,640,591,360 bytes。[清理紀錄](cleanup.json)。重建後空間不足，第二批刪 8 個本輪開始前的舊 Chat 測試執行檔：未被程序映射、無其他硬連結，共 1,565,794,304 bytes。[刪前清單](cleanup-old-tests-plan.json) · [實際結果](cleanup-old-tests-result.json)。總清理配置大小 **3,206,385,664 bytes（約 2.99 GiB）**；其後驗證有重建，不將此值當磁碟淨增量。未刪 .env、任何 DB／備份、lock、src、歷史證據或用途不明檔案；原生 GUI 用執行檔保留。

Gate2 未全部通過。三案 next_day 保持 44、FIFO 與 last_full_comparison 不變，不新增 Day。每案文章分開提交，原文與已發布修訂保留；目前入口指向本次增量修訂。
