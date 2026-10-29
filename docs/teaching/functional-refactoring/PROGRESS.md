# 各案 Refactor 現況

2026-10-30 三案 Day44 **文件 Gate1 已發布**；Refactor 均為與 Red／Green 並列的**同案獨立 patch**，沒有套用 `rust_ai_ring`，不算 Gate2 或 G-01/G-02 完成。

| 專案 | Day44 同案 Refactor | 有效斷言承接 | 後續同案工作 |
| --- | --- | --- | --- |
| Chat | [房間路徑 canonical parser](../daily/day-44/chat.md#refactorchat-專案房間路徑正規化與測試責任)；`chat_server::room_path` | Day41 匿名回跳、Day25 Session、SSR/WS 舊測試不刪；新 parser unit | Session／Repository 錯誤分層、Chat 專案 fixture |
| Pet | [Native 找回寵物幾何搬層](../daily/day-44/pet.md#refactorpet-原生找回寵物位置計算搬層)；`pet_server::window_position` | Day39 負座標與工作區、Day24 A/B、Day40 window label 舊測試不刪；新幾何 unit | Native lifecycle failure fixture、Pet 專案 CI |
| Analytics | [CSV URL 從 Leptos 搬至純模組](../daily/day-44/analytics.md#refactoranalytics-csv-url-決策從-ssr-元件搬到同案純模組)；`dashboard_url` | Day40 CSV SSR、Day42 kind、Day43 source URL 舊測試不刪；新 URL unit | 同案 Session SQL、legacy admin 路由與 PG fixture |

本次來源 `rust_ai_ring@7ce39f0166a2120539e5908b33c88322a930c35a`。原 D01–D09／13 項清理識別與 FIFO 見 [FUTURE_WORK](../../FUTURE_WORK.md)。跨案契約、共用 crates、跨 workspace 相依與全庫 CI **只在 [engineering](../../engineering/README.md)** 管理，不混入上述 Refactor 或重複計功。
