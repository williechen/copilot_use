# 2026-10-17｜P0 Analytics Day 15｜Dashboard 查詢與篩選接線

來源文章：[Notion 原頁](https://app.notion.com/p/3fbe51d4509381319460de180d9e173d)。

## 1. 學習大綱
主體 Red 5 分＋Green 15 分。以現有來源／事件類型統計 API 接出可查詢的 Dashboard：選來源 → 輸入本次憑證 → 查詢／篩選 → 顯示 PostgreSQL 聚合。三專案 Red＋Green 合計 55 分；細節與 Refactor 不計入主體。
來源：rust_ai_chat/main `3e58e650bd63bcd21ba58e7aabf751542401ef80`；2026-10-17 重新讀取 main、相關 source／呼叫端及 CI，六個 workflow 均 success。這是既有來源 CI，不能驗證下列教學增量。copilot_use 教學分支基準 `adad4f26964b2518f47a9fde76544b544cd44334`；產品 `9f632c76ee432d77c1d20f46da9485d982d49c9e` 僅沿用舊靜態證據，本次未重新驗收產品。修訂版 Day15-r2：沿用原頁，原頁標記未套用且最新 main 缺少本次入口，故重整未實作內容。程式未套用、測試未執行、GUI／流程未驗收。
靜態現況：`apps/stats_server/src/lib.rs::build_app` 有 summary／source_kind_summary，沒有 Dashboard route；不需要另造 total API 才能顯示主要流程。既有 source token 授權與參數綁定查詢保留。
## 2. 實作內容
### Red
先以瀏覽器開啟同一 stats-server 的 `/dashboard`，嘗試來源選擇、類型篩選與結果表格；目前無入口，預期手動 HTTP／UI Red，尚未執行。認證／DB 舊測試保留為回歸，不當本次新 Red。
### 最小 Green
在 `apps/stats_server/src/lib.rs` 的 build_app 前新增完整 handler：
```rust
async fn dashboard() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("dashboard.html"))
}
```
在同一 build_app 的 healthz route 旁新增（既有 API routes 全保留）：
```rust
.route("/dashboard", get(dashboard))
```
新增 `apps/stats_server/src/dashboard.html` 全文：
```html
<!doctype html>
<html lang="zh-Hant"><meta charset="utf-8"><title>事件統計</title>
<h1>事件統計</h1>
<form id="query">
<label>來源 <select id="source"><option value="web_chat">聊天室</option><option value="desktop_pet">桌面寵物</option></select></label>
<label>事件類型（留空為全部）<input id="kind" maxlength="128"></label>
<label>本次來源憑證 <input id="token" type="password" autocomplete="off" required maxlength="512"></label>
<button id="load">查詢／重新整理</button>
</form>
<p id="status" role="status">請選來源並查詢</p>
<table><thead><tr><th>來源</th><th>事件類型</th><th>數量</th></tr></thead><tbody id="rows"></tbody></table>
<script>
const form = document.querySelector("#query");
const rows = document.querySelector("#rows");
const status = document.querySelector("#status");
const load = document.querySelector("#load");
form.addEventListener("submit", async event => {
  event.preventDefault();
  rows.replaceChildren();
  status.textContent = "載入中…";
  load.disabled = true;
  const source = document.querySelector("#source").value;
  const kind = document.querySelector("#kind").value;
  const tokenInput = document.querySelector("#token");
  const token = tokenInput.value;
  tokenInput.value = ""; // 僅保留此次請求，不儲存至 URL／localStorage。
  try {
    let url = "/api/v1/events/" + encodeURIComponent(source) + "/summary";
    if (kind !== "") url += "/" + encodeURIComponent(kind);
    const response = await fetch(url, {
      headers: { Authorization: "Bearer " + token }, cache: "no-store"
    });
    if (!response.ok) throw new Error("查詢失敗：" + response.status);
    const data = await response.json();
    if (!Array.isArray(data)) throw new Error("統計格式錯誤");
    for (const item of data) {
      if (item.source_app !== source || typeof item.event_type !== "string"
          || !Number.isSafeInteger(item.total) || item.total < 0) {
        throw new Error("統計格式錯誤");
      }
    }
    for (const item of data) {
      const tr = document.createElement("tr");
      for (const value of [item.source_app, item.event_type, item.total]) {
        const td = document.createElement("td");
        td.textContent = String(value); // 事件文字不當成 HTML。
        tr.append(td);
      }
      rows.append(tr);
    }
    status.textContent = data.length ? "查詢完成" : "尚無事件";
  } catch (error) {
    rows.replaceChildren();
    status.textContent = error.message || "查詢失敗，請重試";
  } finally { load.disabled = false; }
});
</script></html>
```
呼叫鏈：同服務 /dashboard → 使用者送出表單 → 同源 fetch／Bearer → 現有 summary 或 source_kind_summary → 來源認證 → pg_store 參數化聚合 → JSON → 表格。沒有跨源設定、平行資料來源或付費 AI。
這是本機操作教學入口：憑證由操作者當次輸入，不在 HTML 內嵌 server ingestion token、不讀私人環境、不持久化。正式 Dashboard 的管理者 Session／唯讀查詢權限與 TLS 仍屬後續適配，不能直接當正式管理後台發布。
### Refactor
沿用現有來源聚合，移除原未套用 Day15 的額外 total API 提案，避免同功能平行路徑。先用同服務靜態 HTML 接通基本流程，後續再按既有 UI 架構抽元件與 SSE；API／DB 不重寫。載入禁用送出，避免回應亂序；每次先清舊資料，錯誤不留過時成功結果。
### 舊 Day 更正補充
Day14 已實作的篩選 API 原文封存。原未套用 Day15 在既有頁面重整成 Dashboard 主流程；不新增 Day，不回填已實作文章。Chat／Pet 真實動作自動上報至此仍需另驗證，合成 fixture 不等於真實事件接通。
## 3. 套件版本
已讀 stats-server Cargo：沿用 workspace axum／serde／sqlx／tokio／uuid；Html／include_str 不新增依賴，瀏覽器使用原生 fetch。lock／官方 stable／MSRV／安全公告未重新核實，不宣稱版本合規。
## 4. 驗證項目
待執行：`cargo test -p stats-server --locked`、有原生 PostgreSQL 環境時 `cargo test -p stats-server --features postgres-integration --locked`、`cargo fmt --all -- --check`。
真實入口：/dashboard 200 → 專用測試來源事件持久化 → 未篩選及指定類型數量 → 空結果 → 缺憑證／跨來源 401 → DB 關閉 503 → 重試成功。確認輸入文字以 textContent 呈現、憑證不出現在 URL／storage／HTML／日誌；不要使用正式 token fixture。
本次所有測試、DB、瀏覽器與跨服務驗收未執行；只完成教學文件更新，真實 Chat／Pet 上報、SSE 與管理者權限未驗收。
