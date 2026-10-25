# Pet 漫遊與 WebKit 錯誤回報（2026-10-25）

本次保存使用者貼上的錯誤訊息，並於最新 rust_ai_ring/main `744a75fccf79fc176ee145182891742845df3296` 靜態核對；產品唯讀，未執行 GUI、未改環境、未重啟程序。使用者當次 binary／WASM SHA、啟動方式與 WebKitGTK 版本尚未提供，不能以本次原始碼版本代替執行版本。

## 原始回報

```text
desktop roaming unavailable: pet window not found
ERROR: WebKit encountered an internal error. This is a WebKit bug.
Source/WebKit/WebProcess/Network/WebLoaderStrategy.cpp(640) : void WebKit::WebLoaderStrategy::internallyFailedLoadTimerFired()
```

使用者貼文中的 WebKit 訊息重複且交錯；此處保留單組內容，不據此推定失敗視窗數或程序數。

## 已確認：漫遊視窗識別與生命週期缺口

- [roam.rs](https://github.com/williechen/rust_ai_ring/blob/744a75fccf79fc176ee145182891742845df3296/pet_apps/pet_server/src/roam.rs) 的 PET_WINDOW_LABEL 固定為 pet；roam_desktop 找不到該窗就輸出原錯誤並 break。
- [pet_windows.rs:59](https://github.com/williechen/rust_ai_ring/blob/744a75fccf79fc176ee145182891742845df3296/pet_apps/pet_server/src/pet_windows.rs#L59) 的 label 使用 pet- 加 UUID simple，open 依此建立各生活窗；設定檔只宣告 main 控制窗。
- [lib.rs](https://github.com/williechen/rust_ai_ring/blob/744a75fccf79fc176ee145182891742845df3296/pet_apps/pet_server/src/lib.rs) 在 setup 依 roam_enabled 啟動單一 roam_desktop，未由每隻成功建窗／出籠的生命週期啟動。即使只把固定字串換成某個 ID，也仍需處理尚未建窗、重複出籠、回籠取消及重新出籠。
- 現有 day18_roaming_selects_pet_life_window 測試仍斷言固定 pet，只驗舊假設；通過不代表真的能找到動態生活窗。

本錯誤可由目前來源的 label 不一致解釋，不需要先假設 WebKit 崩潰。修正條件回接 FUTURE_WORK 的 pet-roaming：傳入真 pet id／共用 label、按每隻生命週期管理漫遊工作與狀態；回籠停止且不影響其它寵物、重複出籠不得多開工作、再出籠可恢復。需三窗、逐隻開關、睡醒／拖曳、關閉／重開與真實移動驗證；不可只把迴圈改為持續重試或刪除錯誤日誌。

## 未定因：WebKit 載入失敗

WebKit 上游 [WebLoaderStrategy.cpp](https://github.com/WebKit/WebKit/blob/main/Source/WebKit/WebProcess/Network/WebLoaderStrategy.cpp) 的同名 timer 函式將內部失敗轉成資源載入錯誤；上游 main 不是使用者安裝版本，行號不應直接對應。本行本身不足以判定是前端 URL 不可達、特定資源／自訂協定、NetworkProcess 問題或某版 WebKitGTK 缺陷；不從文字斷言必須重裝 WebKit，也不宣稱與漫遊同因。

本產品 tauri.conf.json 的 devUrl 是 http://localhost:1420、frontendDist 是 ../pet_ui/dist；動態生活窗載入 index.html?view=pet&pet_id=...。開發模式需先確認 Trunk readiness 與該 URL／實際資源可讀；直接 cargo run 與 cargo tauri dev 的前置啟動行為須分辨。正式模式則核對打包資產與實際 URL，不把 dev server 檢查當正式資產驗收。

工具當次只讀 ss／ps 觀察：本執行環境 1420 未見 listener、未見 pet-server／trunk serve；這不代表使用者貼出錯誤當時的環境，也不足以結案定因。使用者後續已補充視窗現象，見下節；完整啟動指令仍待提供。

後續先取得啟動命令、失敗 URL、WebView label／pet id、載入前後日誌、WebKitGTK 套件版本及 binary／前端建置來源；開發端等待資源就緒後再重現，記錄是否仍有相同 WebKit 錯誤。未確定前不變更系統套件、GPU／sandbox 設定、不重設 DB。

## 狀態與保留範圍

漫遊：最新來源靜態確認缺口；產品未修正、未驗收。
WebKit：使用者錯誤回報已保存，原因待查，未驗收。
最新來源已見 Day37 的 persist_then_open／lifecycle_guard，不把舊交付時「未套用」延伸為當前程式完全沒有該切片；故障注入與完整 GUI 仍未驗。

此回報沿既有 Pet 生命週期／漫遊／CI產物項追蹤，不新增 Day 或 FIFO、不修改 learning-state／manifest／last_full_comparison、不改寫任何 Day 正文與原始雜湊。

## 使用者後續補充：所有視窗空白、每次出籠重新開啟

使用者原文：「所有視窗空白、每按下出籠都會重新開啟」。

這是人工操作回報，未由工具重現。調查優先序提高為先查共用前端頁面／JS／WASM 與 WebKit 載入；先修漫遊 label 並不能推定空白窗會恢復。每次按出籠「重新開啟」保留原措辭：尚未判定是同窗重新載入、舊窗消失後新建，或相同 pet id 累積重複窗。

最新 pet_windows::open 在建窗前已以相同動態 label get_webview_window 檢查，找到則 show／set_focus；因此應補 pet id、label、已存在視窗集合、build／Destroyed 時序與實際 PID 的證據，不能僅憑現象斷言程式完全沒有防重用邏輯。重現時必須連續操作同一 pet id；選到不同寵物屬不同窗。

本 SHA 的 pet_ui/index.html 只有前端橋接 script 與 Trunk Rust 載入入口，畫面依 WASM 掛載；因此 JS／WASM 失敗是需要查驗的路徑，尚非已確認原因。必須取得失敗 URL、HTTP／自訂協定回應與 WebView console／載入事件，再判定啟動服務、資源打包、JS 或 WebKit 內部錯誤。未安裝／升級任何套件、未關閉 sandbox／GPU、未清資料。

尚待使用者提供完整啟動指令，以辨別 devUrl 或打包資產模式。本補充不宣稱產品已修復或已通過重複出籠／載入驗收。
