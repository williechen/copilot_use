# 本地驗證紀錄（2026-10-11）

**P9 未完成。** 使用者取消正式部署；不建立正式完成 Release/Tag，不產生 Day 教學。`rust_ai_chat` 未修改。

## 基準與證據範圍

起點 `26f25f62e5ade7538b19c5643df71f9e06f609d1`。已提交 `7bb8fc98465390d0173e2d53a6730713073627c3` 的六項 CI 全通過：Rust/PostgreSQL、Docker Compose E2E、本地故障恢復、Linux deb/GUI、macOS app、Windows NSIS。MCP/Agent、journal、租約、觀測 transport 與建議流程已提交 `5be001d06e9da1a00ebe336b8f529d37d909b1d2`。本地 46 個 workspace 測試、54 項整合檢查通過；其 CI 的 macOS variadic mode 型別錯誤已定位，後續修正仍需新 SHA CI。不得沿用舊提交結果。

唯讀需求基準 `rust_ai_chat` SHA `fdc74c1ee244cf86df3fd1c427d893d2956f6211`，以及 Notion 共同主頁與三產品需求頁。本地 Rust/Cargo 1.99.0、PostgreSQL 18.6、pgvector 0.8.7、Node 22.22.1、Trunk 0.21.14、Tauri CLI 2.11.5。依賴版本固定於各 workspace 的 Cargo.lock。

## 已實際通過

- `scripts/local-verify.py`：fmt、嚴格 Clippy、46 個 workspace 測試；全新隔離 DB migrations、兩個 non-superuser role 的 DB 隔離、HTTP/WS、精確訊息 ID 的跨服務事件、來源 token 與 schema 拒絕、Session/ACL/CSRF/邀請/登出、重啟冪等、服務與 DB 中斷恢復、pg_dump/restore 逐列核對。
- Chat/Pet OpenAI 真實 `gpt-5.6-luna` 對話串流完成與持久化；Chat 真實 embedding → pgvector 檢索 → RAG 回覆 → Outbox/Analytics，以及知識文件刪除。生圖使用 Responses 的 image_generation tool，實際 PNG 與 SQLite 快取已驗證；不是 placeholder。
- Chat 真實 Function Calling → MCP 2026-07-28 → owner-scoped pgvector → Strict JSON → 完整回答/Outbox/Analytics 已通過；確認測試代碼 COMET-432 與引用文件。MCP discovery、header mismatch、Origin 與未知工具拒絕皆以真實 HTTP 驗證。
- Chat cursor API 按 `(created_at,id)` 提供最多 100 筆的早期訊息，拒絕不屬於房間的 cursor；UI 可載入舊訊息且最多 1000 筆。WS 全服務最多 128 連線，cookie Session/ACL 在事件送出前、typing 前與每五秒重新驗證。真實 RFC6455 cookie 測試確認登出後連線關閉，後續私人訊息不會送到被撤銷的 session。
- Chat 同 UUID 併發重送只保存一筆訊息與 Outbox；變更 room/author/content 拒絕。真實瀏覽器驗證 32 筆有上限佇列、離線保留、服務恢復後只送達一筆訊息/Outbox/Analytics，及重新訂閱後收到獨立 WS 通知。佇列只在當頁記憶體保存，離開頁面不留私人內容。
- Leptos SSR 與 WASM hydration 編譯、真實瀏覽器登入/受保護路由/建房/送信/刷新 history/登出測試。舊 `/legacy` HTML 入口已移除。
- Linux Tauri/WebKit/WASM/IPC 實際 GUI 測試：五項設定、多寵物、記憶隔離/刪除、複習排程、opaque ID 預覽/拒絕未確認操作/確認後 receipt、200×200 浮動視窗。真實生成 sprite 角色一致性經目視檢查，任意原圖尺寸縮放到固定顯示框。
- Unix 安全 Adapter 的 symlink、根目錄被替換、檔案變更拒絕與正常隔離測試。Linux 實際通過；macOS CI 實際 11 個 Pet 測試與 app bundle 建置通過，完整 runtime 尚未驗收。Windows 危險移動 fail closed。
- 本機 numeric loopback collector 實收 OpenTelemetry traces/metrics/logs；Chat Outbox 與 Analytics W3C trace 關聯通過；未洩露憑證或對話內容。只允許本機 collector。
- Pet 的 GUI 授權/撤銷、逐檔勾選确认、durable journal/收據復原、收據+事件同交易與冪等、SQLite 原子租約/CAS/過期重領/人工重試均已實測。真實 GUI → SQLite → Analytics 各兩筆清理、一筆複習事件，重複復原不增計，W3C trace 關聯通過。
- 本機 OTLP transport 已固定精確 URL、停用 proxy/redirect 並移除 SDK 會合併的環境 headers；本機 receiver 實證沒有認證 header。
- Pet 真實 OpenAI Function Calling → 本人寵物的 readonly SQLite MCP stdio 子程序 → Strict JSON 回答已通過；驗證 ASTRO-771、引用 memory ID、拒絕其他寵物記憶。子程序沒有 API key 或 GUI/OTEL 環境設定。
- Analytics 真實 OpenAI 結構化 1–3 句洞察以新測試庫的 day/event_type/total 快照產生並持久化；審核權限、未核准拒絕、明確確認、Chat 實際 context limit 變更、Native Pet 套用/回退與穩定收據、描述性事件數評估均實測通過。事件數不足以證明因果、學習品質或人數；此限制保存在評估證據中。
- Analytics 同 ID/同事件冪等、同 ID/不同事件衝突、可逆封存/還原測試通過。永久資料刪除尚未提供。

