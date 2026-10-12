# 本地啟動與測試

使用者要求只測試、不正式部署。此文件描述隔離測試環境，不能作為正式部署已驗收的證據。`rust_ai_chat` 的程式與 DB 都不可修改。

## 自動隔離驗證

```sh
python3 scripts/local-verify.py
source ~/.bashrc
export OPENAI_CHAT_MODEL=gpt-5.6-luna
python3 scripts/local-verify.py --live-openai
```

需要本機 PostgreSQL 的 initdb/pg_ctl/psql/pg_dump/pg_restore、Rust 與 Node。腳本建立 UTF-8 暫存 cluster、SQLite Chat 檔案與 Analytics 獨立 app role/DB、random loopback ports，退出時清除 disposable 資料與程序。基礎測試移除 key 以避免外部呼叫；`--live-openai` 明確啟用真正 API 測試。報告位於 `target/local-validation.json`。

## Chat 與 Analytics

Chat 使用 `CHAT_DATABASE_URL=sqlite://data/chat.sqlite3`（未指定時為此預設）。只替 Analytics 建立 PostgreSQL DB 與 non-superuser app role。Chat/Pet 不需要 PostgreSQL 或向量 extension。

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

`PET_AUTHORIZED_ROOT` 必須明確指定使用者授權的檔案目錄，且以非 root 啟動。Linux Adapter 拒絕 symlink、非本人檔案、group/other 可寫檔案與多 hard links；預覽返回 opaque ID，確認後才移動至同根目錄的 private quarantine。GUI 可授權/撤銷目錄、逐檔確認，並以 journal 復原收據與事件。目錄必須是本人所有且無 group/other 寫入權限；quarantine 與 journal 是 private。Trash/restore 與 Windows 安全 Adapter 已實作；平台驗收結果以各 SHA 的實際安全測試及安裝包 GUI 報告為準。

`ANALYTICS_ENDPOINT` 加 `ANALYTICS_PET_TOKEN` 才啟用 Pet 投遞。原始記憶、對話、檔案路徑不放入產品事件。

## 可選本地觀測

`COPILOT_LOCAL_OTLP_ENDPOINT=http://127.0.0.1:4318` 只允許 numeric loopback。三種信號使用 `/v1/traces`、`/v1/metrics`、`/v1/logs`。HTTP transport 固定白名單 URL，停用 proxy/redirect，移除 SDK 合併的全部自訂 headers。不接受遠端 exporter、credential headers 或 query string；不輸出原始對話、密鑰或路徑。

## Docker Compose

複製 `.env.example` 為 `.env`，替換全部 placeholder，以獨立隨機 bootstrap/app DB secrets 與四種 Analytics token 啟動全新 disposable volumes：

```sh
docker compose --env-file .env up --build -d
bash scripts/smoke.sh
```

Chat image 包含編譯後的 WASM/CSS；Chat 在 private `chat-sqlite-data` volume 的 `/data/chat.sqlite3` 使用 SQLite。Analytics PostgreSQL 初始化 script 建立 non-superuser app role。`.env` 的 AI key 不進 image build；只在明確設定後於 runtime 使用。

此初始化只適用新空白 volume，不會自動降權或重建既有 DB。本機 Docker 已啟用；SQLite 配置需要重新建置及驗證，不能沿用 PostgreSQL Chat 的既有映像或測試結果。

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

Chat history 可傳 `before=<message UUID>`，最多 100 筆；UI 最多保留 1000 筆。WebSocket 單程序最多 128 連線，登出/Session 到期/ACL 失效會拒絕後續資料並關閉連線；idle 每五秒檢查。同一主機的多個 Chat 程序共用 SQLite 檔案與持久通知佇列；全域最多512、每位使用者32、單程序128連線，idle 每五秒重查Session/ACL。SQLite WAL 不支援跨主機網路檔案系統。

## 本地備份、角色對話與更新

Pet 的「本地備份與還原」會把生成素材一併存入 SQLite 快照，還原前另外保留目前快照。恢復快照中的寵物資料，保留後來新增的寵物、投遞事件與檔案操作收據；不移動授權目錄中的檔案。每個快照需符合 SHA256、SQLite integrity；受支援的舊 schema 在私有副本升級，原備份不修改。

Chat 房主可保存最多八個 Persona，明確確認後開始有界對話。每次輸出最多512 tokens；一般輪次保留512，工具輪次保留1024（最多兩次請求、一個唯讀檢索工具），最多八輪及180秒；失敗嘗試仍消耗保留預算。完成輪次與記憶、訊息、Outbox 同交易；取消保留已完成輪次。

本地 updater 使用 `PET_UPDATE_ENDPOINT`（numeric loopback HTTP/HTTPS）與 `PET_UPDATE_PUBLIC_KEY`（可信公鑰）。下載網址必須與 channel 同 origin，停用 proxy/redirect，最多256 MiB。簽章必須包含與公告相同的版本；用 `cargo tauri signer sign --app-version VERSION`。GUI 先檢查、下載驗證，再明確確認安裝；先建立資料快照。Linux 自動更新需 AppImage 安裝格式；deb 使用套件管理器更新。測試公鑰/私鑰不等於正式發行簽章，不提交私鑰。

`python3 scripts/pet-gui-verify.py --updater` 使用一次性私鑰與本機 fixture，實測簽章、版本不符、篡改與 URL 邊界，不以此宣稱已驗證三平台更新安裝。Docker 建置以 `COPILOT_BUILD_SHA=$(git rev-parse HEAD)` 設定映像來源 label，驗證報告記錄 image ID 與來源工作樹狀態。

