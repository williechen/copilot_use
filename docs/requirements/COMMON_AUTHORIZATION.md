# Chat／Analytics 共用管理與會員授權邏輯

## 使用者確認的範圍（2026-10-31）

Chat與Analytics的管理功能使用相同的權限邏輯；會員端的mbr_perm／mbr_role_perm也使用「定義不等於授權」的相同分工。共用邏輯不代表共用資料庫、帳號資料或Session，也不代表兩案程式已完成共用接線。

| 責任 | 管理端 | 會員端 |
| --- | --- | --- |
| 權限項目定義 | uac_acl：category／page／action | mbr_perm：只有action |
| 角色定義／指派 | uac_role／uac_user_role | mbr_role／mbr_user_role |
| 角色實際授權 | uac_role_acl(role_id, acl_id) | mbr_role_perm(role_id, perm_id) |
| 個人加減權 | uac_user_acl(user_id, acl_id, acl_filter) | mbr_user_perm(user_id, perm_id, perm_filter) |

管理端的uac_acl定義category／page／action，角色與個人的page及各CRUD action授權分別寫入uac_role_acl、uac_user_acl。會員端mbr_perm只定義action權限項目，角色授權必須寫入mbr_role_perm；現行schema另有mbr_user_perm保存個人加減權，不能把它與定義表混為一談。


## 會員權限只有action等級（2026-10-31 使用者確認）

管理端與會員端共用的是「定義／角色授權／個人加減」分工及有效權限合併規則，**不是相同的權限層級**：

| 範圍 | 權限定義層級 | 受保護操作的檢查 |
| --- | --- | --- |
| 管理端uac_acl | category → page → action | 核對page及該page的action有效權限 |
| 會員端mbr_perm | 只有action | 核對對應action的perm_id有效權限，不要求page／category權限 |

會員角色指派使用mbr_user_role；各角色的action授權寫入mbr_role_perm(role_id, perm_id)，會員個人的action加減寫入mbr_user_perm(user_id, perm_id, perm_filter)。mbr_perm只定義action，不因定義存在而授權。各有效角色的action授權取聯集，再套用個人allow／deny，deny優先；沒有有效授權則拒絕受保護操作。

會員端不建立或推導管理端的category／page ACL層，也不套用「先有page才能有action」的條件。會員修改自身密碼等操作即使共用UI／驗證實作，也只核對會員action，不要求管理端的自身密碼page授權。既有資源範圍、資料所有權與逐房授權仍須檢查；Chat匿名公開清單等已確認流程不因此改變。

本次只補明會員權限層級，不新增schema／權限項目或宣稱runtime已接通，不變更Day與學習進度。


### 會員action範例：建立聊天室（2026-10-31 使用者確認）

「建立聊天室」是會員權限管理中的一個action，不是管理端category或page。以mbr_perm定義此action，mbr_role_perm(role_id, perm_id)保存角色對它的授權，mbr_user_perm(user_id, perm_id, perm_filter)保存個人加減；權限定義存在不代表任何會員已可建立聊天室。

Chat顯示建立入口及處理真正建立請求時，都使用該會員的有效action權限。伺服器在寫入聊天室前須驗有效會員身分與此action；未登入／無權限不能靠直接呼叫API繞過，拒絕時不能留下新聊天室。個人deny優先於角色allow，個人allow可補角色未授予的action。

此action只決定能否執行建立操作；建立輸入、持久化及既有房間管理／逐房資源授權仍按原契約驗證。實際perm_id及資料接線須按schema產製，不新增不存在的欄位。本次確認需求，不宣稱目前產品已接上mbr_role_perm／mbr_user_perm檢查。

## 一致的有效權限合併判斷

1. 由當前系統有效Session取得正確的管理／會員身分，不接受前端任意指定其他user_id。
2. 定義必須有效且適用於目標操作，但「有定義」本身不產生權限。
3. 從該人員的有效角色指派取得角色授權，再套用個人加減權；個人allow可補授權，個人deny優先於角色allow。沒有有效授權預設拒絕。
4. 移除角色授權只移除該角色的對應關係，不刪權限定義。移除個人例外回到角色預設，不等於新增deny。
5. 顯示與操作共用同一份有效權限結果，後端仍須獨立強制檢查；不能因頁面、按鈕或權限定義存在就放行。
6. 帳號停用、角色／個人縮權與Session失效依共同規則重新判斷，不把登入時的權限快照永久當作有效授權。

