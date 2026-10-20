## 目前進度（2026-10-20｜增量核對）
三專案 Day22（含）以前文章均已封存；[Chat Day23](https://app.notion.com/p/3ffe51d4509381cf8532fb767afc33ab?pvs=204)已正式發布並全文讀回，Chat next_day=24；Pet／Analytics next_day=23，Day23尚未發布。舊文更正只在下一 Day 的新 Red／完整 Green／Refactor 後追加，依各專案待補佇列處理。
先前產品狀態快照的 rust_ai_chat/main 為 `cabacb75e946b03c67c455ce13a7a4ea85652f8e`（workspace 0.2.22）。以下是該 SHA 的最近一次實測結果；本次僅整理文件，沒有重跑測試或進行新的全面複驗。
- **Chat：** 註冊表單→HTTP→SQLite 已有實作；13 項測試通過、1 項失敗，失敗為短帳號驗證實際回 409／預期 422。隔離修正候選 19 項通過，未套用 main。登入、Session 與聊天室授權主線仍待接通。
- **Pet：** 控制台→Tauri IPC→SQLite 建立並讀回寵物已接線；42 項測試通過。逐寵物生活窗身分、初始外觀及重啟載入仍待完成。
- **Analytics：** PostgreSQL 管理員初始化與不覆寫密碼已實作；真 PostgreSQL 測試 26 項通過。現有後台仍是舊 key 登入，帳密 Session、首次改密碼與角色授權尚未接通。
**共同待修：** Chat／Pet 從舊庫升級會遇到已修改 migration 的 checksum 阻擋；fresh DB 通過不代表升級相容。需先核對實際 DB 歷史，不改 checksum 或刪庫掩蓋。Chat／Pet WASM check 通過，但瀏覽器／Tauri GUI、重啟、MSRV／跨平台及完整產品驗收尚未完成。
**套件與 CI：** 最近一次官方版本核對的 28 個直接依賴均包含最新穩定版，主要版本為 Argon2 0.6.0、Tokio 1.54.0；未更新來源 lock。[同 SHA 的 Test All](https://github.com/williechen/rust_ai_chat/actions/runs/39404616558) 仍失敗於 test-server，其他五個 workflow 成功；不以 CI 成功代替未涵蓋的功能驗收。
[完整來源／測試／套件證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-20-day22-freeze/REPORT.md) · [現行進度索引](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/current/manifest.json)。copilot_use 產品本次未重新驗收；過往產品試作與舊 SHA 的結果只留歷史證據，不在本節重複累積。


## Chat Day23 教學發布（與產品驗收分開）

來源f38360cd537daf6dc20e1d3868221bb56390cc7c。登入切片與Day22補教在隔離副本7項測試通過、SSR check通過；原main library 8通過／6失敗。全文四節與8個程式碼區塊讀回一致，Day22補教僅在Day23內發布，未改封存正文。仍未套用產品；閒置續期、房間授權與真實舊庫升級等缺口見[發布證據](../day23/chat-completion.json)。
