# 本地啟動與測試

使用者要求只測試、不正式部署。此文件描述隔離測試環境，不能作為正式部署已驗收的證據。`rust_ai_chat` 的程式與 DB 都不可修改。

## 自動隔離驗證

```sh
python3 scripts/local-verify.py
source ~/.bashrc
export OPENAI_CHAT_MODEL=gpt-5.6-luna
python3 scripts/local-verify.py --live-openai
```

需要本機 PostgreSQL 的 initdb/pg_ctl/psql/pg_dump/pg_restore、pgvector 已安裝、Rust 與 Node。腳本建立 UTF-8 暫存 cluster、獨立 app roles/DB、random loopback ports，退出時清除 disposable 資料與程序。基礎測試移除 key 以避免外部呼叫；`--live-openai` 明確啟用真正 API 測試。報告位於 `target/local-validation.json`。

## Chat 與 Analytics

先建立獨立空白測試 DB 及 non-superuser app roles；管理者在 Chat DB 啟用 `CREATE EXTENSION vector`。兩個 app 只對自己的 DB 執行 migrations。

設定 `CHAT_DATABASE_URL`、`ANALYTICS_DATABASE_URL`、`CHAT_API_TOKEN`，以及四個彼此不同、至少 16 字元的 Analytics token：

| 設定 | 權限 |
|---|---|
| ANALYTICS_API_TOKEN | 查詢 |
| ANALYTICS_CHAT_TOKEN | web_chat 投遞、只讀取自己的建議與回報收據 |
| ANALYTICS_PET_TOKEN | desktop_pet 投遞、只讀取自己的建議與回報收據 |
| ANALYTICS_ADMIN_TOKEN | 可逆封存/還原、產生/審核建議、記錄描述性比較 |

Chat 的 `ANALYTICS_ENDPOINT` 使用 `http://127.0.0.1:3100/api/v1/events`。`CHAT_LISTEN` 預設 127.0.0.1:3001；Analytics 預設 127.0.0.1:3100。

```sh
bash scripts/build-chat-ui.sh
cargo run --locked -p day9-analytics
# 另一終端使用相同設定
cargo run --locked -p day9-chat
```

Chat 開啟 `http://127.0.0.1:3001/login` 註冊/登入。密碼至少 12 字元；HttpOnly/SameSite cookie 與 CSRF header，房主邀請後才能讀寫私有房間。管理 API token 是獨立服務權限，不要傳給一般用戶。Analytics Dashboard 在 3100 根路徑，query token 只留當頁記憶體。

AI 設定 `OPENAI_API_KEY`、`OPENAI_CHAT_MODEL=gpt-5.6-luna`；embedding 預設 `text-embedding-3-small`、1536 dimensions。未設定時回 503，不使用 mock。知識庫與 Pet AI 會依 UI 說明傳送選取資料到 OpenAI。

## Linux Desktop Pet

```sh
node scripts/build-pet-ui.mjs
cargo run --locked --manifest-path apps/pet_desktop/Cargo.toml
python3 scripts/pet-gui-verify.py
# 同時驗證真正桌面投遞與三服務 trace：
python3 scripts/local-verify.py --pet-gui
```

需 wasm32-unknown-unknown target、Trunk、Tauri CLI、Node 22、GTK3/WebKit 與 desktop display。建置 helper 只從單一來源 icon 產生必要 ICO/ICNS，產生檔不入 Git。正式 UI 有設定視窗及 200×200 浮動寵物視窗。資料存於 app_data；`PET_DATA_DIR` 可指定獨立測試目錄，請勿與 reference repo 共用。

`PET_AUTHORIZED_ROOT` 必須明確指定使用者授權的檔案目錄，且以非 root 啟動。Linux Adapter 拒絕 symlink、非本人檔案、group/other 可寫檔案與多 hard links；預覽返回 opaque ID，確認後才移動至同根目錄的 private quarantine。GUI 可授權/撤銷目錄、逐檔確認，並以 journal 復原收據與事件。目錄必須是本人所有且無 group/other 寫入權限；quarantine 與 journal 是 private。Trash/restore、Windows 安全移動與完整跨平台 runtime 驗收仍缺。

