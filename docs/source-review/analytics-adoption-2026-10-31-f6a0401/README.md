> 2026-10-31 最新同Day修訂：[Chat r2](https://github.com/williechen/copilot_use/blob/main/docs/teaching/daily/day-45/chat-r2.md) 44項通過；[Analytics r4](https://github.com/williechen/copilot_use/blob/main/docs/teaching/daily/day-45/analytics-r4.md) 67項通過／1 ignored。兩案完整patch直接基於產品f6a0401，已含Refactor，勿與舊patch疊套。Pet沿用原交付；三案next_day=46、未加Day。僅候選驗證，產品未套用、完整Gate2=false。[精確讀回](https://github.com/williechen/copilot_use/blob/main/docs/teaching/daily/day-45/publication-final-rewire.json)。

> **需求解讀更正（2026-10-31）：** 下文「共用修改密碼、不保留獨立頁／ACL page」是助理誤讀，已撤回。正確規則是保留自身密碼ACL page，首次強制修改先顯示該頁，完成後後續登入首頁；功能樹為「會員管理→自身密碼→修改密碼（update有權才顯示）」。以[更正紀錄](../../requirements/revisions/2026-10-31-self-password-page-correction.md)為準。以下原始SQL採用與驗證紀錄仍保留原SHA；錯誤需求解讀僅作歷史追溯。

# Analytics f6a0401 採用核對與共用修改密碼確認

最新產品 `f6a04015188da2155af6c264d0e6326e6a922dca`（modify sql）。相較63b1f93只修改既有`20261031074500_viewer_access.sql`三行，沒有新增runtime接線。原Day45文章與舊來源／測試完整保留；本次不增加Day。

## 本次角色預設的實際變更

- 管理員的`analytics-security` page grant已補上，前次缺少page的問題在此來源SQL定義中修正。新建庫不需要重複套上次的向前補正patch。
- 檢視者不再預設擁有`analytics-accounts-read`，亦不再預設擁有`analytics-password-update`；password-read目前仍在SQL中。
- 認證管理採analytics-token，自身密碼目前SQL仍歸會員管理，管理憑證歸認證管理。這是最新已提交schema的狀態，不表示執行程式已配合新需求。

## 使用者已確認：共用修改密碼，不單獨一頁

本人修改密碼及首次強制改密碼共用同一驗證流程／元件，不另設獨立業務頁或`uac_acl` page，也不要求檢視者取得帳號管理update或另外的密碼page CRUD授權。必須從有效／首次受限Session取得本人身分，不接受指定其他帳號來繞過帳號管理授權；保留目前密碼驗證、確認新密碼、CSRF、密碼變更後撤銷Session與重新登入。

目前產品SQL仍有analytics-password page及其CRUD子action，正式UI也仍有獨立change-password頁；因此**新需求尚未實作**。後續方案需整合共用流程並處理舊ACL節點／路由相容，不是把修改密碼角色grant加回去。管理員管理別人的帳密與權限仍屬管理CRUD授權，與本人修改密碼分開。[共用驗證需求](../../requirements/AUTHENTICATION.md)。

## 真實隔離驗證

專用PG18.6、127.0.0.1:55485、DB adoption_f6a0401；從本精確SHA讀取全部7份SQL，依序以`psql -X -v ON_ERROR_STOP=1 -f`執行，各exit0。raw SQL fresh DB結果：3 category、9 page、36 action；管理員security page=true，viewer accounts-read=false、password-update=false、password-read=true。原始[命令結果](migration.log)、[逐檔exit](migration-exits.json)、[角色矩陣](inspection.log)、[來源blob／SHA256](source-inventory.json)。

本次改了同一migration的內容。如果資料庫已透過SQLx執行63舊版，fresh DB成功不能抵算歷史checksum或舊庫升級通過，需另做向前相容處理。本次未執行使用者DB、SQLx runner、舊庫升級、本地Rust全套或瀏覽器。原64eff527+r3候選62項結果不追認成此SHA結果。

## 採用與交付判定

schema角色預設調整已入產品庫，runtime page／CRUD middleware、bootstrap角色接線等仍未採用。來源CI見[快照](ci.json)，沒有宣稱所有job已成功。產品唯讀；本次只提交需求、採用與驗證紀錄到copilot_use。

舊Day45 r3完整patch只適用64eff527，不可直接套到f6a0401；其中獨立密碼頁的設計也已被最新共用流程需求取代，後續必須版本化修訂。Gate2=false，next_day三案46、FIFO與last_full_comparison不變。首次改密碼的授權問題已依使用者答覆釐清，沒有待確認的角色／個人ACL選項。
