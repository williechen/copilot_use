# Day45 已更新核對｜9ae3c6cf

產品精確來源`9ae3c6cf42b231004e81be0097f50840b0a15eb5`，文件基準`220472ead81adeba05edbdf3d7ad24bb3d541f6e`。產品唯讀，修改與測試只在隔離副本進行。[命令／exit／限制](review.json)、[完整原樣log](analytics.log)、[補正候選log](analytics-fixture-fixed.log)、[來源位元與子樹核對](source.json)。

## 已採用

前次ootstrap匯入、角色表r別名及SQL字面+均已修正；舊migration也已恢復為真正f6位元，新增角色／頁面管理由20261031180000向前migration保存。原runtime與migration修正patch已採用，不可重複套用。

Chat／Pet子樹與d784完全相同，沿用明確標成d784的Chat44、PetUI11／六pose證據；本回合不重標為9ae3實跑。本回合對9ae3實際執行Analytics完整postgres-integration：**67通過／1失敗／1 ignored，exit101**。唯一失敗為f6歷史fixture的檔尾換行。

## 剩餘Red與完整Green

真正f6 SQL原檔沒有最後LF；新版fixture被編輯器補了LF，SQL語義相同但SQLx checksum不同。來源migration本身已正確恢復f6，不能把fixture錯誤說成產品migration仍被改寫。

[完整單檔patch](fixture-exact-bytes.patch)只改tests/day45_final.rs：讀入兩份歷史fixture時只容許去掉一個已知的可選檔尾LF，再核對固定的歷史SHA256。其他字元、空白、CRLF或額外換行仍會被hash斷言拒絕。migration、runtime及依賴皆不改。

修正候選以同一完整命令重跑：**68通過／0失敗／1既有legacy ignored，exit0**。包含凍結f6七→九份SQLx升級、重新執行、角色／頁面／CRUD與原checksum保留；也驗證已執行28改寫版ledger會被拒絕，且不會偷偷更新_sqlx_migrations。[patch乾淨套用與逐檔相同](patch-validation.json)。候選結果不標成產品原樣通過。

若真實DB已跑過28改寫版migration，仍須按備份明確處理ledger差異；本次沒有真實備份，不自動修改紀錄。這不影響空DB／真正f6 ledger的候選升級測試結果。

## Refactor與交付界線

沿用Day45本案PG Session Adapter及既有有效斷言，本回合沒有新增／重複Refactor計功。Chat／Pet未改，不做跨案或全庫重構。臨時PostgreSQL已停止，隔離副本與測試DB已清除，[清理紀錄](cleanup.json)保留；原文章、patch、log、SHA證據保留。

next_day三案46，未新增Day，不改FIFO與last_full_comparison。Gate1為補正文件交付；完整Gate2仍false，包含1 legacy ignored、真實DB備份、瀏覽器及桌面驗收等缺口。CI只作[快照](ci.json)，不抵銷原樣測試失敗。
