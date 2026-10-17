## 1. 學習大綱
來源 `rust_ai_chat/main` `fced2861bba21c33697051f685c50689f8052de8`；[同 SHA CI](https://github.com/williechen/rust_ai_chat/actions/runs/38992992860) 六個 workflows、18 checks Success。**本篇尚未套用、尚未編譯／執行測試，不能借來源 CI 宣稱新功能驗收通過。** Red＋最小 Green 20 分鐘。
承接 Day 15 已有 `/dashboard`、`/api/v1/events/{source}/summary`、`/api/v1/events/{source}/summary/{event_type}` 與 PostgreSQL 聚合，新增**受來源認證保護的單一總數 JSON 查詢**，讓未來 Leptos Dashboard 有真實總數資料來源，而不是在前端假造數字或另維護統計狀態。此切片仍是來源 token 過渡方案，**不是管理員 Session／ACL**；匿名 HTML Dashboard 的安全缺口維持未完成，不能直接公開。FUTURE_WORK 對應 Analytics「Dashboard／Query、來源權限、HTTP JSON」。
## 2. 實作內容
### Red（8 分鐘）
在 `apps/stats_server/src/lib.rs` 末尾新增測試，確認既有來源聚合可以導出可供 UI 使用的總數：
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
**注意：** `EventSummary.total` 的實際整數型別必須以 `apps/stats_server/src/pg_store.rs` 宣告為準；若該型別不相容，下列函式的回傳型別應與原型別一致，不可強制截斷。新增測試在 `summary_total` 未實作時預期 Red；**未執行**。
### 最小 Green（12 分鐘）
在 `apps/stats_server/src/lib.rs` 既有 `source_summary` 函式之後加入：
```rust
// 只對既有來源聚合求總和；不重新讀取另一套資料來源。
fn summary_total(rows: &[pg_store::EventSummary]) -> Option<i64> {
    rows.iter().try_fold(0_i64, |sum, row| {
        // 將聚合值轉為 i64，拒絕溢位；不將異常值當作 0。
        let count = i64::try_from(row.total).ok()?;
        if count < 0 { return None; }
        sum.checked_add(count)
    })
}

#[derive(serde::Serialize)]
struct SourceTotal {
    source_app: String,
    total: i64,
}

async fn source_total(
    State(state): State<IngestState>,
    axum::extract::Path(source): axum::extract::Path<String>,
    headers: HeaderMap,
) -> Result<Json<SourceTotal>, StatusCode> {
    if !matches!(source.as_str(), "web_chat" | "desktop_pet") {
        return Err(StatusCode::BAD_REQUEST);
    }
    let authorization = headers.get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok()).unwrap_or("");
    if !state.tokens.authorized(&source, authorization) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let rows = pg_store::summary_for_source(&state.pool, &source)
        .await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let total = summary_total(&rows).ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(SourceTotal { source_app: source, total }))
}
```
在現有 `build_app` 的 `.route("/api/v1/events/{source}/summary", get(source_summary))` **下一行**新增：
```rust
.route("/api/v1/events/{source}/total", get(source_total))
```
功能鏈：HTTP GET `/api/v1/events/web_chat/total` → 來源白名單與 Bearer 憑證檢查 → `pg_store::summary_for_source` → PostgreSQL 已提交資料聚合 → `{"source_app":"web_chat","total":5}`。不將 token 放 URL，不對未授權來源查 DB，不使用記憶體去重當作持久化證據。
### Refactor
### 根目錄 tests 遷移計畫
範圍僅為根目錄 `tests/analytics/`：`event_kind_tests.rs`、`http_contract_tests.rs` → `apps/stats_server/tests/`。`postgres_ingest.rs` 目前為空檔，先確認沒有測試與其他引用，再移除其 `[[test]]` 登記及空檔；不宣稱它提供 PostgreSQL 覆蓋。既有 `idempotency_contract.rs`、`postgres_event_store.rs` 保留。HTTP 測試目前會讀取 DB／來源憑證環境設定，搬移時必須同步 CI 測試環境；既有過時斷言的修正須與純搬移分開記錄。
遷移是配合本日主功能的整理工作，**不拆分各原始檔內既有單元測試，不要求每個檔案另建 **[**tests.rs**](http://tests.rs)。
Red：以 `tests/Cargo.toml` 的九個明確 `[[test]]` 為基準，記錄測試名稱、數量、既有失敗與環境需求。Green：搬移上述檔案，將所需 dev-dependencies 加到目的 package，移除 workspace-tests 對應登記及不再使用的相依；維持原斷言。Refactor：將 `.github/workflows/test.yml` 的 `cargo test --locked -p workspace-tests` 改為目的 package 的測試命令，確保 CI 持續收集全部案例。新增功能測試與純搬移分開提交／驗證。
三專案搬完後再統一檢查：根目錄 `tests/common/mod.rs` 目前為空；確認無剩餘共用或跨專案測試及引用後，才移除 `tests/Cargo.toml`、工作區 `members` 的 `tests` 與空目錄，並更新 lockfile。若仍有真正跨專案測試則保留其 workspace-tests 容器與 CI，不強制清空。
驗證：`cargo test -p stats-server --test event_kind_tests --test http_contract_tests`（先準備測試專用環境）；搬移前後比對案例，不能出現漏跑、重複、無故 ignored 或以刪測試取得通過。**目前僅更新教學計畫；尚未搬移、未執行上述驗證。**
不修改既有 migration、EventRepository 或 `apps/stats_server/tests/idempotency_contract.rs`；新 handler 僅重用現有來源驗證與 PostgreSQL 聚合。後續可將來源授權提取為共用 extractor，但必須保留現有 source／kind 端點的錯誤碼測試。總數端點是未來 Leptos 管理 UI 的資料能力，**不宣稱已接通 Leptos**。
### 舊 Day 更正補充
Day 15 獨立 `dashboard.html` 與 JS 是已實作歷史，保留不覆寫；產品需求指定未來以 Leptos 管理後台取代，並加人員 Session／ACL，不能把來源 token 當人員登入。此更正放在 Day 16 而不是修改 Day 15。**匿名 ****`/dashboard`**** 尚未符合產品安全 Gate；不可部署到不受信任網路。**
## 3. 套件版本
沿用 `axum`、`serde`、`sqlx`、`tokio` 與既有 PostgreSQL；不新增依賴，不改 migration。實際版本以 `fced2861` 的 `Cargo.toml`／`Cargo.lock` 為準；官方 stable／MSRV **未查證**。
## 4. 驗證項目
1. Red：`cargo test -p stats-server day16_total_tests`，先預期編譯失敗；**未執行**。
2. Green：`cargo test -p stats-server day16_total_tests`、`cargo check -p stats-server`；**未執行**。
3. HTTP：以 fixture token 測 401（缺 token／跨來源）、400（未知來源）、503（DB 失效），以及已提交 PostgreSQL 事件的真實總數、重送不重複計數；**未執行**。
4. `postgres-integration` 測試需真實隔離 PostgreSQL，禁止將來源 CI 的一般測試結果當成本篇 DB 驗證；**未執行**。
5. Leptos 管理 Session、GUI、跨服務 E2E、正式驗收：**未完成**。
