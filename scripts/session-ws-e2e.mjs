// Real HTTP session and RFC6455 socket: logout must prevent further room delivery.
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
assert.equal((await call('/api/v1/rooms',{id:room,name:room},cookie)).status,201);
const socket=net.connect(Number(base.port),base.hostname);let frames=Buffer.alloc(0),upgraded=false,closed=false;const messages=[];
socket.on('close',()=>closed=true);socket.on('error',e=>{throw e});
socket.on('data',part=>{
 frames=Buffer.concat([frames,part]);
 if(!upgraded){const end=frames.indexOf('\r\n\r\n');if(end<0)return;assert.ok(frames.subarray(0,end).toString().startsWith('HTTP/1.1 101'));frames=frames.subarray(end+4);upgraded=true;
  const mask=crypto.randomBytes(4),payload=Buffer.from('{}');for(let i=0;i<payload.length;i++)payload[i]^=mask[i%4];socket.write(Buffer.concat([Buffer.from([0x81,0x80|payload.length]),mask,payload]));
 }
 while(frames.length>=2){let len=frames[1]&127,offset=2;if(len===126){if(frames.length<4)return;len=frames.readUInt16BE(2);offset=4;}assert.notEqual(len,127,'test frame must be bounded');if(frames.length<offset+len)return;
  const opcode=frames[0]&15,payload=frames.subarray(offset,offset+len);frames=frames.subarray(offset+len);
  if(opcode===1)messages.push(JSON.parse(payload.toString()));if(opcode===8)socket.end();
 }
});
await new Promise(resolve=>socket.once('connect',resolve));
socket.write(`GET /api/v1/ws/${room} HTTP/1.1\r\nHost: ${base.host}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: ${crypto.randomBytes(16).toString('base64')}\r\nCookie: ${cookie}\r\nOrigin: ${base.origin}\r\n\r\n`);
const until=async fn=>{const limit=Date.now()+6500;while(!fn()){if(Date.now()>limit)throw Error('session socket timed out');await new Promise(r=>setTimeout(r,25));}};
try{
 await until(()=>messages.some(x=>x.type==='ready'));
 assert.equal((await call('/api/v1/auth/logout',{},cookie)).status,204);
 const saved=await (await call('/api/v1/messages',{room_id:room,content:'must not reach revoked session'},'admin')).json();
 await until(()=>closed);assert.ok(!messages.some(x=>x.id===saved.id));
 console.log('PASS: real cookie WebSocket closes after logout and cannot receive a subsequent private message');
}finally{socket.destroy();}
