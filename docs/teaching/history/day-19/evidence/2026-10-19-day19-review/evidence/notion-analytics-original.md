Here is the result of "fetch" for the Page with URL https://app.notion.com/p/3fde51d4509381ed8a2eecf3ba7e4093 as of 2026-10-18T22:16:47.411Z:
<page url="https://app.notion.com/p/3fde51d4509381ed8a2eecf3ba7e4093">
<ancestor-path></ancestor-path>
<properties>
{"title":"Day19 Analytics login throttling draft"}
</properties>
<iconMetadata>null</iconMetadata>
<content>
## 1. 學習大綱
Successful logins should not count toward failed-login limits.
## 1. 學習大綱（完整）
本日切片：成功登入不消耗失敗登入節流額度，避免合法操作 11 次後被 429；失敗 10 次後仍阻擋。主體 Red 5 分鐘、Green 10 分鐘，Refactor 5 分鐘、HTTP／安全回歸 10 分鐘另計。來源 rust_ai_chat/main `f692c2ebe7b414fe49858c1ab1c4a33ce7c3dfee`、規則 daily-tdd-minimum-v1。
呼叫鏈 `apps/stats_server/src/admin.rs::build` → Axum POST /login → Origin 檢查 → Form\<Login\> → Config SHA256 key 比對 → Admin.attempts → Session／Set-Cookie → Redirect /dashboard。現況在 key 正確性檢查前就執行 `attempts.1 += 1`，成功登入也耗掉 60 秒內 10 次失敗額度。只修這個切片，不冒稱已完成 admin/admin 帳密或角色。
## 2. 實作內容
Red：在 [admin.rs](http://admin.rs)::tests 新增 day19_successful_logins_do_not_consume_failure_budget。使用 app() 與合成 key，連續 11 次成功登入應為 303；10 次錯誤應為 401，第 11 次錯誤應為 429。舊程式第 11 次成功登入就會返回 429。此 Red 尚未實際執行。
Green：保留既有節流時間窗與門檻，移除成功登入會累加失敗次數的動作，將累加移至驗證失敗分支。成功仍建立 Session 與 Cookie，失敗仍拒絕登入。完整 Rust 測試與程式替換碼另存本次附件。Refactor：集中驗證結果，不記錄秘密；全域節流、帳密與角色仍為後續需求。
## 3. 套件版本
已讀 apps/stats_server/Cargo.toml：Axum、SQLx、Tokio、sha2、serde 使用 workspace 版本；Leptos 啟用 SSR；測試 tower 0.5 util。無新增套件、migration 或正式密鑰；Cargo.lock 解析版本、MSRV 本輪未獨立重查。
## 4. 驗證項目
單元：cargo test --locked -p stats-server day19_successful_logins_do_not_consume_failure_budget。回歸：cargo test --locked -p stats-server。真 HTTP 使用合成測試資料與獨立 DB；均未執行。來源 Dashboard／Persistence／Test All CI 均成功，但未涵蓋本篇修改。此頁為私人草稿，父頁發布未完成，next_day 仍 19；程式未套用、產品未整合、完整功能未驗收。
</content>
</page>