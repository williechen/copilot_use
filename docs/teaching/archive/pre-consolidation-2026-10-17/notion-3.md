Here is the result of "fetch" for the Page with URL https://app.notion.com/p/3f3e51d45093817eafaee0cfc9ea4cc8 as of 2026-10-16T16:47:29.863Z:
<page url="https://app.notion.com/p/3f3e51d45093817eafaee0cfc9ea4cc8" icon="📊">
<ancestor-path>
<parent-page url="https://app.notion.com/p/3f2e51d4509381ddb8a8ce7cb4cd3e60" title="Rust AI 三專案｜共同學習規則與前進目標"/>
</ancestor-path>
<properties>
{"title":"統計儀錶板｜聊天室 × 桌面寵物"}
</properties>
<iconMetadata>{"type":"emoji","emoji":"📊"}</iconMetadata>
<content>
## 1. 大綱
目錄／程式銜接以 rust_ai_chat 最新 main 為主，目標／前進項目以 copilot_use 實際產品為主，其餘遵循共同主頁。以下為未來學習銜接項目，產品驗收與學習套用分開。
## 2. 最終目標
獨立 PostgreSQL；事件上報採 gRPC／Protobuf，Dashboard 保留 HTTP 查詢＋SSE 即時更新，HTTP／gRPC 共用同一服務與連接埠。來源認證、四欄低敏事件、持久冪等／衝突、明細／Taipei 趨勢、可逆封存／同意撤回／備份、可追溯 AI 洞察、審核／採用收據／描述性評估／回退、OTel；另須記錄各 AI Provider／Model／功能來源的 Token 使用量、可取得的額度上限與剩餘量、費用及用量趨勢。Chat／Pet 自主授權採用建議。
## 3. 後續缺口
- 不預排下一 Day；依最新 main 與已核實缺口採差異驅動、TDD Red → 最小 Green → Refactor → 驗證，選擇可實作的最小功能切片；必要測試／重構與阻塞修正補在原 Day 第 2 節，教學固定四節。原開發項目跨階段保留，功能完成後回填原階段證據；CI 通過不等於學習或正式驗收完成。
- 銜接 .proto 四欄契約、batch／大小限制、逐事件處理結果、來源／ID 相符才 ACK，以及 Dashboard HTTP＋SSE 的提交後通知、清除憑證及故障重連。
- 銜接 same-source/event ID 重送／並發／重啟／identity conflict、HTTP 與 DB 測試。
- 銜接 Query／Dashboard、資料治理、同意撤回／備份、洞察審核／採用／回退、跨服務事件／trace。
- AI Token 使用量與額度：依 Provider、Model、來源產品（Chat／Pet／Analytics）、功能、請求 ID、時間記錄 input／output／cached input／reasoning tokens（以供應商實際回傳欄位為準）、total tokens、計價單位與估算／實際成本；提供日／月統計、各模型用量、預算警示及可取得的 rate limit／quota／remaining／reset。Token 用量不等於 API 配額或帳單；未提供的額度標示「未知」，不得推算為 0 或假稱可讀取。失敗、重試、串流、中斷、缺失 usage 的情況須定義去重與補帳規則；API key、Prompt、回應本文不得寫入統計事件。此為後續每日學習的功能需求，Dry Run 僅核對與修正文案，不修改程式。
- 事件只表達真正動作；匿名數量不代表獨立真人或因果成效，Trash 不冒稱釋放空間。新事件先按 copilot_use 目標定義與驗收。
每次先核對最新 source，排除已套用項目；不把成品成功算成學習已完成。
## 4. CI/CD
測 gRPC 未授權／來源隔離、schema／batch／payload、DB 失敗不得成功 ACK、唯一／並發／重啟去重、identity conflict、UUID／VARCHAR、DB 隔離、HTTP query／GUI、SSE commit／rollback／重連、撤回／封存／備份、真實來源及 OTel／E2E。記憶體去重和 DB 持久性、產品事件和 traces 分開。AI 用量另測 provider usage 欄位缺漏、模型分類、重試／串流最終結算、同一請求不重複計費、成本價格版本、額度未知與警示門檻；不把 Token 用量誤作剩餘配額。
歷史 CI 不累積於主頁，真正結果與來源分開保存。
## 5. 開發與執行
rust_ai_chat 只讀；copilot_use 對應 apps/stats_server application／persistence／grpc／live／HTTP／Dashboard 與所屬 migrations；crates/shared/proto 定義契約，crates/analytics_transport 為獨立 Driver。
[通訊契約與操作](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/ANALYTICS_TRANSPORT.md)。新需求不回寫固定正式 tag；產品程式更新不等於 rust_ai_chat 已套用。Domain／Application 不引入 SQLx／PgPool；Driver 在 Adapter。Chat／Pet 各自 SQLite，本服務 PostgreSQL。
[操作設定](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/DEPLOYMENT.md) · [未來清單](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/FUTURE_WORK.md) · [依當日差異選題](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/FUTURE_WORK.md)
<page url="https://app.notion.com/p/3fbe51d4509381f2a9a2e4b608706eea">2026-10-16｜P0 Analytics Day 14｜依事件類型篩選來源統計</page>
<page url="https://app.notion.com/p/3fae51d4509381808a9cd813423d52d1">2026-10-16｜P0 Analytics Day 13｜來源隔離的 PostgreSQL 統計查詢</page>
<page url="https://app.notion.com/p/3fae51d4509381ebb1e9f8482b5bf44e">2026-10-15｜P0 Analytics Day 12｜HTTP 來源驗證與 PostgreSQL 持久化接線</page>
<page url="https://app.notion.com/p/3f9e51d45093814a8054d44c4ff9fa55">2026-10-15｜P0 Analytics Day 11｜PostgreSQL 事件儲存與冪等</page>
<page url="https://app.notion.com/p/3f9e51d4509381269984c5f49a429b5d">2026-10-14｜P0 Analytics Day 10｜有效事件的來源與 ID 去重入口</page>
<page url="https://app.notion.com/p/3f8e51d45093810d830ef3fd40e419f0">2026-10-14｜P0 Analytics Day 9｜事件類型前後空白拒絕</page>
<page url="https://app.notion.com/p/3f8e51d45093816e96edd35b83543ed3">2026-10-13｜正式補跑｜Analytics｜來源與版本驗證測試</page>
<page url="https://app.notion.com/p/3f8e51d4509381689c98ed86fecc50a2">2026-10-13｜手動補跑｜Analytics｜現有 EventInput 的有效識別碼前置驗證</page>
</content>
</page>