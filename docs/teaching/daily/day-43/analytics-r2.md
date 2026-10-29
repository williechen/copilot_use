# Analytics Day43 r2｜最新來源隔離 PostgreSQL 驗證

2026-10-30，來源 `d2612fcd154d2132d850086d83a48578abbb07fa`。[完整功能與獨立 Refactor](analytics.md) · [前次本地修訂](analytics-r1.md)。

最新產品已含來源切換保留事件類型與 Dashboard 計數標籤重構；不重複套用原 patch。原附件在 c6708c4 隔離 clone 按 Refactor→Green 完成 check／apply，原候選 fmt 失敗保留。

| 本輪驗證 | 結果 |
| --- | --- |
| metadata／fmt／workspace check／clippy | exit 0 |
| 一般 workspace tests | 41 passed／1 ignored，exit 0 |
| 專用 PG `postgres-integration -- --include-ignored` | 60 passed／0 ignored，exit 0 |
| 有資料合成舊 PG schema 升級 | 管理員雜湊與 2 事件保留、冪等、checksum 漂移拒絕，exit 0 |
| pg_dump／pg_restore 到另一隔離 DB 後升級 | exit 0 |

[完整命令、工具版本及 DB 證據](../../../source-review/local-validation-2026-10-30-day43/README.md)。PG 使用專用 55482 埠及本輪 DB，驗證後停止；未操作正式服務。一般測試 ignored 不當作已跑，額外 PG 命令的結果單獨列示；兩組有重疊，不相加作覆蓋量。

本輪未重跑真瀏覽器、外部 HTTP smoke 或實際使用者舊 PG 庫；目前沒有使用者舊 PG 備份，不能以合成 fixture 代替。未升級 lock／依賴，未重作完整官方版本／MSRV Gate，原版本說明保留原日期。Refactor 僅屬 Analytics；next_day 44、FIFO、last_full_comparison 不變，完整 Gate2 未結案。
