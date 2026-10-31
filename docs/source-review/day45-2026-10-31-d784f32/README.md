# 最後追驗：產品 d784f32c

本回合驗證期間使用者已提交`d784f32c6bff86249dd84fd4294d055a4c774a49`，只修正Pet browser測試錯字；[樹SHA／實測／patch核對](latest-source.json)。Chat與Analytics子樹及lock與28逐位元相同。仍重新跑Chat：44通過；Pet：UI11、browser六pose均通過；Analytics postgres-integration --no-run仍因ootstrap匯入失敗。68通過／1 ignored是基於28的**修正候選**結果，沒有改標成d784產品原樣通過。

Pet修正已採用，**不要再對d784套pet-browser-fix.patch**；該patch及下面28失敗紀錄保留追溯。Analytics三份修正patch已對d784做git apply --check，皆0，適用DB條件不變。latest CI Persistence仍失敗，其餘狀態僅快照，不把部分成功當完整Gate2。

以下是本回合先前28精確來源的完整核對、Red與修正候選證據。

---

# Day45 產品採用核對與修正｜2026-10-31

產品精確SHA：`28b71b67907062f804b2358c41fde349c9b18ca9`；文件基準`e06efc9cc38945aca01fe8af621eb8d3430df07b`。只寫copilot_use，產品庫唯讀；以下產品原樣測試與修正候選分列。[命令／exit／結果](review.json)、[來源blob／SHA256](source-inventory.json)、[當次版本](versions.json)、[暫存清理](cleanup.json)。

| 專案 | 產品28原樣 | 修正候選 |
| --- | --- | --- |
| Chat | 44測試通過、WASM check通過 | 無功能修正；原Day45已採用，勿重套f6完整patch |
| Pet | UI11通過；browser fixture因錯字TypeError退出1 | 六pose通過；越界負向控制以pose clipped退出1 |
| Analytics | full test因ootstrap匯入編譯失敗；獨立day45_final 1通過／4失敗、登入503 | runtime-only五項HTTP旅程通過；完整修正乾淨套用68通過／1 legacy ignored |

## Pet：完整Red／Green修正

[原樣失敗](pet-browser-product.log)：`result.image.xresult.image.width`導致TypeError，沒有真正完成六相檢查。將它改成`result.image.x + result.image.width`；[完整patch](pet-browser-fix.patch)、[Node26真六pose](pet-browser-node26.log)、[越界反例](pet-overflow-negative.log)。反例只在獨立fixture附加translateX(100px)，不修改產品CSS；須以pose clipped失敗，不能拿TypeError當有效越界斷言。這仍是使用產品圖／CSS的DOM fixture，不是Leptos hydration或Tauri視窗驗收。

## Analytics：完整Red／Green與兩種DB路徑

原樣錯誤包括`ootstrap`匯入、PAGE_SQL的uac_role缺`r`別名，以及password_login.rs三行誤留字面`+`。分別有[編譯原始錯誤](analytics-product.log)、[HTTP503失敗](analytics-product-targeted.log)。[Runtime修正](analytics-runtime-fix.patch)只改三個Rust檔，不改migration；[五項HTTP測試](analytics-runtime-only.log)驗page/category/action、deny、首次改密碼與viewer旅程。舊升級測試在此命令明確skip，因為它從目前目錄複製舊migration，無法證明真正f6相容性。

28另在既有20261031074500 migration加入角色管理`analytics-roles`、頁面管理`analytics-acls`。這兩個page及各CRUD定義與admin grant須保留，但真正的f6 SQLx ledger升級會`VersionMismatch(20261031074500)`：[使用凍結f6原文的Red](analytics-frozen-upgrade-red-retry.log)。因此不能把新版fresh DB成功當作f6舊DB升級成功。

- **空DB或ledger仍為f6舊checksum：** [完整修正patch](analytics-complete-correction.patch)＝Runtime修正＋[migration／fixture修正](analytics-f6-upgrade-fix.patch)。保留f6舊migration原位元，以新增20261031180000_role_acl_pages.sql保存兩個新page／八個action／十個admin grant；不恢復viewer角色password-update預設，不刪個人deny。凍結f6 fixture重驗七→九份SQLx migrations、再次執行與checksum保留，仍有3 categories／11 pages／44 actions。
- **已執行28改寫版migration的DB：** 先用runtime-only修正；**不可直接套回復舊migration的完整修正到該DB**。新增測試證明SQLx會拒絕不同checksum且ledger不被改寫。需另依真實DB備份決定明確的migration reconciliation；本次沒有真實備份，不自動UPDATE _sqlx_migrations、不偽稱已升級。

所有patch只針對精確28來源；[乾淨套用及逐檔相等](patch-validation.json)。不要與舊64／f6完整patch疊套。完整修正patch與runtime／migration分件擇一，不可重複累加。角色／頁面管理及管理憑證的定義存在，不代表對應管理UI／API已實作。

## 獨立Refactor與驗收界線

Chat Session SQLite Adapter、Pet pose投影及Analytics PG Session Adapter都已採用；本次沿用原各案獨立Refactor，不重複計功，不修改跨案crate或全庫CI。三案next_day仍46、FIFO及last_full_comparison保留。核心60分鐘不含本次版本、驗證、修訂與清理。

Gate1補齊來源、錯誤、完整patch、真實命令與讀回證據；完整Gate2仍false。產品Analytics與Pet原樣存在上述失敗，不因修正候選通過而改成產品通過。CI快照另外列存；CI名稱或部分成功不能抵銷實際postgres-integration及browser失敗。
