# 教學與主體程式銜接

`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_chat` 是唯一程式、測試、CI 與功能驗收主體。 教學原始碼保持唯讀，由使用者套用；兩庫分別記錄程式來源 SHA 與文件交付 SHA。

## 每次核對

1. 在 rust_ai_chat 最新 main 找出型別／函式、實際 UI／HTTP／IPC 呼叫者、Application／Domain、SQLx Adapter 與測試。目錄入口見 [系統架構](SYSTEM_ARCHITECTURE.md)。
2. 只依該庫 Cargo.toml／Cargo.lock、package／feature、路由與 DB 契約給出可套用的命令和完整 Green；不移植舊庫 day9-*、pet_runtime、analytics_transport 或腳本名稱作為已存在介面。
3. 測試放所屬 app／crate／模組；舊測試搬移計畫按最新已完成部分查重，不因歷史待辦重做。
4. 純 Domain／Application 不依賴 HTTP／DB／Tauri driver。依主體 Cargo normal／build 依賴及實際呼叫鏈驗證；copilot_use 的 architecture-boundaries.py 不是主體的通過證據。
5. 程式測試及 CI 只認 rust_ai_chat 精確 SHA、workflow／job 與覆蓋；文件全文讀回、next_day／manifest 條件同步在 copilot_use，兩者不互相替代。

## 交付

先核對 Day27，再取各案最新 next_day；每案每 Day 獨立文章 commit、全文讀回與條件同步，依 [共用規則](COMMON_LEARNING_RULES.md)。已發布正文與歷史證據不回寫，規則同步不加 Day。

舊雙庫程式適配表[原文保留](teaching/history/evidence/2026-10-23-repository-roles/before/docs/TEACHING_ALIGNMENT.md)，只供歷史追溯；現行缺口依 [FUTURE_WORK](FUTURE_WORK.md) 對 rust_ai_chat 最新來源核對。
