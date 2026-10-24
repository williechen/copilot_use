# Pet Day33 r1｜生活窗「控制台」按鈕選取自己的寵物再開窗

## ① 學習大綱：功能與精確來源

**本日最小可操作切片：** 同時出籠 A、B 兩隻寵物，在控制台先選 A，再點 B 生活窗內的「控制台」按鈕，應先把持久化選取切換為 B，再顯示並聚焦既有控制台；選取失敗時不得假裝開啟成功。Day32 已處理**原生右鍵選單**的寵物身分，卻尚未處理**畫面上的「控制台」按鈕**：目前 `apps/desktop_pet/frontend/index.html` 的 `window.openPetControls` 只呼叫 `open_control_window`，不呼叫 `select_pet`。這兩個入口必須一致。

文件正本 `williechen/copilot_use/main`，程式來源唯讀 `williechen/rust_ai_chat/main@f3f8255b0b701a1b57cb8c0a1737c3caa4319f2d`，規則 `daily-tdd-minimum-v1+github-only-v1`。真實鏈：Leptos CSR `apps/desktop_pet/frontend/src/main.rs` 的生活窗按鈕 → `bridge::open_controls()` → `window.openPetControls()` → Tauri `select_pet` → `PetStore::select`（SQLx SQLite `local_pet_selection`）→ `pet_windows::open` → `pet-profile-changed` → Leptos `listen_pet_profile` → Tauri `open_control_window` 顯示並聚焦同一個 `main`。不新建第二個視窗、資料表或產品專案。

## ② Red → 完整可套用 Green → Refactor

### Red：可執行的 JavaScript Bridge 契約

新增完整檔案 `apps/desktop_pet/frontend/tests/day33-open-controls-selects-origin.test.mjs`。此測試使用既有測試慣用的 `node:test`、`vm`、`readFileSync`，實際執行 `index.html` 內的 script；Green 前生活窗只呼叫 `open_control_window`，第一個斷言預期**失敗**，不是編譯失敗。這裡的測試程式碼是給使用者套用，**本輪未在 rust_ai_chat 執行**。

```javascript
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import vm from "node:vm";

const html = readFileSync(new URL("../index.html", import.meta.url), "utf8");
const script = html.match(/<script>([\s\S]*?)<\/script>/)?.[1];
assert.ok(script, "index.html must contain the real Tauri bridge script");

function boot(search, failSelection = false) {
  const calls = [];
  const location = { search };
  const window = {
    location,
    __TAURI__: {
      core: {
        invoke: async (name, args) => {
          calls.push([name, args]);
          if (name === "select_pet" && failSelection) {
            throw new Error("selection failed");
          }
          return name === "active_pet" ? { id: "fallback" } : undefined;
        }
      },
      event: { listen: async () => () => {} }
    }
  };
  vm.runInNewContext(script, { window, location, URLSearchParams });
  return { window, calls };
}

test("pet window button selects its own pet before opening main", async () => {
  const { window, calls } = boot("?view=pet&pet_id=pet-b");
  await window.openPetControls();
  assert.equal(calls.length, 2);
  assert.equal(calls[0][0], "select_pet");
  assert.equal(calls[0][1].id, "pet-b");
  assert.equal(calls[1][0], "open_control_window");
});

test("control window only focuses itself, never changes selection", async () => {
  const { window, calls } = boot("");
  await window.openPetControls();
  assert.deepEqual(calls.map(([name]) => name), ["open_control_window"]);
});

test("selection failure does not report successful opening", async () => {
  const { window, calls } = boot("?view=pet&pet_id=pet-b", true);
  await assert.rejects(window.openPetControls(), /selection failed/);
  assert.deepEqual(calls.map(([name]) => name), ["select_pet"]);
});
```

**原生 GUI Red：** 開 A、B 生活窗 → 控制台選 A → 隱藏控制台 → 點 B 生活窗「控制台」。現有 `openPetControls` 不會更新 SQLite 選取，預期控制台仍指向 A；這是從程式推導的預期失敗，未實際操作，不得冒稱已取得 GUI 截圖。

### Green：只替換一個現有 JS 函式

在 `apps/desktop_pet/frontend/index.html` 中，將原本完整函式：

```javascript
window.openPetControls = () => {
  return invoke("open_control_window");
};
```

替換成：

```javascript
window.openPetControls = async () => {
  // 生活窗按鈕必須與 Day32 的原生右鍵選單一致：先選自己。
  if (window.isPetWindow()) {
    const id = await currentPetId();
    await invoke("select_pet", { id });
  }
  // 選取失敗會 reject，絕不誤報「控制台已開啟」。
  return invoke("open_control_window");
};
```

**其餘程式不動：** `window.isPetWindow` 與 `currentPetId` 已在同一 script 定義；`select_pet` 和 `open_control_window` 是現有 `apps/desktop_pet/src/lib.rs` Tauri commands；`main.rs` 的按鈕已呼叫 `bridge::open_controls()`，失敗時會顯示「無法開啟控制台」。`select_pet` 原有的 `pet_windows::open` 重用相同 ID 的生活窗，**不新增第二個控制窗**。從控制窗內呼叫 `openPetControls` 時不重新選取，避免無關的持久化變更。

