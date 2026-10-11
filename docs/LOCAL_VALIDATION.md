# 本地驗證紀錄（2026-10-12）

**P9 驗收進行中，尚未固定正式 Release/Tag。** 使用者取消對外部署，仍要求本地 Docker 正式建置與測試。通過必要驗收後才固定正式版本，並建立 `copilot_use` 教學對齊分支。`rust_ai_chat` 保持唯讀。教學測試隨各模組／專案放置，不集中在最外層 `tests/`；正式 tag 前不產生 Day 教學。

## 基準與證據範圍

起點 `26f25f62e5ade7538b19c5643df71f9e06f609d1`。已提交 `7bb8fc98465390d0173e2d53a6730713073627c3` 的六項 CI 全通過：Rust/PostgreSQL、Docker Compose E2E、本地故障恢復、Linux deb/GUI、macOS app、Windows NSIS。MCP/Agent、journal、租約、觀測 transport 與建議流程已提交 `5be001d06e9da1a00ebe336b8f529d37d909b1d2`。最新本地基礎、真實 OpenAI/Persona/Pet GUI 整合通過；`990f703` 的 macOS 型別修正與六項 CI 已通過，新增功能仍須新 SHA CI。不得沿用舊提交結果。

唯讀需求基準 `rust_ai_chat` SHA `fdc74c1ee244cf86df3fd1c427d893d2956f6211`，以及 Notion 共同主頁與三產品需求頁。本地 Rust/Cargo 1.99.0、PostgreSQL 18.6、pgvector 0.8.7、Node 22.22.1、Trunk 0.21.14、Tauri CLI 2.11.5。依賴版本固定於各 workspace 的 Cargo.lock。

## 已實際通過

- `scripts/local-verify.py`：fmt、嚴格 Clippy、48 個 workspace 測試；全新隔離 DB migrations、兩個 non-superuser role 的 DB 隔離、HTTP/WS、精確訊息 ID 的跨服務事件、來源 token 與 schema 拒絕、Session/ACL/CSRF/邀請/登出、重啟冪等、服務與 DB 中斷恢復、pg_dump/restore 逐列核對。
- Chat/Pet OpenAI 真實 `gpt-5.6-luna` 對話串流完成與持久化；Chat 真實 embedding → pgvector 檢索 → RAG 回覆 → Outbox/Analytics，以及知識文件刪除。生圖使用 Responses 的 image_generation tool，實際 PNG 與 SQLite 快取已驗證；不是 placeholder。
- Chat 真實 Function Calling → MCP 2026-07-28 → owner-scoped pgvector → Strict JSON → 完整回答/Outbox/Analytics 已通過；確認測試代碼 COMET-432 與引用文件。MCP discovery、header mismatch、Origin 與未知工具拒絕皆以真實 HTTP 驗證。
- Chat cursor API 按 `(created_at,id)` 提供最多 100 筆的早期訊息，拒絕不屬於房間的 cursor；UI 可載入舊訊息且最多 1000 筆。WS 全服務最多 128 連線，cookie Session/ACL 在事件送出前、typing 前與每五秒重新驗證。真實 RFC6455 cookie 測試確認登出後連線關閉，後續私人訊息不會送到被撤銷的 session。
- Chat 同 UUID 併發重送只保存一筆訊息與 Outbox；變更 room/author/content 拒絕。真實瀏覽器驗證 32 筆有上限佇列、離線保留、服務恢復後只送達一筆訊息/Outbox/Analytics，及重新訂閱後收到獨立 WS 通知。佇列只在當頁記憶體保存，離開頁面不留私人內容。
- Leptos SSR 與 WASM hydration 編譯、真實瀏覽器登入/受保護路由/建房/送信/刷新 history/登出測試。舊 `/legacy` HTML 入口已移除。
- Linux Tauri/WebKit/WASM/IPC 實際 GUI 測試：五項設定、多寵物、進階覆寫保存、記憶隔離/編輯/刪除、複習排程、opaque ID 預覽/拒絕未確認操作/確認後 receipt、200×200 浮動視窗。真實生成 sprite 角色一致性經目視檢查，任意原圖尺寸縮放到固定顯示框。
- Unix 安全 Adapter 的 symlink、根目錄被替換、檔案變更拒絕與正常隔離測試。Linux 實際通過；macOS CI 實際 11 個 Pet 測試與 app bundle 建置通過，完整 runtime 尚未驗收。Windows 危險移動 fail closed。
- 本機 numeric loopback collector 實收 OpenTelemetry traces/metrics/logs；Chat Outbox 與 Analytics W3C trace 關聯通過；未洩露憑證或對話內容。只允許本機 collector。
- Pet 的 GUI 授權/撤銷、逐檔勾選确认、durable journal/收據復原、收據+事件同交易與冪等、SQLite 原子租約/CAS/過期重領/人工重試均已實測。真實 GUI → SQLite → Analytics 各兩筆清理、一筆複習事件，重複復原不增計，W3C trace 關聯通過。
- 本機 OTLP transport 已固定精確 URL、停用 proxy/redirect 並移除 SDK 會合併的環境 headers；本機 receiver 實證沒有認證 header。
- Pet 真實 OpenAI Function Calling → 本人寵物的 readonly SQLite MCP stdio 子程序 → Strict JSON 回答已通過；驗證 ASTRO-771、引用 memory ID、拒絕其他寵物記憶。子程序沒有 API key 或 GUI/OTEL 環境設定。
- Analytics 真實 OpenAI 結構化 1–3 句洞察以新測試庫的 day/event_type/total 快照產生並持久化；審核權限、未核准拒絕、明確確認、Chat 實際 context limit 變更、Native Pet 套用/回退與穩定收據、描述性事件數評估均實測通過。事件數不足以證明因果、學習品質或人數；此限制保存在評估證據中。
- Analytics 同 ID/同事件冪等、同 ID/不同事件衝突、可逆封存/還原測試通過。永久資料刪除尚未提供。

