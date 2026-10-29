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
