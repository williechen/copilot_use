## 1. 學習大綱
本輪main `fa0888ea1e1ea93415a4ba7f16972938969b1be3`，Day18仍是寵物生活窗漫遊切片。63f66e7已改SqlitePool，獨立apps/desktop_pet/migrations三份已真實套用；主鍵NOT NULL與關聯約束已修，不再說NULL主鍵仍可插入。普通啟動與四案原生座標通過；main維持(100,100)，停用pet不動，邊界反彈，關閉pet不回退移動main。探針等待映射，不代表完整手動GUI／普通啟動漫遊時序通過。舊Chat DB仍因已套用Pet migration被移走而升級失敗。
目標為只移動pet生活窗、main控制窗不動、工作區邊界、啟動停用與關閉pet。原學習15分鐘；原生探針重現另估20分鐘，不擴大為所有多寵物／檔案／UI功能通過。
本次fa0888e新增chat_action_log：Chat六份migrations／20張業務表，Pet三份／9張，Analytics三份／15張。P0的table與field可依功能需求增減或修改，現有schema不是固定限制。新建及9a95a10→本版升級通過；action log缺少或NULL主鍵仍能插入、任意操作者ID可接受，且無Rust寫入呼叫端。候選以前向migration加NOT NULL，保留有效舊row，遇NULL舊row停止且保留原資料；操作者身分綁定、低敏稽核與真實寫入仍未完成。舊Chat91733／舊PG31升級仍失敗；本輪未操作使用者資料庫。
## 2. 實作內容
### Red歷史與現在的程式
98f30d8尚未選pet；目前apps/desktop_pet/src/[roam.rs](http://roam.rs)已使用roaming_window_label()，tauri.conf.json的main為控制台、pet為200×200生活窗。open_control_window仍操作main，這是正確分工，不再稱錯選main。
先前eb2a233使用PgPool但PET_DATABASE_URL為SQLite的啟動錯誤已由63f66e7修正。最新入口完整程式為：
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    // 啟動時 fail-closed；不得把密鑰值寫入日誌。
    let database_url = std::env::var("PET_DATABASE_URL")?;

    let db = sqlx::SqlitePool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&db).await?;

    desktop_pet::application();

    Ok(())
}
```
SQLx路徑相對於apps/desktop_pet的Cargo package，所以`./migrations`是Pet自己的三份，並非Chat root。PET_DATABASE_URL提供存在的SQLite檔或測試專用`?mode=rwc`；不得改用正式DB作驗證。
### 已套用的最小 Green
[以下程式已在main的roam.rs](http://以下程式已在main的roam.rs)，不再新增同名符號：
```rust
const PET_WINDOW_LABEL: &str = "pet";
fn roaming_window_label() -> &'static str {
    PET_WINDOW_LABEL
}
```
呼叫鏈為main資料庫連線／遷移 → application/setup → PET_ENABLE_ROAM → roam_desktop → pet WebviewWindow → outer_position／outer_size／current_monitor → bounded_next_position → set_position。main只供控制台顯示／聚焦，缺pet即停止漫遊。
### 原生驗證與 Refactor
候選 `2f60c731ca78c689cb78abbd7bea239146ed3aaa` 僅為Pet加上opt-in acceptance-probe，不改原漫遊核心。先套用[以本次main為基底的完整patch](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-full-fa0888e/functional-fixes.patch)，再於隔離資料夾與測試DB執行；main本身尚無這個feature，不能在未套patch時假稱可執行：
```bash
cargo build --locked -p desktop-pet --features acceptance-probe,tauri/custom-protocol
PET_DATABASE_URL=sqlite:///absolute/fixture-pet.db?mode=rwc PET_ACCEPTANCE_SCENARIO=enabled PET_ENABLE_ROAM=true LIBGL_ALWAYS_SOFTWARE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 target/debug/desktop-pet
```
再依disabled／boundary／close重跑，disabled須PET_ENABLE_ROAM=false，其餘true。探針用真實OS位置、大小與工作區，等待視窗映射後才定位與啟動原漫遊函式。它驗證移動目標與幾何，沒有測普通啟動的漫遊時序、手動拖曳或執行中設定。
### 舊Day及資料庫更正
目前Pet九張業務表已獨立，id與必要關聯皆明確NOT NULL，33案schema測試含Pet與Chat共通檢查均符合預期，無效主鍵／外鍵確實被拒絕。主鍵是文字ID，由呼叫端產生，不再要求自增整數。資料表存在與啟動遷移不能冒充PetMachine已持久化、CRUD／多寵物UI已接通。
Chat舊DB記錄過91733／95626，移走原migration後仍missing applied migration；需保留既有Chat遷移歷史並另做資料搬移／新Pet DB回歸，不把刪DB重建當升級通過。候選未實作這項兼容。
## 3. 套件版本
本輪實際 Rust／Cargo 1.99.1；SQLx 0.9.0、Axum 0.8.9、Leptos 0.8.22、Tokio 1.53.3、Tauri 2.12.4、tauri-build 2.7.3、wasm-bindgen 0.2.129（Cargo.lock）。cargo-leptos 0.3.11、Trunk 0.21.14；Ubuntu 26.04.1、PostgreSQL 18.6。本輪WebSocket測試用Node 22.22.1，先前Node24為歷史環境；未另驗跨平台／MSRV。候選沒有新增產品依賴。 Cargo套件版0.2.17；tauri.conf.json產品版仍0.2.16，bundle.active=false，建置沒有產生安裝包。
## 4. 驗證項目
完整主線workspace126 passed／1 failed，失敗在Chat SSR，Pet原生16單元＋18所屬整合通過。Pet release前端`NO_COLOR=true trunk build --locked --release`、原生`cargo build --locked -p stats-server -p desktop-pet --features tauri/custom-protocol`建置通過。普通啟動使用專用SQLite，三份migration套用、完整性ok、程序持續運行。
原生enabled／disabled／boundary／close四案本輪全部通過。單X11 1280×800，workarea(0,25,1280,775)，pet200×200；main全程(100,100)，disabled pet固定(300,300)，boundary起點(1080,600)向內反彈，close後沒有回退移動main。負座標屬單元測試，真實多螢幕未測。
`cua.getState`實際回apps=\[\]，原生GUI控制介面不可用；因此完整視覺／手動GUI、拖曳暫停恢復、普通啟動漫遊時序、執行中逐寵物啟停與持久化、多螢幕、多寵物、檔案操作端到端仍未驗證。既有file organizer測試只使用自有fixture，不掃描私人檔案。
CI、完整命令、探針JSON、資料庫與失敗原文見[本輪完整證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-full-fa0888e/REPORT.md)。Day18幾何切片的通過不等於全部Pet產品正式驗收。
最新main CI快照：17個jobs success、0個尚未完成；精確SHA／時間／job網址見本輪報告。Rust Desktop Pet/check-pet與build-pet覆蓋建置，Test All/test-desktop-pet覆蓋套件測試，皆沒有代替本輪原生座標或完整手動GUI驗證。精確job連結與時間見本輪報告。
主線fmt通過、嚴格all-targets Clippy exit101（[admin.rs:254](http://admin.rs:254) collapsible_if）；候選已整理logout與refresh條件，但嚴格Clippy仍exit101於apps/desktop_pet/src/[lib.rs:110](http://lib.rs:110) collapsible_if，未宣稱品質全通過。完整命令／結果見本輪證據。
