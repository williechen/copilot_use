# 最新本地資料合併：P0–P12 十三階段完整規劃

本輪依直接使用者指示更正為十三階段，以更新到 main 的本地程式／文章，加上本地既有 curriculum 合併細節。只更新 copilot_use 文件，不修改 rust_ai_ring。

## 本地來源

|來源|版本／內容|取用界線|
|---|---|---|
|程式 rust_ai_ring|`5c713553daee4055738239f047d6c774fcc7a7a1`，2026-10-25 15:54:34 +08:00 的 Phase2 day36|本聊天獨立 checkout fetch main；202 個 tracked 檔重新雜湊、16 處來源取證；未執行測試或產品驗收|
|文章 copilot_use|`4a3c43187e1ffd7cf4ba8d44cd4f8529f0d9d6d4`，2026-10-25 18:28:39 +08:00|105 閱讀版本、157 正文／修訂／鏡像之本地掃描及逐篇分類；Day1 三原文缺失|
|本地 curriculum|P0–P12，13 階段，每階段2個月，2026-10-12至2028-12-12|原 labels／日曆／每日60分鐘／next_day／既有完成結果保留|
|前十二階段 Roadmap|原 blob／SHA256／全文保存於 previous-roadmap.json|撤回「13階段是衝突」說法；不把原十二階段稿當更高權威|

## 規劃成果

- [十三階段詳細 Roadmap](../../ROADMAP_P0_P12.md)：每階段 Scope、Non-goals、Prerequisites、Deliverables、Acceptance、Tests、Evidence、Status；另有產品適用矩陣、原日曆與執行優先序。
- [結構化階段規劃](phase-plan.json)、[階段／文章對照](PHASE_ARTICLE_MAP.md)、[本輪程式來源與差距](SOURCE_BASELINE.md)。
- P0–P10 沿前版功能順序；P1.0 優先基礎 Logging 是 P1 工作包，P10 完整觀測。原末段拆為 P11 部署／營運就緒、P12 版本發布／installer／updater／升級回滾；此細分是本次規劃決策，不假稱本地舊資料已有相同命名。
- 同步 root README、docs README、共用規則、FUTURE_WORK 的規劃入口；learning-state 只增現行規劃引用與名稱，不改既有日期／next_day／歷史測試；manifest 只增引用。
- daily README 尾段過時的 Day35 同步為現行 Day37；不是新增教學日。舊 ROADMAP_P0_P11 保留為新版入口，原文可由 commit／來源保存追溯。

## 驗證與限制

原文章與歷史證據逐位元不變；工程／CI／runtime／功能驗收結果不更新。新規劃是文件交付，P0–P12 沒有因本次編寫而標 Verified。新文件的連結、階段數、日曆／進度、原文雜湊及僅文件變更會於發布前驗證；GitHub 以原 head 為 parent 進行原子更新，發布後重新取得 commit 核對檔案內容。

[原文章掃描（已更正十三階段）](../p0-p11-local-scan-2026-10-25/README.md) · [原十二階段全文與來源](previous-roadmap.json)。
