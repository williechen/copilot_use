# P9 本地與跨平台驗收

交付範圍為三產品的本地測試、Docker release 建置、跨平台安裝／更新與版本固定；使用者已取消公開部署。正式來源 SHA、六項 CI 與安裝包 checksum 以 [GitHub Release](https://github.com/williechen/copilot_use/releases) 的 `release-manifest.json` 為準。每份報告必須指向同一個乾淨來源；舊提交的成功不替代新提交驗收。

## 產品與證據

| 產品／整合 | 已執行驗收 | 證據 |
|---|---|---|
| Chat SQLite | 空庫 migration、Session／ACL、多房間、history、WS presence／typing／重連、跨程序可靠傳送、交易 Outbox／冪等 | `live.json`、CI |
| Chat AI | 真實 OpenAI gpt-5.6-luna Responses 串流、結構化 Persona／AI-to-AI、SQLite 向量 RAG、唯讀 MCP／Agent 與引用 | `live.json` |
| Chat 可攜資料 | 舊 PostgreSQL 唯讀匯入 codec／拒絕覆寫、VACUUM INTO 完整快照、所有資料表比對、搬移後 HTTP history | `live.json` |
| Pet SQLite／檔案 | 多寵物、繁體中文五欄設定、每寵物 TOML／identity manifest／素材、記憶／到期複習／適應／主動提醒、可逆封存、備份與還原 | `packaged_gui.json`、三平台 native／GUI CI |
| Pet AI／動畫 | 真實 AI 對話、RAG／MCP／Agent、生圖與角色 hash 快取、4×4素材、非正方形等比 200×200、浮動視窗更新 | `packaged_gui.json` |
| FileOrganizer | 授權 root／opaque ID、預覽與明確確認、symlink／TOCTOU／ACL／lease／journal、私人 Trash／還原、重送與重啟事件去重 | native CI、GUI、`live.json` |
| Analytics PostgreSQL | 獨立 DB／non-superuser role、來源認證、冪等／衝突、UUID 及 VARCHAR(40)、Taipei 趨勢、明細、可逆封存／還原、pg_dump／restore | `live.json`、`docker.json`、CI |
| 分析閉環 | 真實 AI 使用持久 aggregate 快照產生可追溯洞察，人工審核、Chat／Pet 明確授權套用、收據、描述性前後比較及回退 | `live.json`、GUI |
| 隱私與觀測 | 預設關閉技術詞計數、逐人逐房間同意／撤回、可靠撤銷、不復活詞頻；OTLP 收到 traces／metrics／logs，不含憑證或原始訊息 | `live.json` |
| 負載／故障 | 512 sockets／五程序的 admission 邊界、64併發訊息至32訂閱者、精確 Analytics ID；服務重啟、PG 中斷、Outbox 恢復、金鑰輪替 | `live.json`、fault／Compose CI |
| Docker release | 真實 image revision、非 root、唯讀 rootfs、drop ALL、no-new-privileges、資源邊界、WASM、容器重建、Chat SQLite 在 PG 中斷時可用、備份還原 | `docker.json` |
| 安裝與更新 | Linux deb／AppImage、macOS app、Windows NSIS 實際 GUI；簽章拒絕邊界、真正0.1.0→0.1.1更新、預期bytes、冷啟動版本、SQLite與快照checksum保留 | 三平台 CI artifacts、`appimage_update.json` |

正式 Release 的附件保留上述四份本地報告及三平台 CI 安裝包 ZIP。CI 成功只代表該 SHA 的實際執行結果；不是商業平台憑證簽章或公開部署證據。

## 重現與安全邊界

使用 [操作說明](DEPLOYMENT.md) 與 `scripts/local-verify.py`、`scripts/docker-local-verify.py`、`scripts/pet-gui-verify.py` 重現。真實 AI 測試使用本機 `OPENAI_API_KEY` 與 `OPENAI_CHAT_MODEL=gpt-5.6-luna`；報告和 Git 不保存 key。MCP 用隔離本地唯讀 fixture，沒有冒稱已測任意外部 MCP server。

四份 Cargo.lock 已執行 RustSec audit，當次沒有已知漏洞，但有未維護依賴警告。Tauri／GTK3 所需 GLib 0.18.5 使用帶 MIT 授權的官方安全修補回移，另外執行 optimized release regression；不以忽略 advisory 取代修正，也不宣稱這是全面安全認證。

SQLite 知識庫每 owner 最多1000文件；同主機多程序使用 WAL，不支援多主機網路檔案系統。執行中不得只複製主 SQLite 檔；使用已驗證的備份命令，或完全停止所有寫入程序再搬移。Pet 備份保留 SQLite、設定與素材。

Analytics 為匿名事件和受控詞頻的描述性計數，不代表獨立真人數、品質或因果成效。FileOrganizer 的 Trash 保留資料，不宣稱釋放磁碟空間。資料治理與寵物刪除提供可逆封存／還原，沒有永久抹除功能。跨平台安裝包沒有商業 Authenticode／Apple Developer notarization。

## 清理與教學

移除重複 day9 範例、舊 workflow、未接線舊 UI／ProfileStore、生成檔與私人資料入 Git 的風險；保留必要 source、Cargo.lock、migration、重現腳本及有效驗收證據。舊測試和失敗原因可從 Git 歷史查核；不得把它們標成目前成功。

`williechen/rust_ai_chat` 嚴格唯讀。正式 tag 固定後，才在 copilot_use 的獨立教學 branch 調整實際目錄、程式與對應教學；測試保留在所屬 app／crate／模組，禁止建立外層 tests workspace。每日教學與正式固定產品分開驗收。
