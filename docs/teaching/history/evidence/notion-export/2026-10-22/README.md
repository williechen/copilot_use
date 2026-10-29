# Notion 三專案教學完整搬遷

共同主頁、三專案入口與封存子頁共 **86 頁**，按頁面子樹遞迴擷取。原文逐字新增保存，包含程式碼；metadata 保留來源頁 ID、URL、編輯時間、原始完整 fetch 文字及父子引用。原文中的歷史規則與進度是當時紀錄，現行政策以 [GitHub 共同規則](../../../COMMON_LEARNING_RULES.md) 與 [learning-state](../../../learning-state.json) 為準。

文章按 day／專案分層，頁 ID 保留不同修訂與補跑版本；未帶 Day 的入口放 pages/。原始 Notion 連結不改寫以保留原文，請用下表或 manifest 的 ID 映射在 GitHub 導覽。此搬遷不新增教學 Day、不變更 next_day、makeup_queue、last_full_comparison，不回寫既有 GitHub 封存正文。使用者已確認：精確 commit 全文讀回後，這些教學頁全部移到 Notion 可還原垃圾桶，不永久刪除其他資料。

搬遷與清理已完成：86 頁逐頁 GitHub 全文比對通過、Notion 86 頁垃圾桶狀態讀回通過。原排程提示詞已原地更新並重新載入核對，維持每日 06:00。

[全文讀回證據](readback.json) · [Notion 清理證據](cleanup.json) · [原排程更新證據](scheduler.json) · [結構與來源 manifest](manifest.json) · [每日新文章入口](../../daily/README.md)

