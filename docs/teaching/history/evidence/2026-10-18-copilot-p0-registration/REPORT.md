# copilot_use P0：註冊身分功能試作

程式 `948685fe0692a9e56de94a4ae6a817fdf13fc0a2`，本地分支 `codex/p0-table-trial`，基底 `0bf1d460ea8f20dfaa3bcdaf14b7a895d1b3aacd`；沒有推送／合併main。記錄時間 2026-10-18T21:07:58.642473+08:00。本切片已實作並驗證，**完整P0與整體產品尚未驗收**。rust_ai_chat/main fa0888e的獨立重測保留於[前一份報告](../2026-10-18-full-fa0888e/REPORT.md)，兩庫結果不可互相取代。

## 需求、table與實際接線

採用既有Notion Chat規格：唯一且不分英文大小寫的帳號、唯一Email且不需驗證、註冊完成登出並清除舊選房目標、獨立完成頁、重新登入個人頁。P0 table／field依實際功能可增減；沿用產品既有chat_users／chat_sessions，避免將參考庫uac／mbr表直接覆蓋已有產品資料模型。

新增202610180001_registration_identity.sql：chat_users.email及username／email COLLATE NOCASE唯一索引。新註冊必須提供Email，trim後以ASCII小寫保存，並行重複依DB唯一約束拒絕；帳號保留顯示原文，查詢以NOCASE。既有帳號Email允許NULL，保留原ID與密碼雜湊；舊資料有僅大小寫不同的帳號時migration回滾，不能自動合併、刪除或改身分。

呼叫鏈：Leptos註冊欄位 → POST /api/v1/auth/register（Registration DTO／CSRF／格式檢查／Argon2）→ BEGIN IMMEDIATE → chat_users保存及目前Cookie Session撤銷 → commit＋清Cookie → /registration-complete；登入仍只傳username／password，不需要Email，查NOCASE帳號後驗Argon2、建立Session並導向個人頁。Email語法採常用ASCII非引號信箱，不包含國際化或引號local-part；沒有寄送驗證信，也未假裝已實作密碼重設寄信。

原Trunk入口沒有Rust asset link，已接上同一frontend程式；實際SSR／WASM hydration完成註冊與登入。既有local-verify及稽核HTTP測試的註冊fixture同步新增Email，登入仍保持舊DTO；清Cookie取代原本沒有Set-Cookie的行為，測試改為驗Max-Age=0且原Session不能重用。

## 實際驗證

|檢查|結果|證據|
|---|---|---|
|cargo test --locked -p day9-chat -p chat-ui -p persistence|21通過／0失敗／0忽略；含原稽核交易回滾與公開／私人房間流程|[log](tests.log)|
|真實HTTP／SQLite|16案通過：註冊201、未自動登入401、帳號與Email大小寫重複409、缺Email422、五種無效Email400、大小寫登入200、舊Session撤銷401、兩個並行同Email僅一個201、正規化Email與Argon2讀回|[JSON](http.json)、[script](p0-registration.py)|
|舊資料升級|有效舊帳號ID／hash保留、Email NULL；大小寫衝突升級失敗且原row／schema保留|[JSON](migration-compatibility.json)|
|程序重啟|停止並重啟專用Chat，再登入200，Email與hash保存|[JSON](restart.json)|
|真實瀏覽器|從帶舊選房參數的登入頁註冊→獨立完成頁、登出、URL無舊目標；不同大小寫帳號登入個人管理頁|[完成頁AX](registration-complete.txt)、[登入AX](browser-login.txt)|
|前端／native／品質|release WASM前端與native建置通過；fmt及相關套件all-targets嚴格Clippy通過|[frontend](frontend.log)、[build](build.log)、[fmt](fmt.log)、[Clippy](clippy.log)|

命令：
```sh
cargo test --locked -p day9-chat -p chat-ui -p persistence
cargo build --locked -p day9-chat
NO_COLOR=true bash scripts/build-chat-ui.sh
cargo fmt --all -- --check
cargo clippy --locked -p day9-chat -p chat-ui -p persistence --all-targets -- -D warnings
python3 scripts/tests/p0-registration.py 3111 /absolute/private/fixture.sqlite /absolute/http.json
```

服務僅用本任務work/copilot-p0-db內合成fixture，loopback127.0.0.1:3111；Chat API／服務token全部fixture，Analytics指向59999未啟用的本地端點，OTel停用，未讀正式秘密、未呼叫付費AI、未接觸日常產品資料庫。HTTP腳本會建立固定fixture帳號，重跑須使用新的專用資料庫。真實桌面Linux瀏覽器，不代表跨瀏覽器／手機／全產品E2E。Rust／Cargo1.99.1；實際依賴沿Cargo.lock，沒有改鎖檔。此本地試作commit尚無遠端CI紀錄；本地測試不能冒稱CI成功。

## 尚待實作與精確續接

沿既有需求繼續：舊帳號補Email／變更Email與密碼重設寄信；admin/admin首次初始化與強制改密碼；伺服器保存的遞增等待及剩餘時間；活動Session／密碼或授權撤銷；Pet與Analytics的P0基本流程。本次只完成Chat註冊身分切片，沒有把全部table存在當成整體功能完成。

已有Email資料的舊客戶端需更新註冊request為username／password／email；登入仍username／password。部署前先備份，若NOCASE索引遇衝突須人工確認帳號歸屬，未確認前保留原資料停止升級。此處為實作與升級說明，未增加教學規則。

## 文章與進度

Notion既有Chat入口與共同總頁已記錄產品試作並全文讀回；沒有新增Day、任務或規則。v1正文與原Day17／Day18保留原來源，next_day三專案各19、makeup_queue及last_full_comparison不變。GitHub教學分支存放本報告／patch／證據；程式只保留在本地工作分支，main未合併。
