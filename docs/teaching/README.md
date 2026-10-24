# 三專案教學首頁

<!-- duplicate-review:start -->
[重複套用與來源／產物核對（2026-10-25）](../source-review/2026-10-25-duplicate-application-review.md)：程式來源 `3236b94e`，核對與產品驗收分開記錄。
<!-- duplicate-review:end -->

<!-- daily-delivery:start -->
## 最新交付與下一篇

| 專案 | 最新文章 | 下一篇 | 狀態 |
| --- | --- | --- | --- |
| Chat | [Day34 r2：重用 SignInPage，合併重複登入入口](daily/day-34/chat-r2.md) | Day35 | 教學已交付／功能未驗收 |
| Pet | [Day34：新增寵物名稱規則接入 application 並拒絕控制字元](daily/day-34/pet.md) | Day35 | 教學已交付／功能未驗收 |
| Analytics | [Day34：首次 admin/admin 初始化及強制改密碼銜接](daily/day-34/analytics.md) | Day35 | 教學已交付／功能未驗收 |

來源、套用及測試證據各自記錄；文章發布不代表產品驗收。
<!-- daily-delivery:end -->

2026-10-24 目錄已同步至 [williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)：保留 `chat_apps`、`pet_apps`、`analytics_apps`、`crates` 四個 workspace；[新目錄與套件對照](../SOURCE_LAYOUT.md)。後續教學依當次來源 SHA 調整程式、Cargo、imports、呼叫端與測試。

[共用文章與規則](../COMMON_LEARNING_RULES.md) · [目前進度](current/STATUS.md) · [最新進度檔](../learning-state.json)

| 專案 | 首頁 |
| --- | --- |
| Chat | [目標、操作主線、待辦與全部 Day](chat/README.md) |
| Pet | [目標、操作主線、待辦與全部 Day](pet/README.md) |
| Analytics | [目標、操作主線、待辦與全部 Day](analytics/README.md) |


[新文章目錄](daily/README.md) · [驗收完成文章](archived/README.md) · [既有歷史文章與原始證據](history/README.md)

每新增一個專案 Day 就獨立 commit、推送與全文讀回，再條件同步狀態和 manifest。教學交付與產品驗收分開；資料整理不增加 Day。
