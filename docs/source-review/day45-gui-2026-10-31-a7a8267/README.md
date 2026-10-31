# Day45 真實瀏覽器與原生桌面驗收

產品精確來源`a7a8267f8c2f9003dd0d91d9a8cd25daba2517e4`，文件基準`e17097e45c14539773b27ceb69144ef7e2c810f1`。產品庫唯讀，啟動與候選修正均在隔離副本／測試DB；沒有正式DB、付費AI或外部訊息。[完整驗收資料](gui-review.json)、[可接受範圍分列](accepted-scopes.json)。

| 範圍 | 真實操作結果 | 證據 |
| --- | --- | --- |
| Analytics原樣 | 11段Chromium瀏覽器旅程通過 | [log](browser-run.log)、[admin](analytics-admin.png)、[viewer tree](analytics-viewer-tree.png) |
| Chat原樣 | 註冊／登入、建房、真WS發送持久化、排除後斷線及SSR403通過；本人資料頁403，驗收失敗 | [原樣結果](browser-results.json)、[403畫面](chat-raw-profile403.png) |
| Chat補正候選 | 45項回歸通過，含新真註冊Red／Green；9段真Chromium旅程通過 | [回歸](chat-fixed-regression.log)、[瀏覽器](chat-candidate-browser-results.json)、[畫面](chat-candidate-room.png) |
| Pet原樣debug native | 實際Tauri/Wry GTK WebView、多視窗／200×200／六姿態／原生菜單／回籠／關閉／拖曳／找回／重啟通過指定範圍 | [分段結果](accepted-scopes.json)、[native close](native-close.json)、[menu](native-menu.json)、[drag](native-drag.json)、[restart](native-restart-results.json) |

## Chat：瀏覽器抓到的產品缺陷與完整補正

實際migration定義與member-default角色授權是`member-profile-update`／`member-password-update`，後端卻查`member-self-profile-update`／`member-self-password-update`。真註冊會員無權開自身資料頁，實測403；密碼常數也同樣與來源定義不一致。原44項測試fixture自行建立並加上後者，沒有覆蓋真正註冊→預設角色這條鏈。原44項紀錄不刪，但不能作這條旅程通過的證據。

新增`day45_signup_self_service.rs`從真signup→login走到兩個self-service頁，斷言沒有任何mbr_user_perm人工加權。原樣Red **exit101／403≠200**：[log](chat-signup-red.log)。[完整補正patch](chat-permission-id-fix.patch)只改後端兩個常數、對齊既有測試fixture，並加入上述新回歸；**保留使用者現有migration及權限ID，不修改schema／既有migration**。會員仍只有action，目標仍只從Session取得，不增加管理其他人的能力。

候選完整回歸 **45 passed、exit0**；用正規`cargo leptos build --cargo-locked`建置server與hydrate資產後，真Chromium註冊新帳號、建房、WS訊息落DB、即時排除、自身資料表單、自身密碼表單、舊Session撤銷與新密碼登入、匿名房間登入回跳等9段均通過，無pageerror。[套用／逐檔相等](chat-patch-validation.json)。候選通過不等於產品main已修；使用者仍需套此patch。

中途用普通cargo build配cargo-leptos資產造成_bg.wasm 404，是驗證建置接線錯誤；已改回同一正規cargo-leptos流程再實跑，失敗嘗試保留，未列成產品缺陷。

## Analytics：實際瀏覽器範圍

真正HTML表單與cookie：預設管理員首次改密碼→撤銷Session→新密碼登入首頁；管理員新增Chat-only viewer；viewer首次不能略過改密碼、第二次登入首頁；來源／帳號頁直接請求拒絕；同Session即時action deny／恢復；page-only仍顯示category/page但無update表單，偽造POST拒絕且密碼未變；最後一個會員管理page撤權後category消失；登出後API401。

為驗即時撤權，只在隔離測試DB改grant，再從相同真瀏覽器Session驗反應；**不是宣稱產品已存在權限編輯UI**。沒有攔截request或mock server，沒有補造會員page權限。`browser.cjs`及[整體原樣log](browser-run.log)保留；該次整體exit1來自Chat403，Analytics自身11段完成。

## Pet：真正原生WebView與OS視窗

[官方WebDriver方法](https://v2.tauri.app/develop/tests/webdriver/)的Linux外部tauri-driver＋WebKitWebDriver路徑。實際產品debug程式、trunk生成的真WASM、真Tauri IPC與SQLite，跑在獨立Xvfb :97／xfwm4，沒有替換window.__TAURI__或mock invoke。

控制台實際輸入建寵物；原生life window與DOM皆200×200、單張img；真clock與主控按鈕產生idle/blink、interact-a/b、sleep-a/b，變形bounds留在視窗內。出籠不重複建窗；回籠後原生窗消失與DB停用。Tauri前端close IPC因未授權而拒絕，是權限邊界；改用真正X11 WM_DELETE_WINDOW驗CloseRequested回籠，未放寬capability。

雙寵物同時開三個原生窗（主控＋兩life），B互動不改A；GTK原生右鍵選單的鍵盤睡覺操作作用於B而A維持idle。[菜單畫面](pet-native-menu.png)。WebDriver合成拖曳未移動OS窗，改用XTest真正OS指標事件驗證DragHandle可移動；這次移到離屏位置，實際「找回寵物」UI把它帶回monitor內。[native drag](native-drag.json)。不把這項描述為拖曳邊界clamp通過。

UI設定B預設出籠／A預設留籠，終止真原生程序後以同一DB重新啟動，恢復只有B的life window，A保持回籠。[重啟畫面](pet-native-restarted.png)、[log](native-restart-accepted.log)。[單窗畫面](pet-native-life.png)也保留。

這是**Linux虛擬X11桌面的debug原生驗收**；不是普通DOM fixture，但也不冒充實體桌面／Windows／macOS打包版驗收。軟體renderer有不支援GL警告，透明合成未接受；autonomous roam刻意停用，未驗自動漫遊。多段harness早期的錯表名、enum大小寫、XPath層級及權限拒絕均保留在attempt logs，通過範圍由各分段證據交叉確認，不聲稱每個早期腳本exit0。

## Refactor、版本與交付

沿用三案已發布的獨立Refactor，這次Chat接線補正屬Green缺陷修訂，不增加Refactor完成數。不作跨案crate或全庫CI改動。測試腳本、build log、瀏覽器lockfile及截圖都保存；本地服務、原生程序、虛擬display、測試DB與暫存已停止／清除，[cleanup](cleanup.json)、[cache cleanup](cache-cleanup.json)。

文件Gate1更新；**完整Gate2仍false**，具體剩餘：Chat原樣授權ID缺陷、真實舊DB備份調整、legacy ignored、尚未實作的管理UI/API，以及上述未接受的桌面／OS範圍。不要因這次有真GUI就抹掉其他缺口。next_day三案46、不加Day、不改FIFO或last_full_comparison。原Red／Green、文章、失敗與精確commit證據保留。
