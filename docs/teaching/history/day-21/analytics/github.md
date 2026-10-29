## 1. 學習大綱
Analytics P0 Day 21：將「管理員帳號」欄位從登入頁連到現有 Axum POST /login 的表單 DTO 與檢查器。這是正式帳密登入的第一個最小前置切片，**仍使用舊 64 hex 登入碼，絕不冒稱帳密登入完成**。後續繼續導入持久化帳號、加鹽密碼雜湊、首次強制改密碼、Session 與授權。
來源 rust_ai_chat/main = `5654591f3002fc619c883af9b5197e2126cc6494`（唯讀）。真實入口：`apps/stats_server/src/admin.rs::login_page` → GET /login → `<form action="/login">` → `Form<Login>` → `login` → Admin.config/attempts/session。現行 DTO 只有 `key: String`，Config 使用 `ANALYTICS_ADMIN_KEY_SHA256`，後台非完整帳密系統。工作區版本 0.2.20。
## 2. 實作內容
### Red
在 [admin.rs](http://admin.rs) 既有 `#[cfg(test)] mod tests` 加入：
```rust
#[test]
fn day21_admin_account_is_explicit() {
    assert!(super::admin_account_name_is_valid("admin"));
    assert!(!super::admin_account_name_is_valid("guest"));
    assert!(!super::admin_account_name_is_valid("ADMIN"));
}
```
加入 helper 前預期 E0425（尚未定義）；這是推導的 Red，**未在本輪執行**。這項測試驗證帳號輸入契約，不等於密碼驗證或 PostgreSQL 資料持久化。
### 完整最小 Green
1. 在 `apps/stats_server/src/admin.rs` 加入：
```rust
fn admin_account_name_is_valid(value: &str) -> bool {
    value == "admin"
}

fn default_legacy_admin_name() -> String {
    "admin".into()
}
```
1. 將原本 `#[derive(Deserialize)] struct Login { key: String }` 改為：
```rust
#[derive(Deserialize)]
struct Login {
    #[serde(default = "default_legacy_admin_name")]
    username: String,
    key: String,
}
```
相容預設僅為保留既有 `key=wrong` HTTP 測試，不屬最終登入設計。3. 在 `login_page()` 既有 POST form 內、登入碼輸入前加入：
```rust
<label>"管理員帳號"
    <input name="username" type="text"
        value="admin" required autocomplete="username" />
</label>
```
1. 在 `async fn login` 驗證 Origin 與 `Some(config)` 成功之後、嘗試 `attempts.lock()` 之前加入：
```rust
if !admin_account_name_is_valid(&form.username) {
    return StatusCode::UNAUTHORIZED.into_response();
}
```
因此表單確實傳送 username；錯帳號被服務端拒絕，舊的 64 hex key 與 Session 邏輯不變。這是**過渡入口**，不是正式帳號密碼驗證。繞過 `username` 的舊客戶端仍因相容預設使用 admin，必須於正式切換帳密時移除，不能保留安全側門。
### Refactor／下一步
下一個必需功能為 PostgreSQL 持久化管理員帳號，以及以 Argon2id 等適合密碼儲存的演算法搭配隨機 salt；現有單次 SHA-256 key hash 不可當作密碼雜湊。再接強制首次改密碼、真正 username/password 與 session/CSRF。須補真 HTTP 401/303 測試、migration／資料保留驗證。保留原 Day19/20 測試語意，正式切換後才移除 legacy key 模式。
## 3. 套件版本
`Cargo.toml` workspace 0.2.20 / edition 2024；axum 0.8、leptos 0.8、serde 1.0、tokio 1.53（workspace 宣告）。本切片無新增 crate 或 migration，Cargo.lock 精確 patch 未在本輪重新測試。
## 4. 驗證項目
```bash
cargo test --locked -p stats-server day21
cargo test --locked -p stats-server
cargo check --locked -p stats-server
```
預期：Day21 單元測試通過、GET /login 有 username 欄位，POST username=guest + 合法 key 回 401，舊 key-only fixture 保持既有回應；新增 HTTP 驗證案例仍需實際寫入。**實際：沒有修改 rust_ai_chat/main，沒有執行新測試、真 HTTP 或 DB 驗收。** 本日編寫 Red/Green 為建議 patch，非已執行測試成果。來源 SHA `5654591f3002fc619c883af9b5197e2126cc6494`，舊版 CI success 不涵蓋本篇新增變更。管理員 DB、加鹽密碼、首次改密碼、權限／產品整合、部署及完整驗收均未完成。
