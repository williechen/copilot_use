#!/usr/bin/env python3
"""建立全新隔離 PostgreSQL，以真實 binary/API 驗證；結束時移除測試庫。"""
import json, os, pathlib, secrets, signal, socket, subprocess, sys, tempfile, time, urllib.request, urllib.error, threading
from http.server import ThreadingHTTPServer,BaseHTTPRequestHandler
ROOT = pathlib.Path(__file__).resolve().parents[1]
PG = pathlib.Path(subprocess.check_output(['pg_config','--bindir'],text=True).strip())
REPORT = pathlib.Path(os.environ.get('VERIFY_REPORT', ROOT / 'target/local-validation.json'))
live_key=os.environ.get('OPENAI_API_KEY') if '--live-openai' in sys.argv else None
if '--live-openai' in sys.argv and not live_key: raise SystemExit('OPENAI_API_KEY not configured')
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
        with urllib.request.urlopen(req,timeout=5) as r:
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
    observed=[]
    class Collector(BaseHTTPRequestHandler):
        def log_message(self,*_):pass
        def do_POST(self):
            length=int(self.headers.get('Content-Length','0'))
            if length>2000000:self.send_error(413);return
            payload=self.rfile.read(length)
            observed.append((self.path,payload))
            self.send_response(200);self.send_header('Content-Type','application/json');self.end_headers();self.wfile.write(b'{}')
    collector=ThreadingHTTPServer(('127.0.0.1',0),Collector)
    collector_thread=threading.Thread(target=collector.serve_forever,daemon=True);collector_thread.start()
    env['COPILOT_LOCAL_OTLP_ENDPOINT']=f'http://127.0.0.1:{collector.server_port}'
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
        # 跨角色隔離不能只看資料庫名稱，實際拒絕 CONNECT。
        for app in ('chat','analytics'):
            run([PG/'psql',admin,'-v','ON_ERROR_STOP=1','-c',f'REVOKE CONNECT ON DATABASE {app}_test FROM PUBLIC'],stdout=subprocess.DEVNULL)
        wrong=env['CHAT_DATABASE_URL'].rsplit('/',1)[0]+'/analytics_test'
        denied=subprocess.run([PG/'psql',wrong,'-c','SELECT 1'],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
        check('Chat database role cannot CONNECT to Analytics database',denied.returncode!=0)
        check('unauthorized Chat write rejected',http(chat_url,'/api/v1/rooms',body={'id':'bad','name':'bad'})[0]==401)
        check('unauthorized Analytics read rejected',http(analytics_url,'/api/v1/events/count')[0]==401)
        room='test-'+secrets.token_hex(8)
        check('unconfigured AI returns 503 without mock output',http(chat_url,'/api/v1/ai/reply',chat_token,{'room_id':room})[0]==503)
        check('room created',http(chat_url,'/api/v1/rooms',chat_token,{'id':room,'name':'本地測試'})[0]==201)
        status,message=http(chat_url,'/api/v1/messages',chat_token,{'room_id':room,'content':'transactional test'})
        check('message persisted',status==201)
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
                with urllib.request.urlopen(req,timeout=5) as response:
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
        assert member_http(bob,'/api/v1/auth/logout',{})[0]==204
        check('logout revokes server-side session',member_http(bob,'/api/v1/messages?room_id='+private)[0]==401)
        run(['node','scripts/realtime-e2e.mjs'],env=env)
        check('real WebSocket two-client and room-isolation E2E')
        stop(chat);stop(analytics);chat=start('chat');analytics=start('analytics')
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
        if live_key:
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
            check('knowledge document can be deleted',http(chat_url,'/api/v1/knowledge/'+document['id'],chat_token,method='DELETE')[0]==204)
        wait(lambda:all(any(path==expected for path,_ in observed) for expected in ('/v1/traces','/v1/metrics','/v1/logs')))
        check('real local OTLP traces, metrics and logs received')
        chat_ids=set();analytics_ids=set()
        for path,payload in observed:
            if path!='/v1/traces':continue
            decoded=json.loads(payload)
            for resource in decoded.get('resourceSpans',[]):
                attrs={x['key']:x.get('value',{}).get('stringValue') for x in resource.get('resource',{}).get('attributes',[])}
                service=attrs.get('service.name')
                for scope in resource.get('scopeSpans',[]):
                    for span in scope.get('spans',[]):
                        values={x['key']:x.get('value',{}).get('stringValue') for x in span.get('attributes',[])}
                        if service=='chat' and values.get('http.route')=='/api/v1/messages':chat_ids.add(span['traceId'])
                        if service=='analytics' and values.get('http.route')=='/api/v1/events':analytics_ids.add(span['traceId'])
        check('Chat Outbox and Analytics preserve W3C trace correlation',bool(chat_ids&analytics_ids))

        forbidden=[chat_token,analytics_token,ingest_token,pet_token,'transactional test','retry after outage','not committed']
        check('telemetry excludes credentials and message contents',all(value.encode() not in payload for value in forbidden for _,payload in observed))
        REPORT.parent.mkdir(parents=True,exist_ok=True)
        REPORT.write_text(json.dumps({'status':'passed','checks':results,'scope':'implemented reliability slice only; P9 incomplete','live_openai':bool(live_key),'openai_model':env.get('OPENAI_CHAT_MODEL') if live_key else None},ensure_ascii=False,indent=2)+'\n')
        if '--browser' in sys.argv:
            fixture={'chat_url':chat_url,'analytics_url':analytics_url,'username':'browser_'+secrets.token_hex(5),'password':secrets.token_urlsafe(24)}
            req=urllib.request.Request(chat_url+'/api/v1/auth/register',data=json.dumps({'username':fixture['username'],'password':fixture['password']}).encode(),headers={'Content-Type':'application/json','X-Copilot-CSRF':'1'})
            with urllib.request.urlopen(req) as response: assert response.status==201
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
