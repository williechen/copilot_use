# Analytics Day24封存後待補：PG測試契約、再次改密碼與升級

狀態：pending；來源08b891a22c1934a2bf0b1edbb103c9b16bcf12de；next_day=25。本檔不是正式Day25文章，不推Day。

實測：首次改密碼成功並重啟可用新密碼，但再次GET/change-password=303/dashboard；完整Day24 PG測試引用不存在的20261021071000_unique_account_email.sql而無法編譯。main改動歷史auth migration，4f0cd00舊庫升級VersionMismatch(20261018073932)。只有可執行子集28項通過，不能聲稱完整30項通過。

後續需依最新來源重做真Red／Green與升級驗證，並考慮使用者要求的共用登入／登出／Session套件；DB Adapter與系統政策仍各自注入。Day24不回寫，新增Day25原教學後才能補舊Day更正，不藉補教增加Day。證據：docs/teaching/evidence/2026-10-21-day24-main-08b891a/REPORT.md
