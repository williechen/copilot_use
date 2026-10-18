# 最新 main 6e1c791 驗證與文章同步

main `6e1c791d3e55124cd1de0797fd2569d8ddaf60d1`。正式完整功能驗收：**未通過**。本輪未修改或推送rust_ai_chat/main，沒有新增候選commit。

## 更新與結論

本次移除20261018100114的無效ALTER migration，直接修改20261018091733_create_pet與20261018095626_create_pet_owner：主鍵／pet_id／owner_id改TEXT，owner外鍵改指pet_owner(id)。這已解除新建SQLite的MODIFY語法與missing owner阻擋。文字ID是目前設計，不再把未自增視為錯誤，也不建議直接套用先前INTEGER候選。

**新建DB：通過。既有DB升級：失敗。ID完整性：有缺口。**

- 新建DB的完整SQLx migrations成功；清除persistence套件產物後5 passed／0 failed。main實際編譯與啟動成功，GET /health、/api/rooms均200。
- 明確文字ID新增pet_owner／pet_profile成功；owner_id指向不存在主人被FOREIGN KEY constraint failed拒絕，證明新FK有效。
- 先在專用DB套用1853876的原六份migrations，再執行最新migrations，exit1：`migration 20261018091733 was previously applied but has been modified`。同DB啟動main亦exit1 `VersionMismatch(20261018091733)`。只修新建不能當成舊DB升級完成；本輪沒有刪_sqlx_migrations、改checksum或重建正式DB。
- SQLite普通rowid表的`id TEXT PRIMARY KEY`未隱含NOT NULL。實際`INSERT INTO pet_owner(name) VALUES('missing id')`成功且id=NULL；foreign_key_check依然空，不能把FK檢查當主鍵非空證據。現在尚無完整pet repository/API/UI保證呼叫端必傳ID，schema仍需依實際契約補`TEXT NOT NULL PRIMARY KEY`及文字ID產生／驗證。owner_id目前允許NULL是否符合產品須明確確認，沒有自行增加「每筆必有主人」規則。

## 實際命令、結果與證據

| 命令／範圍 | 實際結果 | 原始證據 |
|---|---|---|
| DATABASE_URL=sqlite://專用新檔?mode=rwc cargo sqlx migrate run --source 最新/migrations | exit0，六份完整套用 | [log](fresh.log) |
| 專用舊DB先 --source 原1853876六份，再 --source 最新/migrations | 舊版exit0；最新版exit1，91733 checksum mismatch | [old](old.log)、[upgrade](upgrade.log) |
| cargo clean -p persistence --target-dir 共用target；CARGO_TARGET_DIR=共用target cargo test --locked -p persistence | 5 passed／0 failed | [log](persistence.log) |
| CARGO_TARGET_DIR=共用target cargo build --locked -p server --features ssr | exit0；latest-main/persistence及server明確重新編譯 | [log](build.log) |
| CHAT_DATABASE_URL=專用新SQLite LEPTOS_OUTPUT_NAME=rust_ai_chat LEPTOS_SITE_ROOT=既有site LEPTOS_SITE_PKG_DIR=pkg target/debug/server | 新建DB啟動成功；/health及/api/rooms均200 | [startup](start-fresh.log)、[HTTP](http.json) |
| 相同最新main binary，CHAT_DATABASE_URL切到已套用舊版專用DB | exit1，VersionMismatch(20261018091733)，未啟動HTTP | [log](start-upgrade.log) |
| Python sqlite3，PRAGMA foreign_keys=ON，明確ID／不存在owner／缺少ID插入 | 有效新增成功、無效owner拒絕；缺少owner主鍵ID仍成功為NULL | [SQL與結果](constraints.json) |

LEPTOS_SITE_ROOT使用前次已生成site僅供本輪health/API smoke，沒有冒稱已重新建置WASM或驗證最新瀏覽器端。這次修改只有root migrations，UI／Stats／Pet漫遊程式與Cargo.lock完全未變。

## 必需修改與保留的缺口

1. 已存在資料庫：恢復91733與95626曾套用版本的原始內容／checksum，新增後續SQLite重建表migration轉到目前TEXT ID設計；保留有效ID、owner關係與資料，缺少ID或映射先明確回填後再遷移。若只有未交付的拋棄式開發DB，可另行決定重建，但本次沒有把刪庫當作升級通過，也沒有操作使用者DB。不同歷史基線需分開驗證，不能略過SQLx校驗。
2. 主鍵補NOT NULL並依TEXT ID契約驗證缺少ID應拒絕。舊NULL ID的修復不能假造主人對應。先前e33971f是2eddd64／INTEGER候選，其parent／欄位契約已不同，不可直接套用最新main。
3. Chat名稱搜尋3匹配只顯示2、SSR歷史提示斷言仍未修；本輪未重跑瀏覽器，其前輪證據保留原1853876來源。現在fresh DB啟動已解除，不再沿用2eddd64「所有新DB啟動失敗」結論。
4. Pet漫遊核心／lock無變；原生四案證據只在該範圍適用，完整GUI、多螢幕、普通啟動／拖曳／執行中設定持久化仍未驗證。新schema不代表完整寵物管理或AI人物功能已接通。
5. Analytics程式與PG migrations未變，Cookie活動回應未續期及PG migration31→32升級失敗仍未修；帳密／角色／scope／撤銷與1800秒瀏覽器soak未完成或未驗證。本輪未重跑相同HTTP／PG流程，先前證據保留原SHA。

前輪原始證據：[1853876](../2026-10-18-main-1853876/REPORT.md)、[2eddd64與歷史候選](../2026-10-18-main-2eddd64/REPORT.md)。本輪沒有把任何歷史候選的105通過／四種升級情境回填給main。

## 文章、環境與CI

既有10個Notion頁（Day18三篇、Day17三篇、三入口及共同現況）已修訂並全文讀回。Day18保留四節，子頁／ID保存；GitHub鏡像、manifest與learning-state觀察同步。v1正文未改，next_day各19、makeup_queue與last_full_comparison未推進。

環境：Ubuntu26.04.1、Rust/Cargo1.99.1、SQLx0.9.0、Python SQLite3.46.1；Axum0.8.9／Leptos0.8.22／Tauri2.12.4。專用新SQLite及1853876升級fixture，未讀正式秘密、未呼叫付費AI、未改遠端main。主線只在隔離checkout執行。

最新main精確CI快照見[ci.json](ci.json)，只代表workflow/job實際覆蓋，不替代DB升級與產品驗收。

| job | 狀態／結論 | 證據 |
|---|---|---|
| test-chat-domain | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566950/job/117649287333) |
| test-shared | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566950/job/117649287507) |
| test-desktop-pet | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566950/job/117649287515) |
| test-ai-core | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566950/job/117649287535) |
| test-server | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566950/job/117649287544) |
| test-chat-application | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566950/job/117649287545) |
| test-ui | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566950/job/117649287557) |
| test-pet-domain | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566950/job/117649287564) |
| test-stats-server | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566950/job/117649287567) |
| check-dashboard | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118567023/job/117649287706) |
| build-dashboard | queued／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118567023/job/117649287873) |
| check-chat | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566926/job/117649287283) |
| build-chat | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566926/job/117649287442) |
| test-node | completed／success | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566933/job/117649287185) |
| test-persistence | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566958/job/117649287444) |
| build-pet | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566957/job/117649287227) |
| check-pet | in_progress／None | [job](https://github.com/williechen/rust_ai_chat/actions/runs/39118566957/job/117649287391) |

觀察時間 2026-10-18T10:17:26.097838+00:00；尚未結束者保留pending。
