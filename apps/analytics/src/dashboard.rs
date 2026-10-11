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
<section><h2>每日趨勢（Asia/Taipei）</h2><label>開始<input id="from" type="date"></label><label>結束（不含當日）<input id="to" type="date"></label><button id="trend">載入趨勢</button><table><thead><tr><th>日期</th><th>來源</th><th>事件</th><th>次數</th></tr></thead><tbody id="timeline"></tbody></table></section>
<section><h2>AI 洞察與可審核建議</h2><p>只使用聚合事件數，不把事件次數當成人數、品質或因果。建議需審核，再由目標 App 的使用者確認採用。</p><label>管理 token（只留本頁）<input id="admin" type="password" autocomplete="off"></label><label>目標<select id="target"><option value="web_chat">Chat</option><option value="desktop_pet">Desktop Pet</option></select></label><button id="generate-insight">根據實際資料產生洞察</button><button id="load-suggestions">查看建議與證據</button><div id="suggestions"></div></section>
<section><h2>可逆資料封存</h2><label>封存／還原此日期以前<input id="before" type="date"></label><label><input id="confirm-archive" type="checkbox">我確認執行此可逆操作</label><button id="archive">封存</button><button id="restore">還原</button></section>
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

const get=id=>document.getElementById(id);
async function action(path,body,admin=false){const res=await fetch(path,{method:body?'POST':'GET',cache:'no-store',headers:{Authorization:'Bearer '+(admin?get('admin').value:token.value),'Content-Type':'application/json'},body:body?JSON.stringify(body):undefined});if(!res.ok)throw Error('操作失敗：'+res.status);return res.status===204?null:res.json();}
const guarded=fn=>async()=>{error.textContent='';try{await fn()}catch(e){error.textContent=e.message}};
get('from').value=new Date(Date.now()-7*86400000).toISOString().slice(0,10);get('to').value=new Date(Date.now()+86400000).toISOString().slice(0,10);get('before').value=new Date().toISOString().slice(0,10);
get('trend').onclick=guarded(async()=>{const from=get('from').value+'T00:00:00+08:00';const to=get('to').value+'T00:00:00+08:00';const rows=await query('/api/v1/events/timeline?from='+encodeURIComponent(from)+'&to='+encodeURIComponent(to));const table=get('timeline');table.replaceChildren();for(const r of rows){const tr=document.createElement('tr');for(const v of [r.day,r.source_app,r.event_type,r.total]){const td=document.createElement('td');td.textContent=String(v);tr.appendChild(td);}table.appendChild(tr);}});
async function suggestions(){const rows=await action('/api/v1/suggestions');const target=get('suggestions');target.replaceChildren();for(const row of rows){const article=document.createElement('article');const title=document.createElement('h3');title.textContent=row.id+' · '+row.status;article.appendChild(title);const text=document.createElement('p');text.textContent=row.summary;article.appendChild(text);const evidence=document.createElement('pre');evidence.textContent=JSON.stringify({proposal:row.proposal,snapshot:row.snapshot,evaluation:row.evaluation},null,2);article.appendChild(evidence);for(const decision of ['approved','rejected']){const button=document.createElement('button');button.textContent=decision==='approved'?'核准此建議':'拒絕此建議';button.disabled=row.status!=='pending';button.onclick=guarded(async()=>{await action('/api/v1/suggestions/'+row.id+'/review',{decision,confirmed:true},true);await suggestions();});article.appendChild(button);}const evaluate=document.createElement('button');evaluate.textContent='記錄目前事件數以供比較（不代表因果）';evaluate.disabled=!['applied','rolled_back'].includes(row.status);evaluate.onclick=guarded(async()=>{await action('/api/v1/suggestions/'+row.id+'/evaluate',{confirmed:true},true);await suggestions();});article.appendChild(evaluate);target.appendChild(article);}}
get('load-suggestions').onclick=guarded(suggestions);get('generate-insight').onclick=guarded(async()=>{await action('/api/v1/insights',{target_app:get('target').value},true);await suggestions();});
for(const op of ['archive','restore'])get(op).onclick=guarded(async()=>{if(!get('confirm-archive').checked)throw Error('請先確認可逆資料操作');await action('/api/v1/events/'+op,{before:get('before').value+'T00:00:00+08:00',confirmed:true},true);await refresh();});
document.getElementById('refresh').addEventListener('click',refresh);
document.getElementById('clear').addEventListener('click',()=>{
 token.value='';get('admin').value='';get('suggestions').replaceChildren();document.getElementById('total').textContent='—';
 document.getElementById('events').textContent='請先驗證';
});
</script></body></html>"##,
    )
}
