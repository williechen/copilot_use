# Chat Day43 r3｜最新產品 SSR 缺少 Router imports

2026-10-30，來源 `d2612fcd154d2132d850086d83a48578abbb07fa`。[完整原功能與獨立 Refactor](chat.md) · [先前瀏覽器修補 r2](chat-r2.md)。

最新產品已接入 `refresh_rooms`，讀取與清除沿正式 list_rooms API 重新取得完整快照；不再把「清除回歸修補尚未套用」當現況。但新 SSR fixture 只加 Route／Routes 用法，漏了 imports，因此本地與遠端 test-ui 都在編譯階段失敗，尚未跑到斷言；這不是行為 Red。

最小修正只替換 `chat_apps/chat_ui/src/signin_contract.rs` 的 router import：

```rust
use leptos_router::{
    components::{Route, Router, Routes},
    location::RequestUrl,
    path,
};
```

[完整可套用補丁](chat-ssr-imports-d2612fc.patch) 以 d2612fc 為基準，已在隔離 clone 真正 check／apply；不與先前 8b88269 的整份清除搜尋補丁重複套用。原有 Router／Route fixture 與全部斷言保留，不混入其他專案。

| 本輪驗證 | 結果 |
| --- | --- |
| metadata／fmt／workspace check／clippy | exit 0 |
| 一般 workspace tests | 60 passed，未覆蓋 SSR fixture |
| 原樣 SSR tests | exit 101：缺 Route／Routes／path |
| hydrate WASM check | exit 0 |
| 隔離 import 補丁後 `cargo test --offline --locked -p chat-ui --features ssr` | 21 passed，exit 0 |
| 補丁後 fmt | exit 0 |

[完整命令、CI 原始 log、DB 與清理證據](../../../source-review/local-validation-2026-10-30-day43/README.md)。本輪有資料舊 SQLite schema 升級／保資料／冪等／checksum 拒絕測試通過；未重跑真瀏覽器或使用者舊庫。產品仍唯讀，補丁未提交／推送；遠端 test-ui 仍失敗，不以隔離成功追認產品全綠。版本與 lock 未升級，原版本說明保留交付日期，未重作完整官方版本／MSRV Gate。next_day 44、FIFO、last_full_comparison 不變。
