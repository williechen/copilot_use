<mention-page url="https://app.notion.com/p/3f2e51d4509381ddb8a8ce7cb4cd3e60">Rust AI 三專案｜共同學習規則與前進目標</mention-page>
本頁說明本專案的功能、操作主線、當前缺口與證據入口；每日完整程式與實作步驟放在既有 Day。現行教學方法、時程、四節格式、發布讀回、條件推進及失敗恢復，統一引用上方共同總頁「每日 TDD 教學最小必要規則 v1」（daily-tdd-minimum-v1／2026-10-18）。
Day18 三篇已在既有頁發布並讀回；learning-state 的 next_day 各為 19。本次正式複驗與文章修訂沒有增加 Day。98f30d8／next_day=17、18 等舊觀察保留歷史身分，目前實作與驗收以本頁第3、4節及 Day18 為準。
## 1. 專案大綱與操作主線
將 Chat 與 Pet 的真實動作轉成可追溯統計，供使用者查詢、篩選與管理。Analytics 提供觀察與建議，Chat／Pet 是否採用仍須各自授權。
主線：真實動作 → 來源認證 → Schema 驗證 → 去重／衝突處理 → PostgreSQL 提交 → Dashboard 查詢／篩選 → 趨勢與明細 → 管理／封存／備份。資料提交與畫面出現是不同步驟，要各有證據。
## 已逐項確認的現行需求（2026-10-17）
以下 16 組需求經使用者逐項核對；屬現行要求，未宣稱已實作或驗收。網頁／本地排程每次重新讀取並採用；與下方歷史來源觀察或封存 Day 不同時，以本節作為後續需求，歷史證據保留原 SHA 與範圍。Day15／Day16 保持封存，不新增 Day，next_day 維持三專案各 17。
### Analytics：帳號、授權、稽核與來源
- 僅首次初始化建立 admin/admin；首次登入強制改密碼並設定唯一 Email，之後可修改密碼。不開放自行註冊；由系統管理者新增帳號、初始密碼與唯一 Email，新帳號首次登入須改密碼，之後可修改或以 Email 重設。密碼加鹽雜湊持久保存，重啟不重設。
- 採系統管理者及唯讀檢視者兩角色；管理者管理帳號、來源與設定，可授權檢視者查看 Chat、Pet 或兩者。
- 除登入及必要的密碼重設流程外，所有業務頁面、查詢／管理 API 與即時資料由伺服器檢查人員 Session、角色及來源範圍。帳號停用或權限縮減立即影響既有 Session 與即時連線，不等待重登。
- 帳號、權限、來源憑證及資料管理操作記錄操作者、時間、對象與結果，不保存密碼、重設連結或憑證秘密。
- Chat／Pet 各用獨立服務身分與權限，採管理者核准配對 → 短效憑證 → 自動續期／輪替 → 可撤銷。服務憑證不能代替人員後台登入。
### Chat 與 Analytics：各系統獨立的人員登入保護
- 連續登入失敗次數以各系統的帳號累計，由伺服器保存；等待自 15 秒起逐次加倍（15、30、60、120 秒……），不設上限，重新整理、換瀏覽器或 IP 不重設。失敗保留原登入導向。
- 等待期間再次送出時拒絕驗證並回傳剩餘時間；不增加次數、不重新起算倒數。登入成功清除失敗次數及等待時間。
- 管理頁修改密碼須驗目前密碼；忘記密碼則寄重設連結至登記 Email。修改或重設成功均清除失敗次數與等待時間，使該帳號所有 Session 失效，須重新登入。
- 登入後持續操作會更新活動時間，Session 持續有效；閒置逾時由各系統的管理後台設定。不能只因登入後經過固定時間而強制登出；密碼變更、停用與授權撤銷仍須生效。
## 2. 功能目標與完成條件
Analytics 定位為純管理後台，Leptos 提供後台畫面；除登入及必要的身分驗證流程外，所有業務頁面、查詢、管理與即時資料均須登入且具相應權限，不提供匿名 Dashboard。管理者 Session 與 Chat／Pet 服務憑證分開：服務上報／換證驗證機器身分，不以人員登入或來源 token 取代後台登入。
- 攝取與持久化：相同來源／事件 ID 重送不重複計數，內容衝突明確拒絕；失敗不能回成功 ACK，重啟後仍可查。
- Dashboard：選來源與事件類型、載入／空資料／錯誤／重試、數量與明細；不同來源資料與授權隔離，交易未提交不得先通知畫面。
- 通訊與資料：產品攝取目標為 gRPC／Protobuf 與逐事件 ACK，Dashboard 使用 HTTP JSON＋SSE；學習庫既有 HTTP 相容入口保留。Analytics 獨立 PostgreSQL，不讀寫 Chat／Pet 的業務 SQLite。
- 管理與洞察：封存／還原／備份、可追溯 AI 洞察、人工審核及採用／回退；再加入 Provider／Model／來源／功能的 Token 用量、成本與實際可取得額度。未知額度明示未知，不能由 Token 數推算。
## 3. 目前程式與 Day16 封存
本次更新核對（main `7917f8b2e05ed74703ff9a83b62732ce04461da7`）：root AI人物關聯ai_person_id及room_id已加NOT NULL，相關缺少欄位拒絕實測通過；AI人物主鍵id仍可NULL。Stats程式、PG migrations及Cargo.lock未變，Cookie續期與PG31→32升級的前輪失敗仍未修；本輪未重跑相同HTTP／PG流程，AI表不等於帳密／角色功能完成。 [本輪命令與27個schema案例](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-7917f8b/REPORT.md)。以下6e1c791及更早版本／候選保留為歷史基線，不回填為最新main通過。完整功能仍未驗收；未新增規則或Day。
歷史候選e33971f只適用舊2eddd64；最新main已修改建表與ID契約並刪除ALTER檔，該patch不可直接套用。先前候選成功結果保留原SHA，不代表本輪checksum升級問題已解決。
本次複驗來源 main `1853876e0eb1cfe8e4562be8b3c661b55f1bc54b`；教學分支更新前 d9ee7f1994b4b115dc81a34e27bb607c5b139c8e。Day18 已發布，next_day 為 19；此次是內容與驗證狀態修訂。
main `1853876e0eb1cfe8e4562be8b3c661b55f1bc54b` 已套用伺服器 touch_session，但活動 Dashboard 200 回應沒有刷新 Cookie，Cookie 續期 Red 本輪再次失敗；獨立 PostgreSQL 登入查詢登出通過。候選 `80ba01e6d965d0e3ff10bef18a5cbd17f01e8f05` 同步 server expiry／Cookie 的 HTTP 回歸通過，尚未合併 main。帳密／角色／來源 scope、立即撤銷與 30 分鐘瀏覽器 soak 未完成或未驗證，migration 31→32 改名造成舊版升級實測 missing migration 失敗，兼容修正尚未完成；完整產品未驗收。
後續需套用並重測 Cookie 修正，以及原有 admin/admin 初始化、首登改密碼／唯一 Email、加鹽雜湊持久化、角色／來源授權與立即撤銷；實測 username/password 登入 422（缺 key），目前仍是登入碼原型。migration 31→32 改名使已套用 31 的測試 DB 升級失敗；兼容修正尚未完成，需保留歷史檔／checksum 並驗證 31 與 32 兩種既有基線。
Day15／Day16 原文與歷史證據保留；舊版的未執行／通過紀錄仍只適用其原 SHA。
Day16 封存入口：<mention-page url="https://app.notion.com/p/3fce51d45093813fa38ae37a47ae4959">2026-10-17｜P0 Analytics Day 16｜登入 Leptos 後台並查詢</mention-page>。<mention-page url="https://app.notion.com/p/3fbe51d4509381319460de180d9e173d">2026-10-17｜P0 Analytics Day 15｜Dashboard 查詢與篩選接線</mention-page>
[目前程式驗證與未完成項目](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-17-source-98f30d8-recheck.json)；以下為保留的功能目標與驗收條件，是否完成以上方最新來源及證據為準。
#### Leptos Dashboard：SSR 已存在，帳密流程尚待完成
Analytics 畫面沿用 Leptos 與 Rust 元件，不以獨立手寫 HTML／JavaScript 維護另一套互動介面。先核對現有 UI crate／SSR／hydration 的責任與建置入口再接線，不為三專案檔名一致新建空骨架。
基本元件為來源選擇、事件類型篩選、查詢／重新整理、載入狀態、空資料、錯誤／重試與結果表格；由 Leptos 狀態與非同步請求呼叫現有 HTTP JSON Query API，保留來源授權與 PostgreSQL Adapter。篩選切換時清除過時結果，取消或辨識舊請求，避免晚回應覆蓋新結果。SSE 於基本查詢接通後逐步補齊。
UI 不持有 SQLx／PgPool，不另建平行 total API 或資料來源；正式管理者 Session／唯讀查詢權限與服務憑證分開，不把後端來源密鑰嵌入 WASM／HTML。來源 token 的既有學習契約保留為過渡事實，不當成正式 Dashboard 授權模型。
Day15 的 HTML／JavaScript 提案保留歷史；改採 Leptos 的完整變更與原因，在下一 Day 的 Refactor 後「舊 Day 更正補充」提出，不回填 Day15。這次只記錄方向與缺口，不新增 Day 或宣稱已接通。
驗證須分別包含 native SSR（若採用）、WASM hydration／CSR、元件狀態、HTTP／來源隔離、真實 PostgreSQL 結果與瀏覽器流程；既有 API 或 CI 成功不能代替 Leptos 畫面驗收。本輪未執行上述新增驗證。
#### 服務間交互憑證：已確認需求，未實作
服務間來源認證採「首次核准配對 → 短效憑證 → 來源端自動交互續期／輪替 → 撤銷／重新配對」；Chat／Pet 與 Analytics 互驗對方身分，不以 source_app 自我宣告或固定 token 無限換證。Leptos 管理畫面負責配對核准、期限／輪替政策、狀態與稽核；後端及來源端執行自動換證。此為已確認需求、未實作／未驗證，完整方案於後續 Day 提出，不回改 Day15。
- 首次配對：管理者核准來源服務／實例，短效一次性配對碼須綁定核准交易與來源身分、可過期且只可成功兌換一次；配對碼只作首次信任建立，不能當永久服務憑證。
- 存取與續期：簽發限定來源、實例、接收服務與權限的短效存取憑證；來源端憑受保護的續期資格／身分證明，於到期前自動申請，成功確認後原子切換。被撤銷或已過期的存取憑證不得單獨換取新憑證。
- 通訊與保管：使用可驗證服務端身分的受保護通道；來源端持久安全保管必要續期資格，發證端對可雜湊的隨機 bearer／續期秘密僅存驗證雜湊，不存可讀明文。若採非對稱身分，私鑰由持有者安全保管，對端保存公鑰與綁定；不預定必須採 JWT 或 mTLS，協定與金鑰格式留待實作選型。
- 可靠輪替：新舊憑證僅在明確、有期限的窗口重疊；處理重試、並發、回應遺失、程序重啟及時鐘偏差，續期資格輪替與重放偵測須有交易一致性。更新 Analytics 必須與 Chat／Pet 的取得、確認和切換接通，不靠人工定期複製 token。
- 撤銷：管理者可停用來源／實例、撤銷續期資格及存取授權；定義撤銷生效上限並由服務端強制執行，失效時拒絕受保護操作，不無限延用舊憑證。來源重新配對須再次核准。
- Leptos 管理畫面：顯示來源／實例、憑證識別碼、權限、狀態、簽發／到期／輪替時間、最後成功通訊與錯誤；提供核准、立即輪替、撤銷／重新配對。完整秘密只在必要簽發交付時可見，不做日常回顯，不進 URL、日誌或統計事件；管理者 Session 與服務憑證分開。
- 驗收：未核准／錯誤對端不能配對；一次性碼過期／重放拒絕；跨來源與越權拒絕；到期前自動換證不中斷；回應遺失／重啟可恢復；舊證窗口結束失效；撤銷後不能續期；畫面狀態與實際授權一致，稽核可追溯且不含秘密。以本機合成測試服務及測試憑證驗證，不讀正式密鑰。本次均未執行。
#### 純後台登入與授權：Session 已存在，帳密與管理尚待完成
Analytics 定位為純管理後台，Leptos 提供後台畫面；除登入及必要的身分驗證流程外，所有業務頁面、查詢、管理與即時資料均須登入且具相應權限，不提供匿名 Dashboard。管理者 Session 與 Chat／Pet 服務憑證分開：服務上報／換證驗證機器身分，不以人員登入或來源 token 取代後台登入。
- 入口：未登入只可進入登入及必要驗證流程，不渲染或傳送業務資料；登入成功後只進入授權頁面。不提供公開業務首頁或匿名統計。
- 服務端強制：Leptos 畫面／SSR／server functions（若採用）、HTTP 查詢／管理 API 及 SSE 均檢查 Session 與操作權限；隱藏按鈕或前台 route guard 不能代替後端授權。未登入的 API 回 401，已登入但權限不足回 403；瀏覽器頁面導向登入，不先輸出敏感 SSR 資料。
- Session 生命週期：登入建立與更新有效工作階段；逾期、登出或管理者撤銷後不能繼續查詢／管理。SSE 在建立、重連及存續期間依授權政策處理失效，不能讓已登出的連線持續收到資料；具體撤銷生效期限在實作時明定與驗證。
- 管理操作：來源配對核准、憑證輪替／撤銷、封存／還原／備份等均須管理者權限與操作稽核；若使用 Cookie Session，狀態變更須有 CSRF 防護。不將來源服務的 ingestion／續期秘密交給後台瀏覽器當登入憑證。
- 機器通道：Chat／Pet 的 HTTP／gRPC 上報、配對兌換與自動換證使用各自服務身分及權限；不能開放匿名，也不要求每次自動上報由人員互動登入。一次性配對須有已登入管理者的核准交易。
- 驗收：直接輸入後台 URL、未登入呼叫 API／建立 SSE、跨權限操作、Session 到期／登出／撤銷、重連及狀態變更防護；確認沒有先洩漏頁面資料或以來源 token 越權登入。機器上報／換證與人員登入分別驗證。本輪未執行，不能宣稱已有登入保護。
此要求納入既有 Dashboard 與來源認證項；不回填 Day15，不新增 Day，不增加 next_day。
## 4. 驗證方式與尚未驗收範圍
本輪 main 14 單元＋2 idempotency 契約通過；最新 migration 改名使舊版升級實測 missing migration 失敗；專用 PostgreSQL ignored login/query/logout 1 passed；實際 HTTP 匿名／Origin／CSRF／登出／來源 token 分離通過，但活動 Cookie 未刷新。候選完整回歸 105 passed／1 ignored，另外 postgres-integration include-ignored suite 22 passed／0 ignored；短 deadline TCP 不是 30 分鐘瀏覽器 soak。
最新 1853876 與歷史 abebbef 的 CI 分開列於證據（abebbef 的 check-chat／build-chat 為 failure）；精確命令、環境、時間、連結、原始錯誤與候選差異見 [正式複驗與原始證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-1853876/REPORT.md)；完整程式與補教集中於 <mention-page url="https://app.notion.com/p/3fde51d4509381c2a0d5f60041137619">2026-10-18｜P0 Analytics Day 18｜有效活動延長伺服器 Session 並拒絕過期</mention-page>。
後續需套用並重測 Cookie 修正，以及原有 admin/admin 初始化、首登改密碼／唯一 Email、加鹽雜湊持久化、角色／來源授權與立即撤銷；實測 username/password 登入 422（缺 key），目前仍是登入碼原型。migration 31→32 改名使已套用 31 的測試 DB 升級失敗；兼容修正尚未完成，需保留歷史檔／checksum 並驗證 31 與 32 兩種既有基線。
本次重新驗證不等於全部產品功能已通過；較早 98f30d8 的數字與舊 binary 觀察已移回歷史證據，不作目前 runtime 結論。
## 5. 開發位置與使用本入口的方式
學習庫 apps/stats_server/src/`lib.rs` 負責 HTTP 路由與來源授權，pg_`store.rs` 負責 PostgreSQL 聚合與持久化，migrations 保留已套用歷史；產品端 shared/proto 與 analytics_transport 提供產品通訊契約，不能視為學習庫已有同一實作。
先沿既有 API 接通粗略可操作畫面，再逐步補趨勢、明細與管理。資料安全或基本流程阻礙當步處理，其他深化留在原功能追蹤；詳細程式與驗證仍看既有 Day。
[產品通訊契約](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/ANALYTICS_TRANSPORT.md) · [操作設定](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/DEPLOYMENT.md) · [產品未來清單](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/FUTURE_WORK.md)
## 既有Day索引
<page url="https://app.notion.com/p/3fde51d4509381c2a0d5f60041137619">2026-10-18｜P0 Analytics Day 18｜有效活動延長伺服器 Session 並拒絕過期</page>
<page url="https://app.notion.com/p/3fce51d4509381ea871ec9c92da37c1b">2026-10-18｜P0 Analytics Day 17｜活動延長管理員 Session</page>
<page url="https://app.notion.com/p/3fce51d45093813fa38ae37a47ae4959">2026-10-17｜P0 Analytics Day 16｜登入 Leptos 後台並查詢</page>
<page url="https://app.notion.com/p/3fbe51d4509381319460de180d9e173d">2026-10-17｜P0 Analytics Day 15｜Dashboard 查詢與篩選接線</page>
<page url="https://app.notion.com/p/3fbe51d4509381f2a9a2e4b608706eea">2026-10-16｜P0 Analytics Day 14｜依事件類型篩選來源統計</page>
<page url="https://app.notion.com/p/3fae51d4509381808a9cd813423d52d1">2026-10-16｜P0 Analytics Day 13｜來源隔離的 PostgreSQL 統計查詢</page>
<page url="https://app.notion.com/p/3fae51d4509381ebb1e9f8482b5bf44e">2026-10-15｜P0 Analytics Day 12｜HTTP 來源驗證與 PostgreSQL 持久化接線</page>
<page url="https://app.notion.com/p/3f9e51d45093814a8054d44c4ff9fa55">2026-10-15｜P0 Analytics Day 11｜PostgreSQL 事件儲存與冪等</page>
<page url="https://app.notion.com/p/3f9e51d4509381269984c5f49a429b5d">2026-10-14｜P0 Analytics Day 10｜有效事件的來源與 ID 去重入口</page>
<page url="https://app.notion.com/p/3f8e51d45093810d830ef3fd40e419f0">2026-10-14｜P0 Analytics Day 9｜事件類型前後空白拒絕</page>
<page url="https://app.notion.com/p/3f8e51d45093816e96edd35b83543ed3">2026-10-13｜正式補跑｜Analytics｜來源與版本驗證測試</page>
<page url="https://app.notion.com/p/3f8e51d4509381689c98ed86fecc50a2">2026-10-13｜手動補跑｜Analytics｜現有 EventInput 的有效識別碼前置驗證</page>
