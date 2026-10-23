import json,pathlib,subprocess,os,time,socket,requests,uuid
root=pathlib.Path(__file__).resolve().parent
rows=[]
def record(name,actual,expected):
 rows.append(dict(name=name,actual=actual,expected=expected,passed=actual==expected))
def req(method,path,**kw):return requests.request(method,'http://127.0.0.1:3100'+path,timeout=10,allow_redirects=False,**kw)
with socket.socket() as s:
 if s.connect_ex(('127.0.0.1',3100))==0:raise RuntimeError('port in use')
env={k:os.environ[k] for k in ['HOME','PATH'] if k in os.environ}
env.update(ANALYTICS_DATABASE_URL='postgres://validation_audit@127.0.0.1:55430/postgres',ANALYTICS_CHAT_TOKEN='audit-chat-token',ANALYTICS_PET_TOKEN='audit-pet-token')
log=open(root/'logs/runtime-gap-server.log','w')
p=subprocess.Popen(['/home/willie/Documents/Codex/2026-10-23/referenced-chatgpt-conversation-this-is-an-2/work/target-full/debug/stats-server'],cwd=root/'analytics-runtime',env=env,stdout=log,stderr=subprocess.STDOUT)
try:
 for _ in range(100):
  try:
   if req('GET','/healthz').status_code==200:break
  except requests.ConnectionError:time.sleep(.1)
 record('healthz',req('GET','/healthz').status_code,200)
 token={'Authorization':'Bearer audit-chat-token'}
 event={'source_app':'web_chat','event_id':str(uuid.uuid4()),'schema_version':1,'event_type':'chat.audit'}
 record('unauthenticated ingest',req('POST','/api/v1/events',json=event).status_code,401)
 record('authenticated ingest',req('POST','/api/v1/events',headers=token,json=event).status_code,201)
 record('duplicate ingest',req('POST','/api/v1/events',headers=token,json=event).status_code,200)
 record('identity conflict',req('POST','/api/v1/events',headers=token,json=event|{'event_type':'other.kind'}).status_code,409)
 record('unsupported version',req('POST','/api/v1/events',headers=token,json=event|{'schema_version':2}).status_code,400)
 record('unknown field',req('POST','/api/v1/events',headers=token,json=event|{'extra':True}).status_code,422)
 record('malformed JSON',req('POST','/api/v1/events',headers=token|{'Content-Type':'application/json'},data='{').status_code,400)
 record('oversized JSON',req('POST','/api/v1/events',headers=token,json=event|{'event_type':'x'*5000}).status_code,413)
 origin={'Origin':'http://127.0.0.1:3100'}
 failed=[req('POST','/login',headers=origin,data={'username':'admin','password':'wrong-password'}).status_code for _ in range(11)]
 record('Day20 legacy rate-limit expectation after 11 bad logins',failed[-1],429)
 r=req('POST','/login',headers=origin,data={'username':'admin','password':'validation-new-password-456'})
 record('ready account login',r.status_code,303)
 h=origin|{'Cookie':r.headers['Set-Cookie'].split(';')[0]}
 record('Day14 event-kind summary route',req('GET','/api/v1/events/web_chat/summary/chat.audit',headers=h).status_code,200)
 record('production source summary',req('GET','/api/v1/events/web_chat/summary',headers=h).status_code,200)
finally:
 p.terminate();p.wait(timeout=5);log.close()
 (root/'logs/runtime-gap-results.json').write_text(json.dumps({'source_sha':'9c2b7cf5a4c60a08254445911916c4ed9b02d7b3','checks':rows},indent=2))
print(json.dumps(rows,indent=2))
