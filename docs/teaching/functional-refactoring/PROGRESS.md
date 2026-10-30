# 各案Refactor目前進度

產品`6b34c42f74fc1585b1b864fb7dd6969318270e0f`已含Day44三案重構；原7ce附件已真check／apply。功能Green、同案重構與跨案工作分列，不增加Day。

| 專案 | 目前caller／舊斷言 | 本次驗證／後續 |
| --- | --- | --- |
| Chat | room_auth→room_patch；原附件名room_path；匿名回跳、Session、SSR／WS斷言保留 | main62、SSR62；原重現加fmt後server lib29＋UI21。後續Session SQL／Repository錯誤與fixture |
| Pet | pet_windows→window_position；負座標、工作區、UUID／A/B斷言保留 | main100、活動候選110；活動Green不是幾何重構。後續真native故障fixture／逐IDGUI |
| Analytics | Dashboard→dashboard_url；CSV、kind／source URL斷言保留 | main43＋1ignored；PG舊fixture失敗、修補後63。後續Session SQL／legacy caller分層 |

[三案全文與附件](../daily/day-44/README.md) · [原始命令與證據](../../source-review/local-validation-2026-10-30-day44/README.md) · [D01–D09原項目](../../FUTURE_WORK.md#adapter-cleanup)。跨案契約、shared相依及全庫CI只在[engineering](../../engineering/README.md)，不以本案進度抵算G-01／G-02。
