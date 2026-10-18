# 最新 main 7917f8b 驗證與文章同步

記錄時間 2026-10-18T18:27:37.877665+08:00（Asia/Taipei）。main `7917f8b2e05ed74703ff9a83b62732ce04461da7`。正式完整功能驗收：**未通過**。

## 本次改動與實際結論

只有兩份root SQLite migrations修改：20261018091733_create_pet、20261018094242_create_ai_person。新增NOT NULL的是pet_profile.owner_id、四張附屬表的pet_id／owner_id、AI附屬表ai_person_id與AI生活紀錄room_id。各表自己的id仍是TEXT PRIMARY KEY，並未加入NOT NULL。本輪沒有修改runtime／UI／Stats PG migrations或Cargo.lock。

新建SQLx遷移成功；persistence 5通過；重新編譯main並在新SQLite啟動，GET /health及/api/rooms均200。27個schema案例實際核對新增必填限制、正確關聯、無效外鍵與缺少主鍵ID；所有觀察符合預期，其中「允許NULL主鍵」是刻意重現的缺口，不是完整性驗收成功。

具體改善：寵物缺owner、四種附屬紀錄缺pet／owner、AI附屬／生活紀錄缺person、AI生活紀錄缺room均拒絕；有效文字ID及外鍵新增成功，不存在pet外鍵遭拒絕。

仍失敗：從上一版6e1c791原六份migration建立的專用資料庫升級最新main，exit1：migration 20261018091733 was previously applied but has been modified。同DB啟動main亦exit1：VersionMismatch(20261018091733)。這是歷史migration checksum問題，不是NOT NULL語法失敗。91733先阻擋，不能據此宣稱後續94242 checksum已相容。

仍有缺口：`INSERT INTO pet_owner(name) VALUES('missing id')`、省略pet_profile.id但提供有效owner、以及省略ai_person.id，實際全部成功並存為NULL。關聯欄位NOT NULL不能代替主鍵id NOT NULL；foreign_key_check仍為空也不能證明主鍵完整。

## 測試命令與證據

| 命令或範圍 | 實際結果 | 原始證據 |
|---|---|---|
| DATABASE_URL=sqlite://work/fresh791.sqlite?mode=rwc cargo sqlx migrate run --source work/rust_ai_chat/migrations | exit0，新建完整六份migration套用 | [fresh](fresh.log) |
| 先以git show 6e1c791取得原六份SQL至work/migrations6e，同專用舊DB先run舊source再run最新source | 原版exit0；最新exit1 modified91733 | [old](old.log)、[upgrade](upgrade.log) |
| 全新隔離clone，cargo test --locked -p persistence | 5 passed／0 failed，無共用target／快取歸屬問題 | [log](persistence.log) |
| cargo build --locked -p server --features ssr | exit0，新的checkout實際編譯 | [log](build.log) |
| CHAT_DATABASE_URL=sqlite://專用新檔?mode=rwc LEPTOS_OUTPUT_NAME=rust_ai_chat LEPTOS_SITE_ROOT=空測試site LEPTOS_SITE_PKG_DIR=pkg target/debug/server | Server running；/health、/api/rooms均200 | [startup](start-fresh.log)、[HTTP](http.json) |
| 同最新binary，CHAT_DATABASE_URL指向已套用6e1c791專用DB | exit1 VersionMismatch(20261018091733) | [log](start-upgrade.log) |
| python3 work/check791.py，sqlite3 PRAGMA foreign_keys=ON | 27案：新增關聯限制生效，三種缺主鍵ID仍為NULL | [完整SQL與結果](constraints.json)、[可重現腳本](check_constraints.py) |

空site只用來驗證API與啟動，沒有宣稱WASM資產、SSR畫面或瀏覽器E2E本輪通過。schema案例包含預期接受NULL的缺陷重現，不能把27案觀察符合預期寫成27個產品gate全綠。

## 必需修改與剩餘驗收

