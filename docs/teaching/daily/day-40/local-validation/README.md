# Day40真正本地驗證（2026-10-26 Asia/Taipei）

## 發布前來源更新：b0e06b7

產品遠端在初輪驗證後新增此SHA，只改Pet原生事件分支。已更新本地並重新在隔離clone執行Pet fmt／check／server57＋Application8＋SQLite4，共69項通過；原生CloseRequested已接，但3個舊測試仍缺。額外[恢復舊斷言patch](../pet-day40-restore-tests-b0e06b7.patch)實際check／apply後，fmt通過、server60項通過；原始產品未套此恢復patch。真GUI仍未驗。

Chat、Analytics與共用crates的Git tree與初輪ff14ca6完全一致，保留原測試SHA與結果，不宣稱在新SHA重跑：Chat57通過／2失敗，Analytics53通過；Chat/account回歸未變。[來源刷新、tree比對與新命令](source-refresh.json)。下方latest表專指初輪ff14ca6，不覆蓋此補正。

最新程式 `ff14ca6224f7c5a65c799bf4a82a743fa79c880c`；文件基底 `715b19794c898b06f72307feefca996c32ca6d0b`；教學patch基底 `bb259aaf76542554a3a704d9c4f4a3a219d65cbf`。

| 範圍 | Chat Rust | Pet Rust | Analytics Rust |
| --- | --- | --- | --- |
| candidate | 59通過／0失敗 | 72通過／0失敗 | 53通過／0失敗 |
| latest | 57通過／2失敗 | 69通過／0失敗 | 53通過／0失敗 |
| followup | 59通過／0失敗 | 60通過／0失敗 | 未執行 |


candidate：各案bb基底先Refactor再Green，Chat／Analytics使用修訂patch。原始標準apply失敗、scratch診斷、修訂後乾淨worktree檢查及實際套用全部保存，沒有將scratch的padding說成原patch可套用。[patch更正](patch-corrections.json)。候選三案fmt均失敗；最新三案fmt均通過。兩組server check、Chat hydrate／Pet WASM均通過；Pet兩組另各Node6項通過。

latest Chat首次workspace因舊登入/account收到404而非401退出101；後以--no-fail-fast完整執行，保留兩輪log，摘要只取後者避免重複計數。最新Pet少原生CloseRequested，且比候選少3個舊測試；不是完整按X回籠。

followup：只在最新SHA額外隔離worktree套Chat補回/account、Pet補native分支與恢復三個舊測試；不是已修改原始產品，也不等於真GUI通過。Analytics ignored舊PG測試在專用庫另補跑；預設suite內的ignored不偽裝已跑，摘要passed加補跑的1項且不重複計數。

[全部命令／cwd／exit code](commands.md) · [JSONL與log SHA256](commands.jsonl) · [結果JSON](summary.json)。本地PostgreSQL專用庫與正式migrations實際執行，不連產品庫；Node使用既有JS bridge測試。

未跑：移除實作後新Red、真瀏覽器hydration／登入與CSV下載、Pet真多窗／主副螢幕／native故障注入、試算表匯入、舊產品DB升級、release安裝與GitHub Actions。工具版本和warnings見log；既有CI不追認最新來源。FIFO、next_day=41及last_full_comparison不變。

本輪專用測試庫、約3.5GB編譯快取與全部隔離來源副本已清除，命令與exit code列於ledger末尾；[最終磁碟空間](day40-logs/final-disk.log)。原始產品／文件repo及備份保留。

恢復舊斷言附件首次fmt仍失敗，修訂後最終patch重新套用、fmt與60項server測試通過：[原始失敗及最終patch hashes](restoration-format.json)。第二輪快取與來源副本亦已清除，[最終空間](day40-logs/refresh-final-disk.log)。

三篇Day40 r1與三篇Day39 r3已於GitHub精確commit全文讀回並比對bytes／blob／SHA256：[遠端讀回證據](../r1-remote-readback.json)。文件發布成功不改變上述產品失敗與未驗項目。
