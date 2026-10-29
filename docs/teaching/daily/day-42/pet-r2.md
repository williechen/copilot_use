# Pet Day42 r2｜主分支修正後驗證

2026-10-28；產品來源 `c6708c42f21ac919e57a379848b4977d27a1c3d6`。Node VM 補入 `location: window.location`；invoke recorder 用 `structuredClone(args)` 保留跨 VM 的參數值。逐 UUID、native reject 與錯誤傳遞斷言全部保留。Pet UI 另移除兩處尾端空白。

Pet Node 10 項全部通過；不是 SQLite／原生 GUI 驗收。本輪直接測隔離 clone 的主分支原樣，未再套 patch。[本輪原始命令與結果](../../../source-review/local-validation-2026-10-28-c6708c4/README.md)。

## 閱讀與套用

[原始功能／完整 Green／獨立 Refactor](pet.md) · [r1 patch 勘誤與修正過程](pet-r1.md)。已發布原文與附件保留；原 patch 仍只適用於其 0b2288e 基準，重現時依 r1 的補修順序。最新 main 已包含修正，不重複套用。

## 尚未驗收

GUI／瀏覽器完整情境、CI、舊 DB 升級及完整版本 Gate 未完成。本篇不結案原功能條件／FIFO，不增加 Day；next_day 仍 42，last_full_comparison 不變。
