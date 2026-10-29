# Pet Day31 r1｜關閉控制窗後，從生活窗重開同一個控制台

## ① 學習大綱、現況與真實入口

本次重新讀取文件規則／需求／進度後取 `next_day.pet=31`。文件正本 `williechen/copilot_use/main`，初讀文件 SHA `259a1e6f08a6522ef596fe1448af4ac2bed56167`；唯讀程式來源 `williechen/rust_ai_chat/main@1d1866dbbb30255fafa4c7f74b1868919662913c`。發布前再次核對兩庫 SHA／規則與當案狀態，交付證據保存實際 parent。規則 `daily-tdd-minimum-v1 / github-only-v1`。

現有 `apps/desktop_pet/tauri.conf.json` 明確定義控制窗 label `main`，800×680、可縮放；`pet_windows::open` 依 UUID 建立 `pet-{simple_uuid}` 生活窗。`lib.rs::open_control_window` 只尋找既有 main、show、focus。現有 `.on_window_event` 只處理 pet 視窗 Destroyed，沒有攔截控制窗 CloseRequested：主窗一旦被關閉銷毀，重開 IPC 就找不到 main；而且 frontend/index.html 的生活窗右鍵選單目前只有互動／睡覺／醒來，尚缺開啟控制窗項目。這是既有多窗流程缺口，不需要新建第二套控制台。

本日切片：控制窗按 X → Tauri 關閉請求 → 保留並隱藏 main → 寵物生活窗繼續運作 → 右鍵「開啟控制窗」→ 既有 `open_control_window` → show/focus 原 main。Leptos CSR 控制畫面／既有 bridge 與 IPC 重用；既有 Application/Domain `Runtime`、`PetMachine` 和 SQLx `PetStore` 不因關控制窗被重設或寫入。這是完整鏈的窗口生命週期增量，沒有必要每天修改全部層。

建立第一隻寵物、清單、選取、SQLite 與獨立生活窗在來源中已存在，本日不重寫。核心 15 分鐘；三案各 15 加共用 15 共 60，原生 GUI、補教、來源版本與文件交付另計。原生 GUI 本次未執行；環境無 rustc（`/bin/bash: line 1: rustc: command not found`，exit 127），這是環境限制，不是 TDD Red。

## ② Red → 完整最小 Green → Refactor → 未完成補充

### Red：以原生 GUI 的可操作流程作為驗收測試

本切片的失敗來自真實 close event；不新增一個只重述 `label == main` 的單元測試，也不把 mock window 存在當原生視窗存續證據。以下是一個完整、可重複的手動驗收案例；先在原 source 執行，再套同篇 Green 後原樣重跑。本次未執行，Red 是經來源核對的預期失敗。

1. 使用全新隔離的 Pet SQLite 和測試設定啟動精確來源的原生 App。只使用測試資料，不連正式目錄，不啟動付費 AI。在控制台建立三隻不同名稱並各出籠，記錄三個不同 pet id 和生活窗標題；三隻逐一睡覺／喚醒，確定事件不串到另外兩隻。
2. 在控制台尚未送出的名稱欄輸入「尚未送出31」，記錄目前分頁及各生活窗的狀態。按控制窗標題列 X，等待 5 秒，確認三個生活窗仍存在且各自可操作。
3. 在其中一隻生活窗右鍵，按「開啟控制窗」。預期控制窗恢復、保留未送出文字／原分頁，不新增寵物，不重置生活窗；再按同入口五次，應始終只有一個控制窗。
4. 原來源右鍵根本沒有開啟控制窗項目；即使透過既有 bridge 呼叫，主窗已被銷毀而 `open_control_window` 只查既有 main，預期步驟 3 無法恢復。若本機結果不同，先記錄平台／source／實際 close event，不假稱重現成功。

判定表：`主窗可重開 && 草稿保留 && 控制窗數=1 && 三個pet id不變 && 生活窗狀態不互相覆蓋` 才是本切片成功。沒有真實 GUI 可用時只能列未執行。

### Green：完整替換既有事件 closure

在 `apps/desktop_pet/src/lib.rs::application`，完整替換原本 `.on_window_event(|window, event| { ... })` 為下面內容。`.manage(...)`、`.setup(...)`、menu handler、invoke_handler 和 `.run(...)` 全部保留。檔案已匯入 `tauri::Manager`，不新增 crate／command／capability／migration。

```rust
        .on_window_event(|window, event| {
            // 控制台關閉僅隱藏，讓既有 show/focus IPC 能恢復同一個 main。
            if window.label() == "main" {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    if let Err(error) = window.hide() {
                        // 隱藏失敗時仍保留視窗；不把錯誤當成銷毀成功。
                        eprintln!("隱藏控制視窗失敗：{error}");
                    }
                }
                return;
            }

            // 保留生活窗原有 Destroyed 清理；本日不改回籠持久化政策。
            if window.label().starts_with("pet-")
                && let tauri::WindowEvent::Destroyed = event
                && let Some(id) = window.label()
                    .strip_prefix("pet-")
                    .and_then(|s| uuid::Uuid::parse_str(s).ok())
            {
                window.app_handle()
                    .state::<pet_windows::Runtime>()
                    .remove(&id.to_string());
            }
        })
```

