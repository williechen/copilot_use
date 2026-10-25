# 按功能分層：來源評估與教學規則調整

2026-10-25；文件基準 `copilot_use@3970b17370fa788c4a8df64553c7529b81214aac`；程式來源 `rust_ai_ring@648b90323af7b381d6a79dc2fd08a53b63173a43`。使用者確認範圍：「更新架構規則與後續教學，逐功能提供完整重構」。本次不直接修改／重構／推送產品。

## 實際來源與判定

| 專案 | 已存在的邊界 | 尚未完成的分層 | 精確來源 |
| --- | --- | --- | --- |
| Chat | Application已定義RoomRepository／MessageRepository Port與部分用例，Persistence實作SQLx，UI使用Leptos | signin／logout等server模組仍直接查／寫Session或帳號資料；功能內UI狀態與正式caller仍有已驗出的缺口 | [application](https://github.com/williechen/rust_ai_ring/blob/648b90323af7b381d6a79dc2fd08a53b63173a43/chat_apps/chat_application/src/lib.rs)、[Repository Port](https://github.com/williechen/rust_ai_ring/blob/648b90323af7b381d6a79dc2fd08a53b63173a43/chat_apps/chat_application/src/repository.rs)、[signin](https://github.com/williechen/rust_ai_ring/blob/648b90323af7b381d6a79dc2fd08a53b63173a43/chat_apps/chat_server/src/signin.rs) |
| Pet | Domain已有狀態機，Application已有名稱規則，Persistence已有flag／清單SQL | PetStore仍有多組SQL，開窗／出籠補償流程與協調鎖在server；用例尚未透過儲存／視窗Port隔離driver | [application](https://github.com/williechen/rust_ai_ring/blob/648b90323af7b381d6a79dc2fd08a53b63173a43/pet_apps/pet_application/src/lib.rs)、[Store](https://github.com/williechen/rust_ai_ring/blob/648b90323af7b381d6a79dc2fd08a53b63173a43/pet_apps/pet_server/src/pet_store.rs)、[windows](https://github.com/williechen/rust_ai_ring/blob/648b90323af7b381d6a79dc2fd08a53b63173a43/pet_apps/pet_server/src/pet_windows.rs) |
| Analytics | 正式Leptos畫面、Session與PG查詢已運作，部分聚合SQL在Persistence | application／domain／ui三庫仍為add範本；password_login與pg_store等server模組混合用例與SQL，EventSummary依賴sqlx::FromRow並由外層直接使用 | [application範本](https://github.com/williechen/rust_ai_ring/blob/648b90323af7b381d6a79dc2fd08a53b63173a43/analytics_apps/analytics_application/src/lib.rs)、[登入／Session](https://github.com/williechen/rust_ai_ring/blob/648b90323af7b381d6a79dc2fd08a53b63173a43/analytics_apps/analytics_server/src/password_login.rs)、[聚合Adapter](https://github.com/williechen/rust_ai_ring/blob/648b90323af7b381d6a79dc2fd08a53b63173a43/analytics_apps/analytics_persistence/src/lib.rs) |

因此應記為「分層未貫徹／部分功能仍混層」，不把已存在的Chat Repository與Pet規則一併說成沒有架構，也不因crate名稱存在就稱已完成。測試內SQL fixture與production SQL分開判讀，Composition root的組裝依賴也不是任意跨層的理由。

## 已確認的規則變更

架構責任與依賴只在 [SYSTEM_ARCHITECTURE](../SYSTEM_ARCHITECTURE.md#functional-layering) 維護；每日交付約束在 [COMMON_LEARNING_RULES](../COMMON_LEARNING_RULES.md)，實際教學檢核在 [TEACHING_ALIGNMENT](../TEACHING_ALIGNMENT.md)。按功能在既有層內組織module，重用既有邊界；每次選原功能項的一個最小完整切片，給出所有必要程式／Cargo／imports／Port／DTO／錯誤轉換／組裝／正式caller與測試。不是建立第二套產品，也不是一次機械搬全庫。

例：Pet出籠用例由Application協調儲存與視窗Port，SQLx實作在Persistence、Tauri實作留native Adapter；Chat保留已有效Repository並把所選功能的剩餘SQL／流程接回用例；Analytics依所選登入／查詢功能接通純用例與PG Adapter、適用Leptos元件。這些是後續完整教學的責任落點，**尚未套用或測試的產品變更**，沒有宣稱已完成重構。

## 完成認定與保留

功能可操作／失敗回饋與授權仍優先；Red對應真實缺口，原通過測試保留為回歸；Green同篇涵蓋必要層與正式caller；舊實作在有效測試接回後清理，保留必要相容橋接。檢查依賴／imports只作邊界證據，不取代真DB／GUI／HTTP／IPC驗收。

本次沒有新產品測試、沒有改產品來源／本地Chat房名修正、沒有改Cargo／lock／migration、沒有更新排程設定或建立新排程。原教學正文／commit／blob／SHA256、next_day（各案38）、manifest、FIFO與last_full_comparison維持不變；未做封存。Day37 r3驗證結果保留，分層規則更新不追認其Gate2，也不新增Day。
