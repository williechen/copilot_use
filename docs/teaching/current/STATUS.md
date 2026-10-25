# 目前功能與驗證狀態

<!-- day37-pet -->
Pet：[Day37 修訂教學已交付](../daily/day-37/pet-r2.md)，來源 `5c713553`；本次僅文件交付，程式未套用、新測試未執行、功能未驗收。[讀回證據](../daily/day-37/pet-r2-readback.json)。

<!-- day37-chat -->
Chat：[Day37 修訂教學已交付](../daily/day-37/chat-r2.md)，來源 `5c713553`；本次僅文件交付，程式未套用、新測試未執行、功能未驗收。[讀回證據](../daily/day-37/chat-r2-readback.json)。

目前程式基準：`williechen/rust_ai_ring@5c713553daee4055738239f047d6c774fcc7a7a1`。文件比對基準：`copilot_use@532e739f1ef4e5920de300eba0f90adeeaf96187`。本輪靜態比對、Node／metadata／schema探測及同 SHA CI 已核對；以下舊來源段落是歷史證據，不覆蓋本段目前判定。

[完整比對與21項差異](../../source-review/2026-10-25-full-comparison-5c713553.md) · [105個閱讀版本／111份修訂](ARTICLE_STATUS.md) · [既有後續條件](../../FUTURE_WORK.md)

| 專案 | 目前來源所見 | 證據與仍缺項目 |
|---|---|---|
| Chat | WS tick／incoming／broadcast 已重查 Session；共用房名與唯一登入表單已接 | 同 SHA 既有 server31／UI14與真WS五情境通過，非本輪重跑；查詢成功仍留查詢中、兩段房間提示、登入回跳、作者／歷史／逐房 ACL、活動Session與撤銷競態仍缺 |
| Pet | Store／persistence 的 set_caged／list_uncaged 及 setup 逐 ID open 已接 | 本輪source Node6通過；同SHA既有server54通過但Store重建不是原生重啟。預設資格／回籠未分離、直接關窗未保存、出籠DB失敗補償、動態漫遊、原生多窗重啟與bundle仍待驗 |
| Analytics | 正式 exact-kind 查詢先驗人員 Session；POST ingestion 保留機器token；Leptos SSR帳密／Dashboard存在 | 同SHA既有PG42通過／1 ignored，另跑ignored1通過與HTTP證據，非本輪重跑。只有admin、角色／來源ACL、正式登入等待、SSE與互動前端未完成 |
| CI | 同SHA七個workflow全success，Pet source／native build與上傳成功 | Persistence工作名為server PG，但實際執行analytics-persistence；Test All的server未加postgres-integration。正式server PG選取與新dist契約仍缺，CI成功不是原生GUI驗收 |

本輪未更新last_full_comparison、不增加Day、不封存或結案FIFO，三案完整產品驗收仍未完成。官方最新穩定版本未重新逐套件查證，未將歷史版本表當本機或目前相容性證明。

## 歷史來源與實測範圍

以下保留各次來源：5d28e96的「WS仍可寫」「重啟只恢復一隻」「token可查精確統計」不能當目前程式事實；其中WS持續重驗、Pet恢復迴圈及精確查詢Session已由後續來源接通。歷史原生日誌／GUI結果不追認為本次SHA已重新驗收。

## Pet 原生驗證補充（2026-10-25）

[真實桌面操作與 SQLite／日誌證據](../../source-review/2026-10-25-pet-native-5d28e96.md)：使用者操作並提供截圖，代理核對 DB／日誌／來源；工具仍無原生 UI 自動控制能力。三個各自具名的生活窗、睡覺／醒來逐 ID 隔離、單隻回籠、控制台隱藏／重開、資料跨重啟保留及手動／重複出籠均通過局部驗證。

重啟只恢復最後選取的丙，漏掉另一隻未回籠寵物；漫遊仍回 pet window not found，兩項未通過。透明窗在淺色背景的文字對比另待調整。第一次 Connection refused 是驗收 runner 未等 Trunk 就緒，已加等待並重開成功；不再將錯誤頁當控制台載入通過。Pet 產品程式未修改，整體功能驗收仍未通過。

## 先前實機驗收：5d28e96（2026-10-25）

[本輪報告](../../source-review/2026-10-25-acceptance-5d28e96.md)：Chat／Analytics 真瀏覽器、HTTP、隔離 SQLite／PostgreSQL，以及 Pet 原生程序啟動。整體尚未通過功能驗收。

- Chat：登入、個人頁建房、中文搜尋、同房雙頁同步、跨房隔離、失敗保留清單與重試通過；歷史未載入、會員作者仍為 anonymous、登出後既有 WS 仍可寫入、匿名進房回登入未接通。
- 使用者確認首頁問題是兩段房間提示。已製作本地單一狀態提示候選修正，實際 SSR／WASM build、UI 測試與瀏覽器驗證；差異檔隨報告保存，沒有提交 rust_ai_ring/main，不將候選修正當遠端已套用。
- Analytics：首次強制改密碼頁、正式登入、Chat／Pet 來源與精確類型篩選、HTTP 改密碼撤銷舊 Session、重啟後資料與新密碼保留通過；改密碼提交由 HTTP 測試，未宣稱瀏覽器完成改密碼。來源 token 無人員 Session 仍可查統計，需求落差保留。
- Pet：編譯、啟動與開啟漫遊旗標已實做，log 重現 `pet window not found`；本環境原生視窗控制不可用且有 EGL 警告，三窗／關窗／重啟互動尚未驗收。
- 5d28e96 新增 Pet persistence 測試，Analytics persistence workflow 改選 analytics-persistence；原七個 CI 全綠只屬 afdbb6e，本輪未以舊 CI 追認新 SHA。尚須確認 Analytics server PG 整合測試在 CI 仍有獨立入口。

