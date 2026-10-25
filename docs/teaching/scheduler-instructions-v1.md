# 三專案每日 TDD 教學排程

2026-10-26 更新文件指令。詳細規則以[共用規則](../COMMON_LEARNING_RULES.md)為準，功能與測試銜接見[整理入口](functional-refactoring/README.md)；進度讀[learning-state](../learning-state.json)與[manifest](current/manifest.json)。

## 排程指令

```text
每日依 Chat → Pet → Analytics 交付各案一個最小完整功能切片。

先重讀 williechen/copilot_use/main 的 docs/COMMON_LEARNING_RULES.md，以及規則指定的架構、功能與測試整理、即時進度、manifest、現行入口和上一篇；取得兩庫最新精確 SHA。依即時 next_day 選題，不固定日編號，不重教已完成內容。

依 williechen/rust_ai_ring 的實際呼叫鏈提供 Red、完整 Green、Refactor：同步必要的功能分層、測試保留／合併／遷移、Cargo／feature／caller與CI調整。跨層契約保留，三案CI互不阻擋；不刪有效斷言換全綠。

核對交付當日官方穩定版本，分列預期、實際及未執行結果。每案文章獨立提交、精確commit全文讀回，保存blob／SHA256後，才條件式同步該案進度、manifest、七首頁及現行入口。

只寫 copilot_use/main 文件；rust_ai_ring唯讀。保留已發布文章與原始證據，不建立第二套產品、不更新last_full_comparison；FIFO補教依共用規則處理。失敗如實記錄，繼續其餘案；不把文件交付冒稱產品驗收。
```

## 同步狀態

已同步網頁版原任務「三專案每日 TDD 教學」，重新載入後全文讀回一致。只修改提示詞，頻率、時間與模型未變更；未新增任務或立即執行。[原任務](https://chatgpt.com/scheduled?automationId=6acd967c15fc8191ade8407954c5b354&automationSource=cloud) · [本次紀錄](current/scheduler-document-refresh-2026-10-26.json) · [前版原文](history/evidence/2026-10-26-scheduler-document-refresh/before.md)
