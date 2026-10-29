# 歷史文件分包｜2026-10-30

固定來源：`williechen/copilot_use@59e30c9dabfcfa1eff036de3a81143521bfac41a`。共 2755 份既有文件，分成六包；每個來源路徑只出現在一包。為保留所有歷史修訂與連結脈絡，收錄完整 docs 文件快照與根 README，現行規則只作對照，不宣稱已封存或驗收。

| 壓縮檔 | 內容 | 檔案數 | ZIP 大小 |
| --- | --- | ---: | ---: |
| [01-chat-2026-10-30.zip](01-chat-2026-10-30.zip) | Chat 教學版本 | 247 | 0.79 MiB |
| [02-pet-2026-10-30.zip](02-pet-2026-10-30.zip) | Pet 教學版本 | 264 | 0.84 MiB |
| [03-analytics-2026-10-30.zip](03-analytics-2026-10-30.zip) | Analytics 教學版本 | 234 | 0.83 MiB |
| [04-shared-teaching-2026-10-30.zip](04-shared-teaching-2026-10-30.zip) | 共用教學歷史與跨案證據 | 1512 | 4.39 MiB |
| [05-source-verification-2026-10-30.zip](05-source-verification-2026-10-30.zip) | 來源檢查與驗證紀錄 | 438 | 3.01 MiB |
| [06-rules-and-indexes-2026-10-30.zip](06-rules-and-indexes-2026-10-30.zip) | 規則、需求與索引對照 | 60 | 0.38 MiB |

每包含 PACKAGE_README.md 與 MANIFEST.json；原始檔案位於 `copilot_use/`。只閱讀單一專案可只下載該包；需使用跨包相對連結時，將所需各包解壓至同一目錄。原有證據 ZIP 保持原樣，不展開或改寫。

來源檔案的 Git blob 與 SHA256、分類與數量見 [總清單](manifest.json)；ZIP 本身校驗見 [SHA256SUMS](SHA256SUMS)。已執行 ZIP CRC 與每份解壓內容 SHA256 核對，未改歷史原文。

依使用者明確指示，2,755 份已打包原檔已刪除，包含現行文章、規則及狀態；只留下新的下載入口與校驗資料。原 Day、FIFO、next_day、last_full_comparison 值保存在壓縮檔內，未改寫其內容。產品庫未改動。來源為上述固定 commit；後續文件更新不自動包含在本批快照。


## 還原原文

下載所需 ZIP，將其中 `copilot_use/` 的內容解壓到同一個資料夾，保留其相對目錄；六包全部解壓可恢復來源 commit 的完整 docs 與原 README。每包頂層的 PACKAGE_README.md／MANIFEST.json 是包裝資訊，跨包解壓時只需合併 copilot_use/，避免覆寫頂層資訊。

若要恢復到儲存庫工作目錄，將 `copilot_use/` 內的 `docs/` 及 `README.md` 複製回儲存庫；README 會取代目前下載入口，操作前先另存目前入口。原始版本也可在 Git commit `59e30c9dabfcfa1eff036de3a81143521bfac41a` 讀取。

[刪除與既有 ZIP 未變更的校驗紀錄](deletion-audit.json)。本次刪除未重跑產品測試或推進教學；既有工具若依賴已刪除文件路徑，須先還原對應分包再使用。
