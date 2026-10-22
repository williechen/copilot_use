# 模組與程式呼叫鏈

以下核對 copilot_use/main 的現有入口與核心符號；[來源及文件驗證](teaching/history/evidence/2026-10-22-current-docs/README.md) 保存精確 commit。本次為靜態接線核對，沒有重跑 UI／DB／跨服務功能測試。兩庫歷次完整比較仍以 [原報告](FULL_SOURCE_REVIEW.md) 的原 SHA 為準。

| 起點 | 核心／Adapter | 回饋及需核對的邊界 |
| --- | --- | --- |
| [Chat main 路由](../apps/server/src/main.rs) | auth、房間／訊息 HTTP、realtime／fanout | Session／ACL 由後端處理；前端路由守衛不能替代 |
| [chat_application::persist](../crates/chat_application/src/lib.rs) | MessageRepository、[ChatWriter](../crates/persistence/src/lib.rs) | 先驗證，再提交訊息／Outbox／realtime；失敗不能通知成功 |
| Chat UI HTTP／WS／SSE | [crates/ui](../crates/ui/src/lib.rs) → server Adapter | 房間隔離、重連、queue 與權限須有實際流程測試 |
| [Tauri save_profile](../apps/desktop_pet/src/main.rs) | [純 save_profile Port](../crates/pet_domain/src/application.rs) → [PetStore](../crates/pet_runtime/src/state.rs) | SQLite 與檔案投影分開；投影未完成不得當完整成功 |
| Pet 事件投遞 | [runtime Outbox](../crates/pet_runtime/src/outbox.rs) → analytics_transport | 真實操作 receipt、來源 token、ACK／重試，不由 UI 自稱成功 |
| [Analytics 路由](../apps/stats_server/src/lib.rs) | application → persistence → PostgreSQL | commit 後 ACK；同來源同 ID 去重與 payload 衝突分開 |
| Analytics Dashboard／管理 | dashboard、live、governance、insights | 現行 query／admin Bearer token；人員 Session 及角色仍是產品需求 |
| [gRPC Driver](../crates/analytics_transport/src/lib.rs) | shared proto 契約 → Tonic／Prost | 純核心不匯入傳輸 driver；HTTP JSON 相容入口另核對 |

## 參考適配不能省略的語意差異

既有比較已指出以下差異，仍須按每次兩庫精確 SHA 核對，不把原比較當最新自動結果：

- 作者與 ID：概念 AnonymousUser／User／AI／System 與產品 author kind／可選 ID 分開映射；UUID 與字串不能僅靠同名型別視為相容。
- 房間：Room category 與產品 owner／ACL／topic 無一對一契約，topic 不代表 category。
- 即時訊息：參考 ClientEvent／ServerEvent 與產品 WS frame、durable realtime／Outbox 格式分開教學與測試。
- Pet：PetMachine／Sleep／Wake 與產品 Action／Speed／動畫不是同一狀態機；Profile JSON 與 TOML 需明列 codec。
- 檔案操作：純 CleanupOperation 狀態不替代實際 OS preview／確認／隔離／restore receipt，事件名稱不得默改。
- Analytics：學習庫 HTTP／PostgreSQL 與產品 gRPC ACK／SSE／來源授權分別驗收；舊 501、記憶體去重或歷史 CI 不當目前通過證據。

## 邊界檢查與待辦

`python3 scripts/architecture-boundaries.py --offline` 依 Cargo 依賴圖檢查 chat-domain、chat-application、pet-domain 的 normal／build 依賴；DB、HTTP、Tauri、Provider／MCP 等 Driver 不得回流。此檢查不取代函式行為與 GUI 驗收。

視窗漫遊、SHA256 分組、SSE 保存前授權複核的原 Linux 證據保留於 [指定來源報告](weekly/2026-10-16/acceptance.json)。新增需求、跨平台及目前版本差異依 [開發清單](FUTURE_WORK.md) 追蹤，不因本頁整理重新宣稱完成。改寫前詳細模組表與舊參考 SHA 原文保留於 [文件封存](teaching/history/evidence/2026-10-22-current-docs/README.md)。
