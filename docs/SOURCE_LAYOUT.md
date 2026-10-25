# 現行程式來源與目錄對照

2026-10-24：使用者已將 `williechen/rust_ai_chat` 改名為 [`williechen/rust_ai_ring`](https://github.com/williechen/rust_ai_ring)，並調整為四個 Cargo workspace。2026-10-25 更新來源為 `e753c36294ed6a7781830e26c5c71f2bdc233eca`；文件基準為 copilot_use `9da2a51d8b10b23ba5a14be7d8fbb2eb6cc231c6`。後續教學每次仍須重新讀取最新 main，不固定使用本頁 SHA。

## 兩個目錄的用途

- `/home/willie/Documents/rust_ai_ring`：唯一產品程式、測試、CI 與功能驗收來源。
- `/home/willie/Documents/copilot_use/docs`：需求、現行規則、後續教學、進度、索引與原始證據。
- copilot_use 既有 apps、crates、scripts、workflow、vendor 與 Release 是歷史程式來源；本次目錄整理集中於現行文件入口與程式路徑對照。

## 四個 workspace

根目錄沒有 Cargo.toml；指令須在對應目錄執行，或指定 `--manifest-path`。每個 workspace 各自保留 Cargo.toml、Cargo.lock、target。

| Workspace | 成員數 | Manifest |
| --- | ---: | --- |
| Chat | 5 | [chat_apps/Cargo.toml](https://github.com/williechen/rust_ai_ring/blob/e753c36294ed6a7781830e26c5c71f2bdc233eca/chat_apps/Cargo.toml) |
| Pet | 5 | [pet_apps/Cargo.toml](https://github.com/williechen/rust_ai_ring/blob/e753c36294ed6a7781830e26c5c71f2bdc233eca/pet_apps/Cargo.toml) |
| Analytics | 5 | [analytics_apps/Cargo.toml](https://github.com/williechen/rust_ai_ring/blob/e753c36294ed6a7781830e26c5c71f2bdc233eca/analytics_apps/Cargo.toml) |
| 共用 crates | 3 | [crates/Cargo.toml](https://github.com/williechen/rust_ai_ring/blob/e753c36294ed6a7781830e26c5c71f2bdc233eca/crates/Cargo.toml) |

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

## 後續教學如何調整程式

使用者最新要求：後續教學應依新目錄調整程式。每篇先核對最新來源、上一篇及實際呼叫鏈，再在教學中給出需要調整的檔案、完整程式碼、imports、Cargo dependency / feature、呼叫端與測試位置，最後提供對應 workspace 的驗證命令。

pet-application 已共用名稱規則；pet-persistence 已承接 set_caged，analytics-persistence 已承接精確事件類型查詢與 EventSummary；均有 caller。其餘 PetStore／pg_store／bootstrap 程式仍多在 server，Analytics application/domain/ui 仍是範本。後續若把使用案例或 DB adapter 移入新層，需在同一最小切片中接回真實呼叫端與測試，不能只把空 crate 加進 members 就宣稱分層完成。每篇僅處理當日必要範圍，不要求每層每天都修改。

copilot_use 保存教學程式碼與交付；產品程式調整以 rust_ai_ring 為主體，由使用者套用。已發 Day 正文、來源 SHA、commit/blob/SHA256 與歷史證據保留；新的改名／搬移修正在後續教學呈現。每案 next_day 由即時狀態讀取，本次不推進進度。

## 文件入口與驗證範圍

[架構](SYSTEM_ARCHITECTURE.md) · [本地啟動](LOCAL_START.md) · [驗證命令](LOCAL_VALIDATION.md) · [後續教學規則](COMMON_LEARNING_RULES.md) · [教學銜接](TEACHING_ALIGNMENT.md) · [目前狀態](teaching/current/STATUS.md) · [機器可讀目錄](source-review/rust-ai-ring-layout.json) · [本次文件核對](source-review/rust-ai-ring-layout-validation.json)

本次 metadata 與增量測試、隔離 SQLite／PostgreSQL、HTTP Router 已執行，最新結果見 [e753c36 驗證](source-review/2026-10-25-update-e753c36.md)。Analytics 測試仍紅；GUI／完整驗收未執行。舊目錄核對 JSON 保留其原日期範圍，不能當新來源成功證據。
