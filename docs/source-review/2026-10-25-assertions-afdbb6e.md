# Analytics 舊文案斷言修復驗證：afdbb6e

2026-10-25（Asia/Taipei）。本地由 bb95a60 fast-forward 到 `afdbb6e81382e8d82095827a90c52cc3309a3e20`（fix test）；三個測試檔共四行斷言更新，產品工作樹乾淨，未由本輪另行修改／推送產品程式。

## 修正內容

- Day29 可及性測試改驗「全部事件類型：3 筆已提交事件」，保留 desktop_pet 與 aria-current 檢查。
- Day28 Dashboard 測試改驗「全部事件類型：2 筆已提交事件」，保留兩個來源連結、修改密碼與秘密不外洩檢查。
- Day27 Session／來源切換測試的 Chat 與 Pet 兩處，分別改驗「全部事件類型：1／2 筆已提交事件」，保留真實 PostgreSQL、帳密／Session、HTTP 狀態、來源切換與未知來源拒絕。

新字串與現行 summary_count_label 的語意一致。此提交沒有變更正式 UI 文案、查詢邏輯、Session 或資料庫 schema；也沒有刪除失敗測試。完整來源標籤與精確事件類型文案另由既有 summary_count_label 測試驗證。

## 實際驗證

```bash
cargo test --locked -p analytics-server -p analytics-persistence --features postgres-integration --no-fail-fast
```

使用本輪新建的隔離 PostgreSQL 18.6 review DB，執行所有上述 package 的測試目標與 doc tests，結果 **44 passed／0 failed／1 ignored，exit 0**。Day27／28／29 三個原失敗測試全數通過；先前五個 E0425 亦未再出現。Ignored 是既有 legacy admin 測試，沒有算入通過數。review DB 已於驗證後移除。

證據：[完整日誌](update-afdbb6e/tests.log)、[機器摘要](update-afdbb6e/summary.json)。本輪是本機回歸結果，未宣稱新 SHA 的 GitHub CI 或瀏覽器／GUI 已通過。

## 後續狀態

[bb95a60 報告](2026-10-25-compile-bb95a60.md) 的三個文案斷言失敗保留為歷史，現行待辦可移除這三敗；原識別 Analytics 5 保留修復來源及本次證據。編譯與這組 PostgreSQL 回歸已通過，不代表全產品功能驗收。

新精確查詢以來源 token 代替人員 Session 的需求落差、舊公開 pg_store 路徑相容性、角色 ACL／SSE／明細／服務配對等缺口不因文案測試修復而結案。Chat／Pet 此提交未變更，沿用原來源證據；已發布 Day35 正文與雜湊保留，三案下一篇仍是 Day36，FIFO／last_full_comparison 不變。