1. 主鍵完整性：依目前文字ID設計，自己的主鍵需明確 `id TEXT NOT NULL PRIMARY KEY`，由建立入口產生／驗證ID。測試省略ID／NULL都應拒絕，合法文字ID應成功；不能把這次owner_id／pet_id NOT NULL當成已修id。既有NULL ID需先明確修復，避免假造關聯。
2. 升級相容性：已被部署套用的91733／94242／95626各版SQL要保留原內容與checksum，使用後續SQLite migration重建表、保留資料與ID並補約束。需逐一驗證實際歷史基線，不能刪_sqlx_migrations或略過checksum後宣稱升級成功。本次僅操作專用fixture，未改使用者DB。
3. Chat：UI／room_api未變；先前1853876的3匹配→2顯示、SSR歷史提示斷言尚未修。已有歷史修正候選與證據仍保留原SHA，本輪未重跑搜尋瀏覽器、WASM或完整帳號／ACL／歷史／跨瀏覽器WS。
4. Pet：漫遊核心與lock未變，前輪四案原生幾何只在相同核心範圍適用；本輪未重跑原生GUI。多寵物／多螢幕／拖曳與睡眠／執行中設定持久化仍未驗證；schema改善不等於UI／repository／API接通。
5. Analytics：Stats程式與PG migrations未變，Cookie續期及PG31→32舊DB升級失敗仍未修。本輪未重跑相同HTTP／PG；root AI人物schema不等於帳密／角色／scope已完成。1800秒瀏覽器soak尚未執行。

前輪證據：[6e1c791](../2026-10-18-main-6e1c791/REPORT.md)、[1853876](../2026-10-18-main-1853876/REPORT.md)。歷史候選e33971f依2eddd64／INTEGER ID契約建立，不能直接套用目前TEXT ID版本；本輪未新增程式修正commit、未推送或修改main。

## 本地目錄與驗證環境

使用者指定的程式目錄已確認為 /home/willie/Documents/rust_ai_chat 與 /home/willie/Documents/copilot_use。前者main仍在98f30d8b3593d80103b1a4bdbb9f0cb8cb08069e，且有資料庫檔案變動；後者main在0bf1d460ea8f20dfaa3bcdaf14b7a895d1b3aacd。沒有讀取本地資料庫內容或覆蓋這些工作目錄；最新7917f8b驗證使用本次隔離clone。原任務checkout已不在原路徑，因此從遠端重建；過往證據仍可由GitHub取回，不以遺失的本機產物冒充本輪結果。

Ubuntu26.04.1、Rust/Cargo1.99.1、sqlx-cli0.9.0、Python SQLite3.46.1；Cargo.lock未變。使用專用SQLite與合成fixture，未讀正式秘密、未呼叫付費AI。

## 文章同步與CI

既有10個Notion頁已更新並全文讀回，Day18四節與子頁保存。v1正文未改，next_day各19、makeup_queue及last_full_comparison未推進。GitHub三篇鏡像、四份產品文件現況、manifest與learning-state同步；規則／原需求沒有增加。

CI完整API快照見 [ci.json](ci.json)，精確job開始／結束時間與workflow/run亦在原始資料；不以CI success替代舊DB升級或產品驗收。

| job | 狀態／結論 | 證據 |
|---|---|---|
| test-node | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864165/job/117650158065) |
| test-persistence | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864202/job/117650158219) |
| build-chat | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864205/job/117650158320) |
| check-chat | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864205/job/117650158704) |
| test-chat-application | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864222/job/117650158347) |
| test-server | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864222/job/117650158409) |
| test-stats-server | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864222/job/117650158454) |
| test-shared | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864222/job/117650158467) |
| test-pet-domain | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864222/job/117650158477) |
| test-chat-domain | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864222/job/117650158487) |
| test-desktop-pet | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864222/job/117650158515) |
| test-ui | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864222/job/117650158538) |
| test-ai-core | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864222/job/117650158804) |
| build-dashboard | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864283/job/117650158545) |
| check-dashboard | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864283/job/117650158788) |
| check-pet | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864189/job/117650158211) |
| build-pet | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118864189/job/117650158380) |

觀察時間 2026-10-18T10:23:52.947953+00:00；尚未完成者保留pending。
