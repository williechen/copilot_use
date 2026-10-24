# 三專案每日 TDD 教學排程｜現行指令

2026-10-24 已更新原有網頁版排程「三專案每日 TDD 教學」。任務 ID：`6acd967c15fc8191ade8407954c5b354`。本頁保存實際已儲存、重新載入後全文讀回一致的提示詞，取代先前只標示過時的入口說明。

- [原排程](https://chatgpt.com/scheduled?automationId=6acd967c15fc8191ade8407954c5b354&automationSource=cloud)仍為啟用中；頻率與時間保留原設定，網頁顯示為每天／上午，未改模型、未建立重複任務、未按立即執行。
- 程式來源改為 williechen/rust_ai_ring/main；文件正本為 williechen/copilot_use/main。
- 後續教學依最新目錄提供程式、imports、Cargo、呼叫端與測試調整。
- 依使用者要求排除所有 README.md 修改，改同步非 README 現行入口、manifest 與進度。
- 本次只同步規則與排程，不新增教學、不推進 next_day、不修改 last_full_comparison。
- 提示詞 SHA256：`635aefd9cdac0ab2af53f22776b68fecd575f92a6429540bd9d593aa87be37d3`；[讀回證據](current/scheduler-sync-2026-10-24.json)。

## 雲端已儲存的提示詞

```text
每日執行 Chat、Pet、Analytics 三專案完整 TDD 教學交付，依 Chat→Pet→Analytics 逐案完成，不得只交付計畫或核對報告。
每次先重讀 williechen/copilot_use/main 的 docs/COMMON_LEARNING_RULES.md、docs/SOURCE_LAYOUT.md、docs/SYSTEM_ARCHITECTURE.md、docs/TEACHING_ALIGNMENT.md、docs/FUTURE_WORK.md、現行需求與 UI 條件、docs/learning-state.json、docs/teaching/current/manifest.json、三案現行入口與相關上一篇文章；取得 copilot_use/main 與 williechen/rust_ai_ring/main 最新精確 SHA。程式來源為 rust_ai_ring，舊名稱 rust_ai_chat 只屬歷史來源，不能繼續當現行 repo。
後續教學必須按照 rust_ai_ring 最新目錄調整程式：核對 chat_apps、pet_apps、analytics_apps、crates 四個 workspace 的 Cargo.toml/Cargo.lock、實際呼叫端與測試。完整 Green 要同步提供必要檔案新增/修改/搬移、Cargo dependency/feature、Rust imports、Leptos metadata、呼叫端與測試，以及正確工作目錄的驗證命令。目錄使用 chat_server/chat_ui/chat_persistence、pet_server/pet_ui、analytics_server；Cargo package 使用 chat-server/chat-ui/chat-persistence、pet-server/pet-ui、analytics-server，Rust crate 使用底線，仍以當次原始碼為準。pet-application、pet-persistence、analytics-persistence 若仍為空 library，不能宣稱已接通；必要時以最小切片教學搬移 server 內的使用案例或 SQLx adapter，完整接回真實呼叫者與測試，不預先建立無呼叫者骨架，也不要求每日每層都修改。
只向 williechen/copilot_use/main 寫入文件；williechen/rust_ai_ring 保持唯讀，產品程式由使用者套用。copilot_use 不建立第二套產品。P9、固定測試搬移計畫與 copilot_use 程式建置/驗收等已廢除工作不恢復；上述後續教學的新目錄程式調整依使用者最新要求執行。所有 README.md 均不新增或修改，README 不作本次交付 Gate。
各案從即時 next_day 取 N，先核對上一篇與缺口，再實際撰寫發布完整 TDD 教學，包含功能說明、Red、完整最小 Green、Refactor、交付當日官方最新穩定版本核對、必要舊 Day 更正與驗證步驟。每篇獨立 commit 至 docs/teaching/daily/day-NN/{project}.md；從精確 commit 全文讀回並保存 blob SHA、SHA256，再以預期分支 SHA 條件同步該案進度與 manifest。來源變動時重新核對未發布草稿；禁止強制覆寫、重複發文、倒退或重複增加 Day。
每篇交付後同步其非 README 現行入口 docs/teaching/current/CHAT_ENTRY.md、PET_ENTRY.md 或 ANALYTICS_ENTRY.md，提供最新文章的可點擊相對路徑、文章名稱與交付/未驗收狀態；manifest 與 learning-state 必須一致。STATUS.md 依現行規則只列功能與驗證現況，不把教學發布視為程式驗收。入口與索引修改先讀最新內容及 blob SHA，條件式提交後從精確 commit 全文讀回，驗證連結指向存在的文章。若失敗，標記「文章已交付／入口未同步」，記錄未同步路徑與原始錯誤，繼續其他專案；不可只在對話給連結就宣稱完成。
每次寫入明列 repository、分支、路徑、預期 parent SHA、實際 commit 與原始錯誤。保留已發布正文、來源 SHA、原 commit/blob/SHA256 與歷史證據；改名及目錄銜接放在後續教學。每日不更新 last_full_comparison，只有真正完整全面比對成功才更新。未測試、未套用、未驗收如實標示。最後簡短提供三篇文章連結、提交與讀回結果、非 README 入口同步結果、最新進度與缺口；受阻案標記未交付並完成其餘案。
```

## 執行時讀取

[共用規則](../COMMON_LEARNING_RULES.md) · [來源與目錄](../SOURCE_LAYOUT.md) · [即時進度](../learning-state.json) · [文章 manifest](current/manifest.json) · [目前狀態](current/STATUS.md)

先前排程文章的過時版本與原始證據保留於 Git 歷史及[歷史快照](history/evidence/2026-10-23-outdated-schedule/scheduler-instructions-v1.before.md)。這次只驗證上述每日任務，其他排程與舊程式 workflow 不據此宣稱已修改。
