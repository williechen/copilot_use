# Chat Day35 r1｜建房名稱規則共用：拒絕控制字元且兩種入口一致

## ① 學習大綱：功能、來源與前置條件

交付日期：2026-10-25（台北）；唯一產品來源：williechen/rust_ai_ring/main@891ff32ae29e1588dc40027cd5c9c99ca91d2378，唯讀；文件發布於 williechen/copilot_use/main。舊稱 rust_ai_chat 只供歷史追溯。此篇從即時 next_day=35 銜接 [Chat Day34 r2](../day-34/chat-r2.md)，不是重做已套用的 SignInPage／GET /login 去重。

**當日唯一切片：** 已登入會員在個人管理頁建立房間，或經授權的 JSON POST /api/rooms 建立房間，均採同一個 Application 驗證規則：非全空白、最多 200 UTF-8 bytes、不得含任何 Unicode control character（包含 NUL、換行、Tab、DEL）；合法名稱保持原字串，不自動 trim 或更改大小寫。過去兩處都只拒絕 NUL，會接受「名稱內含換行」；此篇以新 Red 保護畫面／JSON 入口一致性。個人頁原 422 與 JSON 原 400、CSRF、Session、201、SQLite transaction/insert 契約保持原狀。

真實呼叫鏈 A：Leptos 渲染的 chat_ui::account::AccountView 表單 → POST /account/rooms → chat_server::signin::account_create_room（active_member、Origin、CSRF）→ chat_application::validate_room_name → chat_application::create_room → chat_persistence::SqliteRoomRepository → chat_apps/migrations/rooms SQLite → AccountView 回傳已建立房間連結。鏈 B：JSON POST /api/rooms → chat_server::room_auth::require_member → chat_server::http::create_room → **同一** validate_room_name → **同一** create_room／SQLite Adapter。公開 GET /api/rooms、RoomList 搜尋及現有 Leptos SSR/hydration 保持不變。

本日僅修改 chat_application/src/lib.rs、chat_server/src/http.rs、chat_server/src/signin.rs；不修改 UI、schema、migration、WS 或 Cargo dependency。四 workspace 與 package/lock 均依來源核對：chat_apps、pet_apps、analytics_apps 的 workspace version 0.2.34；crates 為 0.2.32。chat-server / chat-ui / chat-persistence 是 Cargo package；Rust imports 為 chat_server / chat_ui / chat_persistence。Chat 的 Leptos metadata bin-package=chat-server、lib-package=chat-ui、ssr/hydrate 保留；資源輸出名 rust_ai_chat 是現存 metadata，不是現行 repo 名稱。

## ② Red → 完整最小 Green → Refactor

### Red：先把新規則與真實 HTTP／表單呼叫者測到失敗

在 chat_apps/chat_application/src/lib.rs **檔尾**加入（暫時不新增 validate_room_name，預期 E0432／找不到符號，這是預期編譯 Red，不是環境錯誤）：

~~~rust
#[cfg(test)]
mod day35_room_name_tests {
    use super::validate_room_name;

    #[test]
    fn room_name_policy_is_utf8_bytes_and_control_safe() {
        for ok in ["中文房間", " 房間 ", &"a".repeat(200)] {
            assert_eq!(validate_room_name(ok), Ok(()));
        }
        for bad in ["", "  ", "房\n間", "房\t間", "房\r間", "房\0間", "\u{7f}", &"房".repeat(67)] {
            assert!(validate_room_name(bad).is_err(), "{bad:?}");
        }
    }
}
~~~

在 chat_apps/chat_server/src/http.rs 既有 room_http_tests 內加上完整測試（沿用該模組已存在的 fixture、State、Json、CreateRoomRequest、IntoResponse）：

