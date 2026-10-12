# 下一次 Day 9 教學範圍

此為下一次教學的準備清單，尚未建立正式 Day 頁。按共同規則先讀 rust_ai_chat 最新 source／前一有效教學，完成 copilot_use 對應、必要 source／測試／提交後，才發表完整五節。每日預定 08:00 Asia/Taipei；必要 Gate 受阻時先修正並清楚記錄。

## Chat：持久化與通知的可靠邊界

實際對照 chat_domain 訊息／作者規則、chat_application::MessageRepository/persist、persistence::ChatWriter/OutboxStore、server HTTP／WS 及 shared::ProductEvent。先驗證／持久化，再通知／投遞；DB 失敗、部分串流與重送不算新成功訊息。依兩邊相同的 SQLite 目標提供必要遷移銜接，完整 Domain／Port／Adapter／契約／必要 migration／呼叫端與既有測試重構一起提供。測非法作者／輸入、DB 失敗不通知、訊息／Outbox 原子性、重送／精確 ID，必要 HTTP／WS／SQLite 整合。

## Desktop Pet：預覽、確認與真實成功收據

對照 desktop_pet IPC／frontend、pet_domain 純設定／Port、pet_runtime::AuthorizedRoot/receipt/SQLite Outbox。銜接現有 Bridge 的型別化 preview 顯示、授權 root、確認／重新驗證、真實 receipt 與成功事件；預覽或純 state.complete(true) 不代表實際清理。完整 UI／IPC／runtime 與私有／平台測試。測未確認不操作、symlink／TOCTOU／opaque ID／OS 拒絕邊界、成功收據與重送一次計數，必要真實 GUI／FS／SQLite 回歸。

## Product Analytics：來源認證與 PostgreSQL 持久冪等

對照 stats_server::application::EventRepository/Analytics、persistence::EventStore、認證 ingest、shared 四欄事件及所屬 migrations。真正來源認證、落庫成功後才回 2xx、same source/event ID 去重及衝突拒絕；記憶體去重不替代 DB。完整 Application／Adapter／HTTP／migration／測試與 FakeStore／HTTP 契約整併。測 401／400／409／DB 失敗非 2xx、並發／重送／重啟持久去重，以及必要 Chat/Pet→Analytics E2E。

## 正式 Day 五節

1. 學習大綱：切片、輸入／輸出、不變量、來源。
2. 實作內容：實際路徑、完整最終程式、繁體中文註解與資料流程。
3. 套件版本：Cargo 宣告／lock、相容性、官方核對及維持／調整理由。
4. 重構內容：既有必要重構、所屬測試、私有 API／安全邊界；未變檔 N/A。
5. 驗證項目：命令、預期、實際結果、SHA／CI 與未核實項目。

三專案各自查重並確認編號，按 Chat→Pet→Analytics 寫在現行專案主頁下並讀回。封存教學僅供來源／查重；rust_ai_chat 仍唯讀，教學完成不表示使用者已套用。
