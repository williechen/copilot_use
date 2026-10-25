# 問題文章與後續更正索引

2026-10-25 整理；文件基準 `2fc0fc4`，產品實作／實測依 `rust_ai_ring@5d28e96` 及各報告明列的原 SHA。收錄 Day2–35 三案 **102 篇目前閱讀版本、104 份版本**，兩個舊修訂保留；Day1 三案正文仍缺來源，不補造。逐份原文 blob／SHA256 已核對未變。

這是既有逐篇盤點、後續編譯／斷言修復與瀏覽器／原生實測的整併，不是重新宣稱全庫功能驗收。**功能待辦唯一正本仍是 [FUTURE_WORK](../../FUTURE_WORK.md)**；本頁只把文章對應回同一功能項，不建立新 FIFO 或排定未來 Day。原文、原交付證據及封存位置保留，更正於下一個適用教學的新實作／Refactor 後補充。next_day 仍由即時狀態管理，本次不變。

## 需要直接留意的文章

| 原文章 | 問題／目前狀態 | 後續處理 |
| --- | --- | --- |
| [28 r1](../daily/day-28/chat.md)；[28 r1](../daily/day-28/analytics.md) | 原文 Leptos SSR API 已不適用；產品已採 view!／to_html | 提供適用現碼的完整測試與必要 imports，不回貼舊 API。Chat 新版勘誤已存在。 |
| [27 r1](../history/day-27/analytics/github.md)、[28 r1](../daily/day-28/analytics.md)、[29 r1](../history/day-29/analytics/github.md)、[33 r1](../daily/day-33/analytics.md) | 舊總數文案斷言曾失敗，afdbb6e 已修復，44 passed／0 failed／1 ignored | 保留來源切換、Session、escaping／可及性斷言；不再當目前編譯錯誤或重教 Green。 |
| [35 r2](../daily/day-35/analytics-r2.md) | r1 漏正式路由，r2 已取代；產品編譯五個 E0425 已修復，查詢仍只用來源 token | r1 不再套用；修正式人員 Session；舊 pg_store pub 路徑相容性與 server PG CI 另核對。 |
| [34 r1](../daily/day-34/analytics.md) | 原文 admin=4 bytes 的說明有誤，admin 為 5 bytes；已有獨立勘誤。ensure_default 及首次改密碼已見 caller 並局部實測 | 保留原文及既有勘誤，後續引用用 5 bytes；不重新實作初始化。 |
| [34 r2](../daily/day-34/chat-r2.md) | r1 重複登入元件已被 r2 去重；r2 的舊 SSR 字串是勘誤解說 | 只讀 r2，不再建立 LoginView，也不把解說誤列新 API 錯誤。 |
| [15 historical](../history/day-15/chat/notion/3fbe51d450938130a3c0c93171105f3a.md)、[19 historical](../history/day-19/chat/github.md)、[20 historical](../history/day-20/chat/github.md)、[33 r1](../daily/day-33/chat.md) | 清單／搜尋與重試已局部通過；首頁雙提示只在本地候選 patch 修正，D06 舊 helper 尚在 | 套用候選需使用者後續處理；測試遷至真正 helper，再刪舊 helper。 |
| [16 historical](../history/day-16/chat/notion/3fce51d450938143a0b9cc9a3f93860e.md)、[18 historical](../history/day-18/chat/notion/3fde51d45093816bb36dd9099190d907.md)、[29 r1](../history/day-29/chat/github.md)、[30 r3](../daily/day-30/chat.md)、[32 r1](../daily/day-32/chat.md) | 真實作者／ACL、歷史、登出撤銷舊 WS 及匿名房間回登入未完整接通 | 接回原 Session／作者／歷史功能；提示缺歷史不是完成歷史查詢。 |
| [12 historical](../history/day-12/pet/notion/3fae51d4509381d0a80ddbbaf251fb03.md)、[17 historical](../history/day-17/pet/notion/3fce51d4509381ca83e3e825f0313698.md)、[18 historical](../history/day-18/pet/notion/3fde51d4509381bfa106ea4d9b190480.md)、[19 historical](../history/day-19/pet/github.md)、[20 historical](../history/day-20/pet/github.md) | 固定 pet label 不符動態 UUID；原生日誌重現失敗 | 修真實逐 ID 生活窗，驗漫遊／拖曳／多螢幕。 |
| [21 historical](../history/day-21/pet/github.md)、[24 r2](../history/day-24/pet/github.md)、[27 r1](../daily/day-27/pet.md)、[28 r1](../daily/day-28/pet.md)、[29 r1](../daily/day-29/pet.md)、[30 r2](../daily/day-30/pet.md)、[35 r1](../daily/day-35/pet.md) | 三窗／單隻回籠局部通過，但重啟只恢復最後選取一隻，補償故障注入未驗 | 恢復所有未回籠 ID，補真實關窗／DB 失敗驗證。 |
| [16 historical](../history/day-16/pet/notion/3fce51d4509381e9aa03d633822eee89.md)、[23 historical](../history/day-23/pet/github.md)、[31 r1](../daily/day-31/pet.md)、[32 r1](../daily/day-32/pet.md)、[33 r1](../daily/day-33/pet.md) | 控制台關閉／重開與逐 ID 操作已有真實桌面證據；不再整批列 GUI 未執行 | 保留仍未驗的精確選取／錯誤分支、打包更新與 FIFO，不重做 hide/show。 |
| [13 historical](../history/day-13/analytics/notion/3fae51d4509381808a9cd813423d52d1.md)、[14 historical](../history/day-14/analytics/notion/3fbe51d4509381f2a9a2e4b608706eea.md)、[17 historical](../history/day-17/analytics/notion/3fce51d4509381ea871ec9c92da37c1b.md)、[18 historical](../history/day-18/analytics/notion/3fde51d4509381c2a0d5f60041137619.md)、[19 historical](../history/day-19/analytics/github.md)、[20 historical](../history/day-20/analytics/github.md) | legacy Bearer helper／admin 限流／續期不能替代正式 password_login | 正式路由驗人員權限及登入等待／Session 契約；不刪仍有效測試。 |

