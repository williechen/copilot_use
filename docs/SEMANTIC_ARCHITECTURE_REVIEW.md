# 模組職責與介面對照

目錄與程式銜接以 rust_ai_chat 最新 main 為主，產品前進目標以 copilot_use 為主，其餘按 [共同學習規則](COMMON_LEARNING_RULES.md)。此文件保存目前角色、介面映射及資料流程；每次微調與每週六完整比對後更新。內容整理參考 rust_ai_chat SHA `16261e0ebeef1a888997cc0ebc42b08ceaf8db24`。它不是逐行安全認證或兩邊 wire contract 完全一致的宣告。

**逐模組、介面與資料流程的完整靜態核對已完成**：參考 50／成品 67 份自有 Rust 檔案、38 組模組、26 條流程，詳見 [完整報告](FULL_SOURCE_REVIEW.md)。以下保留簡明職責映射；核對完成不代表缺口已修正或功能驗收通過。

## 當前模組邊界

| 模組 | 公開介面／實際職責及資料流 | 決策與參考銜接 |
|---|---|---|
| chat_domain | MessageInput、Committed、MessageError、validate；作者、ID、房間、本文輸入不變量 | 保留純 Domain；參考 ChatMessage/MessageAuthor 的 ID 字串在成品使用 UUID，作者 kind/author_id 對照表見下節 |
| chat_application | MessageRepository::commit、persist：先驗證，再取得 durable commit | 保留已接線 Port；參考 persist_then_publish 的「儲存失敗不得通知」以交易 realtime/Outbox 實現 |
| persistence::ChatWriter | MessageRepository 實作、persist_with_id、stage、persist_persona_turn；SQLite 單一交易訊息／Outbox／realtime／topic | 保留 Adapter；參考最新 SqliteRoomRepository／SqliteMessageRepository 已採 SQLite；成品保留同交易 Outbox／realtime、既有 migration 與安全備份 |
| persistence::database/topics | 私有 SQLite 開啟、VACUUM INTO 備份、同意撤回與可靠投遞 | 保留 Adapter；migration 與 DB Driver 不進 Domain/Application |
| server::main/auth | HTTP 入口、Session／房間 ACL、輸入映射、備份與舊 PG 匯入；呼叫 application::persist | 保留產品入口；SQL/crypto 在 app 的 Adapter 模組，沒有反向匯入 chat_domain |
| server::realtime/fanout | 第一 frame 驗證、同 origin、房間隔離、DB durable 通知與 canonical 訊息讀取、配額／resync | 保留已驗證通知；與參考 ClientEvent/ServerEvent 格式不相同，教學需明示 protocol 變更 |
| server::openai | SSE delta 暫存，完整完成後 SQLite persist_as，done 只在持久化後送出 | 部分串流不計為完成訊息；完成前未重新複核 session／room ACL 是本輪靜態發現的待修正風險 |
| server::knowledge/mcp/agent/personas | owner-scoped RAG、認證 readonly tools、引用 ID 查證、有限工具迴圈、角色 revision／租約與 durable turn | Agent 已有外部呼叫後 principal／room ACL 複核，Persona 保留 revision／lease／run 檢查；不得概括為所有 AI 入口皆有相同授權檢查 |
| server::suggestions/topic_http | 人工確認、限定設定、CAS 回退、撤回同意 | 保留產品工作流；無需為目錄一致改寫已驗證 API |
| ui/browser、server/frontend | Leptos SSR/hydrate、HTTP／WS／SSE、reconnect／delivery queue；只呼叫服務 API | 保留 UI Adapter，沒有 DB Driver；參考 room_api/chat_socket 課程應對應 API 操作與生命週期 |
| pet_domain::profile | 五欄 Profile、可選外觀／動畫、TOML codec、AssetManifest 素材／顯示尺寸不變量 | 保留原有程式與測試；參考 JSON codec 與成品 TOML 需分開教學 |
| pet_domain::application | ProfileRepository、save_profile：不合法 ID/Profile 不到達 Adapter，投影未完成不回成功 | 保留已接線純 Port，Tauri save_profile 實際使用，不是空 trait |
| pet_domain::learning | Schedule::review：評分、間隔、ease 與 due_at，時間由呼叫端提供 | 從帶 FromRow 的 Card 提取原演算法；不改公式。runtime Card::review 映射並委派，公開 Card 格式保留 |
| pet_runtime::state | PetStore、PetItem/Memory/Card SQL rows、SQLite transaction、ProfileRepository 實作 | Driver 位於 pet_runtime；DB／migration／CLI 與 day9-pet package 名稱保留 |
| pet_runtime::files/registry/backup | 每寵物 TOML／資產、私有目錄、投影序列化、可逆封存、checksum／SQLite snapshot 還原 | 保留已驗證 Adapter、原有資料格式與 migration |
| pet_runtime::AuthorizedRoot/unix_fs/windows_fs | 安全 preview／confirmed quarantine／restore、固定 root、opaque ID、journal／recovery、OS ACL／TOCTOU | 平台 Adapter 保留完整安全流程與回歸；參考 CleanupOperation 是純狀態示例，不取代真實 OS 操作結果 |
| pet_runtime::ai/agent/interaction/suggestions/outbox | 真實 AI／素材、scope-bounded MCP stdio、互動與提醒、政策採用／回退、租約 ACK／重送 | HTTP／SQLite 位於 runtime，不進純 Domain。AI 呼叫與資料格式保持原文 |
| desktop_pet::main/updater、frontend | Tauri IPC／confirmation／rendering、Profile Port、runtime Store、簽章／實際更新冷啟動；前端只 invoke IPC | 只更新 runtime dependency 與 build watch，IPC 命令／回傳格式／更新程式保留 |
| stats_server::application | EventRepository、Analytics::receive/recent/summary/lookup；輸入不合法不進 repository | 保留無 DB Driver 的 Application 模組 |
| stats_server::persistence | PostgreSQL atomic insert/dedup、identity conflict、typed query rows | 保留 DB Adapter；參考 InMemoryDeduplicator/501 是學習基線，不覆蓋真正持久化 |
| stats_server::lib/main/dashboard/grpc/live | source token／query token、HTTP／gRPC 路由、SSE、頁面、健康狀態，Application 接線 | 保留 UI／HTTP／gRPC／SSE Adapter，沒有新增空層 |
| stats_server::governance/insights/topics | 可逆事件封存、描述性統計、人工審核／receipt／evaluate、同意撤回 | 保留匿名事件範圍，不假稱獨立使用者或因果成效 |
| analytics_transport | .proto 生成 Tonic／Prost client/server、事件映射、deadline／逐事件 ACK；Chat／Pet Outbox 共用 | 新需求所需 Driver；純 Domain／Application 不依賴它 |
| shared | ProductEvent 四欄、source/type whitelist、固定 topic 字彙；不包含本文／路徑 | 保留跨產品事件契約；參考 shared 的 WS DTO 是另一通道，不混用 |
| ai_core | package openai-adapter；Decoder、stream_text、embedding、generate_sprite、bounded agent | 保留已驗證 Provider Adapter。參考 ChatModel/MockChatModel 是抽象／測試教學階段；此處是具體 Provider，不宣稱是純 Domain |
| mcp_bridge | HTTP MCP protocol／header／metadata、HttpClient、tool schemas；執行 scope 由 host 控制 | 保留通訊 Adapter，Domain/Application 依賴圖禁止匯入它 |
| telemetry | OTel transport、trace correlation、低敏事件／HTTP指標、剝除繼承認證 header | 保留觀測 Adapter；參考空模組不等於成品沒有 telemetry |
| Cargo／Docker／CI | 根 service workspace、獨立 frontend/native locks、WASM/native/docker 管線 | 加入純層依賴守門；新 pet_runtime 為實際使用 crate，沒有根 tests workspace |

