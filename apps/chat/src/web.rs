//! 用最少依賴提供 Web 操作介面；不在 browser storage 持久保存 API Token。
use axum::response::Html;

pub async fn home() -> Html<&'static str> {
    Html(
        r##"<!doctype html><html lang="zh-Hant"><head>
<meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Rust AI Chat</title>
<style>
:root{font-family:system-ui,sans-serif;color-scheme:light dark}
body{max-width:900px;margin:2rem auto;padding:0 1rem;line-height:1.6}
input,button{padding:.6rem}input{min-width:170px}button{cursor:pointer}
nav,form{display:flex;gap:.5rem;flex-wrap:wrap;margin:1rem 0}
#messages{height:370px;overflow:auto;border:1px solid #8886;border-radius:12px;padding:1rem;list-style:none}
#messages li{padding:.4rem;border-bottom:1px solid #8883;overflow-wrap:anywhere}
#messages small{opacity:.6}#error{color:#c33}
</style></head><body>
<h1>Rust AI Chat</h1><p>持久化聊天與 Outbox 事件基礎</p>
<label>API Token（本頁記憶體，不寫入儲存空間） <input id="token" type="password" autocomplete="off"></label>
<nav><button id="loadRooms">載入房間</button><label>房間 <select id="rooms"></select></label>
<button id="refresh">載入訊息</button></nav>
<form id="createRoom"><input id="roomName" required maxlength="100" placeholder="新房間名稱"><button>建立房間</button></form>
<ul id="messages" aria-live="polite"><li>請先載入房間</li></ul>
<form id="chat"><input id="message" required maxlength="65536" placeholder="輸入訊息" style="flex:1"><button>送出</button></form>
<button id="ai">請 AI 回覆此房間</button><pre id="aiReply" aria-live="polite"></pre>
<p id="error" role="alert"></p>
<script>
'use strict';
const q=(id)=>document.getElementById(id);let active=true;let socket=null;let retry=null;let generation=0;
async function api(path,method='GET',body) {
 const res=await fetch(path,{method,cache:'no-store',headers:{
  'Authorization':'Bearer '+q('token').value,
  ...(body?{'Content-Type':'application/json'}:{})
 },body:body?JSON.stringify(body):undefined});
 if(!res.ok)throw Error('API 錯誤：'+res.status);
 return res.status===204?null:res.json();
}
async function guarded(fn){q('error').textContent='';try{await fn()}catch(e){q('error').textContent=e.message}}
async function rooms(){
 const items=await api('/api/v1/rooms');
 const previous=q('rooms').value;q('rooms').replaceChildren();
 for(const item of items){
  const option=document.createElement('option');option.value=item.id;
  option.textContent=item.name;q('rooms').appendChild(option);
 }
 if(items.some(r=>r.id===previous))q('rooms').value=previous;
 connect();await messages();
}
async function messages(){
 const room=q('rooms').value;if(!room)return;
 const items=await api('/api/v1/messages?room_id='+encodeURIComponent(room));
 q('messages').replaceChildren();
 for(const item of items){
   const li=document.createElement('li');const text=document.createElement('div');
   text.textContent=item.content;li.appendChild(text);
   const label=document.createElement('small');label.textContent=item.id;li.appendChild(label);
   q('messages').appendChild(li);
 }
 q('messages').scrollTop=q('messages').scrollHeight;
}
q('loadRooms').onclick=()=>guarded(rooms);
q('refresh').onclick=()=>guarded(messages);
q('rooms').onchange=()=>{connect();guarded(messages)};
q('createRoom').onsubmit=(e)=>{e.preventDefault();guarded(async()=>{
 const id=crypto.randomUUID();await api('/api/v1/rooms','POST',{id,name:q('roomName').value});
 q('roomName').value='';await rooms();q('rooms').value=id;connect();
})};
q('chat').onsubmit=(e)=>{e.preventDefault();guarded(async()=>{
 if(!q('rooms').value)throw Error('請先建立房間');
 await api('/api/v1/messages','POST',{room_id:q('rooms').value,content:q('message').value});
 q('message').value='';await messages();
})};
function connect(){
 const version=++generation;clearTimeout(retry);if(socket)socket.close();
 const room=q('rooms').value;if(!room||!q('token').value)return;
 const url=new URL('/api/v1/ws/'+encodeURIComponent(room),location.href);
 url.protocol=location.protocol==='https:'?'wss:':'ws:';
 const ws=new WebSocket(url);socket=ws;
 ws.onopen=()=>ws.send(JSON.stringify({token:q('token').value}));
 ws.onmessage=(event)=>{if(version!==generation)return;
  const value=JSON.parse(event.data);
  if(['ready','message','resync_required'].includes(value.type))guarded(messages);
  if(value.type==='error'&&value.code==='unauthorized'){q('error').textContent='認證失敗';generation++;ws.close()}
 };
 ws.onclose=()=>{if(active&&version===generation)retry=setTimeout(connect,1500)};
}
q('ai').onclick=()=>guarded(async()=>{
 q('ai').disabled=true;q('aiReply').textContent='';
 try {
  const res=await fetch('/api/v1/ai/reply',{method:'POST',headers:{'Authorization':'Bearer '+q('token').value,'Content-Type':'application/json'},body:JSON.stringify({room_id:q('rooms').value})});
  if(!res.ok)throw Error(res.status===503?'OpenAI 尚未設定或目前無法使用':'AI 請求失敗：'+res.status);
  const reader=res.body.getReader();const decoder=new TextDecoder();let pending='';
  while(true){const {value,done}=await reader.read();if(done)break;pending+=decoder.decode(value,{stream:true});
   let end;while((end=pending.indexOf('\n\n'))>=0){const frame=pending.slice(0,end);pending=pending.slice(end+2);
    const lines=frame.split('\n');const type=lines.find(l=>l.startsWith('event:'))?.slice(6).trim();
    const data=lines.filter(l=>l.startsWith('data:')).map(l=>l.slice(5).trimStart()).join('\n');
    if(type==='delta')q('aiReply').textContent+=JSON.parse(data).text;
    if(type==='done')await messages();
    if(type==='error')throw Error(data);
   }
  }
 }finally{q('ai').disabled=false}
});
q('token').onchange=connect;
q('message').oninput=()=>{if(socket&&socket.readyState===WebSocket.OPEN)socket.send('{"type":"typing"}')};
window.addEventListener('pagehide',()=>{active=false;clearTimeout(retry);if(socket)socket.close()});
</script></body></html>"##,
    )
}
