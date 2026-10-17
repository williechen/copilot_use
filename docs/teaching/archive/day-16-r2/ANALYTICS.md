# 2026-10-17｜P0 Analytics Day 16｜登入 Leptos 後台並查詢

## 1. 學習大綱
修訂 r2（2026-10-17）。來源：`rust_ai_chat/main` `b2733b7f78dce1da1aa07555122384c27109e4e8`；本機工作目錄仍為 `fced2861`，本輪僅 fetch／靜態讀取，未 checkout、未改原始碼。
**狀態：舊版 Day16 的 Red 已部分套用；本次主流程為待套用教學計畫，未提供已驗證的完整實作。** 最新 SHA 的 18 個 checks 為 16 success／2 failure（`test-ui`、`test-stats-server`）；不沿用 Day15 CI 作為本篇通過證據。三篇主體只優先交付可操作的基本流程，搜尋／總數／競態是相關附加工作。

交付流程：開啟 Analytics → 未登入先到登入頁 → 管理員登入 → 使用 Leptos 後台選來源與事件種類 → 查詢既有 PostgreSQL 聚合 → 看見真實結果或空結果 → 登出後無法再查詢。
Analytics 是純後台。人員登入 Session 與來源服務 Bearer 憑證分開；來源 token 不可當作管理員登入。交互換證仍是後續分段工作，不把配對、輪替、撤銷的全部細則塞進本日。
## 2. 實作內容
### Red：先驗證登入與真實查詢邊界
保留已套用的 `day16_total_tests`（見本節末）。另先定義：未登入不能取 Dashboard 業務資料及查詢 JSON；錯誤登入被拒；登入成功可查真實資料；登出或 Session 過期後被拒；來源 Bearer 單獨不能取得管理員權限。
### Green：Leptos 後台與查詢接線
1. 在 `apps/stats_server` 加入 Leptos SSR 頁面／元件與對應 Cargo feature；本日可用伺服器表單完成登入、查詢、登出，不必先引入整套前台 hydration。不可仍用舊 `dashboard.html` 加 JS 卻宣稱已用 Leptos。
2. 設定管理員驗證方式並建立伺服器端 Session。不得硬編碼帳密、把來源 token 當人員憑證或將明文密碼寫入儲存。登入憑證以適合其熵值的安全雜湊驗證；Session 需不可猜測、有期限、可在登出失效，Cookie 設 HttpOnly、SameSite，TLS 環境設 Secure；登入／登出等寫入檢查 CSRF。設定缺漏時明確拒絕啟動或登入，不能匿名放行。
3. 在 `lib.rs` 路由層保護 `/dashboard`、查詢 API 及任何承載業務資料的 SSR／server functions。未登入頁面導向登入，API 回 401；登入成功後用既有 `pg_store::summary_for_source`／`summary_for_source_and_type` 查 PostgreSQL。
4. Leptos 畫面顯示來源選單、事件種類輸入、查詢按鈕、結果表格、空結果／錯誤與登出。僅顯示真實查詢結果。既有 ingest 保留來源憑證驗證；人員 Session 與機器權限分離。
5. 已套用的總數 Red 可在結果表格之後完成：對既有聚合安全求和，處理負值／溢位；不必另加 `/total` API，也不以此取代登入或畫面。
### Refactor：根目錄 tests 的 Analytics 批次
`tests/analytics/event_kind_tests.rs`、`http_contract_tests.rs` → `apps/stats_server/tests/`。空的 `postgres_ingest.rs` 確認無引用後，移除空檔及其 target 登記，不將空檔算成 DB 覆蓋。既有 `idempotency_contract.rs`、`postgres_event_store.rs` 保留。
目的 package 補 `serde_json` 與 `tower`（util）等實際缺少的 dev-dependencies；原 HTTP 測試需要 DB 與兩組來源憑證，CI 必須保留等價測試環境。名稱 `valid_event_remains_unimplemented` 與目前 401 斷言不符，可另行更名，不能偷偷變更原驗證含義。

搬移前，以 `tests/Cargo.toml` 的 target、測試名稱與數量建立基準；搬移時保留原斷言，調整目的 package 的 dev-dependencies，再移除原 `[[test]]` 與不再使用的相依。不要拆分原始碼內嵌單元測試。同步 `.github/workflows/test.yml`：目的測試須在具備原 DB／環境設定的 job 執行；不得將需要 DB 的案例直接丟給未配置 DB 的 job。其他根目錄批次尚未完成時保留 `workspace-tests` 與 CI；全部驗證後才移除根目錄測試容器、workspace member 與空目錄，更新 lockfile。測試名稱／數量不得無故減少或重複，既有失敗與功能變更另記，不在純搬移中偷改斷言。

### 舊 Day 更正補充與已套用紀錄
Day15 原文封存。舊匿名 HTML Dashboard 是歷史實作，不符合本日交付。既有 summary 來源授權測試需配合人員登入邊界新增或明確替代覆蓋，變更理由與純搬移分開。以下 Red 已在來源 SHA 套用，原樣保留（其中 `use pg_store::EventSummary` 的匯入路徑也須於 Green 檢查）：
```rust
#[cfg(test)]
mod day16_total_tests {
    use super::summary_total;
    use pg_store::EventSummary;

    #[test]
    fn total_uses_actual_committed_summary_counts() {
        let rows = vec![
            EventSummary {
                source_app: "web_chat".to_owned(),
                event_type: "chat.message.persisted".to_owned(),
                total: 2,
            },
            EventSummary {
                source_app: "web_chat".to_owned(),
                event_type: "chat.room.created".to_owned(),
                total: 3,
            },
        ];
        assert_eq!(summary_total(&rows), Some(5));
    }
}
```
## 3. 套件版本
來源 workspace 0.2.15／edition 2024；Axum 0.8、SQLx 0.9、Tokio 1.53。Leptos 0.8 SSR 為本日預計加入 stats-server 的依賴。管理員憑證／Session 方案與其依賴仍待落實及驗證，不能標成已完成。精確版本依實作後 Cargo.lock；未核實官方最新 stable 或 MSRV。
## 4. 驗證項目
`cargo test -p stats-server`；`cargo check -p stats-server`；搬移後執行 `--test event_kind_tests --test http_contract_tests`（指定 stats-server package 並準備專用測試環境）。真實 PostgreSQL：`cargo test -p stats-server --features postgres-integration --test postgres_event_store`，環境變數依測試檔設定；不得用日常資料庫做破壞性測試。瀏覽器驗收：未登入拒絕→登入→來源／種類查詢→真實結果／空結果→登出拒絕。
本輪只核對來源、已套用 Red 與遠端 CI，**上述新主流程、Green、tests 搬移及驗收命令均未執行**。來源 Red 的預期失敗與非預期錯誤要分開記錄；修正後重新檢查同一 SHA 的結果，不能刪除測試讓 CI 變綠。完成條件是主流程可操作及相關驗證通過，不是只有頁面發布或 helper 測試通過。
