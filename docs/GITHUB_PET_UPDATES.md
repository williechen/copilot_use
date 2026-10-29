# 舊庫 Pet 發行與更新的適用範圍

現行程式來源與目錄：[rust_ai_ring 四個 workspace 對照](SOURCE_LAYOUT.md)（來源 `0b3e5844`，2026-10-24 靜態核對）。

`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_ring` 是唯一程式、測試、CI 與功能驗收主體。 rust_ai_ring 的安裝／更新能力只依該庫實際程式、版本、來源及簽章契約驗證，不把舊庫 Release 或更新工具作為其更新來源。

原 copilot_use 的發行資產、固定 tag、簽章、公鑰與安裝資料[完整原文保留](teaching/history/evidence/2026-10-23-repository-roles/before/docs/GITHUB_PET_UPDATES.md)，不得改寫成另一套來源的安裝包。既有 github-pet-update.py／release-approved.py 及 workflow 尚未調整，本次不執行更新、搬移 Release 或替換信任來源。

後續若需修改主體安裝／更新流程，須先核對 rust_ai_ring 的平台、版號、簽章、產物及資料相容性並提供實際證據；不能只改 repository 字串或重新標籤既有包。

[剩餘流程設定](REPOSITORY_ROLE_TRANSITION.md)
