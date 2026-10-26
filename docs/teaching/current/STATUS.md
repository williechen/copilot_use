# 目前功能與驗證狀態

最新產品 `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`。本輪在隔離clone真正重跑Cargo與可行瀏覽器流程；產品原始工作樹未修改。

| 專案 | 本輪實際結果 | 尚缺 |
| --- | --- | --- |
| Chat | fmt/check、workspace SSR Rust59、WASM與GUI build通過；瀏覽器登入→/account→首頁管理→登出／重載通過 | 房間ACL、Session到期等完整GUI驗收 |
| Pet | fmt/check、Rust72（server60＋Application8＋SQLite4）、WASM及Node6通過 | 真原生多窗／點X／故障／重啟GUI未執行，缺可操作原生桌面介面 |
| Analytics | fmt/check、帶postgres-integration的52＋單獨ignored1＝53通過；瀏覽器登入、精確篩選、CSV實際下載解析、清除、切來源及登出通過 | 首次改密碼GUI、外部試算表及完整角色／來源ACL |

CI：已嘗試重跑Chat build、Pet test、Analytics check三個job，連線皆回403 Resource not accessible by integration；沒有啟動新attempt。同SHA既有遠端CI僅列觀察，不當本輪重跑成功。

[真正命令／cwd／exit／logs、畫面與限制](../daily/day-40/rerun-9c76797/README.md) · [機器紀錄](../daily/day-40/rerun-9c76797/verification.json) · [上一輪僅來源與fmt](../daily/day-40/source-update-9c76797/README.md) · [歷史實測](../daily/day-40/local-validation/README.md) · [歷史更正](../daily/day-40/historical-corrections.md)。

舊ff14的Chat57通過／2失敗、b0的Pet69通過／缺3項測試，以及修復候選結果保持原SHA。本輪是9c76797實測結果，不能倒改舊失敗。next_day三案仍41；FIFO、last_full_comparison及重構計功不變；完整Gate2未通過。已停止測試伺服器、移除本輪約5.18GB暫存及專用DB，產品工作樹乾淨。
