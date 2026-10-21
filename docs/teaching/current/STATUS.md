## 目前進度（2026-10-20｜增量核對）
三專案 Day23（含）以前文章均已封存；chat／pet／analytics Day23已發布並全文讀回，next_day=24。舊文更正只放下一Day的新實作之後。
先前產品狀態快照的 rust_ai_chat/main 為 `cabacb75e946b03c67c455ce13a7a4ea85652f8e`（workspace 0.2.22）。以下是該 SHA 的最近一次實測結果；本次僅整理文件，沒有重跑測試或進行新的全面複驗。
- **Chat：** 註冊表單→HTTP→SQLite 已有實作；13 項測試通過、1 項失敗，失敗為短帳號驗證實際回 409／預期 422。隔離修正候選 19 項通過，未套用 main。登入、Session 與聊天室授權主線仍待接通。
- **Pet：** 控制台→Tauri IPC→SQLite 建立並讀回寵物已接線；42 項測試通過。逐寵物生活窗身分、初始外觀及重啟載入仍待完成。
- **Analytics：** PostgreSQL 管理員初始化與不覆寫密碼已實作；真 PostgreSQL 測試 26 項通過。現有後台仍是舊 key 登入，帳密 Session、首次改密碼與角色授權尚未接通。
**共同待修：** Chat／Pet 從舊庫升級會遇到已修改 migration 的 checksum 阻擋；fresh DB 通過不代表升級相容。需先核對實際 DB 歷史，不改 checksum 或刪庫掩蓋。Chat／Pet WASM check 通過，但瀏覽器／Tauri GUI、重啟、MSRV／跨平台及完整產品驗收尚未完成。
**套件與 CI：** 最近一次官方版本核對的 28 個直接依賴均包含最新穩定版，主要版本為 Argon2 0.6.0、Tokio 1.54.0；未更新來源 lock。[同 SHA 的 Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39404616558) 仍失敗於 test-server，其他五個 workflow 成功；不以 CI 成功代替未涵蓋的功能驗收。
[完整來源／測試／套件證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-20-day22-freeze/REPORT.md) · [現行進度索引](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/current/manifest.json)。copilot_use 產品本次未重新驗收；過往產品試作與舊 SHA 的結果只留歷史證據，不在本節重複累積。


## Chat Day23 教學發布（與產品驗收分開）

來源f38360cd537daf6dc20e1d3868221bb56390cc7c。登入切片與Day22補教在隔離副本7項測試通過、SSR check通過；原main library 8通過／6失敗。全文四節與8個程式碼區塊讀回一致，Day22補教僅在Day23內發布，未改封存正文。仍未套用產品；閒置續期、房間授權與真實舊庫升級等缺口見[發布證據](../day23/chat-completion.json)。

## pet Day23 教學發布

[正式文章](https://app.notion.com/p/3ffe51d450938198871cddfdb6f7044a?pvs=204)已全文讀回，四節與2個程式碼區塊一致；來源f38360cd537daf6dc20e1d3868221bb56390cc7c，隔離測試44項通過。產品未套用、未宣稱完整驗收；限制與精確命令見正式文章及[發布證據](../day23/pet-completion.json)。

## analytics Day23 教學發布

[正式文章](https://app.notion.com/p/3ffe51d4509381f88d1be27bc5ec5f9f?pvs=204)已全文讀回，四節與2個程式碼區塊一致；來源f38360cd537daf6dc20e1d3868221bb56390cc7c，隔離測試30項通過。產品未套用、未宣稱完整驗收；限制與精確命令見正式文章及[發布證據](../day23/analytics-completion.json)。

## Day23 封存與本地正式檢驗

2026-10-21使用者確認完成並封存Day23；原文未改，next_day維持24。已對main 8b9d59110349609c940e540ee9d5b42883d52b3e完成獨立正式檢驗，不重套教學patch：workspace122通過／PG28通過、真HTTP與Pet原生GUI重啟通過；嚴格Clippy兩處失敗，房間授權與首次改密碼等缺口保留。已清除隔離DB，copilot_use及既有5432保留。詳見[正式檢驗報告](../evidence/2026-10-21-main-8b9d591/REPORT.md)。

## Day24 完整發布與讀回

Day23（含）以前正文封存。chat next_day=25；pet next_day=25；analytics next_day=25。Day24僅在各案完整文章、程式碼與索引讀回成功後推進；排程維持暫停，未新增排程。

- [chat Day24](https://app.notion.com/p/3ffe51d4509381ee9326ce6cc7f1c8a1?pvs=204)：完整讀回；docs/teaching/day24/chat-readback.json
- [pet Day24](https://app.notion.com/p/3ffe51d450938186b15fd1c014df8edd?pvs=204)：完整讀回；docs/teaching/day24/pet-readback.json
- [analytics Day24](https://app.notion.com/p/3ffe51d4509381c88f26f4a90c2958fc?pvs=204)：完整讀回；docs/teaching/day24/analytics-readback.json

## Day24封存

三案Day24含程式與動畫已依使用者要求封存，正文未改，next_day均維持25。後續Pet動畫單圖像、重複渲染與動態窗權限更正放在Day25待辦，不回寫Day24；封存與產品正式驗收分開。
