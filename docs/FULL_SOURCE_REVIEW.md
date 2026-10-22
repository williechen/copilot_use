# 兩庫逐模組、介面與資料流程完整核對

> 歷史雙庫靜態比較，保留原文、固定 SHA 與結果；其中「成品／參考」是當時命名，不是現行儲存庫分工。現行主體只為 rust_ai_chat，分工見 [共用規則](COMMON_LEARNING_RULES.md)。本次未重新執行全量比較。

核對日期：2026-10-14。**逐模組、介面與資料流程的完整靜態核對已完成；兩庫不具全部程式語意等同。** 核對完成不代表差異已修正，也不是目前 SHA 的完整部署或跨平台執行驗收。下列為原完整靜態核對的固定來源快照，非最新 main 的全量重測；其後的局部來源／本地測試／CI 核對見 [參考庫驗證證據](source-review/rust-ai-chat-validation.json)。

- 唯讀參考 `rust_ai_chat/main`：`16261e0ebeef1a888997cc0ebc42b08ceaf8db24`。
- 成品本地 `teaching/rust-ai-chat-alignment`：`cd3df07faef994eb544b0648c2e44f17d1853302`。
- 原核對當時的成品遠端 main／teaching 為 `49e1d946dd3c22ffcc4b1aa0e7715cb5b1686724`；當時本報告尚未推送（歷史狀態）。
- 正式 `v0.1.0` tag 保留 `22b44f33b9e2c9edb0fb398fb9bff3efd57b00ee`。
- 參考庫不寫入；沒有載入 API key、付費 AI 呼叫、重構 source 或發布 Day。

## 覆蓋率與可追溯證據

| 範圍 | 結果 |
|---|---|
| 追蹤檔案身分 | 參考141、成品294；逐檔SHA256與分類，沒有以文件hash冒充內容語意核對 |
| 自有Rust source／tests／build.rs | 參考50、成品67，全數各歸屬一個模組組別；38組職責及接口對照 |
| Rust語法宣告 | 225份Rust零parse error；自有725函式／方法、133struct、25enum、7trait、7type；含private及tests |
| Pet IPC | 主UI40種command；backend44種均註冊；逐command保留payload source及signature，未發現名稱缺漏 |
| 資料流程 | 26條入口、呼叫端、儲存、事件、成功／失敗、授權／交易／平台边界 |
| 第三方vendor | 108份GLibRust解析列冊；不宣稱第三方全部body人工安全審計，無reference counterpart |

可攜式核對證據保存在 repository 的 `docs/source-review/`；完整 raw 資料與核對工具在 ZIP，移動 checkout 不依賴外部本地資料夾。來源連結固定於上述 SHA。

- [證據說明](source-review/README.md) · [完整證據包（AST／反向引用／工具）](source-review/evidence.zip)
- [模組逐檔映射](source-review/module-mapping.json)
- [全部自有宣告／欄位／介面／函式內呼叫](source-review/interface-catalog.md)
- [Cargo package／feature／依賴邊界](source-review/package-boundaries.json)
- [資料流程與來源函式](source-review/data-flow-review.md)
- [44 個 IPC 簽章與 40 種主 UI 命令呼叫點](source-review/ipc-payloads.json)
- [Cargo、SQL、Proto、CI、scripts 與 config 來源](source-review/configuration-contracts.md)
- [完整 tracked snapshot](source-review/tracked-files.json) · [覆蓋率](source-review/coverage.json) · [證據身分](source-review/manifest.json)

## 必須保留的差異與尚未對應項目

