# rust_ai_chat｜三案全部文章驗證與完成封存

2026-10-23（Asia/Taipei）。程式 main `9c2b7cf5a4c60a08254445911916c4ed9b02d7b3`；文件基準 main `fad5db13a6711ac45a9cfece8198b612d7ce4607`。版本仍是 workspace 0.2.28，提交名稱「Phase2 day30」不等於已發布 Day30 文章。

**已逐篇核對 GitHub 現存 Day2–Day29 共84篇主文；結果包含失敗與待驗收，沒有宣稱84篇全部通過。** [逐篇矩陣](ALL_ARTICLES.md)列出每篇實作、呼叫鏈、實測或缺口。另讀取全文並靜態掃描483份 docs Markdown，包括歷史修訂、鏡像、索引及11份補充頁；逐檔 SHA256、命令、API、來源行號、測試名／本輪結果、缺少測試與版本路徑見 `all-articles-validation.json.gz`。靜態掃描不是483篇功能E2E。GitHub現行目錄與交付索引沒有Day1正文，列來源缺口；未以Notion或相似標題補造。

## 實際結果

全部下列程式測試均在原樣 `9c2b7cf5a4c60a08254445911916c4ed9b02d7b3`，`--locked`；沒有套用候選修正。Linux Rust/Cargo1.99.1，PostgreSQL18.6，本輪專用55430／SQLite測試庫；只有合成資料。使用既有target快取加速，本輪不沿用前次SHA測試結論。原始log保存在[logs](logs/)。

| 範圍／命令 | 結果 |
| --- | --- |
| `cargo test --locked -p server --features ssr` | 25 passed |
| `cargo test --locked -p ui --features ssr` | 9 passed / 1 failed：Day16歷史提示斷言 |
| `cargo test --locked -p desktop-pet` | 49 passed |
| `cargo test --locked -p desktop-pet-frontend` | 9 passed |
| `cargo test --locked -p stats-server`（無DB URL） | 22 passed / 1 ignored |
| `DATABASE_URL=隔離PG cargo test --locked -p stats-server --features postgres-integration -- --include-ignored` | 35 passed / 0 failed / 0 ignored |
| `cargo test --locked -p chat-application -p chat-domain -p persistence -p pet-domain -p shared -p ai-core -p telemetry` | 32 passed；chat-domain／telemetry為0 tests，沒有宣稱各自測試覆蓋 |
| `cargo build --locked -p server --features ssr`、`cargo build --locked -p stats-server --bins` | 通過；執行的HTTP binary即本SHA建置 |
| `cargo check --locked -p ui --features hydrate --target wasm32-unknown-unknown` | 通過 |
| `cargo check --locked -p desktop-pet-frontend --target wasm32-unknown-unknown` | 通過 |
| `node --test apps/desktop_pet/frontend/tests/*.test.mjs` | 0 passed / 3 failed，VM缺URLSearchParams |
| `cargo fmt --all -- --check` | 通過，空log且exit0 |
| `cargo clippy --locked -p server --features ssr --all-targets -- -D warnings` | 失敗：UI set_rows未使用、refreshed_room_list及refresh_result_view dead-code |
| `cargo test --locked --workspace` | 連結server時空間不足／ld Bus error，exit101；未完成，不計Pass |
| 額外Chat事件映射probe，直接連結本輪原樣library | 3 passed；補驗schema／穩定ID／隱私／錯誤ID，非原repo已新增測試 |

不同篩選、feature及重跑有重複測試，不加總為獨立案例。完整workspace命令失敗後只清本輪11:05:18之後產生的target建置快取，分套件補測；未清私人資料或修改來源。全部12個workspace package均有對應測試執行範圍，仍不能稱單一workspace命令／全feature整合已成功。嚴格Clippy只實跑Chat範圍，其他範圍未測不當作成功。

## 真實入口驗證與必要更正