## afdbb6e 現存程式與缺口

| 專案／功能 | 此精確來源所見 | 驗證與尚缺項目 |
| --- | --- | --- |
| Chat 帳號與建房 | Day35 validate_room_name 已共用於 JSON／個人頁，拒絕控制字元；原 Session／CSRF 與 transport 契約保留 | 本次 application／server／整合共 35 passed，包含新名稱／400／422／DB 不寫入；作者、歷史、逐房 ACL、共用認證與完整瀏覽器仍待驗收 |
| Chat UI | SignInPage 與 GET /login redirect 保留；RoomList 的舊 helper 仍待測試遷移 | 本次 SSR 14 passed；redirect 測試包含在 server 通過項。未重跑 hydration／真瀏覽器，不追認歷史與權限功能完成 |
| Pet 建立與狀態 | 名稱規則共用 pet-application；Day35 set_caged 已移到 pet-persistence 並由 PetStore／Tauri caller 使用；其它 SQL 仍在 server | persistence＋server 單元／整合共 55 passed；舊 global runtime 仍並存，原生三窗／關窗補償注入／重啟與單幀單圖 FIFO 待驗 |
| Pet 預覽與產物 | 唯讀預覽／摘要／source bridge 保留；舊產物已清，本 SHA CI release 前端／native build／上傳 steps 成功 | 新 dist 契約尚未接 CI，兩個新 persistence library 的自身單元測試也未直接被 CI 選取；實際 binary／GUI、真正清理／receipt／restore 仍待驗 |
| Pet 漫遊 | roam.rs 只查固定 pet；實際生活窗採 pet-UUID。startup 只依 active slot=1 開一窗 | 逐 ID 漫遊／所有未回籠寵物重啟／三窗互動仍有接線與驗收缺口；固定 label 單測或資料層旗標成功不能代替 |
| Analytics 帳密與 UI | ensure_default、正式 password_login／Leptos Dashboard 保留；精確類型查詢已接 analytics-persistence；新增來源 token API | persistence 真 PG 2 passed；HTTP oneshot 證實新 API 無管理者 Cookie、僅來源 token 可讀統計；與業務查詢須人員登入需求有落差。瀏覽器／角色 ACL／登入等待／SSE／服務配對仍待驗收 |
| Analytics 舊碼與範本 | bb95a60 修復五個 E0425；afdbb6e 更新四處舊文案斷言，保留來源／連結／可及性檢查；legacy admin／範本仍保留 | 最新完整 PostgreSQL 回歸 44 passed／0 failed／1 ignored、exit 0；Day27／28／29 全過。查詢授權需求落差、其它功能與 GUI／瀏覽器仍未驗收 |

## 可引用的測試證據範圍

- [本次 13 項重掃與 CI 快照](../../source-review/2026-10-25-adjustments-rescan-afdbb6e.md)：本 SHA 的七個 push workflows 均 success，包含 Test All／Persistence／Pet release build。仍缺 persistence libraries 自身測試的 CI package 選取、新 dist 契約與 GUI／需求驗收；CI 全綠不代表功能全部符合需求。
- [同 SHA 的本機 PostgreSQL 回歸](../../source-review/2026-10-25-assertions-afdbb6e.md)：44 passed／0 failed／1 ignored；編譯與舊文案失敗均已修復。本輪來源未變，沒有重複執行相同測試。人員查詢授權落差保留。
- [e753c36 增量測試與 Router 實證](../../source-review/2026-10-25-update-e753c36.md)：Chat 35＋UI 14、Pet 55 原範圍結果可引用，本輪 git diff 證明兩案全目錄與本 SHA 無差異；原 Analytics 失敗僅屬歷史，已由後續修正。HTTP probe 的授權實作未再變更。
- [891ff32 前次重掃／debug 產物契約](../../source-review/2026-10-25-project-rescan.md) 與 [更早 JS 差異報告](../../source-review/2026-10-25-duplicate-application-review.md) 保留各自輸入及來源；舊 tracked dist 兩敗不再當目前失敗。沒有宣稱原 debug 產物就是本次安裝 binary。
- [歷史正式冷建置摘要](../history/evidence/2026-10-24-formal-f3f8255/SUMMARY.json)、[九筆歷史切片證據](../history/evidence/2026-10-23-browser-sqlite-9c2b7cf/archive-manifest.json) 與 [舊 CI 補充](../daily/day-34/CI_SUPPLEMENT.md) 都限其原 SHA，不能當成最新完成證據。
- 本輪只重新讀來源、盤點追蹤檔與核對 CI；沒有產品修改、新 runtime 測試、GUI／瀏覽器或全產品功能驗收。既有測試證據、文章／進度與歷史雜湊保留。

## 後續調整

[同一份後續調整清單](../../FUTURE_WORK.md) 記錄清除、先遷有效測試再清理與保留條件；[現行流程](../../requirements/USER_FLOWS.md) 與 [UI 條件](../../UI_ALIGNMENT_REQUIREMENTS.md) 仍是功能驗收依據。

[本頁改寫前狀態與歷史原文](https://github.com/williechen/copilot_use/blob/5d66ad95b96a7fffd85856eb80676ddd7120baa6/docs/teaching/current/STATUS.md) 保留其原始來源與時間，不以本次文件整理覆蓋舊驗證。
