# 調整項目重新掃描：afdbb6e

2026-10-25（Asia/Taipei）。產品精確來源 `williechen/rust_ai_ring/main@afdbb6e81382e8d82095827a90c52cc3309a3e20`，與前次文案修復驗證相同；文件基準 `williechen/copilot_use/main@bba8fc5f53b368455eab4482bea52ed237b9a26d`。本輪盤點 202 個追蹤檔案、78 個 Rust 檔，逐項重查原 13 項、呼叫端／route／Tauri 註冊／JS bridge／測試及 CI；沒有修改產品或新增教學 Day。

## 原 13 項目前狀態

7 項仍待清理／測試遷移／相容性盤點，2 項部分完成，2 項程式／建置已處理但驗收未完，1 項明確保留，1 項已結案。這是原分類的狀態統計，並非另開 13 個待辦。詳細判準見 [單一後續調整清單](../FUTURE_WORK.md)。

| 原項目 | 最新狀態 | 下一步與保留條件 |
| --- | --- | --- |
| Chat 1／D01 | 登入頁合併已實作，完整驗收待做 | 保留 SignInPage、GET /login redirect、POST /login；不再套一次 r2。沿登入／Session／瀏覽器條件驗收 |
| Chat 2／未使用函式 | 待清理 | 私有 room_api 的 create_room 仍沒有 caller；移除其專屬 DTO／imports 後依實際剩餘使用調整 web-sys features。server API、個人頁建房與 list/search 保留 |
| Chat 3／D06 | 待測試遷移 | refreshed_room_list 只被舊測試用；正式 RoomList 用 refresh_result_view。Day20 已覆蓋成功／失敗 query 行為，先映射 Day19 空結果／保留／重試斷言再刪 helper |
| Chat 4／D05、D07 | 部分完成 | 房名規則已接 application 與兩種 transport；Cookie／digest／認證共用仍待處理。原共用認證 FIFO 不因名稱規則完成而結案 |
| Pet 1／D02 | 舊產物已清，CI release build 成功，GUI 待驗 | 舊 dist／public／gen 不再追蹤；以 source 重建。將同一契約測試加入新 dist 建置後；實際 binary／三窗 GUI／安裝更新仍待驗 |
| Pet 2／未使用宣告 | 待清理 | validate_new_pet_name_js 孤立 extern 仍在，JS 無 validateNewPetName；server 同名驗證 IPC／PetStore 規則是另一入口，不能一併誤刪 |
| Pet 3／D03 | 待相容性盤點／清理 | get_pet_state/send_pet_command 舊 IPC、全域 Mutex、pet://state-changed 與無 ID menu fallback 仍註冊。UI 的同名 Rust wrapper 實際映射新逐 ID JS 通道，必須保留 |
| Pet 4／D08 | 待清理 | 兩個掃描 handler 的 status reset／loading setter 仍重複；保留 await 後及過時 generation 早退的 busy 釋放，沒有證據可稱雙重 scan invoke |
| Analytics 1／D09 | 待清理 | application/domain/ui 三份 add／it_works 範本仍相同且無業務 caller；保留 workspace。不再把已接通的 persistence 當空 library |
| Analytics 2／D04 | 待相容性盤點／清理 | legacy admin.rs 仍公開編譯；main/build_app 使用 password_login。確認外部 API 使用後再連同舊 Config／註冊／專屬測試清理 |
| Analytics 3／fixture | 明確保留 | bootstrap::initialize 仍被 Day23–27 整合測試使用；ensure_default 在 startup／CLI，政策與用途不同。不能因 main 不呼叫 initialize 就刪除 |
| Analytics 4／handler／查詢 | 部分完成 | 精確查詢已移 persistence 並掛正式 route；五個 E0425 已修。但 source_summary 仍為測試 handler，正式精確查詢缺人員 Session；沿原授權／測試遷移項處理 |
| Analytics 5／舊文案 | 已結案 | 三檔四個舊字串已修；本 SHA 的本機 PostgreSQL 回歸 44 passed／0 failed／1 ignored。保留原識別與證據，不再列成待修失敗 |

## 功能缺口的具體接線位置

以下細化既有功能項，不新增另一套編號，也不宣稱本輪找到新的 runtime 回歸。