| 項目 | 判定／處理需求 |
|---|---|
| Chat room HTTP／WS | 參考name-only建立、200、typed WS傳訊；成品指定id、201、HTTP傳訊+flat WS通知。需要明確adapter教學，不能直接替換DTO |
| Chat Domain／SQLite | 同為SQLite，但String/UUID BLOB、rooms/chat_rooms、category/ACL不同；不能共用migration或把topic當category |
| Pet FSM | Sleep/Wake、monotonic snapshot revision無等同成品流程；Action/Speed不是相同FSM |
| Pet Profile／preview | JSON與TOML、ScanPreview object與Vec<opaqueID/name/size>不同；應明列codec/wire适配 |
| Pet cleanup event | 參考pet.cleanup.succeeded、成品file.cleanup.succeeded；成品白名單不接受前者，需要顯式決定mapping/version，不能偷偷改事件 |
| Pet native window roaming | 參考roam.rs實際移動視窗／monitor邊界；成品僅sprite animation。已納入 P9，尚未實作／驗收 |
| Pet duplicate SHA256 grouping | 參考duplicate_preview sizegroup+hash；成品scan存在但無hashduplicategroup。已納入 P9，尚未實作／驗收 |
| Analytics | 參考validrequest→501與HashSet fake；成品PG交易／durable dedup／gRPC／SSE。保留成品能力，不退回stub |
| Chat SSE授權複核 | openai::reply先auth/ACL，produce完成前不重新principal/ACL；agent入口有複核。屬靜態發現的撤權race風險，待針對性回歸及修正 |
| 參考SQLite測試 | persistence長41字元ID測試assert!(!result.is_err())與「不應接受」訊息相反；VARCHAR(40)本身無SQLite長度CHECK。僅記錄，參考庫不修改 |
| 產品新增功能 | auth/RAG/MCP/personas/outbox/restore/update/OTel/insights/privacy均非reference等同模組；按P9需求保留 |
| Tests／workspace | reference根testsworkspace不採用；成品owning app/crate/private module tests保留 |

## 逐組核對

每組列出所有所屬Rust檔案；完整公開型別／方法及private接線見介面清單。空對應表示reference或產品沒有該職責，不表示掃描遺漏。

### C01 Chat Domain

參考：`crates/chat_domain/src/lib.rs`。

成品：`crates/chat_domain/src/lib.rs`。

介面：ChatMessage/MessageAuthor/Room → MessageInput/Committed/MessageError；String ID → UUID；category_room_id 無對應。

流程：HTTP author/room/content → validate → repository；Domain 不匯入 Driver。

判定：概念可映射、公開型別與序列化不等同；category 不能當 topic。

### C02 Chat Application

參考：`crates/chat_application/src/lib.rs`、`crates/chat_application/src/repository.rs`。

成品：`crates/chat_application/src/lib.rs`、`crates/chat_application/src/tests.rs`。

介面：create_room/RoomRepository、MessageRepository::create、persist_then_publish → MessageRepository::commit/CommitFuture/persist。

流程：參考 create→callback；成品 validate→commit→Committed(inserted)。

判定：儲存後才通知的不變量對應；無 RoomRepository 等同實作；保留 durable Adapter。

### C03 Chat 事件

參考：`crates/chat_application/src/product_event.rs`、`tests/chat/product_event_contract.rs`。

成品：`crates/shared/src/lib.rs`。

介面：PersistedMessageEvent → ProductEvent；四欄 message.persisted 一致；參考 String message ID 必須可轉 UUID。

流程：參考事件 factory 尚非 Outbox；成品 stage 同交易→gRPC。

判定：四欄事件子集可適配，不能宣稱所有事件／發送方式相同。

### C04 Chat SQLite

參考：`crates/persistence/src/lib.rs`。

成品：`crates/persistence/src/lib.rs`、`crates/persistence/src/tests.rs`、`crates/persistence/src/database.rs`、`crates/persistence/src/database/tests.rs`。

介面：SqliteRoom/MessageRepository → ChatWriter；rooms/messages → chat_rooms/chat_messages；UUID BLOB/ACL/Outbox/realtime。

流程：BEGIN IMMEDIATE→message+outbox+topic+realtime→commit；失敗 rollback；SQLite backup/import。

判定：同為 SQLite，schema/交易邊界不相同；參考 41 字元測試斷言與文字相反。

### C05 Chat 入口/HTTP/SSR

參考：`apps/server/src/http.rs`、`apps/server/src/lib.rs`、`apps/server/src/main.rs`、`apps/server/src/ssr.rs`、`apps/server/src/state.rs`、`tests/server/day5_tests.rs`。

成品：`apps/server/src/main.rs`、`apps/server/src/lib.rs`、`apps/server/src/web.rs`、`apps/server/frontend/src/main.rs`。

介面：/api/rooms {name} 200 {id} → /api/v1/rooms {id,name} 201；/health → /healthz；3000 → 3001。

流程：Leptos SSR→browser HTTP→auth/ACL→SQLite；入口狀態從 repositories 變為 StateData。

判定：API 不相容；產品入口保留，教學需明示 adapter/URL/DTO。

### C06 Chat WS/Fanout

參考：`apps/server/src/ws.rs`、`apps/server/src/hub.rs`。

