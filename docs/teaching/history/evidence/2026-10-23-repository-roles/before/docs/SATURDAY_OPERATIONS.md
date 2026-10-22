# 週六來源比對與原生建置

日常操作請讀 [本地啟動](LOCAL_START.md)。本頁僅說明週六工程，執行前使用各自實際 checkout 路徑並記錄兩庫 SHA 與 dirty 狀態。

## 來源比對

在 copilot_use 根目錄，以唯讀參考庫路徑執行：

```sh
python3 scripts/weekly-compare.py --reference /path/to/rust_ai_chat --output target/weekly-comparison.json
```

報告保存完整 tracked tree、Git object ID、mode 及差異。相同物件可輔助辨認搬移；內容差異不能直接判為缺口。再逐項核對型別、呼叫端、流程、DB、UI、契約與測試，區分合理差異、必要適配、產品缺口及未核實項目。

現有 `.github/workflows/weekly-sync.yml` 設定週六 00:00 Asia/Taipei（前一日 16:00 UTC），並可手動觸發；GitHub 可能延遲。它讀取兩庫 main、保存比較 artifact、呼叫 Verify 並略過 Compose，不會自行合併或推進教學狀態。此處描述儲存庫設定，未查驗本週實際執行結果。

既有本機 user unit `deploy/systemd/copilot-weekly.service`／`.timer` 與 `scripts/run-weekly.sh` 保留；是否已安裝／啟用應在該主機查核。執行結果在 `target/weekly/<UTC timestamp>/`。本次沒有新增、安裝或變更任何排程。

## 原生產物與 Gate

```sh
bash scripts/build-production.sh
```

需乾淨的已提交來源及完整工具，細節見本地啟動頁。產物在 `target/native-release/<SHA>/`，包含 Chat／Analytics、前端資產、systemd 參考檔與 SHA256 manifest。Pet 另行原生建置。產物存在不代表已安裝、服務已啟動或 GUI 通過。

工程修正依 Red → 最小 Green → Refactor，提供精確 SHA 的適用測試、DB／HTTP／WS／GUI／整合證據後，才更新對應驗收。完整兩庫比對及 Gate 都完成後才能更新 `last_full_comparison`；單次文件核對不能前移該值。功能驗收不讀 AI key、不新增付費呼叫。

舊機器工具狀態、固定 v0.2.0 安裝與主機部署步驟完整保留於 [原始文件](teaching/history/evidence/2026-10-22-current-docs/before/docs/SATURDAY_OPERATIONS.md)，不再作目前主機已具備條件的聲明。
