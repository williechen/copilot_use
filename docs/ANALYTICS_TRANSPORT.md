# Analytics：gRPC／Protobuf 上報、HTTP＋SSE 儀表板

本文件描述 v0.1.0 固定後新增的需求；不回寫正式 tag、Release 或舊安裝包。rust_ai_chat 保持唯讀。

## 契約與架構邊界

- `crates/shared/proto/analytics.proto`：四欄 ProductEvent、IngestEvents、逐事件 EventResult。新增欄位用新編號，刪除欄位應保留編號／名稱，不重用。
- `crates/analytics_transport`：Tonic／Prost Driver、建置時產生型別與 client/server、UUID／schema 驗證、deadline／ACK 檢查。使用 vendored protoc，不需額外安裝系統 protoc。純 Shared／Domain／Application 不依賴 Tonic／SQLx。
- `apps/stats_server/src/grpc.rs`：來源憑證／batch／payload 限制，呼叫既有 Analytics → EventRepository → PostgreSQL transaction／dedup。
- Chat Persistence 與 Pet Runtime 的 Outbox 改呼叫 gRPC；SQLite 資料、lease、CAS ACK、backoff／blocked／人工重送機制保留。HTTP JSON 不作自動 fallback。
- `apps/stats_server/src/live.rs`：有讀取權限的 SSE，依已提交 PostgreSQL revision 通知更新；Dashboard 重新使用 HTTP 取得聚合。

新 Transport Adapter 是新產品需求帶來的明確差異；目錄／教學對齊採最新唯讀 reference 的既有邊界，不為檔名相同將 Driver 放入 Domain。

## gRPC 行為

服務 `copilot.analytics.v1.EventIngestion/IngestEvents` 與 HTTP Dashboard／Query 共用 Analytics 3100 port。既有 `ANALYTICS_ENDPOINT=http://127.0.0.1:3100/api/v1/events` 不必變更：gRPC 使用 origin，其他 HTTP 管理操作維持原路徑。

每次 batch 1–64 事件、編碼訊息上限 64 KiB；整批驗證來源 token 後才寫入。使用 authorization metadata 的 Bearer token；Chat key 不可代送 Pet，query key 不可上報。每個事件獨立交易；回應逐事件 STORED／ALREADY_EXISTS／INVALID_EVENT／IDENTITY_CONFLICT。資料庫故障回 UNAVAILABLE，批次先前已提交事件可以相同 ID 重送。

只有收到來源／ID 完全相符且 STORED／ALREADY_EXISTS 的 ACK，Outbox 才標記 delivered。連線逾時 3 秒、RPC 10 秒，短於 30 秒 lease；未知／缺少 ACK 仍待重試。傳輸與暫時性失敗重試；錯誤憑證／不合法事件／身份衝突等永久錯誤封鎖，修正後由既有操作重送。

目前工作者每次投遞一筆，使用同一 batch RPC 契約；沒有宣稱已實作多筆 lease worker、長連線串流或效能提升。HTTP `/api/v1/events` 保留相容與回歸測試入口；所有實際 Chat／Pet Outbox 上報走 gRPC。

## SSE 行為

`GET /api/v1/events/stream` 需 ANALYTICS_API_TOKEN。Dashboard 使用 fetch streaming 帶 Authorization header，避免將 token 放入 URL／cookies／localStorage。收到 `event: invalidate` 後合併密集通知，再查詢 count／summary；修改或清除 token 中止連線。

新增 analytics_revision migration／trigger，INSERT／UPDATE／DELETE 與 revision 更新同交易，rollback 不可見，去重重送不變更 revision。每個 SSE 連線每秒檢查 revision，約 1 秒通知粒度；15 秒 heartbeat。每個程序最多 16 條 SSE 連線，超限回 429，讀取 DB 故障時拒絕建立或結束串流。

每次連線／重連先發目前 revision，前端重新查詢；不依賴 Last-Event-ID 歷史重播，也不把通知當成持久業務事件。多個 Analytics instance 讀同一 PostgreSQL 可觀察同一 revision。未啟用儀表板的明細／趨勢自動刷新；它們維持使用者主動查詢。

## 設定與驗證

本地 HTTP／h2c 同 port；跨主機部署需要支援 gRPC HTTP/2 的 TLS termination，SSE proxy 需停用回應 buffering。使用者範圍仍是本地與 Docker 測試，沒有正式外部部署。

```sh
cargo fmt --all -- --check
python3 scripts/architecture-boundaries.py
# DATABASE_URL 必須指向可建立 disposable 測試庫的本地 PostgreSQL
cargo test --locked -p day9-analytics grpc::tests
python3 scripts/local-verify.py
cargo check --locked --manifest-path apps/desktop_pet/Cargo.toml
```

新測試放在 stats_server 的私有 grpc/tests 模組，涵蓋真實 HTTP/2 gRPC、來源隔離、batch 邊界／大小、併發去重、identity conflict、restart、SQLite Chat／Pet 失敗重試與已存在 ACK、SSE 認證／commit／rollback／重連、DB 故障不成功 ACK。實際執行結果另存驗證報告；舊正式 Release 的通過證據不能替代新功能驗證。

本地初次驗證：完整 Cargo／fmt／Clippy 通過，4 項新增 gRPC／SSE 測試通過；既有真實 Chat／Pet GUI／Analytics 跨服務、故障重送、來源輪替、撤回及 OTel 回歸通過。真實瀏覽器已驗證 SSE 自動更新、清除／重新啟用及 PostgreSQL 故障後自動重連。Docker 與新 SHA CI 結果以另存報告為準，不沿用舊 SHA 結果。

完整固定 SHA、Docker、六項 CI 與重試紀錄見 [analytics-transport-validation.json](analytics-transport-validation.json)。
