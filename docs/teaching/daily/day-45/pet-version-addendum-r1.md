# Pet Day45 版本補查 r1（原文與原測試保留）

本日官方 [Node 發行索引](https://nodejs.org/dist/index.json) 最新 stable／LTS為 **26.13.0 Lithium**，27 alpha排除。前次測試用22.22.1；本次在隔離工具目錄下載官方26.13.0並以同一Playwright1.64.1與同一候選DOM腳本重跑，六pose全部通過（exit0），見pet-node26.log。新安裝建議26.13.0；不改使用者全域Node，也不宣稱Tauri WebKit已驗。

官方npm registry：Playwright latest1.64.1、pnpm latest12.16.1；實際安裝工具pnpm11.25.0。本篇建議新安裝採12.16.1搭配Node26；本次未用pnpm12重新安裝，原安裝／失敗日誌保留，不把版本查核當升級成功。相容性以[Playwright官方安裝條件](https://playwright.dev/docs/intro)及套件engines為依據；實際新Node已完成DOM幾何回歸。完整官方來源及實際版本見tool-version-addendum.json。

這是Day45版本證據補充，不增加Day，不改原Pet文章／patch／blob／SHA256，不變更FIFO或last_full_comparison。完整產品Gate2仍false。
