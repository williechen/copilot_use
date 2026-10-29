# Chat Day43 r1｜本地實測修訂

2026-10-29；最新產品 `8b882696c5548accb9d98a32b27db35e11109c74`。原文與 patch 基準 `c6708c4…`；[完整原始功能與獨立 Refactor](chat.md) 保留。

最新產品已存在本案 Day43 實作，不重複正向套用原附件；重現文章則在原文基準的隔離 clone，先 Refactor 再 Green。本輪已真正 check／apply 六份原 patch，[完整命令與證據](../../../source-review/local-validation-2026-10-29-day43/README.md)。

| 副本 | 本案命令 | exit |
|---|---|---:|
| day43-candidate | `cargo metadata --locked --no-deps --format-version 1 --manifest-path chat_apps/Cargo.toml` | 0 |
| day43-candidate | `cargo fmt --all --manifest-path chat_apps/Cargo.toml -- --check` | 1 |
| day43-candidate | `cargo check --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 0 |
| day43-candidate | `cargo clippy --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 0 |
| day43-candidate | `cargo test --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 0 |
| day43-candidate | `cargo test --offline --locked --manifest-path chat_apps/Cargo.toml -p chat-server -p chat-persistence -p chat-ui --features ssr` | 101 |
| day43-candidate | `cargo check --offline --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --target wasm32-unknown-unknown --features hydrate` | 0 |
| day43-latest | `cargo metadata --locked --no-deps --format-version 1 --manifest-path chat_apps/Cargo.toml` | 0 |
| day43-latest | `cargo fmt --all --manifest-path chat_apps/Cargo.toml -- --check` | 0 |
| day43-latest | `cargo check --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 0 |
| day43-latest | `cargo clippy --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 0 |
| day43-latest | `cargo test --offline --locked --workspace --manifest-path chat_apps/Cargo.toml` | 0 |
| day43-latest | `cargo test --offline --locked --manifest-path chat_apps/Cargo.toml -p chat-server -p chat-persistence -p chat-ui --features ssr` | 101 |
| day43-latest | `cargo check --offline --locked --manifest-path chat_apps/Cargo.toml -p chat-ui --target wasm32-unknown-unknown --features hydrate` | 0 |

發現：原文候選與最新產品均有一項清除搜尋 SSR 測試在 matched Route 外渲染 RoomList。保留清除按鈕與 aria-label 斷言，補 Routes／Route。兩個修補候選 UI 均 21 項通過。原文順序：Refactor → Green → [原文基準 fixture 補修](chat-day43-candidate-fix-r1.patch)；最新 8b88269 副本用 [最新基準補修](chat-day43-latest-fix-r1.patch)，不重套 Green。

原文候選格式失敗如實保留；不以套用成功當編譯成功。原生 GUI／瀏覽器／CI／舊 DB 升級與完整產品條件尚未驗收。原文、有效斷言、FIFO 不變；本修訂不增加 Day，next_day 維持 44。
