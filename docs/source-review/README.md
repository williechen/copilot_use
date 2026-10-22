# 靜態核對證據

> 歷史雙庫靜態比較，保留原文、固定 SHA 與結果；其中「成品／參考」是當時命名，不是現行儲存庫分工。現行主體只為 rust_ai_chat，分工見 [共用規則](../COMMON_LEARNING_RULES.md)。本次未重新執行全量比較。

核對範圍及固定來源見 [完整報告](../FULL_SOURCE_REVIEW.md)。可閱讀的介面、流程及模組映射在此目錄；[完整證據包](evidence.zip) 包含全部 AST、反向名稱引用、tracked snapshot、IPC、Cargo／SQL／Proto／CI／scripts、原核對工具與原驗證结果，不含編譯產物或秘密。

證據對應核對前 source SHA，不因文件更新而重寫其結果。report-validation.json 是當次原始核對結果；文件整理與證據包檢查另見 documentation-validation.json。SOURCE連結指向固定 SHA 的GitHub檔案，不依賴本地參考庫路徑。

核對完成與產品缺口修正、週六功能驗收、每日 CI／Day、學習套用各自獨立。第三方 vendor 是解析列冊，未宣稱完整人工安全審計。

可閱讀索引已清除行尾空白；完整 ZIP 保留核對原始資料。

歷史定點判定見 [狀態表](RUST_AI_CHAT_STATUS.md)（bded382 本地及同SHA GitHub CI通過；Day12新功能尚待套用）。當前學習參考庫另見 [先本地、後 GitHub 驗證](rust-ai-chat-validation.json)、[驗證日誌／工具](rust-ai-chat-validation-logs.zip)；這是後續定點驗證，不改寫原完整靜態核對的來源與結果。
