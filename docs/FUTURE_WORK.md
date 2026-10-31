# 目前待辦與完成條件

本頁只保留接續教學需要的未完成條件與既有識別；診斷歷程、過時失敗與完成紀錄只在 [封存分包](archive/packages-2026-10-30/README.md)。[目前實作／驗證](teaching/current/STATUS.md) 與 [已確認需求](requirements/USER_FLOWS.md) 分開；未因整理新增或結案任務。

<a id="article-api"></a>

## 文章與來源

後續教學沿目前 API、四 workspace、Cargo／features 與正式 caller，保留有效斷言；舊文章只供封存參照。

<a id="chat-auth"></a>

## Chat Session／作者／房主與 ACL

首頁Session入口已存在；2026-10-31在2c7ff8b隔離實測：mbr_exclude排除會員仍HTTP200／WS opened=true，完整逐房授權不通過。WS持續Session重驗不能抵算逐房ACL。仍須完成／驗收原房間登入回跳、會員作者、房主、逐房 HTTP／WS ACL、活動 Session 與原子撤銷競態。

<a id="chat-history"></a>

## Chat 歷史／migration／稽核

歷史讀取與畫面重載須恢復真實已提交訊息；要求的稽核必須落庫。合成舊 schema 升級已測，實際舊庫相容與完整旅程另驗。

<a id="chat-feedback"></a>

## Chat 房間與訊息回饋

目前有自動讀取、Session 顯示、搜尋／重新整理與清除的正式 caller；清除重新讀完整快照，SSR imports 已修。保留空／錯誤／重試、失敗草稿、中文多行輸入、切房與競態的完整瀏覽器驗收，不能沿用舊「完全未接」診斷。

<a id="pet-lifecycle"></a>

## Pet 多寵物／回籠／重啟

使用者已確認：以 `pet_activity` 同主人／同寵物的任一 `is_activity=true` 判出籠，不限定散步；全部為 false／無活動才回籠。建立寵物須同交易建立散步與坐下兩筆活動；散步 true 允許漫遊、坐下 true 仍出籠但不漫遊。兩動作切換互斥更新；同時 true 時坐下優先並寫回散步 false，出籠不重複插入同名活動，回籠停止全部活動；不新增狀態或漫遊欄位。`is_default_closure` 僅保留預設語意，不作目前狀態。

目前2c7ff8b已將控制台、Store、Native、關窗及啟動caller切換活動模型，本地Pet112通過。舊中文活動值forward與升級example已在main；[Pet採用後正文](teaching/daily/day-44/pet.md)記錄主分支112、Node12、WASM與實際SQLite唯讀備份副本升級通過，Trunk本輪未重建；保留既有migration checksum。本輪同SHA以私有Xvfb真native跑通三隻、Native X、DB寫入失敗保留、實際位移／坐下、睡覺／醒來與兩次重啟等25步；實機多螢幕／右鍵／縮放及真OS開窗故障仍待驗。舊A/B不追認新模型。

<a id="pet-roaming"></a>

## Pet 逐 ID 漫遊

逐隻生命週期啟停、回籠停活動、拖曳暫停／恢復、多螢幕與手動找回按需求驗收；不將先前固定 pet label／空白畫面診斷當目前全部仍故障。

<a id="pet-display"></a>

## Pet 外觀與生活窗

200×200 等比動畫、一窗一寵物、名稱／狀態／按鈕在透明與淺背景可讀；每幀單圖、素材及外觀沿既有 FIFO。

<a id="analytics-auth"></a>

## Analytics 人員與來源授權

正式查詢採人員Session，POST攝取另驗服務身分。2026-10-31實際Dashboard仍明示角色／來源ACL尚未接通，完整授權不能列為已驗收。角色、Chat／Pet 來源 ACL、停用／縮權立即生效與 SSE 授權仍按需求逐項驗收。

<a id="analytics-session"></a>

## Analytics 登入／Session

目前已有管理員持久化、密碼流程與 DB Session。Day44 logout CSRF已在main，新測試真PG通過；舊測試fixture修補已在4b1008b且真PG63通過。正式登入等待／鎖定、活動／閒置 Session、改密碼與權限變更撤銷、首次改密碼旅程須以正式路由驗收。

<a id="adapter-cleanup"></a>

## D01–D09 與原 13 項清理

沿原識別與相依整理；先承接 caller 與有效斷言再清理，不恢復平行產品或重教已實作項。

<a id="ci-coverage"></a>

## CI／產物／安裝

最後來源4b1008b已採用Analytics兩個fixture修正，真PG63通過；CI依[STATUS](teaching/current/STATUS.md)的精確SHA與時間分列success／pending／cancelled。綠燈不代表完整三平台安裝或native GUI。

<a id="event-transport"></a>

## 來源事件與服務配對

真來源事件→可靠投遞／Outbox→PG commit ACK→人員查詢／SSE，需重試去重、rollback、配對／短效憑證輪替與撤銷；DTO 或 helper 存在不算完成。

<a id="pet-capabilities"></a>

## Pet 設定／記憶／檔案／AI

五欄設定、素材、記憶、檔案授權→掃描→確認→OS 操作→收據→還原，以及三平台更新沿原需求；預覽不等於真清理，付費 AI 不自動呼叫。

<a id="analytics-capabilities"></a>

## Analytics 報表與跨服務能力

保留精確類型查詢、來源切換保留條件、CSV、Token／成本／跨服務需求；角色／來源權限與服務配對不可由畫面隱藏替代。

<a id="chat-capabilities"></a>

## Chat 即時與 AI 能力