命令與設定見 [操作說明](DEPLOYMENT.md)。`local-validation-result.json` 是最近一次基礎驗證的局部結果；`live_openai:false` 不代表該次執行有付費呼叫。GUI、生圖、live RAG 各有獨立證據，不能以基礎測試報告取代。

## 最新驗收與剩餘門檻

`f9ff699a969dcf94fb1e2eff99df05a7c752c02f` 的 CI：Rust/PostgreSQL、Docker Compose、故障恢復、Linux desktop、macOS desktop 通過；Windows desktop 失敗。不可宣稱六項全過。Windows 實際測試發現唯讀 handle 無法 flush 備份，以及回收檔案 ACL 還原失敗；目前修正需要新 SHA 的 Windows 實跑確認。

最新本地工作樹已通過（尚需提交後 CI，結果不能冒稱來自 f9ff699）：
- 真實 512 個 cookie WebSocket、五個 Chat 程序：全域／每人／每程序超限拒絕；64 個併發私人訊息各送至 32 個訂閱者一次，history 與 Analytics 精確事件 ID 一致。p50 49.63 ms，p95 64.24 ms，範圍僅此本機合成負載。
- 真實瀏覽器 presence／typing／跨客戶端訊息；OpenAI Persona 建立、保存、完成發言、切換房間後取消原 run，收據保留一個已完成回合。
- 真實 OpenAI 只選擇 Analytics 快照事實索引；瀏覽器審核、Chat 採用收據、描述性前後比較、回退及狀態更新通過。
- Linux GUI 實際到期題主動提醒顯示與一小時冷卻通過。
- 備份交易故障完整回滾、操作前快照保留，以及舊 schema 在私人工作副本升級後還原通過；原始備份 bytes 完全不变。
- 真實 Chat 管理／Analytics 查詢／來源 ingest 金鑰輪替：舊憑證撤銷、新憑證跨服務投遞成功。
- profile hash 阻止過時生圖結果覆蓋或顯示已修改角色；unit 與真實 OpenAI 生圖／快取／修改後停用舊素材／原設定恢復快取均通過。

