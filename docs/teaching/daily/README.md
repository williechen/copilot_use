# 三專案每日 TDD 教學

唯一交付分支：`williechen/copilot_use/teaching/rust-ai-chat-alignment`。所有新 Day 存於本目錄，按 `day-NN/chat.md`、`day-NN/pet.md`、`day-NN/analytics.md` 分層；NN 使用至少兩位數，不建立空文章或預排題目。

每新增一個專案 Day 即獨立 commit 並推送，不能累積多案或多天後一起提交。從精確 commit SHA 全文讀回，核對四節與全部程式碼並保存 blob SHA、SHA256 與時間，再以最新 branch/blob SHA 條件同步 next_day、manifest 和必要索引，最後讀回狀態；全部 gate 成功才算交付。狀態 commit 可獨立提交並引用文章 commit。

目前各案 next_day＝27，實際執行必須重新讀取 [learning-state](../../learning-state.json)，不能依本頁快照推進。

| 專案 | 最近已發布原文（保留原路徑） | 需求與缺口 |
| --- | --- | --- |
| Chat | [Day26](../day26/chat.md) | [入口](../current/CHAT_ENTRY.md) |
| Pet | [Day26](../day26/pet.md) | [入口](../current/PET_ENTRY.md) |
| Analytics | [Day26](../day26/analytics.md) | [入口](../current/ANALYTICS_ENTRY.md) |

[共同規則](../../COMMON_LEARNING_RULES.md) · [manifest](../current/manifest.json) · [狀態](../current/STATUS.md) · [全部 Notion 搬遷原文](../notion-export/2026-10-22/README.md)

Day25（含）以前 GitHub 封存檔不改寫；歷史 Notion 原文只新增保存於匯出目錄，搬遷不新增 Day、不改補教佇列或 last_full_comparison。Notion 三專案教學頁在完整搬遷及讀回後移至可還原垃圾桶；往後教學無 Notion 依賴。
