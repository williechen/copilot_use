## 1. 學習大綱
**P0 Pet Day 21：讓控制台開始真正的「新增第一隻寵物」操作。** 這一天先接通使用者輸入寵物名稱 → 前端 Tauri IPC → 原生端真實驗證 → 回應驗證後名稱，為後續持久化建立唯一寵物身分的前置切片；**尚未建立資料庫紀錄、沒有新生活窗，不可宣稱已完成建立寵物**。
基底來源 `rust_ai_chat/main=5654591f3002fc619c883af9b5197e2126cc6494`（唯讀）； `apps/desktop_pet/src/lib.rs::application` 目前 `.manage(Mutex::new(PetMachine::default()))`，`generate_handler!` 只有狀態／動作／聊天／掃描／開控制窗等指令，沒有新增寵物命令；`apps/desktop_pet/frontend/src/main.rs::App` 在非 pet 視窗顯示控制台，沒有建立寵物表單；`bridge.rs` 封裝 JS invoke。這個完整切片是**操作表單+IPC 契約**，後續再補 id、SQLite 交易與重啟讀回。使用者明確指出不能跳過建立寵物而先做漫遊。
## 2. 實作內容
### Red：先驗證寵物名稱契約
在 `apps/desktop_pet/src/lib.rs` 的既有 `#[cfg(test)] mod roam_setting_tests` 之外，新增同層測試模組：
```rust
#[cfg(test)]
mod day21_pet_creation_tests {
    use super::normalize_new_pet_name;
    #[test]
    fn day21_accepts_trimmed_chinese_name() {
        assert_eq!(normalize_new_pet_name("  小橘  "), Ok("小橘".into()));
    }
    #[test]
    fn day21_rejects_blank_and_long_names() {
        assert!(normalize_new_pet_name("  ").is_err());
        assert!(normalize_new_pet_name(&"寵".repeat(33)).is_err());
    }
}
```
尚無 helper 時預期編譯錯誤 E0432（未解析 import），是**提案的預期 Red，未實測**。驗證不應只靠前端 required，原生端必須防止繞過 IPC 送非法值。
### 完整最小 Green：建立可呼叫的「驗證新增資料」流程
一、在 `apps/desktop_pet/src/lib.rs` 加入：
```rust
fn normalize_new_pet_name(input: &str) -> Result<String, String> {
    let name = input.trim();
    if name.is_empty() || name.chars().count() > 32 {
        return Err("寵物名稱須為 1 到 32 字元".into());
    }
    Ok(name.to_owned())
}

#[tauri::command]
fn validate_new_pet_name(name: String) -> Result<String, String> {
    normalize_new_pet_name(&name)
}
```
在 `tauri::generate_handler![` 現有清單最後增加 `validate_new_pet_name,`。此命令只回傳經驗證名稱；沒有悄悄把 `PetMachine::default()` 當新寵物，也不寫 DB。
二、在 `apps/desktop_pet/frontend/src/bridge.rs`，沿用檔案既有 `#[wasm_bindgen]` JS extern 區塊，新增：
```rust
#[wasm_bindgen(js_namespace = window, js_name = validateNewPetName)]
fn validate_new_pet_name_js(name: &str) -> Promise;
```
並在 bridge 外層包裝（延續此檔既有 `js_error`）：
```rust
pub async fn validate_new_pet_name(name: String) -> Result<String, String> {
    let value = JsFuture::from(validate_new_pet_name_js(&name))
        .await
        .map_err(js_error)?;
    value.as_string().ok_or_else(|| "名稱回應格式錯誤".into())
}
```
三、來源已確認 JS 橋接實作檔案是 `apps/desktop_pet/frontend/index.html`，目前透過 `window.__TAURI__.core.invoke` 暴露 `window.getPetState` 等函式。在既有 `<script>` 中，沿用相同模式直接新增：
```javascript
window.validateNewPetName = (name) => {
  return invoke("validate_new_pet_name", { name });
};
```
`bridge.rs` 上述 extern 使用 `js_namespace = window`、回傳 `Promise`，再由 `JsFuture::from` 包裝，符合既有 bridge 真正運作方式，不需要猜測新 JS module。四、在 `apps/desktop_pet/frontend/src/main.rs` 的 `App` 非 `is_pet_window()` 控制台分支新增輸入和驗證按鈕，狀態獨立於檔案掃描：
```rust
let (new_pet_name, set_new_pet_name) = signal(String::new());
let (new_pet_feedback, set_new_pet_feedback) = signal(String::new());
// 插入 view! 的控制台 <main class="control-shell"> 內：
<section>
    <h2>"新增寵物"</h2>
    <label>"寵物名稱"
        <input prop:value=move || new_pet_name.get()
            on:input=move |ev| set_new_pet_name.set(event_target_value(&ev)) />
    </label>
    <button type="button" on:click=move |_| {
        let proposed = new_pet_name.get_untracked();
        spawn_local(async move {
            let response = bridge::validate_new_pet_name(proposed).await;
            set_new_pet_feedback.set(match response {
                Ok(name) => format!("名稱「{name}」有效；尚未儲存寵物"),
                Err(error) => error,
            });
        });
    }>"檢查名稱"</button>
    <p role="status">{move || new_pet_feedback.get()}</p>
</section>
```
上方為需要插入現有 `App` 的完整互動片段；**若未先確認現有 JS extern import module 與其匯出實作，不得宣稱 Green 可執行**，必須先接通真正 JS → Rust invoke。不能把按鈕標「建立完成」。
### Refactor 與下一個必要環節
下一步先設計新寵物 `id`、name、初始 appearance 的資料模型與 SQLite schema／前向 migration，確保 unique ID、原子新增、重啟載入，然後把上面的「檢查名稱」改成真正 `create_pet` 的保存動作，並在生活窗綁定該 ID。不得覆寫已有寵物、不得把 default singleton 與多寵物資料混淆。安全上注意本日來源 `apps/desktop_pet/migrations/20261018091733_create_pet.sql` 在最新 SHA **有修改**，必須先檢查已套用 migration checksum 與升級相容性，不可直接重建／清空使用者 SQLite。
## 3. 套件版本
最新 Cargo 工作區版號 `0.2.20`、edition 2024；使用現有 Tauri 2.x、Leptos 0.8、tokio 1.53，沒有新增依賴或新增 migration。本切片未實測 Rust、OS/WM、WASM/原生 SDK。
## 4. 驗證項目
```bash
cargo test --locked -p desktop-pet day21
cargo test --locked -p desktop-pet
cargo check --locked -p desktop-pet
cargo check --locked -p desktop-pet-frontend --target wasm32-unknown-unknown
```
預期：新增名稱校驗兩項單元測試成功，從控制台可輸入中文名稱並經真 Tauri IPC 回傳結果；空白／過長顯示錯誤而非新增。**實際：本輪未在 rust_ai_chat/main 套用程式，未執行新 Day21 測試或本地 GUI/IPC。已核實 ****`frontend/index.html`**** 的 window JS bridge 與 ****`bridge.rs`**** 的 Promise/JsFuture 模式，本篇提供三層完整變更；仍需實際套用與編譯，不能視作 Green 已實測通過。** 已有 Day20 的 CI 成功不代表此變更通過。SQLite create/save/reload、寵物外觀、生活窗身份、多寵物及產品驗收均未完成，優先繼續本功能。
