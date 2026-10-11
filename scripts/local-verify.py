#!/usr/bin/env python3
"""建立全新隔離 PostgreSQL，以真實 binary/API 驗證；結束時移除測試庫。"""
import json, uuid, os, pathlib, secrets, signal, socket, subprocess, sys, tempfile, time, urllib.request, urllib.error, threading
from http.server import ThreadingHTTPServer,BaseHTTPRequestHandler
ROOT = pathlib.Path(__file__).resolve().parents[1]
PG = pathlib.Path(subprocess.check_output(['pg_config','--bindir'],text=True).strip())
REPORT = pathlib.Path(os.environ.get('VERIFY_REPORT', ROOT / 'target/local-validation.json'))
live_key=os.environ.get('OPENAI_API_KEY') if any(flag in sys.argv for flag in ('--live-openai','--live-insights','--browser-live')) else None
if any(flag in sys.argv for flag in ('--live-openai','--live-insights','--browser-live')) and not live_key: raise SystemExit('OPENAI_API_KEY not configured')
def interrupted(*_): raise KeyboardInterrupt
signal.signal(signal.SIGTERM,interrupted)
results=[]
processes=[]

def run(argv, **kw):
    subprocess.run([str(a) for a in argv], check=True, cwd=ROOT, **kw)

def port():
    with socket.socket() as s:
        s.bind(('127.0.0.1',0)); return s.getsockname()[1]

def check(label, ok=True):
    assert ok, label
    results.append(label)
    print('PASS:',label,flush=True)

def http(base,path,token=None,body=None,method=None):
    req=urllib.request.Request(base+path, data=json.dumps(body).encode() if body is not None else None,
        headers={'Content-Type':'application/json', **({'Authorization':'Bearer '+token} if token else {})},method=method)
    try:
        with urllib.request.urlopen(req,timeout=120) as r:
            data=r.read(); return r.status, json.loads(data) if data else None
    except urllib.error.HTTPError as e:
        return e.code,None

def wait(fn):
    limit=time.monotonic()+20
    while time.monotonic()<limit:
        try:
            if fn(): return
        except (OSError,urllib.error.URLError): pass
        time.sleep(.2)
    raise AssertionError('condition timed out')

def stop(proc):
    if proc.poll() is None:
        proc.terminate()
        try: proc.wait(10)
        except subprocess.TimeoutExpired: proc.kill();proc.wait()