重開沿用已存在且已核實的 `open_control_window`，不必再次貼入同一函式或另建新窗。它完整的現行契約如下，**只供核對，不重複新增**：

```rust
#[tauri::command]
fn open_control_window(app: tauri::AppHandle) -> Result<(), String> {
    let window = app.get_webview_window("main").ok_or("控制視窗不存在")?;
    window.show().map_err(|_| "無法顯示控制視窗")?;
    window.set_focus().map_err(|_| "無法聚焦控制視窗".into())
}
```

在 `apps/desktop_pet/frontend/index.html` 的既有 `<script>` 內，完整替換 `getPetContextMenu` 函式如下。保留 `petContextMenu` 變數與 `showPetContextMenu`，只在現有同一選單增加可操作入口，不另建第二個 menu：

```javascript
    async function getPetContextMenu() {
      if (petContextMenu) return petContextMenu;
      const { Menu } = window.__TAURI__.menu;
      const id = await currentPetId();
      petContextMenu = await Menu.new({
        items: [
          { id: "open_control", text: "開啟控制窗" },
          { id: `pet:${id}:interact`, text: "互動" },
          { id: `pet:${id}:sleep`, text: "睡覺" },
          { id: `pet:${id}:wake`, text: "醒來" },
        ],
      });
      return petContextMenu;
    }
```

原生 `.on_menu_event` 已有 `event.id() == "open_control"` 分支並呼叫相同 `open_control_window`，Linux 亦沿用原生 event，而不依賴 JS action callback。既有 frontend `bridge.rs::open_controls → index.html::window.openPetControls → invoke("open_control_window")` 也保持不變。今天新增右鍵入口並修復其主窗生命週期，兩處修改缺一不可。這只完成開啟／聚焦；從不同生活窗開啟時自動選取對應 pet id 的功能尚未加入，不宣稱整條多寵物控制政策完成。

