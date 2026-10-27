# 靜態核對證據

[2026-10-27 正式本地驗證（912e5e7）](local-validation-2026-10-27-912e5e7/README.md)：未套用修正；Chat／Pet Node 四項失敗重現，隔離 PostgreSQL 55 項通過。完整命令與原始輸出另存；未更動完整比對或產品驗收狀態。

**現行目錄導覽：** [SOURCE_LAYOUT](../SOURCE_LAYOUT.md)（2026-10-27 本地四 workspace tree／manifest／lock／入口核對，metadata 通過，來源 `912e5e77d8e3d2e24f0e9d804f28d86ae768c1d4`）。本頁其餘比較、測試、CI 與證據連結保留各自歷史 SHA，未重新全面比對，`last_full_comparison` 不變。


<!-- comparison-5c713553:start -->
[最新三專案程式／文章比對](2026-10-25-full-comparison-5c713553.md)：105個閱讀版本、111份修訂、21項差異；來源 `5c713553`，局部測試與完整驗收分開記錄。
<!-- comparison-5c713553:end -->

> 歷史雙庫靜態比較，保留原文、固定 SHA 與結果；其中「成品／參考」是當時命名，不是現行儲存庫分工。現行產品主體為 [rust_ai_ring](https://github.com/williechen/rust_ai_ring)，`rust_ai_chat` 僅為歷史名稱；分工見 [共用規則](../COMMON_LEARNING_RULES.md)。本次未重新執行全量比較。

核對範圍及固定來源見 [完整報告](../FULL_SOURCE_REVIEW.md)。可閱讀的介面、流程及模組映射在此目錄；[完整證據包](evidence.zip) 包含全部 AST、反向名稱引用、tracked snapshot、IPC、Cargo／SQL／Proto／CI／scripts、原核對工具與原驗證结果，不含編譯產物或秘密。

證據對應核對前 source SHA，不因文件更新而重寫其結果。report-validation.json 是當次原始核對結果；文件整理與證據包檢查另見 documentation-validation.json。SOURCE連結指向固定 SHA 的GitHub檔案，不依賴本地參考庫路徑。

核對完成與產品缺口修正、週六功能驗收、每日 CI／Day、學習套用各自獨立。第三方 vendor 是解析列冊，未宣稱完整人工安全審計。

可閱讀索引已清除行尾空白；完整 ZIP 保留核對原始資料。

歷史定點判定見 [狀態表](RUST_AI_CHAT_STATUS.md)（bded382 本地及同SHA GitHub CI通過；Day12新功能尚待套用）。當前學習參考庫另見 [先本地、後 GitHub 驗證](rust-ai-chat-validation.json)、[驗證日誌／工具](rust-ai-chat-validation-logs.zip)；這是後續定點驗證，不改寫原完整靜態核對的來源與結果。

## 正式 P0–P12 十三階段本地文章掃描

[2026-10-25 逐篇分類、重複／過時／缺口與證據矩陣](p0-p11-local-scan-2026-10-25/README.md)：更新至 `4a3c431` 後重新讀取本地原文；105 個閱讀版本、157 份正文／修訂／鏡像。未追認產品、CI 或功能驗收。

[十三階段完整本地合併規劃](phase-plan-merge-2026-10-25/README.md)：最新程式／文章、本地13階段日曆與640行詳細Roadmap；原文與產品驗收狀態保留。
