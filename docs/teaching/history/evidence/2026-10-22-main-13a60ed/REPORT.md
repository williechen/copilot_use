# rust_ai_chat main 13a60ed 歷史增量驗證

此為13a60ed驗證時的歷史快照；後續aaa9194已恢復ignore，最新結果見[aaa9194報告](../2026-10-22-main-aaa9194/REPORT.md)。

2026-10-22（Asia/Taipei）。最新來源 `13a60edeafd6c0f54460f0d2af5ee330fbc34f7e`；結論：Day24 測試修正已通過完整 PostgreSQL 測試，預設測試／Test All 仍有資料庫環境缺口。

## 本輪來源與變更

教學分支基準 `9716c24f7b3c87a1337f91a82ad10635db598591`。隔離副本170個 tracked 檔案與 Git blob 逐位元組一致；產品遠端及使用者未提交文件未修改。

相較前次 `a06c038`，本輪涵蓋兩個提交：

- `183a058`：Day24 表單的 `mail` 改為 `email`，修正兩處 SQL 字串引號；另將 `login_query_postgres_and_logout_flow` 由 ignored 改成預設執行，連線變數改用 `DATABASE_URL`。
- `13a60ed`：Persistence workflow 不再只跑 postgres_event_store，改跑完整 `cargo test --locked -p stats-server --features postgres-integration`。

僅上述兩個 Rust 測試檔與一個 workflow 有差異；production handler、UI、migration、Cargo manifest／lock 未改。上輪 HTTP／GUI缺口／舊庫探針結果只在相關程式與依賴未變的範圍內作歷史證據引用，不冒稱在新 SHA 全部重新執行。

## 最新 SHA 的實跑結果

環境：Linux、Rust/Cargo1.99.1、原生 PostgreSQL18.6，專用loopback55449；測試憑證及資料僅用於隔離庫。未讀正式秘密、未執行付費AI。

| 命令／條件 | 結果 |
|---|---|
| 隔離 PostgreSQL `DATABASE_URL`；`cargo test --locked -p stats-server --features postgres-integration` | **31 passed、0 failed、0 ignored** |
| 未設定 `DATABASE_URL`；`cargo test --locked -p stats-server` | **18 passed、1 failed**；admin.rs:673 讀取連線變數時 `NotPresent` |

完整 PG 31項：library23、Day23登入2、Day24改密碼2、Day25再次改密碼1、idempotency_contract2、postgres_event_store1。Day24 的錯誤CSRF、重複Email、撤銷Session／交易回滾與重複資料migration測試均已執行通過；先前「mail／email與SQL引號導致兩項失敗」已不適用於最新main。

`183a058` 也先以同一隔離PG對上述targets驗證31通過；`13a60ed` 改workflow後另在最新SHA實跑完整命令，結果相同。這兩次不能相加成62個不同測試。

本輪沒有重跑完整workspace、Clippy、fmt、WASM、真HTTP、Pet原生GUI、舊庫升級或跨平台；前次workspace137通過／1ignored只屬於a06c038，不能標成新SHA全套通過。舊版fmt、Clippy、Node缺口相關程式未修，保留待辦。

## CI與仍需修正的測試設定

最新 [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39650799044) 已failure：8個job成功，`test-stats-server`失敗。[該job](https://github.com/williechen/rust_ai_chat/actions/runs/39650799044/job/119398281891) 的workflow未設定DATABASE_URL或提供PG服務，卻執行已預設開啟的PG測試；本地以相同缺環境條件重現NotPresent。未讀取遠端原始job log，因此只確認本地重現與workflow缺口，不宣稱遠端每一個錯誤堆疊相同。

Persistence workflow雖已擴大PG覆蓋，但它是另一個job，不能替Test All job提供資料庫或環境變數。後續需在Test All配置專用PG，或把需PG的測試明確歸入已配置PG的feature／target，同時保留完整PG驗證；不能只以忽略測試掩蓋缺口。本輪依唯讀規則只記錄問題，未修改產品workflow。

[Node](https://github.com/williechen/rust_ai_chat/actions/runs/39650799137)仍failure。其餘workflow的擷取狀態、job、精確SHA、時間及連結見 `ci-13a.json`；擷取時尚在執行者不標成功，後續完成狀態以GitHub為準。

## 文章、進度與保留項目

更新 Notion 共同主頁與Analytics入口，撤換已過時的「Day24 fixture失敗」描述，加入31項通過與預設DB環境失敗，並同步教學庫目前進度、manifest及原始log。三案Day25（含）以前正文與程式不改，next_day維持Chat26／Pet26／Analytics26，last_full_comparison不變。

Analytics Day24補教保留pending：fixture技術錯誤已修正並驗證，但Day24歷史庫checksum升級仍未修；本輪沒有在下一原教學的新Red／Green／Refactor後完成補教發布，因此不將整筆補教結案。共用登入／登出／Cookie／Session、Chat API／WS房間ACL、Pet每幀單一圖像與新SHA三窗GUI等既有待辦保留。

前次舊庫證據：Pet自4f0cd00仍VersionMismatch(20261018095626)，自Day24可升級；Analytics自4f0cd00可升級，自Day24仍VersionMismatch(20261018073932)。本輪migration沒有差異，未重測這四條路徑，也未改checksum或刪除既有資料。詳見[前次完整報告](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-22-main-a06c038/REPORT.md)。

依使用者說明，排程內容已更新；本輪未再修改、啟停、新增或觸發排程，也未重新驗證實際模型設定。

測試完成後停止並清除本輪專用PG與編譯快取；既有5432、產品資料庫及原始工作目錄保留。
