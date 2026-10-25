# 教學與主體程式銜接

現行程式來源與目錄：[rust_ai_ring 四個 workspace 對照](SOURCE_LAYOUT.md)（來源 `0b3e5844`，2026-10-24 靜態核對）。

`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_ring` 是唯一程式、測試、CI 與功能驗收主體。 教學原始碼保持唯讀，由使用者套用；兩庫分別記錄程式來源 SHA 與文件交付 SHA。

## 按功能提供完整重構的教學檢核

依 [functional-layering-v1](COMMON_LEARNING_RULES.md) 與 [架構責任正本](SYSTEM_ARCHITECTURE.md#functional-layering)，每篇在第②節提供當次功能的「目前呼叫鏈／混層位置 → 目標責任與依賴 → Red → 完整Green → Refactor與必要清理」。

1. 列明該功能的真實UI、正式路由／IPC、用例、Port、資料／原生Adapter及現有測試；分清已存在、準備新增與暫不涉及。不能以helper被測到就當正式caller已改。
2. 給出必要完整函式／模組、Port／DTO／錯誤型別、Cargo normal／build／dev依賴、features、mod／pub use／imports、組裝注入與所有受影響caller。變更層不要求每日湊齊；未改層說明重用位置。
3. 移動SQL、用例或UI時，同篇重接handler／command／元件與真Adapter測試，交代相容橋接與舊實作／註冊清理；不留兩套平行業務流程。有效斷言先遷再清理，不重啟已廢除的固定測試搬移計畫。
4. 用例／Domain測規則，Adapter測真DB／交易或native契約，正式HTTP／IPC測接線與授權，UI測可見載入／成功／錯誤／重試；核對依賴方向。每項明列預期／實際／未執行與證據SHA，未跑GUI不冒稱完整功能重構已驗收。
5. 本次只是更新規則與後續教學，不新增教學日、產品修改或新待辦。每次從即時next_day與上一篇未完成事項選功能；後續若交付新教學仍依原獨立提交／讀回／條件同步規則，舊文章與進度證據保持不可變。

## 每次核對

1. 在 rust_ai_ring 最新 main 找出型別／函式、實際 UI／HTTP／IPC 呼叫者、Application／Domain、SQLx Adapter 與測試。目錄入口見 [系統架構](SYSTEM_ARCHITECTURE.md)。
2. 只依該庫 Cargo.toml／Cargo.lock、package／feature、路由與 DB 契約給出可套用的命令和完整 Green；不移植舊庫 day9-*、pet_runtime、analytics_transport 或腳本名稱作為已存在介面。
3. 測試放所屬 app／crate／模組；舊測試搬移計畫按最新已完成部分查重，不因歷史待辦重做。
4. 純 Domain／Application 不依賴 HTTP／DB／Tauri driver。依主體 Cargo normal／build 依賴及實際呼叫鏈驗證；copilot_use 的 architecture-boundaries.py 不是主體的通過證據。
5. 程式測試及 CI 只認 rust_ai_ring 精確 SHA、workflow／job 與覆蓋；文件全文讀回、next_day／manifest 條件同步在 copilot_use，兩者不互相替代。

## 交付

先取得各案即時 next_day，核對其上一篇與未完成事項；每案每 Day 獨立文章 commit、全文讀回與條件同步，依 [共用規則](COMMON_LEARNING_RULES.md)。已發布正文與歷史證據不回寫，規則同步不加 Day。

舊雙庫程式適配表[原文保留](teaching/history/evidence/2026-10-23-repository-roles/before/docs/TEACHING_ALIGNMENT.md)，只供歷史追溯；現行缺口依 [FUTURE_WORK](FUTURE_WORK.md) 對 rust_ai_ring 最新來源核對。

## 後續新目錄程式調整

依使用者 2026-10-24 最新要求，後續教學必須以 [新目錄](SOURCE_LAYOUT.md) 提供真正可銜接的程式調整：路徑、Cargo package、Rust import、feature、實際呼叫端及測試一併對齊。現有 Pet / Analytics server 中的程式若需移入新 application / persistence，以當日最小切片教學，不先宣稱空 library 已接通。

不重寫已發布教學；下一篇從即時 next_day 取值，重新核對上一篇及來源 SHA，不能把目錄整理當成交付或驗收完成。
