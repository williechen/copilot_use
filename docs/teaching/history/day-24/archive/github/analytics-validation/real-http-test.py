from pathlib import Path
import subprocess,os,time,socket,http.client,json,urllib.parse
base=Path.cwd();out=base/'work/day24-logs';children=[]
env={k:os.environ[k] for k in ['PATH','HOME','XDG_RUNTIME_DIR'] if k in os.environ};env.update(ANALYTICS_DATABASE_URL='postgres://day24@127.0.0.1:55439/day24_http',ANALYTICS_CHAT_TOKEN='fixture-chat',ANALYTICS_PET_TOKEN='fixture-pet')
def start():
 with socket.socket() as sock:assert sock.connect_ex(('127.0.0.1',3100))!=0,'occupied port'
 f=(out/'analytics-http-runtime.log').open('a');p=subprocess.Popen(['/home/willie/Documents/rust_ai_chat/target/debug/stats-server'],cwd=base/'work/day24-source',env=env,stdout=f,stderr=subprocess.STDOUT);children.append((p,f))
 for _ in range(100):
  if p.poll() is not None:raise RuntimeError('runtime exited')
  with socket.socket() as sock:
   if sock.connect_ex(('127.0.0.1',3100))==0:return p
  time.sleep(.1)
 raise RuntimeError('runtime unavailable')
def request(method,path,form=None,cookie=None):
 c=http.client.HTTPConnection('127.0.0.1',3100,timeout=15);h={};body=None
 if form is not None:body=urllib.parse.urlencode(form);h={'Origin':'http://127.0.0.1:3100','Content-Type':'application/x-www-form-urlencoded'}
 if cookie:h['Cookie']=cookie
 c.request(method,path,body,h);r=c.getresponse();b=r.read().decode();h=dict(r.getheaders());status=r.status;c.close();return status,h,b
result={}
try:
 p=start();status,h,_=request('POST','/login',{'username':'admin','password':'lesson-only-password'});assert status==303 and h['location']=='/change-password';cookie=h['set-cookie'].split(';')[0]
 assert request('GET','/api/v1/events/web_chat/summary',cookie=cookie)[0]==403;result['restricted_before_change']=True
 status,h,html=request('GET','/change-password',cookie=cookie);assert status==200;csrf=html.split('name="csrf" value="')[1].split('"')[0]
 status,h,_=request('POST','/change-password',{'current_password':'lesson-only-password','new_password':'new-lesson-password','confirm_password':'new-lesson-password','email':'learner@example.test','csrf':csrf},cookie);assert status==303 and h['location']=='/login' and 'Max-Age=0' in h['set-cookie'];result['real_http_change']=True
 assert request('GET','/api/v1/events/web_chat/summary',cookie=cookie)[0]==401;result['old_cookie_rejected']=True
 p.terminate();p.wait(timeout=10);p=start()
 assert request('POST','/login',{'username':'admin','password':'lesson-only-password'})[0]==401
 status,h,_=request('POST','/login',{'username':'admin','password':'new-lesson-password'});assert status==303 and h['location']=='/dashboard';cookie=h['set-cookie'].split(';')[0]
 assert request('GET','/dashboard',cookie=cookie)[0]==200;assert request('GET','/change-password',cookie=cookie)[0]==200;result['restart_new_password_and_dashboard']=True
finally:
 for p,f in children:
  if p.poll() is None:p.terminate();p.wait(timeout=10)
  f.close()
 (out/'analytics-http-results.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result))
