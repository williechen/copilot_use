# 每日新文章

<!-- comparison-5c713553:start -->
[最新三專案程式／文章比對](../../source-review/2026-10-25-full-comparison-5c713553.md)：105個閱讀版本、111份修訂、21項差異；來源 `5c713553`，局部測試與完整驗收分開記錄。
<!-- comparison-5c713553:end -->

[問題文章與後續更正總表](../current/ARTICLE_CORRECTIONS.md)：已整併最新瀏覽器／Pet 原生證據；下方個別來源的交付／測試摘要保留其當次範圍。

<!-- duplicate-review:start -->
[後續調整清單（13 項）](../../FUTURE_WORK.md) · [原始核對報告](../../source-review/2026-10-25-duplicate-application-review.md)。可清除、先遷測試與保留項目分別列明，產品套用／驗收另記。
<!-- duplicate-review:end -->

<!-- daily-delivery:start -->
## 最新交付與下一篇

| 專案 | 最新文章 | 下一篇 | 狀態 |
| --- | --- | --- | --- |
| Chat | [Day37 r2：房間搜尋完成時更新真實畫面狀態](day-37/chat-r2.md) | Day38 | 教學已交付／程式未套用／新測試未執行／產品未驗收 |
| Pet | [Day36 r3：重啟恢復全部未回籠寵物，而非只恢復最後選取者](day-36/pet-r3.md) | Day37 | 程式已存在／相關實測通過／完整產品未驗收 |
| Analytics | [Day36 r2：精確事件統計 API 改用人員 Session，不再接受來源 token 查詢](day-36/analytics-r2.md) | Day37 | 程式已存在／相關實測通過／完整產品未驗收 |

