# Analytics 角色與階層 ACL

## 使用者已確認的契約（2026-10-31）

- `uac_role` 區分角色，`uac_user_role` 指派人員角色。
- `uac_acl` 是階層：**category → page → action**，以既有 `acl_parent_id` 表示父節點。
- category 包含 **會員管理、資訊分析、認證管理**。
- action 判斷 **CRUD：create（新增）、read（讀取）、update（修改）、delete（刪除）**。
- `uac_role_acl` 提供角色預設權限；`uac_user_acl` 可獨立增減個人的功能／頁面權限，不能被硬式角色判斷蓋掉。

```text
category：會員管理
├─ page：帳號管理 → action：create/read/update/delete
└─ page：自身密碼 → action：依該page的有效CRUD權限顯示
category：資訊分析
└─ page：分析後台、Chat來源、Pet來源、彙總查詢、類型查詢、CSV匯出（各自有CRUD action）
category：認證管理
└─ page：管理憑證 /security → action：create/read/update/delete（目前只有schema）
```

這是目前候選的頁面配置，並未宣稱所有管理頁面或CRUD API均已實作；角色、ACL與個人加減權的編輯介面仍須接續。

## 本修訂採用的授權規則

1. 先驗有效人員Session與帳號狀態；服務憑證不代替人員登入。
2. 找唯一的page與其指定CRUD action，驗證完整三層、父節點及啟用狀態。缺父節點、循環、目標action／page重複或停用節點均拒絕。
3. page與action各自滿足「角色預設allow 或 個人allow」。只有page權限不能取得其全部CRUD；category是分組容器，allow不自動授予下層操作。
4. 個人對category、page或指定action的deny優先於所有角色與allow；category／page deny會關閉其下層操作。只減掉delete，不影響另有授權的read／create。
5. 每個請求重新查有效權限；本次帳號新增會在交易內再驗一次；其他既有寫入仍須逐項核對ACL與交易邊界。這保證後續請求反映縮權，不保證跨請求／正在進行的交易全部同步取消。
6. 由伺服器固定映射operation：GET／HEAD→read；POST /accounts→create；POST /change-password→update；DELETE /accounts分類為delete，但目前Router未實作此方法，仍回405。沒有實作的update／delete路由不因ACL seed存在而自動開放；未知操作拒絕。

`acl_category` 存category／page／action種類；page的`acl_path`為正式頁面／路由模板，action的`acl_path`為四個操作鍵之一；action不能只憑全域名稱授權，必須連同自己的page父節點判斷。人員新增帳號只取得既定viewer角色與選定來源page，不自動得到create／delete等業務操作。

## 方案、套用與驗收

[Day45 r3全文與完整patch](../teaching/daily/day-45/analytics-r3.md) 以 rust_ai_ring 精確來源為基準；產品保持唯讀，由使用者套用。舊r1／r2文章、patch與證據保留。此需求修訂不增加Day，不變更FIFO或last_full_comparison。

## 使用者已採用schema的頁面配置

63b1f93使用analytics-token作認證管理ID；自身密碼page移至會員管理，管理憑證security page歸認證管理。採此來源配置接續方案，不用舊候選的analytics-auth／密碼歸屬覆寫。[採用核對](../source-review/analytics-adoption-2026-10-31-63b1f93/README.md)。schema入庫與runtime實作／DB執行分別判定。

## 最新更正：自身密碼保留ACL page

自身密碼保留為 ACL page。首次登入需強制修改時先顯示自身密碼頁，完成修改後，後續登入才預設顯示首頁；未完成者重登仍須先完成，不以單純登入次數繞過。

首次與日後本人修改使用同一個自身密碼 ACL page及共用元件／後端流程，不再另建第二個首次修改頁。保留目前密碼驗證、新密碼確認、CSRF、成功後清除失敗等待及撤銷所有 Session 再登入；身分取自 Session，不可藉此修改其他帳號。