## 掃描標記如何處理

既有 100 份版本盤點標記：舊路徑 87、舊庫名 84、省略候選 7、廢除政策 6、SSR 字串 3；計數可重疊，不能加總成錯誤篇數。7 個省略候選含 JavaScript rest args、註解位置、placeholder 或歷史斷言，沒有一律判成 Green 缺碼；3 個 SSR 命中中，Chat 新版是在說明舊 API 勘誤。舊政策只供歷史，不恢復其效力。下表保留逐篇原標記與目前後續分類。

舊 tracked dist 的兩敗屬舊產物，已由後續來源清理及重建取代；仍待新 dist 接 CI 與實際安裝驗收。Pet 啟動 Connection refused 是本地驗收 runner 未等 Trunk 就緒，已修啟動順序，不新增產品 bug。名稱多出的右引號是使用者輸入，未列程式改名錯誤。

## 全部目前閱讀版本與關聯修正

「關聯」表示此篇所涉功能仍需處理或驗證，不表示整篇錯誤；最新局部實測見 [目前狀態](STATUS.md)、[瀏覽器／HTTP 報告](../../source-review/2026-10-25-acceptance-5d28e96.md)及 [Pet 原生報告](../../source-review/2026-10-25-pet-native-5d28e96.md)。完整原 SHA、標記與邊界見 [JSON](article-corrections.json)。

### Chat

