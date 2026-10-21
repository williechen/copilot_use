## 1. 學習大綱與現況

Day25 Pet 原教學：<i>讓動態的 pet- 生活窗真正具有 Tauri 事件監聽權限</i>\*，把既有後端 saved-pet-state 變更送到可操作的生活畫面，不重做 SQLite PetStore 或多寵物建立。來源 rust\_ai\_chat/main 08b891a22c1934a2bf0b1edbb103c9b16bcf12de。現有 apps/desktop\_pet/src/pet\_[windows.rs](http://windows.rs) 的 label 為 pet-\{UUID.simple()\}，且 dispatch\_and\_emit 已廣播 saved-pet-state；frontend/index.html 已呼叫 window.<b>TAURI</b>.event.listen 並比對 pet\_id；但 capabilities/default.json 的 windows 只有 main、pet，實際建立的 pet-xxxxxxxx 不匹配。Day24 原生 UI 證據顯示事件監聽被拒、睡眠畫面未可靠更新。本日選最早缺少的權限契約，範圍是「允許這一類生活窗 event.listen」，並不因此宣稱畫面完整驗收。

## 2. Red → 最小完整 Green → Refactor + FIFO 舊 Day 補教

Red：檔案 apps/desktop\_pet/src/[lib.rs](http://lib.rs) 最末追加這個獨立測試模組。它檢查目前真正供 Tauri 讀取的 capability JSON 的 windows、permissions；不僅測 helper 或字串，且拒絕把所有視窗設成 "<i>" 的過大權限。原本 \["main","pet"\] 不含 "pet-</i>"，第一項應失敗。本輪未執行實際 Red。

```rs
#[cfg(test)]
mod day25_capability_tests {
    #[test]
    fn dynamic_pet_windows_can_listen_without_global_window_wildcard() {
        let capability: serde_json::Value =
            serde_json::from_str(include_str!("../capabilities/default.json")).unwrap();
        let windows = capability["windows"].as_array().unwrap();
        assert!(windows.iter().any(|v| v.as_str() == Some("main")));
        assert!(windows.iter().any(|v| v.as_str() == Some("pet-*")));
        assert!(!windows.iter().any(|v| v.as_str() == Some("*")));
        let permissions = capability["permissions"].as_array().unwrap();
        assert!(permissions.iter().any(|v| v.as_str() == Some("core:event:default")));
    }
}
```

完整最小 Green：在 apps/desktop\_pet/capabilities/default.json 中，把既有內容<b>完整替換</b>為：

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Main controller and dynamically-created pet life windows",
  "windows": ["main", "pet-*"],
  "permissions": [
    "core:default",
    "core:window:allow-start-dragging",
    "core:event:default"
  ]
}
```

注意事項：Tauri v2 的 [capability.windows](http://capability.windows) 支援 glob labels；pet-\* 僅匹配動態寵物視窗，不應用 \["\*"\] 擴大到其他視窗。原有 "core:default" 對應目前程式的預設權限集合；新增的 core\:event\:default 為生活窗事件 API 授權，不授予檔案系統或 shell。實際是否在目前精確 Tauri build 中生效，必須以 Tauri GUI/WebDriver 測試證明。不要在測試成功之前稱為已接通。

真實資料流：控制台先建立 SQLite 寵物，Tauri open\_saved\_pet(id) → 開 pet-\<simple uuid> Webview → frontend/index.html 取得 pet\_id → listen("saved-pet-state") → 後端 dispatch\_and\_emit 發送 \{id,snapshot\} → 只將 matching id 的 snapshot 更新到正確生活窗。現有 JS / WASM listener、後端 FSM 由產品既有程式提供，不新增第二套事件匯流排；若窗口仍無更新，檢查 payload、訂閱時機與 Webview 標籤，不能只依 capability 單元測試驗收。

Refactor：等有 GUI 證據，再考慮將預設 capability 分成控制台與生活窗各自最小權限；目前先保持同檔、僅限 main/pet-\*。未解決：生活窗 CSS 200px 裁切、動畫多符號圖像、舊 SQLite migration checksum 相容及重啟畫面驗收。

<b>FIFO：Day24 舊日更正補教（新 Red/Green/Refactor 之後）</b>。來源 docs/teaching/pending/day25/[pet-day24-corrections.md](http://pet-day24-corrections.md) 與 2026-10-21 本地報告：main 的 frontend/src/[main.rs](http://main.rs) 在生活窗分支中有兩處 PetStatus，導致每窗兩個 .pet-face。這不是剛才事件權限的 Green。為修正 Day24 套用差異，把生命窗分支中無條件的一行

```rs
<PetStatus snapshot blinking animation_phase />
```

刪除，<b>保留</b>下面 Show when=move \|\| active\_pet.get().is\_some() 包住的那一個 PetStatus。新的驗證必須檢查每個 pet-\* Webview 在載入資料後 .pet-face 計數恰為 1，且 display 維持同一個 pet id；在資料未載入時 0 是預期狀態。動畫內容另含 🐢💤/🐢z，不等於視覺只有一個圖像；每幀單一寵物圖像是既定需求，必須修正動畫元件並驗證各動畫狀態；本篇尚未提供此修復。已知舊庫 VersionMismatch(20261018095626)，此日不碰原 migration、不刪庫。本段僅教學補正，<b>未套用</b>，也不得回寫 Day24 封存文章。補教是否標 completed，應以本篇全文讀回確認及 repo 狀態同步決定。

## 3. 套件版本與相容性

來源 workspace version 0.2.23、edition 2024；apps/desktop\_pet/Cargo.toml 的 tauri 2.12、tauri-build 2.7、serde\_json.workspace、tokio.workspace、sqlx.workspace；frontend Leptos 0.8。這是宣告版號，不冒稱 Cargo.lock 實際解析及 OS GUI 的完整跨平台驗收。Tauri 官方 Capability config 支援 windows glob patterns，應對照專案產生 schema 與實際視窗 label。參考：[https://v2.tauri.app/reference/config/](https://v2.tauri.app/reference/config/) （Capability windows）。

## 4. 指令／預期／實際／CI 證據

```bash
cargo test --locked -p desktop-pet dynamic_pet_windows_can_listen_without_global_window_wildcard -- --nocapture
cargo test --locked -p desktop-pet
cargo check --locked -p desktop-pet
cargo fmt --all -- --check
```

預期 Red：未更新 JSON 時缺 pet-\* 且 core\:event\:default，不通過；Green：JSON 驗證通過；<b>這個測試本身不會驗證 Tauri 真正允許 event.listen</b>。須再用 Tauri/WebKit GUI 打開三個 pet-\* 視窗，送 Sleep/Wake，檢查各窗沒有 "event.listen not allowed"、各狀態即時更新且無跨 id 污染；同時讀取 .pet-face 計數和裁切。此為實際產品驗收門檻。

<b>Day25 新 Red/Green 沒有在本輪執行；未套用至 rust\_ai\_chat/main；沒有新的 CI/job 或 GUI 成功證據。</b> 同 SHA 既有 [Rust Desktop Pet](https://github.com/williechen/rust_ai_chat/actions/runs/39595880622) completed/success、[Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39595880206) completed/success，並未包含上述修改。[Day24 本地驗證](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-21-day24-main-08b891a/REPORT.md) 保存了真正的 Tauri GUI 失敗證據：事件授權被拒、每窗兩個 face；後端狀態測試成功不能替代視覺驗收。完成教學發布不代表實際功能已驗收。

### Day25 需求覆核更正

Pet 必須支援兩隻以上：至少以三個不同 pet id 的生活窗驗證新增、重開聚焦、各自狀態與關閉互不影響，不可把數量硬編碼為二。每窗單一 PetStatus 與動畫每幀單一寵物圖像是兩個獨立條件；睡眠、清醒及切換動畫都必須只呈現一個寵物圖像，不能用 🐢💤／🐢z 的複合字元充當已修復動畫。

能力 JSON 的字串斷言只驗證設定內容，不能證明原生事件監聽已授權。需實跑三窗事件更新、每窗 .pet-face 計數、動畫各幀與裁切檢查。這些是既定驗收要求，尚未完成。

目前僅文章發布及全文讀回完成，補教文字完成不等於動畫或 GUI 修復完成。Red／Green 與功能驗收未完成；Day26 不代表驗收通過，不得據此繼續遞增或恢復排程。Day24 及以前保持封存。