# Day30 驗證報告｜2026-10-23

**結果：三案 Day30 程式都已套用；Chat 建房 Session 與 Analytics Leptos 登入切片實測通過，Pet 補償邏輯仍待原生失敗注入／GUI 驗證。** 不把「已套用」當成三案全部驗收完成，不更新 `last_full_comparison`，三案 `next_day` 保持31。

## 精確來源與跨 SHA 證據

- 最終程式來源：`williechen/rust_ai_chat/main@1d1866dbbb30255fafa4c7f74b1868919662913c`。
- 本次起跑／完整原生測試、PG、WASM、HTTP及瀏覽器使用來源：`974db9ff29f0ba640d1df84a3aaaf2556d7624e1`。
- 驗證期間 main 新增 `1d1866d`；已 fetch 並 fast-forward 本地唯讀副本，逐檔 diff 確認只改3個 Node `.test.mjs`，其餘Rust／UI／JS bridge／Cargo.lock／migrations完全不變。此差異核對允許沿用前述範圍的同 bytes 證據，**不是宣稱全部測試在最後SHA重新跑過**。
- 最後SHA另實跑：Node全套、Chat Day30指定測試、Analytics Day30指定測試，均通過。
- 文件基準：`williechen/copilot_use/main@36559d4549dc71bda6dffb15e057d07493d9a8a1`；Chat Day30 r3、Pet r2、Analytics r2。原教學的「未套用／未執行」屬當時事實，本報告補記目前結果，不改寫原歷史。
- `rust_ai_chat` 沒有修改tracked source／lock／migration、沒有推送；沒有使用正式DB、秘密或付費AI。測試為新建SQLite與原生PostgreSQL18.6隔離cluster，結束已停止服務／cluster。

## 逐案結論

| 專案 | 程式套用 | 本次實測 | 尚待驗證／調整 |
|---|---|---|---|
| Chat Day30 r3 | 已套用：production `/api/rooms` 改為公開GET＋受會員Session middleware保護的POST；新增指定測試 | 指定測試1/1通過；生產HTTP確認未登入401、偽造401、一般會員201、過期401、登出撤銷後401、GET一直200；寫入與服務重啟讀回成立 | 文章仍提Leptos建房UI，但實際只有browser helper，個人頁尚無建房操作；完整UI Gate未完成。逐房ACL、ownership／CSRF屬另外追蹤事項，不由會員驗證通過推論完成 |
| Pet Day30 r2 | 已套用：`window.close()`同步回錯時嘗試`set_caged(false)`；DB補償也失敗時回報兩個錯誤；成功才remove Runtime | desktop-pet既有49項Rust tests通過、WASM check通過；靜態呼叫鏈符合當日Green | **缺真正關窗失敗、DB寫入失敗、補償失敗注入、3個pet id原生GUI與App重啟驗證**。49個既有測試不包含新增補償失敗路徑，不能記Day30完整通過 |
| Analytics Day30 r2 | 已套用：LoginView/render_login＋正式login_page呼叫Leptos SSR；原Argon2／Session／PG契約保留 | 指定SSR測試1/1；一個doctype與label/input契約；完整PG36/36含ignored；真HTTP初始登入→改密碼→撤銷→重登；真瀏覽器登入→Chat/Pet切換→登出 | **本日登入SSR切片通過**；改密碼頁Leptos遷移、角色／來源ACL等產品目標仍獨立待辦。原文／封存／manifest未在本次報告提交中搬移或改寫 |

Chat需求以使用者最後澄清為準：**先建立帳號並登入，有有效會員Session才能建立房間；一般會員可以建房，匿名訪客不能建房**。本輪不是管理者專屬建房規則。HTTP使用新註冊的一般會員證明201，未藉管理者fixture通過。

## 真實生產 HTTP：32／32通過

實際啟動本次建置的server及stats-server binary，沒有測試替代router。完整逐項 actual／expected 見`logs/http-results.json`。

Chat：公開GET200；匿名／偽造POST401且rooms仍0；註冊303、登入303；一般會員POST201且DB存在該id；Session到期後POST401；服務重啟房間仍在且有效Session可POST201；logout303後同Cookie POST401；公開GET仍200。

Analytics：真bootstrap CLI成功；GET/login200，doctype僅1個、標籤與密碼欄位對應；匿名後台導向/login；首次帳密登入303並被限制於/change-password；變更密碼303後舊Cookie查詢401；新密碼登入成功；Chat/Pet各以獨立fixture服務token攝取201；各來源Dashboard200，真PG總數各1且aria-current只有1；未知來源400。

瀏覽器由隔離測試帳號登入Leptos新登入頁，進入Chat來源後台；點Pet來源顯示真實總數1；點登出返回登入頁。登入憑證為本次新建fixture，不是使用者帳號。AX結果保存在`logs/browser-analytics.txt`，畫面如下。

![Analytics 真瀏覽器來源切換](day30-analytics-browser.png)

## 測試與命令

Cargo全程`--locked --offline`，產物在獨立`work/target-day30`，`CARGO_PROFILE_DEV_DEBUG=0`、`CARGO_INCREMENTAL=0`。本次沒有磁碟不足中止。

