## 目前進度（2026-10-22｜main aaa9194 增量驗證）

最新已核驗來源 rust_ai_chat/main `aaa9194a966aefae1ef4724f5d2b576b4289c550`，教學基準 `9716c24f7b3c87a1337f91a82ad10635db598591`。隔離副本170個tracked檔案與Git blob逐位元組一致，產品原始碼與遠端未修改。相較前次a06c038，只改Analytics測試及Persistence workflow；production handler、UI、migration與Cargo／lock未變。

**最新SHA：預設stats-server測試20通過／1ignored；隔離PostgreSQL加 --include-ignored 後31通過、0失敗、0ignored。** Day24的mail／email與兩處SQL引號錯誤已修正並實跑通過。183a058／13a60ed曾因取消ignore而在缺DB URL時失敗；aaa9194恢復ignore後預設測試已通過，不能再把先前的NotPresent失敗寫成最新結果。被略過的PG測試實際讀DATABASE_URL，雖然ignore提示仍保留舊變數名。

最新同SHA Test All九個job均success；Dashboard亦success，Node failure，其他workflow擷取時尚在執行者保留in_progress。Persistence workflow已擴大到全部PG targets，但沒有 --include-ignored，因此仍會略過上述登入／查詢／登出PG測試；若要宣稱CI全部31項執行，需在已配置PG的job加上該旗標或等效命令。本輪只記錄覆蓋範圍，未改來源workflow。

本輪未重跑workspace、fmt／Clippy、WASM、真HTTP、Pet GUI或舊庫升級。前次a06c038的137通過／1ignored不冒稱新SHA整套結果；相關程式與依賴未變的HTTP／UI／DB觀察只保留原SHA與範圍，見[前次報告](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-22-main-a06c038/REPORT.md)。

保留缺口：Chat共用登入／登出／Cookie／Session與API／WS房間ACL；Pet單圖像動畫及此SHA三窗GUI；Analytics角色／來源授權及歷史庫checksum。舊庫migration未改：前次Pet自4f0cd00升級阻擋、自Day24成功；Analytics自4f0cd00成功、自Day24阻擋，均非本輪新實測。

三案Day25（含）以前正文與程式封存，next_day均維持26，last_full_comparison不變。後續補教按各專案既有佇列FIFO，在下一個適用原教學的新Red／完整Green／Refactor後加入，每篇最多一個舊Day單元；不預先指定新題目、不回寫封存原文、不因補教或文件同步增加Day。copilot_use產品未重新驗收，不宣稱完整產品功能通過。

- **Analytics Day24：pending。** fixture已修正並驗證；仍須核對實際歷史migration／checksum，提出並驗證保留資料的升級方案，再完成正式補教發布、全文讀回及狀態同步，才能整筆結案。
- **Chat Day25：pending。** 共用登入／登出／Cookie／Session要求列入Chat後續FIFO補教；不能以房間頁Gate測試通過當作共用實作已完成。
- **Pet Day25：pending。** 單圖像動畫及對應三窗原生GUI驗證列入Pet後續FIFO補教；每窗單一元件、每幀單一圖像與事件同步分別驗證。

[本輪完整驗證與原始證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-22-main-aaa9194/REPORT.md)。依使用者說明排程內容已更新，本輪未再修改、啟停、新增或觸發排程，亦未重新驗證實際模型設定。2026-10-22依使用者要求開啟通知，已實際檢查ChatGPT設定→通知→任務：推播與電子郵件皆已勾選開啟。這是帳戶的「任務」通知分類；查驗時本來即為開啟，本輪無須切換、未變更其他通知分類。此實際介面觀察取代先前依使用者回報的「關閉」快照。未測試真實通知送達，結果仍以任務紀錄、Notion文章與GitHub證據讀回確認。
