# 系統架構與現行程式入口

`copilot_use` 維護規則、教學、進度與證據；`rust_ai_ring` 為唯一程式、測試、CI 與功能驗收主體。現行目錄及名稱見 [SOURCE_LAYOUT](SOURCE_LAYOUT.md)，政策正本見 [共用規則](COMMON_LEARNING_RULES.md)。

本次功能分層評估來源為 `648b90323af7b381d6a79dc2fd08a53b63173a43`，見 [分層來源評估](source-review/2026-10-25-functional-layering.md)。下列入口表的歷史來源為 `5c713553daee4055738239f047d6c774fcc7a7a1`；[最新靜態比對與CI範圍](source-review/2026-10-25-full-comparison-5c713553.md)。本輪Node／metadata／schema探測與同SHA既有實測分開記錄，沒有重跑完整DB／HTTP／GUI。

| 邊界 | 精確來源 | 實際位置與責任 |
| --- | --- | --- |
| Chat HTTP / WS / Session | [chat_apps/chat_server/src/lib.rs](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/chat_apps/chat_server/src/lib.rs) | Axum路由、會員Session與HTTP／WS守衛；逐房ACL尚未完整。 |
| Chat UI | [chat_apps/chat_ui/src/lib.rs](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/chat_apps/chat_ui/src/lib.rs) | Leptos SSR / hydration、Router 與互動。 |
| Chat application / domain | [chat_apps/chat_application/src/lib.rs](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/chat_apps/chat_application/src/lib.rs) | 使用案例與 ports；領域型別位於同 workspace 的 chat_domain。 |
| Chat persistence | [chat_apps/chat_persistence/src/lib.rs](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/chat_apps/chat_persistence/src/lib.rs) | SQLx SQLite adapter；migrations 在 chat_apps/migrations。 |
| Pet UI | [pet_apps/pet_ui/src/main.rs](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/pet_apps/pet_ui/src/main.rs) | Leptos CSR 與 Tauri IPC 呼叫端。 |
| Pet native / windows | [pet_apps/pet_server/src/lib.rs](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/pet_apps/pet_server/src/lib.rs) | Tauri commands、狀態、原生視窗與實際產品呼叫鏈。 |
| Pet storage | [pet_apps/pet_server/src/pet_store.rs](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/pet_apps/pet_server/src/pet_store.rs) | 剩餘SQLx SQLite仍多在server；set_caged／list_uncaged已在pet-persistence並有Store／Tauri caller。migrations在pet_apps/migrations。 |
| Analytics HTTP / UI | [analytics_apps/analytics_server/src/lib.rs](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/analytics_apps/analytics_server/src/lib.rs) | Axum HTTP、Session、Dashboard 與事件入口。 |
| Analytics startup | [analytics_apps/analytics_server/src/main.rs](https://github.com/williechen/rust_ai_ring/blob/5c713553daee4055738239f047d6c774fcc7a7a1/analytics_apps/analytics_server/src/main.rs) | PostgreSQL 與服務啟動；migrations 在 analytics_apps/migrations。 |

<a id="functional-layering"></a>

## 按功能分層的目標與責任

2026-10-25 使用者確認：本次更新規則與後續教學，每次依實際功能提供完整重構，不直接重構產品。本節為責任／依賴的正本，[共用規則](COMMON_LEARNING_RULES.md) 管交付門檻；[本次精確來源評估](source-review/2026-10-25-functional-layering.md) 區分既有邊界與缺口。**目錄已存在不等於分層已完成，部分程式集中server也不等於整庫完全沒有分層。**

| 層／邊界 | 職責 | 不應承擔 |
| --- | --- | --- |
| Leptos UI | 元件、表單、畫面狀態、使用者事件；呼叫既有API／IPC，呈現正確錯誤 | 直接查DB、把前台隱藏／route guard當授權 |
| Delivery：Axum／Tauri | 路由與command、協定驗證、身份上下文、DTO／錯誤轉換；呼叫用例並在伺服器強制授權 | 新增SQL與業務流程塞在handler；把Tauri Window型別傳進Domain |
| Application | 功能使用案例、流程協調、授權決策、交易／補償意圖；定義Repository／Window／其他所需Port與邊界DTO | SQL語句、SQLx Pool／Row、Axum State／Response、Leptos Signals或Tauri AppHandle |
| Domain | 業務規則、值物件、狀態機與不變條件；可獨立測試 | 依賴Application／UI／HTTP／DB／OS實作 |
| Persistence Adapter | 實作儲存Port；SQLx查詢、交易／migration與資料映射；每案DB隔離 | 決定HTTP回應、操作UI／Tauri或讓業務層反向依賴sqlx DTO |
| 原生／外部 Adapter | 實作視窗、檔案、模型／來源通道等Port；封裝Tauri／OS／外部SDK | 把業務決策藏在driver或重新建一套無caller的流程 |
| Composition root | server啟動時建立pool、adapter、用例與state，注入router／commands；組裝Leptos render | 將組裝特例擴散成所有handler可直接跨層寫SQL |

依賴方向：Delivery → Application → Domain；Persistence／原生 Adapter → Application Port（及需要的Domain型別）；UI只透過已確認的API／IPC／server-function邊界。Composition root接合實作，不形成環。測試用fake可替換Port，但必須保留真Adapter與正式caller測試。

### 既有crate內按功能組織

沿用 chat_apps／pet_apps／analytics_apps 既有 *_ui、*_server、*_application、*_domain、*_persistence，不先新增空crate。對選定功能，在相關層內建立清楚命名的module，例如房間查詢、寵物生命週期、統計查詢授權；具體檔案樹依當次來源與切片決定，不把下例當已存在路徑或全庫搬移命令。

同一功能的必要調整須在同篇接通：UI事件 → 正式HTTP／IPC → 用例 → Port → 真Adapter → 回應／畫面。新型別或目錄存在、re-export成功、cargo check成功皆不足以認定功能重構完成。若當日只涉及UI狀態，重用既有用例／Adapter即可，不硬造Domain wrapper。

### 三案後續調整落點

| 功能 | 已存在且保留 | 功能切片內需要補齊的邊界 |
| --- | --- | --- |
| Chat 房間／訊息與人員流程 | chat-application Repository Port與部分用例；chat-persistence SQLx；chat-domain與Leptos UI | 依所選功能將server中剩餘認證／Session SQL與業務流程移到適當用例／Adapter；保留房間ID、名稱、API與WS契約。房間查詢畫面缺口仍須真UI驗證。 |
| Pet 建立／出入籠／選取 | pet-domain狀態機；application名稱規則；persistence已移出的flag／清單查詢 | 選定生命週期用例後，將Store剩餘SQL與Tauri command內流程分開；Application透過儲存／視窗Port協調，SQLite與Tauri各實作Adapter，涵蓋Runtime、鎖、補償與真IPC呼叫者。 |
| Analytics 登入授權／統計查詢 | 正式password_login／Leptos畫面、PG查詢與部分persistence聚合 | application/domain/ui目前範本不當完成；依功能接通純用例／授權政策、Session／統計Port、PG Adapter及Leptos元件；server保留路由／middleware與組裝，不再承擔所有層。 |

這是功能優先的重構方向，非固定Day排程或新增平行待辦。可操作基本流程與已知錯誤優先，在同功能修補時完成必要分層，不以架構整理推遲現有錯誤回饋、資料一致性與授權問題。跨系統共用登入邏輯仍沿原需求設Port／政策，Chat／Analytics各自儲存與Session隔離，不混表或因抽象共用而共享登入。

## 原有分層要求與來源說明

- Leptos 負責頁面與互動；Chat SSR / hydration、Pet CSR + Tauri、Analytics UI 依實際接線核對。
- Axum 負責 HTTP / WebSocket、routing / middleware、Cookie / Session 與服務端 ACL。
- Application / Domain 負責使用案例與規則；SQLx adapter 處理資料庫查詢、交易與 migrations。
- pet-application已共用名稱規則；pet-persistence承接set_caged／list_uncaged，analytics-persistence承接精確事件類型SQLx聚合，均有真caller。Analytics application/domain/ui仍是範本，其餘SQL多仍在server。後續教學依最新來源進行必要的小切片搬移、介面調整與呼叫端／測試接線，不能以目錄存在代替實作完成。
- 共用 crates 的 shared 目前依賴 chat-domain，仍有跨 workspace 的 package path dependency；不得假設三案已完全解耦。

[使用者流程](requirements/USER_FLOWS.md) · [後續缺口](FUTURE_WORK.md) · [教學銜接](TEACHING_ALIGNMENT.md) · [本地操作](LOCAL_START.md) · [驗證](LOCAL_VALIDATION.md)
