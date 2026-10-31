# Analytics 教學現況

產品來源 `63b1f936b32f131f0f801bb4f36b3163f88c91fa`；本次只新增Analytics階層ACL migration，正式runtime尚未接線。隔離raw SQL確認3 category／9 page／36 action；完整Gate2未通過。來源CI以最新採用紀錄快照為準。

[目前狀態](../current/STATUS.md) · [需求](../../requirements/USER_FLOWS.md) · [規則](../../COMMON_LEARNING_RULES.md) · [來源](../../SOURCE_LAYOUT.md) · [架構](../../SYSTEM_ARCHITECTURE.md)

目前教學：Day45三案獨立發布與全文讀回完成，next_day三案46。

[Day45 ENTRY](../daily/day-45/ENTRY.md) · [Chat](../daily/day-45/chat.md) · [Pet](../daily/day-45/pet.md) · [Analytics r3](../daily/day-45/analytics-r3.md) · [讀回證據](../daily/day-45/delivery-verification.json)

[manifest](../current/manifest.json) · [learning-state](../../learning-state.json) · [各案Refactor](../functional-refactoring/PROGRESS.md) · [跨案工程](../../engineering/README.md) · [歷史分包](../../archive/README.md)

ACL契約：category（會員管理、資訊分析、認證管理）→page→action（CRUD）；角色預設與個人加減權分別判斷。詳見[需求](../../requirements/ANALYTICS_ACL.md)。

[Analytics新來源採用核對與向前補正](../../source-review/analytics-adoption-2026-10-31-63b1f93/README.md)。Day45原r3附件只適用舊64eff527，不可直接套用新63b1f93。