## 三條關鍵資料流程

1. Chat：HTTP／ACL → MessageInput → application::persist/validate → MessageRepository → ChatWriter transaction → 訊息、Outbox、topic、realtime 同時 commit → fanout 讀取 canonical 內容／WS；Outbox 租約以 gRPC／Protobuf 送到 Analytics，失敗保留重試。參考 publish closure 沒有直接拷貝到成品，因為成品通知必須跨程序且 durable。
2. Pet：Tauri save_profile → 純 save_profile Port → runtime PetStore → SQLite commit → 序列化檔案投影；ProjectionPending 原語意保留。複習 IPC → runtime Card → 純 Schedule → 同交易保存 Card／Outbox。cleanup preview 與確認的 OS 操作結果由 runtime 驗證，再保存成功 receipt／Outbox；GUI 和 Analytics 的實際事件 ID 在本地整合回歸核對。
3. Analytics：來源 token／四欄事件驗證 → Analytics::receive → EventRepository → PostgreSQL dedup／product_events commit → gRPC ACK（HTTP 相容入口仍回 2xx）；同來源同 ID 重送不重計，衝突不回成功。Dashboard 以 HTTP 查詢、SSE 依已提交 revision 通知重新查詢；封存／撤回／審核是獨立授權路徑。

## 公開介面與適配

MessageAuthor::AnonymousUser → anonymous_user/None；User{user_id} → user/Some(id)；Ai{persona_id} → ai/Some(id)；System → system/None。這是概念映射，型別與序列化格式並不相同。

參考 Room.category_room_id、RoomRepository/create_room 與成品房間 ACL／owner/topic 沒有一對一契約，不能把 topic 字彙當 category_room_id。後續教學需按參考的實際新增／微調銜接，保留成品房間資料與安全邊界。參考 PetMachine(PetState/PetCommand) 與成品 Action/Speed／動畫選擇也不是同一個 FSM；不建立未使用的同名型別來假裝已對應。

## 核對後的前進項目

內容宣告順序整理保留公開介面與功能，不以檔名一致宣稱語意等同。Chat HTTP／WS／DB、Pet FSM／codec／事件名稱的適配與安全風險逐項記於完整報告；視窗漫遊及 SHA256 重複檔案分組已納入產品目標，尚未實作。

後續只處理已確認缺口、架構邊界及必要教學適配，保留已驗證功能；活動項目以 [FUTURE_WORK.md](FUTURE_WORK.md) 為準。完整靜態核對、週六工程驗收、每日 CI Gate、學習實際套用分開記錄。
