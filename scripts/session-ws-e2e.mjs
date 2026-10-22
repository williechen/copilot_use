// Actual RFC6455 cookie clients on two processes: shared quota and revoked access.
import assert from 'node:assert/strict';
import net from 'node:net';
import crypto from 'node:crypto';
const base=new URL(process.env.CHAT_URL);assert.equal(base.protocol,'http:');
const token=process.env.CHAT_API_TOKEN;const name='expiry-'+crypto.randomUUID().slice(0,12);const room=name;
const call=async(path,body,credential)=>fetch(new URL(path,base),{method:body?'POST':'GET',headers:{'Content-Type':'application/json','X-Copilot-CSRF':'1',...(credential==='admin'?{Authorization:'Bearer '+token}:{Cookie:credential||''})},body:body?JSON.stringify(body):undefined});
const credentials={username:name,password:'isolated-test-password-'+crypto.randomUUID()};
assert.equal((await call('/api/v1/auth/register',credentials)).status,201);
const login=await call('/api/v1/auth/login',credentials);assert.equal(login.status,200);
const cookie=login.headers.get('set-cookie').split(';')[0];
assert.equal((await call('/api/v1/rooms',{id:room,name:room,is_private:true},cookie)).status,201);
const clients=[];
const until=async fn=>{const limit=Date.now()+10000;while(!fn()){if(Date.now()>limit)throw Error('session socket timed out');await new Promise(r=>setTimeout(r,25));}};
async function connect(endpoint){
 const target=new URL(endpoint);const socket=net.connect(Number(target.port),target.hostname);let frames=Buffer.alloc(0),upgraded=false;const client={socket,messages:[],closed:false};clients.push(client);
 socket.on('close',()=>client.closed=true);socket.on('error',e=>{throw e});
 socket.on('data',part=>{
  frames=Buffer.concat([frames,part]);
  if(!upgraded){const end=frames.indexOf('\r\n\r\n');if(end<0)return;assert.ok(frames.subarray(0,end).toString().startsWith('HTTP/1.1 101'));frames=frames.subarray(end+4);upgraded=true;
   const mask=crypto.randomBytes(4),payload=Buffer.from('{}');for(let i=0;i<payload.length;i++)payload[i]^=mask[i%4];socket.write(Buffer.concat([Buffer.from([0x81,0x80|payload.length]),mask,payload]));
  }
  while(frames.length>=2){let len=frames[1]&127,offset=2;if(len===126){if(frames.length<4)return;len=frames.readUInt16BE(2);offset=4;}assert.notEqual(len,127,'test frame must be bounded');if(frames.length<offset+len)return;
   const opcode=frames[0]&15,payload=frames.subarray(offset,offset+len);frames=frames.subarray(offset+len);
   if(opcode===1)client.messages.push(JSON.parse(payload.toString()));if(opcode===8)socket.end();
  }
 });
 await new Promise(resolve=>socket.once('connect',resolve));
 socket.write(`GET /api/v1/ws/${room} HTTP/1.1\r\nHost: ${target.host}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: ${crypto.randomBytes(16).toString('base64')}\r\nCookie: ${cookie}\r\nOrigin: ${target.origin}\r\n\r\n`);
 await until(()=>client.messages.some(x=>x.type==='ready'||x.type==='error'));return client;
}
try{
 for(let i=0;i<32;i++){const c=await connect(i%2?(process.env.CHAT_URL_SECOND||base):base);assert.ok(c.messages.some(x=>x.type==='ready'));}
 const denied=await connect(process.env.CHAT_URL_SECOND||base);assert.ok(denied.messages.some(x=>x.code==='connection_quota'));assert.ok(!denied.messages.some(x=>x.type==='ready'));
 console.log('PASS: 32 per-user sockets admitted and 33rd rejected across independent processes');
 assert.equal((await call('/api/v1/auth/logout',{},cookie)).status,204);
 const saved=await (await call('/api/v1/messages',{room_id:room,content:'must not reach revoked session'},'admin')).json();
 await until(()=>clients.every(c=>c.closed));assert.ok(clients.every(c=>!c.messages.some(x=>x.id===saved.id)));
 console.log('PASS: all real cookie WebSockets close after logout without receiving subsequent private messages');
}finally{for(const c of clients)c.socket.destroy();}
