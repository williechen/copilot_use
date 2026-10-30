# Day44 本地正式驗證｜2026-10-30

產品來源 **`6b34c42f74fc1585b1b864fb7dd6969318270e0f`**；文件起點 `dbde2846fdcad3ce2168ec6852f64dcdd3de9bf6`。產品只讀：乾淨 main 從 `7ce39f0166a2120539e5908b33c88322a930c35a` fast-forward 到此 SHA，沒有產品 commit／push。兩庫 main、remote、完整 tree、未提交／未追蹤清單見 [原始命令](commands.json) 的 000–007；皆無使用者改動。發布前fetch及產品FF reflog見[後續核對](prepublication-commands.json)。隔離可寫 clone 承載補丁與測試，沒有改正式 DB。來源清單的 [77 個 blob／SHA256](source-inventory.json) 覆蓋四 workspace manifest／lock、入口、caller、features、migration、SQLx／Leptos／Tauri 及 CI。

## 四 workspace 與三案矩陣

| 來源／範圍 | 實際結果 | 命令／原始輸出 |
| --- | --- | --- |
| 主分支 crates | metadata --locked --no-deps、fmt、check、clippy、test 通過；8 tests | [commands](commands.json)，029–033 |
| 主分支 Chat | 同上；62 tests；SSR server＋persistence＋UI 62；hydrate WASM 通過 | commands 034–038、[SSR](049.log)、[WASM](050.log) |
| 主分支 Pet | 同上；100 tests；WASM／Node 10 通過 | commands 039–043、[WASM](051.log)、[Node](052.log) |
| 主分支 Analytics | 同上；43 tests＋1 ignored；一般測試沒有執行 PG | commands 044–048 |
| 主分支 Analytics PG／正式 Router | Day44 CSRF 真 PG 測試1通過；全套找到 day37 415、day39 422（期望303） | [no-fail-fast 全套](pg-followup/commands.json)；[首次失敗](055.log) |
| Analytics＋兩個 fixture 修補 | 真 Dashboard 取 CSRF，再送 form；PG 全套63通過／0 ignored | [重試命令](pg-fixture-retry/commands.json)；[補丁](analytics-logout-fixture-fix.patch) |
| Pet＋活動 Green r3 | fmt／metadata／check／clippy、workspace110、WASM、Node12通過 | [完整命令](pet-r3/commands.json)、[強制本地 crate 重建後再驗](green-rebuild/commands.json) |
| Pet 前端正式建置 | Trunk release build 成功；JS／WASM／CSS hashes 保存 | [成功輸出](pet-trunk-retry.log)、[資產清單](frontend-assets.json)、[命令](manual-commands.json) |
| Pet 同測試 Red→Green | 基準6b34行為斷言失敗（無兩活動）；同一測試在候選通過 | [原測試](day44_creation_red.rs)、[Red](red-green/red.log)、[Green](green-rebuild/00.log) |
| Pet 舊庫升級 | 合成舊資料＋實際本機 SQLite 唯讀 backup 的副本：真 Migrator、保資料、冪等；checksum 漂移拒絕 | [命令](pet-upgrade/commands.json)、[非活動資料前後 hash 相同](pet-upgrade/actual-copy-result.json) |

Clippy exit0 不表示沒有 warning。上述數字按 suite 分列，SSR／重跑／Red→Green 不重複加總為覆蓋量。0 tests 不計功能通過。PG 是專用 localhost:55482 cluster／測試 DB，SQLx 真交易與 Router request，並非正式 DB、外部瀏覽器或完整人工旅程。候選不是主分支實作。

## Patch 前置與真套用

原文基準 `7ce39f0166a2120539e5908b33c88322a930c35a`：Chat Refactor→Green、Analytics Refactor→Green、Pet Refactor 的五個原附件均實際 `git apply --check`／`git apply`，見 commands 017–026。原 Chat／Analytics 合成後 fmt 未通過；已附各案 format 修補，真 reverse／check／apply 後 fmt 通過，Chat SSR server lib29＋UI21通過，見 [candidate-format](candidate-format/commands.json)。

最新 `6b34c42f74fc1585b1b864fb7dd6969318270e0f` 已含 Chat／Analytics Day44 功能及三案 Refactor（Chat 實際模組 `room_patch`，原文重現為 `room_path`），不要重複套用舊附件。Pet 新 Green 有兩個**擇一**基準：最新6b34用 `pet-day44-activity-green-r3.patch`；7ce＋原 Pet Refactor 用 `pet-day44-activity-green-r3-after-refactor.patch`。[真 check／apply、四 workspace metadata、實際套用後 Pet10項整合測試](final-apply-r3/commands.json)；[最新基準的14檔套用結果與受測來源 SHA256 完全一致](final-apply-r3/applied-source-equality.json)。四 Cargo.toml／lock 沒有變更。

