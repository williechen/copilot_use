# 舊文章與現存狀態核對（2026-10-25）

> 最新整併請讀 [問題文章與後續更正索引](ARTICLE_CORRECTIONS.md)：涵蓋至 Day35 的 102 篇閱讀版本與兩個舊修訂，含最新測試／瀏覽器／原生證據。下列 99 篇表格是原 3aeab095 靜態盤點快照；其中 next_day=35、舊 dist 失敗及「GUI 未新驗」只屬當次，不作最新結論。

文件基準 `5d66ad95b96a7fffd85856eb80676ddd7120baa6`；產品精確來源 `williechen/rust_ai_ring@3aeab095f27aa3fc7f1b76995e0d059a94c5dd4d`。逐篇讀取 Day2–34 的 **99 篇目前閱讀版本**，另列 Chat 的舊修訂，共100篇版本；同時對教學目錄 492 份 Markdown 全文做雜湊與靜態掃描。Day1 三案正文未找到，保留來源缺口，不補造文章。

本次調整的是現行索引與機器可讀狀態，不改原教學正文、原交付 commit/blob/SHA256，不把程式存在或舊來源成功提升為當次驗收。沒有重新編譯／執行 Rust、DB、HTTP 或 GUI，也不是全面功能比對；next_day 仍為三案35、last_full_comparison 不變。

## 狀態如何閱讀

- 原文的「未套用／未執行」保留發布當時語境；下表記錄此次精確來源所見，不要求把已接通功能重教一遍。
- 「已見」只代表對應程式與選定 caller 的靜態證據；完整 Gate 2 未在此輪重跑。符號匹配僅是輔助，不能單獨認定整篇已套用。
- 歷史自動化／GUI／切片成功仍限原來源 SHA；9筆既有切片封存證據保留，不新移動或新增驗收封存。詳細歷史結果逐篇放在 JSON。
- 舊 repo/package/路徑、根 workspace-tests、舊 SSR API、stub 501、legacy admin 與廢除政策均按歷史讀取，不能直接套回現行來源。對應新路徑見 [SOURCE_LAYOUT](../../SOURCE_LAYOUT.md)。

## 本次主要狀態更正

1. Analytics 登入、改密碼頁、事件篩選與初始化已見正式 caller，不能繼續以舊文章的「尚未套用／缺主啟動」描述現在。
2. Pet 名稱規則已接 application；控制窗 hide/show 已存在，不能繼續標為未攔截 close。來源 bridge 的舊 VM 錯誤已由後續測試修正；tracked dist 仍有二個契約失敗，原生 GUI 尚未新驗。
3. Chat 房間狀態、個人頁建房、SignInPage 已存在；真正歷史查詢／作者／ACL 仍是缺口。r1 已被 r2 取代，最新來源已見 r2 去重與測試；當次 Rust／HTTP／瀏覽器尚未執行。
4. Analytics 舊 admin 限流／續期測試不代表目前帳密路由完成；舊 Bearer query helper 亦須與正式人員 Session 分開。
5. 舊 Leptos SSR API、過時標籤斷言及目錄銜接需後續修訂；不藉刪測試掩蓋尚有用途的驗證。bootstrap::initialize 仍是多項整合測試 fixture。

## 逐篇目前閱讀版本

