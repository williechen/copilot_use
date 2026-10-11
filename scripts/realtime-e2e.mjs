// Node 22+ 原生 WebSocket：兩客戶端、房間隔離、認證、typing、history 恢復。
import assert from 'node:assert/strict';
const base = process.env.CHAT_URL || 'http://127.0.0.1:3001';
const token = process.env.CHAT_API_TOKEN;
const suffix = crypto.randomUUID();
const rooms = [`ws-${suffix.slice(0,20)}`, `isolated-${suffix.slice(0,20)}`];
const api = async (path, body) => {
  const r = await fetch(base + path, {method: body ? 'POST' : 'GET',
    headers: {Authorization: `Bearer ${token}`, 'Content-Type':'application/json'},
    body: body ? JSON.stringify(body) : undefined});
  assert.ok(r.ok, `HTTP ${r.status}`);
  return r.status === 201 && path.endsWith('/rooms') ? null : r.json();
};
for (const id of rooms) await api('/api/v1/rooms', {id, name:id});
const clients=[];
async function connect(room, credential=token, endpoint=base) {
  const ws = new WebSocket(endpoint.replace(/^http/, 'ws') + '/api/v1/ws/' + room);
  const events=[];
  ws.addEventListener('message', e => events.push(JSON.parse(e.data)));
  clients.push(ws);
  await new Promise((resolve,reject)=>{
    const timer=setTimeout(()=>reject(Error('WS connect timeout')),5000);
    ws.addEventListener('open',()=>{clearTimeout(timer);ws.send(JSON.stringify({token:credential}));resolve()}, {once:true});
    ws.addEventListener('error',()=>{clearTimeout(timer);reject(Error('WS error'))},{once:true});
  });
  return {ws,events};
}
async function until(predicate) {
  const limit=Date.now()+6000;
  while(Date.now()<limit){if(predicate())return;await new Promise(r=>setTimeout(r,30));}
  throw Error('WS event timeout');
}
try {
  const denied=await connect(rooms[0],'invalid');
  await until(()=>denied.events.some(e=>e.code==='unauthorized'));
  assert.ok(!denied.events.some(e=>e.type==='ready'));
  const a=await connect(rooms[0]), b=await connect(rooms[0],token,process.env.CHAT_URL_SECOND||base), other=await connect(rooms[1]);
  await until(()=>[a,b,other].every(c=>c.events.some(e=>e.type==='ready')));
  assert.ok(b.events.find(e=>e.type==='ready').presence.length>=2,'presence snapshot');
  a.ws.send('{"type":"typing"}');
  await until(()=>b.events.some(e=>e.type==='typing'));
  const saved=await api('/api/v1/messages',{room_id:rooms[0],content:'WS integration test'});
  await until(()=>[a,b].every(c=>c.events.some(e=>e.type==='message' && e.id===saved.id)));
  assert.ok(!other.events.some(e=>e.type==='message' && e.id===saved.id));
  b.ws.close();
  const missed=await api('/api/v1/messages',{room_id:rooms[0],content:'missed while disconnected'});
  const back=await connect(rooms[0]);
  await until(()=>back.events.some(e=>e.type==='ready'));
  const history=await api('/api/v1/messages?room_id='+rooms[0]);
  assert.ok(history.some(m=>m.id===missed.id));
  console.log('PASS: authenticated WS, presence snapshot, cross-process clients, room isolation, typing, reconnect/history');
} finally {for(const ws of clients) ws.close();}
