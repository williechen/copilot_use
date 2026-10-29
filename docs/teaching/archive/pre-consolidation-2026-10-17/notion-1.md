Here is the result of "fetch" for the Page with URL https://app.notion.com/p/3f3e51d45093807da638c336e857543a as of 2026-10-16T16:47:27.655Z:
<page url="https://app.notion.com/p/3f3e51d45093807da638c336e857543a">
<ancestor-path>
<parent-page url="https://app.notion.com/p/3f2e51d4509381ddb8a8ce7cb4cd3e60" title="Rust AI 三專案｜共同學習規則與前進目標"/>
</ancestor-path>
<properties>
{"title":"聊天室"}
</properties>
<iconMetadata>null</iconMetadata>
<content>
## 1. 大綱
目錄／程式銜接以 rust_ai_chat 最新 main 為主，目標／前進項目以 copilot_use 實際產品為主，其餘遵循共同主頁。以下為未來學習銜接項目，產品驗收與學習套用分開。
## 2. 最終目標
多人多房間／多 Persona Chat：Session／ACL、訊息／history、SSR／hydration、WS presence／typing／reconnect／queue、SQLite／Outbox、真實 streaming／RAG／MCP／Agent、OTel。
## 3. 後續缺口
- 不預排下一 Day；依最新 main 與已核實缺口採差異驅動、TDD Red → 最小 Green → Refactor → 驗證，選擇可實作的最小功能切片；必要測試／重構與阻塞修正補在原 Day 第 2 節，教學固定四節。原開發項目跨階段保留，功能完成後回填原階段證據；CI 通過不等於學習或正式驗收完成。
- 銜接訊息／作者／房間、MessageRepository、SQLite schema／history／備份、HTTP／WS／UI。
- 銜接 Session／ACL、跨程序通知、真實 AI／多 Persona、同意撤回、政策採用／回退及 E2E。
每次先核對最新 source，排除已套用項目；不把成品成功算成學習已完成。
## 4. CI/CD
測非法輸入、DB 失敗不通知、交易／Outbox 原子性、重送／精確 ID、房間隔離、HTTP／WS／SQLite／UI／必要 E2E。fmt、Clippy、unit／integration、WASM 與 matching SHA CI 分開記錄。
歷史 CI 不累積於主頁，真正結果與來源分開保存。
## 5. 開發與執行
rust_ai_chat 只讀；copilot_use 對應 apps/server、crates/chat_domain、chat_application、persistence、ui 與 apps/server/frontend。Chat 目標為可攜式 SQLite，SQL／migration 放 Adapter，測試放 owning app／crate。
[操作設定](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/DEPLOYMENT.md) · [未來清單](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/FUTURE_WORK.md) · [依當日差異選題](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/FUTURE_WORK.md)
<page url="https://app.notion.com/p/3fbe51d450938149af5dd68abc6109bd">2026-10-16｜P0 Chat Day 14｜以 HTTP 讀回 SQLite 房間資料</page>
<page url="https://app.notion.com/p/3fae51d4509381e499e5e02cfba58336">2026-10-16｜P0 Chat Day 13｜HTTP 建房名稱驗證與 201 Created</page>
<page url="https://app.notion.com/p/3fae51d45093812bbf88c3e7f51d6522">2026-10-15｜P0 Chat Day 12｜WebSocket 房間 Typing 即時通知</page>
<page url="https://app.notion.com/p/3f9e51d4509381508c0dd3ca22cf5df7">2026-10-15｜P0 Chat Day 11｜WebSocket 寫入時重新核對房間</page>
<page url="https://app.notion.com/p/3f9e51d45093818a849dd8b1fbf2c045">2026-10-14｜P0 Chat Day 10｜防止錯誤房間查詢結果寫入訊息</page>
<page url="https://app.notion.com/p/3f8e51d45093814b992eea570411bd7d">2026-10-14｜P0 Chat Day 9｜房間不存在時禁止持久化訊息</page>
<page url="https://app.notion.com/p/3f8e51d450938174a707d200f2e0de52">2026-10-13｜正式補跑｜Chat｜持久化成功後通知的對稱測試</page>
<page url="https://app.notion.com/p/3f8e51d45093816690f8d5b4ff04ff70">2026-10-13｜手動補跑｜Chat｜既有 MessageRepository 的 SQLite 遷移切片</page>
</content>
</page>