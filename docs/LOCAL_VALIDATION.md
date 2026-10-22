# 主體程式測試與功能驗收

`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_chat` 是唯一程式、測試、CI 與功能驗收主體。 所有程式測試、CI 與功能驗收只對 rust_ai_chat 精確 SHA 記錄；文件更新不代表重新執行測試。

## 驗證方式

在隔離的 rust_ai_chat checkout，依當時 Cargo／lock／feature 與實際入口選擇 fmt、Clippy、unit／DB／HTTP／WS、SSR／WASM、GUI、重啟與跨層驗收。測試命令、預期／實際、來源 SHA、workflow／job、平台與未覆蓋部分分別揭露。必要工具或資料庫不可用時標示未執行，不以另一庫結果代替。

Analytics postgres-integration 必須涵蓋所有適用 targets 與專用測試資料庫；若當時有 ignored 的 DB 流程，核對其環境條件後用 --include-ignored 執行，不能把 CI 未執行項目算作通過。Chat／Pet SQLite 與 Analytics PostgreSQL 隔離，不操作正式 DB／秘密，不執行付費 AI。

CI 只採 [rust_ai_chat Actions](https://github.com/williechen/rust_ai_chat/actions) 的精確來源與覆蓋。本機測試、該庫 CI、UI／GUI、程式套用及完整功能驗收各自記錄；任何單項 success 不代表全部完成。copilot_use 只做文章／規則／JSON／連結與交付讀回一致性核對，不執行其舊程式驗收。

## 歷史與現行 Gate

舊 copilot_use 512 sockets、gRPC、跨平台安裝／更新與 Release 驗收[完整原文保留](teaching/history/evidence/2026-10-23-repository-roles/before/docs/LOCAL_VALIDATION.md)，只屬原來源，不作 rust_ai_chat 的已完成能力或測試門檻。本次沒有對新程式 SHA `f61ea1c59667f3c92c78e576e3c48038a8ce030a` 執行新測試。

[共同規則](COMMON_LEARNING_RULES.md) · [目前進度與驗證邊界](teaching/current/STATUS.md) · [既有流程尚未調整](REPOSITORY_ROLE_TRANSITION.md)
