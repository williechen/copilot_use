# 現行程式來源與目錄對照

**歷史核對（2026-10-26，非最新 main）：** 最新程式來源 `9b89656`（upgrade）；新CI快照為1成功／6執行中，尚未全綠。舊648b903為六成功／Persistence失敗，已配置的Analytics PostgreSQL步驟被略過。當日官方版本、manifest／lock與本機工具差異已補查；未升級產品、未重跑測試、未完成整體驗收。[版本與摘要核對](source-review/2026-10-26-versions-and-summary-refresh.md)。

**目前目錄基準：** rust_ai_ring `912e5e77d8e3d2e24f0e9d804f28d86ae768c1d4`（2026-10-27 本地更新後；四個 workspace 的 tracked tree、Cargo.toml／Cargo.lock、18 個 member 與 entrypoints 核對）。下方較舊的來源 SHA、CI 與測試結果均是歷史快照，不代表最新產品驗收。

2026-10-24：使用者已將 `williechen/rust_ai_chat` 改名為 [`williechen/rust_ai_ring`](https://github.com/williechen/rust_ai_ring)，並調整為四個 Cargo workspace。2026-10-25 更新來源為 `5c713553daee4055738239f047d6c774fcc7a7a1`；文件基準為 copilot_use `532e739f1ef4e5920de300eba0f90adeeaf96187`。後續教學每次仍須重新讀取最新 main，不固定使用本頁 SHA。

## GitHub 現行 workspace 目錄樹（2026-10-27）

依 [rust_ai_ring 固定來源](https://github.com/williechen/rust_ai_ring/tree/912e5e77d8e3d2e24f0e9d804f28d86ae768c1d4) 的四份 workspace `Cargo.toml` 成員宣告核對；來源 commit `912e5e77d8e3d2e24f0e9d804f28d86ae768c1d4`。以下是**已確認的 Cargo workspace／member 目錄樹**，不是遞迴列出所有檔案；其他子目錄與檔案仍應以 GitHub 該 SHA 的實際 tree 為準。

```text
rust_ai_ring/
├── chat_apps/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── chat_server/
│   ├── chat_application/
│   ├── chat_domain/
│   ├── chat_persistence/
│   └── chat_ui/
├── pet_apps/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── pet_server/
│   ├── pet_ui/
│   ├── pet_domain/
│   ├── pet_application/
│   └── pet_persistence/
├── analytics_apps/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── analytics_server/
│   ├── analytics_persistence/
│   ├── analytics_ui/
│   ├── analytics_domain/
│   └── analytics_application/
└── crates/
    ├── Cargo.toml
    ├── Cargo.lock
    ├── ai_core/
    ├── shared/
    └── telemetry/
```

四份 manifest 的 workspace 成員分別為 Chat 5、Pet 5、Analytics 5、共用 crates 3；Chat／Pet／Analytics workspace package version 為 `0.2.37`，共用 crates 為 `0.2.35`。Leptos metadata 的 `bin-package = "chat-server"`、`lib-package = "chat-ui"` 與輸出名稱 `rust_ai_chat` 仍應保留，不能因 repo 改名直接更動外部契約。

**本地核對範圍：** 四份 Cargo.lock 已解析，18 個 workspace package 的版本與 manifest 一致；tracked tree、各套件入口及測試來源已讀回。四組 `cargo metadata --locked --no-deps --format-version 1` 通過；此命令不驗證完整依賴解析或編譯。目錄與 package 表不代表所有 library 已接入正式 caller，也不代表產品驗收；測試結果須按本次固定 SHA 另列，舊版歷史證據不追認。

## 兩個目錄的用途

- `/home/willie/Documents/rust_ai_ring`：唯一產品程式、測試、CI 與功能驗收來源。
- `/home/willie/Documents/copilot_use/docs`：需求、現行規則、後續教學、進度、索引與原始證據。
- copilot_use 既有 apps、crates、scripts、workflow、vendor 與 Release 是歷史程式來源；本次目錄整理集中於現行文件入口與程式路徑對照。

## 四個 workspace

下列 workspace 成員與 package 名稱已按 2026-10-27 的四份 Cargo manifest 重讀；歷史測試仍維持原 SHA。

根目錄沒有 Cargo.toml；指令須在對應目錄執行，或指定 `--manifest-path`。每個 workspace 各自保留 Cargo.toml、Cargo.lock、target。

| Workspace | 成員數 | Manifest |
| --- | ---: | --- |
| Chat | 5 | [chat_apps/Cargo.toml](https://github.com/williechen/rust_ai_ring/blob/912e5e77d8e3d2e24f0e9d804f28d86ae768c1d4/chat_apps/Cargo.toml) |
| Pet | 5 | [pet_apps/Cargo.toml](https://github.com/williechen/rust_ai_ring/blob/912e5e77d8e3d2e24f0e9d804f28d86ae768c1d4/pet_apps/Cargo.toml) |
| Analytics | 5 | [analytics_apps/Cargo.toml](https://github.com/williechen/rust_ai_ring/blob/912e5e77d8e3d2e24f0e9d804f28d86ae768c1d4/analytics_apps/Cargo.toml) |
| 共用 crates | 3 | [crates/Cargo.toml](https://github.com/williechen/rust_ai_ring/blob/912e5e77d8e3d2e24f0e9d804f28d86ae768c1d4/crates/Cargo.toml) |

| 程式目錄 | Cargo package／`-p` | Rust crate／import |
| --- | --- | --- |
| `chat_apps/chat_server` | `chat-server` | `chat_server` |
| `chat_apps/chat_application` | `chat-application` | `chat_application` |
| `chat_apps/chat_domain` | `chat-domain` | `chat_domain` |
| `chat_apps/chat_persistence` | `chat-persistence` | `chat_persistence` |
| `chat_apps/chat_ui` | `chat-ui` | `chat_ui` |
| `pet_apps/pet_server` | `pet-server` | `pet_server` |
| `pet_apps/pet_ui` | `pet-ui` | `pet_ui` |
| `pet_apps/pet_domain` | `pet-domain` | `pet_domain` |
| `pet_apps/pet_application` | `pet-application` | `pet_application` |
| `pet_apps/pet_persistence` | `pet-persistence` | `pet_persistence` |
| `analytics_apps/analytics_server` | `analytics-server` | `analytics_server` |
| `analytics_apps/analytics_persistence` | `analytics-persistence` | `analytics_persistence` |
| `analytics_apps/analytics_application` | `analytics-application` | `analytics_application` |
| `analytics_apps/analytics_domain` | `analytics-domain` | `analytics_domain` |
| `analytics_apps/analytics_ui` | `analytics-ui` | `analytics_ui` |
| `crates/ai_core` | `ai-core` | `ai_core` |
| `crates/shared` | `shared` | `shared` |
| `crates/telemetry` | `telemetry` | `telemetry` |

Leptos `bin-package = "chat-server"`、`lib-package = "chat-ui"` 使用 package 名稱；Rust imports 使用 chat_server、chat_ui、chat_persistence、pet_server、analytics_server。檔案目錄使用底線，例如 pet_apps/pet_ui。Leptos 的資源輸出名稱仍為 rust_ai_chat，資料庫名稱、protocol 欄位與既有外部契約須按原始碼核對，不能隨 repository 更名做全域替換。

## 舊路徑對照

| 舊路徑 | 現行路徑 |
| --- | --- |
| `apps/server` | `chat_apps/chat_server` |
| `crates/ui` | `chat_apps/chat_ui` |
| `crates/chat_application` | `chat_apps/chat_application` |
| `crates/chat_domain` | `chat_apps/chat_domain` |
| `crates/persistence` | `chat_apps/chat_persistence` |
| `apps/desktop_pet` | `pet_apps/pet_server` |
| `apps/desktop_pet/frontend` | `pet_apps/pet_ui` |
| `crates/pet_domain` | `pet_apps/pet_domain` |
| `apps/stats_server` | `analytics_apps/analytics_server` |
| `migrations` | `chat_apps/migrations` |
| `apps/stats_server/migrations` | `analytics_apps/migrations` |
| `apps/desktop_pet/migrations` | `pet_apps/migrations` |
| `chat_apps/server` | `chat_apps/chat_server` |
| `chat_apps/ui` | `chat_apps/chat_ui` |
| `chat_apps/persistence` | `chat_apps/chat_persistence` |
| `pet_apps/desktop_pet` | `pet_apps/pet_server` |
| `pet_apps/frontend` | `pet_apps/pet_ui` |
| `analytics_apps/stats_server` | `analytics_apps/analytics_server` |

Chat / Pet SQLite 與 Analytics PostgreSQL 的 migrations 分別位於 chat_apps/migrations、pet_apps/migrations、analytics_apps/migrations。各產品 SQLx 路徑依所屬 package 計算；include_str! 則依來源檔位置計算，兩者不能只套用同一層數。

## 教學與工程的目錄用途

每日教學依當時真實目錄、package、features、imports與正式caller交付當次功能，沿用現行分層；不把跨功能搬層、批次測試遷移或CI重整當每日主題。

全專案架構／測試／CI調整依[工程範圍](teaching/functional-refactoring/README.md)安排獨立工作包，涵蓋三案與共用crates。每個工作包完整處理實作、Cargo、組裝、caller、測試映射與CI，不以目錄或空library代替完成。工程與教學交付分別記錄，均保留精確來源及既有文章證據。

## 文件入口與驗證範圍

[架構](SYSTEM_ARCHITECTURE.md) · [本地啟動](LOCAL_START.md) · [驗證命令](LOCAL_VALIDATION.md) · [後續教學規則](COMMON_LEARNING_RULES.md) · [教學銜接](TEACHING_ALIGNMENT.md) · [目前狀態](teaching/current/STATUS.md) · [機器可讀目錄](source-review/rust-ai-ring-layout.json) · [本次文件核對](source-review/rust-ai-ring-layout-validation.json)

本輪四workspace metadata、Node source6項與Pet記憶體SQLite探測通過；該5c713553快照七個CI成功但server PG feature未完整選取；648b903已配置正式server PG，但被Pet步驟失敗阻擋。Rust／PG／HTTP引用同SHA既有證據，非本輪重跑；GUI／完整驗收未執行。[最新比對](source-review/2026-10-25-full-comparison-5c713553.md)明列範圍，舊目錄核對JSON仍限其原日期。
