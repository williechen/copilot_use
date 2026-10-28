# 目前功能與驗證狀態

> 2026-10-28 現行驗證修訂：產品 `e8a640978732651cbbca1483b07e771f876a439b` 已含三案 Day42 實作／各案 Refactor；[本地實測矩陣與限制](../../source-review/local-validation-2026-10-28-e8a6409/README.md) 優先於下列舊來源現況摘要。歷史結果維持原 SHA，不追認本次通過；GUI／CI／完整版本 Gate 未完成，next_day 三案維持 42。


最新產品 `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`。本輪在隔離clone真正重跑Cargo與可行瀏覽器流程；產品原始工作樹未修改。

| 專案 | 本輪實際結果 | 尚缺 |
| --- | --- | --- |
| Chat | fmt/check、workspace SSR Rust59、WASM與GUI build通過；瀏覽器登入→/account→首頁管理→登出／重載通過 | 房間ACL、Session到期等完整GUI驗收 |
| Pet | fmt/check、Rust72（server60＋Application8＋SQLite4）、WASM及Node6通過 | 原生指定多窗／Alt+F4逐隻回籠／重新出籠／兩種重啟：使用者操作確認＋SQLite核對通過；故障注入與漫遊未驗 |
| Analytics | fmt/check、帶postgres-integration的52＋單獨ignored1＝53通過；瀏覽器登入、精確篩選、CSV實際下載解析、清除、切來源及登出通過 | 首次改密碼GUI、外部試算表及完整角色／來源ACL |

CI：已嘗試重跑Chat build、Pet test、Analytics check三個job，連線皆回403 Resource not accessible by integration；沒有啟動新attempt。同SHA既有遠端CI僅列觀察，不當本輪重跑成功。

[真正命令／cwd／exit／logs、畫面與限制](../daily/day-40/rerun-9c76797/README.md) · [機器紀錄](../daily/day-40/rerun-9c76797/verification.json) · [上一輪僅來源與fmt](../daily/day-40/source-update-9c76797/README.md) · [歷史實測](../daily/day-40/local-validation/README.md) · [歷史更正](../daily/day-40/historical-corrections.md)。

舊ff14的Chat57通過／2失敗、b0的Pet69通過／缺3項測試，以及修復候選結果保持原SHA。本輪是9c76797實測結果，不能倒改舊失敗。next_day三案仍41；FIFO、last_full_comparison及重構計功不變；完整Gate2未通過。已停止測試伺服器、移除本輪約5.18GB暫存及專用DB，產品工作樹乾淨。

[後續原生GUI實測](../daily/day-40/native-gui-9c76797/README.md)：代理沒有原生點擊／截圖能力，操作及視窗顯示由使用者直接確認，與代理查核的DB／程序證據分列。