### Refactor：防止平行入口與副作用漂移

Day32 的 native menu 由 Rust `control_menu_pet_id` 驗證 UUID；本日 Leptos 生活窗按鈕透過已存在且具錯誤傳遞的 Tauri `select_pet`，兩者都使用同一 `PetStore`。不重寫 `PetStore::select`、不複製 SQLite SQL、不修改 `pet_windows::Runtime`，也不在 UI 直接操作 DB。後續若要統一兩條入口，可在 Rust 抽取共用用例，但不是本日必要重構。

### 上一 Day 與 FIFO 補教

Day32 原生右鍵選取已在來源中；但三窗 GUI、選取與焦點、重啟後資料一致性以及 SQLite 失敗注入仍需對本篇新按鈕重新驗證。Day25 單圖 FIFO 補教未結案，本日不更改歷史文章與進度證據。

## ③ 套件與軟體版本（2026-10-24 官方核對）

| 項目 | rust_ai_chat manifest／lock 或環境 | 當日官方最新穩定 | 相容性／選版 |
| --- | --- | --- | --- |
| Rust | `rust-toolchain.toml: stable`；實際安裝版未量測 | **1.99.1**（10/15） | edition 2024；維持 stable；[官方公告](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) |
| Tauri | `apps/desktop_pet/Cargo.toml: 2.12`／lock **2.12.5** | **2.12.5**（10/21） | 原有 Tauri v2 command/JS Bridge；[官方 release](https://github.com/tauri-apps/tauri/releases/tag/tauri-v2.12.5) |
| tauri-build | manifest `2.7`／lock **2.7.4** | **2.7.4**（10/21） | 無需更改 build.rs；[官方 release](https://github.com/tauri-apps/tauri/releases/tag/tauri-build-v2.7.4) |
| UUID | workspace `1.29`／lock **1.29.0** | **1.29.0**（10/23） | JS 只傳字串；Rust 現有 command 負責 ID；[官方 release](https://github.com/uuid-rs/uuid/releases/tag/v1.29.0) |
| Leptos | frontend `0.8`／lock **0.8.22** | **0.8.22**（10/05） | 保留 CSR/WASM；[官方 release](https://github.com/leptos-rs/leptos/releases/tag/v0.8.22) |
| SQLx | workspace `0.9`／lock **0.9.0** | **0.9.0**（05/06） | 既有 SQLite adapter 不改；[官方 changelog](https://github.com/transact-rs/sqlx/blob/main/CHANGELOG.md) |
| SQLite | 本輪未量測嵌入式 SQLite runtime 版本 | **3.54.1**（10/16） | 不升級、不改 migration；以 `SELECT sqlite_version()` 核對實際 runtime；[官方 release history](https://www.sqlite.org/changes.html) |
| Node.js（測試執行器） | 專案未鎖定；本輪未執行 `node -v` | **26.12.1 Current**（10/21），**24.22.1 LTS** | `node:test` 與 `vm` 為內建模組；建議 LTS 24.22.1；[官方 releases](https://github.com/nodejs/node/releases) |

`Cargo.lock` blob `85d844f618f0fd31a81775aa25f4c0af1a3754cd`；未執行安裝、升級、編譯或 migration。Node 26 Current 是最新穩定，產品驗證可優先選 24 LTS；Tauri 原生環境須有對應 WebView2/WebKitGTK 與 OS GUI，單純 Node 測試不能替代 GUI。

## ④ 驗證步驟、預期與實際

| 範圍 | 命令／流程 | 預期 | 本輪實際 |
| --- | --- | --- | --- |
| Red | `node --test apps/desktop_pet/frontend/tests/day33-open-controls-selects-origin.test.mjs`（套用 Red） | 第一測試因缺少 select_pet 而失敗 | 未執行 |
| Green | 同上（套用 Green） | 3 測試全部通過；含失敗短路 | 未執行 |
| Native Rust | `cargo test --locked -p desktop-pet` | Tauri/Store 原測試不退化 | 未執行 |
| WASM | `cargo check --locked -p desktop-pet-frontend --target wasm32-unknown-unknown` | Leptos CSR Bridge 可編譯 | 未執行 |
| 真實 GUI／DB | 兩隻不同 UUID→選 A→點 B 生活窗「控制台」→重啟讀回 | 同一 main 選 B；無重複生活窗；SQLite slot B | 未執行 |
| 失敗注入 | SQLite select 失敗／視窗聚焦失敗／通知失敗 | 不誤報成功；狀態與畫面可恢復 | 未執行 |

**狀態：** 文件可套用，未在 rust_ai_chat 寫入任何程式；測試與功能驗收尚未執行，Gate2 未通過。