# Pet Day40：原生 GUI 人工操作與 SQLite 交叉驗證

2026-10-26；精確來源 `9c767978b20bd56e1c2182dac2f37e6ae9b7d988`。真正編譯及啟動 Tauri 原生程式，使用隔離 clone、專用 SQLite 與暫存 XDG profile。原生控制工具未開放，因此**視窗操作與顯示由使用者執行並直接確認**；代理執行編譯、三次啟動／停止、DB與log核對。這不是代理自動操作GUI，也不是拿Rust單元測試代替原生GUI。

| 實際情境 | 人工觀察 | SQLite／程序佐證 |
| --- | --- | --- |
| 原生控制台啟動 | 看得到且顯示正常 | 原生程序存活、原生WebView載入JS/WASM HTTP200 |
| 建立GUI-A、GUI-B | 兩生活窗成功 | 兩獨立UUID，A0／B0（0出籠、1回籠） |
| 原生關閉A、保留B | 全部成功，B不受影響 | A0→1，B保持0 |
| A重新出籠 | 全部成功 | A1→0，B保持0 |
| 同DB重啟 | 兩個都恢復且正常 | A0／B0 |
| 再關閉A，不重新出籠 | A已關閉，B仍正常 | A1／B0 |
| 再用同DB重啟 | 只有B，A沒有出現 | 最終DB仍A1／B0 |

生活窗設定 `decorations(false)`，沒有標題列X時使用 `Alt+F4` 觸發原生關窗；不將這次寫成可見X按鈕已測。逐隻回籠、重新出籠與重啟排除已回籠的指定情境通過。

[使用者直接確認](human-observations.json) · [DB狀態轉換](db-transitions.jsonl) · [第二次重啟前](db-a-caged-before-restart.json) · [最終DB](db-final.json) · [命令／cwd／exit](commands.json) · [啟動命令與範圍](verification.json) · [執行檔SHA256](binary.json)。三次原生程序均於測試後主動Ctrl-C，exit130是結束方式，沒有當作崩潰。原生程式日誌保留在本目錄native-*.log。

前端初次exit2：NO_COLOR=1不符合Trunk布林參數；僅測試程序unset後重試。第二次exit1：離線工具搜尋找不到wasm-bindgen0.2.129；指定本機既有相同版工具PATH後成功。Cargo／lock／產品程式未修改，原錯誤logs保留。

日誌有DRI3／DRI2／VMware無3D加速警告；使用者確認內容顯示正常，未觀察到啟動崩潰。漫遊設定為false，未做漫遊、DB故障注入／完整Gate2。原生截圖介面不可用，沒有製造截圖或將人工觀察寫成代理目視證據。

測試後已停止三次原生程序與前端server，移除本輪暫存／profile／專用DB；清理量及原始工作樹乾淨狀態見[cleanup.json](cleanup.json)。[本輪Cargo／瀏覽器實測](../rerun-9c76797/README.md)保留當時「原生未執行」事實，由本補充紀錄更新現行狀態。CI重跑403與完整Gate2未通過的限制不變。Day41、FIFO與last_full_comparison不變。
