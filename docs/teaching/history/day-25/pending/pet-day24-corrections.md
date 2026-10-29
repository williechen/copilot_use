# Pet Day24封存後待補：顯示、動畫、事件與升級

狀態：pending；來源08b891a22c1934a2bf0b1edbb103c9b16bcf12de；next_day=25。本檔不是已完成的Day25教學，不推Day。

實測：三個生活窗各兩個.pet-face、按鈕裁切、pet-*事件listen被拒絕；後端FSM分離不等於畫面同步。動畫字串另含🐢💤／🐢z，使用者要求動畫只保留單一寵物圖像。歷史Pet owner migration變更使4f0cd00舊庫升級VersionMismatch(20261018095626)。

下一Day先重新核對最新來源，再提出新Red／完整最小Green／Refactor；Day24更正只能在其後追加，每篇最多一個舊Day單元。不要回寫已封存Day24，不刪DB或改checksum掩蓋失敗。證據：docs/teaching/evidence/2026-10-21-day24-main-08b891a/REPORT.md