| 必要門檻 | 剩餘驗證 |
|---|---|
| Windows 安全與 GUI | 新 SHA 上實跑備份 flush、handle-relative ACL 還原、安全測試、WebView GUI 與 NSIS |
| 安裝包 runtime | Linux deb 解包後的 release 程式已通過 GUI／SQLite／回收還原／簽章驗收；macOS app 與 Windows 安装包加入 CI，尚待結果 |
| Updater | Linux/macOS 簽章拒絕邊界通過；Linux AppImage 真實 0.1.0→0.1.1 安裝、checksum、冷啟動版本、既有寵物/SQLite 完整性與更新前快照已過。本地測試私鑰不進 repository |
| Analytics Dashboard | 已通過 Taipei 當日趨勢、實際封存 13→12、還原 12→13 的瀏覽器操作 |
| 身份／架構 | 實際 Analytics Adapter 的原生 UUID↔VARCHAR(40) 轉換、並發去重/衝突/查詢及完整 VARCHAR HTTP/WS/故障 E2E 通過，domain/API 保持 UUID；正式 tag 後新 branch 對齊目錄與模組內測試 |
| 安全／來源固定 | 最新四份 lock audit、手工 GLib 回移證據、新提交全部 CI；同一乾淨 SHA 的 Docker 重建、容器測試、安裝包與輸出校驗 |
| 交付固定 | 更新文件、移除無用暫存、完成上列門檻後才正式 release/tag；目前尚未完成 |

## 清理範圍

移除未使用的 `day9/`、重複 `rust-day9.yml`、舊 Chat HTML/JS UI、生圖臨時診斷、產生的 Tauri schema；新增忽略規則防止 .env/SQLite/generated files 誤提交。保留實際產品、各 Cargo.lock、canonical migrations、必要歷史 migration、重現腳本及測試證據。

新增階段證據不代表完整 P9。測試矩陣和未完成清單仍是 Release/Tag 門檻。

RustSec 的 glib 問題修補於 >=0.20.0；目前 Tauri/GTK3 依賴仍使用 0.18.5，已將官方 `let mut p`/`&mut p` 修正回移到 MIT 相容來源 `vendor/glib-0.18.5`；所有受影響 iterator 方法的 release regression 通過，並加入 CI；其餘相容升級與完整安全驗收尚未完成，不能宣稱安全驗收完成。來源：[RustSec advisory](https://rustsec.org/advisories/RUSTSEC-2024-0429.html)。

清理紀錄：已移除任務暫存發布 payload、過時 fixture/進度檔、舊測試庫及重複日誌；移除可重建增量編譯快取，保留 source/lock/migration/有效證據與唯讀參考 clone。

新增已測功能：
- 私人回收區 list/確認 restore/NOREPLACE/身份核對/跨程序 flock；journal 還原收據與 SQLite 事件同交易，重送及重啟不重複計數。實際 GUI 操作還原 sample.txt，Analytics 精確一筆 `file.restore.succeeded`。
- Chat 以 transaction PG NOTIFY 的 message ID 跨程序通知，讀取 canonical DB content；兩個獨立程序驗證訊息與 typing 隔離、presence snapshot。每人 32／全系統 512／每程序 128 連線上限；presence lease 20 秒，每五秒續租，關閉後移除。
- Session 成功建立/撤銷與登入/登出 Outbox 同交易；事件只有原四欄位，不帶帳號或 cookie。實際跨服務匿名事件通過。
- 相容 GLib 安全回移保留 MIT 授權與來源；不修改全域 Cargo registry，也不以忽略 advisory 代替安全證據。