- **Chat**：`chat_http.py`啟動本SHA真正server及全套fresh migrations；SSR登入表單→註冊保持登出→登入→account/room→WS Upgrade。匿名、偽造、過期及登出Cookie皆401，有效Session101；兩客戶端收到同一持久化訊息，SQLite查到一筆。實際停止／重啟server後Session仍可用；登出只撤銷一筆，另一筆仍有效。這涵蓋Day26/27/29切片；未驗證hydration、逐房ACL、所有瀏覽器互動。匿名POST建房仍201，API授權未完成。
- **Analytics**：實際執行本SHA bootstrap_admin CLI於全新PG，經password_login正式路由驗首次登入限制、改密、舊Session撤銷、重登入及兩來源Dashboard。只有作用中連結有aria-current，未知source400，未登入303/login。PG Day27測試含非零兩來源分別計數；額外runtime probe驗真正ingest401/201/200/409、版本400、JSON422/400/413與source summary200。
- **Analytics舊Day14**：正式 `/api/v1/events/web_chat/summary/chat.audit` **404**。source_kind_summary/helper及legacy `admin::build`測試成功，不能代替目前 `password_login::build` 的真路由。恢復事件型別查詢須接入目前Session與權限；尚未修改程式。
- **Analytics舊Day19/20**：目前帳密login沒有legacy attempts／Retry-After政策；真HTTP連續11次錯密碼最後仍**401，沒有預期429**。舊admin單測綠燈不能結案現行登入保護。尚未實作帳號級等待／持久化。
- **Pet**：49項native及9項frontend測試證明其各自契約，沒有證明真WebView/OS生命週期。Day29已把set_caged放在close之前；DB故障／關窗失敗注入、三id GUI／真App重啟未驗收。控制main X沒有CloseRequested保留或重建，open_control_window只找舊main，仍可能叫不回。roam只尋固定`pet`，設定只有main而生活窗為`pet-UUID`，舊漫遊測試不能證明動態窗會動。
- **Leptos原文勘誤**：Chat／Analytics Day28原文 `leptos::ssr::render_to_string` 不適用lock中Leptos0.8.22；原樣最新程式已用 `view! { ... }.to_html()`。以prelude匯入RenderHtml；保留原文，後續修改只走版本化勘誤／下一適用Day。另Day16完整SSR仍缺「歷史載入尚未接通」，應按真實歷史能力修復畫面及契約，不能單純刪斷言掩蓋。
- **命令勘誤**：Pet Day29 `desktop_pet` 改 `desktop-pet`；Analytics Day29 `stats_server` 改 `stats-server`；舊 `chat_application`／`pet_domain`亦須用連字號。已不存在的 `workspace-tests`不另重建，按當時原test與本輪JSON對照定位目前擁有的套件。`stats_server`是Rust import名，並非Cargo package名。
- **Node測試環境**：原樣3個VM fixture缺URLSearchParams，並缺供bridge讀取的location；本輪沒有改來源。修正時應在各VM context提供 `URLSearchParams` 與 `location: { search: "" }`，依測試用途提供`window.location`；再跑3項，不能由本次WASM編譯推定它們已通過。
- **歷史語境**：Analytics早期合法501是未持久化stub；現行授權PG攝取201/200已接通，不能照舊文回退。舊token Dashboard／登入不可用提示也已被後續流程取代。過時的Notion交付規則只保留為歷史，不恢復Notion使用。原Red、舊CI、舊庫checksum及跨平台沒有本輪證據就保持未驗，不重寫歷史。

## 完成封存與尚未完成

使用者本輪明確要求「已驗證完成的文章進入封存」。本輪按**本篇最小切片**新增封存6篇：Chat Day26/27/29、Analytics Day26/27/29。這些不等於全產品完成；各自必需範圍及排除項在archive manifest。原文逐位元組移至history/day-NN/project/github.md，原blob與SHA256不變；當時「未套用／未執行」是原交付記錄，最新程式與結果另見本報告。原文勘誤單獨列上節，Analytics Day29的package命令勘誤不被當成原樣命令成功。

其他目前活動文章不因單元測試綠燈直接封存。Day28兩案保留原文API差異；Pet需實機驗收。Day25以前既有封存不回寫，本次重新驗出缺口不刪除其歷史身分。next_day三案保持30、makeup_queue與last_full_comparison不變；沒有新增Day30、結案FIFO或修改排程。本報告是全部文章核對，不冒稱週六全程式／多平台產品驗收完成。

## CI快照

精確SHA `9c2b7cf5a4c60a08254445911916c4ed9b02d7b3`，取回時間 `2026-10-23T11:16:38.361440+08:00`；詳見[workflow及逐job快照](ci-final.json)。Test All的UI命令沒有--features ssr；Persistence沒有--include-ignored，因此不能覆蓋本輪全部35項PG；Node失敗另列。尚未完成job保留真狀態，不因本地通過推定遠端成功。

- [Node](https://github.com/williechen/rust_ai_chat/actions/runs/39891849761)：completed / failure；jobs：test-node=failure。

- [Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39891849769)：completed / success；jobs：test-ai-core=success、test-desktop-pet=success、test-chat-domain=success、test-server=success、test-stats-server=success、test-pet-domain=success、test-shared=success、test-chat-application=success、test-ui=success。

- [Rust Dashboard](https://github.com/williechen/rust_ai_chat/actions/runs/39891849841)：completed / success；jobs：check-dashboard=success、build-dashboard=success。

- [Persistence](https://github.com/williechen/rust_ai_chat/actions/runs/39891849758)：completed / success；jobs：test-persistence=success。

- [Rust AI Chat](https://github.com/williechen/rust_ai_chat/actions/runs/39891849782)：completed / success；jobs：check-chat=success、build-chat=success。

- [Rust Desktop Pet](https://github.com/williechen/rust_ai_chat/actions/runs/39891849805)：completed / success；jobs：check-pet=success、build-pet=success。

## 重跑與覆蓋限制

命令均從rust_ai_chat根目錄執行；PG URL只能指向空的隔離測試叢集（SQLx測試需可建測試庫權限），不得使用正式資料。HTTP腳本使用本輪固定loopback端口並先確認未被佔用，測試伺服器結束均已停止；bootstrap只對本輪空PG執行。腳本中的target路徑需改成你的隔離建置目錄。PG已停機，不會將本次帳密／資料留作正式服務。

本次未操作現有桌面GUI、未做瀏覽器互動hydration、Windows/macOS、真多螢幕、既有使用者庫升級、完整安全稽核或付費AI。逐篇未驗收項目均見矩陣，不用更新SHA或文件讀回替代驗證。
