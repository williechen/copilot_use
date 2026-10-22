# 最新 main 全面重測與文章同步

來源 `fa0888ea1e1ea93415a4ba7f16972938969b1be3`；本地 `/home/willie/Documents/rust_ai_chat` 已 fast-forward 至相同版本。時間 2026-10-18T20:53:35.305857+08:00。**正式完整功能驗收未通過**。修正候選 `2f60c731ca78c689cb78abbd7bea239146ed3aaa`（基底為上述main，候選有多筆commit）保存在本地 `codex/full-acceptance-fa0888e`，未推送／合併rust_ai_chat/main。

## 最新差異與驗收範圍

相對9a95a10，僅新增Chat action log migration；Rust與Cargo.lock沒有改動。Chat、Pet、Analytics Day18切片分別為名稱查詢、只移動pet生活窗、有效Session同時延長server和Cookie期限；完整產品需求另依已讀取的P9_TARGET／FUTURE_WORK／SYSTEM_ARCHITECTURE／UI_ALIGNMENT_REQUIREMENTS及v1規則逐項核對。未增加規則或修改需求。

新Chat六份migrations／20張業務表，Pet三份／9張，Analytics PostgreSQL三份／15張。新建及9a95a10→本版三服務升級通過；新增表27個有效／缺必要欄位約束案例通過。表存在沒有完成服務配對／換證／輪替／撤銷；Rust呼叫端搜尋無結果，實際HTTP／原生啟動後這些表仍空。`token_context`、`local_token`和任意`json_data`欄位尚無低敏記錄或加密策略可驗證，不應當成已安全接線的稽核功能。

## 實際通過、失敗與未驗證

|範圍|實際結果與限制|原始證據|
|---|---|---|
|主線workspace|126 passed／1 failed／0 ignored，exit101；SSR缺少「歷史載入尚未接通」|[log](workspace-tests.log)|
|候選workspace|129 passed／0 failed／0 ignored，exit0；含真PG及短deadline TCP過期測試|[log](candidate-workspace-tests.log)|
|Chat SSR／WASM、Pet release frontend、native／Stats建置|全部成功；最後重新建置並還原主線執行檔，非installer|[Leptos](main-leptos.log)、[Trunk](main-pet-frontend.log)、[native](main-native-build.log)、[restore](restored-main-build.log)|
|主線瀏覽器|完整房名應3匹配只列2；不存在仍成功提示。部分中文3、rUsT4、空白5；503保留5筆及重試通過|[search](main-browser.txt)、[other](main-other-search.txt)、[empty](main-no-match.txt)、[503](main-http-failure.txt)、[retry](main-http-retry.txt)|
|候選瀏覽器|完整3筆與明確空結果提示通過；實際載入候選SSR／WASM|[search](candidate-browser.txt)、[empty](candidate-no-match.txt)|
|真WebSocket|ping/pong、跨房拒絕、空白拒絕、同房typing、兩客戶同訊息ID、他房無洩漏、重連共7案；SQLite row讀回|[WS](ws.json)、[row](ws-persistence.json)|
|Analytics實際HTTP|匿名Dashboard303、query／來源token人員query401、Origin／CSRF403、fixture key登入303、攝取201／去重200、PG summary200、登出303與Cookie撤銷401通過；主線Cookie活動續期assert exit1，候選exit0|[main](restored-main-http.log)、[main Red](restored-main-cookie-red.log)、[candidate](candidate-analytics-http.log)|
|Schema／新建|既有33案＋新表27案通過（不能與workspace相加當唯一測試總數）；三服務fresh migrations成功|[33](schema.json)、[27](new-schema.json)、[SQLite](migration-results.json)、[PG](pg-schema.log)|
|資料庫升級|9a→新版三服務通過；舊Chat419／791仍missing91733；舊Stats31仍missing31，實際啟動exit1 VersionMissing(20261018073931)|[Chat9a](chat-upgrade9a.log)、[Pet9a](pet-upgrade9a.log)、[PG9a](pg-upgrade9a.log)、[oldChat](old419-new.log)、[oldPG](pg-upgrade-new.log)、[startup](main-stats-old-db.log)|
|Pet實際OS幾何|enabled／disabled／boundary／close四案通過；單X11 1290×800工作區0,25,1290,775，pet200×200，main全程100,100；disabled pet300,300不動，邊界向內反彈，關閉pet無fallback main|[enabled](pet-enabled.log)、[disabled](pet-disabled.log)、[boundary](pet-boundary.log)、[close](pet-close.log)|
|帳號／產品功能|/signin及/api/messages404；Analytics username/password422缺key；完整帳號／ACL／角色／scope／立即縮權／歷史未完成或未驗證|[HTTP](additional-http.json)|
|未執行gate|native CUA apps空；完整手動GUI／拖曳／執行中設定／多螢幕／多寵物未驗證；探針等待映射後才啟動漫遊，不驗普通啟動漫遊時序；1800秒browser soak未執行；跨平台／真AI未測|[capability](native-ui-capability.json)|
|休眠測試target|workspace-tests未宣告postgres-integration，實際指定feature exit101；通常0 tests不可作通過證據，Stats所屬PG測試確實已執行|[error](dormant-pg-test.log)|

