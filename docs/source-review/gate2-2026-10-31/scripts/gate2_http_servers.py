from pathlib import Path
import subprocess,os,sqlite3,json,time,hashlib,socket,urllib.request,signal,datetime
b=Path.cwd();d=b/'work/day44-recheck-latest';work=b/'work/gate2-20261031';out=b/'outputs/gate2-2026-10-31';rows=[];processes=[]
env=os.environ.copy()
for k in ['CHAT_DATABASE_URL','PET_DATABASE_URL','ANALYTICS_DATABASE_URL','DATABASE_URL','OPENAI_API_KEY','ANTHROPIC_API_KEY','GEMINI_API_KEY']:env.pop(k,None)
for port in [3000,3100]:
 s=socket.socket();s.bind(('127.0.0.1',port));s.close()
def run(a,cwd=b):
 t=time.time();p=subprocess.run(a,cwd=cwd,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=60);name=f'http-setup-{len(rows):02}.log';(out/name).write_text(p.stdout);rows.append({'command':a,'cwd':str(cwd),'exit_code':p.returncode,'started_utc':datetime.datetime.fromtimestamp(t,datetime.timezone.utc).isoformat(),'seconds':time.time()-t,'log':name});(out/'http-setup-commands.json').write_text(json.dumps(rows,indent=2)+'\n');return p.returncode
def ready(url,p):
 for _ in range(100):
  try:urllib.request.urlopen(url,timeout=1).close();return
  except Exception:
   if p.poll() is not None:raise RuntimeError('server exited before health check')
   time.sleep(.1)
 raise RuntimeError('health timeout')
def chat_env(db):return {**env,'CHAT_DATABASE_URL':'sqlite://'+str(db)+'?mode=rwc','LEPTOS_OUTPUT_NAME':'rust_ai_chat','LEPTOS_SITE_ROOT':str(d/'chat_apps/target/site'),'LEPTOS_SITE_PKG_DIR':'pkg','LEPTOS_SITE_ADDR':'127.0.0.1:3000','LEPTOS_ENV':'DEV'}
def spawn(name,binary,e):
 cwd=work/('run-'+name);cwd.mkdir(exist_ok=True);(cwd/'.env').write_text('');log=(out/(name+'.log')).open('w');p=subprocess.Popen([str(binary)],cwd=cwd,env=e,stdout=log,stderr=subprocess.STDOUT,start_new_session=True);processes.append(p);return p
def data_snapshot(db):
 c=sqlite3.connect('file:'+str(db)+'?mode=ro',uri=True);result={}
 for(table,)in c.execute("SELECT name FROM sqlite_master WHERE type='table' AND name!='_sqlx_migrations' AND name NOT LIKE 'sqlite_%' ORDER BY name"):
  cols=[x[1] for x in c.execute('PRAGMA table_info("'+table+'")')];data=sorted(c.execute('SELECT * FROM "'+table+'"').fetchall(),key=repr);result[table]={'columns':cols,'rows':data}
 c.close();return result
ctl='/usr/lib/postgresql/18/bin/pg_ctl';pg=b/'work/pg';pg_started=False
try:
 # Actual user's Chat SQLite is opened read-only; only its backup copy is migrated.
 source=Path('/home/willie/Documents/rust_ai_ring/.local/chat.sqlite');copy=work/'chat-user-backup.sqlite';assert not copy.exists();original=sqlite3.connect('file:'+str(source)+'?mode=ro',uri=True);backup=sqlite3.connect(copy);original.backup(backup);backup.close();original.close();before=data_snapshot(copy)
 p=spawn('chat-old-copy',d/'chat_apps/target/debug/chat-server',chat_env(copy));ready('http://127.0.0.1:3000/health',p);after=data_snapshot(copy);preserved={}
 for table,item in before.items():
  c=sqlite3.connect('file:'+str(copy)+'?mode=ro',uri=True);cols=','.join('"'+x+'"' for x in item['columns']);current=sorted(c.execute('SELECT '+cols+' FROM "'+table+'"').fetchall(),key=repr);c.close();preserved[table]={'before_sha256':hashlib.sha256(repr(item['rows']).encode()).hexdigest(),'after_sha256':hashlib.sha256(repr(current).encode()).hexdigest(),'rows_before':len(item['rows']),'rows_after':len(current),'unchanged':current==item['rows']}
 assert all(x['unchanged'] for x in preserved.values());(out/'chat-actual-old-copy-preservation.json').write_text(json.dumps({'source_sha':'2c7ff8b861d32ab6d879bf80c9f2dce50de38097','source_open_mode':'SQLite read-only online backup','original_db_written':False,'application_real_startup_migrator':True,'tables':preserved,'private_rows_published':False},indent=2)+'\n');p.terminate();p.wait(timeout=10);processes.remove(p)
 chatdb=work/'chat-browser.sqlite';assert not chatdb.exists();chat=spawn('chat-browser',d/'chat_apps/target/debug/chat-server',chat_env(chatdb));ready('http://127.0.0.1:3000/health',chat)
 assert run([ctl,'-D',str(pg),'-l',str(out/'gate2-postgres.log'),'-o',"-k '' -p 55482 -h 127.0.0.1",'start'])==0;pg_started=True
 db='gate2_browser_20261031';assert run(['/usr/lib/postgresql/18/bin/createdb','-h','127.0.0.1','-p','55482',db])==0
 analytics_env={**env,'ANALYTICS_DATABASE_URL':'postgres://willie@127.0.0.1:55482/'+db,'ANALYTICS_CHAT_TOKEN':'gate2-local-chat-fixture','ANALYTICS_PET_TOKEN':'gate2-local-pet-fixture'}
 analytics=spawn('analytics-browser',d/'analytics_apps/target/debug/analytics-server',analytics_env);ready('http://127.0.0.1:3100/healthz',analytics)
 (out/'http-server-connection.json').write_text(json.dumps({'source_sha':'2c7ff8b861d32ab6d879bf80c9f2dce50de38097','chat_pid':chat.pid,'analytics_pid':analytics.pid,'chat_url':'http://127.0.0.1:3000','analytics_url':'http://127.0.0.1:3100','chat_database':str(chatdb),'pg_database':db,'pg_port':55482,'paid_ai_keys_removed':True,'formal_databases_written':False},indent=2)+'\n');print('HTTP SERVERS READY',flush=True)
 while chat.poll() is None and analytics.poll() is None:time.sleep(1)
finally:
 for p in processes:
  if p.poll() is None:p.terminate()
 if pg_started:run([ctl,'-D',str(pg),'stop'])
 print('HTTP servers stopped',flush=True)
