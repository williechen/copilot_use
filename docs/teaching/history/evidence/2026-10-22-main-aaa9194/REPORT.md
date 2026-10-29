# rust_ai_chat 最新 main 驗證更新

2026-10-22（Asia/Taipei）。來源 `aaa9194a966aefae1ef4724f5d2b576b4289c550`，教學基準 `9716c24f7b3c87a1337f91a82ad10635db598591`。本輪隔離副本170個tracked檔案與Git blob逐位元組一致，未修改產品遠端或使用者未提交文件。

## 最新結果

| 最新SHA實跑 | 結果 |
|---|---|
| 設定隔離PG的 `DATABASE_URL`；`cargo test --locked -p stats-server --features postgres-integration -- --include-ignored` | **31 passed、0 failed、0 ignored** |
| 不設定DB URL；`cargo test --locked -p stats-server` | **20 passed、0 failed、1 ignored** |

預設20項包括library18及idempotency_contract2，另1項PG測試ignored。PG31項包括library23、Day23登入2、Day24改密碼2、Day25再次改密碼1、idempotency_contract2、postgres_event_store1。原Day24表單欄位與SQL引號造成的兩項失敗已修正並在最新SHA通過。環境為Linux、Rust/Cargo1.99.1、原生PostgreSQL18.6、專用loopback55449；使用本輪合成資料與測試憑證，未讀正式秘密或執行付費AI。

## 連續提交與驗證範圍

- `183a058` 修正Day24的mail／email及兩處SQL引號，同時取消一個PG測試的ignore，改讀DATABASE_URL。隔離PG31項通過，但缺DB URL的預設測試會失敗。
- `13a60ed` 將Persistence workflow由單一postgres_event_store改為完整postgres-integration；該SHA完整PG31項通過，無DB URL預設測試18通過／1失敗。原始log及當時Notion快照保留於13a60ed證據目錄。
- `aaa9194` 恢復該PG測試的ignore；本輪以最新SHA重跑上表兩種條件，預設測試已恢復通過，並以`--include-ignored`明確驗證全部31項。

**測試數不可相加。** 三個SHA的31項是同一組測試重驗，不能算成93個不同案例。忽略訊息仍提到ANALYTICS_TEST_DATABASE_URL，但實作實際讀取DATABASE_URL；本輪按實作設定隔離PG。

相較前次a06c038，只改兩個Rust測試位置及一個workflow；production handler、UI、migration、Cargo／lock未變。本輪未重跑workspace、fmt／Clippy、WASM、真HTTP、Pet原生GUI、四條舊庫升級或跨平台；前次137通過／1ignored僅屬a06c038，不寫成最新SHA的整套結果。相關程式未變的前次HTTP／資料庫觀察保留原SHA及覆蓋範圍，不冒稱重新實跑。

## CI覆蓋與剩餘限制

最新workflow／job／時間／精確SHA／連結見`ci-aaa.json`。本輪發布快照時[Test All九個job均success](https://github.com/williechen/rust_ai_chat/actions/runs/39651403292)，Dashboard亦success，Node仍failure；尚在執行的job只標in_progress，不因本地測試通過推定遠端成功。

Persistence workflow現已涵蓋完整PG targets，但命令沒有`--include-ignored`，因此仍會略過恢復ignore的登入／查詢／登出PG測試。一般Test All可在無PG環境略過它；若要在CI聲稱本輪31項全部執行，需在已配置隔離PG的job明確加入`--include-ignored`或等效專用命令。這是覆蓋範圍提示，本輪沒有改寫來源workflow或恢復已封存教學。

預設測試原先缺DATABASE_URL的失敗，已由最新ignore變更解除；不再列為最新main的必然失敗。Node、既有fmt／Clippy、共用登入與產品驗收缺口仍分開追蹤，不能以Rust測試通過結案。

## 必要文章與進度

更新Notion共同主頁與Analytics入口、GitHub目前進度／manifest／learning-state及證據。三案Day25（含）以前正文、程式及封存雜湊保留；next_day維持Chat26／Pet26／Analytics26，last_full_comparison不變，沒有發布新Day。

Analytics Day24補教仍pending：fixture已修正並驗證，但Day24歷史庫checksum與下一原教學的新Red／Green／Refactor後補教發布仍未完成。Chat共用登入／登出／Cookie／Session、API／WS房間ACL、Pet每幀單一圖像與此SHA三窗GUI、Analytics角色／來源授權等既有待辦保留。舊庫migration沒有差異，本輪未重測或改checksum、不刪使用者資料。

前次完整範圍見[a06c038報告](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-22-main-a06c038/REPORT.md)。本輪僅驗證學習來源，不代表copilot_use產品已整合或完整驗收。

依使用者說明排程內容已更新，本輪未再修改、啟停、新增或觸發排程，亦未重新驗證實際模型設定。測試完成後停止並清除本輪專用PG與編譯快取，既有5432與產品資料保留。
