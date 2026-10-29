# Pet 原生 GUI：隔離 A/B 回籠生命週期驗收（2026-10-29）

來源 `rust_ai_ring@8b882696c5548accb9d98a32b27db35e11109c74`；產品唯讀，未提交或推送。人工作業畫面回報與 SQLite 唯讀快照、程序 FD 交叉核對，未宣稱工具自動操控原生 GUI。此為該來源的局部驗收，未追認其他 SHA 或整份 Day43 Green／Refactor。

## 初次資料準備錯誤與重新驗收

原預置 A/B 屬於 `legacy-owner`，正式 `PetStore::list` 僅查 `local-owner`，因此預置兩隻不會顯示。使用者另建同名寵物，曾造成比對對象不一致。先前結果不算通過；舊 DB、回報與失配紀錄保留，沒有改寫歷史證據。這是驗收 fixture 錯誤，沒有據此認定產品清單有缺陷。

另以 SQLite backup 建立 `pet-gui-restart-v2.sqlite`，只在新副本調整預置 A/B 為本機主人、正式建立流程所用 tortoise，初始回籠；其他資料逐列核對不變。此設定是測試準備，不算 GUI 操作。[準備前後資料](fixture-setup.json)。名稱為「重新驗收A-a1／重新驗收B-b2」，固定 UUID 結尾為 `0000000000a1／0000000000b2`。

## 本輪結果

| 情境 | 人工畫面回報 | 資料庫／程序證據 |
| --- | --- | --- |
| 清單 | 兩隻重新驗收均可見 | 本機主人歸屬；初次程序尚未捕捉，下一步取得連線證據 |
| 初始查詢 | A/B 均已回籠 | PID 40828 連線新 DB；旗標 1/1 |
| A 單獨出籠 | A 有生活窗且已出籠，B 仍回籠 | 同程序、同 DB，旗標 0/1 |
| A 回籠 | A 生活窗關閉，A/B 均回籠 | 同程序、同 DB，旗標 1/1 |
| 程序重啟 | A/B 均回籠，沒有 A 生活窗 | 新 PID 41347 連線同 DB，旗標 1/1 |

本表範圍通過。[逐步回報與快照](checkpoints.jsonl) · [唯讀變更觀察](database-events.jsonl) · [最終結果](result.json) · [來源與二進位 SHA256](provenance.json)。資料庫檔案本身不發布。

啟動沿用預建前端／原生 binary；本次程序使用軟體繪圖與停用 WebKit DMA-BUF 的環境設定，無系統設定變更。[啟動腳本](launch-isolated-pet.sh) 為本機絕對路徑版本，需既有預建產物；[觀察器](watch_pet_gui.py) 僅唯讀兩個 UUID。

## 保留限制

未驗收 SQLite 故障逐列錯誤、重新載入清單清除查詢快照、B 操作不覆蓋 A 訊息、三隻寵物、預設出籠資格、直接關生活窗、漫遊、多螢幕或 native 故障補償。本輪不是實際舊 DB 升級，也不使 CI 變綠。遠端 CI 失敗與真實舊 PG 無備份維持原狀；next_day 三案 44、FIFO、last_full_comparison 不變。