文案修復與最新 PG 結果見 [afdbb6e 驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-assertions-afdbb6e.md)；前次編譯修復見 [bb95a60 驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-compile-bb95a60.md)；前次套用／測試見 [e753c36 更新驗證](https://github.com/williechen/copilot_use/blob/main/docs/source-review/2026-10-25-update-e753c36.md)；原 Day35 正文與交付證據保留，完整產品驗收尚未完成。
<!-- daily-delivery:end -->

2026-10-24 目錄已同步至 [williechen/rust_ai_ring](https://github.com/williechen/rust_ai_ring)：保留 `chat_apps`、`pet_apps`、`analytics_apps`、`crates` 四個 workspace；[新目錄與套件對照](../../SOURCE_LAYOUT.md)。後續教學依當次來源 SHA 調整程式、Cargo、imports、呼叫端與測試。

[共用文章](../../COMMON_LEARNING_RULES.md) · [Chat 首頁](../chat/README.md) · [Pet 首頁](../pet/README.md) · [Analytics 首頁](../analytics/README.md) · [目前進度](../current/STATUS.md)

**必要 Gate 2 驗證通過後，正文原樣移至 `../archived/day-NN/{project}.md`。** 本目錄只追蹤尚待驗收或勘誤的教學；舊有 `history` 已封存文章與證據保留原位，不批次重寫。

## 現存狀態與保留文章

[全部逐篇狀態](../current/ARTICLE_STATUS.md) 為舊內容與當前程式／證據的單一核對入口。下表只列仍在 daily 的原文；搬入 history 的原篇及九筆舊來源切片證據保留原路徑，不因本次靜態核對再搬移。

| 專案／Day／版本 | 原文 | 現存狀態 |
| --- | --- | --- |
| pet / 26 / r1 | [文章](day-26/pet.md) | 程式已見／仍有缺口：建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| pet / 27 / r1 | [文章](day-27/pet.md) | 程式已見／仍有缺口：建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| chat / 28 / r1 | [文章](day-28/chat.md) | 原文待勘誤／現碼另記：SignInPage 已有正式表單；原文的 leptos::ssr::render_to_string 已不適用，現行程式用 view!/.to_html()。後續 r2 已說明 API，原篇仍不能直接原樣套用。 |
| pet / 28 / r1 | [文章](day-28/pet.md) | 程式已見／仍有缺口：建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| analytics / 28 / r1 | [文章](day-28/analytics.md) | 原文待勘誤／現碼另記：Dashboard Leptos renderer 已見；原文 SSR API／舊總數標籤測試需版本化勘誤，不能原樣重貼。 |
| pet / 29 / r1 | [文章](day-29/pet.md) | 程式已見／仍有缺口：建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| chat / 30 / r3 | [文章](day-30/chat.md) | 程式已見／待當次驗收：註冊、帳密／Session、完成頁、會員守衛、登出、個人頁建房及 WS 守衛的對應 caller 已見；舊來源實證保留其範圍，本次不追認全部驗收。 |
| pet / 30 / r2 | [文章](day-30/pet.md) | 程式已見／仍有缺口：建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| analytics / 30 / r2 | [文章](day-30/analytics.md) | 程式已見／待當次驗收：正式 LoginView／ChangePasswordView／Dashboard 均已由 Leptos renderer 接到 password_login；舊「登入／改密碼仍 HTML 字串」或「尚未套用」已過時；目前 DB／瀏覽器未新驗。 |
| chat / 31 / r1 | [文章](day-31/chat.md) | 程式已見／待當次驗收：註冊、帳密／Session、完成頁、會員守衛、登出、個人頁建房及 WS 守衛的對應 caller 已見；舊來源實證保留其範圍，本次不追認全部驗收。 |
| pet / 31 / r1 | [文章](day-31/pet.md) | 程式已見／歷史驗證保留：控制窗 CloseRequested 已攔截並 hide，open_control_window show/focus 同窗；舊「關閉後無法恢復」缺漏已在來源修正。歷史 GUI 證據限原來源，本次未重跑。 |
| analytics / 31 / r1 | [文章](day-31/analytics.md) | 程式已見／待當次驗收：正式 LoginView／ChangePasswordView／Dashboard 均已由 Leptos renderer 接到 password_login；舊「登入／改密碼仍 HTML 字串」或「尚未套用」已過時；目前 DB／瀏覽器未新驗。 |
| chat / 32 / r1 | [文章](day-32/chat.md) | 程式已見／仍有缺口：房間畫面、多行輸入與歷史未接通提示已見，舊 SSR 提示缺漏已由後續來源修正；真正歷史查詢、作者與完整 ACL 仍未完成。 |
| pet / 32 / r1 | [文章](day-32/pet.md) | 程式已見／仍有缺口：建立／列表／逐 ID Runtime、出入籠、選取／元件的對應切片已見；歷史 GUI 證據限原來源，本次失敗注入、三窗／重啟／產物 GUI 尚未重跑。 |
| analytics / 32 / r1 | [文章](day-32/analytics.md) | 程式已見／仍有缺口：對應改密碼／Session、來源切換、事件類型與標籤已在正式 password_login／Leptos／PG caller；不再列單純未套用。舊標籤測試與正式授權／角色等缺口仍需核對。 |
| chat / 33 / r1 | [文章](day-33/chat.md) | 程式已見／待當次驗收：room_list_feedback 與 has_snapshot 已接 RoomList；不再列單純未套用，當次瀏覽器狀態驗收未執行。 |
| pet / 33 / r1 | [文章](day-33/pet.md) | 來源 JS 已驗／產物仍有差異：來源 openPetControls 已先 select_pet 再開控制窗，既有三個 source 契約測試全過；tracked dist 的同測試一過兩敗，完整打包／GUI 未完成。 |
| analytics / 33 / r1 | [文章](day-33/analytics.md) | 程式已見／仍有缺口：對應改密碼／Session、來源切換、事件類型與標籤已在正式 password_login／Leptos／PG caller；不再列單純未套用。舊標籤測試與正式授權／角色等缺口仍需核對。 |
| chat / 34 / r2 | [文章](day-34/chat-r2.md) | 程式已見／待當次驗收：現行閱讀版本為 r2；最新來源已移除 LoginView／模組宣告，GET /login redirect /signin 與唯一表單測試已見，POST /login 保留。本次尚未執行 Rust／HTTP／瀏覽器，不追認 Gate 2。 |
| pet / 34 / r1 | [文章](day-34/pet.md) | 程式已見／待當次驗收：名稱規則已移到 pet-application，Tauri 及 PetStore 使用同一函式，含控制字元拒絕與測試；pet-persistence 仍空。本次 Rust／SQLite／GUI 未執行。 |
| analytics / 34 / r1 | [文章](day-34/analytics.md) | 程式已見／待當次驗收：ensure_default 已接主啟動與 CLI，login 允許預設 admin 抵達 Argon2 驗證，首次改密碼路由重用；本文「admin=4 bytes」已有獨立勘誤為5。本次新 PG／HTTP／瀏覽器未重跑。 |
| chat / 34 / r1 | [文章](day-34/chat.md) | 已被 r2 取代：已由 chat-r2.md 取代閱讀／套用指引；r1 曾被套入而造成登入頁重複，最新來源已依 r2 移除；原文不再建議照貼，證據保留。 |

原文「未套用／未執行」保留交付當時語境；不能由本表的程式所見推論整篇 Green 原樣可用，或當次 DB／GUI 已驗收。Chat r1 已被 r2 取代；Leptos 舊 SSR API、路徑與舊標籤仍按 [後續調整](../../FUTURE_WORK.md) 銜接。

## 封存與進度

只有必要 Gate 2 實測成立才移動原篇與更新驗收；本次僅推進已完整交付並精確讀回的專案，不新增驗收封存、不結案 FIFO。各案下一篇以即時狀態為準，以 [learning-state](../../learning-state.json)／[manifest](../current/manifest.json) 一致值為準。

[歷史文章與來源](../history/README.md) · [封存政策](../archived/README.md) · [共同規則](../../COMMON_LEARNING_RULES.md)