```text
會員管理                 ← category，至少一個可用page才顯示
└─ 自身密碼              ← 保留的ACL page
   └─ 修改密碼           ← update action，有權限才顯示
```


若只有自身密碼page權限，不連帶顯示帳號管理等其他page；category由page權限推導，action依所屬page與自己的CRUD權限顯示。首次／後續登入只決定預設開啟內容，功能樹仍按ACL顯示。需要首次強制修改的人員必須能完成此page必要操作，不能被唯讀設定擋住，也不因此取得管理其他帳密的權限。

先前移除analytics-password page／CRUD的文字是助理誤讀，已撤回。[安全規則](AUTHENTICATION.md) · [更正紀錄](revisions/2026-10-31-self-password-page-correction.md)。

## 顯示關係：以page為起點（2026-10-31 使用者確認）

資料儲存維持category → page → action與既有acl_parent_id；畫面顯示的判斷方向為：

```text
page ─→ category：至少有一個具有效page權限的子頁，才顯示該category
     └→ action：依該page下各action的有效權限，分別顯示CRUD操作
```

- 先取得目前人員具有效權限且啟用的page。有效page權限合併角色預設與uac_user_acl個人加減權，deny優先，並遵守父節點啟用／拒絕規則。
- 以這些page的category關係產生類別；同一category有多個可用page時只顯示一次，沒有任何可用page就隱藏，不保留空分類。
- action顯示必須以其所屬page為範圍；page不可用時不顯示其action，可用page下的create/read/update/delete各自核權，不因page或category顯示而全部放行。
- category顯示的條件是page權限，不以「只擁有category節點」或「只擁有某個action」代替，也不額外要求先取得某個CRUD action才顯示category。
- 角色或個人權限更新後，重新取得有效權限並重算page→category與page→action。減去最後一個可用page後，其category須隱藏。
- 顯示只是UI投影；後端page與CRUD action授權仍必須執行，不能用選單／按鈕隱藏取代伺服器檢查。

| 有效權限情況 | category顯示 | action顯示 |
| --- | --- | --- |
| 沒有任何子page權限 | 隱藏 | 不顯示該分類下的page操作 |
| 至少一個子page有權限 | 顯示一次 | 僅顯示該page獲授權的action |
| 只有category或action權限，沒有page權限 | 隱藏 | 隱藏 |
| page有權限，但某個CRUD action被deny | 顯示 | 隱藏被deny的action，其餘逐項判斷 |
| 最後一個可用page被減權 | 重新計算後隱藏 | 隱藏該page的action |

這是已確認的顯示需求，尚未宣稱產品UI或runtime已實作；不新增Day、不改next_day、FIFO或last_full_comparison。

## CRUD action也寫入uac_role_acl（2026-10-31 使用者確認）

`uac_role_acl`不只記錄角色可用的page；角色獲授權的create/read/update/delete **action也必須各自寫入此表**。每筆以`role_id + acl_id`對應，其中`acl_id`指向所屬page下該操作的`uac_acl` action節點，不能只在UI保存勾選或以程式硬編角色權限。

| 資料 | 存放位置 |
| --- | --- |
| page與其CRUD action定義、父子關係 | uac_acl |
| 角色對page的授權 | uac_role_acl，acl_id指向page |
| 角色對各CRUD action的授權 | uac_role_acl，acl_id分別指向該page的action |
| 個人對page／action的獨立加減權 | uac_user_acl |

例如某角色獲准使用自身密碼page及update操作，需分別存在「該角色→analytics-password」與「該角色→analytics-password-update」兩筆關係。這只是結構示例，不表示自動對任何角色加權，也不表示有page即授予全部CRUD。

角色權限編輯時，勾選某CRUD即新增對應的uac_role_acl關係，取消則移除該角色對該action的關係；不刪除uac_acl定義，不影響其他角色與既有個人例外。相同操作名稱在不同page下是不同action，不可跨page套用。