## 管理功能樹與登入流程一致

Chat與Analytics管理端均由page權限推導category顯示：至少一個可用page才顯示其category；每個page再依自己的action權限顯示／執行CRUD。只有page不會取得所有CRUD，只有action也不會取得其page。

自身密碼保留ACL page，首次需要強制修改時先開該頁；完成後，後續登入才預設開首頁。同一page與修改密碼實作共用，不再建立第二個首次修改頁。若只具自身密碼page，功能樹是「會員管理→自身密碼」，update action有權才顯示「修改密碼」，不連帶顯示其他page。相關安全規則見[共用驗證](AUTHENTICATION.md)。

## Schema與既有流程的界線

相同邏輯不等於表結構相同。已讀rust_ai_ring@f6a04015188da2155af6c264d0e6326e6a922dca的chat_apps/migrations/20261018073931_create_auth_user.sql：mbr_perm目前有id／perm_name／perm_status等欄位，並無uac_acl的acl_category／acl_parent_id／acl_path。會員權限的設計就是只有action等級，不建立category／page權限層；具體action項目仍須沿真實schema與caller接線，不擅自新增或改名欄位。

管理端與會員端各用自己的身分、角色、授權資料及Session上下文；跨案仍由SQLite／PostgreSQL Adapter接入各自DB，不能因邏輯一致而互相信任另一系統的Cookie／權限。Chat公開房間清單、會員註冊與逐房授權，以及Analytics不開放自行註冊等既有需求不因此改變。

本文件是共同需求契約，未宣稱產品runtime、共用程式或管理／會員權限編輯UI已完成；不寫產品庫、不增加Day、不變更next_day、FIFO或last_full_comparison。Analytics專案的category／page配置與採用證據另見[Analytics ACL](ANALYTICS_ACL.md)。

## 必要會員功能：資料修改與密碼修改（2026-10-31 使用者確認）

「修改會員資料」與「修改會員密碼」都是必須提供且納入權限管理的會員功能；兩者是**不同的action**，分開定義、保存授權與核對，不設會員category／page ACL。

| 會員action | 權限定義 | 角色授權 | 個人加減 |
| --- | --- | --- | --- |
| 修改本人會員資料 | mbr_perm的一個action項目 | mbr_role_perm對應該perm_id | mbr_user_perm對應該perm_id |
| 修改本人會員密碼 | mbr_perm的另一個action項目 | mbr_role_perm對應該perm_id | mbr_user_perm對應該perm_id |

能修改資料不代表能修改密碼，反之亦然。不能只因已登入、能開設定畫面或資料屬於本人就省略action授權；必要功能也不表示所有角色自動獲權。各有效角色的action授權取聯集，再套個人加減，deny優先，沒有有效授權拒絕受保護操作。

實際更新前由伺服器從Session取得本人身分並檢查對應action及輸入驗證，拒絕時不寫入資料／密碼。會員個人管理只限本人，不提供管理其他會員資料／密碼的功能；即使增加會員角色或個人action授權，也不能擴大此範圍。若管理端需要操作會員資料，必須使用獨立管理身分、管理API與管理授權，不能沿會員本人修改入口放行。

密碼修改仍保留目前密碼驗證、新密碼確認、加鹽雜湊、CSRF及成功後撤銷Session等共同安全規則。首次強制修改與本人日後修改重用密碼修改實作；初始化／角色指派／個人加權必須確保需首次修改者具有效的密碼修改action。若被deny或缺權限，不可繞過授權或略過首次修改進首頁，應處理權限配置。

這是功能與授權需求確認；實際perm_id、資料接線與UI由後續完整方案及精確產品SHA驗證。本次不新增產品權限資料或宣稱runtime已完成，不更動Day與學習進度。

### 會員只管理本人（使用者補充確認）

會員資料與密碼管理一律以當前會員Session的user_id為對象，不以表單、URL或JSON傳入的其他user_id決定修改對象。即使會員具有資料修改／密碼修改action，也只可修改本人；跨會員請求必須拒絕且不得留下變更。角色授權與個人加減權只能決定是否可執行本人action，不能授予管理其他會員的能力。這不改變聊天室等既有業務資源的獨立授權需求。