| 命令／目標 | 結果 | 精確來源與限制 |
|---|---|---|
| cargo test -p server -p desktop-pet -p stats-server --no-fail-fast | 98 passed／0 failed／1 ignored | 974db9f；PG feature未啟用的第一輪；ignored於下一輪補跑 |
| DATABASE_URL=隔離PG cargo test -p stats-server --features postgres-integration --no-fail-fast -- --include-ignored | **36 passed／0 failed／0 ignored** | 974db9f；包含本次新增登入SSR test，以及所有PG integration targets；也含舊admin router，結果不能全歸給正式password_login router |
| cargo check -p ui --features hydrate --target wasm32-unknown-unknown | exit0 | 974db9f；WASM check，非browser hydration E2E |
| cargo check -p desktop-pet-frontend --target wasm32-unknown-unknown | exit0 | 974db9f；不是原生GUI驗收 |
| cargo build -p server --features ssr -p stats-server | exit0 | HTTP／browser使用此產物 |
| cargo test -p ui --features ssr --no-fail-fast | **9 passed／1 failed** | 原Day16要求的「歷史載入尚未接通」提示仍缺，非Day30 middleware／Analytics render_login的失敗 |
| cargo fmt --all -- --check | exit0 | 974db9f |
| node --test（974db9f） | 0 passed／3 failed | `window is not defined`；不是原先的URLSearchParams未定義 |
| node --test（**1d1866d**） | **3 passed／0 failed** | 已更新的正式最新source通過，非僅修補副本；不再把此舊問題列為目前未修 |
| cargo test -p server anonymous_create_is_rejected_while_public_list_is_available | 1 passed／0 failed | **1d1866d**；非0 tests的空成功 |
| cargo test -p stats-server login_page_is_server_rendered_with_accessible_fields | 1 passed／0 failed | **1d1866d**；完整其他PG tests仍引用已核對不變的974db9f |

Pet實際package名稱是`desktop-pet`，執行命令依其Cargo.toml，不以文章中的`desktop_pet`拼法推定已跑測試。此輪不是Clippy或全專案比對重跑；之前報告的lint／其他產品缺口未因本次Day30測試而結案。

## Node修正歷程

`974db9f`把`window.location = location`及新的vm.runInNewContext放在檔案頂層，早於test內的window mock與script宣告，故Node CI／本地同樣3 fail。本次先在獨立副本驗證把呼叫放回mock建立之後可3 pass，之後使用者新commit `1d1866d` 已完成相同方向修正；對該最新原庫再跑也3 pass。舊CI失敗log保留為歷史，最新Node CI success才是目前結論。

## CI

最終來源`1d1866dbbb30255fafa4c7f74b1868919662913c`查詢快照：

| Workflow | Run | 查核時狀態 |
|---|---:|---|
| Node | [39932989077](https://github.com/williechen/rust_ai_chat/actions/runs/39932989077) | success |
| Persistence | [39932989037](https://github.com/williechen/rust_ai_chat/actions/runs/39932989037) | in_progress |
| Rust AI Chat | [39932989025](https://github.com/williechen/rust_ai_chat/actions/runs/39932989025) | in_progress |
| Test All | [39932989041](https://github.com/williechen/rust_ai_chat/actions/runs/39932989041) | in_progress |
| Rust Dashboard | [39932989009](https://github.com/williechen/rust_ai_chat/actions/runs/39932989009) | in_progress |
| Rust Desktop Pet | [39932988976](https://github.com/williechen/rust_ai_chat/actions/runs/39932988976) | in_progress |

精確created／updated／head_sha存`logs/latest-ci-runs.json`。執行中不記success；本地成功也不替代所有CI完成。前一SHA的Node run39931779620／job120341548313為failure，完整log在`logs/node-ci.log`；兩個SHA不混用。

## 版本與文件

Cargo.toml與lock的0.2.28→0.2.30僅12個workspace package版本變更，外部依賴resolved沒有升級。本次向crates.io官方registry重查10項相關依賴latest non-yanked stable與MSRV（明細`logs/versions.json`）：Axum0.8.9、SQLx0.9.0、Tauri2.12.5／tauri-build2.7.4、Leptos0.8.22／router0.8.17／leptos_axum0.8.10、Argon2 0.6.0、tower0.5.3、uuid1.28.0，皆與目前lock相符。Rust本機1.99.1高於其中最高宣告MSRV1.94.0；本次native／WASM編譯證據如上。

這是涉及依賴的補驗，不是把原文第③節當時「尚未確認」改寫為歷史已通過。SQLx SQLite runtime版本、全部native工具上游版本／跨平台仍不在本輪完成聲明內。原文章Gate1交付、目前Gate2切片證據與完整P9產品驗收分開。

## 狀態與證據保留

- 三案next_day仍31，`last_full_comparison`仍`2026-10-16T22:47:14.399027+08:00`。
- 不結案makeup_queue，不以本輪切片通過宣稱全專案全面比對成功。
- 本輪新增驗證報告與原始證據，不覆寫原教學／歷史SHA，不移動文章。Pet必要原生GUI證據仍待完成；Chat完整建房UI仍缺。
- `SUMMARY.json`記錄各案狀態、來源與evidence.tar.gz SHA256。archive含測試命令／日誌、source alignment、文章快照與檢查腳本；不含DB或編譯cache。
- 提交前重讀兩庫main，copilot_use以expected branch SHA／force=false條件更新；提交後從精確commit讀回全文與blob，保存READBACK.json。報告提交成功不是產品全部驗收成功。