成品：`apps/server/src/realtime.rs`、`apps/server/src/fanout.rs`。

介面：ClientEvent/ServerEvent tagged DTO → flat notifications/HTTP send；broadcast128 → durable realtime polling。

流程：參考 WS persist_then_publish→room hub；成品 HTTPcommit→canonical DB fanout→WS，lag→resync_required。

判定：保留跨程序 durability/ACL；不能把參考 socket 當成品 client。

### C07 Chat UI

參考：`crates/ui/src/lib.rs`、`crates/ui/src/chat_socket.rs`、`crates/ui/src/room_api.rs`。

成品：`crates/ui/src/lib.rs`、`crates/ui/src/browser.rs`。

介面：參考 typed WS/room API → auth UI、HTTP messages、notification WS、SSE/retryqueue。

流程：Hydrate→session→room ACL→HTTP；斷線 reconnect + resync，不以樂觀UI當 durable。

判定：映射頁面／生命週期，非逐檔移植；參考 hardcoded localhost WS 不適用部署。

### C08 Chat 認證

參考：無獨立對應模組。

成品：`apps/server/src/auth.rs`。

介面：Argon2/users/sessions/CSRF/owner/member/invite/throttle；參考無對應。

流程：登入→hash sessiontoken→SQLite；write→CSRF+ACL；auth事件與狀態同交易。

判定：產品安全擴充，不刪除以追求 reference 相同。

### C09 Chat RAG/MCP/Agent

參考：無獨立對應模組。

成品：`apps/server/src/knowledge.rs`、`apps/server/src/knowledge/tests.rs`、`apps/server/src/mcp.rs`、`apps/server/src/agent.rs`。

介面：owner-scoped document/1536 embedding；MCP readonly tools；bounded citations。

流程：auth→owner retrieve/MCP→provider→citation查證→重新 principal/ACL→persist。

判定：產品擴充；Agent 外部呼叫後有授權複核，不可概括到所有 AI 入口。

### C10 Chat AI SSE

參考：無獨立對應模組。

成品：`apps/server/src/openai.rs`。

介面：ReplyRequest{room_id,rag}；delta/done/error；completed 才 persist。

流程：先 auth/ACL→provider stream→Decoder/output buffer→ChatWriter.persist_as→done。

判定：靜態缺口：produce 未攜 actor/headers，完成前未複核 session/ACL；尚未修正或實測撤權 race。

### C11 Chat personas/policy

參考：無獨立對應模組。

成品：`apps/server/src/personas.rs`、`apps/server/src/suggestions.rs`。

介面：persona revision/lease/run budgets；approved proposal/context policy/CAS receipts。

流程：owner→bounded run→durable turn revision/lease CAS；proposal→apply→rollback compare revision。

判定：產品擴充，不能用一般 ChatMessage factory 取代。

### C12 Chat 同意撤回

參考：無獨立對應模組。

成品：`apps/server/src/topic_http.rs`、`crates/persistence/src/topics.rs`。

介面：consent flag/topic outbox/withdrawal batch；本文與 owner 不進事件。

流程：同意→訊息 txn stage topics；withdraw→local disable+outbox→Analytics tombstone→ack。

判定：產品隱私擴充；category 與 topic 語意分離。

### C13 Chat persist/publish 回歸

參考：`tests/chat/persist_publish.rs`。

成品：無等同實作；見對照決策／關聯组。

介面：FakeRepo 成功一次/失敗不publish；成品 C02/C04 tests。

流程：參考 callback ordering；成品 DB atomic+fanout 重讀。

判定：測試目的映射，保留 owning crate tests，自 Day15 起逐批遷移根 tests，全部歸位且驗證後才移除根 tests workspace。

### P01 Pet FSM

參考：`crates/pet_domain/src/lib.rs`、`crates/pet_domain/src/error.rs`、`tests/pet/pet_machine.rs`。

成品：`crates/pet_domain/src/lib.rs`、`crates/pet_runtime/src/interaction.rs`。

介面：Idle/Interacting/Sleeping、Interact/FinishInteraction/Sleep/Wake、revision → Action/Speed/proactive。

流程：參考 dispatch→snapshot→event→UI revision；成品 settings→animation/reminder。

判定：狀態與命令不相容；Sleep/Wake/revision 目前無等同流程，不能空造同名型別。

### P02 Pet Profile

參考：`crates/pet_domain/src/profile.rs`、`tests/pet/profile_contract.rs`、`tests/pet/day5_codec_tests.rs`。