名稱搜尋目前是比較room_name完全相等，**不是room ID**；問題是先篩出完全同名後，既有contains filter拿不到「Rust 技術交流延伸」。候選改用list_rooms取得完整資料後再套現有filter，並恢復SSR限制提示。Session候選在成功受保護GET後重新檢查未撤銷且未過期，再一起touch及Set-Cookie；TCP測試活動跨原2秒期限仍200，再將fixture deadline30ms並等60ms，回401不復活。生產TTL1800未改，不能冒稱30分鐘實時瀏覽器驗收。

## 測試命令與環境

Ubuntu26.04.1、Rust／Cargo1.99.1、SQLx0.9.0、Axum0.8.9、Leptos0.8.22、Tokio1.53.3、Tauri2.12.4、cargo-leptos0.3.11、Trunk0.21.14、PostgreSQL18.6；本輪WS實用Node22.22.1，非前輪bundled24。[版本記錄](versions.json)。未另驗MSRV。主線／候選採相同命令，checkout不同：

```sh
env -u OPENAI_API_KEY \
 ANALYTICS_DATABASE_URL=postgres://verify63@127.0.0.1:55463/verifyfa \
 ANALYTICS_TEST_DATABASE_URL=postgres://verify63@127.0.0.1:55463/verifyfa \
 DATABASE_URL=postgres://verify63@127.0.0.1:55463/verifyfa \
 ANALYTICS_CHAT_TOKEN=fixture-chat ANALYTICS_PET_TOKEN=fixture-pet \
 cargo test --locked --workspace --exclude desktop-pet-frontend \
 --features ui/ssr,stats-server/postgres-integration --no-fail-fast -- --include-ignored
cargo fmt --all -- --check
cargo clippy --locked --workspace --exclude desktop-pet-frontend --all-targets \
 --features ui/ssr,stats-server/postgres-integration -- -D warnings
cargo leptos build --cargo-locked
(cd apps/desktop_pet/frontend && NO_COLOR=true trunk build --locked --release)
cargo build --locked -p stats-server -p desktop-pet --features tauri/custom-protocol
python3 full_http.py analytics require-renewal
node wsfa.mjs
python3 schemafa.py
python3 new-schemafa.py
```

Postgres專用fixture cluster127.0.0.1:55463，角色verify63／資料庫verifyfa、verifyfa_upgrade、verifyfa_from63。SQLite全用本任務work/的fixture；沒有刪除／重建使用者chat.db／pet.db、沒有正式秘密或付費AI呼叫，ai_core以Mock模型測試。所有測試腳本附在本證據目錄，需先啟動專用服務和fixture；不可將URL換成正式資料庫。

本輪先清除ui／stats-server／server／desktop-pet／persistence及target/front的ui，重編主線126／1、候選129／0後才採用結果。結束重新清除候選產物、重建正常main執行檔；最終runtime再次重現Cookie與SSR失敗。本輪沒有沿用前輪測試結論。

## action log新發現與欄位修正

P0 table與field可依功能需求增減／修改，沿用使用者確認，不新增規則。main action log省略id及明確NULL皆被接受，任意不存在的操作者ID也被接受；沒有Rust runtime writer。[Red](action-schema-red.json)。候選新增前向migration20261018130000，不覆寫已套用的20261018122554，id明確NOT NULL；保留有效舊row並重新連線讀回。缺ID／NULL插入或UPDATE／重複ID皆拒絕。有NULL舊row時整份migration回滾，原row／schema／history保留，須先修復既有身分再重試，不能默默丟棄紀錄。六項實測通過，另有persistence回歸已包含於129總數。[Green](action-schema-green.json)、[有效舊row](action-valid-upgrade.log)、[NULL回滾](action-null-upgrade.log)。created_user_id綁定真正授權人員、低敏內容及稽核寫入流程仍未完成。

## 品質檢查

主線fmt通過；嚴格all-targets Clippy exit101於apps/stats_server/src/admin.rs:254 collapsible_if。[main log](clippy.log)。候選logout及refresh巢狀if已整理，嚴格Clippy仍exit101於apps/desktop_pet/src/lib.rs:110 collapsible_if。[candidate log](candidate-clippy.log)。兩者皆未達全品質檢查通過。

## 精確 SHA 的 CI

