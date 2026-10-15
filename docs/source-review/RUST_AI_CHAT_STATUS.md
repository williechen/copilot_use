# rust_ai_chat Day 12 正式驗證判定

實際本地專案 `/home/willie/Documents/rust_ai_chat`，驗證取樣時的乾淨提交 `a57e532c125f5f18d7fd2a250401efde0c9d2612`；時間 `2026-10-15T21:53:02.011664+08:00`。先驗本地，再核對同 SHA GitHub，未修改學習倉庫。

| 範圍 | 判定 |
|---|---|
| Linux 本地正式驗證 | 通過：107 項 Rust、3 項 Node、fmt、嚴格 Clippy、SSR／兩個 WASM |
| Release 建置 | Linux native、Chat Leptos SSR/hydrate、Pet frontend 通過 |
| Chat Day 12 | 實作與測試已套用；實際 WS Typing 廣播／房間隔離／不寫訊息通過 |
| Pet Day 12 | 明確啟用控制測試與 native build 通過；實際 GUI 移動尚未驗證 |
| Analytics Day 12 | HTTP／PostgreSQL 真實攝取、認證、201／200／409／400／503、併發、重啟去重通過 |
| 同 SHA GitHub CI | 3/6 workflows、13/18 checks，failed |
| 全部 Day 12／P9／週六驗收 | 不宣稱全部完成；未前移週驗收或學習階段 |

Analytics 現已接上 PostgreSQL，已不再對有效事件固定回 501。當前 Test All workflow 未提供新增 workspace 測試要求的 ANALYTICS_DATABASE_URL／兩個來源 token／PostgreSQL；本次本地使用獨立資料庫與合成 token，CI 結果另列，不沿用舊 SHA 綠燈。rust_ai_chat 嚴格唯讀，不修改其 workflow。

未操作實際 GUI／跨平台安裝包；gRPC／SSE 及正式憑證安全審查不在本次 HTTP 切片通過範圍。未讀取私人 .env／AI key、未執行付費 AI。`.env.example` 的 PET_ENABLE_ROAM=true 與 PET_AUTHORIZED_ROOT=/ 不是本次安全測試設定；本次未授權或掃描根目錄。

[精確結果](rust-ai-chat-validation.json) · [可攜式命令／日誌／來源雜湊](rust-ai-chat-validation-logs.zip) · [三篇文章](../teaching/current/README.md)。

[CI 測試環境診斷與待套用設定](DAY12_CI_ENVIRONMENT.md)。驗證取樣後本地 `persistence.yml` 出現未提交調整，未納入上述 SHA 的驗證／CI；其 env 不會作用到另一個 workflow 的 test-workspace。