有效權限仍合併角色預設與uac_user_acl個人加減，deny優先。功能樹維持page→category、page→action；UI與後端必須讀取保存後的權限重新判斷。本次僅補明需求，不宣稱產品接線或權限編輯UI已完成，不增加Day或變更學習進度。

## 個人CRUD action也寫入uac_user_acl（2026-10-31 使用者確認）

`uac_user_acl`不只保存個人對page的加減權；個人對該page下各create/read/update/delete **action的加減權，也必須各自保存於此表**。每筆以`user_id + acl_id`指定人員及目標ACL節點，`acl_filter`保存allow／deny；action的acl_id指向其所屬page下的uac_acl節點。

例如某人對自身密碼page的個人權限與update操作的個人權限，是「user_id→analytics-password」及「user_id→analytics-password-update」兩筆獨立關係。這只是資料結構示例，不自動替任何實際人員加權。

- 個人取得page不表示自動取得全部CRUD；個人取得某action也不表示自動取得其page。
- 新增個人allow可補上角色未授予的指定page／action；個人deny可減去角色已有的指定page／action，deny優先。
- 移除個人例外關係表示回到角色預設，不等同新增deny。若要明確拒絕角色已授權的action，須保存該action的個人deny。
- 個人CRUD設定不能只存在UI勾選、暫存狀態或角色表；保存後須讀回uac_user_acl，與uac_role_acl合併判斷。
- 同名CRUD在不同page下是不同action，不可跨page共用授權；修改個人例外不覆寫角色預設或其他人的權限。

因此，角色與個人的page／action都保存對應acl_id：角色使用uac_role_acl，個人加減權使用uac_user_acl。功能樹仍依page→category、page→action推導，後端同樣核對有效權限。本次是需求補明，不宣稱權限編輯UI或runtime已完成，不變更學習進度。

## 定義與權限實作分工（2026-10-31 使用者確認）

**uac_acl只是定義；權限實作／授權資料由uac_role_acl與uac_user_acl承擔。**

| 表 | 責任 | 不可推論 |
| --- | --- | --- |
| uac_acl | 定義category／page／action、名稱、路徑、父子關係及節點狀態 | 有定義或節點啟用，不代表任何角色／人員已獲權限 |
| uac_role_acl | 保存角色對已定義page與各CRUD action的授權關係 | 有page關係，不代表同時取得全部CRUD action |
| uac_user_acl | 保存個人對已定義page與各CRUD action的allow／deny加減權 | 個人加減權不覆寫ACL定義或其他角色／人員資料 |

伺服器以uac_user_role取得人員角色，從uac_role_acl合併角色授權，再套用uac_user_acl個人加減權，依既定deny優先規則判定有效權限。不存在有效授權時預設拒絕；不能只查uac_acl存在、節點啟用或其category來放行。有效授權仍須對應有效且結構正確的ACL定義；節點停用不會因既有授權關係而變成可用。

功能樹與操作檢查使用相同的有效權限：由獲授權page推導category，並依該page各action的授權顯示／執行CRUD。資料定義、選單顯示與後端授權不能混為同一件事。本次僅補明契約，不宣稱產品授權接線已完成，不更改Day或學習進度。

## Chat／Analytics管理與會員權限的共用邏輯

本文件的管理授權規則同時適用Chat與Analytics；Analytics特有page／來源配置與採用紀錄仍各自保留。會員端mbr_perm定義權限，mbr_role_perm保存角色授權，mbr_user_perm保存個人加減權，分工與管理端一致；不能從定義存在推論已授權，也不假設mbr_perm已具有uac_acl的階層欄位。[共同契約](COMMON_AUTHORIZATION.md)。

會員層級補充：上述category／page／action及page→category／action顯示規則只適用管理端。會員mbr_perm只有action，mbr_role_perm與mbr_user_perm也只保存action授權／加減；不要求會員具備管理page權限。[共同契約](COMMON_AUTHORIZATION.md)。