- **Chat 身分／作者／ACL**：room_auth.rs:16 驗證 active_member 後未把取得的 member ID 往 handler 傳；ws.rs:125 使用 ChatService::send_message，而 application/lib.rs:21 固定 AnonymousUser。RoomRepository::create 未綁真實房主。這些仍接回既有作者／房間授權／共用認證項，不重建第二套身分表。
- **Pet 漫遊**：roam.rs:12 固定 label=`pet`，:88 只查該窗；pet_windows.rs:33 真正建立 `pet-{uuid.simple()}`。正常生活窗無法由此查找命中，且啟動時 window 未就緒即退出迴圈的條件仍在。先修逐 ID 生命週期接線，再做三窗／拖曳／多螢幕驗收；原固定字串單元測試不能作成功證據。
- **Pet 重啟載入**：server/lib.rs:192 只呼叫 store.active() 後開一窗；active() 依 local_pet_selection slot=1 查一隻。尚未逐筆重開所有未回籠寵物，Runtime 也以 default 機器狀態建立。沿原多寵物／重啟需求追蹤，不能把 DB 回籠旗標測試當成多窗重啟完成。
- **Analytics 查詢授權**：password_login.rs:440 的新精確查詢掛 source_kind_summary；lib.rs:185 的 handler 只認來源 Bearer token，而原 summary() 走人員 Session。前次隔離 DB／真 Router 已重現「無 Cookie、有來源 token→200」。該段來源在本 SHA 未再修改；這與業務查詢須人員登入需求的落差仍在。來源隔離測試通過不足以驗收人員權限。
- **真實事件**：Chat persisted_message_event 無本庫 caller；Pet succeeded_event 只有契約測試／測試 caller。沿原 Analytics 上報／Outbox／receipt 缺口處理，不能因無 caller 就直接刪除公開契約，也不能宣稱實際事件已送達。

## CI 已綠，但兩個覆蓋缺口仍須追蹤

本精確 SHA 的七個 push workflows 均 completed/success，快照見 [ci.json](rescan-afdbb6e/ci.json)：

- [Test All](https://github.com/williechen/rust_ai_ring/actions/runs/40147802697)、[Persistence](https://github.com/williechen/rust_ai_ring/actions/runs/40147802736)、[Node](https://github.com/williechen/rust_ai_ring/actions/runs/40147802752)、[Cargo workspaces](https://github.com/williechen/rust_ai_ring/actions/runs/40147802705)。
- [Rust AI Chat](https://github.com/williechen/rust_ai_ring/actions/runs/40147802822)、[Rust Analytics](https://github.com/williechen/rust_ai_ring/actions/runs/40147802729)、[Rust Desktop Pet](https://github.com/williechen/rust_ai_ring/actions/runs/40147802689)。Desktop Pet 的 release 前端、native build、Upload build artifacts steps 均 success；沒有把 build／上傳當作 GUI 或已安裝版本驗收。

**資料層自身測試未直接納入 CI。** test.yml:47 只 `-p pet-server`；persistence.yml:66 只 `-p analytics-server --features postgres-integration`。編譯 dependency／啟用 dependency feature 不會自動執行 dependency 的單元測試，workspace check／metadata 亦不能代替。後續在原項目補上 pet-persistence 與 analytics-persistence 的測試 package 選擇；這兩組各 2 個測試已有本機成功證據。可沿用已實測命令：

```bash
# pet_apps
cargo test --locked -p pet-server -p pet-persistence
# analytics_apps，隔離 PostgreSQL 測試 DB
cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration
```

**新 dist 的 bridge 契約未納入 build 後驗證。** node.yml 讀來源 index.html，desktop_pet.yml 建出 dist 後直接 build native／上傳；後續應以同一批契約讀新 dist，對應 D02，無需另維護第二份邏輯。

## 本輪證據範圍與交付

來源沒有新增變更，因此不重複跑剛通過的相同測試：直接引用 [同 SHA 的 Analytics 44/0/1 實測](2026-10-25-assertions-afdbb6e.md)。git diff 證明 e753c36→本 SHA 的 chat_apps／pet_apps 全目錄無差異，故可沿用原範圍 Chat 35＋UI 14、Pet 55 結果；pet_ui 與 891ff32 同樣無差異，原六個 Node 契約保留其輸入範圍。這不是在新來源重跑所有測試的聲明。

盤點及來源一致性：[inventory.json](rescan-afdbb6e/inventory.json)、[summary.json](rescan-afdbb6e/summary.json)、[13 項機器清單](rescan-afdbb6e/adjustments.json)、[Pet CI job／step](rescan-afdbb6e/desktop-pet-jobs.json)。全檔相同的 toolchain／平台 icon 是合法資源，沒有列成可刪垃圾。

本次向 copilot_use 更新後續清單、統一狀態、機器索引與本報告；產品程式、已發布 Day 正文及雜湊不變。三案 next_day=36、FIFO、last_full_comparison 保持；完整功能／GUI／瀏覽器驗收未因 CI 全綠或清單整理自動結案。
