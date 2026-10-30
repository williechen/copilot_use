# Gate2 實測與阻擋｜2026-10-31（Asia/Taipei）

**完整Gate2尚未通過。** 本輪已把「未驗」縮小為實際結果：Pet原生功能25項、Chat／Analytics真瀏覽器各11項檢查通過，精確產品`2c7ff8b861d32ab6d879bf80c9f2dce50de38097`的七個CI workflow全成功；同時確認Chat排除名單未被HTTP／WS強制執行，Analytics角色／來源ACL尚未接通。Day44功能切片與整體授權／實機／舊庫驗收分列，不能互相抵算。

[機器摘要](summary.json) · [CI原始snapshot](ci.json) · [原生步驟／DB／視窗證據](pet-gui-checks.json) · [Chat瀏覽器](chat-browser-checks.json) · [Analytics瀏覽器](analytics-browser-checks.json) · [Git狀態](git-source-final.json)

## 實際驗收矩陣

| 範圍 | 本輪結果 | 證據／界線 |
| --- | --- | --- |
| 精確SHA CI | 7個completed/success；未重跑或修改workflow | ci.json；CI不代替GUI |
| Pet建置 | 前端真正release；原生debug＋tauri/custom-protocol，內嵌同來源資產成功 | [命令／exit／時間](build-commands.json)、[產物SHA256](built-artifacts.json)；不是native release或安裝包 |
| Pet native GUI／IPC | 三隻由真正UI建立；三生活窗200×200、任一活動出籠、坐下停實際位移、重複出籠不增列／不重開窗、只關A、Native X、控制台隱藏／重開選自己、窄控制台操作、睡覺／醒來、兩次真程序重啟與預設資格共25項通過 | [25步](pet-gui-checks.json)、[WebDriver完整命令](webdriver-commands.jsonl)、[Native事件](native-events.jsonl)；專用Xvfb軟體繪圖，沒有mock IPC |
| Native DB失敗 | 只在專用SQLite注入trigger，UI回籠與WM_DELETE_WINDOW失敗時精確保留坐下／視窗，恢復後只回籠A | [同一程序的SQLite FD](native-process-db-proof.json)；真OS開窗失敗未注入 |
| Chat build／HTTP／browser | 本來源SSR＋hydrate建置，註冊保持登出、清除舊回跳目標、登入管理、正式表單建房、公開清單自動載入、匿名回跳／登入進房、缺房404、DB故障503／恢復、SQLite Session與房間重啟保留共11項通過 | [build](http-build-commands.json)、[browser](chat-browser-checks.json)；hydration由自動載入持久房間確認，不只看SSR字樣 |
| Chat逐房ACL | **不通過：排除會員仍HTTP200，WebSocket仍open** | [完整重現](chat-room-acl-acceptance.json)；只在合成帳號／房間fixture測試，非正式服務 |
| Chat既有SQLite副本 | 原庫mode=ro online backup；真產品startup migrator／health成功，既有資料投影hash／筆數相同 | [保資料](chat-actual-old-copy-preservation.json)、[版本範圍](chat-migration-version-scope.json)；觀測原庫／副本均9版，不能宣稱本輪真的新增了舊版migration |
| Analytics真PG／browser | 初始admin強制改密碼、兩頁wrong logout CSRF403保留Session、真表單登出／PG撤銷、改密碼撤銷、Dashboard／來源切換保留kind、CSV、舊Cookie重放拒絕、PG及程序重啟後新密碼保留共11項通過 | [browser](analytics-browser-checks.json)、[HTTP命令](browser-commands.jsonl)；cookie／測試輸入已遮罩 |
| Analytics角色／來源授權 | **未實作／未完整驗收** | [實際Dashboard提示](analytics-role-acceptance.json)：目前僅支援管理員，角色與來源ACL尚待接通 |
| 實際舊PG升級 | **未執行：使用者先前確認無備份，本輪沒有新提供的備份／來源版本** | 本輪是專用fresh PG與重啟，不能替代實際舊庫還原 |
| 實機／發行 | 實際桌面拖曳、右鍵、HiDPI／背景可讀性、多螢幕、真OS開窗故障、native release／三平台安裝仍未驗 | 已準備下列免重編腳本，不算人工已通過 |

## 已確認的授權缺口

