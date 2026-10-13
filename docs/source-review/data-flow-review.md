# 逐資料流程核對

本輪為固定 SHA 的 source-traced review，沒有執行付費 AI 或重跑部署。每條列出入口→用例→Driver→commit/通知、失敗及邊界；引用完整 symbol inventory 補足 private 接線。

## F01 建立房間

參考：[reference-rust_ai_chat/crates/ui/src/room_api.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/crates/ui/src/room_api.rs#L1) → [reference-rust_ai_chat/apps/server/src/http.rs:20](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/apps/server/src/http.rs#L20) → [reference-rust_ai_chat/crates/chat_application/src/lib.rs:35](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/crates/chat_application/src/lib.rs#L35) → [reference-rust_ai_chat/crates/persistence/src/lib.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/crates/persistence/src/lib.rs#L1)。

成品：[copilot_use/crates/ui/src/lib.rs:76](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/ui/src/lib.rs#L76) → [copilot_use/apps/server/src/main.rs:98](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/main.rs#L98) → [copilot_use/apps/server/src/auth.rs:41](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/auth.rs#L41)。

成功／順序：成品 DB房間/owner/membership commit，201；參考UUID與default-category，200。

失敗／恢復：invalid/auth/duplicate error不得當create成功；兩種request不能直接互送。

邊界／差異：HTTP DTO/status/schema及category不等同。

## F02 訊息持久化與即時通知

參考：[reference-rust_ai_chat/crates/ui/src/chat_socket.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/crates/ui/src/chat_socket.rs#L1) → [reference-rust_ai_chat/apps/server/src/ws.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/apps/server/src/ws.rs#L1) → [reference-rust_ai_chat/crates/chat_application/src/lib.rs:49](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/crates/chat_application/src/lib.rs#L49) → [reference-rust_ai_chat/apps/server/src/hub.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/apps/server/src/hub.rs#L1)。

成品：[copilot_use/apps/server/src/main.rs:187](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/main.rs#L187) → [copilot_use/crates/chat_application/src/lib.rs:17](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/chat_application/src/lib.rs#L17) → [copilot_use/crates/chat_domain/src/lib.rs:28](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/chat_domain/src/lib.rs#L28) → [copilot_use/crates/persistence/src/lib.rs:16](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/persistence/src/lib.rs#L16) → [copilot_use/crates/persistence/src/lib.rs:76](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/persistence/src/lib.rs#L76) → [copilot_use/apps/server/src/fanout.rs:12](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/fanout.rs#L12) → [copilot_use/apps/server/src/realtime.rs:73](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/realtime.rs#L73)。

成功／順序：message/outbox/topic/realtime 同 SQLite txn commit；fanout取 canonical message；HTTP replay回200不重計。

失敗／恢復：DB rollback無成功通知；broadcast lag/seq gap通知resync，UI重新讀HTTP。

邊界／差異：参考單程序callback vs成品跨程序durable通知；不能混WS payload。

## F03 Session、CSRF 與房間隔離

參考：無參考 counterpart。

成品：[copilot_use/crates/ui/src/lib.rs:50](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/ui/src/lib.rs#L50) → [copilot_use/apps/server/src/auth.rs:177](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/auth.rs#L177) → [copilot_use/apps/server/src/auth.rs:41](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/auth.rs#L41) → [copilot_use/apps/server/src/auth.rs:72](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/auth.rs#L72) → [copilot_use/apps/server/src/realtime.rs:205](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/realtime.rs#L205)。

成功／順序：tokenhash/expiry存SQLite；cookie/CSRF或服務token；WS首次驗證及週期授權。

失敗／恢復：token錯誤、跨room或Origin不得讀取訊息；throttle保護登入。

邊界／差異：auth Adapter SQL/crypto不進 Domain。

## F04 AI SSE 完成與保存

參考：參考 ai_core MockChatModel，尚無 Chat Provider串流接線。

成品：[copilot_use/apps/server/src/openai.rs:28](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/openai.rs#L28) → [copilot_use/crates/ai_core/src/lib.rs:10](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/ai_core/src/lib.rs#L10) → [copilot_use/apps/server/src/openai.rs:116](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/openai.rs#L116) → [copilot_use/crates/persistence/src/lib.rs:46](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/persistence/src/lib.rs#L46)。

成功／順序：completed+nonblank+size bound→SQLitecommit→done ID；delta是暫態。

失敗／恢復：incomplete/failed/closed/oversized→error/no complete message；本輪沒有付費API測試。

邊界／差異：**待修正風險**：produce 無actor/headers，未重新 principal/ACL；不能以agent已有複核宣稱此入口也有。

## F05 RAG 所有權與 MCP/Agent

參考：參考只有ChatModel mock，無RAG/MCP。

成品：[copilot_use/apps/server/src/knowledge.rs:59](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/knowledge.rs#L59) → [copilot_use/apps/server/src/knowledge.rs:131](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/knowledge.rs#L131) → [copilot_use/apps/server/src/mcp.rs:9](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/mcp.rs#L9) → [copilot_use/apps/server/src/agent.rs:17](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/agent.rs#L17) → [copilot_use/crates/ai_core/src/agent.rs:157](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/ai_core/src/agent.rs#L157)。

成功／順序：embedding1536+ownerfilter/cosinerank；toolcatalog有scope；citations需實際retrieved ID；完成後複核session/room才保存。

失敗／恢復：malformedvector/tool/citation、externalfailure或失授權不保存成功回答。

邊界／差異：MCP傳輸不授予任意資料/檔案能力；document文字不可作系統指令。

## F06 Persona 多輪與租約

參考：參考僅MessageAuthor::Ai，不含run。

成品：[copilot_use/apps/server/src/personas.rs:65](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/personas.rs#L65) → [copilot_use/apps/server/src/personas.rs:177](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/personas.rs#L177) → [copilot_use/apps/server/src/personas.rs:254](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/personas.rs#L254) → [copilot_use/crates/persistence/src/lib.rs:112](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/persistence/src/lib.rs#L112)。

成功／順序：owner→budget/roomlease→persona revision→run status/reservedtokens + turn/memory/message/event同txn。

失敗／恢復：cancel/status/revision/lease變動拒絕commit，不當成功turn。

邊界／差異：persona_id與模型名稱不是可無條件互換的author_id契約。

## F07 Chat 同意與關鍵字撤回

參考：無參考 category_room_id 等同流程。

成品：[copilot_use/apps/server/src/topic_http.rs:34](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/topic_http.rs#L34) → [copilot_use/crates/persistence/src/topics.rs:15](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/persistence/src/topics.rs#L15) → [copilot_use/crates/persistence/src/topics.rs:24](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/persistence/src/topics.rs#L24) → [copilot_use/crates/persistence/src/topics.rs:95](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/persistence/src/topics.rs#L95) → [copilot_use/apps/stats_server/src/topics.rs:34](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/topics.rs#L34)。

成功／順序：consent gating +固定字彙→fourfields；撤回localdisable→withdrawoutbox→PGtombstone→localACK。

失敗／恢復：離線保留pending；late ingest不能復活withdrawn topic；不漏傳本文/path。

邊界／差異：撤回通道保留HTTP，事件上報gRPC；topic不是reference category。

## F08 Chat Outbox→Analytics

參考：參考persisted_event factory未有可靠發送。

成品：[copilot_use/crates/persistence/src/lib.rs:169](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/persistence/src/lib.rs#L169) → [copilot_use/crates/analytics_transport/src/lib.rs:42](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/analytics_transport/src/lib.rs#L42) → [copilot_use/apps/stats_server/src/grpc.rs:19](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/grpc.rs#L19) → [copilot_use/apps/stats_server/src/persistence.rs:34](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/persistence.rs#L34) → [copilot_use/crates/persistence/src/lib.rs:190](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/persistence/src/lib.rs#L190)。

成功／順序：30s lease→10s RPC→PGcommit→id/source相符Stored/AlreadyExists→CASACK delivered。

失敗／恢復：transport/unavailable重試；永久invalid/auth/conflict blocked；stale lease不得ACK。

邊界／差異：at-least-once +durable idempotency，不能宣稱所有故障下 exactly-once delivery。

## F09 Profile / TOML 投影

參考：[reference-rust_ai_chat/crates/pet_domain/src/profile.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/crates/pet_domain/src/profile.rs#L1) → [reference-rust_ai_chat/tests/pet/day5_codec_tests.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/tests/pet/day5_codec_tests.rs#L1)。

成品：[copilot_use/apps/desktop_pet/frontend/src/main.rs:156](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/frontend/src/main.rs#L156) → [copilot_use/apps/desktop_pet/src/main.rs:240](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/src/main.rs#L240) → [copilot_use/crates/pet_domain/src/application.rs:21](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_domain/src/application.rs#L21) → [copilot_use/crates/pet_runtime/src/state.rs:142](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/state.rs#L142) → [copilot_use/crates/pet_runtime/src/files.rs:148](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/files.rs#L148)。

成功／順序：pure validation→canonicalSQLite→serializedprivateTOML；startupreconcile補投影。

失敗／恢復：invalid不碰Adapter；投影失敗ProjectionPending，GUI不稱完整成功。

邊界／差異：reference JSON codec vs產品TOML codec；DB和filesystem投影有部分成功窗口。

## F10 互動、revision、桌面漫遊

參考：[reference-rust_ai_chat/crates/pet_domain/src/lib.rs:62](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/crates/pet_domain/src/lib.rs#L62) → [reference-rust_ai_chat/apps/desktop_pet/src/lib.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/apps/desktop_pet/src/lib.rs#L1) → [reference-rust_ai_chat/apps/desktop_pet/frontend/src/component/control.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/apps/desktop_pet/frontend/src/component/control.rs#L1) → [reference-rust_ai_chat/apps/desktop_pet/src/roam.rs:60](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/apps/desktop_pet/src/roam.rs#L60)。

成品：[copilot_use/apps/desktop_pet/src/main.rs:200](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/src/main.rs#L200) → [copilot_use/crates/pet_runtime/src/interaction.rs:50](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/interaction.rs#L50) → [copilot_use/apps/desktop_pet/frontend/src/main.rs:56](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/frontend/src/main.rs#L56)。

成功／順序：參考FSM snapshot revision→event→apply_snapshot；成品settings→sprite animation。

失敗／恢復：reference invalidtransition errors；成品尚無Sleep/Wake及native window roaming等同流程。

邊界／差異：缺口是功能/狀態契約，不可用改名或畫面內sprite移動冒充。

## F11 安全掃描與重複檔案

參考：[reference-rust_ai_chat/apps/desktop_pet/src/file_organizer.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/apps/desktop_pet/src/file_organizer.rs#L1) → [reference-rust_ai_chat/apps/desktop_pet/frontend/src/bridge.rs:109](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/apps/desktop_pet/frontend/src/bridge.rs#L109)。

成品：[copilot_use/apps/desktop_pet/frontend/src/main.rs:365](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/frontend/src/main.rs#L365) → [copilot_use/apps/desktop_pet/src/main.rs:495](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/src/main.rs#L495) → [copilot_use/crates/pet_runtime/src/lib.rs:150](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/lib.rs#L150)。

成功／順序：root授權→read-onlypreview→opaque IDs/name/size；reference另外sizegroups/SHA256duplicates。

失敗／恢復：root不安全/relativeescape/symlink拒絕；IPC不接受scan任意path。

邊界／差異：**尚未對應** duplicate_preview SHA256分組；參考ScanPreview object vs成品Vec不同。

## F12 確認隔離與收據恢復

參考：[reference-rust_ai_chat/crates/pet_domain/src/cleanup.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/crates/pet_domain/src/cleanup.rs#L1) → [reference-rust_ai_chat/crates/pet_domain/src/cleanup_event.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/crates/pet_domain/src/cleanup_event.rs#L1)。

成品：[copilot_use/apps/desktop_pet/src/main.rs:511](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/src/main.rs#L511) → [copilot_use/crates/pet_runtime/src/lib.rs:193](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/lib.rs#L193) → [copilot_use/crates/pet_runtime/src/unix_fs.rs:356](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/unix_fs.rs#L356) → [copilot_use/crates/pet_runtime/src/windows_fs.rs:593](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/windows_fs.rs#L593) → [copilot_use/crates/pet_runtime/src/state.rs:208](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/state.rs#L208)。

成功／順序：confirmed=true+opaque cache+root revalidate→真 OS rename/receipt→receipt+event SQLite txn。

失敗／恢復：FS成功DB失敗明示須recover_cleanup_receipts；journal確保可復原，不發假成功事件。

邊界／差異：reference純state不證實真FS操作；pet.cleanup.succeeded與file.cleanup.succeeded不相容。

## F13 Trash 還原與不覆蓋

參考：無參考 destructive/restore流程。

成品：[copilot_use/apps/desktop_pet/src/main.rs:567](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/src/main.rs#L567) → [copilot_use/crates/pet_runtime/src/lib.rs:224](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/lib.rs#L224) → [copilot_use/crates/pet_runtime/src/state.rs:252](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/state.rs#L252) → [copilot_use/apps/desktop_pet/src/main.rs:604](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/src/main.rs#L604)。

成功／順序：確認→rootidentity/quarantineidentity→不覆蓋原位置→durablerestore receipt→event txn。

失敗／恢復：新檔案/換root/未确认拒绝；multiprocess/stale journal不雙計。

邊界／差異：unix/windows OSdriver分支；本輪未重新執行Windows runtime。

## F14 Pet 學習、記憶與主動提醒

參考：參考沒有Card/Schedule。

成品：[copilot_use/apps/desktop_pet/src/main.rs:367](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/src/main.rs#L367) → [copilot_use/crates/pet_runtime/src/state.rs:50](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/state.rs#L50) → [copilot_use/crates/pet_domain/src/learning.rs:10](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_domain/src/learning.rs#L10) → [copilot_use/crates/pet_runtime/src/interaction.rs:77](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/interaction.rs#L77)。

成功／順序：quality+callerclock→pureSchedule→Card+Outboxtxn；reminder optin/cooldown。

失敗／恢復：invalid scope/quality拒绝；clock由外部注入，Domain不持clock/sqlx。

邊界／差異：memory/Card/interaction私有SQLite資料，analytics只有type/id。

## F15 Pet 問答與 stdio MCP

參考：[reference-rust_ai_chat/apps/desktop_pet/src/chat.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/apps/desktop_pet/src/chat.rs#L1) → [reference-rust_ai_chat/crates/ai_core/src/lib.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/crates/ai_core/src/lib.rs#L1)。

成品：[copilot_use/apps/desktop_pet/src/main.rs:383](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/src/main.rs#L383) → [copilot_use/crates/pet_runtime/src/ai.rs:10](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/ai.rs#L10) → [copilot_use/apps/desktop_pet/src/main.rs:424](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/src/main.rs#L424) → [copilot_use/crates/pet_runtime/src/agent.rs:11](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/agent.rs#L11) → [copilot_use/crates/pet_runtime/src/main.rs:10](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/main.rs#L10)。

成功／順序：profile+petmemory+history→actualprovider；request/citation/scope checks→conversation+question eventtxn。

失敗／恢復：providerfailure不保存完整回答；tool不能改pet/scope或writeStore。

邊界／差異：reference mock≠真實API；本輪未驗證外部AI provider服務。

## F16 Pet 生圖與顯示比例

參考：reference optional appearance/animation only。

成品：[copilot_use/apps/desktop_pet/src/main.rs:465](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/src/main.rs#L465) → [copilot_use/crates/pet_runtime/src/ai.rs:72](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/ai.rs#L72) → [copilot_use/crates/pet_runtime/src/ai.rs:92](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/ai.rs#L92) → [copilot_use/crates/pet_domain/src/profile.rs:178](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_domain/src/profile.rs#L178) → [copilot_use/apps/desktop_pet/frontend/src/main.rs:39](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/frontend/src/main.rs#L39)。

成功／順序：profilehash→providerimage→manifest validation→persist assets/projection→200×200 aspect-fit。

失敗／恢復：stale profile/invalidasset拒绝，不當目前角色asset；providerfailure不假mock。

邊界／差異：原始PNG尺寸和顯示canvas區分；固定尺寸不是壓扁圖片。

## F17 Pet 封存/快照/還原

參考：無參考對應。

成品：[copilot_use/crates/pet_runtime/src/registry.rs:12](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/registry.rs#L12) → [copilot_use/crates/pet_runtime/src/backup.rs:25](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/backup.rs#L25) → [copilot_use/crates/pet_runtime/src/backup.rs:110](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/backup.rs#L110) → [copilot_use/crates/pet_runtime/src/backup.rs:183](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/backup.rs#L183)。

成功／順序：confirmed archive可逆；VACUUM snapshot+checksums→privatecopy migration→merge保留delivery、新pets→reconcile。

失敗／恢復：驗證失敗拒絕；restore前safety snapshot，失敗保留恢復資料。

邊界／差異：成品SQLite可攜式不代表忽略OSpermissions/sidefiles。

## F18 Pet 可靠上報

參考：參考 cleanup_event factory only。

成品：[copilot_use/crates/pet_runtime/src/outbox.rs:14](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/outbox.rs#L14) → [copilot_use/crates/analytics_transport/src/lib.rs:42](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/analytics_transport/src/lib.rs#L42) → [copilot_use/apps/stats_server/src/grpc.rs:19](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/grpc.rs#L19) → [copilot_use/crates/pet_runtime/src/outbox.rs:18](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/outbox.rs#L18)。

成功／順序：lease/attempts→grpc→committedACK→CASdelivered；failed events可手動requeue。

失敗／恢復：永久invalid/auth/conflict block；stalelease不得mutate。

邊界／差異：pet DB SQLite、Analytics DB PostgreSQL；只傳四欄，不傳profile/memory/path。

## F19 Analytics 接收与 durable 冪等

參考：[reference-rust_ai_chat/apps/stats_server/src/lib.rs:60](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/apps/stats_server/src/lib.rs#L60) → [reference-rust_ai_chat/tests/analytics/postgres_ingest.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/tests/analytics/postgres_ingest.rs#L1)。

成品：[copilot_use/apps/stats_server/src/lib.rs:44](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/lib.rs#L44) → [copilot_use/apps/stats_server/src/application.rs:51](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/application.rs#L51) → [copilot_use/apps/stats_server/src/persistence.rs:34](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/persistence.rs#L34)。

成功／順序：sourcecredential+strictwhitelist→dedup+event txn→202/duplicate；revision trigger同txn。

失敗／恢復：same(source,id)differenttype409；invalid400；DB503不成功ACK。

邊界／差異：reference501/FakeStore不等PG；source+id而非globalUUID identity。

## F20 gRPC 批次与 Protobuf

參考：無參考 counterpart。

成品：[copilot_use/crates/analytics_transport/src/lib.rs:23](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/analytics_transport/src/lib.rs#L23) → [copilot_use/apps/stats_server/src/grpc.rs:19](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/grpc.rs#L19) → [copilot_use/apps/stats_server/src/application.rs:51](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/application.rs#L51)。

成功／順序：wholebatch來源authbeforewrite；各事件自己的transaction；Stored/AlreadyExists ACK onlycommit。

失敗／恢復：1..64/64KiB；非法個別outcome；Unavailable可部分已提交，重試冪等。

邊界／差異：uint32轉u16溢位和UUID parse必檢查；HTTPquery不改gRPC。

## F21 Dashboard HTTP+SSE

參考：reference只health/501 stub。

成品：[copilot_use/apps/stats_server/src/dashboard.rs:5](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/dashboard.rs#L5) → [copilot_use/apps/stats_server/src/live.rs:20](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/live.rs#L20) → [copilot_use/apps/stats_server/src/live.rs:15](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/live.rs#L15) → [copilot_use/apps/stats_server/src/application.rs:39](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/application.rs#L39)。

成功／順序：Bearerqueryauth→snapshot/revision changednotification→count/summary重新HTTP；keepalive。

失敗／恢復：max16 streams；DB errorclose；disconnect可reconnect重新snapshot；recent/trendmanual。

邊界／差異：SSE無逐event replay；commitnotification不是整個DBevent內容。

## F22 事件封存与同意撤回

參考：無參考 counterpart。

成品：[copilot_use/apps/stats_server/src/governance.rs:44](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/governance.rs#L44) → [copilot_use/apps/stats_server/src/topics.rs:34](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/topics.rs#L34) → [copilot_use/apps/stats_server/src/persistence.rs:52](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/persistence.rs#L52)。

成功／順序：admin archive/restore；sourceauth withdrawal+privacy tombstone；queries排除withdrawn/archived。

失敗／恢復：retry不復活archived/withdrawn；invalidsource/credential拒绝。

邊界／差異：schema migrations SQL authority，不把歷史PGchat migration當目前SQLite migrations。

## F23 Insights、核准、套用/回退

參考：無參考 counterpart。

成品：[copilot_use/apps/stats_server/src/insights.rs:78](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/insights.rs#L78) → [copilot_use/apps/stats_server/src/insights.rs:146](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/insights.rs#L146) → [copilot_use/apps/server/src/suggestions.rs:49](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/server/src/suggestions.rs#L49) → [copilot_use/crates/pet_runtime/src/suggestions.rs:6](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/pet_runtime/src/suggestions.rs#L6) → [copilot_use/apps/stats_server/src/insights.rs:171](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/insights.rs#L171) → [copilot_use/apps/stats_server/src/insights.rs:204](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/stats_server/src/insights.rs#L204)。

成功／順序：aggregatefacts→boundedproposal→humanapprove→target限定policy/CASrevision→receipt→描述性比較。

失敗／恢復：未approved/錯target/stalerevision拒绝；remote receipt失敗不隱藏local部分成功。

邊界／差異：AI生成與功能週六驗證分開；沒有userlevelpayload或因果推論。

## F24 更新、GUI與原生建置

參考：reference bundle.active=false、320×360單mainwindow。

成品：[copilot_use/apps/desktop_pet/src/updater.rs:64](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/src/updater.rs#L64) → [copilot_use/apps/desktop_pet/src/updater.rs:120](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/src/updater.rs#L120) → [copilot_use/apps/desktop_pet/src/updater.rs:164](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/src/updater.rs#L164) → [copilot_use/apps/desktop_pet/tests/glib_variant_safety.rs:4](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/apps/desktop_pet/tests/glib_variant_safety.rs#L4)。

成功／順序：配置signedlocalupdate→downloadverify→confirmedbackup/install/restart；frontendinvoke capability。

失敗／恢復：沒有配置pubkey/endpoints不能宣稱正式update可用；本輪未重建package或驗證install。

邊界／差異：beforeBuildCommand默认frontend cwd為native父apps，../scripts指向root/scripts；不是當前已證實buildpath錯誤。

## F25 Driver 邊界与 Telemetry

參考：[reference-rust_ai_chat/crates/telemetry/src/lib.rs:1](https://github.com/williechen/rust_ai_chat/blob/16261e0ebeef1a888997cc0ebc42b08ceaf8db24/crates/telemetry/src/lib.rs#L1)。

成品：[copilot_use/crates/telemetry/src/lib.rs:86](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/telemetry/src/lib.rs#L86) → [copilot_use/crates/telemetry/src/lib.rs:237](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/telemetry/src/lib.rs#L237) → [copilot_use/crates/telemetry/src/lib.rs:168](https://github.com/williechen/copilot_use/blob/cd3df07faef994eb544b0648c2e44f17d1853302/crates/telemetry/src/lib.rs#L168)。

成功／順序：route/method/status spans→configuredcollector；traceparent穿過Outbox，不載入requestsecret。

失敗／恢復：sanitize inheritedOTelcredentialheaders；collector failure不當業務commit成功。

邊界／差異：puredependency guard檢查normal/build graph；測試devdriver允许；thirdpartyGLib108份只列獨立inventory。

## F26 Docker/CI資料與建置

參考：reference workspace包含root tests/native/frontend。

成品：成品 scripts/build-chat-ui.sh→Dockerfile builder→Chat/Analyticsimages→compose backendnetwork/PGvolume/SQLitevolume；Tauri獨立native/frontendmanifest/lock；verify.yml→owningcrate/PG/compose/GUI/package jobs。

成功／順序：Chat/Pet SQLite、Analytics PG；設定名稱與migration來源已追踪；Docker不包含桌面GUI。

失敗／恢復：本輪未dockerup/CIremote/nativepackaging，歷史release附件不能當目前SHA驗證；secrets只確認變數名不讀實際值。

邊界／差異：reference testworkspace不移植；cargo-leptos使用--cargo-locked（不接受--locked）。
