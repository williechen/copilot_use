# 排程文章狀態與執行依據

**2026-10-23：原排程文章已過時，不再作現行排程提示詞或執行規格。** 原文中的固定教學篇章、P9、雙庫程式工作、舊排程時間、Task ID、ACTIVE 與讀回敘述均只屬歷史紀錄，不能據此確認目前任務設定。

## 執行時應讀取的正本

- [共用規則](../COMMON_LEARNING_RULES.md)：現行教學、交付、補教及驗收規則。
- [即時進度](../learning-state.json)與[文章索引](current/manifest.json)：各案實際進度、已交付版本及證據；不得固定指定先核對某一舊篇章。
- [目前功能與驗證狀態](current/STATUS.md)、[使用者流程](../requirements/USER_FLOWS.md)、[人員驗證](../requirements/AUTHENTICATION.md)、[UI 條件](../UI_ALIGNMENT_REQUIREMENTS.md)：核對最新需求、功能與缺口。

每次取得 rust_ai_chat/main 的程式來源 SHA 及 copilot_use/main 的文件來源 SHA，再按最新確認需求與真實呼叫鏈選擇尚未完成的工作。rust_ai_chat 保持唯讀；copilot_use 只維護文件、教學、進度、索引與證據，不執行其舊程式工作。P9 與已廢除工作不得從歷史提示詞恢復。

只有全面比對真正完整成功才更新 last_full_comparison；文件整理、比對或排程修訂不改 next_day，也不自動結案補教。

## 實際排程尚未核實

本頁是文件依據與狀態說明，不是已部署的任務提示詞。每日、每週實際時間、任務 ID、啟用狀態、模型與提示詞內容均須從實際排程服務讀回確認；本次未修改或觸發任何任務。本機未找到可對應的 automation.toml，不能因此判定雲端任務不存在或已停用。每週至少一次全面比對的需求保留。

[過時文章原文](history/evidence/2026-10-23-outdated-schedule/scheduler-instructions-v1.before.md)僅供追溯；其中相對連結請依[原始 commit](https://github.com/williechen/copilot_use/blob/d575053e1d1770bdfacbb000c7bd386d659be9ae/docs/teaching/scheduler-instructions-v1.md)解析。
