# Chat Day43 r2｜真實 GUI 清除搜尋回歸與 CI fixture

2026-10-29；產品基準 `8b882696c5548accb9d98a32b27db35e11109c74`。[原功能／Refactor](chat.md) · [前次本地測試](chat-r1.md)。

## 正式呼叫鏈缺陷

RoomList → search_rooms(query) → /api/rooms?q=… 會以伺服器篩選結果替換 rows。空搜尋結果之後只清空 query，無法找回被覆蓋的完整快照。真瀏覽器已重現「清除後仍無房間」。

補修將讀取／重新整理與清除按鈕共用 refresh_rooms，沿既有 list_rooms() → 真 API 重新取得完整清單；沿用 loading 排他、refresh_result_view 與 clear_room_search。成功才替換快照並清除 query；失敗保留原 query／rows 並顯示可重試錯誤。

SSR 新測試依正式路由上下文包 Routes／Route，保留原清除按鈕及 aria-label 斷言。UI 21 項、Leptos build 與真瀏覽器正常／失敗／重試通過。遠端 attempt #2 仍是原樣 SHA，不能追認為修補成功。

[兩檔完整 patch](../../../source-review/acceptance-2026-10-29/chat-gui-ci-fix.patch) · [實際命令、GUI／CI／DB 證據與限制](../../../source-review/acceptance-2026-10-29/README.md)。在 8b88269 的隔離副本先 git apply --check 再 apply；此補修含前次 fixture 修正，不與該版 fixture patch 重複套用。原功能／獨立 Refactor 與所有有效斷言保留，不混入其他專案；使用者指定產品唯讀，本輪不提交或推送產品。

next_day 維持 44，GUI 部分驗證不等於完整 Gate2；全房間 ACL、CI 新 SHA、原生／舊庫各有獨立界線。
