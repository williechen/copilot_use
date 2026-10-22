// Real cookie HTTP and durable cross-service checks using disposable synthetic users only.
import {randomUUID} from 'node:crypto';
import {readFile,writeFile} from 'node:fs/promises';
const chat=process.env.CHAT_URL,analytics=process.env.ANALYTICS_URL,path=process.env.VERIFY_TOPIC_FIXTURE;
for(const base of [chat,analytics]){const u=new URL(base);if(!['127.0.0.1','localhost','[::1]'].includes(u.hostname))throw Error('isolated localhost fixture required');}
const assert=(ok,label)=>{if(!ok)throw Error(label);};
async function request(base,path,body,headers={}){const r=await fetch(base+path,{method:body===undefined?'GET':'POST',redirect:'error',headers:{'Content-Type':'application/json','X-Copilot-Csrf':'1',...headers},body:body===undefined?undefined:JSON.stringify(body)});return{status:r.status,value:await r.json().catch(()=>null),headers:r.headers};}
async function wait(test){const limit=Date.now()+20000;while(!await test()){if(Date.now()>limit)throw Error('topic condition timeout');await new Promise(r=>setTimeout(r,100));}}
const summary=async()=>{const r=await request(analytics,'/api/v1/events/summary',undefined,{Authorization:'Bearer '+process.env.ANALYTICS_API_TOKEN});assert(r.status===200,'summary authenticated');return Object.fromEntries(r.value.filter(v=>v.source_app==='web_chat'&&v.event_type.startsWith('keyword.')).map(v=>[v.event_type,v.total]));};
if(process.argv.includes('--withdraw')){
 const f=JSON.parse(await readFile(path,'utf8'));const r=await request(chat,`/api/v1/rooms/${f.room}/topic-consent`,{enabled:false},{Cookie:f.cookie});assert(r.status===200&&!r.value.enabled&&r.value.withdrawal_pending>0,'offline Analytics withdrawal remains pending durably');console.log('PASS: consent disabled while Analytics is offline; exact prior topic IDs await acknowledgement');
}else if(process.argv.includes('--verify-withdrawal')){
 const f=JSON.parse(await readFile(path,'utf8'));await wait(async()=>{const r=await request(chat,`/api/v1/rooms/${f.room}/topic-consent`,undefined,{Cookie:f.cookie});return r.status===200&&!r.value.enabled&&r.value.withdrawal_pending===0;});
 const totals=await summary();for(const key of ['keyword.rust','keyword.ownership'])assert((totals[key]??0)===(f.before[key]??0),'withdrawn vocabulary excluded');
 console.log('PASS: service recovery acknowledges withdrawal and removes exact opted-in vocabulary counts');
}else{
 async function user(){const username='topic_'+randomUUID().replaceAll('-','').slice(0,20),password='fixture-'+randomUUID();assert((await request(chat,'/api/v1/auth/register',{username,password})).status===201,'synthetic user registered');const login=await request(chat,'/api/v1/auth/login',{username,password});assert(login.status===200,'synthetic user logged in');return login.headers.get('set-cookie').split(';')[0];}
 const cookie=await user(),foreign=await user(),room='topics-'+randomUUID().replaceAll('-','').slice(0,24);
 assert((await request(chat,'/api/v1/rooms',{id:room,name:'Synthetic topic consent fixture',is_private:true},{Cookie:cookie})).status===201,'owned room created');
 const endpoint=`/api/v1/rooms/${room}/topic-consent`,initial=await request(chat,endpoint,undefined,{Cookie:cookie});assert(initial.status===200&&initial.value.enabled===false&&initial.headers.get('cache-control')==='no-store','explicit default-off private consent');
 assert([403,404].includes((await request(chat,endpoint,{enabled:true},{Cookie:foreign})).status),'foreign room cannot grant consent');
 const before=await summary();
 async function send(content,id=randomUUID()){const body={id,room_id:room,content};assert([200,201].includes((await request(chat,'/api/v1/messages',body,{Cookie:cookie})).status),'message persisted');await wait(async()=>(await request(analytics,'/api/v1/events/web_chat/'+id,undefined,{Authorization:'Bearer '+process.env.ANALYTICS_API_TOKEN})).status===200);return body;}
 await send('Rust rust ownership');assert(JSON.stringify(await summary())===JSON.stringify(before),'no counting before consent');
 assert((await request(chat,endpoint,{enabled:true},{Cookie:cookie})).value.enabled===true,'explicit consent saved');
 const body=await send('Rust rust ownership');await wait(async()=>{const totals=await summary();return(totals['keyword.rust']??0)===(before['keyword.rust']??0)+2&&(totals['keyword.ownership']??0)===(before['keyword.ownership']??0)+1;});
 assert((await request(chat,'/api/v1/messages',body,{Cookie:cookie})).status===200,'message retry idempotent');
 await send('Rust token=synthetic-secret');const totals=await summary();assert((totals['keyword.rust']??0)===(before['keyword.rust']??0)+2,'credential-like content excluded and retry does not duplicate topics');
 const recent=await request(analytics,'/api/v1/events/recent?source_app=web_chat&event_type=keyword.rust',undefined,{Authorization:'Bearer '+process.env.ANALYTICS_API_TOKEN});assert(recent.status===200&&recent.value.length>=2&&recent.value.length<=100&&recent.value.every(row=>row.event_type==='keyword.rust'&&row.event_id&&Number.isInteger(row.received_unix_ms)&&!('content'in row)),'real bounded anonymous event details and filter');
 await writeFile(path,JSON.stringify({cookie,room,before}),{mode:0o600});
 console.log('PASS: real default-off/own-room consent, fixed-word occurrence counts, sensitive filtering and idempotent retry');
}
