# 既有每日排程提示詞

既有 task ID：`6acd967c15fc8191ade8407954c5b354`。僅原地更新，保留每日 06:00 Asia/Taipei、啟用狀態及現有實際模型設定；不建立新任務。實際更新證據另見交付報告，保存本檔不等於已更新。

每天 06:00 Asia/Taipei 沿用既有「三專案每日 TDD 教學」任務（ID 6acd967c15fc8191ade8407954c5b354），只原地更新，不建立任何新排程。GitHub williechen/copilot_use 的 teaching/rust-ai-chat-alignment 是唯一規則、進度與教學交付正本。停止所有 Notion 教學建立、更新、發文和讀回要求；三專案既有 Notion 教學資料搬回 GitHub 後已依使用者要求移至垃圾桶，不再依賴 Notion。先讀最新 docs/COMMON_LEARNING_RULES.md、docs/learning-state.json、docs/teaching/current/manifest.json、docs/teaching/daily/README.md、docs/P9_TARGET.md、docs/FUTURE_WORK.md、docs/SYSTEM_ARCHITECTURE.md、docs/UI_ALIGNMENT_REQUIREMENTS.md 與相關 Day。取得教學分支與 rust_ai_chat/main 最新完整 SHA，核對真實程式、符號、呼叫端、UI/HTTP/IPC/DB/ACL、套件及 CI；必要來源失敗只暫停受影響專案。rust_ai_chat 全庫唯讀，不修改 main，不讀正式秘密，不使用付費 AI。

依 Chat → Pet → Analytics 逐案完成。各案從最新 learning-state.next_day 取 N，不寫死 Day、不從舊 Day 重來；開始與寫入前檢查執行中任務及最新 branch/blob SHA，拒絕併發覆寫。每新增一個專案 Day，就獨立 commit 並推送到指定教學分支（不可累積多天或多案後才一起 commit），路徑 docs/teaching/daily/day-NN/{chat,pet,analytics}.md。先查重；相同 project/Day/revision 只補缺少步驟。完整四節包含：1 學習大綱與真實現況；2 對應同篇的真 Red、完整可套用最小 Green、Refactor、必要 FIFO 舊 Day 更正（每篇最多一項）；3 Cargo/套件版本；4 命令、預期與實際、來源/CI/job/SHA/連結、限制。發文前再核對兩庫 SHA 與規則，來源變動重建未發布草稿。

文章 commit 推送後，以精確 commit SHA 從 GitHub 全文讀回，逐字核對四節、全部程式碼及來源版本，保存路徑、blob SHA、內容雜湊與讀回證據。只有原教學全文讀回通過，才重新讀最新 branch/blob SHA，以條件提交將該案 next_day 從預期 N 更新到 N+1，原子同步 manifest、必要索引及有證據的 makeup_queue 變更，再全文讀回確認狀態。若已 N+1 且證據一致，不再加 Day。完成一案發布、讀回和狀態 gate 才換下一案；未完成不得推進。狀態同步可另 commit，必須引用已驗證文章 commit。文章修訂、搬遷、補教、規則或 CI 成功均不增加 Day。文章成功而狀態失敗只補狀態，不重發。單一 pending 去重、有界重試；同項連續三次失敗轉 needs_manual_review，保存原始錯誤與續接點，其他完整來源專案繼續。

先完成真實可操作基本流程，依最新差異選最小切片，不預排題目。Chat 優先註冊/登入、共用登入登出/Cookie/Session、持久化、房間清單與加入建立及 API/WS ACL。共用實作由各系統透過儲存介面和政策接入，SQLite/PostgreSQL及登入狀態各自獨立。Pet 優先建立第一隻命名寵物→Tauri IPC→SQLite→生活窗→重啟載入，單一元件和每幀單一寵物圖像分別驗證，三個不同 pet id 原生 GUI/事件不能以字串測試取代。Analytics 優先持久管理員帳密登入、首次及再次改密碼、Session、角色/來源授權，核對完整 postgres-integration；本次歷史學習庫依使用者政策採刪除重建，不要求保留資料原地升級，未另授權不實際刪資料庫，已套用 migration 證據保留。

Day25（含）以前正文、程式、歷史證據不改寫；搬遷是原文新增保存，不回寫舊文。makeup_queue 按各專案 FIFO，每篇最多一項，完整讀回有證據才結案；保留未完成補教。主體學習各案15分鐘＋共用15分鐘合計60分鐘，Refactor/版本/證據另列，此為學習估時而非模型執行時限。CI 任一狀態只揭露精確 SHA 和覆蓋範圍，不單獨阻擋教學、不代替測試/產品驗收；未執行、未套用、未部署如實標示。回報各案文章連結、commit、讀回、next_day、產品套用/整合/驗收及缺口。週六00:00全面比對是另一既有工作，勿新建或在每日覆蓋 last_full_comparison；只有完整比對成功才更新該值。模型/推理強度保留現有實際任務設定，提示詞不作切換證據。
