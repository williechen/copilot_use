# 各案重構現況

| 專案 | 已見正式來源 | 接續範圍 |
| --- | --- | --- |
| Chat | Session 入口決策、room_list_state 與正式 UI caller；SSR fixture imports 已修 | 認證／Session SQL 用例邊界，沿 D05／D06／D07 保留錯誤與狀態斷言 |
| Pet | UUID menu 解析、Domain／Application 與部分 persistence、逐 ID 生命週期 | Store SQL／Port、native 事件與錯誤補償、legacy global 入口與有效測試承接 |
| Analytics | Dashboard URL／標籤／CSV、Session、PG adapter 與部分聚合 | 正式登入／查詢用例邊界、範本與 legacy admin 盤點，保留 PG／HTTP／Session 斷言 |

來源 `7ce39f0166a2120539e5908b33c88322a930c35a`；來源存在不等於每項完整產品驗收。原 D01–D09 與 13 項清理識別／條件見 [FUTURE_WORK](../../FUTURE_WORK.md)，功能補教沿既有 FIFO。跨案 G-01／G-02 只在 [engineering](../../engineering/README.md) 計量。
