## 1. 學習大綱
本次更新核對（main `7917f8b2e05ed74703ff9a83b62732ce04461da7`）：這次NOT NULL加在owner_id及附屬表pet_id，缺少關聯、無效外鍵拒絕已實測。各表id仍為TEXT PRIMARY KEY，沒有NOT NULL；主人及寵物缺少主鍵ID仍可插入NULL。新建遷移通過，舊DB checksum升級失敗。漫遊核心未變，前輪原生四案僅保留原SHA適用性，完整GUI未重跑。 [本輪命令與27個schema案例](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-7917f8b/REPORT.md)。以下6e1c791及更早版本／候選保留為歷史基線，不回填為最新main通過。完整功能仍未驗收；未新增規則或Day。
歷史候選e33971f只適用舊2eddd64；最新main已修改建表與ID契約並刪除ALTER檔，該patch不可直接套用。先前候選成功結果保留原SHA，不代表本輪checksum升級問題已解決。
本次複驗 main `1853876e0eb1cfe8e4562be8b3c661b55f1bc54b`。目前 tauri.conf.json 的 main 是控制台、pet 是生活窗；roam_desktop 已呼叫 roaming_window_label() 並選 pet，open_control_window 仍選 main。原頁「main 尚未套用」「尚未核對建窗標籤」已由實際來源核對取代。
目標是生活窗移動、控制窗不動、工作區邊界、啟動停用與關閉生活窗。這個原生幾何切片已有實測；完整手動 GUI、多螢幕、多寵物與普通啟動時序仍分開標示未驗證。原學習主體 15 分鐘；原生探針重現另估 20 分鐘。
## 2. 實作內容
### Red 的歷史與本次現況
98f30d8 發布時設計「缺少 roaming_window_label」的編譯 Red，當時未執行，沒有回填成已實測。現在 main 已有該函式與 day18_roaming_selects_pet_life_window 測試；重跑是回歸，不能再說符號缺少或重複插入同名測試。
### 已套用的最小 Green
以下程式目前已在 main，供核對，不重複加入：
```rust
const PET_WINDOW_LABEL: &str = "pet";
fn roaming_window_label() -> &'static str {
    PET_WINDOW_LABEL
}
```
roam_desktop 的選窗為：
```rust
let window = match app.get_webview_window(roaming_window_label()) {
    Some(window) => window,
    None => {
        eprintln!("desktop roaming unavailable: pet window not found");
        break;
    }
};
```
呼叫鏈為 application/setup → PET_ENABLE_ROAM → roam_desktop → pet WebviewWindow → outer_position/outer_size/current_monitor → bounded_next_position → set_position → 位移／stall 檢查。原生核心與修正候選 `80ba01e6d965d0e3ff10bef18a5cbd17f01e8f05` 相同；候選僅另加 acceptance-probe 測試入口。
### 原生重現與 Refactor
先在隔離工作分支套用 [完整可套用 patch](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-1853876/day18-functional-fixes.patch)，再依 patch 中 scripts/acceptance/[README.md](http://README.md) 建置測試特性。這不是 main 已存在的 feature；不在未套用 patch 的 main 直接執行：
```bash
cargo build --locked -p desktop-pet --features acceptance-probe
PET_ACCEPTANCE_SCENARIO=enabled PET_ENABLE_ROAM=true LIBGL_ALWAYS_SOFTWARE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 target/debug/desktop-pet
PET_ACCEPTANCE_SCENARIO=disabled PET_ENABLE_ROAM=false LIBGL_ALWAYS_SOFTWARE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 target/debug/desktop-pet
PET_ACCEPTANCE_SCENARIO=boundary PET_ENABLE_ROAM=true LIBGL_ALWAYS_SOFTWARE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 target/debug/desktop-pet
PET_ACCEPTANCE_SCENARIO=close PET_ENABLE_ROAM=true LIBGL_ALWAYS_SOFTWARE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 target/debug/desktop-pet
```
探針等視窗映射完成後讀真實 200×200 尺寸／工作區，再執行原有 roam_desktop，取回兩窗 OS 座標。初版映射前讀到暫時 540×324 的夾具錯誤已修正，舊失敗保留為診斷，未採為邊界通過證據。沒有更動原漫遊演算法。
## 3. 套件版本
Rust 1.99.1／Cargo 1.99.1；Cargo.lock：Axum 0.8.9、Leptos 0.8.22、SQLx 0.9.0、Tokio 1.53.3、Tauri 2.12.4、tauri-build 2.7.3、wasm-bindgen 0.2.129、web-sys 0.3.106、Tower 0.5.3。Ubuntu 26.04.1／PostgreSQL 18.6／Node 22.22.1／cargo-leptos 0.3.11／Trunk 0.21.14。本次不新增產品依賴；MSRV／跨平台未另行驗證。 原生環境 X11 單螢幕 1280×800，工作區 (0,25,1280,775)，縮放 1；未測真實多螢幕／不同 DPI。tauri.conf.json 的產品版號 0.2.16 與 workspace package 0.2.17 為不同欄位，沒有當成同一版本。
## 4. 驗證項目
`cargo test --locked -p desktop-pet`：原生單元 16＋所屬整合 18，共 34 passed；含 label、負座標邊界與明確啟用值。原生幾何本輪四案再次 4 passed：enabled 有位移、disabled 維持 (300,300)、boundary 自 (1080,600) 向內反彈、close 關生活窗後不 fallback main；main 各案維持 (100,100)。[本次正式複驗與原始證據](https://github.com/williechen/copilot_use/blob/teaching/rust-ai-chat-alignment/docs/teaching/evidence/2026-10-18-main-1853876/REPORT.md) 保存逐次座標與命令。
這證明實際 Tauri 視窗幾何與原核心行為，不等於普通 setup 立即漫遊、視覺呈現、控制窗關閉／重開、拖曳暫停／恢復、睡眠政策、執行中停用／持久化、多螢幕拔除／找回、多寵物或安全整理／還原全流程通過。這些仍列原產品缺口；完整功能驗收維持未通過。
Day17 的標籤待核對缺口已完成，本篇明確記錄已套用範圍及真實證據，沒有新增規則或增加 Day。