成品：`crates/pet_domain/src/profile.rs`、`crates/pet_domain/src/application.rs`、`crates/pet_domain/src/application/tests.rs`。

介面：五欄/optional appearance-animation；JSON codec → TOML codec/limits/AssetManifest/ProfileRepository。

流程：Tauri→save_profile validation→PetStore.save→SQLite→private TOML projection。

判定：概念對應，格式/驗證/錯誤不等同；ProjectionPending 必須保留。

### P03 Pet cleanup Domain

參考：`crates/pet_domain/src/cleanup.rs`、`crates/pet_domain/src/cleanup_event.rs`、`tests/pet/cleanup_contract.rs`、`tests/pet/cleanup_event_contract.rs`。

成品：無等同實作；見對照決策／關聯组。

介面：Previewed→Confirmed→Succeeded/Failed；pet.cleanup.succeeded → file.cleanup.succeeded。

流程：參考純狀態，並非真 OS receipt；成品真隔離成功 receipt→SQLite/Outbox。

判定：事件名稱不相容；對照成品 P08，不以狀態成功代替真操作成功。

### P04 Pet Tauri 入口

參考：`apps/desktop_pet/src/lib.rs`、`apps/desktop_pet/src/main.rs`、`apps/desktop_pet/build.rs`。

成品：`apps/desktop_pet/src/main.rs`、`apps/desktop_pet/build.rs`。

介面：參考四 IPC/get_pet_state/send_pet_command/chat_with_pet/scan_file_preview → 44 IPC。

流程：GUI→invoke→runtimeStore/confirmedRoot→serialized result；preview opaqueID cache。

判定：40 frontend command names 對應44 backend；非參考 bridge/DTO 等同；詳列呼叫端與簽章。

### P05 Pet Frontend

參考：`apps/desktop_pet/frontend/src/bridge.rs`、`apps/desktop_pet/frontend/src/component/activity.rs`、`apps/desktop_pet/frontend/src/component/chat.rs`、`apps/desktop_pet/frontend/src/component/control.rs`、`apps/desktop_pet/frontend/src/component/mod.rs`、`apps/desktop_pet/frontend/src/main.rs`。

成品：`apps/desktop_pet/frontend/src/main.rs`。

介面：window.getPetState/... wrappers + snapshot revision → __TAURI__.core.invoke + settings/asset UI。

流程：參考 listener→apply_snapshot；成品 IPC result→reactive UI；drag權限由Tauri capability。

判定：前端 schema/狀態不同；保留既有功能，不能直接替换參考 component。

### P06 Pet 真視窗漫遊

參考：`apps/desktop_pet/src/roam.rs`。

成品：無等同實作；見對照決策／關聯组。

介面：monitor work_area/current_position/set_position bounded movement。

流程：參考 window moves→verify progress→bounce/stop；成品僅window內角色動畫。

判定：參考功能尚未對應成品；缺失為 native window roaming，不把sprite移动宣稱等同。

### P07 Pet preview/duplicates

參考：`apps/desktop_pet/src/file_organizer.rs`。

成品：無等同實作；見對照決策／關聯组。

介面：AuthorizedRoot/ScanPreview/duplicate_preview SHA256 → runtime AuthorizedRoot/FilePreview。

流程：參考 recursivescan→sizegroups→revalidate+hash；成品 root.scan→opaqueUUID/name/size。

判定：對照P08：安全preview存在，重複檔案雜湊分組尚未對應。

### P08 Pet 真 OS quarantine/restore

參考：無獨立對應模組。

成品：`crates/pet_runtime/src/lib.rs`、`crates/pet_runtime/src/unix_fs.rs`、`crates/pet_runtime/src/windows_fs.rs`。

介面：pinnedroot/preview/confirm/Trash/CleanupReceipt/RestoreReceipt/journal。

流程：確認→revalidate pinned OS entry→隔離/還原→journal receipt；failedDB→recovery→Outbox。

判定：產品安全擴充；FS與DB非單一交易，明確部分成功與恢復，不假稱零窗口。

### P09 Pet Store/Profile 投影

參考：無獨立對應模組。

成品：`crates/pet_runtime/src/state.rs`、`crates/pet_runtime/src/files.rs`、`crates/pet_runtime/src/registry.rs`。

介面：PetStore/memory/cards/conversation/assets/archive/projection。