| 文章 | 內容注意 | 關聯原功能／修正 |
| --- | --- | --- |
| [Day2 historical](../history/day-02/chat/notion/3f4e51d450938170954ff09378afc11a.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day3 historical](../history/day-03/chat/notion/3f4e51d45093819c975fc886e09fe861.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day4 historical](../history/day-04/chat/notion/3f5e51d45093811ab62acc4bc8fc83fa.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day5 historical](../history/day-05/chat/notion/3f5e51d45093817e93b6ea7c37c3852e.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day6 historical](../history/day-06/chat/notion/3f5e51d45093818ab246f8fcdd0b39f6.md) | 歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day7 historical](../history/day-07/chat/notion/3f5e51d45093815c82c7e32b73b567fa.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day8 historical](../history/day-08/chat/notion/3f5e51d45093814cb540cf6772e9b8d2.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[來源事件／Outbox／gRPC／SSE／服務配對](../../FUTURE_WORK.md#event-transport) |
| [Day9 historical](../history/day-09/chat/notion/3f8e51d45093814b992eea570411bd7d.md) | 未新增文字錯誤判定 | 原驗證邊界保留；未新增特定勘誤 |
| [Day10 historical](../history/day-10/chat/notion/3f9e51d45093818a849dd8b1fbf2c045.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day11 historical](../history/day-11/chat/notion/3f9e51d4509381508c0dd3ca22cf5df7.md) | 歷史路徑、語法／省略候選已辨別 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[來源事件／Outbox／gRPC／SSE／服務配對](../../FUTURE_WORK.md#event-transport) |
| [Day12 historical](../history/day-12/chat/notion/3fae51d45093812bbf88c3e7f51d6522.md) | 歷史庫名、歷史路徑、廢除政策僅歷史 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day13 historical](../history/day-13/chat/notion/3fae51d4509381e499e5e02cfba58336.md) | 歷史庫名、歷史路徑、廢除政策僅歷史 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day14 historical](../history/day-14/chat/notion/3fbe51d450938149af5dd68abc6109bd.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day15 historical](../history/day-15/chat/notion/3fbe51d450938130a3c0c93171105f3a.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat 房間與訊息回饋](../../FUTURE_WORK.md#chat-feedback)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day16 historical](../history/day-16/chat/notion/3fce51d450938143a0b9cc9a3f93860e.md) | 歷史庫名、歷史路徑、語法／省略候選已辨別 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat 歷史／migration／稽核](../../FUTURE_WORK.md#chat-history)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |
| [Day17 historical](../history/day-17/chat/notion/3fce51d45093818b913bebb8dab5ab65.md) | 歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat 房間與訊息回饋](../../FUTURE_WORK.md#chat-feedback)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day18 historical](../history/day-18/chat/notion/3fde51d45093816bb36dd9099190d907.md) | 歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat 歷史／migration／稽核](../../FUTURE_WORK.md#chat-history)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |
| [Day19 historical](../history/day-19/chat/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat 房間與訊息回饋](../../FUTURE_WORK.md#chat-feedback)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day20 historical](../history/day-20/chat/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat 房間與訊息回饋](../../FUTURE_WORK.md#chat-feedback)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day21 historical](../history/day-21/chat/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day22 historical](../history/day-22/chat/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |
| [Day23 historical](../history/day-23/chat/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |
| [Day24 r1](../history/day-24/chat/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |
| [Day25 historical](../history/day-25/archive/notion/chat.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |
| [Day26 r1](../history/day-26/chat/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |
| [Day27 r1](../history/day-27/chat/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |
| [Day28 r1](../daily/day-28/chat.md) | 歷史庫名、歷史路徑、SSR 文字，需辨別語境 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |
| [Day29 r1](../history/day-29/chat/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |
| [Day30 r3](../daily/day-30/chat.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |
| [Day31 r1](../daily/day-31/chat.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |
| [Day32 r1](../daily/day-32/chat.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat 歷史／migration／稽核](../../FUTURE_WORK.md#chat-history)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |
| [Day33 r1](../daily/day-33/chat.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Chat 房間與訊息回饋](../../FUTURE_WORK.md#chat-feedback)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day34 r2](../daily/day-34/chat-r2.md) | 歷史庫名、SSR 文字，需辨別語境 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day35 r1](../daily/day-35/chat.md) | 未新增文字錯誤判定 | [原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup)、[Chat Session／作者／房主與 ACL](../../FUTURE_WORK.md#chat-auth) |

### Pet

| 文章 | 內容注意 | 關聯原功能／修正 |
| --- | --- | --- |
| [Day2 historical](../history/day-02/pet/notion/3f4e51d4509381e7ac5cea661af8c67d.md) | 歷史庫名、歷史路徑、語法／省略候選已辨別 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[CI／產物與測試覆蓋](../../FUTURE_WORK.md#ci-coverage) |
| [Day3 historical](../history/day-03/pet/notion/3f4e51d4509381c482a3c68721761dee.md) | 歷史庫名、歷史路徑、語法／省略候選已辨別 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[CI／產物與測試覆蓋](../../FUTURE_WORK.md#ci-coverage) |
| [Day4 historical](../history/day-04/pet/notion/3f5e51d4509381ebadf2d9adc6210e80.md) | 歷史庫名 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 設定／記憶／檔案／AI 與平台能力](../../FUTURE_WORK.md#pet-capabilities) |
| [Day5 historical](../history/day-05/pet/notion/3f5e51d4509381739b61d517b0772094.md) | 歷史庫名 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 設定／記憶／檔案／AI 與平台能力](../../FUTURE_WORK.md#pet-capabilities) |
| [Day6 historical](../history/day-06/pet/notion/3f5e51d4509381f48e6ac9a9d7dd758a.md) | 歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 設定／記憶／檔案／AI 與平台能力](../../FUTURE_WORK.md#pet-capabilities) |
| [Day7 historical](../history/day-07/pet/notion/3f5e51d4509381cd8d7efeb14bfc1b45.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 設定／記憶／檔案／AI 與平台能力](../../FUTURE_WORK.md#pet-capabilities) |
| [Day8 historical](../history/day-08/pet/notion/3f5e51d450938123b43be0fe0b543ade.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 設定／記憶／檔案／AI 與平台能力](../../FUTURE_WORK.md#pet-capabilities) |
| [Day9 historical](../history/day-09/pet/notion/3f8e51d4509381c3b989f2365fd0c0af.md) | 未新增文字錯誤判定 | [Pet 設定／記憶／檔案／AI 與平台能力](../../FUTURE_WORK.md#pet-capabilities) |
| [Day10 historical](../history/day-10/pet/notion/3f9e51d4509381f7ab32d0dc8b389a86.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 設定／記憶／檔案／AI 與平台能力](../../FUTURE_WORK.md#pet-capabilities) |
| [Day11 historical](../history/day-11/pet/notion/3f9e51d450938152a147c931e3e945be.md) | 歷史路徑、語法／省略候選已辨別 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[CI／產物與測試覆蓋](../../FUTURE_WORK.md#ci-coverage) |
| [Day12 historical](../history/day-12/pet/notion/3fae51d4509381d0a80ddbbaf251fb03.md) | 歷史庫名、歷史路徑、廢除政策僅歷史 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 逐 ID 漫遊／桌面例外](../../FUTURE_WORK.md#pet-roaming) |
| [Day13 historical](../history/day-13/pet/notion/3fae51d45093814dbdabee6deac48554.md) | 歷史庫名、歷史路徑、廢除政策僅歷史 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 設定／記憶／檔案／AI 與平台能力](../../FUTURE_WORK.md#pet-capabilities) |
| [Day14 historical](../history/day-14/pet/notion/3fbe51d4509381e484f4eb9e1f98fc68.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 設定／記憶／檔案／AI 與平台能力](../../FUTURE_WORK.md#pet-capabilities) |
| [Day15 historical](../history/day-15/pet/notion/3fbe51d4509381d68666d3031d1c8011.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 設定／記憶／檔案／AI 與平台能力](../../FUTURE_WORK.md#pet-capabilities) |
| [Day16 historical](../history/day-16/pet/notion/3fce51d4509381e9aa03d633822eee89.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle) |
| [Day17 historical](../history/day-17/pet/notion/3fce51d4509381ca83e3e825f0313698.md) | 未新增文字錯誤判定 | [Pet 逐 ID 漫遊／桌面例外](../../FUTURE_WORK.md#pet-roaming) |
| [Day18 historical](../history/day-18/pet/notion/3fde51d4509381bfa106ea4d9b190480.md) | 未新增文字錯誤判定 | [Pet 逐 ID 漫遊／桌面例外](../../FUTURE_WORK.md#pet-roaming) |
| [Day19 historical](../history/day-19/pet/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 逐 ID 漫遊／桌面例外](../../FUTURE_WORK.md#pet-roaming) |
| [Day20 historical](../history/day-20/pet/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 逐 ID 漫遊／桌面例外](../../FUTURE_WORK.md#pet-roaming) |
| [Day21 historical](../history/day-21/pet/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle)、[Pet 外觀與生活窗可讀性](../../FUTURE_WORK.md#pet-display) |
| [Day22 historical](../history/day-22/pet/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle) |
| [Day23 historical](../history/day-23/pet/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle) |
| [Day24 r2](../history/day-24/pet/github.md) | 歷史庫名、歷史路徑、語法／省略候選已辨別 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle)、[Pet 外觀與生活窗可讀性](../../FUTURE_WORK.md#pet-display) |
| [Day25 historical](../history/day-25/archive/notion/pet.md) | 歷史庫名 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle)、[Pet 外觀與生活窗可讀性](../../FUTURE_WORK.md#pet-display) |
| [Day26 r1](../daily/day-26/pet.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle)、[Pet 外觀與生活窗可讀性](../../FUTURE_WORK.md#pet-display) |
| [Day27 r1](../daily/day-27/pet.md) | 歷史庫名、歷史路徑、語法／省略候選已辨別 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle) |
| [Day28 r1](../daily/day-28/pet.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle) |
| [Day29 r1](../daily/day-29/pet.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle) |
| [Day30 r2](../daily/day-30/pet.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle) |
| [Day31 r1](../daily/day-31/pet.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle) |
| [Day32 r1](../daily/day-32/pet.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle) |
| [Day33 r1](../daily/day-33/pet.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[CI／產物與測試覆蓋](../../FUTURE_WORK.md#ci-coverage)、[Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle) |
| [Day34 r1](../daily/day-34/pet.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day35 r1](../daily/day-35/pet.md) | 未新增文字錯誤判定 | [Pet 多寵物／回籠／重啟](../../FUTURE_WORK.md#pet-lifecycle)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |

### Analytics

| 文章 | 內容注意 | 關聯原功能／修正 |
| --- | --- | --- |
| [Day2 historical](../history/day-02/analytics/notion/3f4e51d4509381bcbddccfba35f4b329.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api) |
| [Day3 historical](../history/day-03/analytics/notion/3f4e51d4509381ebb781cf9b2f2f0db6.md) | 歷史庫名 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day4 historical](../history/day-04/analytics/notion/3f5e51d45093811a8ac5dafd3cd3bbcc.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api) |
| [Day5 historical](../history/day-05/analytics/notion/3f5e51d45093813cabb7e6a48ee3a0f0.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day6 historical](../history/day-06/analytics/notion/3f5e51d450938160a376fde5e2fad496.md) | 歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day7 historical](../history/day-07/analytics/notion/3f5e51d450938138a33ddadedb5a5e98.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day8 historical](../history/day-08/analytics/notion/3f5e51d4509381f98c33c2df7ab97663.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api) |
| [Day9 historical](../history/day-09/analytics/notion/3f8e51d45093810d830ef3fd40e419f0.md) | 未新增文字錯誤判定 | 原驗證邊界保留；未新增特定勘誤 |
| [Day10 historical](../history/day-10/analytics/notion/3f9e51d4509381269984c5f49a429b5d.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day11 historical](../history/day-11/analytics/notion/3f9e51d45093814a8054d44c4ff9fa55.md) | 歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day12 historical](../history/day-12/analytics/notion/3fae51d4509381ebb1e9f8482b5bf44e.md) | 歷史庫名、歷史路徑、廢除政策僅歷史 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance) |
| [Day13 historical](../history/day-13/analytics/notion/3fae51d4509381808a9cd813423d52d1.md) | 歷史庫名、歷史路徑、廢除政策僅歷史 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api)、[Analytics 正式人員查詢授權](../../FUTURE_WORK.md#analytics-auth) |
| [Day14 historical](../history/day-14/analytics/notion/3fbe51d4509381f2a9a2e4b608706eea.md) | 歷史庫名 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api)、[Analytics 正式人員查詢授權](../../FUTURE_WORK.md#analytics-auth) |
| [Day15 historical](../history/day-15/analytics/notion/3fbe51d4509381319460de180d9e173d.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day16 historical](../history/day-16/analytics/notion/3fce51d45093813fa38ae37a47ae4959.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session) |
| [Day17 historical](../history/day-17/analytics/notion/3fce51d4509381ea871ec9c92da37c1b.md) | 未新增文字錯誤判定 | [Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session) |
| [Day18 historical](../history/day-18/analytics/notion/3fde51d4509381c2a0d5f60041137619.md) | 歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session) |
| [Day19 historical](../history/day-19/analytics/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day20 historical](../history/day-20/analytics/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day21 historical](../history/day-21/analytics/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day22 historical](../history/day-22/analytics/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |
| [Day23 historical](../history/day-23/analytics/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session) |
| [Day24 r1](../history/day-24/analytics/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式人員查詢授權](../../FUTURE_WORK.md#analytics-auth)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session) |
| [Day25 historical](../history/day-25/analytics/notion/400e51d45093810796cbd6074c8d35c3.md) | 歷史庫名 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式人員查詢授權](../../FUTURE_WORK.md#analytics-auth)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session) |
| [Day26 r1](../history/day-26/analytics/github.md) | 歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式人員查詢授權](../../FUTURE_WORK.md#analytics-auth)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session) |
| [Day27 r1](../history/day-27/analytics/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式人員查詢授權](../../FUTURE_WORK.md#analytics-auth)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api) |
| [Day28 r1](../daily/day-28/analytics.md) | 歷史庫名、歷史路徑、SSR 文字，需辨別語境 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api)、[Analytics 正式人員查詢授權](../../FUTURE_WORK.md#analytics-auth) |
| [Day29 r1](../history/day-29/analytics/github.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式人員查詢授權](../../FUTURE_WORK.md#analytics-auth)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api) |
| [Day30 r2](../daily/day-30/analytics.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session) |
| [Day31 r1](../daily/day-31/analytics.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session) |
| [Day32 r1](../daily/day-32/analytics.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式人員查詢授權](../../FUTURE_WORK.md#analytics-auth) |
| [Day33 r1](../daily/day-33/analytics.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式人員查詢授權](../../FUTURE_WORK.md#analytics-auth)、[文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api) |
| [Day34 r1](../daily/day-34/analytics.md) | 歷史庫名、歷史路徑 | [歷史路徑／庫名／政策與範例判讀](../../FUTURE_WORK.md#layout-guidance)、[Analytics 正式登入／Session 保護](../../FUTURE_WORK.md#analytics-session) |
| [Day35 r2](../daily/day-35/analytics-r2.md) | 未新增文字錯誤判定 | [文章 API／修訂與歷史版本](../../FUTURE_WORK.md#article-api)、[Analytics 正式人員查詢授權](../../FUTURE_WORK.md#analytics-auth)、[原 13 項清理／D01–D09](../../FUTURE_WORK.md#adapter-cleanup) |

## 被取代版本

- [chat Day34 r1](../daily/day-34/chat.md)：保留原文，改讀同案同日 r2。
- [analytics Day35 r1](../daily/day-35/analytics.md)：保留原文，改讀同案同日 r2。

## 後續更正的交付條件

依即時 next_day 與原 FIFO 選適用切片，不預排 Day。當篇 Red→完整最小 Green→Refactor 後，必要時分列「上一 Day 未完成事項補充」及「舊 Day 更正補充」。列真實檔案、完整必要程式、Cargo／features／caller、版本相容與測試／UI／DB 證據；未執行明示。每篇最多結案一個 FIFO 舊單元，文件讀回不等於功能驗收；已解決項保留來源，未套用候選不標遠端完成。
