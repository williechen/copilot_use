# 歷史文件分包

前六包沿用固定文件來源 `59e30c9dabfcfa1eff036de3a81143521bfac41a`，SHA256 未變；第七包只收錄後續 `22cdfff1944d937f8e9492f91ab9511cce76aced` 中尚未被前六包收錄的不同版本，沒有重複壓縮同一來源路徑與內容。

| 包 | 範圍 | 原檔數 |
| --- | --- | ---: |
| [01-chat-2026-10-30.zip](01-chat-2026-10-30.zip) | Chat 教學版本 | 247 |
| [02-pet-2026-10-30.zip](02-pet-2026-10-30.zip) | Pet 教學版本 | 264 |
| [03-analytics-2026-10-30.zip](03-analytics-2026-10-30.zip) | Analytics 教學版本 | 234 |
| [04-shared-teaching-2026-10-30.zip](04-shared-teaching-2026-10-30.zip) | 共用教學歷史與跨案證據 | 1512 |
| [05-source-verification-2026-10-30.zip](05-source-verification-2026-10-30.zip) | 來源檢查與驗證紀錄 | 438 |
| [06-rules-and-indexes-2026-10-30.zip](06-rules-and-indexes-2026-10-30.zip) | 規則、需求與索引對照 | 60 |
| [07-superseded-document-versions-2026-10-30.zip](07-superseded-document-versions-2026-10-30.zip) | 後續過時導覽／規則／狀態的差異版本 | 24 |

每包保留 `copilot_use/` 內的原始相對路徑與內容。需要參照跨包連結時解壓至同一個獨立目錄；不要把全部歷史檔案解回目前工作目錄，否則會重新引入過時文件。未驗證的下一篇原稿在工作目錄保留。

[原始六包清單](manifest.json) · [本次逐檔映射與清理紀錄](current-files.json) · [全部 ZIP SHA256](SHA256SUMS)。封存只改保存方式，不代表內容符合目前產品或已完成驗收。現況只見 [STATUS](../../teaching/current/STATUS.md)。

## Pet 活動狀態契約更正

[第八包：被取代的 Pet Day44 新增欄位方案及必要導覽版本](08-pet-activity-contract-superseded-2026-10-30.zip)，來源 `ceb3719b7543f491fc230a05a55a28ef2b729bf7`。只封存尚未被既有 ZIP 收錄的不同版本；現行規則以 [Pet Day44 修訂](../../teaching/daily/day-44/pet.md) 為準。原文與新契約未混同為已驗收。

## Pet 基本活動與漫遊定義

[第九包：散步／坐下規則確認前的不同版本](09-pet-walk-sit-contract-superseded-2026-10-30.zip)，來源 `c5180fbe265560e21a7f8abb46b907a68b435906`。只補封存既有 ZIP 尚未收錄的版本；[現行 Pet Day44 修訂](../../teaching/daily/day-44/pet.md) 保留原檔，該封存版本當時替代Green未完成；現行完整r3與驗收界線見Pet正文。

## 坐下優先規則

[第十包：坐下優先確認前的不同版本](10-pet-sit-priority-superseded-2026-10-30.zip)；只補封存尚未被既有 ZIP 收錄的版本。現行規則：同時 true 時保留坐下，持久化關閉散步；仍出籠但不漫遊。

## Day44本地驗證修訂前原文

[第十一包](11-day44-validation-superseded-2026-10-30.zip)只存本次被取代、尚未在前十包收錄的27個不同版本；[逐檔blob／SHA256與重複排除](day44-validation-superseded.json)。來源dbde2846fdcad3ce2168ec6852f64dcdd3de9bf6，既有十包hash均未改。Day44現行修訂與完整證據仍保留原檔，因產品完整Gate2未完成。[本次實測](../../source-review/local-validation-2026-10-30-day44/README.md)。

## Day44本地重核分包

[第十二包：前輪6b34完整驗證原始證據](12-previous-day44-validation-2026-10-30.zip)與[第十三包：被本輪取代的現行文章／入口／狀態／patch](13-day44-recheck-superseded-documents-2026-10-30.zip)分開保存；[逐檔原blob／SHA256與去重](day44-recheck-archive.json)。前十一包hash不變；原檔經ZIP內容與SHA256核對後才移除，不重複打包已存在的相同路徑／內容。[最新現況與證據](../../source-review/local-validation-2026-10-30-day44-r2/README.md)。

## Pet forward採用後

[第十四包](14-pet-day44-before-adoption-2026-10-30.zip)封存採用前的候選正文／附件及被取代入口狀態；[逐檔blob／SHA256與去重](pet-day44-adoption-archive.json)。舊候選附件已移除，避免在已採用的main重複套用。前十三包hash未變。[最新採用後證據](../../source-review/pet-day44-adopted-2026-10-30/README.md)。