presence、typing、reconnect／queue、streaming、RAG、MCP、Agent、Persona、同意撤回與窄螢幕 UI 依原產品需求及 P0–P12 相依推進。

<a id="layout-guidance"></a>

## 來源與套用

只依最新 rust_ai_ring 真實目錄與 SHA 交付；Day43 原 patch 基準已過時且產品已有實作，不重複套用。新切片與未套用方案重新核對 caller／features／migrations。

## 原 13 項清理範圍

下表保留原編號、位置與必要保留條件；執行前按目前來源核對，舊版測試數與舊 SHA 成敗已移至封存，不當作本次完成量。

| 專案／識別 | 調整範圍 | 保留與判準 |
| --- | --- | --- |
| Chat 1／D01 | 維護單一 SignInPage 與 GET／POST login 正式契約；不重建登入頁 | 保留 Session、SSR／hydrate 與正式登入旅程斷言 |
| Chat 2／未使用函式 | 盤點私有 room_api::create_room 與只供它使用的 DTO／imports | 保留 list_rooms／search_rooms、server JSON API 與個人頁建房；先核對 caller |
| Chat 3／D06 | 目前 RoomList 使用 refresh_result_view；舊 refreshed_room_list 已未找到，不重做已完成的 helper 移除 | 保留空結果、失敗保留清單／query、成功與重試的正式 UI 測試 |
| Chat 4／D05、D07 | 房名驗證已進 Application；認證 Cookie／digest 共用邊界沿 FIFO 接續 | 保留 JSON／HTML 錯誤、Session／Origin／CSRF、控制字元與 UTF-8 邊界 |
| Pet 1／D02 | 維護 source bridge 單一來源與新 dist／native 產物契約 | 不手改雙份程式；建置／上傳與實際安裝、三窗、重啟／更新分開驗收 |
| Pet 2／未使用宣告 | 盤點 validate_new_pet_name_js extern 的實際 wrapper／JS caller 後清理 | 保留正式 PetStore 名稱規則及合法／非法名稱的 DB 行為 |
| Pet 3／D03 | 盤點 legacy get_pet_state／send_pet_command、global state／event 與舊選單 | 不是零入口；先承接相容客戶端、逐 ID Runtime 與有效斷言再刪 |
| Pet 4／D08 | 盤點掃描 handler 的 reset／loading setter | 成功／失敗／取消／過時 generation 皆須釋放 busy，不刪唯一必要清理 |
| Analytics 1／D09 | 盤點 Application／Domain／UI 的 add／it_works 範本 | 保留既有 workspace，不另加空架構；移除前核對外部 API 與 caller |
| Analytics 2／D04 | 正式 build_app 使用 password_login；盤點 legacy admin／memory Session 入口 | 確認相容 caller 後才清模組出口，不刪正式帳密／Session／PG 測試 |
| Analytics 3／測試 fixture | 保留測試仍使用的 bootstrap::initialize；遷移完整後才評估移除 | main 未呼叫不代表無 caller；保留密碼、Session、Dashboard 整合 fixture |
| Analytics 4／測試 handler／查詢 | source_summary 為 cfg(test)，正式精確查詢採人員 Session 與 persistence | 沿正式路由保留角色／來源 ACL，勿新增過時 pg_store re-export 或重做已修編譯錯誤 |
| Analytics 5／斷言維護 | 已修正的事件總數文案斷言保持新語意，不再當成待修敗項 | 保留來源切換、HTTP／Session、連結、可及性、escaping 及秘密不外洩斷言 |

補教依 learning-state.makeup_queue 的既有 FIFO；主體 Red／Green 及同案 Refactor 按既有相依前進，跨案工程只在 [engineering](engineering/README.md)。保留來源測試有效斷言；任何舊原文若有疑義，從分包讀回比對。

[Pet 2c7ff8b採用後最新驗證](source-review/pet-day44-adopted-2026-10-30/README.md)；其他案測試保留原4b1008b來源，不追認新SHA。

[本輪Gate2實測、截圖與失敗重現](source-review/gate2-2026-10-31/README.md)。以上回填原功能，不新增Day／FIFO或以部分通過結案。

[64eff52 SQL更新後最新回歸](source-review/gate2-sql-2026-10-31/README.md)：Pet112／Node12／WASM及四workspace metadata通過；舊native／browser證據仍屬2c7ff8b，完整Gate2未通過。

<a id="scheduler-delivery"></a>
## 教學排程：啟動成功但未交付

2026-10-31既有雲端任務兩次手動執行均未建立Day45文章或commit；回合completed／error=null不等於交付成功。提示詞已保存、重新載入全文讀回且第二次收到相同內容，仍因來源／fixture核對未完成而提早結束，未提供具體工具錯誤。**未結案**：需真正完成逐案Gate1交付，或取得可處理的明確執行阻擋；不能把Gate2缺口推定為排程根因，不調整Day／FIFO。[診斷與原始結果](source-review/scheduler-2026-10-31/README.md)。

Analytics ACL原項目最新採用：[63b1f93 schema-only／runtime待接線／security page grant向前補正](source-review/analytics-adoption-2026-10-31-63b1f93/README.md)。不是新增重複功能項，next_day與週比對日期不變。

Analytics原ACL／登入項目續記：f6a0401已補security page預設grant；viewer角色縮權按新SQL；自身密碼保留ACL page；首次先開該頁，完成後後續登入開首頁；實作待對齊與驗證。[採用與新需求](source-review/analytics-adoption-2026-10-31-f6a0401/README.md)。不另增Day或重複功能清單。