## 失敗與修正均保留

- Analytics 原產品 PG 失敗是舊 fixture 缺少正確表單／CSRF，不是把新防護移除。修補保留 Cookie 歧義、Session 撤銷、CSV 等有效斷言；遠端產品未修。
- fixture 首次完整編譯 link bus error（磁碟壓力）保存於 [pg-fixture-fix](pg-fixture-fix/commands.json)；清理可重建產物、jobs1 重試才取得63通過。
- Pet Trunk 首次 offline 缺 wasm-bindgen0.2.129，見 [失敗](pet-trunk-build.log)；允許下載匹配工具後 build 成功，未改 lock。
- Pet 最新基準補丁直接套到7ce＋Refactor曾失敗，見 [final-apply](final-apply/commands.json)。沒有忽略 hunk；另產生明確基準的 after-refactor 版本才成功。
- 同一 Pet Green 測試第一次因共用 target 殘留基準版依賴而編譯失敗，見 [red-green](red-green/commands.json)。強制本地 Rust 來源重建（內容不變）後同測試、workspace110及WASM全部通過，見 [green-rebuild](green-rebuild/commands.json)。未將編譯錯誤當功能 Red。

## 暫存清理

先列路徑／大小／可重建判斷，再刪除已結束、未被程序映射、單硬連結的 Cargo target/deps 測試執行檔。三批共90項，實際釋放 allocated bytes **14,172,524,544（13.20 GiB）**；後續編譯會重新使用空間，這不是淨磁碟增量。

| 批次 | 刪除項目 | allocated bytes | 前置清單／實際刪除 |
| --- | ---: | ---: | --- |
| 首批 | 48 | 8,720,592,896 | [plan](cleanup-plan.json)／[result](cleanup-result.json) |
| Chat 後續測試產物 | 26 | 3,475,849,216 | [plan](cleanup-chat-plan.json)／[result](cleanup-chat-result.json) |
| Analytics 後續測試產物 | 16 | 1,976,082,432 | [plan](cleanup-analytics-plan.json)／[result](cleanup-analytics-result.json) |

未刪除 .env、SQLite／PG 資料、Cargo.lock、src、歷史證據、未知未追蹤檔或保留的 Pet 原生執行檔。沒有 reset --hard／clean -fdx。真 DB 及編譯二進位不發布 GitHub；前端只保存雜湊。

## 遠端 CI 與未執行

[固定 SHA 的七 workflow](ci.json) 六成功、一失敗：Persistence 的 analytics-postgres 失敗；[job清單](ci-failed-jobs.json)、[完整失敗日誌](ci-analytics-postgres.log) 與本地 day37 的415吻合。本次沒有重跑 CI，候選 fixture 本地63通過不代表遠端綠燈。

未執行：新 SHA 的真瀏覽器／外部 HTTP 全旅程、Pet 新模型 native GUI（三隻／原生X／併發／真 OS 故障補償／實際漫遊）、native binary／安裝包三平台驗收、實際舊 PG 備份還原（使用者目前沒有備份）、新 SHA Chat 實際舊 SQLite 全升級、所有套件官方最新穩定與跨平台 MSRV 全量相容性。Pet 舊庫副本升級不代表 GUI 或所有使用者 DB 都合格；原旗標曾混用，無法由現值還原遺失的歷史預設意圖。

工具版本在008–014與 resolved-versions.json；CLI／native另見prepublication-commands.json（tauri-cli2.12.3、WebKitGTK2.52.6、GTK3.24.52）：Rust1.99.1、四 lock 的SQLx0.9.0／Leptos0.8.22、Pet Tauri2.12.5，未升級依賴。官方對照：[Rust releases](https://blog.rust-lang.org/releases/)、[SQLx](https://docs.rs/crate/sqlx/latest)、[Leptos](https://docs.rs/crate/leptos/latest)、[Tauri](https://docs.rs/crate/tauri/latest)。本次執行的 Linux／WASM 相容性不代表完整版本 Gate。

**完整 Gate2 未完成**。Pet r3 為已實測、可套用的文件方案，未套用產品。三案 next_day 維持45，FIFO不變；last_full_comparison仍為2026-10-16T22:47:14.399027+08:00。[教學現況](../../teaching/current/STATUS.md)。

## 現行文件修正範圍

三案Day44完整正文、ENTRY／Day44總覽、七首頁與current入口、STATUS／manifest／learning-state／validation／CI、SOURCE_LAYOUT／架構／啟動／驗證、兩份需求過時SHA說明、FUTURE_WORK、各案重構進度與跨案工程狀態均對齊此次來源。原版本只存第十一差異包，前十包hash不變；目前未完全驗收的Day44文章保留原檔。精確文章提交與讀回見[修訂證據](../../teaching/daily/day-44/revision-verification-2026-10-30.json)。