流程：SQLite canonical→serialized private fileprojection；reconcile；archive active setting+event txn。

判定：產品擴充；對照Profile與五欄概念；資料隔離／filepath不進publicevent。

### P10 Pet 學習/備份

參考：無獨立對應模組。

成品：`crates/pet_domain/src/learning.rs`、`crates/pet_runtime/src/backup.rs`。

介面：Schedule::review/time injected；SQLite snapshots/checksums/restore safety snapshot。

流程：quality→pure Schedule→Card+Outbox txn；VACUUM→hash→private restore migration/merge。

判定：產品擴充；不在Domain匯SQLx，backup與reference preview不是對應功能。

### P11 Pet Chat/Agent/生圖

參考：`apps/desktop_pet/src/chat.rs`。

成品：`crates/pet_runtime/src/ai.rs`、`crates/pet_runtime/src/agent.rs`、`crates/pet_runtime/src/main.rs`。

介面：MockChatModel → actualOpenAI/profile+memories+boundedstdio MCP；generated asset profilehash。

流程：userprompt→profile/memory/history→provider→conversations/eventtxn；assets staleversioncheck。

判定：參考 mock 不等同成品AI；真實AI本輪未執行，readonlytool限制保留。

### P12 Pet Outbox/policy

參考：無獨立對應模組。

成品：`crates/pet_runtime/src/outbox.rs`、`crates/pet_runtime/src/suggestions.rs`。

介面：lease_token/lease_until/CASACK/backoff/blocked/requeue；approved policy receipt。

流程：claim→gRPC→matching ACK→CAS delivered；fail重試/blocked；policy rollback revision。

判定：產品可靠投遞擴充；At-least-once delivery+durable dedup，非無限 exactly-once保證。

### P13 Pet 更新/平台回歸

參考：無獨立對應模組。

成品：`apps/desktop_pet/src/updater.rs`、`apps/desktop_pet/tests/glib_variant_safety.rs`。

介面：check→verify→confirmed install；optimized Linux GLib str iterator regression。

流程：configured signedupdate→downloadverify→install/restart；test platformcfgLinux。

判定：無參考 counterpart；Windows/macOS runtime 本輪未重跑；vendor列獨立清單。

### A01 Analytics reference契約

參考：`apps/stats_server/src/lib.rs`、`apps/stats_server/src/main.rs`、`tests/analytics/event_kind_tests.rs`、`tests/analytics/http_contract_tests.rs`、`tests/analytics/idempotency_contract.rs`、`tests/analytics/postgres_ingest.rs`、`tests/common/mod.rs`。

成品：`apps/stats_server/src/application.rs`。

介面：EventInput/InMemoryDeduplicator/501 → EventRepository/Analytics/RepositoryError。

流程：參考 validinput→501，FakeStore並非PG；成品 receive→durable repository。

判定：參考未實作不是成品應退回501；strict event whitelist不等同任意非空kind。

### A02 Analytics HTTP/入口

參考：無獨立對應模組。

成品：`apps/stats_server/src/lib.rs`、`apps/stats_server/src/main.rs`。

介面：source token ingest/query/admin tokens；HTTP 202/409/400/503；PGhealth。

流程：sourceauth→receive→PG commit→HTTPack；separatequeryauth。

判定：產品擴充；四欄shape對應，驗證/status/security不相容。

### A03 Analytics PostgreSQL

參考：無獨立對應模組。

成品：`apps/stats_server/src/persistence.rs`。

介面：PgPool/EventStore；source+eventID durable dedup/typehash conflict。

流程：BEGIN→dedupkey+event insert→commit/revision trigger；duplicate不重計；query排除archive/withdraw。

判定：產品 durable實作；参考HashSet不能替換；PGmigration非SQLite。

### A04 Analytics gRPC

參考：無獨立對應模組。

成品：`apps/stats_server/src/grpc.rs`、`apps/stats_server/src/grpc/tests.rs`。

介面：EventIngestion.IngestEvents；1..64/64KiB/4fields/outcomes。

流程：wholebatch auth beforewrite→per-event transaction→ACK；Unavailable後可重送已commit事件。

判定：批次非全有全無；來源不可冒用；共享application非旁路SQL。

### A05 Analytics Dashboard/SSE

參考：無獨立對應模組。

成品：`apps/stats_server/src/dashboard.rs`、`apps/stats_server/src/live.rs`。

