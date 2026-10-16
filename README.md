# copilot_use

Rust AI Chat、Tauri/Leptos Desktop Pet、Product Analytics 的獨立產品工作區。驗收與正式版本以 [GitHub Releases](https://github.com/williechen/copilot_use/releases) 附帶的 tag SHA、CI、Docker 與本地報告為準；未有匹配證據的提交不得宣稱完成。 2026-10-16 使用者要求直接使用本機環境，補齊週六同步與原生正式建置；不使用 Docker，外部正式上線尚未執行。`williechen/rust_ai_chat` 嚴格唯讀。原始成品須先驗收並固定正式版本；既有 v0.1.0 固定後，每日依適用 CI Gate 產生教學，後續變更的週六功能驗收另記，不重新阻擋全部每日教學。

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

## 現行文件

[現行三專案 Day 12 文章鏡像](docs/teaching/current/README.md) · [先本地、後 GitHub 的參考庫驗證](docs/source-review/rust-ai-chat-validation.json)（Day12 已套用；本地 a57e532 正式驗證通過，最新 d054389 僅 CI 變更且 6／6 workflows、18／18 checks 通過；GUI／整體驗收另列）。

[目前系統架構與 DB 表格](docs/SYSTEM_ARCHITECTURE.md) · [教學格式與執行方式](docs/TEACHING_FORMAT.md)

- [共同學習規則](docs/COMMON_LEARNING_RULES.md)：目錄／內容以 rust_ai_chat 為主；目標／前進項目以 copilot_use 為主，其他遵循共同規則。
- [未來前進項目](docs/FUTURE_WORK.md)：未完成的學習銜接、三產品切片及週六完整比對。
- [產品目標](docs/P9_TARGET.md)、[目錄與程式銜接](docs/TEACHING_ALIGNMENT.md)、[模組與介面對照](docs/SEMANTIC_ARCHITECTURE_REVIEW.md)、[完整靜態核對與差異](docs/FULL_SOURCE_REVIEW.md)。
- [操作說明](docs/DEPLOYMENT.md)、[可攜式資料](docs/SQLITE_TRANSITION.md)、[驗收方式](docs/LOCAL_VALIDATION.md)。
- docs/learning-state.json 保存當前來源、有效程式驗證、下一教學編號與週六到期日。

正式 [v0.1.0](https://github.com/williechen/copilot_use/releases/tag/v0.1.0) 保持固定；每日教學在 teaching/rust-ai-chat-alignment，main 整合匹配驗收的變更。根 workspace 管理服務／共享 crates；frontend/native 保留獨立 lock。測試放所屬 app／crate／模組，沒有根 tests workspace。私人 .env／DB 與生成檔不提交。

主文件保留有效規則與未來工作；封存及 Git 歷史不作現行待辦。已套用 migrations、固定 release 證據及操作手冊保留。

## Analytics 通訊更新

Chat／Pet SQLite Outbox 改以 gRPC／Protobuf 上報；Dashboard 保留 HTTP 查詢並以 SSE 即時更新。使用既有 Analytics port／來源 token，PostgreSQL 提交後才 ACK。這是正式 v0.1.0 固定後的新需求，不變更既有 tag 或安裝包。契約、測試與運作限制見 [ANALYTICS_TRANSPORT.md](docs/ANALYTICS_TRANSPORT.md)。

## 週六系統與部署建置準備

2026-10-16 重新要求的週六同步與正式部署建置入口見 [週六操作](docs/SATURDAY_OPERATIONS.md)。尚未上線；既有固定 release 的驗收範圍不變。
