# Pet Day42 r1｜本地驗證修訂

2026-10-28；最新產品 `e8a640978732651cbbca1483b07e771f876a439b`。原文基準 `0b2288e4996b6d80b00e7a77e2cd023835306528`；[原始已發布全文](pet.md) 與兩份原 patch 保留不動。本修訂不增加 Day，原文功能、完整 Green、Refactor、有效斷言與回復順序持續適用於其精確基準。

最新產品已存在本案 Day42 正式 caller 與 Refactor；不應對最新 main 再正向套同一份 patch。重現原教學應在隔離 clone checkout 原文基準，再依 Refactor → Green 執行 check／apply。最新產品回歸與候選 patch 測試分列於[本輪完整命令與結果](../../../source-review/local-validation-2026-10-28-e8a6409/README.md)。原文「未執行」只保留為發布當時狀態，本次結果以上述精確 SHA 證據為準，不能沿用歷史成功。

尚未完成 GUI／CI 與完整版本 Gate，Gate2 不通過，next_day 維持 42。原文後續風險與 FIFO 不結案。當次本機工具版本與四個 workspace manifest／lock resolved 見同目錄證據；未自動升級依賴或套 migration 到既有 DB。

Node 原樣 6 通過／4 失敗；VM fixture 補 `location: window.location`，invoke recorder 使用 `structuredClone(args)` 保留參數值與全部斷言。修正後 10 項通過，屬 fixture 候選，非原生驗收。完整修正與命令見本輪證據。