`CloseRequested` 的 `api.prevent_close()` 同步呼叫，在事件結束前阻止銷毀，再 hide；沒有 await／DB transaction 混進事件 callback。Tauri 的事件列舉與該 variant 都是 non-exhaustive，所以保留 `..`。官方契約見 [WindowEvent](https://docs.rs/tauri/latest/tauri/enum.WindowEvent.html) 與 [CloseRequestApi](https://docs.rs/tauri/latest/tauri/struct.CloseRequestApi.html)。

### Refactor：一個視窗一種生命週期

控制台沿用 main，不因每次開啟建立新 Leptos root；隱藏保留當前 CSR 狀態。生活窗繼續由 pet id 管理，原生「退出程式」仍使用既有 `quit_pet → app.exit(0)`，不能把所有視窗 CloseRequested 一概攔截。這個修改沒有更新寵物資料與預設出籠設定，也沒有把控制窗關閉當回籠。直接關閉生活窗等同回籠的持久化需求是另一個未完成項，原 Destroyed 清理不足以證明該需求完成。

### 上一 Day 未完成事項補充

來源為 Pet Day30 r2 的回籠補償。最新 `pet_windows.rs::cage_saved_pet` 已有 `set_caged(true) → close → 失敗時 set_caged(false) → 成功才 Runtime::remove`，不再次教相同 Green。SQLite 失敗、關窗失敗、補償再失敗的注入與三窗重啟仍待驗；應在隔離資料中逐項造成失敗，檢查 DB 標記、真實窗與 Runtime，不可只由命令回傳錯誤推論補償正確。此篇主窗 hide 修補不替代上述測試，沒有證據不把舊篇封存。

### 舊 Day 更正補充：FIFO

最早 pending `pet-day25-single-image-20261022` 保留。單一 Leptos component 不等於每幀單一寵物圖像；本篇三窗測試還要觀察每幀圖像、200×200 等比裁切、逐 ID 事件與獨立狀態，但本次沒有實跑，不能結案。Day25 以前原文、commit、blob、SHA256 不修改；不因文件讀回通過結案或額外加 Day。

## ③ 套件與軟體版本（2026-10-23 官方穩定版核對）

| 項目／官方來源 | 現有宣告／lock | 最新穩定與選版 |
| --- | --- | --- |
| [Rust／Cargo](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) | stable；edition 2024；本機 rustc 不存在 | 1.99.1，建議驗證用此版；未安裝 |
| [Tauri](https://docs.rs/crate/tauri/latest) | 2.12／2.12.5 | 2.12.5，與 lock 相同，沿用 |
| [tauri-build](https://docs.rs/crate/tauri-build/latest) | 2.7／2.7.4 | 2.7.4，沿用 |
| [Leptos](https://docs.rs/crate/leptos/latest) | frontend 0.8/csr／0.8.22 | 0.8.22，沿用既有 CSR |
| [SQLx](https://docs.rs/sqlx/latest/sqlx/) | 0.9／0.9.0 | 0.9.0，沿用；不改查詢與 migration |
| [Tokio](https://docs.rs/crate/tokio/latest) | 1.54/full／1.54.1 | 1.54.1，沿用 |
| [UUID](https://docs.rs/crate/uuid/latest) | 1.28／1.28.0 | 1.28.0，沿用 label 解析 |
| [Serde](https://docs.rs/crate/serde/latest)／[JSON](https://docs.rs/crate/serde_json/latest) | 各1.0／1.0.229、1.0.154 | 相同；事件契約不變 |
| [wasm-bindgen](https://docs.rs/crate/wasm-bindgen/latest)／[futures](https://docs.rs/crate/wasm-bindgen-futures/latest) | 0.2、0.4／0.2.129、0.4.79 | 相同；既有 bridge 不升級 |
| [js-sys](https://docs.rs/crate/js-sys/latest)／[web-sys](https://docs.rs/crate/web-sys/latest) | 各0.3／各0.3.106 | 各0.3.106；沿用 |
| [SQLite](https://www.sqlite.org/)／[bindings](https://docs.rs/crate/libsqlite3-sys/latest) | SQLx 間接；libsqlite3-sys 0.37.0；runtime engine 未執行查詢 | engine 3.54.1、bindings 0.38.2；不是目前 engine 版號聲明 |

[官方 Tauri manifest](https://docs.rs/crate/tauri/latest/source/Cargo.toml) 的 MSRV 為 1.90；Leptos 為 1.88、SQLx 為 1.94.0，所以應至少考慮較高門檻，採目前穩定 Rust 1.99.1。全圖 MSRV 與三平台 ABI 尚未在本機驗證。Tauri 是 native，Leptos frontend 是 wasm32-unknown-unknown；hide／show／focus 由既有原生橋接呼叫，不把 Tauri native crate 編到前端。

本篇沒有新增 CLI；驗收使用既有 App 或 Cargo，沒有安裝 Tauri CLI／Trunk／WebDriver。Linux 原生 GTK／WebKitGTK、macOS WebKit、Windows WebView2 環境須沿既有平台前置條件重驗；本次未讀取其安裝版、未變更環境，也不宣稱跨平台已通過。SQLite 最新穩定是後續驗證目標，但 SQLx 目前鎖定 bindings 0.37，不能只因上游最新是0.38.2直接改 lock；升級須先核對 SQLx semver／links／bundled native library 並由使用者另行套用。本文不 cargo update、不改 Cargo.lock、不假稱完成 DB 升級。

## ④ 驗證步驟、CI 與剩餘缺口

使用者套用後在工作副本設定外部全新 `CARGO_TARGET_DIR`，先跑：

```sh
cargo check --locked -p desktop-pet
cargo test --locked -p desktop-pet
cargo check --locked -p desktop-pet-frontend --target wasm32-unknown-unknown
```

再使用全新隔離 Pet SQLite／設定與實際 native App，完整重跑第②節 Red 的四步。不要在正式 DB 上刪表、清資料或修改 checksum。若還沒有可核對來源的前端 build，先補其建置證據；不能拿舊 WASM 配新 native binary 當完整結果。

| 項目 | 預期 | 本次實際 |
| --- | --- | --- |
| 原來源 Red | 關主窗後既有 IPC 無法恢復 main | 來源推導；GUI 未執行 |
| 套用 Green 的 compile/native tests/WASM check | 可編譯，既有測試無退化 | 未執行，環境缺 rustc |
| main X→生活窗右鍵→重開五次 | 同一控制台，草稿／分頁保留、可操作 | 未執行 |
| 三個不同 pet id、睡覺／喚醒、事件與單圖動畫 | 各自獨立，無重設或多 component | 未執行 |
| hide 失敗、show/focus 失敗 | 保留 main，錯誤不冒充成功 | 未注入；錯誤處理待實測 |
| 全回籠後 main 隱藏、既有退出入口 | 行為與可恢復／可退出政策需實測 | 未執行；最後可見窗關閉後 UX 仍待檢查 |
| Day30 回籠補償與 App 重啟 | 逐窗／DB／Runtime 一致 | 未執行；不在本篇宣稱完成 |

精確來源 CI：`1d1866dbbb30255fafa4c7f74b1868919662913c` 的 [Test All 39932989041](https://github.com/williechen/rust_ai_chat/actions/runs/39932989041) completed/success（2026-10-23T09:38:46Z），job `test-desktop-pet` 120345579348 success，workflow 命令 `cargo test --locked -p desktop-pet`。另 [Rust Desktop Pet 39932988976](https://github.com/williechen/rust_ai_chat/actions/runs/39932988976) completed/success，2026-10-23T10:15:54Z；`check-pet` 120345579301、`build-pet` 120345578869 success。已讀 workflow，包含 native check/build 及 frontend WASM／Trunk build；未覆蓋本篇尚未套用的 close/hide GUI 流程。

文件 Gate 1 的 commit／全文讀回／SHA256／進度同步另存交付證據。原生 GUI Gate 2 未通過，文章留 daily，不封存；程式套用、產品整合及功能驗收均未執行。本次只寫 copilot_use 文件，rust_ai_chat 嚴格唯讀；`last_full_comparison` 不變。後續仍按最新來源與最早缺口選題，不預排功能、不重複增加 Day。
