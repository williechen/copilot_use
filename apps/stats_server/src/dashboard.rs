//! Analytics 與 Ingestion/Query API 共用同一 Web 服務。
//! Dashboard 不保存 Token，使用者必須自行輸入；禁止把憑證放進 URL。
use axum::response::Html;

pub async fn dashboard() -> Html<String> {
    render(false)
}
pub async fn management() -> Html<String> {
    render(true)
}
fn render(admin: bool) -> Html<String> {
    let template = r##"<!doctype html>
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
<header><div><h1>Product Analytics · {{page-label}}</h1>{{management-link}}<p>Chat · Desktop Pet · Second Brain</p></div><span id="status">尚未連線</span></header>
<section aria-label="驗證">
<label for="token">API Bearer Token（僅保存在本頁的記憶體）</label>
<div class="row"><input id="token" type="password" autocomplete="off" placeholder="輸入 API Token">
<button id="refresh">更新統計並即時連線</button><button id="clear">清除</button></div>
<p id="error" role="alert" class="error"></p><p id="live-status" role="status">即時更新尚未連線</p>
</section>
<main>
<article><h2>累積事件</h2><output id="total">—</output><p><small>以資料庫持久化並依來源及事件 ID 去重</small></p></article>
<article><h2>來源與事件類型</h2><table><thead><tr><th>來源</th><th>事件</th><th>數量</th></tr></thead><tbody id="events"><tr><td colspan="3">請先驗證</td></tr></tbody></table></article>
</main>
<section><h2>最近事件明細（最多100筆）</h2><label>來源<select id="recent-source"><option value="">全部</option><option value="web_chat">Chat</option><option value="desktop_pet">Desktop Pet</option></select></label><label>事件類型<input id="recent-kind" placeholder="例如 keyword.rust；留白顯示全部"></label><button id="recent-events">查詢事件明細</button><p>時間為 Analytics 實際接收時間（Asia/Taipei）；匿名技術詞彙事件只包含固定詞表中的詞與次數。</p><table><thead><tr><th>接收時間</th><th>來源</th><th>事件</th><th>事件 ID</th></tr></thead><tbody id="recent-rows"></tbody></table></section>
<section><h2>每日趨勢（Asia/Taipei）</h2><label>開始<input id="from" type="date"></label><label>結束（不含當日）<input id="to" type="date"></label><button id="trend">載入趨勢</button><table><thead><tr><th>日期</th><th>來源</th><th>事件</th><th>次數</th></tr></thead><tbody id="timeline"></tbody></table></section>
<!-- management-start --><section><h2>管理驗證</h2><label>管理 token（只留本頁）<input id="admin" type="password" autocomplete="off"></label></section><section><h2>聊天室使用者操作紀錄</h2><p>需管理 token（上方輸入）。僅包含新版啟用後的紀錄，歷史匿名事件無法還原帳號。失敗登入顯示輸入的帳號，不代表該帳號存在；管理者建房顯示「系統管理者」。最多顯示最新100筆，時間範圍沿用上方每日趨勢的台北日期。</p>
<label>帳號（留白為全部）<input id="audit-user" maxlength="100"></label><label>動作<select id="audit-kind"><option value="">全部</option><option value="login.succeeded">登入成功</option><option value="login.failed">登入失敗</option><option value="logout.succeeded">登出</option><option value="room.created">建立房間</option></select></label><label>結果<select id="audit-result"><option value="">全部</option><option value="true">成功</option><option value="false">失敗</option></select></label><button id="audit-query">查詢使用者操作紀錄</button><p id="audit-status" role="status">尚未查詢</p><table><thead><tr><th>操作時間（Asia/Taipei）</th><th>帳號</th><th>動作</th><th>結果</th><th>房間</th><th>事件 ID</th></tr></thead><tbody id="audit-rows"></tbody></table></section>
<section><h2>AI 洞察與可審核建議</h2><p>只使用聚合事件數，不把事件次數當成人數、品質或因果。建議需審核，再由目標 App 的使用者確認採用。</p><label>目標<select id="target"><option value="web_chat">Chat</option><option value="desktop_pet">Desktop Pet</option></select></label><button id="generate-insight">根據實際資料產生洞察</button><button id="load-suggestions">查看建議與證據</button><div id="suggestions"></div></section>
<section><h2>可逆資料封存</h2><label>封存／還原此台北時間以前<input id="before" type="datetime-local"></label><label><input id="confirm-archive" type="checkbox">我確認執行此可逆操作</label><button id="archive">封存</button><button id="restore">還原</button></section>
<!-- management-end --><script>
'use strict';
const token = document.getElementById('token');
document.getElementById('recent-events').onclick=async()=>{try{const params=new URLSearchParams();const source=document.getElementById('recent-source').value,kind=document.getElementById('recent-kind').value.trim();if(source)params.set('source_app',source);if(kind)params.set('event_type',kind);const rows=await query('/api/v1/events/recent?'+params);const body=document.getElementById('recent-rows');body.replaceChildren();for(const row of rows){const tr=document.createElement('tr');for(const value of [new Date(row.received_unix_ms).toLocaleString('zh-TW',{timeZone:'Asia/Taipei'}),row.source_app,row.event_type,row.event_id]){const td=document.createElement('td');td.textContent=String(value);tr.appendChild(td);}body.appendChild(tr);}}catch(e){error.textContent=e.message;}};

const error = document.getElementById('error');
const status = document.getElementById('status');
async function query(path) {
  const res = await fetch(path, {headers:{Authorization:'Bearer '+token.value},cache:'no-store'});
  if(!res.ok) throw Error('API 錯誤：HTTP '+res.status);
  return await res.json();
}
async function refresh() {
  const credential=token.value;
  error.textContent=''; status.textContent='查詢中…';
  try {
    const [counts, breakdown] = await Promise.all([
      query('/api/v1/events/count'),query('/api/v1/events/summary')
    ]);
    if(token.value!==credential)return;
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
  } catch (e) {if(token.value!==credential)return;error.textContent=String(e.message);status.textContent='查詢失敗';}
}

const get=id=>document.getElementById(id);
async function action(path,body,admin=false){const res=await fetch(path,{method:body?'POST':'GET',cache:'no-store',headers:{Authorization:'Bearer '+(admin?get('admin').value:token.value),'Content-Type':'application/json'},body:body?JSON.stringify(body):undefined});if(!res.ok)throw Error('操作失敗：'+res.status);return res.status===204?null:res.json();}
const guarded=fn=>async()=>{error.textContent='';try{await fn()}catch(e){error.textContent=e.message}};
const taipeiDay=ms=>{const p=Object.fromEntries(new Intl.DateTimeFormat('en',{timeZone:'Asia/Taipei',year:'numeric',month:'2-digit',day:'2-digit'}).formatToParts(new Date(ms)).map(p=>[p.type,p.value]));return p.year+'-'+p.month+'-'+p.day;};get('from').value=taipeiDay(Date.now()-7*86400000);get('to').value=taipeiDay(Date.now()+86400000);get('before').value=taipeiDay(Date.now())+'T00:00';
get('trend').onclick=guarded(async()=>{const from=get('from').value+'T00:00:00+08:00';const to=get('to').value+'T00:00:00+08:00';const rows=await query('/api/v1/events/timeline?from='+encodeURIComponent(from)+'&to='+encodeURIComponent(to));const table=get('timeline');table.replaceChildren();for(const r of rows){const tr=document.createElement('tr');for(const v of [r.day,r.source_app,r.event_type,r.total]){const td=document.createElement('td');td.textContent=String(v);tr.appendChild(td);}table.appendChild(tr);}});
// management-script-start
async function suggestions(){const rows=await action('/api/v1/suggestions');const target=get('suggestions');target.replaceChildren();for(const row of rows){const article=document.createElement('article');const title=document.createElement('h3');title.textContent=row.id+' · '+row.status;article.appendChild(title);const text=document.createElement('p');text.textContent=row.summary;article.appendChild(text);const evidence=document.createElement('pre');evidence.textContent=JSON.stringify({proposal:row.proposal,snapshot:row.snapshot,evaluation:row.evaluation},null,2);article.appendChild(evidence);for(const decision of ['approved','rejected']){const button=document.createElement('button');button.textContent=decision==='approved'?'核准此建議':'拒絕此建議';button.disabled=row.status!=='pending';button.onclick=guarded(async()=>{await action('/api/v1/suggestions/'+row.id+'/review',{decision,confirmed:true},true);await suggestions();});article.appendChild(button);}const evaluate=document.createElement('button');evaluate.textContent='記錄目前事件數以供比較（不代表因果）';evaluate.disabled=!['applied','rolled_back'].includes(row.status);evaluate.onclick=guarded(async()=>{await action('/api/v1/suggestions/'+row.id+'/evaluate',{confirmed:true},true);await suggestions();});article.appendChild(evaluate);target.appendChild(article);}}
get('audit-query').onclick=async()=>{
 const rows=get('audit-rows'),status=get('audit-status');rows.replaceChildren();error.textContent='';
 if(!get('admin').value){status.textContent='請先在上方填入管理 token；一般查詢 token 無法查看使用者紀錄。';return;}
 const credential=get('admin').value,username=get('audit-user').value.trim(),kind=get('audit-kind').value,result=get('audit-result').value;
 const input={from_unix_ms:Date.parse(get('from').value+'T00:00:00+08:00'),to_unix_ms:Date.parse(get('to').value+'T00:00:00+08:00')};
 if(!Number.isFinite(input.from_unix_ms)||!Number.isFinite(input.to_unix_ms)||input.to_unix_ms<=input.from_unix_ms){status.textContent='請設定有效的開始與結束日期。';return;}
 if(username)input.username=username;if(kind)input.event_type=kind;if(result)input.succeeded=result==='true';
 status.textContent='查詢中…';get('audit-query').disabled=true;
 try {
  const res=await fetch('/api/v1/audit/chat',{method:'POST',cache:'no-store',headers:{Authorization:'Bearer '+credential,'Content-Type':'application/json'},body:JSON.stringify(input)});
  if(!res.ok)throw Error(res.status===401?'管理 token 不正確或權限不足。':res.status===503?'尚未設定 Chat 操作紀錄來源。':res.status===502?'暫時無法連線到 Chat 操作紀錄，請稍後重試。':res.status===400?'篩選條件不符，日期範圍最多366天。':'查詢失敗：'+res.status);
  const data=await res.json();if(get('admin').value!==credential)return;
  const names={'login.succeeded':'登入','login.failed':'登入','logout.succeeded':'登出','room.created':'建立房間'};
  for(const row of data){const tr=document.createElement('tr');for(const value of [new Date(row.occurred_unix_ms).toLocaleString('zh-TW',{timeZone:'Asia/Taipei'}),row.username||'系統管理者',names[row.event_type]||row.event_type,row.succeeded?'成功':'失敗',[row.room_name,row.target_username?'→ '+row.target_username:''].filter(Boolean).join(' ')||'—',row.event_id]){const td=document.createElement('td');td.textContent=value;tr.appendChild(td);}rows.appendChild(tr);}
  status.textContent=data.length?'已查得 '+data.length+' 筆紀錄（最多100筆）。':'此條件沒有紀錄；新版啟用前的匿名事件不含帳號。';
 }catch(e){if(get('admin').value===credential)status.textContent=e.message;}finally{get('audit-query').disabled=false;}
};
get('load-suggestions').onclick=guarded(suggestions);get('generate-insight').onclick=guarded(async()=>{await action('/api/v1/insights',{target_app:get('target').value},true);await suggestions();});
for(const op of ['archive','restore'])get(op).onclick=guarded(async()=>{if(!get('confirm-archive').checked)throw Error('請先確認可逆資料操作');const before=get('before').value;if(!before)throw Error('請選擇台北時間');const result=await action('/api/v1/events/'+op,{before:before+(before.length===16?':00':'')+'+08:00',confirmed:true},true);await refresh();status.textContent=(op==='archive'?'已封存 ':'已還原 ')+result.changed+' 筆；操作可回復';});
// management-script-end
// Use fetch streaming so Bearer credentials stay in headers, never URL/cookies/storage.
let liveController=null, liveTimer=null, refreshTimer=null, refreshing=false, refreshAgain=false;
const liveStatus=document.getElementById('live-status');
function stopLive(){
 if(liveController)liveController.abort();liveController=null;
 clearTimeout(liveTimer);clearTimeout(refreshTimer);liveTimer=null;refreshTimer=null;refreshAgain=false;
 liveStatus.textContent='即時更新已停止';
}
function scheduleRefresh(){
 if(refreshing){refreshAgain=true;return;}
 if(refreshTimer)return;
 refreshTimer=setTimeout(async()=>{refreshTimer=null;refreshing=true;try{await refresh();}finally{refreshing=false;if(refreshAgain){refreshAgain=false;scheduleRefresh();}}},250);
}
async function connectLive(controller, credential){
 try{
  const res=await fetch('/api/v1/events/stream',{headers:{Authorization:'Bearer '+credential},cache:'no-store',signal:controller.signal});
  if(!res.ok){if([401,403].includes(res.status)){liveStatus.textContent='即時更新驗證失敗';return;}throw Error('SSE HTTP '+res.status);}
  if(!res.headers.get('content-type')?.startsWith('text/event-stream'))throw Error('SSE 格式不正確');
  liveStatus.textContent='即時更新已連線';
  const reader=res.body.getReader(),decoder=new TextDecoder();let buffer='';
  while(!controller.signal.aborted){
   const chunk=await reader.read();if(chunk.done)break;
   buffer+=decoder.decode(chunk.value,{stream:true});
   if(buffer.length>65536)throw Error('SSE 訊息過大');
   let boundary;while((boundary=buffer.indexOf('\n\n'))>=0){
    const frame=buffer.slice(0,boundary);buffer=buffer.slice(boundary+2);
    if(frame.split('\n').some(line=>line.trim()==='event: invalidate'))scheduleRefresh();
   }
  }
 }catch(e){if(controller.signal.aborted)return;}
 if(!controller.signal.aborted){liveStatus.textContent='即時更新中斷，重新連線中…';liveTimer=setTimeout(()=>{liveTimer=null;connectLive(controller,credential);},2000);}
}
function startLive(){stopLive();if(!token.value)return;liveController=new AbortController();connectLive(liveController,token.value);}
token.addEventListener('input',stopLive);
document.getElementById('refresh').addEventListener('click',()=>{startLive();scheduleRefresh();});
document.getElementById('clear').addEventListener('click',()=>{
 stopLive();token.value='';if(get('admin'))get('admin').value='';
 for(const id of ['audit-rows','suggestions','recent-rows','timeline'])get(id)?.replaceChildren();
 if(get('audit-status'))get('audit-status').textContent='尚未查詢';get('total').textContent='—';get('events').textContent='請先驗證';
});
if(get('admin'))get('admin').addEventListener('input',()=>{get('audit-rows').replaceChildren();get('audit-status').textContent='尚未查詢';});
</script></body></html>"##;
    let mut page = template.to_owned();
    if !admin {
        for (a, b) in [
            ("<!-- management-start -->", "<!-- management-end -->"),
            ("// management-script-start", "// management-script-end"),
        ] {
            if let (Some(start), Some(end)) = (page.find(a), page.find(b)) {
                page.replace_range(start..end + b.len(), "");
            }
        }
    }
    page = page
        .replace(
            "{{management-link}}",
            if admin {
                r#"<a href="/">回統計畫面</a>"#
            } else {
                r#"<a href="/admin">管理後台</a>"#
            },
        )
        .replace(
            "{{page-label}}",
            if admin {
                "管理後台"
            } else {
                "統計畫面"
            },
        );
    Html(page)
}
