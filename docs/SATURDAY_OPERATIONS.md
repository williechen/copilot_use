# 週六比對與原生建置操作

此環境使用 `/home/willie/Documents/rust_ai_chat` 作唯讀參考，
`/home/willie/Documents/copilot_use` 的 `main` 作目前整合工作區。
本頁保存週六比對、原生建置及安裝操作；目前交付範圍為本地測試與 CI。主機安裝章節是操作參考，不代表已獲得外部部署授權或已完成上線。
既有 v0.1.0、release 附件與歷史驗收保持固定。

## 週六執行

`weekly-sync.yml` 在 main 安裝後，每週六台北時間 00:00 執行，也可手動觸發。
GitHub 排程可能延遲；未推送與整合 main 前不會啟用。
完整 tracked tree 比對保存兩庫 SHA、檔案 Git object ID、mode、新增／刪除／內容差異及 dirty 狀態。
搬移候選可用相同 object ID 查核；差異本身不代表必要整合。

```sh
python3 scripts/weekly-compare.py --reference /home/willie/Documents/rust_ai_chat --output target/weekly-comparison.json
```

接著逐模組核對介面、契約、DB、流程與 UI，把差異分類為合理差異、必要銜接、架構違反、產品缺口與未核實。
週六 workflow 呼叫既有 Verify（Rust／DB／故障復原／三平台 GUI；週六明確略過 Compose job）。
不傳入 AI key、不新增付費呼叫；不自動覆寫 source、merge 或推進 learning-state。
完整語意核對、必要修正、精確 SHA 的功能 Gate 與提交完成後，才能更新完整比對日期與驗收。
此工具只提供比對證據，不能代替人工或代理的語意核對。

## 原生正式建置（不使用 Docker）

需要 Rust stable、wasm32-unknown-unknown target 與 Trunk 0.21.14。
提交並完成該 SHA 的適用 Verify 後執行：

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --version 0.21.14 --locked
bash scripts/build-production.sh
```

產出 `target/native-release/<SHA>/`，含 Chat／Analytics release 執行檔、WASM／CSS、systemd unit 與 SHA256 manifest。
Desktop Pet 沿用原生三平台建置，不作背景 systemd 服務。
原有 Docker 檔案保留為歷史可選工具；本次操作與週六 workflow 不使用 Docker。

## 主機安裝與啟動

以下需主機系統管理權限，尚未自動執行：

1. 在本機 PostgreSQL 建立 analytics_db 與獨立 non-superuser analytics_user，只授權該 DB；先為既有資料備份。
2. 把產物複製到 `/opt/copilot/releases/<SHA>`，使 `/opt/copilot/current` 指向該版本；程式及資產需服務可讀。
3. 將 deploy 的兩份 env.example 複製到 `/etc/copilot/chat.env` 及 `analytics.env`，由 root 擁有、模式 0600。
   填入獨立隨機 token、DB 密碼，兩服務的 ANALYTICS_CHAT_TOKEN 必須相同。AI key 可不設定。
4. 將兩份 unit 安裝到 `/etc/systemd/system/`，執行 `systemctl daemon-reload`，先啟動 copilot-analytics，再啟動 copilot-chat；驗證成功後才 enable。
   DynamicUser 與 StateDirectory 管理 SQLite 專用可寫目錄；不使用開發者家目錄或 reference DB。
5. 主機 TLS proxy 轉發 Chat loopback 3001（支援 WebSocket）與 Analytics loopback 3100（支援 HTTP/2 gRPC、SSE 停用 buffering）。
   Secure cookie 必須從 HTTPS 入口驗證登入；沒有 TLS 時不能宣稱正式登入驗收通過。
6. 檢查兩個 `/healthz`、HTTPS 登入／權限／WS／SSE／gRPC、重新啟動與備份還原，保存來源 SHA 與結果。

尚需部署網域、TLS、監控告警、排程備份與實際還原演練。
回滾先停服務、備份 DB、核對 schema 向後相容，再切回舊 release 並啟動；不可盲目降版 DB。
目前只新增原生建置與服務設定，不代表已完成外部上線或功能驗收。

## 直接使用本機環境

在專案目錄完成原生建置後：

```sh
python3 scripts/local-services.py start
python3 scripts/local-services.py status
python3 scripts/local-services.py stop
```

不需 root，不裝容器；使用既有 PostgreSQL 工具，在 `.local/runtime` 建立私人 SCRAM cluster（55432）、non-superuser app role、SQLite 與隨機 token。
Chat 3001／Analytics 3100 只監聽 loopback；本機 HTTP 不啟用 Secure cookie，不代表公網正式部署。
私密設定與服務日誌只留本機，不會輸出 token；Analytics dashboard 的查詢 token 由本人從 settings.json 安全取得。
啟動不呼叫 AI；此管理入口清除繼承的 OPENAI_API_KEY，避免週六功能檢查誤用付費 API。

## 本機週六排程

`deploy/systemd/copilot-weekly.service` 與 `.timer` 是 user unit；在同一台電腦直接執行 `scripts/run-weekly.sh`，不使用 GitHub runner 或 Docker。
安裝到 `~/.config/systemd/user/` 後 `systemctl --user daemon-reload`、`systemctl --user enable --now copilot-weekly.timer`。
台北時間週六 00:00 執行；未登入時能否常駐取決於使用者服務／linger 設定。Persistent 會在恢復時補跑錯過的排程。
此流程不 fetch／覆寫工作目錄、不自動 merge、不推進驗收日期；執行前應確認兩庫來源已更新。
結果在 `target/weekly/<UTC timestamp>/`，失敗可查 `journalctl --user -u copilot-weekly.service`。

## 此機器的原生工具

Ubuntu 26.04 缺少 GCC／CMake／pkg-config／OpenSSL headers 且 sudo 需密碼。
`scripts/setup-native-tools.sh` 可從已設定的 Ubuntu APT 來源下載並解包到 `target/native-tools/`，不修改系統套件庫。
`native-env.sh` 為建置與週六測試載入私有編譯器／函式庫並限制 Cargo jobs=2。
Trunk 0.21.14 已由官方 GitHub release 下載、核對 SHA256，安裝到使用者 Cargo bin；WASM target 已安裝。
Node 使用 Codex 現有 bundled runtime 作 fallback，沒有 Docker 依賴。

## v0.2.0 桌面執行與安裝

正常桌面執行使用系統 runtime：

```sh
sudo apt-get install -y libgtk-3-0t64 libwebkit2gtk-4.1-0 libayatana-appindicator3-1 libxdo3
```

本機 Ubuntu 26.04 已由使用者完成安裝。WebKitNetworkProcess 須存在於 `/usr/lib/x86_64-linux-gnu/webkit2gtk-4.1/`。
不要只放 `.so` 而缺少 WebKit 子程序；這會在視窗初始化時退出。

```sh
source scripts/native-env.sh
node scripts/build-pet-ui.mjs
cargo build --release --locked --manifest-path apps/desktop_pet/Cargo.toml
python3 scripts/install-pet-local.py
~/.local/bin/copilot-desktop-pet
```

安裝至本人的 XDG applications 與 `~/.local/share/copilot-desktop-pet/releases/v0.2.0/`，保留舊版本與既有資料。
安裝器只執行 `--version` 核對版本，不自行啟動 GUI 或加入開機自啟。
後續 GUI 驗證直接使用目前桌面及系統 WebKit，不需要 Xvfb、容器或 WebKit 路徑轉接。
週六 `--functional-only` 排除 MCP/Provider 驗證；授權修正使用本地文字 fixture。
