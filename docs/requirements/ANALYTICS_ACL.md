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
└─ 本人修改密碼：共用驗證流程，不另列page／CRUD ACL
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

## 最新確認：移除獨立自身密碼page的設計

使用者確認「共用修改密碼，不單獨一頁」。因此analytics-password與其CRUD不是目標ACL節點；本人／首次修改走共用驗證流程，不需viewer取得帳號管理update。管理他人帳密仍按管理page/action授權。目前f6a0401 SQL尚有此舊節點，後續需作版本化相容處理，不能把需求確認冒稱已從產品移除。[安全規則](AUTHENTICATION.md)。

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
