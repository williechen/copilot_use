# Day39 本地實測證據（2026-10-26）

這裡保存前一輪實際執行的完整命令及 stdout/stderr；本輪文件整理重新核對來源 SHA，沒有再跑 Cargo 或重複計功。

| 來源 | Chat | Pet | Analytics | fmt |
| --- | --- | --- | --- | --- |
| 8fc4b27＋各案 Refactor／Green | Rust 57 通過 | Rust 70＋Node 6 通過 | 52 個不同測試通過 | 三案 exit 1 |
| 最新 bb259aa | Rust 57 通過 | Rust 70＋Node 6 通過 | 52 個不同測試通過 | 三案 exit 0 |

兩組 server check、Chat SSR／hydrate WASM、Pet native／WASM 都通過。Analytics suite 為51 passed／1 ignored；ignored 另跑1通過，CSV單元測試補跑不重複計數。真PostgreSQL與正式migrations已執行，測試庫已清除。

六份舊 patch 只在教學8fc基底實際 check／apply成功；最新版正向check均exit1，未強行重套。舊候選Chat／Pet diff whitespace檢查exit2；沒有修飾失敗為通過。[結果摘要](summary.json)、[來源／patch hashes](provenance.json)、[逐檔比較](latest-file-comparison.json)。

[104筆命令詳細表](commands.md)、[原始JSONL](commands.jsonl)保存精確cwd、exit code、測試摘要、環境覆寫、耗時、log SHA256。原始日誌在logs/，均隨本次文章本地保存。本文沒有將舊來源GUI／CI追認為最新驗收。

首次工作目錄遭刪除的試跑不作成功證據；重建後重新驗證。Pet check 曾為清空間主動中斷，該次exit status未取得，之後重跑通過：[中斷紀錄](interruption-record.json)。本地修改已在更新前備份；驗證後來源保持不變：[保全結果](preservation.json)。清除約35GB舊target，本次快取也已清掉：[清理明細](disk-cleanup.json)、[最終空間](logs/final-disk-space.log)。

未執行：新Red、真瀏覽器、Pet真多視窗／桌面競態、試算表匯入、release／安裝、GitHub Actions與完整Gate2。沒有push或GitHub寫入。
