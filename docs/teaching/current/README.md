# 現行教學文章同步

本目錄保存既有 Notion Day 13 的四節教學鏡像；原文與程式區塊保留，來源及同步狀態見 [manifest.json](manifest.json)。本次將「整合現有測試」加入各篇第 2 節 Refactor，不新增 Day，不計新功能進度；Chat／Pet／Analytics 主體仍合計 60 分鐘。

- [Chat：HTTP 建房名稱驗證與 201 Created](CHAT.md)
- [Pet：重複預覽顯示群組大小與檔案數](PET.md)
- [Analytics：來源隔離的 PostgreSQL 統計查詢](ANALYTICS.md)

測試重構依參考庫 `3e58e650bd63bcd21ba58e7aabf751542401ef80` 盤點：11 個有內容的測試檔歸入擁有行為的 app／crate，更新 Cargo／CI，再移除根 tests 套件與空目錄。由使用者套用至 rust_ai_chat；助手維持該庫唯讀。重構尚未套用、搬移後測試尚未執行，原教學的 CI 證據不作本次重構驗收。
