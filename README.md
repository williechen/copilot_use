# copilot_use

Rust AI Chat、Tauri/Leptos Desktop Pet、Product Analytics 的獨立產品工作區。驗收與正式版本以 [GitHub Releases](https://github.com/williechen/copilot_use/releases) 附帶的 tag SHA、CI、Docker 與本地報告為準；未有匹配證據的提交不得宣稱完成。 使用者已取消正式部署；目前範圍為本地測試與 CI。`williechen/rust_ai_chat` 嚴格唯讀，完成驗收與固定成品前禁止產生新 Day 教學。

## 實際入口

| 產品 | 程式與資料 | 產品能力 |
|---|---|---|
| Chat | `apps/server`、`crates/ui`、`apps/server/frontend`；可攜式 SQLite（包含知識庫向量） | Leptos SSR/Hydration、登入 Session、私人房間 ACL、訊息/history、WS、交易 Outbox、真實 OpenAI 串流、RAG、唯讀 MCP/Agent |
| Desktop Pet | `apps/desktop_pet`、`apps/desktop_pet/frontend`、`crates/pet_domain`（規則／Port）、`crates/pet_runtime`（SQLite／平台 Adapter）；獨立 SQLite | Linux Tauri/WebKit GUI、多寵物與五項設定、記憶、複習排程、真實 AI 對話/生圖/快取、200×200 浮動視窗、安全預覽與確認 IPC、journal/租約、唯讀 MCP/Agent |
| Analytics | `apps/stats_server`；獨立 PostgreSQL | 同服務 Dashboard + API、來源 token、持久冪等、事件查證、Taipei 日趨勢 API、可逆封存/還原、備份還原、結構洞察與人工審核/套用/回退 |

上表列出實作能力；完整驗收須逐項取得相同來源版本的證據。最新資料庫變更及待驗收項目見 [SQLite 調整](docs/SQLITE_TRANSITION.md)；同來源驗收及重現見 [驗證紀錄](docs/LOCAL_VALIDATION.md)；既定目標見 [P9 規格](docs/P9_TARGET.md)。

## 本地驗證

```sh
python3 scripts/local-verify.py
bash scripts/build-chat-ui.sh
(cd apps/desktop_pet/frontend && NO_COLOR=true trunk build --release=true --no-color)
python3 scripts/pet-gui-verify.py
# 選擇性真實 OpenAI 測試；使用本機環境中的 key，會產生模型費用。
source ~/.bashrc
export OPENAI_CHAT_MODEL=gpt-5.6-luna
python3 scripts/local-verify.py --live-openai
```

需要 Rust、Node 22、PostgreSQL（僅 Analytics）、wasm32 target、Trunk，以及 Linux Tauri 的 GTK/WebKit 開發套件。API key 不進入 repository、測試報告或日誌。實際啟動與設定見 [操作說明](docs/DEPLOYMENT.md)。

## 目錄與歷史

根 Cargo workspace 管理服務、Pet domain 和共享 crates；三個 frontend/native app 的獨立 workspace 各自保留 Cargo.lock。`day9/` 重複範例與重複 workflow 已移除，歷史仍可由 Git 查閱。現有 crate 名稱 `day9-*` 為既有命令相容性而保留，不代表教學進度。

既有 PostgreSQL Chat migrations 保留於 `apps/server/postgres-migrations`，不會套用到 SQLite；SQLite 使用獨立 schema/migration history，舊 PostgreSQL volumes 不會被覆寫或刪除。其他已套用的 migration 不重寫；`docs/legacy-migrations` 保留先前重複 migration 原文供舊測試庫查核，不會在啟動時執行。產生的 dist/target/schema、私人資料庫、.env 不提交。

## 對應 rust_ai_chat 的目錄

正式產品固定於 [v0.1.0](https://github.com/williechen/copilot_use/releases/tag/v0.1.0)。`main` 已整合 `teaching/rust-ai-chat-alignment` 的實際目錄與程式；後續每日教學仍在教學分支推進。程式／目錄／所屬測試對齊見 [TEACHING_ALIGNMENT.md](docs/TEACHING_ALIGNMENT.md)，以獨立 CI 驗證；rust_ai_chat 嚴格唯讀。