| Day | 專案 | 原頁標題與完整正文 |
| --- | --- | --- |
| 入口 | 共同／封存 | [📦 文件與教學封存（非現行規則）](pages/3f7e51d4509381faa894d3660c3252f7.md) |
| 入口 | 共同／封存 | [🦀 Rust AI 三專案｜共同學習規則與前進目標](pages/3f2e51d4509381ddb8a8ce7cb4cd3e60.md) |
| 入口 | analytics | [📊 統計儀錶板｜聊天室 × 桌面寵物](pages/3f3e51d45093817eafaee0cfc9ea4cc8.md) |
| 入口 | analytics | [2026-10-13｜手動補跑｜Analytics｜現有 EventInput 的有效識別碼前置驗證](pages/3f8e51d4509381689c98ed86fecc50a2.md) |
| 入口 | analytics | [2026-10-13｜正式補跑｜Analytics｜來源與版本驗證測試](pages/3f8e51d45093816e96edd35b83543ed3.md) |
| 入口 | chat | [2026-10-13｜手動補跑｜Chat｜既有 MessageRepository 的 SQLite 遷移切片](pages/3f8e51d45093816690f8d5b4ff04ff70.md) |
| 入口 | chat | [2026-10-13｜正式補跑｜Chat｜持久化成功後通知的對稱測試](pages/3f8e51d450938174a707d200f2e0de52.md) |
| 入口 | chat | [聊天室](pages/3f3e51d45093807da638c336e857543a.md) |
| 入口 | pet | [2026-10-13｜手動補跑｜Pet｜既有 CleanupOperation 狀態機的重入防護](pages/3f8e51d45093815fbec6ec85bc547bd6.md) |
| 入口 | pet | [2026-10-13｜正式補跑｜Pet｜成功事件契約測試](pages/3f8e51d4509381bf9da4c44750ffc726.md) |
| 入口 | pet | [桌面寵物](pages/3f3e51d450938067923ae5a424d30950.md) |
| 2 | analytics | [Phase 2｜Product Analytics Day 2｜事件驗證補測與重構（待驗證）](day-02/analytics/3f4e51d4509381bcbddccfba35f4b329.md) |
| 2 | chat | [Phase 2｜Chat Day 2｜WebSocket 房間守衛補測（待驗證）](day-02/chat/3f4e51d450938170954ff09378afc11a.md) |
| 2 | pet | [Phase 2｜Desktop Pet Day 2｜掃描 Bridge 契約補測（待驗證）](day-02/pet/3f4e51d4509381e7ac5cea661af8c67d.md) |
| 3 | analytics | [Phase 2｜Product Analytics Day 3｜事件類型長度限制補測（待驗證）](day-03/analytics/3f4e51d4509381ebb781cf9b2f2f0db6.md) |
| 3 | chat | [Phase 2｜Chat Day 3｜訊息內容規則測試（待驗證）](day-03/chat/3f4e51d45093819c975fc886e09fe861.md) |
| 3 | pet | [Phase 2｜Desktop Pet Day 3｜掃描預覽契約防止路徑注入（待驗證）](day-03/pet/3f4e51d4509381c482a3c68721761dee.md) |
| 4 | analytics | [Phase 2｜Product Analytics Day 4｜HTTP Router 200／400／422／501／413 契約補測（待實作／待驗證）](day-04/analytics/3f5e51d45093811a8ac5dafd3cd3bbcc.md) |
| 4 | chat | [Phase 2｜Chat Day 4｜持久化成功後才廣播的可執行契約測試（待實作／待驗證）](day-04/chat/3f5e51d45093811ab62acc4bc8fc83fa.md) |
| 4 | pet | [Phase 2｜Desktop Pet Day 4｜PetProfile 基本設定契約（待實作）](day-04/pet/3f5e51d4509381ebadf2d9adc6210e80.md) |
| 5 | analytics | [Phase 2｜Product Analytics Day 5｜事件去重鍵與冪等契約（待實作／待驗證）](day-05/analytics/3f5e51d45093813cabb7e6a48ee3a0f0.md) |
| 5 | chat | [Phase 2｜Chat Day 5｜RoomHub 房間隔離與清理契約（待實作／待驗證）](day-05/chat/3f5e51d45093817e93b6ea7c37c3852e.md) |
| 5 | pet | [Phase 2｜Desktop Pet Day 5｜PetProfile 安全 JSON Codec 契約（待實作／待驗證）](day-05/pet/3f5e51d4509381739b61d517b0772094.md) |
| 6 | analytics | [Phase 2｜Product Analytics Day 6｜認證攝取與持久冪等前置契約（教學／待驗證）](day-06/analytics/3f5e51d450938160a376fde5e2fad496.md) |
| 6 | chat | [Phase 2｜Chat Day 6｜持久化與跨系統事件可靠性驗收（教學／待驗證）](day-06/chat/3f5e51d45093818ab246f8fcdd0b39f6.md) |
| 6 | pet | [Phase 2｜Desktop Pet Day 6｜預覽與實際成功操作事件分離（教學／待驗證）](day-06/pet/3f5e51d4509381f48e6ac9a9d7dd758a.md) |
| 7 | analytics | [Phase 2｜Product Analytics Day 7｜持久冪等與認證攝取契約（教學／待驗證）](day-07/analytics/3f5e51d450938138a33ddadedb5a5e98.md) |
| 7 | chat | [Phase 2｜Chat Day 7｜移除空測試與持久化事件驗收（教學／待驗證）](day-07/chat/3f5e51d45093815c82c7e32b73b567fa.md) |
| 7 | pet | [Phase 2｜Desktop Pet Day 7｜清理狀態不變量與安全事件（教學／待驗證）](day-07/pet/3f5e51d4509381cd8d7efeb14bfc1b45.md) |
| 8 | analytics | [Phase 2｜Product Analytics Day 8｜事件 Schema v1 驗證（教學／待驗證）](day-08/analytics/3f5e51d4509381f98c33c2df7ab97663.md) |
| 8 | chat | [Phase 2｜Chat Day 8｜持久化後最小產品事件契約（教學／待驗證）](day-08/chat/3f5e51d45093814cb540cf6772e9b8d2.md) |
| 8 | pet | [Phase 2｜Desktop Pet Day 8｜可信任清理事件邊界（教學／待驗證）](day-08/pet/3f5e51d450938123b43be0fe0b543ade.md) |
| 9 | analytics | [2026-10-14｜P0 Analytics Day 9｜事件類型前後空白拒絕](day-09/analytics/3f8e51d45093810d830ef3fd40e419f0.md) |
| 9 | chat | [2026-10-14｜P0 Chat Day 9｜房間不存在時禁止持久化訊息](day-09/chat/3f8e51d45093814b992eea570411bd7d.md) |
| 9 | pet | [2026-10-14｜P0 Pet Day 9｜Profile 外觀覆寫與預設值合併](day-09/pet/3f8e51d4509381c3b989f2365fd0c0af.md) |
| 10 | analytics | [2026-10-14｜P0 Analytics Day 10｜有效事件的來源與 ID 去重入口](day-10/analytics/3f9e51d4509381269984c5f49a429b5d.md) |
| 10 | chat | [2026-10-14｜P0 Chat Day 10｜防止錯誤房間查詢結果寫入訊息](day-10/chat/3f9e51d45093818a849dd8b1fbf2c045.md) |
| 10 | pet | [2026-10-14｜P0 Pet Day 10｜Profile 驗證後才解析外觀](day-10/pet/3f9e51d4509381f7ab32d0dc8b389a86.md) |
| 11 | analytics | [2026-10-15｜P0 Analytics Day 11｜PostgreSQL 事件儲存與冪等](day-11/analytics/3f9e51d45093814a8054d44c4ff9fa55.md) |
| 11 | chat | [2026-10-15｜P0 Chat Day 11｜WebSocket 寫入時重新核對房間](day-11/chat/3f9e51d4509381508c0dd3ca22cf5df7.md) |
| 11 | pet | [2026-10-15｜P0 Pet Day 11｜SHA256 重複預覽接入 Tauri 與 UI](day-11/pet/3f9e51d450938152a147c931e3e945be.md) |
| 12 | analytics | [2026-10-15｜P0 Analytics Day 12｜HTTP 來源驗證與 PostgreSQL 持久化接線](day-12/analytics/3fae51d4509381ebb1e9f8482b5bf44e.md) |
| 12 | chat | [2026-10-15｜P0 Chat Day 12｜WebSocket 房間 Typing 即時通知](day-12/chat/3fae51d45093812bbf88c3e7f51d6522.md) |
| 12 | pet | [2026-10-15｜P0 Pet Day 12｜桌面漫遊明確啟用控制](day-12/pet/3fae51d4509381d0a80ddbbaf251fb03.md) |
| 13 | analytics | [2026-10-16｜P0 Analytics Day 13｜來源隔離的 PostgreSQL 統計查詢](day-13/analytics/3fae51d4509381808a9cd813423d52d1.md) |
| 13 | chat | [2026-10-16｜P0 Chat Day 13｜HTTP 建房名稱驗證與 201 Created](day-13/chat/3fae51d4509381e499e5e02cfba58336.md) |
| 13 | pet | [2026-10-16｜P0 Pet Day 13｜重複預覽顯示群組大小與檔案數](day-13/pet/3fae51d45093814dbdabee6deac48554.md) |
| 14 | analytics | [2026-10-16｜P0 Analytics Day 14｜依事件類型篩選來源統計](day-14/analytics/3fbe51d4509381f2a9a2e4b608706eea.md) |
| 14 | chat | [2026-10-16｜P0 Chat Day 14｜以 HTTP 讀回 SQLite 房間資料](day-14/chat/3fbe51d450938149af5dd68abc6109bd.md) |
| 14 | pet | [2026-10-16｜P0 Pet Day 14｜主動掃描授權目錄與摘要顯示](day-14/pet/3fbe51d4509381e484f4eb9e1f98fc68.md) |
| 15 | analytics | [2026-10-17｜P0 Analytics Day 15｜Dashboard 查詢與篩選接線](day-15/analytics/3fbe51d4509381319460de180d9e173d.md) |
| 15 | chat | [2026-10-17｜P0 Chat Day 15｜首頁房間列表與選房流程](day-15/chat/3fbe51d450938130a3c0c93171105f3a.md) |
| 15 | pet | [2026-10-17｜P0 Pet Day 15｜掃描後確認與取消流程](day-15/pet/3fbe51d4509381d68666d3031d1c8011.md) |
| 16 | analytics | [2026-10-17｜P0 Analytics Day 16｜登入 Leptos 後台並查詢](day-16/analytics/3fce51d45093813fa38ae37a47ae4959.md) |
| 16 | chat | [2026-10-17｜P0 Chat Day 16｜清楚選房與聊天](day-16/chat/3fce51d450938143a0b9cc9a3f93860e.md) |
| 16 | pet | [2026-10-17｜P0 Pet Day 16｜寵物生活與控制雙視窗](day-16/pet/3fce51d4509381e9aa03d633822eee89.md) |
| 17 | analytics | [2026-10-18｜P0 Analytics Day 17｜活動延長管理員 Session](day-17/analytics/3fce51d4509381ea871ec9c92da37c1b.md) |
| 17 | chat | [2026-10-18｜P0 Chat Day 17｜查詢房間接通 HTTP](day-17/chat/3fce51d45093818b913bebb8dab5ab65.md) |
| 17 | pet | [2026-10-18｜P0 Pet Day 17｜漫遊指向寵物生活窗](day-17/pet/3fce51d4509381ca83e3e825f0313698.md) |
| 18 | analytics | [2026-10-18｜P0 Analytics Day 18｜有效活動延長伺服器 Session 並拒絕過期](day-18/analytics/3fde51d4509381c2a0d5f60041137619.md) |
| 18 | chat | [2026-10-18｜P0 Chat Day 18｜查詢房間完成 HTTP 接線與錯誤恢復](day-18/chat/3fde51d45093816bb36dd9099190d907.md) |
| 18 | pet | [2026-10-18｜P0 Pet Day 18｜漫遊只移動生活窗並核對真實標籤](day-18/pet/3fde51d4509381bfa106ea4d9b190480.md) |
| 19 | analytics | [2026-10-19｜P0 Analytics Day 19｜成功登入不消耗失敗額度](day-19/analytics/3fde51d4509381ed8a2eecf3ba7e4093.md) |
| 19 | chat | [2026-10-19｜P0 Chat Day 19｜重新整理失敗保留房間列表](day-19/chat/3fde51d4509381d2aac7ea4b356c213b.md) |
| 19 | pet | [2026-10-19｜P0 Pet Day 19｜觀察到非預期位移時停止漫遊](day-19/pet/3fde51d450938194ae7fdcc55bce44e2.md) |
| 20 | analytics | [2026-10-19｜P0 Analytics Day 20｜登入限流回應提供剩餘等待秒數](day-20/analytics/3fee51d4509381618739f23dcd766996.md) |
| 20 | chat | [2026-10-19｜P0 Chat Day 20｜重新整理後顯示完整房間快照](day-20/chat/3fee51d4509381d1a1e6eb5e5a1c996e.md) |
| 20 | pet | [2026-10-19｜P0 Pet Day 20｜可用工作區不足時停止無效漫遊](day-20/pet/3fee51d45093816897feec3468239e43.md) |
| 21 | analytics | [2026-10-19｜P0 Analytics Day 21｜管理員帳號欄位接通登入請求](day-21/analytics/3fee51d450938178bdb6c37fede38fd4.md) |
| 21 | chat | [2026-10-19｜P0 Chat Day 21｜登入頁可達但不偽裝已登入](day-21/chat/3fee51d450938168aaa0c11d9ad7d827.md) |
| 21 | pet | [2026-10-19｜P0 Pet Day 21｜新增寵物表單與名稱 IPC 前置接線](day-21/pet/3fee51d450938163b451e803639e657c.md) |
| 22 | analytics | [2026-10-20｜Analytics Day 22｜PostgreSQL 管理員初始化【已封存】](day-22/analytics/3fee51d450938103b0eafc50f32822c1.md) |
| 22 | chat | [2026-10-20｜P0 Chat Day 22｜會員註冊表單與 SQLite【已封存】](day-22/chat/3fee51d45093814385cdcdc9e693789a.md) |
| 22 | pet | [2026-10-20｜Pet Day 22｜建立寵物與 SQLite 儲存【已封存】](day-22/pet/3fee51d45093811c83eed982ef9ab69e.md) |
| 23 | analytics | [2026-10-20｜Analytics Day 23｜帳密登入與 PostgreSQL 受限 Session【已封存】](day-23/analytics/3ffe51d4509381f88d1be27bc5ec5f9f.md) |
| 23 | chat | [2026-10-20｜P0 Chat Day 23｜帳密登入與持久 SQLite Session【已封存】](day-23/chat/3ffe51d4509381cf8532fb767afc33ab.md) |
| 23 | pet | [2026-10-20｜Pet Day 23｜控制台載入已儲存寵物清單【已封存】](day-23/pet/3ffe51d450938198871cddfdb6f7044a.md) |
| 24 | analytics | [2026-10-21｜Analytics Day 24｜首次改密碼與唯一Email【已封存】](day-24/analytics/3ffe51d4509381c88f26f4a90c2958fc.md) |
| 24 | chat | [2026-10-21｜Chat Day 24｜註冊完成頁保持登出【已封存】](day-24/chat/3ffe51d4509381ee9326ce6cc7f1c8a1.md) |
| 24 | pet | [2026-10-21｜Pet Day 24｜兩隻以上獨立生活窗與出入籠 r2【已封存】](day-24/pet/3ffe51d450938186b15fd1c014df8edd.md) |
| 25 | analytics | [2026-10-21｜Analytics Day 25｜已初始化管理員再次修改密碼與 Email（未套用）【已封存・不可再編修】](day-25/analytics/400e51d45093810796cbd6074c8d35c3.md) |
| 25 | chat | [2026-10-21｜Chat Day 25｜聊天室頁登入 Session Gate（未套用）【已封存・不可再編修】](day-25/chat/400e51d450938155a6a1c5dee070f1b8.md) |
| 25 | pet | [2026-10-21｜Pet Day 25｜動態生活窗事件權限與 Day24 補教（未套用）【已封存・不可再編修】](day-25/pet/400e51d4509381d69745c299feea5a8f.md) |
| 26 | analytics | [2026-10-22｜Analytics Day 26｜PostgreSQL Session schema 與登入契約【未套用】](day-26/analytics/400e51d4509381d28023deaccdef65bf.md) |
| 26 | chat | [2026-10-22｜Chat Day 26｜登出立即撤銷單一 SQLite Session【教學・尚未套用】](day-26/chat/400e51d45093811ea480d6d75449afe4.md) |
| 26 | pet | [2026-10-22｜Pet Day 26｜生活窗每幀單一寵物圖像【教學・GUI待驗收】](day-26/pet/400e51d4509381cc8510dcb681580d5b.md) |
