# Day45 Chat 採用與瀏覽器追驗｜448dc72f

產品精確SHA`448dc72f9f7c7bfe7e0f92f235faba65825c1c8b`，文件基準`348799729eef0a90fd8b2dcd521b4b969354ac80`。產品庫唯讀。前次member-profile-update／member-password-update與真註冊回歸已採用，不要重套舊runtime授權ID patch。

## 原樣真實瀏覽器：9段通過

正規cargo-leptos建置後，以Chromium對原樣產品及隔離SQLite重新註冊新會員；完成登入、建房、真WS發送並落DB、即時排除斷線及SSR403、自身資料表單、自身密碼表單、撤銷舊Session／新密碼登入、匿名房間登入回跳。9段完成、無pageerror、exit0：[結果](browser-results.json)、[log](browser.log)、[截圖](chat-room.png)。這次是448原樣實跑，不是前次候選結果改標。

## 回歸Red／Green：只剩測試一行

完整SSR回歸原樣44通過／1失敗，exit101：[log](chat-regression.log)。前次runtime缺陷已修，這次失敗在強制改密碼測試將CSRF purpose誤改為`member-password-update`。**perm_id是授權鍵；CSRF purpose是另一個協定標籤，兩者不是同一欄位。**產品表單及後端使用`member-password`，因此該測試偽造了錯的CSRF而收到403。

[完整單檔一行patch](csrf-test-label-fix.patch)只把該測試字串改回`member-password`，不改runtime、migration或角色授權。候選完整回歸45通過、exit0：[log](chat-test-fixed.log)。[乾淨套用／逐檔相同](patch-validation.json)保留，產品尚未套此測試補正。

Chat／Analytics／Pet的既有獨立Refactor不重複計功。本次只有Chat三檔變更；Analytics與Pet子樹完全相同，[來源blob／SHA256](source.json)核對，沿用原SHA證據，不重標成本次實跑。測試服務、副本與DB已清除，[cleanup](cleanup.json)。

Gate1為本次文件／證據更新；完整Gate2仍false：尚有這一項測試、Analytics legacy ignored、真實DB備份、實體／打包跨OS桌面與透明合成／自動漫遊及未完成管理功能等缺口。next_day三案46，未新增Day，不改FIFO或last_full_comparison。歷史失敗與候選證據保留。
