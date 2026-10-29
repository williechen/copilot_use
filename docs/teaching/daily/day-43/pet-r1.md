# Pet Day43 r1｜本地實測修訂

2026-10-29；最新產品 `8b882696c5548accb9d98a32b27db35e11109c74`。原文與 patch 基準 `c6708c4…`；[完整原始功能與獨立 Refactor](pet.md) 保留。

最新產品已存在本案 Day43 實作，不重複正向套用原附件；重現文章則在原文基準的隔離 clone，先 Refactor 再 Green。本輪已真正 check／apply 六份原 patch，[完整命令與證據](../../../source-review/local-validation-2026-10-29-day43/README.md)。

| 副本 | 本案命令 | exit |
|---|---|---:|
| day43-candidate | `cargo metadata --locked --no-deps --format-version 1 --manifest-path pet_apps/Cargo.toml` | 0 |
| day43-candidate | `cargo fmt --all --manifest-path pet_apps/Cargo.toml -- --check` | 1 |
| day43-candidate | `cargo check --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 0 |
| day43-candidate | `cargo clippy --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 0 |
| day43-candidate | `cargo test --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 0 |
| day43-candidate | `cargo check --offline --locked --manifest-path pet_apps/Cargo.toml -p pet-ui --target wasm32-unknown-unknown` | 0 |
| day43-candidate | `node --test pet_apps/pet_ui/tests/day33-open-controls-selects-origin.test.mjs pet_apps/pet_ui/tests/duplicate-preview-contract.test.mjs pet_apps/pet_ui/tests/scan-preview-contract.test.mjs pet_apps/pet_ui/tests/day41-cage-by-id.test.mjs pet_apps/pet_ui/tests/scan-preview-no-path.test.mjs pet_apps/pet_ui/tests/day42-cage-status.test.mjs` | 0 |
| day43-latest | `cargo metadata --locked --no-deps --format-version 1 --manifest-path pet_apps/Cargo.toml` | 0 |
| day43-latest | `cargo fmt --all --manifest-path pet_apps/Cargo.toml -- --check` | 1 |
| day43-latest | `cargo check --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 0 |
| day43-latest | `cargo clippy --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 0 |
| day43-latest | `cargo test --offline --locked --workspace --manifest-path pet_apps/Cargo.toml` | 101 |
| day43-latest | `cargo check --offline --locked --manifest-path pet_apps/Cargo.toml -p pet-ui --target wasm32-unknown-unknown` | 0 |
| day43-latest | `node --test pet_apps/pet_ui/tests/day33-open-controls-selects-origin.test.mjs pet_apps/pet_ui/tests/duplicate-preview-contract.test.mjs pet_apps/pet_ui/tests/scan-preview-contract.test.mjs pet_apps/pet_ui/tests/day41-cage-by-id.test.mjs pet_apps/pet_ui/tests/scan-preview-no-path.test.mjs pet_apps/pet_ui/tests/day42-cage-status.test.mjs` | 0 |

最新 workspace 首輪因磁碟 ENOSPC 在連結階段失敗；清理後 -j 2 重試通過。最新 UI fmt 有 import 排序與 macro 內尾端空白問題；[最新 8b88269 基準格式補修](pet-day43-format-fix-r1.patch) 在隔離副本真正 check／apply，修正後四 workspace fmt 通過。Node／WASM／SQLite／native 測試各自列示，未冒稱 GUI 通過。

原文候選格式失敗如實保留；不以套用成功當編譯成功。原生 GUI／瀏覽器／CI／舊 DB 升級與完整產品條件尚未驗收。原文、有效斷言、FIFO 不變；本修訂不增加 Day，next_day 維持 44。
