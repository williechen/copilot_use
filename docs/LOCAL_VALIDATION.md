# 本地驗證紀錄（2026-10-11；P9 未完成）

本次使用者已改為「不正式部署，只測試完成」。未進行正式部署，不建立正式完成 Release/Tag，也不產生新 Day 教學。`williechen/rust_ai_chat` 只作唯讀需求參考，未修改或提交。

## 需求與基準

- copilot_use 起點：`26f25f62e5ade7538b19c5643df71f9e06f609d1`。
- 唯讀需求參考：rust_ai_chat `fdc74c1ee244cf86df3fd1c427d893d2956f6211`、Notion 共同主頁及 Chat/Pet/Analytics 三主頁。
- 本地工具：Rust/Cargo 1.99.0、PostgreSQL 18.6、Node 22.22.1、Tauri CLI 2.11.5、Trunk 0.21.14；實際解析依賴固定於 Cargo.lock。此紀錄不宣稱已完成所有最新版本/MSRV/安全稽核。

## 實作與可重現驗證

```sh
python3 scripts/local-verify.py
# 明確指定的 OpenAI 真實測試；需要有效 key，會有一次模型呼叫費用。
source ~/.bashrc
export OPENAI_CHAT_MODEL=gpt-5.6-luna
python3 scripts/local-verify.py --live-openai
```

測試建立全新的 UTF-8 PostgreSQL cluster、Chat/Analytics 各自 non-superuser role 與資料庫，動態選用 loopback port。所有測試資料/背景程序在結束時清除。不使用 rust_ai_chat 的 DB。測試報告寫入 `target/local-validation.json`，不含 token、API key、私人路徑或對話內容。

- 修正缺失 serde_json dev dependency、無法解析的繁體中文 TOML fixture、fmt/Clippy 錯誤。
- 消除兩套重複 CREATE TABLE migrations；保留完整 canonical migration 與 legacy 原文至本文件旁的 legacy-migrations。只對新建隔離 DB 驗證；已使用舊錯誤 schema 的測試庫須另行處理，不直接改寫已套用 canonical migration。
- 實作 room-filtered WebSocket message/presence/typing 通知、第一 frame Token 認證、同 Origin 檢查、frame 限制與 subscriber lag resync。瀏覽器重連後重新載入最近 100 筆 history。這不是完整 send queue、私人房間授權或多實例 WS 系統。
- 只有 DB commit 成功才發 WS message。新增 Outbox 寫入失敗回滾、租約崩潰回收、舊 token 拒絕 mutation、永久失敗 blocked 契約。
- dispatcher 遇到暫時 DB 錯誤仍繼續重試；DB acquire timeout 限制健康檢查等待。
- Analytics query、Chat ingest、Pet ingest 使用三個不同 token，拒絕跨來源冒用。只允許已實作的兩種事件，不將 arbitrary event_type 當成隱私保護。
- 同一訊息 ID 的 Chat → Outbox → Analytics 真實事件查證、重啟與持久冪等、Analytics 中斷投遞重試、PostgreSQL 中斷復原、真實 pg_dump/pg_restore 逐列核對。
- OpenAI Responses HTTPS Adapter、SSE 增量解碼、完整回答才存入 Chat 與 Outbox。未設定 key/model 回 503；上游失敗或不完整串流不保存 partial answer。這不等於完成多 Persona orchestration、Structured Output、tools、RAG/MCP 或 Agent Runtime。

實際 Pass/Fail 以測試 JSON、命令輸出及該 SHA 的 CI 結果為準；新增 workflow 不算 CI 通過。

## 尚未完成／未驗證

| 項目 | 狀態與具體缺口 |
|---|---|
| Chat 完整 UI/身份 | 未實作 Leptos SSR/Hydration、Router/Protected Route、登入/session、使用者/私人房間 ACL；目前仍共享本機 bearer token 與 HTML/JS |
| Chat 完整即時協定 | 未實作可靠 send queue/客戶端 idempotent message replay、無限 history cursor、presence snapshot、跨程序 fan-out、完整 WS 資源上限 |
| Chat AI | 初步 Responses 串流入口；多 AI Persona、memory、speaker selection、turn budget/cooldown、Structured Output/tool calling 尚未完成 |
| Pet GUI | `cargo tauri build --no-bundle` 實際失敗：apps/pet 沒有 tauri.conf/config；Tauri/Leptos GUI 尚未實作，不能把 CLI 稱為桌面成品 |
| Pet AI 與學習 | 生圖/一致性動畫/快取回退、持久 memory、active recall/spaced repetition/difficulty adaptation、真實 Pet AI 尚未實作 |
| Pet 檔案安全 | 現有 quarantine 仍有 metadata→rename TOCTOU，未通過敵對檔案系統安全驗收；無 opaque-ID GUI/receipt durable delivery、非 root 入口保護、Trash/restore workflow |
| Analytics 完整產品 | 未實作時間區間/趨勢/詳細報表、登入/學習/提問等事件、資料留存/刪除、完整存取控制、AI 摘要與可審核建議 |
| RAG/MCP/閉環 | pgvector RAG、MCP server/Adapter、Agent Runtime、三產品授權採用/回退/成效追蹤尚未實作與整合 |
| OTel/安全 | 尚無完整 OTel trace/metrics/log exporter、完整安全稽核/依賴 audit、滲透/負載/隱私驗收 |
| Docker/GUI/跨平台 | 本機未安裝 Docker/Podman，未跑本機 Compose；無 GUI runtime/Windows/macOS installer/updater 驗收。CI 必須查核實際結果 |
| 正式部署 | 使用者取消此項；本次不做正式部署 |
| 完成版本 | P9 未驗收，不建立正式完成 Release/Tag，不開啟 Day 教學 |

本次本地可靠性與初步 AI 串流驗證不能取代三產品 P9 最終驗收。
