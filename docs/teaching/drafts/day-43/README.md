# Day43｜2026-10-29 三案新教學候選（未發布 Gate1）

**狀態：候選草稿，不是已發布 Day43；三案 `next_day` 仍為 42。** 這次不再重寫 Day42 原文，而是在不冒進正式文件 Gate 的前提下交付三案各自的新功能切片、獨立 Refactor 與完整候選 patch。產品唯一來源 [rust_ai_ring@c6708c4](https://github.com/williechen/rust_ai_ring/tree/c6708c42f21ac919e57a379848b4977d27a1c3d6)；僅寫入 copilot_use/main，產品唯讀。

| 專案 | 新功能 Red／Green | 自身獨立 Refactor | 文章與 patch |
|---|---|---|---|
| Chat | 房間搜尋一鍵清除，恢復已載入清單 | Session 入口狀態決策移至 `session_entry.rs` | [文章](chat.md) · [Refactor](chat-day43-refactor.patch) · [Green](chat-day43-green.patch) |
| Pet | 控制台逐 UUID 顯示回籠狀態與錯誤 | 原生選單 UUID 解析移至 `menu_scope.rs` | [文章](pet.md) · [Refactor](pet-day43-refactor.patch) · [Green](pet-day43-green.patch) |
| Analytics | 切換 Chat／Pet 來源時保留精確事件類型 | Dashboard 計數標籤決策移至 `dashboard_summary.rs` | [文章](analytics.md) · [Refactor](analytics-day43-refactor.patch) · [Green](analytics-day43-green.patch) |

## 本次接續核對（2026-10-29）

[Gate1 增量核對、六份 patch 精確 hunk 重播、官方版本／MSRV 補查與環境限制](gate1-followup-2026-10-29.md)：六份 patch 按 Refactor → Green 均與固定產品來源吻合；已更正 Pet Tauri CLI 穩定版為 2.12.4。**目前仍沒有原生 git apply、Cargo／本地相容性實測，因此維持候選狀態，不提前同步 next_day。**

## 已有證據與待補

- 已讀回兩庫來源 SHA、規則、需求、架構、狀態、manifest、Day42 三案及正式 caller；核對四個 workspace 與既有 lock 版本。
- 已為三案各建立獨立文件與 patch 提交，精確 commit 全文讀回，記錄 blob/SHA256；[候選證據 JSON](candidate-evidence.json)。
- 已在 JavaScript 中按原始檔行與修改片段重建 Refactor → Green 的 unified diff，確認目標修改單元與新模組不存在於來源 SHA；**不是** `git apply --check` 或 Cargo 編譯。
- 本次執行環境無 `cargo`／`rustc`，沒有本地 rust_ai_ring checkout，也不能直接存取 GitHub Git 遠端。未實際執行 Red、Green、Cargo、Node、SQLite、PostgreSQL、SSR、WASM、Tauri GUI 或 CI。原 Day42 的 2026-10-28 實測證據仍只屬原 SHA，不能追認新候選。
- 官方 Rust、主要依賴已查核；遞迴 MSRV、CLI、native／DB 相容與正式 patch 套用仍待完成，因此 **Gate1 未通過，Gate2 未執行**。不可把這批草稿移至 `docs/teaching/daily/day-43` 或推進 `next_day`，直到依 [共用規則](../../../COMMON_LEARNING_RULES.md) 補齊 Gate1、每案正式 commit 讀回及條件同步。

## 下一步

在隔離的 `rust_ai_ring@c6708c4` 工作樹，按每案文章列出的 Refactor → Green 順序實際 `git apply --check`／套用，執行對應 Cargo／Node／SSR／WASM／PG 測試，核對全遞迴版本與 MSRV；發現編譯或 patch 錯誤時只修訂候選，不改已發布 Day42。文件 Gate1 合格後才正式發布 Day43、更新七首頁／ENTRY／manifest／learning-state。產品套用、功能驗收及 GUI/CI 獨立列 Gate2，不得以此阻擋已合格的文件發布。

全專案獨立重構仍由 [docs/engineering/README.md](../../../engineering/README.md) 追蹤，本次沒有新增跨案工作包或重複計功；FIFO 與 `last_full_comparison` 不變。
