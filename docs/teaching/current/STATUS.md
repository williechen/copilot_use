# 目前教學與驗證狀態

最新產品`2c7ff8b861d32ab6d879bf80c9f2dce50de38097`已採用Pet forward。只認各自受測SHA，不將未變動專案的舊測試追認新SHA。

| 範圍 | 結果與來源 | 界線 |
| --- | --- | --- |
| 四workspace metadata | 2c7ff8b locked／no-deps通過 | 不代替完整測試 |
| Pet | **2c7ff8b main112**、fmt／check／clippy、Node12、WASM通過 | 新模型native GUI未驗 |
| Pet升級 | 2c7ff8b合成及實際SQLite唯讀backup副本、冪等／checksum拒絕通過 | 原DB未寫，原四版migration不改 |
| Chat／crates | 4b1008b Chat62／SSR62／hydrate WASM；crates8 | tree與2c7ff8b相同，但未重跑這些測試 |
| Analytics | 4b1008b一般43＋1ignored、真PG63 | tree未變，未重跑；實際舊PG無備份 |
| 最新CI snapshot | {"in_progress": 2, "success": 5}，來源2c7ff8b | [時間／workflow](ci.json)，in_progress不當通過 |

[Pet最新命令／SHA／checksum／資料保留](../../source-review/pet-day44-adopted-2026-10-30/README.md) · [4b1008b完整矩陣](../../source-review/local-validation-2026-10-30-day44-r2/README.md) · [Day44正文](../daily/day-44/README.md) · [Pet精確讀回](../daily/day-44/pet-adoption-verification-2026-10-30.json)

舊候選檔尾換行與main不同，SQLx checksum也不同；以main原檔驗證，不改已部署migration。新模型native GUI／三隻／Native X／真OS補償／漫遊、完整瀏覽器旅程、native binary／release／三平台安裝、實際舊PG還原、完整官方版本／MSRV Gate仍待驗。本輪沒有重建Trunk，舊dev建置結果只屬原SHA。

完整Gate2=false；next_day三案45、FIFO、last_full_comparison=`2026-10-16T22:47:14.399027+08:00`不變。[FUTURE_WORK](../../FUTURE_WORK.md)。
