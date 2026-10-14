# P9 本地與跨平台驗收

交付範圍為三產品的本地測試、Docker release 建置、跨平台安裝／更新與版本固定；使用者已取消公開部署。正式來源 SHA、六項 CI 與安裝包 checksum 以 [GitHub Release](https://github.com/williechen/copilot_use/releases) 的 `release-manifest.json` 為準。報告需核對精確來源及 dirty 說明；新程式變更要有匹配證據。有效程式基準見 learning-state.json。

## 固定成品與來源匹配證據

以下為固定成品證據涵蓋的能力；只歸屬各報告所記來源 SHA，不代表當前分支或新增目標已重新驗收。

| 產品／整合 | 固定證據涵蓋範圍 | 證據 |
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

## 變更驗收

目錄與程式微調以 rust_ai_chat 為主，產品目標以 copilot_use 為主。依 [共同學習規則](COMMON_LEARNING_RULES.md) 和 [未來清單](FUTURE_WORK.md) 執行必要 source／fmt／Clippy／unit／DB／HTTP／WS／WASM／GUI／E2E／CI。文件整理不重複付費 AI 或相同程式的完整建置；確認程式 tree 未改變與文件連結有效。

正式 Release／docs/release-evidence 保存固定成品證據；當前已驗證程式及參考來源見 learning-state.json。比較和教學的結果不得混算 rust_ai_chat 的實際套用進度。

## Analytics 通訊驗收索引

有效程式基準 `39f34854b102eb96b44980d42d7b3f272307ec20`、匹配 CI 與各類實際驗證結果保留於 [analytics-transport-validation.json](analytics-transport-validation.json)。契約與限制見 [ANALYTICS_TRANSPORT.md](ANALYTICS_TRANSPORT.md)。此基準不同於本輪靜態核對 source SHA；不得把它算成最新分支完整重測。

## 當前核對與待驗收

兩庫既有完整靜態核對的來源與覆蓋率見 [FULL_SOURCE_REVIEW.md](FULL_SOURCE_REVIEW.md)，其歷史 SHA 不代表今日全量重測。最新參考庫局部複驗見 [rust_ai_chat 驗證證據](source-review/rust-ai-chat-validation.json)：2026-10-14 21:51 Asia/Taipei，乾淨 main `3b524f4720efbda691a8d2829b47e3b7dd8b45a5` 的 fmt、Clippy、WASM、UI／server SSR、desktop-pet 編譯及 89 項測試通過、零警告；遠端 CI 為 5/6 workflows、16/18 Check Runs success，Rust AI Chat 仍執行中，最新 Gate 尚未全通過。

rust_ai_chat 的 Day 10 三項契約已實作並通過本地測試，但 Chat WS 房間守衛、Pet 驗證後外觀解析與 Analytics 去重入口仍僅測試使用；Analytics 有效 HTTP 事件仍回 501，故參考庫的產品路徑驗收未通過。此結果不替代 copilot_use 的固定成品／`validated_program_sha` 證據；本輪未重新驗收 copilot_use。copilot_use 的視窗漫遊、SHA256 重複檔案分組及 SSE 授權修正仍按既有來源列為未實作／待驗收，活動項目維持在 [FUTURE_WORK.md](FUTURE_WORK.md)。`last_full_comparison=null`、Day／階段及週六到期日未前移。

週六功能 Gate 依共同規則排除 AI 項目，不讀取 key、不以 AI 未重測阻擋功能通過；固定 Release 的 AI 證據及選擇性真實 AI 操作仍只歸屬原來源。每日 Day 由適用 CI Gate 決定，與週六提交／功能驗收及學習套用分開。
