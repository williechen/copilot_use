// Real bounded sockets and HTTP load against five isolated Chat processes.
import assert from 'node:assert/strict';import net from 'node:net';import crypto from 'node:crypto';import fs from 'node:fs';
const urls=JSON.parse(process.env.CHAT_LOAD_URLS);assert.equal(urls.length,5);const sockets=[];
async function until(test,seconds=30){const deadline=Date.now()+seconds*1000;while(!await test()){if(Date.now()>deadline)throw Error('bounded load deadline exceeded');await new Promise(r=>setTimeout(r,20));}}
async function api(base,path,cookie,body){return fetch(new URL(path,base),{method:body?'POST':'GET',headers:{'Content-Type':'application/json','X-Copilot-CSRF':'1',Cookie:cookie||''},body:body?JSON.stringify(body):undefined});}
async function connect(base,room,cookie){
 const url=new URL(base),socket=net.connect(Number(url.port),url.hostname),client={socket,messages:[],closed:false,status:0,error:null};sockets.push(client);let data=Buffer.alloc(0),upgraded=false;
 socket.on('close',()=>client.closed=true);socket.on('error',e=>client.error=e);
 socket.on('data',chunk=>{try{data=Buffer.concat([data,chunk]);assert.ok(data.length<1048576);
  if(!upgraded){const end=data.indexOf('\r\n\r\n');if(end<0)return;client.status=Number(data.subarray(0,end).toString().split(' ')[1]);if(client.status!==101){socket.end();return}data=data.subarray(end+4);upgraded=true;const mask=crypto.randomBytes(4),body=Buffer.from('{}');for(let i=0;i<body.length;i++)body[i]^=mask[i%4];socket.write(Buffer.concat([Buffer.from([0x81,0x80|body.length]),mask,body]));}
  while(data.length>=2){let length=data[1]&127,offset=2;if(length===126){if(data.length<4)return;length=data.readUInt16BE(2);offset=4;}if(length===127){if(data.length<10)return;length=Number(data.readBigUInt64BE(2));offset=10;}assert.ok(length<=262144);if(data.length<offset+length)return;const opcode=data[0]&15,body=data.subarray(offset,offset+length);data=data.subarray(offset+length);if(opcode===1)client.messages.push(JSON.parse(body));if(opcode===8)socket.end();}
 }catch(error){client.error=error;socket.destroy();}});
 await new Promise(resolve=>socket.once('connect',resolve));socket.write(`GET /api/v1/ws/${room} HTTP/1.1\r\nHost: ${url.host}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: ${crypto.randomBytes(16).toString('base64')}\r\nCookie: ${cookie}\r\nOrigin: ${url.origin}\r\n\r\n`);
 await until(()=>client.error||client.status&&client.status!==101||client.messages.some(m=>m.type==='ready'||m.type==='error'));if(client.error)throw client.error;return client;
}
const ready=c=>c.messages.some(m=>m.type==='ready');const denied=c=>c.messages.some(m=>m.code==='connection_quota');const users=[],opened=[];const started=performance.now();
try{
 for(let i=0;i<17;i++){const credentials={username:'load_'+crypto.randomUUID().slice(0,12),password:'synthetic-load-password-'+crypto.randomUUID()};assert.equal((await api(urls[0],'/api/v1/auth/register',null,credentials)).status,201);const response=await api(urls[0],'/api/v1/auth/login',null,credentials);assert.equal(response.status,200);users.push(response.headers.get('set-cookie').split(';')[0]);}
 for(let user=0;user<16;user++){for(let j=0;j<32;j++){const room=crypto.randomUUID(),server=j%4;assert.equal((await api(urls[server],'/api/v1/rooms',users[user],{id:room,name:'Synthetic bounded load room'})).status,201);const client=await connect(urls[server],room,users[user]);assert.ok(ready(client));opened.push({client,room,user,server});}}
 assert.ok(opened.every(({client})=>!client.closed&&!client.error));
 const extra=crypto.randomUUID();assert.equal((await api(urls[4],'/api/v1/rooms',users[16],{id:extra,name:'Synthetic quota boundary room'})).status,201);
 const global=await connect(urls[4],extra,users[16]);assert.ok(denied(global)&&!ready(global));
 const released=opened.find(x=>x.user===1&&x.server===1);released.client.socket.destroy();await until(async()=>{const r=await api(urls[1],`/api/v1/rooms/${released.room}/presence`,users[1]);return r.status===200&&(await r.json()).length===0;});
 const perUser=await connect(urls[4],opened[0].room,users[0]);assert.ok(denied(perUser)&&!ready(perUser));
 const perProcess=await connect(urls[0],extra,users[16]);assert.equal(perProcess.status,429);
 const restored=await connect(urls[1],released.room,users[1]);assert.ok(ready(restored));
 console.log('PASS: 512 real sockets across four processes; fifth-process global, per-user and per-process boundaries reject excess');
 for(const socket of sockets)socket.socket.destroy();await until(()=>sockets.every(s=>s.closed));
 const subscribers=[];for(let i=0;i<32;i++){const c=await connect(urls[i%4],extra,users[16]);assert.ok(ready(c));subscribers.push(c);}
 const ids=Array.from({length:64},()=>crypto.randomUUID());const latency=[];
 await Promise.all(ids.map(async(id,i)=>{const start=performance.now();const r=await api(urls[i%4],'/api/v1/messages',users[16],{id,room_id:extra,content:'Synthetic bounded fan-out fixture '+i});assert.equal(r.status,201);latency.push(performance.now()-start);}));
 await until(()=>subscribers.every(s=>ids.every(id=>s.messages.some(m=>m.id===id))),60);
 for(const s of subscribers){for(const id of ids)assert.equal(s.messages.filter(m=>m.id===id).length,1);}
 const history=await(await api(urls[0],`/api/v1/messages?room_id=${extra}`,users[16])).json();assert.equal(history.length,64);assert.equal(new Set(history.map(m=>m.id)).size,64);
 const analytics=process.env.ANALYTICS_URL;const pending=new Set(ids);
 await until(async()=>{for(const id of [...pending]){const response=await fetch(new URL(`/api/v1/events/web_chat/${id}`,analytics),{headers:{Authorization:'Bearer '+process.env.ANALYTICS_API_TOKEN}});if(response.status===200)pending.delete(id);else assert.equal(response.status,404);}return pending.size===0;},60);
 latency.sort((a,b)=>a-b);const report={status:'passed',peak_sockets:512,processes:5,fanout_subscribers:32,concurrent_messages:64,exactly_once_observed:true,p50_ms:latency[31],p95_ms:latency[60],max_ms:latency.at(-1),elapsed_ms:performance.now()-started,scope:'local synthetic bounded admission, fan-out, persistence and exact Analytics IDs'};
 fs.writeFileSync(process.env.CHAT_LOAD_REPORT,JSON.stringify(report,null,2)+'\n');console.log('PASS: 64 concurrent private writes delivered once to all 32 sockets, durable history and Analytics IDs');console.log(JSON.stringify(report));
}finally{for(const s of sockets)s.socket.destroy();}