with tempfile.TemporaryDirectory(prefix='copilot-verify-') as scratch:
    d=pathlib.Path(scratch); dbport=port(); chatport=port(); analyticsport=port()
    admin=f'postgresql://{os.environ.get("USER","willie")}@127.0.0.1:{dbport}/postgres'
    common=os.environ.copy(); common['DATABASE_URL']=admin
    common.pop('OPENAI_API_KEY',None)  # 基礎可靠性測試不觸發外部付費呼叫。
    chat_token=secrets.token_urlsafe(32); analytics_token=secrets.token_urlsafe(32)
    ingest_token=secrets.token_urlsafe(32); pet_token=secrets.token_urlsafe(32)
    chat_url=f'http://127.0.0.1:{chatport}'; analytics_url=f'http://127.0.0.1:{analyticsport}'
    env=common | {'CHAT_API_TOKEN':chat_token,'ANALYTICS_API_TOKEN':analytics_token,
        'ANALYTICS_ADMIN_TOKEN':secrets.token_urlsafe(32),'ANALYTICS_CHAT_TOKEN':ingest_token,'ANALYTICS_PET_TOKEN':pet_token,
        'CHAT_URL':chat_url,'ANALYTICS_URL':analytics_url,
        'CHAT_LISTEN':f'127.0.0.1:{chatport}','ANALYTICS_LISTEN':f'127.0.0.1:{analyticsport}',
        'ANALYTICS_ENDPOINT':analytics_url+'/api/v1/events'}
    observed=[];observed_headers=[]
    class Collector(BaseHTTPRequestHandler):
        def log_message(self,*_):pass
        def do_POST(self):
            length=int(self.headers.get('Content-Length','0'))
            if length>2000000:self.send_error(413);return
            payload=self.rfile.read(length)
            observed.append((self.path,payload))
            observed_headers.append(dict(self.headers))
            self.send_response(200);self.send_header('Content-Type','application/json');self.end_headers();self.wfile.write(b'{}')
    collector=ThreadingHTTPServer(('127.0.0.1',0),Collector)
    collector_thread=threading.Thread(target=collector.serve_forever,daemon=True);collector_thread.start()
    env['COPILOT_LOCAL_OTLP_ENDPOINT']=f'http://127.0.0.1:{collector.server_port}'
    env['OTEL_EXPORTER_OTLP_HEADERS']='Authorization=metadata-synthetic-secret,x-private=metadata-synthetic-secret'
    for signal_name in ('TRACES','METRICS','LOGS'):
        env['OTEL_EXPORTER_OTLP_'+signal_name+'_HEADERS']='Authorization=metadata-synthetic-secret'
    env['OTEL_BSP_SCHEDULE_DELAY']='100';env['OTEL_BLRP_SCHEDULE_DELAY']='100';env['OTEL_METRIC_EXPORT_INTERVAL']='250'
    started=False
    try:
        run([PG/'initdb','-D',d/'data','-A','trust','--no-locale','-E','UTF8'],stdout=subprocess.DEVNULL)
        run([PG/'pg_ctl','-D',d/'data','-l',d/'pg.log','-o',f'-h 127.0.0.1 -p {dbport} -k {scratch}','start'],stdout=subprocess.DEVNULL)
        started=True
        run([PG/'psql',admin.rsplit('/',1)[0]+'/template1','-v','ON_ERROR_STOP=1','-c','CREATE EXTENSION IF NOT EXISTS vector'],stdout=subprocess.DEVNULL)
        run(['cargo','fmt','--all','--','--check'])
        run(['cargo','clippy','--workspace','--all-targets','--locked','--','-D','warnings'])
        run(['cargo','test','--workspace','--all-targets','--locked'],env=common)
        run(['cargo','build','--workspace','--locked'])
        check('fmt, strict clippy, workspace unit/PostgreSQL tests, binaries')
        # 應用使用各自 non-superuser role；測試 admin 只用於新建測試庫及備份。
        for app in ('chat','analytics'):
            password=secrets.token_hex(24)
            sql=f"CREATE ROLE {app}_test LOGIN PASSWORD '{password}'; CREATE DATABASE {app}_test OWNER {app}_test;"
            run([PG/'psql',admin,'-v','ON_ERROR_STOP=1','-c',sql.split(';')[0]],stdout=subprocess.DEVNULL)
            run([PG/'psql',admin,'-v','ON_ERROR_STOP=1','-c',sql.split(';')[1]],stdout=subprocess.DEVNULL)
            env[app.upper()+'_DATABASE_URL']=f'postgresql://{app}_test:{password}@127.0.0.1:{dbport}/{app}_test'
        handles=[]
        def start(app):
            log=open(d/f'{app}.log','a');handles.append(log)
            proc=subprocess.Popen([ROOT/'target/debug'/f'day9-{app}'],cwd=ROOT,env=env,stdout=log,stderr=log)
            processes.append(proc)
            url=chat_url if app=='chat' else analytics_url
            wait(lambda:http(url,'/healthz')[0]==200)
            return proc
        chat=start('chat'); analytics=start('analytics')
        check('empty-database migrations and two independently running services')
        abuse_headers={'Content-Type':'application/json','X-Copilot-Csrf':'1'}
        def rejected_login(base):
            req=urllib.request.Request(base+'/api/v1/auth/login',data=json.dumps({'username':'missing_abuse_fixture','password':'wrong-synthetic-password'}).encode(),headers=abuse_headers)
            try:
                with urllib.request.urlopen(req,timeout=10)as response:return response.status
            except urllib.error.HTTPError as e:return e.code
        check('sequential unknown-account attempts bounded before expensive password work',all(rejected_login(chat_url)==401 for _ in range(8)) and rejected_login(chat_url)==429)

        # 跨角色隔離不能只看資料庫名稱，實際拒絕 CONNECT。
        for app in ('chat','analytics'):
            run([PG/'psql',admin,'-v','ON_ERROR_STOP=1','-c',f'REVOKE CONNECT ON DATABASE {app}_test FROM PUBLIC'],stdout=subprocess.DEVNULL)
        wrong=env['CHAT_DATABASE_URL'].rsplit('/',1)[0]+'/analytics_test'
        denied=subprocess.run([PG/'psql',wrong,'-c','SELECT 1'],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
        check('Chat database role cannot CONNECT to Analytics database',denied.returncode!=0)
        check('unauthorized Chat write rejected',http(chat_url,'/api/v1/rooms',body={'id':'bad','name':'bad'})[0]==401)
        check('unauthorized Analytics read rejected',http(analytics_url,'/api/v1/events/count')[0]==401)
        def mcp_request(method,params=None,extra=None):
            params=params or {};params['_meta']={'io.modelcontextprotocol/protocolVersion':'2026-07-28','io.modelcontextprotocol/clientInfo':{'name':'local-verify','version':'0.1'},'io.modelcontextprotocol/clientCapabilities':{}}
            headers={'Content-Type':'application/json','Authorization':'Bearer '+chat_token,'MCP-Protocol-Version':'2026-07-28','Mcp-Method':method,'Accept':'application/json, text/event-stream'}
            if 'name' in params:headers['Mcp-Name']=params['name']
            headers.update(extra or {})
            request=urllib.request.Request(chat_url+'/api/v1/mcp',data=json.dumps({'jsonrpc':'2.0','id':1,'method':method,'params':params}).encode(),headers=headers)
            try:
                with urllib.request.urlopen(request,timeout=120) as response:return response.status,json.loads(response.read())
            except urllib.error.HTTPError as e:return e.code,None
        check('actual MCP modern server discovery',mcp_request('server/discover')[1]['result']['supportedVersions']==['2026-07-28'])
        check('MCP catalog exposes only authorized readonly knowledge tool',mcp_request('tools/list')[1]['result']['tools'][0]['name']=='knowledge_search')
        check('MCP rejects method/header mismatch',mcp_request('tools/list',extra={'Mcp-Method':'tools/call'})[0]==400)
        check('MCP rejects untrusted browser Origin',mcp_request('tools/list',extra={'Origin':'https://untrusted.invalid'})[0]==403)
        check('MCP cannot invoke file or arbitrary execution tools',mcp_request('tools/call',{'name':'delete_file','arguments':{'query':'anything'}})[0]==400)
        room='test-'+secrets.token_hex(8)
        check('unconfigured AI returns 503 without mock output',http(chat_url,'/api/v1/ai/reply',chat_token,{'room_id':room})[0]==503)
        check('room created',http(chat_url,'/api/v1/rooms',chat_token,{'id':room,'name':'本地測試'})[0]==201)
        status,message=http(chat_url,'/api/v1/messages',chat_token,{'room_id':room,'content':'transactional test'})
        check('message persisted',status==201)
        check('nil message replay ID rejected before persistence',http(chat_url,'/api/v1/messages',chat_token,{'id':'00000000-0000-0000-0000-000000000000','room_id':room,'content':'invalid id'})[0]==400)
        event={'schema_version':1,'event_id':message['id'],'source_app':'web_chat','event_type':'message.persisted'}
        lookup='/api/v1/events/web_chat/'+message['id']
        wait(lambda:http(analytics_url,lookup,analytics_token)[0]==200)
        check('this exact Chat message traversed Outbox to Analytics')
        before=http(analytics_url,'/api/v1/events/count',analytics_token)[1]['total']
        for _ in range(3):
            assert http(analytics_url,'/api/v1/events',ingest_token,event)[0]==202
        check('repeated event counted once',http(analytics_url,'/api/v1/events/count',analytics_token)[1]['total']==before)
        check('sensitive/unknown payload field rejected',http(analytics_url,'/api/v1/events',ingest_token,event|{'content':'private'})[0]==422)
        check('Chat credential cannot impersonate Pet',http(analytics_url,'/api/v1/events',ingest_token,event|{'source_app':'desktop_pet','event_type':'file.cleanup.succeeded'})[0]==401)
        check('read credential cannot ingest',http(analytics_url,'/api/v1/events',analytics_token,event)[0]==401)
        check('invalid schema rejected',http(analytics_url,'/api/v1/events',ingest_token,event|{'schema_version':99})[0]==400)
        def account(name):
            payload={'username':name,'password':secrets.token_urlsafe(24)}
            req=urllib.request.Request(chat_url+'/api/v1/auth/register',data=json.dumps(payload).encode(),headers={'Content-Type':'application/json','X-Copilot-CSRF':'1'})
            with urllib.request.urlopen(req) as response: assert response.status==201
            req=urllib.request.Request(chat_url+'/api/v1/auth/login',data=json.dumps(payload).encode(),headers={'Content-Type':'application/json','X-Copilot-CSRF':'1'})
            with urllib.request.urlopen(req) as response:
                cookie=response.headers['Set-Cookie']; assert 'HttpOnly' in cookie and 'SameSite=Strict' in cookie
                return cookie.split(';')[0]
        def member_http(cookie,path,body=None,csrf=True):
            req=urllib.request.Request(chat_url+path,data=json.dumps(body).encode() if body is not None else None,headers={'Content-Type':'application/json','Cookie':cookie,**({'X-Copilot-CSRF':'1'} if csrf else {})})
            try:
                with urllib.request.urlopen(req,timeout=120) as response:
                    text=response.read();return response.status,json.loads(text) if text else None
            except urllib.error.HTTPError as e:return e.code,None
        alice_name='alice_'+secrets.token_hex(5);bob_name='bob_'+secrets.token_hex(5)
        alice=account(alice_name);bob=account(bob_name);private='private-'+secrets.token_hex(5)
        check('session-authenticated owner creates private room',member_http(alice,'/api/v1/rooms',{'id':private,'name':'私人測試'})[0]==201)
        check('uninvited user cannot read private history',member_http(bob,'/api/v1/messages?room_id='+private)[0]==403)
        check('uninvited user cannot write private message',member_http(bob,'/api/v1/messages',{'room_id':private,'content':'denied'})[0]==403)
        check('session writes require CSRF header',member_http(alice,'/api/v1/messages',{'room_id':private,'content':'csrf'},csrf=False)[0]==403)
        check('owner can invite another user',member_http(alice,'/api/v1/rooms/invite',{'room_id':private,'username':bob_name})[0]==204)
        check('invited user can send private message',member_http(bob,'/api/v1/messages',{'room_id':private,'content':'authorized'})[0]==201)
        persona_route='/api/v1/rooms/'+private+'/personas'
        persona_id=str(uuid.uuid4());persona_body={'id':persona_id,'name':'老師','instructions':'協助學習 Rust，只使用授權資料。','memory':'','cooldown_seconds':2,'enabled':True}
        check('room member cannot change owner personas',member_http(bob,persona_route,persona_body)[0]==403)
        check('owner can configure personas',member_http(alice,persona_route,persona_body)[0]==204)
        check('persona configuration persists with scoped memory',member_http(alice,persona_route)[1][0]['id']==persona_id)
        run_route='/api/v1/rooms/'+private+'/persona-runs'
        bounded={'id':str(uuid.uuid4()),'topic':'只討論測試資料','max_turns':2,'max_output_budget':1024,'tools':False,'confirmed':True}
        check('persona runs reject unconfirmed requests',member_http(alice,run_route,bounded|{'confirmed':False})[0]==400)
        check('persona turn budget rejects excessive rounds',member_http(alice,run_route,bounded|{'max_turns':99})[0]==400)
        check('persona run with no provider configuration fails without a fake answer',member_http(alice,run_route,bounded)[0]==503)
        assert member_http(bob,'/api/v1/auth/logout',{})[0]==204

        check('logout revokes server-side session',member_http(bob,'/api/v1/messages?room_id='+private)[0]==401)
        wait(lambda:any(row['event_type']=='login.succeeded' and row['total']>=2 for row in http(analytics_url,'/api/v1/events/summary',analytics_token)[1]))
        wait(lambda:any(row['event_type']=='logout.succeeded' and row['total']>=1 for row in http(analytics_url,'/api/v1/events/summary',analytics_token)[1]))
        check('successful login/logout emits durable anonymous Analytics events')

        second_port=port();second_url=f'http://127.0.0.1:{second_port}'
        second_env=env|{'CHAT_LISTEN':f'127.0.0.1:{second_port}'}
        second_log=open(d/'second-chat.log','a');handles.append(second_log)
        second_chat=subprocess.Popen([ROOT/'target/debug/day9-chat'],cwd=ROOT,env=second_env,stdout=second_log,stderr=second_log);processes.append(second_chat)
        wait(lambda:http(second_url,'/healthz')[0]==200)
        run(['node','scripts/realtime-e2e.mjs'],env=env|{'CHAT_URL_SECOND':second_url})
        check('two independent Chat processes share presence, typing and canonical message fan-out')
        check('real WebSocket two-client and room-isolation E2E')
        run(['node','scripts/session-ws-e2e.mjs'],env=env|{'CHAT_URL_SECOND':second_url})
        stop(second_chat)
        check('revoked cookie session cannot retain WebSocket access')
        # Seed reviewable, non-AI fixture to prove the real PostgreSQL/HTTP policy lifecycle.
        suggestion=str(uuid.uuid4())
        proposal=json.dumps({'summary':'測試資料只有事件次數。','action':'chat_context_messages','value':42,'rationale':'僅測試安全政策變更，不推論品質。'})
        sql="INSERT INTO analysis_suggestions(id,target_app,metric_snapshot,summary,proposal) VALUES ('"+suggestion+"','web_chat','{\"rows\":[]}','測試資料只有事件次數。','"+proposal+"')"
        run([PG/'psql',env['ANALYTICS_DATABASE_URL'],'-v','ON_ERROR_STOP=1','-c',sql],stdout=subprocess.DEVNULL)
        route='/api/v1/rooms/'+room+'/suggestions/'+suggestion
        admin_token=env['ANALYTICS_ADMIN_TOKEN']
        check('pending suggestion cannot be applied',http(chat_url,route+'/apply',chat_token,{'confirmed':True})[0]==409)
        check('cross-source proposal reads rejected',http(analytics_url,'/api/v1/suggestions/'+suggestion,pet_token)[0]==404)
        check('non-admin suggestion review rejected',http(analytics_url,'/api/v1/suggestions/'+suggestion+'/review',analytics_token,{'decision':'approved','confirmed':True})[0]==401)
        check('unapplied suggestion cannot be evaluated',http(analytics_url,'/api/v1/suggestions/'+suggestion+'/evaluate',admin_token,{'confirmed':True})[0]==409)
        assert http(analytics_url,'/api/v1/suggestions/'+suggestion+'/review',admin_token,{'decision':'approved','confirmed':True})[0]==204
        check('policy change needs explicit confirmation',http(chat_url,route+'/apply',chat_token,{'confirmed':False})[0]==400)
        status,applied=http(chat_url,route+'/apply',chat_token,{'confirmed':True})
        assert status==200 and applied['analytics_acknowledged'] and applied['value']==42
        check('scoped policy adoption has stable idempotent receipt',http(chat_url,route+'/apply',chat_token,{'confirmed':True})[1]==applied)
        actual=subprocess.check_output([PG/'psql',env['CHAT_DATABASE_URL'],'-At','-c',"SELECT context_messages FROM chat_room_policy WHERE room_id='"+room+"'"],text=True).strip()
        check('adoption modifies actual room context limit',actual=='42')
        status,rolled=http(chat_url,route+'/rollback',chat_token,{'confirmed':True})
        assert status==200 and rolled['value']==30 and rolled['analytics_acknowledged']
        check('policy rollback is acknowledged and idempotent',http(chat_url,route+'/rollback',chat_token,{'confirmed':True})[1]==rolled)
        status,evaluation=http(analytics_url,'/api/v1/suggestions/'+suggestion+'/evaluate',admin_token,{'confirmed':True})
        check('evaluation stores actual descriptive counts without a causal claim',status==200 and evaluation['after']['target_app']=='web_chat' and 'cannot prove causal impact' in evaluation['interpretation'])

        stop(chat);stop(analytics);chat=start('chat');analytics=start('analytics')
        recent=http(chat_url,'/api/v1/messages?room_id='+room,chat_token)[1]
        cursor=recent[-1]['id']
        older=http(chat_url,'/api/v1/messages?room_id='+room+'&before='+cursor,chat_token)
        check('history cursor excludes its anchor without overlapping messages',older[0]==200 and all(row['id']!=cursor for row in older[1]))
        check('foreign or missing history cursor is rejected',http(chat_url,'/api/v1/messages?room_id='+room+'&before='+str(uuid.uuid4()),chat_token)[0]==404)
        check('history survives service restart',any(x['id']==message['id'] for x in http(chat_url,'/api/v1/messages?room_id='+room,chat_token)[1]))
        count=http(analytics_url,'/api/v1/events/count',analytics_token)[1]['total']
        assert http(analytics_url,'/api/v1/events',ingest_token,event)[0]==202
        check('idempotency survives service restart',http(analytics_url,'/api/v1/events/count',analytics_token)[1]['total']==count)
        stop(analytics)
        status,queued=http(chat_url,'/api/v1/messages',chat_token,{'room_id':room,'content':'retry after outage'})
        assert status==201
        time.sleep(1.5)
        analytics=start('analytics')
        wait(lambda:http(analytics_url,'/api/v1/events/web_chat/'+queued['id'],analytics_token)[0]==200)
        check('Analytics outage causes durable retry and later delivery')
        # 真實 pg_dump/pg_restore 到第三個隔離庫，逐列核對，不以備份檔存在宣稱成功。
        run([PG/'pg_dump',env['ANALYTICS_DATABASE_URL'],'-Fc','-f',d/'analytics.dump'])
        run([PG/'psql',admin,'-v','ON_ERROR_STOP=1','-c','CREATE DATABASE analytics_restore'],stdout=subprocess.DEVNULL)
        restored=admin.rsplit('/',1)[0]+'/analytics_restore'
        run([PG/'pg_restore','--no-owner','--no-acl','--exit-on-error','-d',restored,d/'analytics.dump'])
        sql="SELECT source_app,event_id,event_type FROM product_events ORDER BY source_app,event_id"
        a=subprocess.check_output([PG/'psql',env['ANALYTICS_DATABASE_URL'],'-At','-c',sql])
        b=subprocess.check_output([PG/'psql',restored,'-At','-c',sql])
        check('Analytics backup restored with exact matching event rows',a==b)
        # PostgreSQL 故障時 health 非 200，復原後 worker 仍工作。
        run([PG/'pg_ctl','-D',d/'data','stop','-m','fast'],stdout=subprocess.DEVNULL);started=False
        check('database outage makes Chat unready',http(chat_url,'/healthz')[0]==503)
        check('database outage makes Analytics unready',http(analytics_url,'/healthz')[0]==503)
        run([PG/'pg_ctl','-D',d/'data','-l',d/'pg.log','-o',f'-h 127.0.0.1 -p {dbport} -k {scratch}','start'],stdout=subprocess.DEVNULL);started=True
        wait(lambda:http(chat_url,'/healthz')[0]==200 and http(analytics_url,'/healthz')[0]==200)
        status,recovered=http(chat_url,'/api/v1/messages',chat_token,{'room_id':room,'content':'after DB recovery'})
        assert status==201
        wait(lambda:http(analytics_url,'/api/v1/events/web_chat/'+recovered['id'],analytics_token)[0]==200)
        check('dispatcher recovers after PostgreSQL interruption')
        # Rotate fixture-only credentials through real process restart; no old credential grace.
        old_chat=chat_token;old_read=analytics_token;old_ingest=ingest_token
        chat_token=secrets.token_urlsafe(32);analytics_token=secrets.token_urlsafe(32);ingest_token=secrets.token_urlsafe(32)
        env.update(CHAT_API_TOKEN=chat_token,ANALYTICS_API_TOKEN=analytics_token,ANALYTICS_CHAT_TOKEN=ingest_token)
        stop(analytics);analytics=start('analytics')
        check('old Analytics query key is revoked after rotation',http(analytics_url,'/api/v1/events/count',old_read)[0]==401 and http(analytics_url,'/api/v1/events/count',analytics_token)[0]==200)
        rotated_event={'schema_version':1,'event_id':str(uuid.uuid4()),'source_app':'web_chat','event_type':'message.persisted'}
        check('old source ingest key is revoked after rotation',http(analytics_url,'/api/v1/events',old_ingest,rotated_event)[0]==401)
        stop(chat);chat=start('chat')
        check('old Chat administrative key is revoked after rotation',http(chat_url,'/api/v1/rooms',old_chat)[0]==401 and http(chat_url,'/api/v1/rooms',chat_token)[0]==200)
        rotated_id=str(uuid.uuid4());assert http(chat_url,'/api/v1/messages',chat_token,{'id':rotated_id,'room_id':room,'content':'credential rotation fixture'})[0]==201
        wait(lambda:http(analytics_url,'/api/v1/events/web_chat/'+rotated_id,analytics_token)[0]==200)
        check('new Chat/Analytics source keys preserve durable cross-service delivery')

        if '--load' in sys.argv:
            load_urls=[chat_url]
            additional=[]
            for index in range(4):
                loadport=port();url=f'http://127.0.0.1:{loadport}';log=open(d/f'load-chat-{index}.log','a');handles.append(log)
                process=subprocess.Popen([ROOT/'target/debug/day9-chat'],cwd=ROOT,env=env|{'CHAT_LISTEN':f'127.0.0.1:{loadport}'},stdout=log,stderr=log);processes.append(process);additional.append(process);wait(lambda:http(url,'/healthz')[0]==200);load_urls.append(url)
            run(['node','scripts/realtime-load-e2e.mjs'],env=env|{'CHAT_LOAD_URLS':json.dumps(load_urls),'CHAT_LOAD_REPORT':str(ROOT/'target/realtime-load-validation.json')})
            check('real 512-socket admission boundaries and 64-message private cross-process fan-out load')
            for process in additional:stop(process)
        if '--live-openai' in sys.argv:
            stop(chat)
            env['OPENAI_API_KEY']=live_key
            env['OPENAI_CHAT_MODEL']=os.environ.get('OPENAI_CHAT_MODEL','gpt-5.6-luna')
            chat=start('chat')
            live_room='live-'+secrets.token_hex(8)
            assert http(chat_url,'/api/v1/rooms',chat_token,{'id':live_room,'name':'OpenAI 驗證'})[0]==201
            assert http(chat_url,'/api/v1/messages',chat_token,{'room_id':live_room,'content':'請只回答四個中文字：測試成功。'})[0]==201
            status,document=http(chat_url,'/api/v1/knowledge',chat_token,{'title':'授權的實驗資料','content':'本地驗證的實驗代碼是 COMET-432。這是供 RAG 測試使用的非私人資料。'})
            check('real OpenAI embedding persisted in pgvector',status==201)
            status,hits=http(chat_url,'/api/v1/knowledge/search',chat_token,{'query':'實驗代碼是什麼？'})
            check('pgvector retrieves the expected document',status==200 and any(hit['id']==document['id'] for hit in hits))
            request=urllib.request.Request(chat_url+'/api/v1/ai/reply',data=json.dumps({'room_id':live_room,'rag':True}).encode(),headers={'Authorization':'Bearer '+chat_token,'Content-Type':'application/json'})
            streamed='';saved_id=None;event_type=None
            with urllib.request.urlopen(request,timeout=100) as response:
                for line in response:
                    text=line.decode().strip()
                    if text.startswith('event:'): event_type=text[6:].strip()
                    if text.startswith('data:'):
                        data=text[5:].strip()
                        assert event_type!='error', 'OpenAI stream failed without persistence'
                        if event_type=='delta':streamed+=json.loads(data)['text']
                        if event_type=='done':saved_id=json.loads(data)['id']
            check('real OpenAI Responses stream reaches completion',bool(streamed.strip()) and bool(saved_id))
            records=http(chat_url,'/api/v1/messages?room_id='+live_room,chat_token)[1]
            check('completed real AI stream matches persisted message',any(row['id']==saved_id and row['content']==streamed for row in records))
            wait(lambda:http(analytics_url,'/api/v1/events/web_chat/'+saved_id,analytics_token)[0]==200)
            check('real AI answer emits durable Analytics event')
            status,agent=http(chat_url,'/api/v1/ai/agent',chat_token,{'room_id':live_room,'question':'請使用知識庫工具找到本地驗證的實驗代碼，回答代碼、引用文件，並提出一個理解問題。'})
            check('real OpenAI Function Calling through authenticated local MCP and strict JSON answer',status==200 and agent['steps'] and agent['follow_up_question'] and document['id'] in agent['sources'] and 'COMET-432' in agent['answer'])
            wait(lambda:http(analytics_url,'/api/v1/events/web_chat/'+agent['id'],analytics_token)[0]==200)
            check('completed Agent answer emits a durable Analytics event')
            persona_base='/api/v1/rooms/'+live_room+'/personas'
            for label,prompt in [('老師A','你是老師，短句解釋 Rust 所有權，再問一個理解問題。記憶不超過30字。'),('學生B','你是學生，回應老師的說明並提出一個簡短理解問題。記憶不超過30字。')]:
                assert http(chat_url,persona_base,chat_token,{'id':str(uuid.uuid4()),'name':label,'instructions':prompt,'memory':'','cooldown_seconds':1,'enabled':True})[0]==204
            run_base='/api/v1/rooms/'+live_room+'/persona-runs'
            discussion={'id':str(uuid.uuid4()),'topic':'只用短句討論 Rust 所有權。本例只有測試資料。','max_turns':2,'max_output_budget':1024,'tools':False,'confirmed':True}
            status,exchange=http(chat_url,run_base,chat_token,discussion)
            check('real two-persona round-robin completes within reserved output budget',status==200 and exchange['status']=='completed' and len(exchange['turns'])==2 and len({turn['persona_id']for turn in exchange['turns']})==2 and exchange['reserved_output_tokens']==1024)
            check('completed persona run replays without new provider calls',http(chat_url,run_base,chat_token,discussion)[1]==exchange)
            for turn in exchange['turns']:
                wait(lambda:http(analytics_url,'/api/v1/events/web_chat/'+turn['message_id'],analytics_token)[0]==200)
            saved_personas=http(chat_url,persona_base,chat_token)[1]
            check('each persona retains its own bounded memory',len(saved_personas)==2 and all(isinstance(p['memory'],str) and len(p['memory'].encode())<=2048 for p in saved_personas))
            exhausted=discussion|{'id':str(uuid.uuid4()),'max_output_budget':512}
            status,stopped=http(chat_url,run_base,chat_token,exhausted)
            check('real persona loop stops before a second request when output budget is exhausted',status==200 and stopped['status']=='budget_exhausted' and len(stopped['turns'])==1 and stopped['reserved_output_tokens']==512)
            tool_run=discussion|{'id':str(uuid.uuid4()),'max_turns':1,'tools':True,'topic':'使用知識庫工具找到本地驗證實驗代碼，短句回答、引用文件並提出理解問題。'}
            status,grounded=http(chat_url,run_base,chat_token,tool_run)
            print('Synthetic persona tool result:',status,json.dumps(grounded,ensure_ascii=False),flush=True)
            check('real bounded persona Function Calling uses scoped MCP citations',status==200 and grounded['status']=='completed' and grounded['turns'][0]['steps'] and document['id'] in grounded['turns'][0]['sources'] and 'COMET-432' in grounded['turns'][0]['answer'])

            check('knowledge document can be deleted',http(chat_url,'/api/v1/knowledge/'+document['id'],chat_token,method='DELETE')[0]==204)
        if live_key:
            env['OPENAI_API_KEY']=live_key
            env['OPENAI_CHAT_MODEL']=os.environ.get('OPENAI_CHAT_MODEL','gpt-5.6-luna')
            stop(analytics);analytics=start('analytics')
            status,insight=http(analytics_url,'/api/v1/insights',env['ANALYTICS_ADMIN_TOKEN'],{'target_app':'web_chat'})
            check('real OpenAI Analytics structured insight uses a persisted aggregate snapshot',status==200 and insight['snapshot']['rows'] and insight['proposal']['action']=='chat_context_messages' and insight['status']=='pending')

        wait(lambda:all(any(path==expected for path,_ in observed) for expected in ('/v1/traces','/v1/metrics','/v1/logs')))
        check('real local OTLP traces, metrics and logs received')
        def trace_ids(expected_service,expected_route=None):
            ids=set()
            for path,payload in observed:
                if path!='/v1/traces':continue
                for resource in json.loads(payload).get('resourceSpans',[]):
                    attrs={x['key']:x.get('value',{}).get('stringValue') for x in resource.get('resource',{}).get('attributes',[])}
                    if attrs.get('service.name')!=expected_service:continue
                    for scope in resource.get('scopeSpans',[]):
                        for span in scope.get('spans',[]):
                            values={x['key']:x.get('value',{}).get('stringValue') for x in span.get('attributes',[])}
                            if expected_route is None or values.get('http.route')==expected_route:ids.add(span['traceId'])
            return ids
        chat_ids=trace_ids('chat','/api/v1/messages');analytics_ids=trace_ids('analytics','/api/v1/events')
        check('Chat Outbox and Analytics preserve W3C trace correlation',bool(chat_ids&analytics_ids))

        forbidden=([live_key] if live_key else [])+[chat_token,analytics_token,ingest_token,pet_token,'transactional test','retry after outage','not committed']
        if '--pet-gui' in sys.argv:
            pet_suggestion=str(uuid.uuid4())
            pet_proposal=json.dumps({'summary':'複習事件測試。','action':'pet_review_min_days','value':3,'rationale':'測試可回退設定，不推論學習品質。'})
            sql="INSERT INTO analysis_suggestions(id,target_app,metric_snapshot,summary,proposal,status) VALUES ('"+pet_suggestion+"','desktop_pet','{\"rows\":[]}','複習事件測試。','"+pet_proposal+"','approved')"
            run([PG/'psql',env['ANALYTICS_DATABASE_URL'],'-v','ON_ERROR_STOP=1','-c',sql],stdout=subprocess.DEVNULL)
            native_env=env | {'PET_GUI_TEST_ANALYTICS':'true','PET_GUI_TEST_SUGGESTION':pet_suggestion}
            run(['python3','scripts/pet-gui-verify.py','--analytics']+(['--live-ai'] if '--live-openai' in sys.argv else []),env=native_env)
            summary=http(analytics_url,'/api/v1/events/summary',analytics_token)[1]
            bytype={(row['source_app'],row['event_type']):row['total'] for row in summary}
            check('native Pet policy apply and rollback acknowledged by Analytics',http(analytics_url,'/api/v1/suggestions/'+pet_suggestion,pet_token)[1]['status']=='rolled_back')
            check('actual Pet restore event delivered exactly once to Analytics',bytype.get(('desktop_pet','file.restore.succeeded'))==1)
            check('actual Pet GUI receipts and learning events delivered exactly once to Analytics',bytype.get(('desktop_pet','file.cleanup.succeeded'))==2 and bytype.get(('desktop_pet','learning.review.completed'))==1)
            # Native file operations preserve traceparent into durable SQLite outbox.
            time.sleep(.5)
            pet_ids=trace_ids('desktop_pet')
            analytics_ids=trace_ids('analytics','/api/v1/events')
            check('Pet and Analytics preserve W3C trace correlation',bool(pet_ids&analytics_ids))
            forbidden += ['安全隔離的測試記憶','Rust 的所有權如何避免 double free？','sample.txt','second.txt']
        check('OTLP transport strips inherited authentication headers',all('metadata-synthetic-secret' not in str(headers) and 'Authorization' not in headers for headers in observed_headers))
        check('telemetry excludes credentials and message contents',all(value.encode() not in payload for value in forbidden for _,payload in observed))
        REPORT.parent.mkdir(parents=True,exist_ok=True)
        REPORT.write_text(json.dumps({'status':'passed','checks':results,'scope':'implemented reliability slice only; P9 incomplete','live_openai':bool(live_key),'openai_model':env.get('OPENAI_CHAT_MODEL') if live_key else None},ensure_ascii=False,indent=2)+'\n')
        if '--browser' in sys.argv or '--browser-live' in sys.argv:
            if '--browser-live' in sys.argv:
                stop(chat);chat=start('chat')
            fixture={'chat_url':chat_url,'analytics_url':analytics_url,'analytics_read_token':analytics_token,'analytics_admin_token':env['ANALYTICS_ADMIN_TOKEN'],'username':'browser_'+secrets.token_hex(5),'password':secrets.token_urlsafe(24)}
            req=urllib.request.Request(chat_url+'/api/v1/auth/register',data=json.dumps({'username':fixture['username'],'password':fixture['password']}).encode(),headers={'Content-Type':'application/json','X-Copilot-CSRF':'1'})
            with urllib.request.urlopen(req) as response: assert response.status==201
            run([PG/'psql',env['ANALYTICS_DATABASE_URL'],'-v','ON_ERROR_STOP=1','-c',"UPDATE product_events SET received_at=received_at-interval '1 day' WHERE event_id='"+message['id']+"'"],stdout=subprocess.DEVNULL)
            path=ROOT.parent/'browser-fixture.json';path.write_text(json.dumps(fixture));path.chmod(0o600)
            print('BROWSER_FIXTURE_READY',flush=True)
            control=ROOT.parent/'browser-control.txt'
            control.unlink(missing_ok=True)
            while True:
                if control.exists():
                    command=control.read_text().strip();control.unlink()
                    if command=='stop_chat':stop(chat)
                    elif command=='start_chat' and chat.poll() is not None:
                        chat=start('chat')
                    (ROOT.parent/'browser-control-result.json').write_text(json.dumps({'command':command,'chat_running':chat.poll() is None}))
                time.sleep(.2)

    finally:
        for proc in processes:stop(proc)
        collector.shutdown();collector.server_close()
        if started:subprocess.run([PG/'pg_ctl','-D',d/'data','stop','-m','fast'],stdout=subprocess.DEVNULL)