[六個workflow／17個job原始快照](ci.json)，查詢時間 2026-10-18T12:50:52.760898+00:00；17個皆success。Test All/test-ui只跑cargo test --locked -p ui，未啟用SSR；build-chat為建置；test-stats-server未開postgres-integration／include-ignored。Rust Persistence/test-persistence會跑其PG store，仍不取代Cookie／舊DB／原生GUI。詳[tests workflow](ci-test-workflow.yml)、[chat workflow](ci-chat-workflow.yml)。

- [Rust Dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39127850074) `fa0888ea1e1ea93415a4ba7f16972938969b1be3` completed/success；[build-dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39127850074/job/117676642575) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:36:36Z)；[check-dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39127850074/job/117676643415) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:35:58Z)
- [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39127850027) `fa0888ea1e1ea93415a4ba7f16972938969b1be3` completed/success；[test-chat-application](https://github.com/williechen/rust_ai_chat/actions/runs/39127850027/job/117676642694) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:35:14Z)；[test-shared](https://github.com/williechen/rust_ai_chat/actions/runs/39127850027/job/117676642818) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:35:11Z)；[test-ui](https://github.com/williechen/rust_ai_chat/actions/runs/39127850027/job/117676642824) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:35:51Z)；[test-desktop-pet](https://github.com/williechen/rust_ai_chat/actions/runs/39127850027/job/117676642852) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:36:56Z)；[test-pet-domain](https://github.com/williechen/rust_ai_chat/actions/runs/39127850027/job/117676642878) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:35:11Z)；[test-chat-domain](https://github.com/williechen/rust_ai_chat/actions/runs/39127850027/job/117676642885) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:35:00Z)；[test-ai-core](https://github.com/williechen/rust_ai_chat/actions/runs/39127850027/job/117676642910) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:35:05Z)；[test-stats-server](https://github.com/williechen/rust_ai_chat/actions/runs/39127850027/job/117676642952) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:36:14Z)；[test-server](https://github.com/williechen/rust_ai_chat/actions/runs/39127850027/job/117676642998) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:35:56Z)
- [Persistence](https://github.com/williechen/rust_ai_chat/actions/runs/39127850046) `fa0888ea1e1ea93415a4ba7f16972938969b1be3` completed/success；[test-persistence](https://github.com/williechen/rust_ai_chat/actions/runs/39127850046/job/117676642700) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:38:06Z)
- [Rust Desktop Pet](https://github.com/williechen/rust_ai_chat/actions/runs/39127850084) `fa0888ea1e1ea93415a4ba7f16972938969b1be3` completed/success；[build-pet](https://github.com/williechen/rust_ai_chat/actions/runs/39127850084/job/117676642863) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:40:34Z)；[check-pet](https://github.com/williechen/rust_ai_chat/actions/runs/39127850084/job/117676643025) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:43:02Z)
- [Node](https://github.com/williechen/rust_ai_chat/actions/runs/39127850065) `fa0888ea1e1ea93415a4ba7f16972938969b1be3` completed/success；[test-node](https://github.com/williechen/rust_ai_chat/actions/runs/39127850065/job/117676642609) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:34:37Z)
- [Rust AI Chat](https://github.com/williechen/rust_ai_chat/actions/runs/39127850092) `fa0888ea1e1ea93415a4ba7f16972938969b1be3` completed/success；[check-chat](https://github.com/williechen/rust_ai_chat/actions/runs/39127850092/job/117676643118) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:46:03Z)；[build-chat](https://github.com/williechen/rust_ai_chat/actions/runs/39127850092/job/117676643323) completed/success (2026-10-18T12:34:33Z → 2026-10-18T12:48:03Z)

## 必要修正及重測次序

1. 在工作分支套用functional-fixes.patch（先git apply --check），main保留；驗證名稱3筆、空結果、HTTP失敗／重試、SSR及Cookie/TCP expiry。候選已做到Green，尚未合併main。
2. 保留既有migration歷史及checksum，不把已套migration移走或改名；另做前向資料搬移。以419／791 Chat、舊31 PG與63各自含既有資料的fixture重測，確認保留資料及重啟。
3. 新protocol表需要接通既有配對／短效憑證／輪替／撤銷／低敏稽核需求的真實流程後，再測授權、撤銷、重試及資料保存；不把存憑證／任意JSON等同安全實作。
4. 完成既有帳號／ACL／角色／scope／歷史及Pet多寵物／GUI等產品項目，取得原生GUI、多螢幕、30分鐘browser Session等缺失證據後再判定完整功能驗收。

## 同步範圍

既有Notion共同總頁、三入口、Day17三頁及Day18三頁共10頁已精確更新並全文讀回；保留Day17歷史程式碼／child pages與v1正文，Day18必要程式與schema修正已同步。Day18仍四節；Day17原始歷史教學保留。GitHub current三篇取自Notion讀回；更新現有狀態、索引與四份產品文件的當前觀察，產品需求與v1未新增；next_day各19、makeup_queue、last_full_comparison不變。參考rust_ai_chat驗證並不自動完成copilot_use產品驗收。