介面：HTTP queries + SSE revision/polling/max16 streams；fetchBearer。

流程：commit→DBrevision→SSEnotification→dashboardquery；disconnectDBfailurecloses。

判定：SSE非event replay或gRPC替代；aggregate自動更新、recent/trend手動。

### A06 Analytics governance/topics

參考：無獨立對應模組。

成品：`apps/stats_server/src/governance.rs`、`apps/stats_server/src/topics.rs`。

介面：Taipei timeline/archive/restore；topic tombstones/withdraw。

流程：authorizedtransaction→archiveflags/revision；latearrivalprivacy excludes via tombstone。

判定：產品擴充；archive後重送不復活，withdraw不等archive。

### A07 Analytics Insights

參考：無獨立對應模組。

成品：`apps/stats_server/src/insights.rs`。

介面：aggregate→proposal pending/approved/rejected/applied/rolled_back；receipts/evaluate。

流程：grounded stats→AI suggestion→humanreview→targetapply→receipt→descriptiveevaluation。

判定：產品擴充；不把描述性變化當因果/uniqueusers；本輪不執行AI。

### X01 Shared wire/type

參考：`crates/shared/src/lib.rs`。

成品：`crates/shared/src/topics.rs`。

介面：參考 WS DTO → 產品 ProductEvent(C03)/fixedtopiclabels。

流程：typed WSconsumer(C06/C07) 對照 product通知；事件→transport。

判定：同名crate不同職責；禁止把 WS Message本文加入low-sensitivity analytics schema。

### X02 AI Provider

參考：`crates/ai_core/src/lib.rs`。

成品：`crates/ai_core/src/lib.rs`、`crates/ai_core/src/agent.rs`。

介面：ChatModel/ChatRequest/Response/Mock → Provider/Decoder/embedding/sprite/agent tools。

流程：bounded HTTP Responses/Image/embedding→callercheckedstorage；decoder splitschunkframes。

判定：具體Adapter非純Domain；參考trait抽象沒有直接實作等同保證。

### X03 MCP

參考：無獨立對應模組。

成品：`crates/mcp_bridge/src/lib.rs`。

介面：HttpClient/JSONRPC/toolmetadata/header validation；stdio另P11。

流程：host-scopedtools→protocol→boundedexecute；不可tools自行擴scope。

判定：產品 Driver，不進 Domain/Application；實際server另server::mcp/runtime。

### X04 Analytics transport

參考：無獨立對應模組。

成品：`crates/analytics_transport/build.rs`、`crates/analytics_transport/src/lib.rs`。

介面：Tonic/Prost build from shared/proto/analytics.proto；u16↔uint32、UUID↔string。

流程：Outbox→3s connect/10s request→sourceBearer→matchingid/source/Stored或AlreadyExists→ACK。

判定：產品driver，timeout<30s lease；permanentstatusesblocked；proto不存在參考。

### X05 Telemetry

參考：`crates/telemetry/src/lib.rs`。

成品：`crates/telemetry/src/lib.rs`。

介面：參考空crate → OTelinit/exporter/header sanitize/span HTTP metrics。

流程：route/method/status+traceparent→collector；禁止繼承敏感OTelheaders。

判定：產品擴充；沒有空模組功能等同；不以AST說明取代部署collector驗證。

## 驗證與限制

上一輪內容整理已有fmt、Clippy libs -D warnings、pure dependency guard及48項功能測試通過；一個child-process fixture由父測試實際呼叫。證據摘要在 [本輪內容整理驗證](source-review/content-alignment-validation.json)，不是本輪重新跑出的部署證據。

本輪驗證覆蓋率／每檔唯一歸屬、Rust解析、來源檔存在、IPC命令註冊及報告連結。没有重跑PostgreSQL跨服務E2E、Docker正式build、GUI/Windows/macOS或remote CI；沒有驗證新發現授權race的實際可觸發性。

AST呼叫清單不展開巨集、不解析overload/receiver/trait動態dispatch，cfg分支是source-level列冊，不代表各平台皆編譯。Macro內view!/json!/async_stream接線以來源及人工流程核對補充；不是形式化等同性證明。第三方108份vendor另列冊，既有GLib regression的source已檢查，本輪不宣稱所有第三方實作皆安全。

下一步只處理已列缺口／安全風險或妨礙教學對應的部分，保留既有產品功能；不得把「核對完成」改寫為「全部差異已修正」或「兩庫全部程式語意等同」。
