# 本地啟動

本頁按 copilot_use/main 的實際啟動腳本核對；核對來源及範圍見 [文件核對紀錄](teaching/history/evidence/2026-10-22-current-docs/README.md)。以下在 Linux 桌面、一般使用者及儲存庫根目錄操作。

## 首次建置 Chat 與 Analytics

需要 Python 3、Rust/Cargo/rustup、Trunk、`wasm32-unknown-unknown` target，以及含 `initdb`、`pg_ctl`、`psql` 的 PostgreSQL 工具；腳本以 `pg_config --bindir` 尋找 PostgreSQL。先確認工具已安裝。原生編譯所需系統函式庫依平台配置，`native-env.sh` 可載入已有的專案工具。

```sh
source scripts/native-env.sh
bash scripts/build-production.sh
python3 scripts/local-services.py start
python3 scripts/local-services.py status
```

`build-production.sh` 要求已提交且乾淨的 Git 工作目錄，先處理自己的未提交工作；它執行 fmt 檢查、Chat 前端 release 及兩個服務 release 建置。另產出 `target/native-release/<來源 SHA>/` 與 checksum manifest。建置不啟動服務，也不包含 Pet。

`local-services.py` 使用 `target/release/day9-chat`、`target/release/day9-analytics` 與 `target/site/pkg`，不會自動重建。服務管理器建立專用 PostgreSQL cluster、SQLite 與隨機憑證，並先啟動 Analytics，再啟動 Chat。

| 入口 | 使用方式 |
| --- | --- |
| Chat `http://127.0.0.1:3001/` | 公開房間入口；`/login` 提供註冊／登入，登入後使用個人管理頁 |
| Analytics `http://127.0.0.1:3100/` | 目前產品 Dashboard 仍使用 query Bearer token |
| Analytics `http://127.0.0.1:3100/admin` | 目前產品管理頁使用獨立 admin token |

Analytics token 由本人在本機安全開啟 `.local/runtime/settings.json` 取得：查詢用 `query_token`、管理用 `admin_token`。不要貼出或提交此檔。帳密／Session 後台是 [產品需求](requirements/AUTHENTICATION.md)，不能把學習庫的帳密登入當作此產品入口已完成。

## 日常啟停與重新建置

```sh
python3 scripts/local-services.py status
python3 scripts/local-services.py start
# 要結束服務時
python3 scripts/local-services.py stop
```

已在執行時先看 status，重複 start 會因連接埠已占用失敗。更新程式後，停止服務、完成上述建置，再 start。status 檢查兩個 `/healthz`，不代表註冊、房間權限、事件投遞或完整產品驗收通過。stop 管理本腳本啟動的 Chat、Analytics 與私人 PostgreSQL，不管理 Pet。

| 本機資源 | 位置／連接埠 |
| --- | --- |
| 私有設定 | `.local/runtime/settings.json`，0600 |
| Chat SQLite | `.local/runtime/chat.sqlite3` |
| Analytics PostgreSQL | `.local/runtime/postgres`，127.0.0.1:55432 |
| 日誌 | `.local/runtime/chat.log`、`analytics.log`、`postgres.log` |
| Chat／Analytics | 僅 loopback 3001／3100 |

保留 runtime 目錄才能沿用既有資料及憑證。此入口清除服務繼承的 `OPENAI_API_KEY`，本機 HTTP 使用非 Secure cookie；它不啟用付費 AI 功能。

## Desktop Pet

Pet 是獨立桌面程式，需要 Node、Trunk、Cargo Tauri CLI、WASM target，以及系統 GTK／WebKit 等原生依賴。先建置前端，再建置原生程式：

```sh
source scripts/native-env.sh
node scripts/build-pet-ui.mjs
cargo build --release --locked --manifest-path apps/desktop_pet/Cargo.toml
apps/desktop_pet/target/release/copilot-desktop-pet
```

上例直接執行本次建置，適合開發。若要安裝至個人應用程式入口，結束開發視窗後執行：

```sh
python3 scripts/install-pet-local.py
~/.local/bin/copilot-desktop-pet
```

安裝器從 `apps/desktop_pet/tauri.conf.json` 讀版本，安裝到個人 XDG 資料目錄的 `copilot-desktop-pet/releases/v<版本>/`，再建立 launcher 與桌面入口；它不自動啟動 GUI。同版目錄已有內容時會拒絕覆寫。已安裝版可直接執行 launcher；要測修改後的同版程式則使用前述開發執行檔。

首次安裝若已有 `.local/runtime/settings.json`，安裝器可建立個人的 `copilot-desktop-pet/runtime.env`，銜接 Analytics Pet 上報；因此要使用這條整合流程，先啟動 Chat／Analytics，再安裝 Pet。既有 runtime.env 不會被自動覆寫。直接執行開發 binary 不會自動載入安裝 launcher 的 runtime.env。Linux 安裝器亦檢查系統 WebKitNetworkProcess，不能僅有 `.so` 檔案。

## 問題排查與驗收

| 現象 | 檢查 |
| --- | --- |
| 缺少 release binary 或 WASM | 完成 `build-production.sh`，確認從儲存庫根目錄操作 |
| 3001／3100／55432 已占用 | 先查本管理器 status；辨識占用程序，不任意終止其他服務 |
| PostgreSQL 工具不存在 | 核對 `pg_config --bindir` 與所需三個工具 |
| 建置拒絕 dirty tree／fmt | 處理未提交工作與格式，再建置 |
| Pet 無法顯示 | 核對目前桌面 session、系統 GTK／WebKit 與執行日誌 |
| 同版 Pet 安裝被拒絕 | 執行已安裝 launcher，或使用本次開發 binary |

一般啟動與隔離測試分開。測試入口見 [本地驗收](LOCAL_VALIDATION.md)，資料備份見 [SQLite 操作](SQLITE_TRANSITION.md)，更新機制見 [Pet 更新](GITHUB_PET_UPDATES.md)，週六工程見 [週六操作](SATURDAY_OPERATIONS.md)。本次文件核對只執行 CLI／語法檢查，沒有重新建置、啟動服務或執行 GUI 驗收。
