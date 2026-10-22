## 1. 學習大綱
本日 **Analytics Day20** 單一最小 Green：既有管理員登入在 **10 次失敗／60 秒** 全域窗口內觸發 HTTP 429 時，回傳標準 `Retry-After` 秒數標頭，讓呼叫端能知道何時重試，而非只能猜測。保留 Day19 成功登入不增加計數與現有 session/Cookie 行為；不擴大成多帳號策略、重設密碼或權限系統。
實際入口：`POST /login → apps/stats_server/src/admin.rs::login → origin_ok → Admin.config → attempts: Arc<Mutex<(Instant,u8)>> → 60秒窗口重置 → >=10 門檻 → 429 → HTTP header`；成功分支仍建立 `Session` 並設定 `analytics_session` cookie。原教學基底 `rust_ai_chat/main=0cd9f7995c196a7359864a9395c0fe2372d09222`，目前來源 `rust_ai_chat/main=c92831e105fb0000860f995ea288e29631b5fed8`（唯讀），教學分支在本案寫入前已推進至 Pet Day20 狀態；規則 `daily-tdd-minimum-v1`。Red 5分鐘、Green 10分鐘；Refactor、額外驗證另計。
**契約界線**：現行主線仍是全域共享 10 次／60 秒計數，不是產品規格要求的帳號級無上限倍增等待、成功清零與跨重啟持久化；此篇只教 HTTP 429 的等待提示，不冒稱完整登入保護。
### 2026-10-19｜r2 真實程式與測試核對
現行 `c92831e105fb0000860f995ea288e29631b5fed8` 的 `apps/stats_server/src/admin.rs` blob `fac1101f1aebe8d6f1c8b2aae8d0d5173392e667` 已於 `async fn login` 中在判斷 `attempts.1 >= 10` 後計算 `Duration::from_secs(60).saturating_sub(attempts.0.elapsed())`，使用向上取整的剩餘秒數設定 `header::RETRY_AFTER`，再回傳 429。模組中已有 `day20_login_rate_limit_provides_retry_after`，並與前日的登入 Session／Cookie 測試同在既有 tests 模組。`Login` 入口為現有 POST `/login`，沿用 Origin、`Admin.config`、共享 attempts 互斥鎖及回應契約；Day20 的 helper／修改已**合入 main**。原文 Red 測試的「429 缺少標頭」僅適用舊 SHA，**在現行 main 已不會重現**；不可再直接貼入同名測試。
[新 SHA Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39202728570) 的 `test-stats-server`（job `117898950939`）成功；[Rust Dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39202728399) 的 `check-dashboard`、`build-dashboard` 均成功。這些屬已提交程式的 CI 驗證，而不是人工實測 60 秒真時鐘、併發、完整帳號級限流或真 TCP／PostgreSQL 整合。尤其目前仍採**全域 10 次／60 秒**，且成功登入不清零先前失敗次數；不能當作使用者要求的帳號級倍增等待政策已實作。
## 2. 實作內容
### Red
在 `apps/stats_server/src/admin.rs` 已存在 `#[cfg(test)] mod tests` 中新增以下測試（沿用該模組已存在的 `app()`／`request()`、`tower::ServiceExt`、`header` 與 `StatusCode`；不新建測試框架）：
```rust
#[tokio::test]
async fn day20_login_rate_limit_provides_retry_after() {
    let router = app();
    for _ in 0..10 {
        let response = router.clone()
            .oneshot(request("POST", "/login", "key=wrong"))
            .await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    let response = router
        .oneshot(request("POST", "/login", "key=wrong"))
        .await.unwrap();
    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    let seconds: u64 = response.headers()[header::RETRY_AFTER]
        .to_str().unwrap().parse().unwrap();
    assert!((1..=60).contains(&seconds));
    assert!(!response.headers().contains_key(header::SET_COOKIE));
}
```
預期在目前 main 跑測試時會因為 **429 缺少 RETRY_AFTER 標頭**而 panic（HeaderMap Index 缺少對應鍵），這是同功能的預期行為 Red；**本輪尚未執行**，不能宣稱觀察到實際測試失敗。
### 完整最小 Green
只需修改 `apps/stats_server/src/admin.rs` 的既有 `login` 函式，不改登入成功分支。將以下原碼：
```rust
if attempts.1 >= 10 {
    return StatusCode::TOO_MANY_REQUESTS.into_response();
}
```
替換為完整處理：
```rust
if attempts.1 >= 10 {
    // 窗口已在本函式上方檢查是否超過 60 秒並重置。
    // 將剩餘時間向上取整，避免回傳 0 秒但仍拒絕登入。
    let remaining = Duration::from_secs(60)
        .saturating_sub(attempts.0.elapsed());
    let mut seconds = remaining.as_secs();
    if remaining.subsec_nanos() > 0 {
        seconds += 1;
    }
    return (
        StatusCode::TOO_MANY_REQUESTS,
        [(header::RETRY_AFTER, seconds.max(1).to_string())],
    ).into_response();
}
```
保留本函式上方 `if attempts.0.elapsed() >= Duration::from_secs(60) { *attempts = (Instant::now(),0); }`，使新的 429 Retry-After 只會在目前窗口仍生效時發出；若窗口到期則重置計數、不應再回 429。既有 Origin 錯誤與設定錯誤仍分別回 403／503，登入碼格式／雜湊錯誤仍為 401 並計次，登入成功仍是 303 並發 session Cookie。這是既有 HTTP 路由中的完整最小 Green，不是未接線的輔助函式。
### Refactor／必要補教
可在後續將計數與窗口操作封裝成 `LoginAttemptLimiter` 並改為每帳號持久化；不能藉此篇宣稱完成規格中的 15、30、60、120 秒倍增或無上限等待。現有 `day19_success_does_not_reset_previous_failures` 測試代表當前既有全域計數行為，這篇不改變它。安全上 `Retry-After` 是提示，不可當唯一限流防線，實際拒絕仍由伺服器做；共享全域上限可能讓一個攻擊者影響所有管理員，應在後續完整政策修正。
## 3. 套件版本
原教學基底工作區版號 `0.2.19`；現行主線工作區版號 `0.2.20`、edition 2024；`axum=0.8`、`tokio=1.53`、`serde=1.0`、`sha2=0.11` 為 workspace 宣告版本，精確 patch 由來源 SHA 的 `Cargo.lock` 決定。使用 axum 標準 `IntoResponse` 與 `HeaderMap`，無新增 crate、migration、設定或外部服務；CI 的 Linux Rust toolchain 不能替代本地 MSRV/跨平台驗證。
## 4. 驗證項目
```bash
cargo test --locked -p stats-server day20
cargo test --locked -p stats-server
cargo check --locked -p stats-server
```
**預期**：429 有整數 `Retry-After` 值介於 1..=60，沒有 Set-Cookie；401 仍計次、403 不計次、成功 303 不消耗額度，原 Day19 測試仍通過。**本次未在本地重新執行以上命令**，但 Day20 改動及測試已存在於 `c92831e105fb0000860f995ea288e29631b5fed8`；[最新 Test All/test-stats-server](https://github.com/williechen/rust_ai_chat/actions/runs/39202728570) success（job `117898950939`），[Rust Dashboard check/build](https://github.com/williechen/rust_ai_chat/actions/runs/39202728399) success。舊 `0cd9f799…` 的 CI 為歷史結果，不能用來聲稱新版測試成功。真 HTTP、跨窗口 60 秒與併發壓力未驗證；真 PostgreSQL、完整帳號策略、使用者主線部署與產品整合亦未驗收。Day20 程式已套用 `rust_ai_chat/main`（本次對來源唯讀）；使用者環境部署、產品整合、完整登入保護策略與產品功能驗收仍未完成。
