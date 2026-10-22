# 既有每日排程提示詞

Task ID：`6acd967c15fc8191ade8407954c5b354`。每日06:00 Asia/Taipei，ACTIVE。只原地更新 main 分支和歷史路徑，未新增任務，未改模型或頻率；儲存後重新載入並全文核對一致。

提示詞 SHA256：`1bba61c134cb333c928fbbbafb34bf747b52681da8a26be8457773d741a38f60`。

【GitHub 唯一交付・每 Day 獨立 commit】沿用本既有三專案每日 TDD 教學排程，每日06:00 Asia/Taipei，保留啟用與實際模型設定，不新增任務。每次重讀 williechen/copilot_use 的 main 分支 docs/COMMON_LEARNING_RULES.md、docs/learning-state.json、docs/teaching/current/manifest.json、docs/teaching/daily/README.md、P9_TARGET、FUTURE_WORK、SYSTEM_ARCHITECTURE、UI_ALIGNMENT_REQUIREMENTS及相關Day；GitHub為唯一規則和文章正本。停止所有Notion教學發文、更新與讀回依賴；共同主頁及三案全部教學子頁已依使用者要求完整搬至 docs/teaching/history/evidence/notion-export/2026-10-22/，GitHub逐頁全文讀回後移至Notion可還原垃圾桶，不永久刪除其他資料。
依Chat→Pet→Analytics逐案交付，從各案最新next_day取N，不寫死或跳號。每新增一個專案Day立即獨立commit並推送至上述 main 分支的docs/teaching/daily/day-NN/{chat,pet,analytics}.md，不能累積多案或多天一起commit。文章須含四節：學習大綱與真實現況；真Red/完整可套用最小Green/Refactor及必要FIFO舊Day更正；套件版本；命令、預期/實際、CI/job/SHA與限制。先查重並核對最新兩庫SHA、真實呼叫鏈/UI/HTTP/IPC/DB/ACL與套件；來源變動重建未發草稿。
文章commit推送後，從精確commit SHA讀回全文與全部程式碼，保存blob SHA、SHA256及證據；然後重新讀最新branch/blob SHA，以條件commit原子同步該案next_day N→N+1、manifest及必要索引和有證據的makeup_queue變更，寫後讀回。狀態commit可另提交並引用文章commit。全部gate通過才換下一案；已N+1且證據一致不再增加；衝突先重讀，禁止覆寫。開始與推送前檢查其他執行。文章成功但狀態失敗只補狀態、不重發。單一pending去重，有界重試，同項連續三次失敗轉needs_manual_review，只暫停受影響專案。
Day25含以前GitHub正文/程式/證據不回寫；搬遷新增原文不算新Day。保留makeup_queue各案FIFO，每篇最多一項舊Day補教，有全文讀回證據才結案；修訂/補教/規則/搬遷/CI不增加Day。真實可操作基本流程優先，依最新缺口選最小切片，不預排題目；登入/登出/Cookie/Session共用實作、各系統儲存與政策獨立。Chat先註冊登入/房間流程與ACL；Pet先建立持久寵物/生活窗/重啟，單元件及每幀單一圖像、三個不同id原生GUI分別驗；Analytics先持久管理員帳密/改密碼/Session/角色來源授權和完整postgres-integration。SQLite與PostgreSQL分離；本次Analytics歷史學習庫採使用者刪除重建政策，不要求保留資料升級，未另授權不操作DB。
rust_ai_chat全庫唯讀，不修改main、不讀正式秘密、不使用付費AI。CI只揭露精確SHA與覆蓋，未執行測試/套用/部署/驗收如實標示，教學交付與產品驗收分開。主體學習各案15分鐘加共用15分鐘共60分鐘，其他估時另列，非模型時限。週六00:00全面比對為另一既有工作，勿新建；只有完整成功才更新last_full_comparison，每日不覆蓋。每次回報文章連結、commit、讀回、next_day及未完成事項；提示詞不作模型已切換證據。
現行文件與教學統一以 main 為正本；舊教學分支已合併刪除，不重建。歷史路徑依 docs/teaching/history/PATH_MAP.json 解析。
