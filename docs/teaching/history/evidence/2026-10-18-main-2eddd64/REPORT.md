# 最新 main 寵物 migration 修正與實測

最新 main `2eddd6428cda6c81c53b1e261d0846ebcf56e6a0`；本次只有 root `migrations/20261018100114_update_pet.sql` 改動。工作分支 `codex/sqlite-pet-migration-2eddd64` 的修正 commit `e33971f508e9612770ad72960b62a8b090efd53e`，未推送／合併 main。完整功能驗收仍為 **未通過**。

## main 實測結果

- `DATABASE_URL=sqlite://<專用檔>?mode=rwc cargo sqlx migrate run --source <main>/migrations`：新建 DB 與先套用1853876 migrations的既有 DB 都 exit1，version20261018100114 回 `near "MODIFY": syntax error`。
- `cargo clean -p persistence` 後 `cargo test --locked -p persistence`：0 passed／5 failed，全部在 migration 失敗。第一次未清除套件快取的5通過已排除，原log標記excluded，不能引用其綠燈。
- 清除 persistence／ui、明確 latest-main checkout 執行 `cargo build --locked -p server --features ssr`：編譯成功；以新專用 CHAT_DATABASE_URL 啟動實際 exit1：Migrate/ExecuteMigration/SQLite near MODIFY。尚不能執行最新服務瀏覽器 E2E。
- 根因：Chat persistence 使用 SqlitePool／root migrations，SQLite不支援 MODIFY COLUMN／DROP CONSTRAINT；`pet_owner`只有`id`欄位，沒有pet_owner_id；DROP COLUMN後分號使ADD COLUMN變成獨立無ALTER TABLE語句。只修第一行仍不足。

## 如何修改（完整候選已提供）

保留所有先前已套用migration原檔與checksum，只替換目前失敗、尚未成功套用於SQLite的20261018100114檔案。若其他環境曾成功套用該version或採用不同SQL，須先核對_sqlx_migrations，不能直接改歷史checksum；本次沒有操作正式DB或刪migration紀錄。

候選在SQLx單一transaction中：

1. 檢查pet_owner既有id非NULL／非空字串，以及五張表每筆owner_id有確切主人。缺少映射則CHECK失敗並rollback，不自動猜配、不刪資料。
2. 建pet_owner_new（id TEXT NOT NULL PRIMARY KEY），pet_profile_new與四張附屬表；後者id為INTEGER PRIMARY KEY，對齊原SERIAL的自增整數意圖，pet_owner_id為TEXT NOT NULL、外鍵指向新owner，pet_id維持關聯。
3. 複製全部欄位／資料，保留有效既有ID與時間戳，owner_id明確CAST為文字pet_owner_id；只有原本NULL的寵物／附屬紀錄主鍵由SQLite產生新整數，不假造缺少的主人ID。既有NULL主人ID須人工建立明確對應後重試。
4. 子表先DROP、再DROP舊parent，rename新表。全程不切換foreign_keys；有效資料、NOT NULL與FK在真實SQLx測試中確認。

完整SQL與測試見 [patch](sqlite-pet-fix.patch)、[重現腳本](pet_migration.py)。patch以本次main為parent，可 `git apply --check sqlite-pet-fix.patch` 後套用；先在工作分支及備份的測試DB驗證。此候選只修SQLite migration；Chat搜尋／SSR、Analytics Cookie的舊候選80ba01e另存，未合併進本候選。

## 修正後實測

| 命令／案例 | 結果 | 原始證據 |
|---|---|---|
| cargo clean -p persistence；cargo test --locked -p persistence | 5 passed／0 failed | [log](fix-persistence.log) |
| python3 scripts/acceptance/pet_migration.py：fresh | 真正SQLx migrate成功；owner明確TEXT ID、pet／四張附屬表自增ID、無效FK／缺少owner ID／刪除被引用主人均拒絕 | [log](fix-migration-scenarios.log) |
| 同腳本：upgrade | 先執行舊六份migration再植入有效舊資料；最新SQLx migration成功，owner7轉文字7、pet10／子紀錄20及內容全部保留，foreign_key_check空 | [log](fix-migration-scenarios.log) |
| 同腳本：null_owner_id／orphan_owner | 兩案預期拒絕，原資料與schema完整、版本未記成功、沒有遺留_new表，transaction rollback確認 | [log](fix-migration-scenarios.log) |
| cargo build --locked -p server --features ssr；專用新SQLite啟動 | 建置成功、Server running；GET /health及/api/rooms均200 | [build](fix-server-build.log)、[startup](fix-startup-final.log)、[HTTP](fix-http.json) |

## 尚未通過的產品範圍與證據適用性

main仍未含候選，啟動失敗尚未解除。SQLite修正不等於完整寵物管理、登入／ACL／AI人物UI已實作。Chat名稱搜尋3匹配→2顯示與SSR提示、Analytics Cookie續期／PG migration31→32兼容仍待先前修正與產品驗證。Pet完整GUI／多螢幕／執行中設定持久化及1800秒瀏覽器Session soak未驗證。

本輪僅root migration變動；runtime source／Cargo.lock／Pet核心／Stats PG migrations均與1853876相同。先前原生4案、Cookie／搜尋的證據保留原SHA，沒有宣稱本輪重跑；舊候選105通過不涵蓋新migration。先前證據見 [1853876報告](../2026-10-18-main-1853876/REPORT.md)。

環境沿用Ubuntu26.04.1、Rust/Cargo1.99.1、SQLx0.9.0、Axum0.8.9、Leptos0.8.22、Tauri2.12.4；本次Python SQLite3.46.1，SQLx使用自己的SQLite driver並獨立實測。只使用臨時／專用SQLite；未讀正式秘密、未呼叫付費AI、未改遠端main。

## 文章同步與CI

10個既有Notion頁面（Day18三篇、Day17三篇、三入口及共同現況）已更新並全文讀回；Day18保留四節，子頁與ID保存。既有v1規則正文、next_day各19、makeup_queue及last_full_comparison維持。GitHub文章鏡像、索引及產品文件現況同步，本次內容為程式與驗證結果，沒有新增規則。

最新CI快照如下；精確開始／結束時間、run與job原始資料在 [ci.json](ci.json)，CI不能代替上方啟動驗收。

| job | 狀態／結論 | 證據 |
|---|---|---|
| test-node | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967072/job/117647515589) |
| build-pet | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967141/job/117647515613) |
| check-pet | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967141/job/117647515843) |
| build-chat | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967178/job/117647515736) |
| check-chat | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967178/job/117647515834) |
| test-stats-server | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967113/job/117647515490) |
| test-server | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967113/job/117647515620) |
| test-ui | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967113/job/117647515631) |
| test-ai-core | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967113/job/117647515701) |
| test-chat-domain | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967113/job/117647515703) |
| test-pet-domain | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967113/job/117647515717) |
| test-shared | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967113/job/117647515761) |
| test-desktop-pet | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967113/job/117647515765) |
| test-chat-application | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967113/job/117647515774) |
| build-dashboard | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967139/job/117647515565) |
| check-dashboard | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967139/job/117647515745) |
| test-persistence | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39117967146/job/117647515684) |

觀察時間 2026-10-18T10:08:05.296135+00:00；尚未完成者維持pending。候選未推送，沒有遠端CI。