命令與設定見 [操作說明](DEPLOYMENT.md)。`local-validation-result.json` 是最近一次基礎驗證的局部結果；`live_openai:false` 不代表該次執行有付費呼叫。GUI、生圖、live RAG 各有獨立證據，不能以基礎測試報告取代。

## 尚未完成／未驗證

| 範圍 | 必須補齊 |
|---|---|
| Chat 即時產品 | presence snapshot、typing UI、每使用者連線配額與更完整資源限制、跨程序 fan-out；reload 後不保存未送出私人內容 |
| Chat AI | 多 Persona orchestration、AI-to-AI speaker selection/turn budget/cooldown/context/memory、多 Persona 的工具/結構輸出整合與品質驗收 |
| 共同 AI | Chat/Pet 的本地唯讀 MCP/Agent 已驗證；更多工具、完整權限及品質驗收仍缺 |
| Pet 完整互動 | 進階覆寫 UI、多動作動畫選擇、主動互動、記憶編輯與學習適應的完整 UI/流程 |
| Pet 檔案安全 | Trash/restore、Windows 安全 Adapter、macOS/Windows 完整 GUI 執行與故障邊界 |
| Pet 投遞與備份 | SQLite/asset 備份還原與更完整故障矩陣；租約/CAS、人工重試與 GUI 跨產品 E2E 已實測 |
| Analytics | 登入等完整事件目錄、AI 摘要數字/語義的更完整 grounding 驗收、長期追蹤；新洞察/審核/採用/回退/描述性比較已本地實測，Dashboard 新控制尚需瀏覽器操作驗證 |
| 安全/維運 | 四份 Cargo.lock RustSec 掃描沒有已知漏洞，但 paste/proc-macro-error 有未維護警告；Native glib 0.18.5 有 RUSTSEC-2024-0429 unsoundness 警告；仍需負載/滲透/金鑰輪替、Session 濫用控制、全部故障邊界、UUID/VARCHAR 雙模式驗證 |
| Docker/CI/安裝 | 本機無 Docker/Podman；更新後 Compose/UI/pgvector 配置仍需新 SHA CI。`7bb8fc9` 六項 CI 全通過，含三平台包建置；最新變更仍須其新 SHA CI。updater、Windows/macOS runtime 全驗收尚缺 |
| 最終固定 | 所有必要功能通過後才可正式 release/tag；目前未達門檻 |

## 清理範圍

移除未使用的 `day9/`、重複 `rust-day9.yml`、舊 Chat HTML/JS UI、生圖臨時診斷、產生的 Tauri schema；新增忽略規則防止 .env/SQLite/generated files 誤提交。保留實際產品、各 Cargo.lock、canonical migrations、必要歷史 migration、重現腳本及測試證據。

新增階段證據不代表完整 P9。測試矩陣和未完成清單仍是 Release/Tag 門檻。

RustSec 的 glib 問題修補於 >=0.20.0；目前 Tauri/GTK3 依賴仍使用 0.18.5，尚未完成相容升級或回移修補，不能宣稱安全驗收完成。來源：[RustSec advisory](https://rustsec.org/advisories/RUSTSEC-2024-0429.html)。
