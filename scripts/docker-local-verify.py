#!/usr/bin/env python3
"""Verify real local release containers without printing credentials or full inspect data."""
import argparse,hashlib,json,os,pathlib,subprocess,time,urllib.request,urllib.error,uuid
ROOT=pathlib.Path(__file__).resolve().parents[1]
a=argparse.ArgumentParser();a.add_argument('--env-file',default='.env.docker-local');a.add_argument('--project',default='copilot-p9-local');args=a.parse_args()
settings=ROOT/args.env_file
if settings.stat().st_mode&0o077:raise SystemExit('Private settings file must have mode 0600')
values={line.split('=',1)[0]:line.split('=',1)[1]for line in settings.read_text().splitlines()if '='in line and not line.startswith('#')}
compose=['docker','compose','--env-file',str(settings),'-p',args.project]
checks=[]
def command(extra,**kwargs):return subprocess.check_output(compose+extra,cwd=ROOT,**kwargs)
def checked(name,ok=True):assert ok,name;checks.append(name);print('PASS:',name,flush=True)
class NoRedirect(urllib.request.HTTPRedirectHandler):
 def redirect_request(self,*_):return None
client=urllib.request.build_opener(urllib.request.ProxyHandler({}),NoRedirect())
def http(base,path,key=None,body=None):
 r=urllib.request.Request(base+path,data=json.dumps(body).encode()if body is not None else None,headers={'Content-Type':'application/json',**({'Authorization':'Bearer '+key}if key else{})})
 try:
  with client.open(r,timeout=30)as response:
   data=response.read();return response.status,json.loads(data)if data else None
 except urllib.error.HTTPError as e:return e.code,None
 except (urllib.error.URLError,OSError):return 0,None
def wait(test,seconds=45):
 limit=time.monotonic()+seconds
 while not test():
  if time.monotonic()>limit:raise AssertionError('condition timed out')
  time.sleep(.3)
def sql(service,query,db):return command(['exec','-T',service,'psql','-U','bootstrap_owner','-d',db,'-At','-v','ON_ERROR_STOP=1','-c',query],text=True).strip()
chat='http://127.0.0.1:3001';analytics='http://127.0.0.1:3100';ct=values['CHAT_API_TOKEN'];at=values['ANALYTICS_API_TOKEN']
wait(lambda:http(chat,'/healthz')[0]==200 and http(analytics,'/healthz')[0]==200)
checked('both release services ready on localhost')
container_info={}
for service in ('chat','analytics'):
 cid=command(['ps','-q',service],text=True).strip()
 fmt='{{json .Config.User}} {{json .HostConfig.ReadonlyRootfs}} {{json .HostConfig.CapDrop}} {{json .HostConfig.SecurityOpt}} {{json .HostConfig.Memory}} {{json .HostConfig.PidsLimit}}'
 raw=subprocess.check_output(['docker','inspect','--format',fmt,cid],text=True).strip()
 checked(service+' uses non-root, read-only filesystem, dropped capabilities and resource bounds','"65532:65532" true ["ALL"] ["no-new-privileges:true"] 536870912 128'==raw)
 container_info[service]={'image_id':subprocess.check_output(['docker','inspect','--format','{{.Image}}',cid],text=True).strip(),'hardening':raw}
checked('Analytics app database role is not privileged',sql('analytics-db',"SELECT rolsuper,rolcreatedb,rolcreaterole FROM pg_roles WHERE rolname='analytics_user'",'analytics_db')=='f|f|f')
checked('Chat cannot reuse its role in Analytics database',sql('analytics-db',"SELECT count(*) FROM pg_roles WHERE rolname='chat_user'",'analytics_db')=='0')
with client.open(chat+'/pkg/chat_ui_bg.wasm')as response:checked('real WASM hydration asset is shipped',response.read(4)==b'\0asm')
checked('unauthenticated writes rejected',http(chat,'/api/v1/messages',body={'room_id':'forbidden','content':'fixture'})[0]==401)
room='docker-'+uuid.uuid4().hex[:20];assert http(chat,'/api/v1/rooms',ct,{'id':room,'name':'Docker release fixture'})[0]==201
mid=str(uuid.uuid4());body={'id':mid,'room_id':room,'content':'Docker release integration fixture'}
assert http(chat,'/api/v1/messages',ct,body)[0]==201
wait(lambda:http(analytics,'/api/v1/events/web_chat/'+mid,at)[0]==200)
checked('this container Chat message reaches persistent Analytics')
checked('same UUID retry stays idempotent',http(chat,'/api/v1/messages',ct,body)[0]==200)
command(['up','-d','--force-recreate','--no-deps','--wait','chat','analytics']);wait(lambda:http(chat,'/healthz')[0]==200 and http(analytics,'/healthz')[0]==200)
checked('messages survive container recreation',any(row['id']==mid for row in http(chat,'/api/v1/messages?room_id='+room,ct)[1]))
command(['stop','analytics']);delayed=str(uuid.uuid4());assert http(chat,'/api/v1/messages',ct,{'id':delayed,'room_id':room,'content':'durable container outage fixture'})[0]==201
command(['start','analytics']);wait(lambda:http(analytics,'/api/v1/events/web_chat/'+delayed,at)[0]==200)
checked('durable retry recovers from an actual Analytics container outage')
command(['stop','analytics-db']);wait(lambda:http(analytics,'/healthz')[0]!=200)
checked('SQLite Chat stays ready while Analytics PostgreSQL is stopped',http(chat,'/healthz')[0]==200)
queued=str(uuid.uuid4());assert http(chat,'/api/v1/messages',ct,{'id':queued,'room_id':room,'content':'SQLite while PostgreSQL offline'})[0]==201
command(['start','analytics-db']);wait(lambda:http(analytics,'/healthz')[0]==200)
wait(lambda:http(analytics,'/api/v1/events/web_chat/'+queued,at)[0]==200)
checked('SQLite outbox delivers after actual PostgreSQL recovery')
portable='/data/docker-portable-'+uuid.uuid4().hex+'.sqlite3'
command(['exec','-T','chat','/app/server','--backup',portable])
checked('release Chat creates integrity-checked portable SQLite snapshot')
command(['exec','-T','chat','rm',portable])
backup=command(['exec','-T','analytics-db','pg_dump','-U','bootstrap_owner','-d','analytics_db','-Fc'])
fixture='p9_restore_'+uuid.uuid4().hex[:12]
command(['exec','-T','analytics-db','createdb','-U','bootstrap_owner',fixture])
try:
 subprocess.run(compose+['exec','-T','analytics-db','pg_restore','-U','bootstrap_owner','--no-owner','--exit-on-error','-d',fixture],input=backup,check=True,cwd=ROOT)
 query='SELECT source_app,event_id,event_type FROM product_events ORDER BY source_app,event_id'
 checked('container database backup restores exact event rows',sql('analytics-db',query,'analytics_db')==sql('analytics-db',query,fixture))
finally:command(['exec','-T','analytics-db','dropdb','-U','bootstrap_owner',fixture])
report={'status':'passed','checks':checks,'containers':container_info,'backup_sha256':hashlib.sha256(backup).hexdigest(),'scope':'actual local release Docker integration; complete P9 gates remain separate','source_head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'working_tree_dirty':bool(subprocess.check_output(['git','status','--porcelain'],cwd=ROOT,text=True).strip())}
(ROOT/'target/docker-local-validation.json').write_text(json.dumps(report,indent=2)+'\n')