## 驗收來源與教學分支

`pet-gui-verify.py --executable PATH` 實際執行安裝包的 release 程式。入口需要 `--acceptance-gui`、明確測試設定、全新 state/report 與已知可拋棄檔案 fixture；報告包含 source SHA、dirty、版本及 release 狀態。一般啟動不執行驗收脚本。三平台更新測試只更換 tempfile 應用副本，驗證有效簽章／checksum／冷啟動版本／SQLite 完整性／資料及備份保留。`build-test-update.py` 依平台產生 AppImage／macOS app archive／Windows NSIS 的 0.1.1 測試套件並還原 manifest/lock，該測試套件不是正式發行版。

`local-verify.py --varchar-events` 在新測試庫將 Analytics 事件 ID 欄位改為 VARCHAR(40)，以同一個 production EventStore、UUID domain/API 跑完整可靠性測試；預設 production migrations 仍使用原生 UUID。不是全資料庫任意 schema 的切換開關。

正式 tag 固定後才建立 `copilot_use` 的教學對齊分支。測試隨各自模組／專案放置，不集中至最外層 `tests/`。每日教學必須列出該分支真實路徑、程式變更、驗證命令與結果；`rust_ai_chat` 保持唯讀。

寵物實際設定位於 app_data_dir 的 `pets/<uuid>/pet.toml`（GUI 顯示精確位置）。五項必填設定使用中文鍵；選用覆寫分於中文 quoted tables。編輯後可啟動 GUI 明確確認匯入；啟動不覆蓋同一已保存版本的手動草稿。SQLite 備份包含已保存／匯入的設定與生成PNG；未匯入的檔案草稿不屬於已保存設定。封存區提供可還原的寵物移除，資料留在本機。

Chat 房間中的技術詞統計預設關閉，可在「匿名技術詞彙統計」選擇同意。只對本人的後續訊息用固定詞表計數，停用會排程封存自己的既有詞彙事件；可查看實際 ACK 進度。Analytics 的 `/api/v1/events/recent` 與 Dashboard 明細使用 read token，最多100筆，支持來源／事件類型過濾；時間為實際接收時間，四欄 v1 事件契約維持，不傳送原文／帳號／檔案路徑。

## Chat／Pet 可攜式 SQLite

Chat 的訊息、Session、房間權限、角色、知識文件／1536維向量、Outbox、分析同意及採用收據都在同一 SQLite。每位文件 owner 最多1000份文件；以真正 embedding 做 cosine distance 精確排序，不需要平台 extension。寫入使用 WAL、FULL synchronous、foreign keys、busy timeout；涉及多步驟狀態更新採 BEGIN IMMEDIATE，通知只有交易提交後才可被其他程序讀取。

```sh
export CHAT_DATABASE_URL=sqlite://data/chat.sqlite3
# 一致性線上備份，目的檔必須不存在；不需要停止服務。
cargo run --locked -p day9-chat -- --backup backups/chat.sqlite3
# 以備份檔搬到另一個本地目錄後啟動。
export CHAT_DATABASE_URL=sqlite://relocated/chat.sqlite3
cargo run --locked -p day9-chat
```

先建立 backups 目的目錄。Chat DB 的所在目錄必須為本人所有且不可讓其他使用者寫入；Linux 建議 `mkdir -m 700 data backups relocated`。不要單獨複製正在使用的 chat.sqlite3；未 checkpoint 的資料可能仍在 -wal。`--backup` 使用 VACUUM INTO，檢查 integrity/foreign keys，且拒絕覆寫既有檔案。停止全部 Chat 程序後亦可搬移整個資料目錄。備份含密碼雜湊與 Session，須保持私密、由可信使用者保管；服務 token/API key 仍由環境設定，不寫入 DB。

Pet 保留既有 SQLite 資料及 GUI 一致性備份／还原；完整可攜資料包含 pet.sqlite 與 pets/ 下的設定／生成素材。備份還原後由 SQLite 重建設定／素材 projection；手動未匯入的 pet.toml 草稿另行保存。Analytics 的 PostgreSQL、pg_dump/pg_restore、API 及 Dashboard 不變。

舊 PostgreSQL Chat 資料不會自動轉成 SQLite。舊 migration 與 Docker volume 保留供讀取／遷移；不可把舊 DB URL 指向新 SQLite 服務，也不可刪除舊 volume 後宣稱已遷移。

需要搬移舊 Chat PostgreSQL 資料時，先停止舊 Chat 寫入，保留原 DB／備份，在本機環境設定 `CHAT_LEGACY_DATABASE_URL`，執行 `python3 scripts/import-chat-postgres.py --output data/imported-chat.sqlite3`。工具以唯讀 repeatable-read snapshot 串流匯出所有已知表，转换 UUID／時間／JSON／向量，檢查 foreign keys/integrity，且只建立新檔，不覆寫原 DB 或既有 SQLite。未知表／欄位會拒絕，不能默默丟棄資料。匯入成功後另指定 `CHAT_DATABASE_URL` 啟動新服務；原資料保留到核對完成。

跨平台 updater 安裝必須用實際套件與 `--update-package`；macOS／Windows 另提供 `--expected-executable` 核對安裝後 bytes。Windows SDK 會退出原程序並啟动 NSIS，因此先保存「待驗證」狀態，外部驗收在新 binary checksum／冷啟動版本及 SQLite／備份都通過後才標 success。更新目的目錄固定為目前執行檔所在目錄，manifest 不可指定任意目錄。
