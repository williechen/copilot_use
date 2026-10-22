# 週六主體程式與文件核對

`copilot_use` 是規則、教學文章、next_day、manifest、交付讀回與歷史證據的正本；`rust_ai_chat` 是唯一程式、測試、CI 與功能驗收主體。 既有週六 00:00 Asia/Taipei 身份與時程保持原狀，不新增或觸發任務。

## 正確核對範圍

1. 取 rust_ai_chat 最新 main 精確 SHA，核對 tracked source、目錄、型別、介面、呼叫鏈、契約、DB／migration、UI、設定、測試與該庫 CI。
2. 取 copilot_use 的文件交付 SHA，核對需求、規則、Day、進度、manifest 與主體程式的實際對應；文件庫不作第二套產品程式比對或功能驗收。
3. 依主體環境完成適用測試／GUI／重啟驗收，揭露未跑與失敗；只有既定完整 gate 真正完成才依共同規則更新 last_full_comparison。文件角色修訂不前移該值。

## 執行設定限制

舊 weekly-sync.yml 仍將 copilot_use 作 product 並呼叫本庫 Verify；run-weekly.sh 與 systemd 範例仍指向舊程式工具。這些設定尚未修改，不作已符合本頁政策的證據，也不以其結果認定主體驗收成功。本次不執行、安裝、停用或調整任何排程；雲端及其他主機執行情況未重新驗證。

[分工同步範圍](REPOSITORY_ROLE_TRANSITION.md) 列出剩餘流程；[舊週六操作全文](teaching/history/evidence/2026-10-23-repository-roles/before/docs/SATURDAY_OPERATIONS.md) 保留原命令與來源。
