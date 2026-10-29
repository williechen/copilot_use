# Pet 教學現況

目前 Day44 修訂中：以既有 `pet_activity` 判斷目前出／回籠，同主人／寵物任一活動 `is_activity=true` 即出籠，全部為 false／無紀錄才回籠，不限散步，不新增欄位。回籠停止該寵物全部活動，不影響其他寵物；預設關籠設定與當前活動分開。

原新增 auto_release_on_start 的 Green 已撤回封存，替代 Green 尚未完成；獨立 Refactor 保留。產品仍是舊旗標模型，未驗證新契約。next_day 維持已發布後的 45，先完成 Day44 修訂，再接續新 Day，不重開 FIFO。

[Day44 規則與修訂](../daily/day-44/pet.md) · [目前驗證](../current/STATUS.md) · [完成條件](../../FUTURE_WORK.md#pet-lifecycle) · [共用規則](../../COMMON_LEARNING_RULES.md)
