# 各案Refactor目前進度

產品`2c7ff8b861d32ab6d879bf80c9f2dce50de38097`已含Day44三案重構；本輪只重驗Pet，其餘重構測試仍維持4b1008b來源；原7ce附件已真check／apply。功能Green、同案重構與跨案工作分列，不增加Day。

| 專案 | 目前caller／舊斷言 | 本次驗證／後續 |
| --- | --- | --- |
| Chat | room_auth→room_path；已採用教學命名；匿名回跳、Session、SSR／WS斷言保留 | main62、SSR62；原重現加fmt後server lib29＋UI21（歷史來源）。後續Session SQL／Repository錯誤與fixture |
| Pet | pet_windows→window_position；負座標、工作區、UUID／A/B斷言保留 | main2c7ff8b112；其他案結果仍維持4b1008b來源；活動Green不是幾何重構。後續真native故障fixture／逐IDGUI |
| Analytics | Dashboard→dashboard_url；CSV、kind／source URL斷言保留 | main43＋1ignored；最後main真PG63，fixture已採用。後續Session SQL／legacy caller分層 |

[三案全文與附件](../daily/day-44/README.md) · [原始命令與證據](../../source-review/local-validation-2026-10-30-day44-r2/README.md) · [D01–D09原項目](../../FUTURE_WORK.md#adapter-cleanup)。跨案契約、shared相依及全庫CI只在[engineering](../../engineering/README.md)，不以本案進度抵算G-01／G-02。

[Pet 2c7ff8b採用後最新驗證](../../source-review/pet-day44-adopted-2026-10-30/README.md)；其他案測試保留原4b1008b來源，不追認新SHA。
