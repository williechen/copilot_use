import os,pathlib,subprocess,time,socket,requests,sqlite3,json
r=pathlib.Path(__file__).resolve().parent
runtime=r/'runtime';runtime.mkdir(exist_ok=True)
db=runtime/'browser.sqlite'
binary='/home/willie/Documents/Codex/2026-10-23/referenced-chatgpt-conversation-this-is-an-2/work/target-full/debug/server'
with socket.socket() as s:
 if s.connect_ex(('127.0.0.1',3000))==0:raise RuntimeError('port 3000 in use; leaving existing service alone')
env={k:os.environ[k] for k in ['PATH','HOME'] if k in os.environ}
env.update(CHAT_DATABASE_URL='sqlite:'+str(db)+'?mode=rwc',LEPTOS_OUTPUT_NAME='rust_ai_chat',LEPTOS_SITE_ROOT=str(r.parent/'rust_ai_chat/target/site'))
log=open(r/'logs/browser-server.log','w')
p=subprocess.Popen([binary],cwd=runtime,env=env,stdout=log,stderr=subprocess.STDOUT)
(r/'server-pid.json').write_text(json.dumps({'pid':p.pid,'db':str(db),'source_sha':'9c2b7cf5a4c60a08254445911916c4ed9b02d7b3','site_root':env['LEPTOS_SITE_ROOT']}))
try:
 for _ in range(100):
  if p.poll() is not None:raise RuntimeError('server exited')
  try:
   if requests.get('http://127.0.0.1:3000/health',timeout=1).status_code==200:break
  except requests.ConnectionError:time.sleep(.1)
 else:raise TimeoutError('health')
 names=[('browser-room-1','Rust 技術交流'),('browser-room-2','Rust 技術交流'),('browser-room-3','Rust 技術交流延伸'),('browser-room-4','一般閒聊'),('browser-room-pct','百分%房間'),('browser-room-under','底線_房間'),('browser-room-back','反斜線\\房間')]
 with sqlite3.connect(db) as c:
  for id,name in names:c.execute("INSERT OR IGNORE INTO rooms (id,category_room_id,name,desc,created_at,updated_at) VALUES (?,'browser-fixture',?,'',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)",(id,name))
  c.commit()
 origin={'Origin':'http://127.0.0.1:3000'}
 reg=requests.post('http://127.0.0.1:3000/signup',headers=origin,data={'username':'browser_audit','email':'browser@example.test','display_name':'Browser Audit','password':'browser-fixture-password-123'},allow_redirects=False,timeout=10)
 assert reg.status_code in (303,409)
 (r/'logs/browser-setup.json').write_text(json.dumps({'source_sha':'9c2b7cf5a4c60a08254445911916c4ed9b02d7b3','fixture_rooms':names,'signup_status':reg.status_code,'migrations':sqlite3.connect(db).execute('select version from _sqlx_migrations order by version').fetchall()},ensure_ascii=False,indent=2))
 print('ready: http://127.0.0.1:3000',flush=True)
 p.wait()
finally:
 if p.poll() is None:p.terminate();p.wait(timeout=5)
 log.close()
