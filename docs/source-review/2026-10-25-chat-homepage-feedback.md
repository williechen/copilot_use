# Chat 首頁操作回報與後續修正（2026-10-25）

使用者提供 21:51–21:52 截圖，並明確回報「已登入，首頁仍顯示登入」及「進入首頁必須按讀取／重新整理才有房間列表」。本報告將人工觀察、靜態來源與未執行驗證分開記錄；不以截圖推定執行中的 binary／WASM SHA 或伺服器 Session 有效性。

文件基準：`copilot_use/main@6dd9d5b0e9ac61bb228c3cf0f3c66f25af9d2868`。
本次即時讀取程式：[`rust_ai_ring/main@dbe00814f1c6c461e7c88258a890063fb35034b1`](https://github.com/williechen/rust_ai_ring/tree/dbe00814f1c6c461e7c88258a890063fb35034b1)，維持唯讀。

## 人工觀察與來源對照

| 項目 | 使用者觀察／截圖 | 最新來源靜態所見 | 判定 |
| --- | --- | --- | --- |
| 登入入口 | 使用者表示已登入；圖一／二仍顯示「登入」 | [chat_ui/src/lib.rs:155](https://github.com/williechen/rust_ai_ring/blob/dbe00814f1c6c461e7c88258a890063fb35034b1/chat_apps/chat_ui/src/lib.rs#L155) HomePage 固定輸出 href=/signin 與「登入」，無 Session 分支 | 來源缺少依 Session 切換「管理」的接線；實際 Session 尚未由本次工具查驗 |
| 首頁列表 | 圖一「請先讀取房間列表」；圖二按讀取後已有 122、233 | [chat_ui/src/lib.rs:344](https://github.com/williechen/rust_ai_ring/blob/dbe00814f1c6c461e7c88258a890063fb35034b1/chat_apps/chat_ui/src/lib.rs#L344) RoomList 初始 rows 空、has_snapshot=false；list_rooms 只在重新整理按鈕的事件內呼叫，無初次載入 | 缺少進入首頁即自動讀取的生命週期流程 |
| 房間頁側欄 | 圖三進房後仍要求讀取列表 | RoomPage 與 HomePage 重用 RoomList | 保留現象；修改共用元件時須回歸房間頁，不能宣稱本次已修復 |

首頁有效 Session 顯示「管理」原已列於 USER_FLOWS，這次是缺陷證據；「進入首頁即自動讀取」補成明確完成條件。兩項回接 FUTURE_WORK 的 chat-auth／chat-feedback，不新建 Day／FIFO／平行清單。

## 完成條件與後續實作範圍

1. 以伺服器驗證的現有 Session 決定首頁入口：有效登入顯示「管理」，指向既有個人管理入口；未登入／失效顯示「登入」。不要讀取或暴露 HttpOnly Cookie 給 JS，也不要用 localStorage 旗標冒充登入。個人管理仍由後端授權，不能只更改文字。登入返回首頁、重新整理、登出及 Session 過期均需重新核對，不短暫把有效登入錯顯為未登入。
2. 進入首頁不需任何點擊即自動讀取公開房間列表；載入中、成功非空、成功空列表與失敗可重試明確區分。「讀取／重新整理」保留為手動刷新入口，不能是首次取得列表的前置操作。匿名也能讀公開清單；進房授權仍由後端檢查。
3. 自動載入與手動查詢／刷新重用既有真 HTTP API 與結果處理，避免 hydration 重複請求、舊回應覆寫新搜尋、重複點擊及 unmount 後回寫。失敗保留已有 rows 與必要查詢字串；首次失敗則顯示錯誤／重試，不能當空清單。
4. 以 SSR／hydration／瀏覽器真 Session、HTTP 請求與 DOM 驗證：未登入首頁、登入後返回首頁、重新整理、有效／失效 Session、首次自動載入、空結果、離線與重試；房間頁共用列表亦做回歸。helper 測試與 CI 成功不能代替此操作驗收。

## 與既有教學的關係

本來源已見 Day37 的 apply_room_search 與真 WASM 搜尋 caller，不能繼續把該切片說成完全未套用。但這個函式修的是「使用者送出搜尋後的完成狀態」，不包含首頁登入入口或首次自動讀取；不能由 Day37 文件交付或新 helper 推定這兩個需求已完成。

Day37 r2 正文與原始 commit/blob/SHA256 維持不變。此回報只補需求、既有修正項目、現況與證據，不推進 next_day、不更新 last_full_comparison、不結案 FIFO。沒有修改 rust_ai_ring、沒有重新編譯、沒有執行新產品測試或瀏覽器／HTTP／DB 驗收。

## 截圖原始識別

圖片由使用者附於本次對話；未另上傳到產品庫。檔名時間為原檔名，不推定系統實際執行版本。

| 原始檔名 | SHA256 |
| --- | --- |
| 螢幕快照 2026-10-25 21-51-02.png | `81a13d98b0ad411c81c5c7fcf993fb4668511413db64bacac979b2f8e560a687` |
| 螢幕快照 2026-10-25 21-51-12.png | `5ef54b2c7b607dd399059c8b82a2819c6937864bc5e07c419094683b7234208c` |
| 螢幕快照 2026-10-25 21-52-01.png | `d0310c2d2c4f7e0519234c30b589e9da560b867fe66e6b4cf666abbeb1a85689` |
