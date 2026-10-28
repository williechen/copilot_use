# 目前功能與驗證狀態

產品 `c6708c42f21ac919e57a379848b4977d27a1c3d6`；2026-10-28 增量重測。[本輪原始命令與結果](../../source-review/local-validation-2026-10-28-c6708c4/README.md)。

| 專案／層級 | 本輪新 SHA 實際結果 | 未執行與限制 |
|---|---|---|
| 四 workspace | metadata --locked --no-deps、fmt --all 四組通過 | 不代表編譯或功能驗收 |
| Chat | server／SQLite／SSR 57 項通過，含 UI 18 項；Router fixture 修正已在 main | 本輪未重跑 WASM、瀏覽器、完整 ACL／Session 情境 |
| Pet | Node 10 項通過，VM location／參數 fixture 修正已在 main | 本輪未重跑 Cargo／SQLite／native／GUI |
| Analytics | metadata／fmt 通過，workspace tree 與 c801cb2 相同 | 本輪未重跑 PG／HTTP；角色／來源 ACL 未驗收 |

[前輪 c801cb2 四 workspace、WASM、PG 結果](../../source-review/local-validation-2026-10-28-c801cb2/README.md) 僅屬舊 SHA。[更早 GUI／原生人工證據](../daily/day-40/rerun-9c76797/README.md) 亦保留原版本，不追認本輪。

GUI／CI rerun、舊 DB 升級與完整 MSRV／版本 Gate 未完成；Gate2 未通過。next_day 三案 42，FIFO、last_full_comparison 不變。

[功能問題／原完成條件](../../FUTURE_WORK.md) · [文章勘誤](ARTICLE_CORRECTIONS.md) · [三案文章](README.md)。
