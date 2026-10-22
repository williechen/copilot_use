from pathlib import Path
import os,subprocess,time,socket,http.client,json,urllib.parse,sqlite3
root=Path.cwd();logs=root/'work';db=root/'work/chat-http.sqlite';children=[]
env={k:os.environ[k] for k in ['PATH','HOME','XDG_RUNTIME_DIR'] if k in os.environ};env.update(CHAT_DATABASE_URL='sqlite:'+str(db)+'?mode=rwc',LEPTOS_OUTPUT_NAME='ui')
def start():
 with socket.socket() as s:assert s.connect_ex(('127.0.0.1',3000))!=0,'occupied port'
 f=(logs/'chat-http-runtime.log').open('a');p=subprocess.Popen(['/var/tmp/codex-rust-validation-20261022/target/debug/server'],cwd=root/'work/validation/rust_ai_chat',env=env,stdout=f,stderr=subprocess.STDOUT);children.append((p,f))
 for _ in range(100):
  if p.poll() is not None:raise RuntimeError('server exited')
  with socket.socket() as s:
   if s.connect_ex(('127.0.0.1',3000))==0:return p
  time.sleep(.1)
 raise RuntimeError('server unavailable')
def request(method,path,data=None,cookie=None):
 c=http.client.HTTPConnection('127.0.0.1',3000,timeout=15);h={};body=None
 if data is not None:body=urllib.parse.urlencode(data);h={'Origin':'http://127.0.0.1:3000','Content-Type':'application/x-www-form-urlencoded'}
 if cookie:h['Cookie']=cookie
 c.request(method,path,body,h);r=c.getresponse();b=r.read().decode();h=dict(r.getheaders());s=r.status;c.close();return s,h,b
results={}
try:
 p=start();results['fresh_full_root_migration_startup']=True
 form={'username':'validation_user','email':'validation@example.test','display_name':'本地驗證','password':'lesson-only-password'}
 s,h,b=request('POST','/signup',form);assert s==303 and h['location']=='/signup/complete' and 'Max-Age=0' in h['set-cookie'];results['signup_redirect_and_logged_out']=True
 s,h,b=request('GET','/signup/complete');assert s==200 and 'location' not in h and '註冊完成' in b and 'http-equiv="refresh"' not in b and '<script' not in b;results['completion_page_stays_until_home_link']=True
 s,h,_=request('POST','/login',{'username':'VALIDATION_USER','password':'lesson-only-password'});assert s==303;cookie=h['set-cookie'].split(';')[0];assert request('GET','/account',cookie=cookie)[0]==200
 results['valid_session_room_status']=request('GET','/room/validation-room',cookie=cookie)[0]
 p.terminate();p.wait(timeout=10);p=start();assert request('GET','/account',cookie=cookie)[0]==200;results['login_session_survives_process_restart']=True
 form.update(username='validation_second',email='second@example.test');s,h,_=request('POST','/signup',form,cookie);assert s==303 and h['location']=='/signup/complete';assert request('GET','/account',cookie=cookie)[0]==401;results['new_signup_revokes_current_browser_session']=True
 form.update(username='VALIDATION_USER',email='third@example.test');assert request('POST','/signup',form)[0]==409;results['duplicate_account_casefold_rejected']=True
 with sqlite3.connect(db) as con:assert con.execute('SELECT count(*) FROM mbr_user').fetchone()[0]==2
 results['unauthenticated_room_page_status']=request('GET','/room/validation-room')[0]
 results['unauthenticated_rooms_api_status']=request('GET','/api/rooms')[0]
 results['chat_logout_route_status']=request('POST','/logout',{})[0]
finally:
 for p,f in children:
  if p.poll() is None:p.terminate();p.wait(timeout=10)
  f.close()
 (logs/'chat-http-results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(results,ensure_ascii=False))