| Day | 專案／原文 | 現行狀態 | 現存程式與缺口 |
| --- | --- | --- | --- |
| 2 | [chat historical](../../../docs/teaching/history/day-02/chat/notion/3f4e51d450938170954ff09378afc11a.md) | 程式已見／待當次驗收 | 對應規則／API 或持久化流程在現行程式可見；僅列靜態對應，當次 Rust／DB／HTTP 未重跑。 |
| 2 | [pet historical](../../../docs/teaching/history/day-02/pet/notion/3f4e51d4509381e7ac5cea661af8c67d.md) | 來源 JS 已驗／產物仍有差異 | 來源 bridge 已存在，先前實測來源六個 Node 契約全過；舊 VM 缺 URLSearchParams 的失敗不能繼續當作目前來源結論。tracked dist 的對應契約仍二敗，原生 IPC／GUI 未新驗。 |
| 2 | [analytics historical](../../../docs/teaching/history/day-02/analytics/notion/3f4e51d4509381bcbddccfba35f4b329.md) | 後續實作已取代 | 合法事件 501 的舊 stub 已被來源授權與 PostgreSQL 持久化 ACK 取代；不能為通過舊斷言把程式退回 501。 |
| 3 | [chat historical](../../../docs/teaching/history/day-03/chat/notion/3f4e51d45093819c975fc886e09fe861.md) | 程式已見／待當次驗收 | 對應規則／API 或持久化流程在現行程式可見；僅列靜態對應，當次 Rust／DB／HTTP 未重跑。 |
| 3 | [pet historical](../../../docs/teaching/history/day-03/pet/notion/3f4e51d4509381c482a3c68721761dee.md) | 來源 JS 已驗／產物仍有差異 | 來源 bridge 已存在，先前實測來源六個 Node 契約全過；舊 VM 缺 URLSearchParams 的失敗不能繼續當作目前來源結論。tracked dist 的對應契約仍二敗，原生 IPC／GUI 未新驗。 |
| 3 | [analytics historical](../../../docs/teaching/history/day-03/analytics/notion/3f4e51d4509381ebb781cf9b2f2f0db6.md) | 程式已見／待當次驗收 | 對應輸入驗證、記憶體去重或 PostgreSQL 攝取契約仍在；記憶體與持久化、HTTP 與 gRPC/SSE 的範圍分開，本次 PG 未重跑。 |
| 4 | [chat historical](../../../docs/teaching/history/day-04/chat/notion/3f5e51d45093811ab62acc4bc8fc83fa.md) | 程式已見／待當次驗收 | 對應規則／API 或持久化流程在現行程式可見；僅列靜態對應，當次 Rust／DB／HTTP 未重跑。 |
| 4 | [pet historical](../../../docs/teaching/history/day-04/pet/notion/3f5e51d4509381ebadf2d9adc6210e80.md) | 模型範圍／非完整產品 | 對應 Profile／Codec／預覽狀態或事件模型仍可追溯；不代表設定儲存 UI、AI 素材、真 OS 清理／Trash／receipt 或投遞已接通。 |
| 4 | [analytics historical](../../../docs/teaching/history/day-04/analytics/notion/3f5e51d45093811a8ac5dafd3cd3bbcc.md) | 後續實作已取代 | 合法事件 501 的舊 stub 已被來源授權與 PostgreSQL 持久化 ACK 取代；不能為通過舊斷言把程式退回 501。 |
| 5 | [chat historical](../../../docs/teaching/history/day-05/chat/notion/3f5e51d45093817e93b6ea7c37c3852e.md) | 程式已見／待當次驗收 | 對應規則／API 或持久化流程在現行程式可見；僅列靜態對應，當次 Rust／DB／HTTP 未重跑。 |
| 5 | [pet historical](../../../docs/teaching/history/day-05/pet/notion/3f5e51d4509381739b61d517b0772094.md) | 模型範圍／非完整產品 | 對應 Profile／Codec／預覽狀態或事件模型仍可追溯；不代表設定儲存 UI、AI 素材、真 OS 清理／Trash／receipt 或投遞已接通。 |
| 5 | [analytics historical](../../../docs/teaching/history/day-05/analytics/notion/3f5e51d45093813cabb7e6a48ee3a0f0.md) | 程式已見／待當次驗收 | 對應輸入驗證、記憶體去重或 PostgreSQL 攝取契約仍在；記憶體與持久化、HTTP 與 gRPC/SSE 的範圍分開，本次 PG 未重跑。 |
| 6 | [chat historical](../../../docs/teaching/history/day-06/chat/notion/3f5e51d45093818ab246f8fcdd0b39f6.md) | 歷史骨架／目錄需銜接 | 舊測試骨架、根 workspace-tests／路徑不適用；現有持久化與 publish 契約仍在，不應照舊骨架重新建立。 |
| 6 | [pet historical](../../../docs/teaching/history/day-06/pet/notion/3f5e51d4509381f48e6ac9a9d7dd758a.md) | 模型範圍／非完整產品 | 對應 Profile／Codec／預覽狀態或事件模型仍可追溯；不代表設定儲存 UI、AI 素材、真 OS 清理／Trash／receipt 或投遞已接通。 |
| 6 | [analytics historical](../../../docs/teaching/history/day-06/analytics/notion/3f5e51d450938160a376fde5e2fad496.md) | 程式已見／待當次驗收 | 對應輸入驗證、記憶體去重或 PostgreSQL 攝取契約仍在；記憶體與持久化、HTTP 與 gRPC/SSE 的範圍分開，本次 PG 未重跑。 |
| 7 | [chat historical](../../../docs/teaching/history/day-07/chat/notion/3f5e51d45093815c82c7e32b73b567fa.md) | 歷史骨架／目錄需銜接 | 舊測試骨架、根 workspace-tests／路徑不適用；現有持久化與 publish 契約仍在，不應照舊骨架重新建立。 |
| 7 | [pet historical](../../../docs/teaching/history/day-07/pet/notion/3f5e51d4509381cd8d7efeb14bfc1b45.md) | 模型範圍／非完整產品 | 對應 Profile／Codec／預覽狀態或事件模型仍可追溯；不代表設定儲存 UI、AI 素材、真 OS 清理／Trash／receipt 或投遞已接通。 |
| 7 | [analytics historical](../../../docs/teaching/history/day-07/analytics/notion/3f5e51d450938138a33ddadedb5a5e98.md) | 程式已見／待當次驗收 | 對應輸入驗證、記憶體去重或 PostgreSQL 攝取契約仍在；記憶體與持久化、HTTP 與 gRPC/SSE 的範圍分開，本次 PG 未重跑。 |
| 8 | [chat historical](../../../docs/teaching/history/day-08/chat/notion/3f5e51d45093814cb540cf6772e9b8d2.md) | 部分接線／功能差異 | 產品事件映射程式仍在；真正 Analytics 投遞／Outbox／可靠送達未因映射函式存在完成。 |
| 8 | [pet historical](../../../docs/teaching/history/day-08/pet/notion/3f5e51d450938123b43be0fe0b543ade.md) | 模型範圍／非完整產品 | 對應 Profile／Codec／預覽狀態或事件模型仍可追溯；不代表設定儲存 UI、AI 素材、真 OS 清理／Trash／receipt 或投遞已接通。 |
| 8 | [analytics historical](../../../docs/teaching/history/day-08/analytics/notion/3f5e51d4509381f98c33c2df7ab97663.md) | 後續實作已取代 | 合法事件 501 的舊 stub 已被來源授權與 PostgreSQL 持久化 ACK 取代；不能為通過舊斷言把程式退回 501。 |
| 9 | [chat historical](../../../docs/teaching/history/day-09/chat/notion/3f8e51d45093814b992eea570411bd7d.md) | 程式已見／待當次驗收 | 對應規則／API 或持久化流程在現行程式可見；僅列靜態對應，當次 Rust／DB／HTTP 未重跑。 |
| 9 | [pet historical](../../../docs/teaching/history/day-09/pet/notion/3f8e51d4509381c3b989f2365fd0c0af.md) | 模型範圍／非完整產品 | 對應 Profile／Codec／預覽狀態或事件模型仍可追溯；不代表設定儲存 UI、AI 素材、真 OS 清理／Trash／receipt 或投遞已接通。 |
| 9 | [analytics historical](../../../docs/teaching/history/day-09/analytics/notion/3f8e51d45093810d830ef3fd40e419f0.md) | 程式已見／待當次驗收 | 對應輸入驗證、記憶體去重或 PostgreSQL 攝取契約仍在；記憶體與持久化、HTTP 與 gRPC/SSE 的範圍分開，本次 PG 未重跑。 |
| 10 | [chat historical](../../../docs/teaching/history/day-10/chat/notion/3f9e51d45093818a849dd8b1fbf2c045.md) | 程式已見／待當次驗收 | 對應規則／API 或持久化流程在現行程式可見；僅列靜態對應，當次 Rust／DB／HTTP 未重跑。 |
| 10 | [pet historical](../../../docs/teaching/history/day-10/pet/notion/3f9e51d4509381f7ab32d0dc8b389a86.md) | 模型範圍／非完整產品 | 對應 Profile／Codec／預覽狀態或事件模型仍可追溯；不代表設定儲存 UI、AI 素材、真 OS 清理／Trash／receipt 或投遞已接通。 |
| 10 | [analytics historical](../../../docs/teaching/history/day-10/analytics/notion/3f9e51d4509381269984c5f49a429b5d.md) | 程式已見／待當次驗收 | 對應輸入驗證、記憶體去重或 PostgreSQL 攝取契約仍在；記憶體與持久化、HTTP 與 gRPC/SSE 的範圍分開，本次 PG 未重跑。 |
| 11 | [chat historical](../../../docs/teaching/history/day-11/chat/notion/3f9e51d4509381508c0dd3ca22cf5df7.md) | 程式已見／待當次驗收 | 對應規則／API 或持久化流程在現行程式可見；僅列靜態對應，當次 Rust／DB／HTTP 未重跑。 |
| 11 | [pet historical](../../../docs/teaching/history/day-11/pet/notion/3f9e51d450938152a147c931e3e945be.md) | 來源 JS 已驗／產物仍有差異 | 來源 bridge 已存在，先前實測來源六個 Node 契約全過；舊 VM 缺 URLSearchParams 的失敗不能繼續當作目前來源結論。tracked dist 的對應契約仍二敗，原生 IPC／GUI 未新驗。 |
| 11 | [analytics historical](../../../docs/teaching/history/day-11/analytics/notion/3f9e51d45093814a8054d44c4ff9fa55.md) | 程式已見／待當次驗收 | 對應輸入驗證、記憶體去重或 PostgreSQL 攝取契約仍在；記憶體與持久化、HTTP 與 gRPC/SSE 的範圍分開，本次 PG 未重跑。 |
| 12 | [chat historical](../../../docs/teaching/history/day-12/chat/notion/3fae51d45093812bbf88c3e7f51d6522.md) | 程式已見／待當次驗收 | 對應規則／API 或持久化流程在現行程式可見；僅列靜態對應，當次 Rust／DB／HTTP 未重跑。 |
| 12 | [pet historical](../../../docs/teaching/history/day-12/pet/notion/3fae51d4509381d0a80ddbbaf251fb03.md) | 部分接線／功能差異 | 幾何／開關／偏差 guard 程式存在，但 roam runtime 固定找 pet 視窗，動態生活窗用 pet-UUID；真正逐寵物漫遊與拖曳／多螢幕仍有接線缺口。 |
| 12 | [analytics historical](../../../docs/teaching/history/day-12/analytics/notion/3fae51d4509381ebb1e9f8482b5bf44e.md) | 程式已見／待當次驗收 | 對應輸入驗證、記憶體去重或 PostgreSQL 攝取契約仍在；記憶體與持久化、HTTP 與 gRPC/SSE 的範圍分開，本次 PG 未重跑。 |
| 13 | [chat historical](../../../docs/teaching/history/day-13/chat/notion/3fae51d4509381e499e5e02cfba58336.md) | 程式已見／待當次驗收 | 對應規則／API 或持久化流程在現行程式可見；僅列靜態對應，當次 Rust／DB／HTTP 未重跑。 |
| 13 | [pet historical](../../../docs/teaching/history/day-13/pet/notion/3fae51d45093814dbdabee6deac48554.md) | 部分接線／功能差異 | 預覽 parser／摘要／確認取消 UI 已見；來源與 dist 有分歧，OS 實際整理未接通，不能由確認或純模型冒充檔案操作成功。 |
| 13 | [analytics historical](../../../docs/teaching/history/day-13/analytics/notion/3fae51d4509381808a9cd813423d52d1.md) | 後續實作已取代 | 舊 Bearer summary/helper 的授權模型已由正式人員 Session 路由取代；cfg(test) helper 綠燈不能當正式路由驗收。 |
| 14 | [chat historical](../../../docs/teaching/history/day-14/chat/notion/3fbe51d450938149af5dd68abc6109bd.md) | 程式已見／待當次驗收 | 對應規則／API 或持久化流程在現行程式可見；僅列靜態對應，當次 Rust／DB／HTTP 未重跑。 |
| 14 | [pet historical](../../../docs/teaching/history/day-14/pet/notion/3fbe51d4509381e484f4eb9e1f98fc68.md) | 部分接線／功能差異 | 預覽 parser／摘要／確認取消 UI 已見；來源與 dist 有分歧，OS 實際整理未接通，不能由確認或純模型冒充檔案操作成功。 |
| 14 | [analytics historical](../../../docs/teaching/history/day-14/analytics/notion/3fbe51d4509381f2a9a2e4b608706eea.md) | 部分接線／功能差異 | 舊 /summary/{event_type} 路由未掛入 password_login 正式 router；Dashboard 的 event_type query 篩選已另接通，兩個契約不可混為已完成。 |
| 15 | [chat historical](../../../docs/teaching/history/day-15/chat/notion/3fbe51d450938130a3c0c93171105f3a.md) | 程式已見／歷史驗證保留 | 清單、搜尋與失敗保留／刷新流程已見；已有舊來源瀏覽器切片證據，當次來源未重跑。舊刷新 helper 仍有重複待清理。 |
| 15 | [pet historical](../../../docs/teaching/history/day-15/pet/notion/3fbe51d4509381d68666d3031d1c8011.md) | 部分接線／功能差異 | 預覽 parser／摘要／確認取消 UI 已見；來源與 dist 有分歧，OS 實際整理未接通，不能由確認或純模型冒充檔案操作成功。 |
| 15 | [analytics historical](../../../docs/teaching/history/day-15/analytics/notion/3fbe51d4509381319460de180d9e173d.md) | 後續實作已取代 | 來源 token／登入碼後台已由 PostgreSQL 帳密與 Session 主路徑取代；舊 admin 模組僅歷史／相容用途，不在當前 main。 |
| 16 | [chat historical](../../../docs/teaching/history/day-16/chat/notion/3fce51d450938143a0b9cc9a3f93860e.md) | 程式已見／仍有缺口 | 房間畫面、多行輸入與歷史未接通提示已見，舊 SSR 提示缺漏已由後續來源修正；真正歷史查詢、作者與完整 ACL 仍未完成。 |
| 16 | [pet historical](../../../docs/teaching/history/day-16/pet/notion/3fce51d4509381e9aa03d633822eee89.md) | 程式已見／歷史驗證保留 | 控制窗 CloseRequested 已攔截並 hide，open_control_window show/focus 同窗；舊「關閉後無法恢復」缺漏已在來源修正。歷史 GUI 證據限原來源，本次未重跑。 |
| 16 | [analytics historical](../../../docs/teaching/history/day-16/analytics/notion/3fce51d45093813fa38ae37a47ae4959.md) | 程式已見／待當次驗收 | 正式 LoginView／ChangePasswordView／Dashboard 均已由 Leptos renderer 接到 password_login；舊「登入／改密碼仍 HTML 字串」或「尚未套用」已過時；目前 DB／瀏覽器未新驗。 |
| 17 | [chat historical](../../../docs/teaching/history/day-17/chat/notion/3fce51d45093818b913bebb8dab5ab65.md) | 程式已見／歷史驗證保留 | 清單、搜尋與失敗保留／刷新流程已見；已有舊來源瀏覽器切片證據，當次來源未重跑。舊刷新 helper 仍有重複待清理。 |
| 17 | [pet historical](../../../docs/teaching/history/day-17/pet/notion/3fce51d4509381ca83e3e825f0313698.md) | 部分接線／功能差異 | 幾何／開關／偏差 guard 程式存在，但 roam runtime 固定找 pet 視窗，動態生活窗用 pet-UUID；真正逐寵物漫遊與拖曳／多螢幕仍有接線缺口。 |
| 17 | [analytics historical](../../../docs/teaching/history/day-17/analytics/notion/3fce51d4509381ea871ec9c92da37c1b.md) | 部分接線／功能差異 | 舊 admin::touch_session／response layer 不在主路徑；password_login::session 有獨立 PG 續期，錯誤回應時 cookie/server 時鐘一致性仍待正式驗證。 |
| 18 | [chat historical](../../../docs/teaching/history/day-18/chat/notion/3fde51d45093816bb36dd9099190d907.md) | 部分接線／功能差異 | SQLite 與搜尋已見；稽核寫入、作者身分與歷史庫 migration 相容性仍有缺口，不能由 fresh DB 結果推定升級完成。 |
| 18 | [pet historical](../../../docs/teaching/history/day-18/pet/notion/3fde51d4509381bfa106ea4d9b190480.md) | 部分接線／功能差異 | 幾何／開關／偏差 guard 程式存在，但 roam runtime 固定找 pet 視窗，動態生活窗用 pet-UUID；真正逐寵物漫遊與拖曳／多螢幕仍有接線缺口。 |
| 18 | [analytics historical](../../../docs/teaching/history/day-18/analytics/notion/3fde51d4509381c2a0d5f60041137619.md) | 部分接線／功能差異 | 舊 admin::touch_session／response layer 不在主路徑；password_login::session 有獨立 PG 續期，錯誤回應時 cookie/server 時鐘一致性仍待正式驗證。 |
| 19 | [chat historical](../../../docs/teaching/history/day-19/chat/github.md) | 程式已見／歷史驗證保留 | 清單、搜尋與失敗保留／刷新流程已見；已有舊來源瀏覽器切片證據，當次來源未重跑。舊刷新 helper 仍有重複待清理。 |
| 19 | [pet historical](../../../docs/teaching/history/day-19/pet/github.md) | 部分接線／功能差異 | 幾何／開關／偏差 guard 程式存在，但 roam runtime 固定找 pet 視窗，動態生活窗用 pet-UUID；真正逐寵物漫遊與拖曳／多螢幕仍有接線缺口。 |
| 19 | [analytics historical](../../../docs/teaching/history/day-19/analytics/github.md) | 僅舊流程／正式入口仍缺 | 成功登入額度／Retry-After 測試仍屬 legacy admin；正式 password_login 尚未接通同等帳號級等待政策，不能把舊測試當完成。 |
| 20 | [chat historical](../../../docs/teaching/history/day-20/chat/github.md) | 程式已見／歷史驗證保留 | 清單、搜尋與失敗保留／刷新流程已見；已有舊來源瀏覽器切片證據，當次來源未重跑。舊刷新 helper 仍有重複待清理。 |
| 20 | [pet historical](../../../docs/teaching/history/day-20/pet/github.md) | 部分接線／功能差異 | 幾何／開關／偏差 guard 程式存在，但 roam runtime 固定找 pet 視窗，動態生活窗用 pet-UUID；真正逐寵物漫遊與拖曳／多螢幕仍有接線缺口。 |
| 20 | [analytics historical](../../../docs/teaching/history/day-20/analytics/github.md) | 僅舊流程／正式入口仍缺 | 成功登入額度／Retry-After 測試仍屬 legacy admin；正式 password_login 尚未接通同等帳號級等待政策，不能把舊測試當完成。 |
| 21 | [chat historical](../../../docs/teaching/history/day-21/chat/github.md) | 後續實作已取代 | 當年的登入不可用提示已由完整 SignInPage 表單取代；僅供歷史，不能把舊提示再套回去。 |
| 21 | [pet historical](../../../docs/teaching/history/day-21/pet/github.md) | 程式已見／仍有缺口 | 建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| 21 | [analytics historical](../../../docs/teaching/history/day-21/analytics/github.md) | 後續實作已取代 | 來源 token／登入碼後台已由 PostgreSQL 帳密與 Session 主路徑取代；舊 admin 模組僅歷史／相容用途，不在當前 main。 |
| 22 | [chat historical](../../../docs/teaching/history/day-22/chat/github.md) | 程式已見／待當次驗收 | 註冊、帳密／Session、完成頁、會員守衛、登出、個人頁建房及 WS 守衛的對應 caller 已見；舊來源實證保留其範圍，本次不追認全部驗收。 |
| 22 | [pet historical](../../../docs/teaching/history/day-22/pet/github.md) | 程式已見／仍有缺口 | 建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| 22 | [analytics historical](../../../docs/teaching/history/day-22/analytics/github.md) | 後續實作已取代 | CLI／startup 已共用 ensure_default 建立首次 admin/admin；舊 initialize 仍被整合測試 fixture 呼叫，不能直接刪除。 |
| 23 | [chat historical](../../../docs/teaching/history/day-23/chat/github.md) | 程式已見／待當次驗收 | 註冊、帳密／Session、完成頁、會員守衛、登出、個人頁建房及 WS 守衛的對應 caller 已見；舊來源實證保留其範圍，本次不追認全部驗收。 |
| 23 | [pet historical](../../../docs/teaching/history/day-23/pet/github.md) | 程式已見／歷史驗證保留 | 控制窗 CloseRequested 已攔截並 hide，open_control_window show/focus 同窗；舊「關閉後無法恢復」缺漏已在來源修正。歷史 GUI 證據限原來源，本次未重跑。 |
| 23 | [analytics historical](../../../docs/teaching/history/day-23/analytics/github.md) | 程式已見／待當次驗收 | 正式 LoginView／ChangePasswordView／Dashboard 均已由 Leptos renderer 接到 password_login；舊「登入／改密碼仍 HTML 字串」或「尚未套用」已過時；目前 DB／瀏覽器未新驗。 |
| 24 | [chat r1](../../../docs/teaching/history/day-24/chat/github.md) | 程式已見／待當次驗收 | 註冊、帳密／Session、完成頁、會員守衛、登出、個人頁建房及 WS 守衛的對應 caller 已見；舊來源實證保留其範圍，本次不追認全部驗收。 |
| 24 | [pet r2](../../../docs/teaching/history/day-24/pet/github.md) | 程式已見／仍有缺口 | 建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| 24 | [analytics r1](../../../docs/teaching/history/day-24/analytics/github.md) | 程式已見／仍有缺口 | 對應改密碼／Session、來源切換、事件類型與標籤已在正式 password_login／Leptos／PG caller；不再列單純未套用。舊標籤測試與正式授權／角色等缺口仍需核對。 |
| 25 | [chat historical](../../../docs/teaching/history/day-25/archive/notion/chat.md) | 程式已見／待當次驗收 | 註冊、帳密／Session、完成頁、會員守衛、登出、個人頁建房及 WS 守衛的對應 caller 已見；舊來源實證保留其範圍，本次不追認全部驗收。 |
| 25 | [pet historical](../../../docs/teaching/history/day-25/archive/notion/pet.md) | 程式已見／仍有缺口 | 建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| 25 | [analytics historical](../../../docs/teaching/history/day-25/analytics/notion/400e51d45093810796cbd6074c8d35c3.md) | 程式已見／仍有缺口 | 對應改密碼／Session、來源切換、事件類型與標籤已在正式 password_login／Leptos／PG caller；不再列單純未套用。舊標籤測試與正式授權／角色等缺口仍需核對。 |
| 26 | [chat r1](../../../docs/teaching/history/day-26/chat/github.md) | 程式已見／待當次驗收 | 註冊、帳密／Session、完成頁、會員守衛、登出、個人頁建房及 WS 守衛的對應 caller 已見；舊來源實證保留其範圍，本次不追認全部驗收。 |
| 26 | [pet r1](../../../docs/teaching/daily/day-26/pet.md) | 程式已見／仍有缺口 | 建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| 26 | [analytics r1](../../../docs/teaching/history/day-26/analytics/github.md) | 程式已見／仍有缺口 | 對應改密碼／Session、來源切換、事件類型與標籤已在正式 password_login／Leptos／PG caller；不再列單純未套用。舊標籤測試與正式授權／角色等缺口仍需核對。 |
| 27 | [chat r1](../../../docs/teaching/history/day-27/chat/github.md) | 程式已見／待當次驗收 | 註冊、帳密／Session、完成頁、會員守衛、登出、個人頁建房及 WS 守衛的對應 caller 已見；舊來源實證保留其範圍，本次不追認全部驗收。 |
| 27 | [pet r1](../../../docs/teaching/daily/day-27/pet.md) | 程式已見／仍有缺口 | 建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| 27 | [analytics r1](../../../docs/teaching/history/day-27/analytics/github.md) | 程式已見／仍有缺口 | 對應改密碼／Session、來源切換、事件類型與標籤已在正式 password_login／Leptos／PG caller；不再列單純未套用。舊標籤測試與正式授權／角色等缺口仍需核對。 |
| 28 | [chat r1](../../../docs/teaching/daily/day-28/chat.md) | 原文待勘誤／現碼另記 | SignInPage 已有正式表單；原文的 leptos::ssr::render_to_string 已不適用，現行程式用 view!/.to_html()。後續 r2 已說明 API，原篇仍不能直接原樣套用。 |
| 28 | [pet r1](../../../docs/teaching/daily/day-28/pet.md) | 程式已見／仍有缺口 | 建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| 28 | [analytics r1](../../../docs/teaching/daily/day-28/analytics.md) | 原文待勘誤／現碼另記 | Dashboard Leptos renderer 已見；原文 SSR API／舊總數標籤測試需版本化勘誤，不能原樣重貼。 |
| 29 | [chat r1](../../../docs/teaching/history/day-29/chat/github.md) | 程式已見／待當次驗收 | 註冊、帳密／Session、完成頁、會員守衛、登出、個人頁建房及 WS 守衛的對應 caller 已見；舊來源實證保留其範圍，本次不追認全部驗收。 |
| 29 | [pet r1](../../../docs/teaching/daily/day-29/pet.md) | 程式已見／仍有缺口 | 建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| 29 | [analytics r1](../../../docs/teaching/history/day-29/analytics/github.md) | 程式已見／仍有缺口 | 對應改密碼／Session、來源切換、事件類型與標籤已在正式 password_login／Leptos／PG caller；不再列單純未套用。舊標籤測試與正式授權／角色等缺口仍需核對。 |
| 30 | [chat r3](../../../docs/teaching/daily/day-30/chat.md) | 程式已見／待當次驗收 | 註冊、帳密／Session、完成頁、會員守衛、登出、個人頁建房及 WS 守衛的對應 caller 已見；舊來源實證保留其範圍，本次不追認全部驗收。 |
| 30 | [pet r2](../../../docs/teaching/daily/day-30/pet.md) | 程式已見／仍有缺口 | 建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| 30 | [analytics r2](../../../docs/teaching/daily/day-30/analytics.md) | 程式已見／待當次驗收 | 正式 LoginView／ChangePasswordView／Dashboard 均已由 Leptos renderer 接到 password_login；舊「登入／改密碼仍 HTML 字串」或「尚未套用」已過時；目前 DB／瀏覽器未新驗。 |
| 31 | [chat r1](../../../docs/teaching/daily/day-31/chat.md) | 程式已見／待當次驗收 | 註冊、帳密／Session、完成頁、會員守衛、登出、個人頁建房及 WS 守衛的對應 caller 已見；舊來源實證保留其範圍，本次不追認全部驗收。 |
| 31 | [pet r1](../../../docs/teaching/daily/day-31/pet.md) | 程式已見／歷史驗證保留 | 控制窗 CloseRequested 已攔截並 hide，open_control_window show/focus 同窗；舊「關閉後無法恢復」缺漏已在來源修正。歷史 GUI 證據限原來源，本次未重跑。 |
| 31 | [analytics r1](../../../docs/teaching/daily/day-31/analytics.md) | 程式已見／待當次驗收 | 正式 LoginView／ChangePasswordView／Dashboard 均已由 Leptos renderer 接到 password_login；舊「登入／改密碼仍 HTML 字串」或「尚未套用」已過時；目前 DB／瀏覽器未新驗。 |
| 32 | [chat r1](../../../docs/teaching/daily/day-32/chat.md) | 程式已見／仍有缺口 | 房間畫面、多行輸入與歷史未接通提示已見，舊 SSR 提示缺漏已由後續來源修正；真正歷史查詢、作者與完整 ACL 仍未完成。 |
| 32 | [pet r1](../../../docs/teaching/daily/day-32/pet.md) | 程式已見／仍有缺口 | 建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| 32 | [analytics r1](../../../docs/teaching/daily/day-32/analytics.md) | 程式已見／仍有缺口 | 對應改密碼／Session、來源切換、事件類型與標籤已在正式 password_login／Leptos／PG caller；不再列單純未套用。舊標籤測試與正式授權／角色等缺口仍需核對。 |
| 33 | [chat r1](../../../docs/teaching/daily/day-33/chat.md) | 程式已見／待當次驗收 | room_list_feedback 與 has_snapshot 已接 RoomList；不再列單純未套用，當次瀏覽器狀態驗收未執行。 |
| 33 | [pet r1](../../../docs/teaching/daily/day-33/pet.md) | 來源 JS 已驗／產物仍有差異 | 來源 openPetControls 已先 select_pet 再開控制窗，既有三個 source 契約測試全過；tracked dist 的同測試一過兩敗，完整打包／GUI 未完成。 |
| 33 | [analytics r1](../../../docs/teaching/daily/day-33/analytics.md) | 程式已見／仍有缺口 | 對應改密碼／Session、來源切換、事件類型與標籤已在正式 password_login／Leptos／PG caller；不再列單純未套用。舊標籤測試與正式授權／角色等缺口仍需核對。 |
| 34 | [chat r2](../../../docs/teaching/daily/day-34/chat-r2.md) | 程式已見／待當次驗收 | 現行閱讀版本為 r2；最新來源已移除 LoginView／模組宣告，GET /login redirect /signin 與唯一表單測試已見，POST /login 保留。本次尚未執行 Rust／HTTP／瀏覽器，不追認 Gate 2。 |
| 34 | [pet r1](../../../docs/teaching/daily/day-34/pet.md) | 程式已見／待當次驗收 | 名稱規則已移到 pet-application，Tauri 及 PetStore 使用同一函式，含控制字元拒絕與測試；pet-persistence 仍空。本次 Rust／SQLite／GUI 未執行。 |
| 34 | [analytics r1](../../../docs/teaching/daily/day-34/analytics.md) | 程式已見／待當次驗收 | ensure_default 已接主啟動與 CLI，login 允許預設 admin 抵達 Argon2 驗證，首次改密碼路由重用；本文「admin=4 bytes」已有獨立勘誤為5。本次新 PG／HTTP／瀏覽器未重跑。 |

## 已被取代的閱讀版本

[Chat Day34 r1](../daily/day-34/chat.md) 保留原文與證據，狀態為「已被 r2 取代，不再建議套用」；現行閱讀入口為 [r2](../daily/day-34/chat-r2.md)，最新來源已見去重修正與測試，當次功能驗收尚未執行。

## 證據與後續調整

[機器可讀逐篇狀態與全文雜湊](article-states.json) 含每篇原文 blob/SHA256、原交付記錄、歷史驗證、當次程式對應、內容風險標記與驗收邊界。[後續調整清單](../../FUTURE_WORK.md) 沿用原功能與 FIFO；不由這份核對增加 Day 或結案。

歷史主要證據：[舊84篇核對](../history/evidence/2026-10-23-all-articles-9c2b7cf/ALL_ARTICLES.md)、[九筆切片證據](../history/evidence/2026-10-23-browser-sqlite-9c2b7cf/archive-manifest.json)、[固定來源冷建置](../history/evidence/2026-10-24-formal-f3f8255/SUMMARY.json)、[當次相關來源與 JS 結果](../../source-review/2026-10-25-duplicate-application-review.md)。
