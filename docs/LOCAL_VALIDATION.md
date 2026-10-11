# 本地驗證紀錄（2026-10-11）

**P9 未完成。** 使用者取消正式部署；不建立正式完成 Release/Tag，不產生 Day 教學。`rust_ai_chat` 未修改。

## 基準與證據範圍

起點 `26f25f62e5ade7538b19c5643df71f9e06f609d1`；已發布的先前局部修復 `05295d41a2078edd691c2b04ae02f6c825921a1f`。其四個 CI jobs 通過，但不能代表後續工作樹。當前新功能與清理必須以新提交的 CI 再驗證。`d37c341` 與 `c55ab89` 的 Rust/PostgreSQL 和 Docker Compose jobs 已通過；fault-recovery 因 runner 開發套件/LLVM 設定失敗，desktop 因 hook 路徑與 Windows 拒絕測試假設失敗，後续已修正但不得沿用舊 SHA 宣稱新 CI 全通過。

唯讀需求基準 `rust_ai_chat` SHA `fdc74c1ee244cf86df3fd1c427d893d2956f6211`，以及 Notion 共同主頁與三產品需求頁。本地 Rust/Cargo 1.99.0、PostgreSQL 18.6、pgvector 0.8.7、Node 22.22.1、Trunk 0.21.14、Tauri CLI 2.11.5。依賴版本固定於各 workspace 的 Cargo.lock。

## 已實際通過

- `scripts/local-verify.py`：fmt、嚴格 Clippy、33 個 workspace 測試；全新隔離 DB migrations、兩個 non-superuser role 的 DB 隔離、HTTP/WS、精確訊息 ID 的跨服務事件、來源 token 與 schema 拒絕、Session/ACL/CSRF/邀請/登出、重啟冪等、服務與 DB 中斷恢復、pg_dump/restore 逐列核對。
- Chat/Pet OpenAI 真實 `gpt-5.6-luna` 對話串流完成與持久化；Chat 真實 embedding → pgvector 檢索 → RAG 回覆 → Outbox/Analytics，以及知識文件刪除。生圖使用 Responses 的 image_generation tool，實際 PNG 與 SQLite 快取已驗證；不是 placeholder。
- Chat 同 UUID 併發重送只保存一筆訊息與 Outbox；變更 room/author/content 拒絕。真實瀏覽器驗證 32 筆有上限佇列、離線保留、服務恢復後只送達一筆訊息/Outbox/Analytics，及重新訂閱後收到獨立 WS 通知。佇列只在當頁記憶體保存，離開頁面不留私人內容。
- Leptos SSR 與 WASM hydration 編譯、真實瀏覽器登入/受保護路由/建房/送信/刷新 history/登出測試。舊 `/legacy` HTML 入口已移除。
- Linux Tauri/WebKit/WASM/IPC 實際 GUI 測試：五項設定、多寵物、記憶隔離/刪除、複習排程、opaque ID 預覽/拒絕未確認操作/確認後 receipt、200×200 浮動視窗。真實生成 sprite 角色一致性經目視檢查，任意原圖尺寸縮放到固定顯示框。
- Unix 安全 Adapter 的 symlink、根目錄被替換、檔案變更拒絕與正常隔離測試。Linux 實際通過；macOS CI 實際 11 個 Pet 測試通過，但完整 native installer/runtime 尚未驗收。Windows 危險移動 fail closed。
- 本機 numeric loopback collector 實收 OpenTelemetry traces/metrics/logs；Chat Outbox 與 Analytics W3C trace 關聯通過；未洩露憑證或對話內容。只允許本機 collector。
- Analytics 同 ID/同事件冪等、同 ID/不同事件衝突、可逆封存/還原測試通過。永久資料刪除尚未提供。

命令與設定見 [操作說明](DEPLOYMENT.md)。`local-validation-result.json` 是最近一次基礎驗證的局部結果；`live_openai:false` 不代表該次執行有付費呼叫。GUI、生圖、live RAG 各有獨立證據，不能以基礎測試報告取代。

## 尚未完成／未驗證

| 範圍 | 必須補齊 |
|---|---|
| Chat 即時產品 | history cursor、presence snapshot、typing UI、完整連線資源限制、跨程序 fan-out；reload 後不保存未送出私人內容 |
| Chat AI | 多 Persona orchestration、AI-to-AI speaker selection/turn budget/cooldown/context/memory、Structured Output、受控 tool calling |
| 共同 AI | MCP server/Adapter、Agent Runtime、安全工具權限及真正端到端驗收 |
| Pet 完整互動 | 進階覆寫 UI、多動作動畫選擇、主動互動、記憶編輯與學習適應的完整 UI/流程 |
| Pet 檔案安全 | 可操作的完整 Organizer GUI、durable filesystem journal/recovery、Trash/restore、Windows 安全 Adapter、macOS 完整 GUI/打包證據 |
| Pet 投遞與備份 | 跨 instance 租約/CAS、DLQ 管理、故障恢復、SQLite/asset 備份還原與跨產品 E2E |
| Analytics | 完整趨勢 Dashboard、登入等事件目錄、1–3 句有資料依據的 AI 洞察與審核建議、授權採用/回退/成效追蹤閉環 |
| 安全/維運 | 四份 Cargo.lock RustSec 掃描沒有已知漏洞，但 paste/proc-macro-error/glib 有未維護警告；仍需負載/滲透/金鑰輪替、Session 濫用控制、全部故障邊界、UUID/VARCHAR 雙模式驗證 |
| Docker/CI/安裝 | 本機無 Docker/Podman；更新後 Compose/UI/pgvector 配置仍需新 SHA CI。Windows/macOS installer/updater 與跨平台 runtime 未驗收 |
| 最終固定 | 所有必要功能通過後才可正式 release/tag；目前未達門檻 |

## 清理範圍

移除未使用的 `day9/`、重複 `rust-day9.yml`、舊 Chat HTML/JS UI、生圖臨時診斷、產生的 Tauri schema；新增忽略規則防止 .env/SQLite/generated files 誤提交。保留實際產品、各 Cargo.lock、canonical migrations、必要歷史 migration、重現腳本及測試證據。
