# 目前教學與驗證狀態

2026-10-30 核對產品 `7ce39f0166a2120539e5908b33c88322a930c35a`。三案下一待發布均為 Day44；Day43 已發布正文只留在 [分包入口](../daily/day-43/README.md)。本次整理沒有新產品測試、沒有新增 Day；完整 Gate2 尚未結案。

| 範圍 | 最新可引用結果 | 精確來源 |
| --- | --- | --- |
| 遠端 CI | 七個 workflow 均 completed/success，含 Test All；已解除 SSR import 失敗 | `7ce39f0`，本次重新讀取 |
| Chat 本地 SSR | UI 21、server／SQLite／UI 組合 60 通過 | `7ce39f0` |
| 共用／Chat／Pet／Analytics 本地 | metadata／fmt／check／clippy；一般測試各 8／60／99／41，Analytics 一般另 1 ignored | `d2612fc` |
| WASM／Node／PG | Chat hydrate／Pet WASM、Node 10、Analytics PG 60（include-ignored）通過 | `d2612fc` |
| 合成舊 DB | 三案保資料升級／冪等／checksum 拒絕及 PG dump/restore 通過 | `d2612fc`；不等於使用者實際舊庫 |
| Pet 原生局部 | A/B 可見／查詢，A 單獨出籠／回籠及重啟，人工回報與 SQLite／FD 一致 | `8b88269`；未追認新 SHA |

[機器可讀範圍與 CI](validation.json) · [同 SHA CI 原始快照](ci.json) · [既有原始證據 ZIP](../../archive/packages-2026-10-30/05-source-verification-2026-10-30.zip)。各結果只代表自己的 SHA，不把不同命令的重疊測試相加當覆蓋量。

尚待：完整 Chat 房間 ACL／歷史／Session 旅程；Pet 預設資格分離、三隻／漫遊／native 故障補償／完整逐列顯示；Analytics 角色與來源 ACL、完整密碼／Session 旅程及實際舊 PG 備份驗收。現無使用者舊 PG 備份，不能以合成 schema 代替。全部完成條件見 [FUTURE_WORK](../../FUTURE_WORK.md)。

Day44 尚無原稿；後續未驗證新稿保留原檔。last_full_comparison 維持 `2026-10-16T22:47:14.399027+08:00`，未因日常整理更新。