`ANALYTICS_ENDPOINT` 加 `ANALYTICS_PET_TOKEN` 才啟用 Pet 投遞。原始記憶、對話、檔案路徑不放入產品事件。

## 可選本地觀測

`COPILOT_LOCAL_OTLP_ENDPOINT=http://127.0.0.1:4318` 只允許 numeric loopback。三種信號使用 `/v1/traces`、`/v1/metrics`、`/v1/logs`。HTTP transport 固定白名單 URL，停用 proxy/redirect，移除 SDK 合併的全部自訂 headers。不接受遠端 exporter、credential headers 或 query string；不輸出原始對話、密鑰或路徑。

## Docker Compose

複製 `.env.example` 為 `.env`，替換全部 placeholder，以獨立隨機 bootstrap/app DB secrets 與四種 Analytics token 啟動全新 disposable volumes：

```sh
docker compose --env-file .env up --build -d
bash scripts/smoke.sh
```

Chat image 包含編譯後的 WASM/CSS；Chat DB 使用 [pgvector 官方 image](https://github.com/pgvector/pgvector#docker)，初始化 script 建立 non-superuser app role 與 extension。`.env` 的 AI key 不進 image build；只在明確設定後於 runtime 使用。

此初始化只適用新空白 volume，不會自動降權或重建既有 DB。本機沒有 Docker，更新後的配置必須由新提交的 CI 驗證，不能沿用先前 SHA 的成功結果。

## MCP 與 Agent

Chat 的 `/api/v1/mcp` 使用 2026-07-28 stateless MCP，需 Session+CSRF 或管理 Bearer 認證；headers 必須與 method/name/protocol metadata 相符。只提供 `knowledge_search`，結果仍受文件 owner 限制。不提供任意命令、檔案寫入或外部工具目的地。

聊天室的 Agent 有明確唯讀工具同意、工具活動、引用與理解問題。`/api/v1/ai/agent` 至多四次工具呼叫，只有完整且引用合法的回答才存入 DB/Outbox。真正測試使用 `--live-openai`；不能把未設定 key 的基礎測試算成真實模型驗證。

## 洞察與人工採用

Analytics 服務需自己的 `OPENAI_API_KEY` 與 `OPENAI_CHAT_MODEL` 才可生成洞察。Dashboard 輸入查詢/管理 token，查看聚合快照與建議，核准後把 UUID 帶到 Chat 房間或 Pet 的「已審核的設定建議」。必須勾選確認；只允許 Chat 歷史 context 10–100 筆、Pet 成功回想的最短間隔 1–30 天。沒有自動採用或任意模型設定寫入。

收據 UUID 與原設定持久化；回退檢查設定 revision，拒絕覆蓋後續變更。`analytics_acknowledged:false` 表示本地成功、遠端未確認，可重試同一建議回報，不應建立新建議。評估記錄當時與目前的七日事件數；重疊視窗、曝光變動與缺乏品質指標不能證明因果效果。

```sh
# 只測試真實聚合洞察，使用全新 disposable DB 的人工事件計數：
python3 scripts/local-verify.py --live-insights --pet-gui
```

Pet 的記憶工具使用同一 executable 的 `--mcp-once` readonly SQLite stdio 模式；只有所選寵物的資料可見，子程序不繼承 API key。MCP 支援範圍為上述本地唯讀工具，不代表任意第三方 MCP server 相容。

Chat history 可傳 `before=<message UUID>`，最多 100 筆；UI 最多保留 1000 筆。WebSocket 全服務最多 128 連線，登出/Session 到期/ACL 失效會拒絕後續資料並關閉連線；idle 每五秒檢查。跨程序 fan-out 和更完整配額仍是未完成事項。
