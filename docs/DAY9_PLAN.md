# 下一次 Day 9 教學範圍

此為候選範圍，不是已確認的每日選題。Notion 已有 Day9 頁面，須先查重；每日以兩個 repository 最新 SHA 的完整差異基線與逐項分類，重新確認各專案最小切片及五節內容。以下大範圍不得直接整包作為單日題目，未經確認不得推進 Day10。每日預定 08:00 Asia/Taipei；教學發布與產品實作驗收分開，未執行的測試明列未執行，不以產品先提交為教學發布條件。

## Chat：持久化與通知的可靠邊界

實際對照 chat_domain 訊息／作者規則、chat_application::MessageRepository/persist、persistence::ChatWriter/OutboxStore／gRPC Transport、server HTTP／WS 及 shared::ProductEvent。先驗證／持久化，再通知／投遞；DB 失敗、部分串流與重送不算新成功訊息。依兩邊相同的 SQLite 目標提供必要遷移銜接，完整 Domain／Port／Adapter／契約／必要 migration／呼叫端與既有測試重構一起提供。測非法作者／輸入、DB 失敗不通知、訊息／Outbox 原子性、重送／精確 ID，必要 HTTP／WS／SQLite 整合。

## Desktop Pet：預覽、確認與真實成功收據

對照 desktop_pet IPC／frontend、pet_domain 純設定／Port、pet_runtime::AuthorizedRoot/receipt/SQLite Outbox。銜接現有 Bridge 的型別化 preview 顯示、授權 root、確認／重新驗證、真實 receipt 與成功事件；預覽或純 state.complete(true) 不代表實際清理。完整 UI／IPC／runtime 與私有／平台測試。測未確認不操作、symlink／TOCTOU／opaque ID／OS 拒絕邊界、成功收據與重送一次計數，必要真實 GUI／FS／SQLite 回歸。

## Product Analytics：來源認證與 PostgreSQL 持久冪等

對照 stats_server::application::EventRepository/Analytics、persistence::EventStore、grpc 認證 ingest、shared/proto 四欄契約、analytics_transport Driver 及所屬 migrations。真正來源認證、落庫成功後才回逐事件 gRPC ACK、same source/event ID 去重及衝突拒絕；記憶體去重不替代 DB。完整 Application／Adapter／gRPC／HTTP／SSE／migration／測試與 FakeStore／傳輸契約整併。測來源認證／batch／schema／identity conflict／DB 失敗不得成功 ACK、SSE 提交／回滾／重連、並發／重送／重啟持久去重，以及必要 Chat/Pet→Analytics E2E。

## 正式 Day 五節

1. 學習大綱：切片、輸入／輸出、不變量、來源。
2. 實作內容：實際路徑、完整最終程式、繁體中文註解與資料流程。
3. 套件版本：Cargo 宣告／lock、相容性、官方核對及維持／調整理由。
4. 重構內容：既有必要重構、所屬測試、私有 API／安全邊界；未變檔 N/A。
5. 驗證項目：命令、預期、實際結果、SHA／CI 與未核實項目。

三專案各自查重並確認編號，按 Chat→Pet→Analytics 寫在現行專案主頁下並讀回。封存教學僅供來源／查重；rust_ai_chat 仍唯讀，教學完成不表示使用者已套用。
