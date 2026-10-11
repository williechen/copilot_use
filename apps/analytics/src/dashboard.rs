//! Analytics 與 Ingestion/Query API 共用同一 Web 服務。
//! Dashboard 不保存 Token，使用者必須自行輸入；禁止把憑證放進 URL。
use axum::response::Html;

pub async fn dashboard() -> Html<&'static str> {
    Html(
        r##"<!doctype html>
<html lang="zh-Hant">
<head>
<meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Product Analytics · Dashboard</title>
<style>
:root{font-family:system-ui,sans-serif;color-scheme:light dark}
body{max-width:1080px;margin:2rem auto;padding:0 1rem;line-height:1.5}
header{display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap}
main{display:grid;grid-template-columns:repeat(auto-fit,minmax(270px,1fr));gap:1rem}
article{border:1px solid #9996;border-radius:12px;padding:1.2rem}
label{display:block;margin:1rem 0}.row{display:flex;gap:.6rem;flex-wrap:wrap}
input{padding:.65rem;min-width:250px}button{padding:.65rem 1rem;cursor:pointer}
table{border-collapse:collapse;width:100%}td,th{padding:.6rem;text-align:left;border-bottom:1px solid #8885}
small{opacity:.8}.error{color:#c33}#total{font-size:2.4rem;font-weight:700}
</style>
</head>
<body>
<header><div><h1>Product Analytics</h1><p>Chat · Desktop Pet · Second Brain</p></div><span id="status">尚未連線</span></header>
<section aria-label="驗證">
<label for="token">API Bearer Token（僅保存在本頁的記憶體）</label>
<div class="row"><input id="token" type="password" autocomplete="off" placeholder="輸入 API Token">
<button id="refresh">更新統計</button><button id="clear">清除</button></div>
<p id="error" role="alert" class="error"></p>
</section>
<main>
<article><h2>累積事件</h2><output id="total">—</output><p><small>以資料庫持久化並依來源及事件 ID 去重</small></p></article>
<article><h2>來源與事件類型</h2><table><thead><tr><th>來源</th><th>事件</th><th>數量</th></tr></thead><tbody id="events"><tr><td colspan="3">請先驗證</td></tr></tbody></table></article>
</main>
<script>
'use strict';
const token = document.getElementById('token');
const error = document.getElementById('error');
const status = document.getElementById('status');
async function query(path) {
  const res = await fetch(path, {headers:{Authorization:'Bearer '+token.value},cache:'no-store'});
  if(!res.ok) throw Error('API 錯誤：HTTP '+res.status);
  return await res.json();
}
async function refresh() {
  error.textContent=''; status.textContent='查詢中…';
  try {
    const [counts, breakdown] = await Promise.all([
      query('/api/v1/events/count'),query('/api/v1/events/summary')
    ]);
    document.getElementById('total').textContent=Number(counts.total).toLocaleString();
    const tbody=document.getElementById('events');tbody.replaceChildren();
    for(const row of breakdown){
      const tr=document.createElement('tr');
      for(const value of [row.source_app,row.event_type,row.total]){
        const td=document.createElement('td');td.textContent=String(value);tr.appendChild(td);
      }
      tbody.appendChild(tr);
    }
    if(!breakdown.length) tbody.textContent='尚無事件';
    status.textContent='已更新 · '+new Date().toLocaleString();
  } catch (e) {error.textContent=String(e.message);status.textContent='查詢失敗';}
}
document.getElementById('refresh').addEventListener('click',refresh);
document.getElementById('clear').addEventListener('click',()=>{
 token.value='';document.getElementById('total').textContent='—';
 document.getElementById('events').textContent='請先驗證';
});
</script></body></html>"##,
    )
}
