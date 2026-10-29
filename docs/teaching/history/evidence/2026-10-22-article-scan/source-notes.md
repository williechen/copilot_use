# 文章掃描移出的歷史來源註記

以下原文出自 commit `5a8516b40176b0d88e71a74a05251d3e4ddecf2f`；保留特定來源的舊診斷與狀態，不作現行產品結論。

## docs/LOCAL_VALIDATION.md

> 本輪參考庫完整複驗：本輪來源main `2c7ace259ef790cea303d841f5779cd5f022d554`；修正分支codex/revalidate-f9484f2，commit `1abcb8aa6bc279e888a057b87069bea788281c7c`，未合併main。完整workspace明確啟用SSR、PostgreSQL integration及include-ignored：main126通過／1失敗／0忽略，候選131通過／0失敗／0忽略。main的WASM／SSR、Pet前端及原生建置成功；fmt通過，嚴格Clippy仍失敗於logout巢狀if，候選嚴格Clippy通過。規則v1正文、既有需求及next_day各19不變；文章修訂不推進Day。 [實際命令、通過／失敗／未驗證](teaching/history/evidence/2026-10-19-full-2c7ace2/REPORT.md)。下方成品固定報告保留其原SHA，不代表本輪產品重新驗收。

## docs/SYSTEM_ARCHITECTURE.md

本輪參考庫新增的chat_action_log尚無runtime writer，NULL主鍵及任意操作者可插入；候選僅修主鍵，完整身分／稽核仍待接線。P0資料模型按功能調整，並以實際操作、授權、保存及升級驗證。

## docs/UI_ALIGNMENT_REQUIREMENTS.md

歷史來源畫面診斷（不是目前狀態）：3e58e650 的 tauri.conf.json 是固定 320×360、resizable=false；style.css 對 html／body 設 overflow:hidden；所有互動與掃描結果集中 frontend/src/main.rs::App。比較至 8bbf884e，這些檔案未變。此為靜態裁切風險診斷，未冒稱本輪重現 GUI。產品端 copilot_use 已有 main／pet 視窗及控制分頁，其既有能力作適配參考，不將本文件解讀為產品必須重建相同功能。

## docs/UI_ALIGNMENT_REQUIREMENTS.md

來源 roam_desktop 仍移動 main 控制台，須改為 pet 生活窗；Control 不受自動漫遊移動，但可手動拖曳。開啟／聚焦控制台的 IPC 仍使用 main。

## docs/GITHUB_PET_UPDATES.md

目前 GitHub 正式 Release 為歷史 v0.1.0，其資產保持不變；新版原生更新格式用於後續正式 Release。不以本機建置或測試包冒稱 GitHub 已發行。

## docs/SATURDAY_OPERATIONS.md

使用者於 2026-10-16 重新要求正式部署建置；本次新增建置準備，尚無外部上線證據。
