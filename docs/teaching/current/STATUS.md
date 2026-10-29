# 目前教學與驗證狀態

**Pet Day44 修訂更正：** 原新增 `auto_release_on_start` 欄位的 Green 已撤回封存；新規則要求出／回籠依 `pet_activity` 任一 `is_activity=true` 判斷，不限散步、不新增欄位。替代 Green 待完成／待驗，原 Gate1 歷史讀回不代表修訂已完成。next_day 保持三案 45，先補齊 Pet Day44 修訂，不重複加 Day。


| 本次 Gate1 文件 | 文章 | commit | 讀回 |
| --- | --- | --- | --- |
| Chat | [Day44](../daily/day-44/chat.md) | [`26705cb8a185`](https://github.com/williechen/copilot_use/commit/26705cb8a185be4a13ff7e777d0860edb6b0db4a) | 全文相等，blob／SHA256 已存 |
| Pet | [Day44](../daily/day-44/pet.md) | [`971f0e3e17a6`](https://github.com/williechen/copilot_use/commit/971f0e3e17a6705916a324ac9675f68ebb136fac) | 原方案讀回；目前修訂 Green 待補 |
| Analytics | [Day44](../daily/day-44/analytics.md) | [`585384c39d3a`](https://github.com/williechen/copilot_use/commit/585384c39d3aeac9c535992e110ebd5c9d1f2854) | 全文相等，blob／SHA256 已存 |

[完整交付讀回證據](../daily/day-44/delivery-verification.json) · [ENTRY](../daily/day-44/ENTRY.md) · [manifest](manifest.json)。

## 歷史驗證（只代表原 SHA，不代表 Day44）

| 範圍 | 最新可引用結果 | 精確來源 |
| --- | --- | --- |
| 遠端 CI | 七個 workflow completed/success（含 Test All）；SSR import 修正 | `7ce39f0` |
| Chat 本地 SSR | UI 21、server／SQLite／UI 組合 60 通過 | `7ce39f0` |
| 共用／Chat／Pet／Analytics 本地 | metadata／fmt／check／clippy；一般測試各 8／60／99／41，Analytics 另 1 ignored | `d2612fc` |
| WASM／Node／PG | Chat hydrate／Pet WASM、Node 10、Analytics PG 60（include-ignored）通過 | `d2612fc` |
| 合成舊 DB | 三案保資料升級／冪等／checksum 拒絕與 PG dump/restore 通過 | `d2612fc`，非使用者實際舊庫 |
| Pet 原生局部 | A/B 可見／查詢、A 單獨出／回籠及重啟；人工回報 | `8b88269`，不可追認新 SHA |

[歷史機器驗證範圍](validation.json) · [原 CI 快照](ci.json) · [舊原始證據 ZIP](../../archive/packages-2026-10-30/05-source-verification-2026-10-30.zip)。同一測試不得跨 SHA 疊加計功。

**具體缺口**：Chat 房主／逐房 ACL、歷史與 Session 旅程；Pet Day44 活動契約替代 Green／舊庫保資料轉換、三隻／漫遊／Native X 故障補償；Analytics 真 PG logout CSRF、角色／來源 ACL、密碼與 Session 全旅程；使用者實際舊 PG 備份仍未取得。FIFO Chat Day25、Pet Day25 不變。所有完成條件見 [FUTURE_WORK](../../FUTURE_WORK.md)。

`last_full_comparison` 維持 `2026-10-16T22:47:14.399027+08:00`，未因本次教學文件更新。

Pet 目前資料模型修訂：任一活動 `is_activity=true` 即出籠，所有活動皆非 true 才回籠，不限散步且不新增欄位。[隔離 SQL 契約 12 項通過](pet-activity-contract.json)，但產品 caller 仍使用舊旗標，替代 Green 與 GUI／升級驗收未完成。

Pet 活動建立與漫遊規則：建立寵物同交易建立「散步」「坐下」；散步 true 可漫遊、坐下 true 仍出籠但不可漫遊，回籠停止全部活動。此新增確認尚未接入產品或經 GUI／建立交易測試；原 12 項 SQL 探針只涵蓋任意活動的出回籠判斷，不涵蓋這次新增的動作／建立流程。
