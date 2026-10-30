# Day45 文章、附件與套用

[ENTRY](ENTRY.md) · [Chat](chat.md) · [Pet](pet.md) · [Analytics現行r2](analytics-r2.md) · [讀回證據](delivery-verification.json)

產品唯讀來源64eff527283fd05657841297879aefeaadd31f64；所有patch只適用此來源，使用者在隔離副本套用。各案互相獨立，先Refactor再選一份Green；做Red時先只套Refactor＋Red，完成後reverse Red再套完整Green。

| 專案 | Refactor | Red | 完整Green |
| --- | --- | --- | --- |
| Chat | [patch](chat-day45-refactor.patch) | [patch](chat-day45-red.patch) | [patch](chat-day45-green.patch) |
| Pet | [patch](pet-day45-refactor.patch) | [patch](pet-day45-red.patch) | [patch](pet-day45-green.patch) |
| Analytics | [patch](analytics-day45-refactor.patch) | [patch](analytics-day45-red.patch) | [r2 patch](analytics-day45-green-r2.patch) |

Analytics r2 Green完整替代r1，不能兩份相接；若已在別處執行r1 migration，須另做向前修訂而非改checksum。本次產品main並未套用任一版。原[analytics.md](analytics.md)及原patch／日誌保留，只讀追溯。

候選驗證：[Chat](chat-validation.json)30通過；[Pet](pet-validation.json)UI11、六pose DOM通過及[最新工具補查](pet-version-addendum-r1.md)；[Analytics r2](analytics-validation-r2.json)真PG60通過、1既有legacy ignored。來源CI成功不抵算這些候選已套用或完整產品驗收。未執行項目逐篇詳列。

核心60分鐘只計主要Red／Green；Refactor、版本、工具準備、提交／讀回／同步、產品驗收另計。全庫工程只見[engineering](../../../engineering/README.md)。