~~~rust
#[tokio::test]
async fn day35_json_create_rejects_control_names_before_sqlite() {
    let (state, pool) = fixture().await;
    for name in ["房\n間", "房\t間", "房\r間", "\u{7f}"] {
        let response = create_room(
            State(state.clone()), Json(CreateRoomRequest { name: name.into() })
        ).await.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM rooms")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(count, 0);
    pool.close().await;
}
~~~

在 chat_apps/chat_server/src/signin.rs 檔尾加入下列**真實帳號／Session／CSRF／表單 handler** 測試，不能以未登入的 401 冒充名稱驗證：

~~~rust
#[cfg(test)]
mod day35_account_room_tests {
    use super::*;
    use axum::http::header;

    #[tokio::test]
    async fn logged_in_account_rejects_newline_without_room_write() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1).connect("sqlite::memory:").await.unwrap();
        chat_persistence::migrate(&pool).await.unwrap();
        sqlx::query("INSERT INTO mbr_user (id,user_acnt,user_pd,user_email,user_name,user_status,is_init_pd,created_at,updated_at) VALUES ('day35','learner35','unused','day35@example.test','Learner','active',FALSE,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
            .execute(&pool).await.unwrap();
        let raw = "b".repeat(64);
        sqlx::query("INSERT INTO mbr_session(id,mbr_user_id,token_hash,expires_at) VALUES ('s35','day35',?1,unixepoch()+60)")
            .bind(shared::auth::token_digest(&raw))
            .execute(&pool).await.unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, format!("chat_session={raw}").parse().unwrap());
        headers.insert(header::ORIGIN, "http://127.0.0.1:3000".parse().unwrap());
        let csrf = crate::auth_session::csrf_for_create_room(&headers).unwrap();
        let response = account_create_room(
            State(pool.clone()), headers,
            Form(AccountRoomForm { name: "房\n間".into(), csrf }),
        ).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM rooms")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count, 0);
        pool.close().await;
    }
}
~~~

目前原碼對內嵌換行會走 create_room 而寫入，JSON 預期 400 卻得到 201，個人表單預期 422 卻得到 201。以上只是原碼對照後的**預期 Red**；未套用／未實跑，不虛構失敗紀錄。DB 連線、缺 migration 或缺環境是另一類錯誤，應分開排查。

### Green：完整新增函式、兩個 handler 的唯一必要替換

在 chat_apps/chat_application/src/lib.rs 的 CreateRoom 結構前新增以下完整純函式，保留原 create_room 與 RoomRepository，不改資料庫：

~~~rust
/// 表單與 JSON 建房共用；保留合法輸入原字串。
pub fn validate_room_name(name: &str) -> Result<(), &'static str> {
    if name.trim().is_empty() {
        return Err("聊天室名稱不得為空白");
    }
    if name.len() > 200 {
        return Err("聊天室名稱最多 200 UTF-8 bytes");
    }
    if name.chars().any(char::is_control) {
        return Err("聊天室名稱不得含控制字元");
    }
    Ok(())
}
~~~

chat_apps/chat_server/src/http.rs::create_room 的原驗證 if 區塊**整段**替換為：

~~~rust
if chat_application::validate_room_name(&request.name).is_err() {
    return Err(StatusCode::BAD_REQUEST);
}
~~~

chat_apps/chat_server/src/signin.rs::account_create_room 原本第 148–159 行的驗證 if **整段**替換為：

~~~rust
if let Err(message) = chat_application::validate_room_name(&form.name) {
    return Ok((
        StatusCode::UNPROCESSABLE_ENTITY,
        account_html(&headers, message.into(), form.name, None)?,
    ).into_response());
}
~~~

其餘檔案完整保留：signin.rs 原先 active_member／Origin／CSRF 必須先於驗證；HTTP JSON 的 require_member middleware 仍在 chat_server/src/lib.rs 註冊；成功仍呼叫 chat_application::create_room（UUID、default-category、SQLite adapter）；失敗不得先 INSERT。AccountView 使用既有 Leptos 表單與跳脫輸出，無需第二份 UI。既有 chat_application、chat_server Cargo.toml 已接通依賴，Cargo.lock、Cargo features、Rust imports、Leptos metadata 全部**無新增／修改**；不對 copilot_use 建立產品程式。

### Refactor 與上一篇／舊 Day 更正

Refactor 僅移除兩處重複 if，將規則集中在 Application；Transport 各自保留 400 JSON／422 HTML 與 CSRF。合法的 200-byte ASCII 和中文字（總 byte 長度不超過 200）保留原樣。若產品後續要改為 Unicode 字素長度、房名唯一性或分類 ACL，必須另訂需求與 migration，不能在本篇暗改。