Chat以真UI註冊兩帳號，由第一帳號建立Gate2 Room；第二帳號登入後在**專用測試庫**加入同房間的mbr_exclude列，再透過帶有效Session的真HTTP fetch／WebSocket連線。預期拒絕，實際HTTP200及WS opened=true。每次probe後移除該測試排除列，不影響使用者資料。這不否定Day44「存在房間／缺房404」切片已實作；它證明原已確認的逐房授權條件仍未成立，完整產品Gate2不能通過。

回填既有[FUTURE_WORK Chat授權](../../FUTURE_WORK.md#chat-auth)與[Analytics授權](../../FUTURE_WORK.md#analytics-auth)，不另開Day／重複FIFO。產品維持唯讀；本輪沒有寫產品補丁、提交、推送或PR。需要產品修正後以新SHA驗收拒絕／允許、HTTP／WS、既有Session撤銷與有效斷言。

## 截圖與方法

[初始控制台](pet-initial.png) · [三隻建立](pet-three-created.png) · [A生活窗](pet-life-A.png) · [窄控制台](pet-controller-narrow.png) · [第一次重啟](pet-after-restart.png) · [預設A再次重啟](pet-final-restart.png) · [Chat註冊完成](chat-signup-complete.png) · [Chat房間](chat-existing-room.png) · [Analytics首次改密碼](analytics-first-password-change.png) · [Dashboard](analytics-dashboard.png)

使用已安裝的tauri-driver／WebKitWebDriver與MiniBrowser，無新增Tauri測試plugin、無攔截／mock IPC。方法依[Tauri官方WebDriver](https://v2.tauri.app/develop/tests/webdriver/)及[官方Selenium範例](https://v2.tauri.app/develop/tests/webdriver/example/selenium/)，直接記錄W3C命令與回應。Native close是只對私有Xvfb視窗送WM_PROTOCOLS／WM_DELETE_WINDOW，並核對真SQLite與視窗存活。

測試工具誤判另存[原始修正紀錄](harness-setup-errors.json)：C locale無法讀中文窗名、負向IPC的正常error物件被當協定錯誤、PG證據查詢誤用表名；均修正harness、沒有改產品。WebKit對A名稱的Get Element Text／displayed回報與實際可見截圖、非零DOM尺寸不同，該atom沒有被當作通過依據，限制保留在原生證據。沒有單一「47個單元測試exit0」；這是25／11／11個有命令與狀態對照的功能檢查步驟，另有一個真正ACL驗收失敗。

## 可 review 的實機接續

已保存同SHA原生產物副本與SHA256，[啟動腳本](manual-pet.sh)通過bash語法與`--check`預檢。使用另外的pet-manual.sqlite，首次才從測試fixture備份；不覆蓋後續手動資料，不連正式DB，並隔離XDG資料／cache。無需重新編譯：

```bash
bash '/home/willie/Documents/Codex/2026-10-28/referenced-chatgpt-conversation-this-is-an/work/gate2-20261031/啟動Pet實機驗收.sh'
```

僅操作Gate2-A／B／C，驗實際桌面拖曳、原生右鍵、縮放／可讀性與多螢幕；Ctrl+C結束整個程序，關控制台只隱藏。產物／專用DB／log路徑見[frozen-native](frozen-native.json)。本輪只做預檢，沒有把此實機操作列為通過。

## 清理、來源與保存

先列[25個已結束Cargo target/deps執行檔路徑與大小](cleanup-plan.json)，再刪[4,672,729,088 allocated bytes（4.35GiB）](cleanup-result.json)。只清未映射、單硬連結ELF；原生舊產物先備份，DB／.env／Cargo.lock／src／未知檔／歷史證據保留。沒有reset --hard、clean -fdx、正式DB或付費AI操作。

所有SQLite、PG、瀏覽器與driver測試程序已停止，PG正常關閉，保留測試DB在本地供重現；DB、原生binary、WASM不發布GitHub，只保存雜湊及synthetic資料截圖。原產品與隔離Git tree均乾淨。Chat／Analytics／crates既有Rust單元結果仍保留4b1008b來源，本輪不重跑或追認；Pet112的既有來源是同一2c7ff8b，本輪也沒有重新加總。

前版文章／入口／狀態於[第十五差異包](../../archive/packages-2026-10-30/15-before-gate2-functional-acceptance-2026-10-31.zip)去重封存，前十四包不變。next_day三案45、既有FIFO與last_full_comparison均不變；本輪是有邊界的Gate2補驗，不是完成的每週全專案比對。[本次獨立文章commit／blob／SHA256讀回](../../teaching/daily/day-44/gate2-delivery-2026-10-31.json)。
