# Day44 本地驗證重核｜2026-10-30

**最後受測產品：`4b1008b95bc3e170381fcd1db84695e421f036bd`**。文件起點`8e291229d7b5e980839c530c328c0224561fd860`；兩庫main原本乾淨，未提交／未追蹤使用者檔案為空。fetch後依序安全快轉產品6b34→e98fbbf→c726de1→4b1008b；後兩次是驗證期間遠端新提交，均先核對差異／乾淨程度。原產品只讀，沒有產品提交、推送或PR。所有測試／補丁在隔離clone；不寫正式DB、不呼叫付費AI。

[初始Git盤點](initial-git.json) · [最新四workspace manifest／lock、入口／caller／migration／CI blob與SHA256](final-main/source-inventory.json) · [實際命令、exit、UTC與工具版本](final-main/commands.json) · [機器摘要](summary.json)。各phase都有獨立source_sha，不能把中間來源的通過追認到最後SHA。

## 最後來源的矩陣

| 範圍 | 4b1008b實測結果 | 精確輸出 |
| --- | --- | --- |
| crates | metadata／fmt／check／clippy；8項通過 | [014](final-main/014.log) |
| Chat | 同上；一般62、SSR62；hydrate WASM通過 | [019](final-main/019.log)、[030 SSR](final-main/030.log)、[031 WASM](final-main/031.log) |
| Pet | 同上；110項、Node12、WASM通過 | [024](final-main/024.log)、[032 WASM](final-main/032.log)、[033 Node](final-main/033.log) |
| Analytics | 同上；一般43＋1ignored；真PG完整63／0ignored通過 | [029](final-main/029.log)、[036 PG](final-main/036.log) |
| Pet＋forward修補r4 | 四workspace metadata、Pet fmt／check／clippy、workspace112、WASM／Node12、Trunk dev建置通過 | [候選完整命令](final-fix/commands.json)、[frontend雜湊](final-fix/frontend-assets.json) |
| Pet舊庫副本 | 合成舊3版→最新4版＋forward第5版；實際本機SQLite唯讀online backup副本升級／冪等；checksum漂移拒絕 | [副本保資料證據](final-fix/actual-copy-preservation.json)；原DB未寫，沒有發布私有資料 |

Clippy exit0不代表零warning；一般Analytics的1ignored不能抵算PG。PG另以`--no-fail-fast -- --include-ignored`真SQLx與Router執行，63通過／0ignored；不是外部瀏覽器全旅程。重跑、SSR、不同feature及候選測試不重複加總為覆蓋量。

## 真正的問題、遠端修正與本輪剩餘修補

e98fbbf新增Pet活動實作但migration呼叫SQLite不支援的NOW()，23項失敗；先修migration後，舊中文代碼fixture另有3項失敗。[原始e98命令](commands.json)、[24測試log](024.log)、[遠端Pet失敗log](ci-job-124087342435.log)。c726de1已在遠端修正NOW與fixture，本地重驗110通過；本文件沒有把舊錯誤寫成最後主分支仍故障。

Analytics原day37空body回415、day39缺CSRF回422；[c726原PG61過／2敗](latest-retry/004.log)、[當時失敗job](ci-latest-analytics-postgres.log)。最後4b1008b採用了兩個fixture修正；本次在最後SHA重新取得PG63通過，不再宣稱主分支仍未修。

**仍需Pet forward資料相容修補**：活動目前儲存walk／sit，顯示為散步／坐下；舊中文活動列需轉碼並重新套坐下優先。新增`20261030140000_normalize_activity_codes.sql`，保留原活動ID／owner／字典／預設及第4版migration checksum；同步修正升級example的代碼斷言。主分支加相同回歸測試會行為Red（舊中文／兩個true仍在）；實際套用完整patch後112通過。[Pet完整教學與逐檔patch](../../teaching/daily/day-44/pet.md)。這個forward方案沒有推送產品。原第4版既有字典主鍵碰撞仍安全拒絕，沒有自動覆寫；既有字典資料全部保留，不宣稱已消除歷史重複代碼。

補丁已真實reverse／`git apply --check`／apply，四workspace metadata及受測完整patch同一來源；原4版migration與四Cargo.toml／Cargo.lock沒有變更。原7ce附件與已在main的Green／Refactor不可再套到4b1008b。

## 暫存清理與環境失敗

實際刪除332個可重建Cargo target/deps執行檔，三批allocated bytes共**24,110,133,248（22.45GiB）**。這是累計實際清理量，部分同路徑重建後再清理，不是淨磁碟增量或332個不同來源檔案。

| 批次 | 數量 | allocated bytes | 先列明／實際刪除 |
| --- | ---: | ---: | --- |
| 初始 | 204 | 12,176,609,280 | [plan](cleanup-plan.json)／[result](cleanup-result.json) |
| 滿碟緊急釋放 | 3 | 654,200,832 | 路徑／大小先記於工具輸出，再刪；[紀錄](emergency-cleanup.json) |
| 已結束重建產物 | 125 | 11,279,323,136 | [plan](rebuild-cleanup-plan.json)／[result](rebuild-cleanup-result.json) |

僅刪未被程序映射、單硬連結、Cargo target/deps內可重建ELF；保留原生pet-server、.env、src、lock、SQLite／PG資料、未知檔與歷史證據。沒有reset --hard或clean -fdx。

滿碟造成c726中間輪Analytics／SSR linker bus error、PG建庫ENOSPC、新回歸測試編譯錯誤：[中間輪](latest/commands.json)、[編譯失敗](latest-fix/red.log)。清理後jobs1重試得到該SHA的SSR62／PG61過2敗和真行為Red，再於最後4b完整重跑。環境失敗不當功能Red或成功；第一次計畫檔因ENOSPC無法完整寫入，緊急三檔先於工具輸出列路徑／大小再刪並補存記錄。

Trunk第一次offline缺wasm-bindgen0.2.129，允許下載匹配工具後成功；本輪最後的Trunk是**dev profile**，沒有宣稱release、native binary或安裝包已建置。版本與全部失敗／重試命令保留，不升級lock。

## CI與未執行

最後來源[CI快照](ci-final-source.json)：{"success": 5, "in_progress": 2}，觀測時間2026-10-30T13:59:41.681381+00:00；來源固定4b1008b，不從舊SHA推定。未重跑／修改遠端CI，in_progress／cancelled另列，不當成功。

未執行：新活動模型native GUI（三隻、真Native X、真OS失敗補償、併發／漫遊／多螢幕）、完整外部HTTP／瀏覽器旅程、native binary／release／三平台安裝、實際舊PostgreSQL備份還原（使用者沒有備份）、最後SHA的Chat實際舊SQLite完整升級、全套件官方最新穩定／完整MSRV Gate。舊A/B人工結果不能追認新活動模型。SQLite驗證僅涵蓋本次合成資料與實際備份副本，不能推定所有舊DB都合格。

**完整Gate2=false**；next_day三案45、FIFO、last_full_comparison均不變。舊6b34證據只存[第十二分包](../../archive/packages-2026-10-30/12-previous-day44-validation-2026-10-30.zip)，被取代文章／入口／狀態／patch只存[第十三分包](../../archive/packages-2026-10-30/13-day44-recheck-superseded-documents-2026-10-30.zip)，前十一包hash不變。現行三案文章與本輪必要證據保留原檔。[精確commit全文讀回](../../teaching/daily/day-44/recheck-verification-2026-10-30.json)。