Day34 r2 的 GET /login redirect／單一 SignInPage 已在本次來源，無需重貼。Day20 的 refresh_result_view 為 RoomList 真實 caller，Day19 的 refreshed_room_list 只在舊測試使用；先遷有效斷言後再清理，不因本篇順手刪掉。Day25 共用登入／Session FIFO 仍 pending，不能用建房名稱測試結案。歷史文章及原 commit/blob/SHA256 保留，不倒退 Day。

## ③ 套件與軟體版本（2026-10-25 官方穩定版核對）

| 項目 | rust_ai_ring manifest → Cargo.lock | 交付當日官方最新穩定版／依據 |
| --- | --- | --- |
| Rust / Cargo | rust-toolchain.toml=stable；實際安裝未量測 | [Rust 1.99.1（10/15）](https://blog.rust-lang.org/releases/) |
| Axum | 0.8 + ws → 0.8.9 | [0.8.9](https://docs.rs/crate/axum/latest) |
| SQLx（SQLite） | 0.9 runtime-tokio/macros/sqlite → 0.9.0 | [0.9.0](https://docs.rs/crate/sqlx/latest) |
| Leptos / Router | 0.8 → 0.8.22 / 0.8.17 | [Leptos](https://docs.rs/crate/leptos/latest)／[Router](https://docs.rs/crate/leptos_router/latest) |
| Tokio / UUID | 1.54/full → 1.54.1；1.29/v4 → 1.29.0 | [Tokio](https://docs.rs/crate/tokio/latest)／[UUID](https://docs.rs/crate/uuid/latest) |
| Serde | 1.0/derive → 1.0.229 | [Serde](https://docs.rs/crate/serde/latest) |
| SQLite engine | 由 SQLx 實際 runtime 決定，未量測 | [3.54.1（10/16）](https://www.sqlite.org/download.html) |

來源四份 Cargo.lock blob：chat 8fe9db93e078e8633f283834fd01e6805d97f9b3、pet b38668fac8b3d8cd105d54d3baf9e3a8f0b3e1a5、analytics 9f6a1720ec999263d162a1e8c90c0fbc1b3dd6a7、crates 5321bfdaaaa24bb796c40092e39a4aa8a27f2d8f；以來源 SHA 891ff32 固定。上述為最新穩定版核對而非安裝／升級聲明；0.9 RC／nightly 排除。edition 2024 至少 Rust 1.85；建議 1.99.1，其他依賴 MSRV 矩陣未實跑。SQLite 的系統版本不等於 SQLx 編譯綁定版本；以 SELECT sqlite_version() 實測，不自動修改 lock。

## ④ 驗證步驟、預期／實際、缺口

**工作目錄：rust_ai_ring/chat_apps**，只在使用者的產品庫套用：

~~~bash
cd chat_apps
cargo test --locked -p chat-application day35_room_name_tests
cargo test --locked -p chat-server --features ssr day35_json_create_rejects_control_names_before_sqlite
cargo test --locked -p chat-server --features ssr day35_account_room_tests
cargo test --locked -p chat-server --features ssr
cargo test --locked -p chat-ui --features ssr
cargo check --locked -p chat-ui --features hydrate --target wasm32-unknown-unknown
cargo fmt --all -- --check
~~~

Red 預期編譯失敗／既有建房接受控制字元；Green 預期 200-byte 與中文合法、非法 400/422、DB 0 寫入，既有建房成功及 Session/CSRF/JSON 回歸不變。另在隔離 SQLite／真瀏覽器核對：未登入建房 401；登入個人頁輸入合法中文→201 及房間連結；非法換行（直接 POST）→422 且不寫；JSON 非法→400；合法 JSON→201；重新整理公開房間列表可讀，既有房間 ID 不變。UI maxlength 是字元數，伺服器依 bytes 驗證；中文過長需顯示既有錯誤。

**實際：** 本篇只在 copilot_use 發布教學；rust_ai_ring 未修改，Red／Green、SQLite、HTTP、SSR、WASM、瀏覽器與 CI 均未在本次執行，Gate 2 未驗收。上一篇的歷史 CI 只屬其原 SHA，不推論此新切片。後續仍有選房登入導向、逐房 ACL、作者／歷史、共用認證 FIFO、RoomList 舊 helper/測試清理。交付讀回、精確 commit/blob/SHA256 與首頁/ENTRY/進度另存文件證據；不更新 last_full_comparison。
