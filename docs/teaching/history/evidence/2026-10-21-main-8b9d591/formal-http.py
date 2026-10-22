import pathlib,subprocess,os,socket,time,http.client,json,sqlite3
root=pathlib.Path.cwd();logs=root/'work/formal-logs';repo=root/'work/formal-main';binary=pathlib.Path('/home/willie/Documents/rust_ai_chat/target/debug');children=[]
env={k:os.environ[k] for k in ['PATH','HOME','XDG_RUNTIME_DIR'] if k in os.environ}
env.update(CHAT_DATABASE_URL='sqlite:'+str(root/'work/formal-runtime/chat.sqlite'),ANALYTICS_DATABASE_URL='postgres://day23_formal@127.0.0.1:55439/formal_analytics',ANALYTICS_CHAT_TOKEN='fixture-chat',ANALYTICS_PET_TOKEN='fixture-pet')
def start(name,port):
 with socket.socket() as s: assert s.connect_ex(('127.0.0.1',port))!=0,'occupied port'
 f=(logs/(name+'-runtime.log')).open('a');p=subprocess.Popen([str(binary/name)],cwd=repo,env=env,stdout=f,stderr=subprocess.STDOUT);children.append((p,f))
 for _ in range(100):
  if p.poll() is not None:raise RuntimeError(name+' exited')
  with socket.socket() as s:
   if s.connect_ex(('127.0.0.1',port))==0:return p
  time.sleep(.1)
 raise RuntimeError(name+' not ready')
def stop(p):p.terminate();p.wait(timeout=10)
def req(port,method,path,body=None,cookie=None):
 c=http.client.HTTPConnection('127.0.0.1',port,timeout=10);h={}
 if body is not None:h.update({'Origin':f'http://127.0.0.1:{port}','Content-Type':'application/x-www-form-urlencoded'})
 if cookie:h['Cookie']=cookie
 c.request(method,path,body,h);r=c.getresponse();b=r.read().decode();headers=dict(r.getheaders());c.close();return r.status,headers,b
results={}
try:
 p=start('server',3000)
 s,h,b=req(3000,'GET','/signup');assert s==200;results['chat_signup_page']=s
 s,h,b=req(3000,'POST','/signup','username=formaluser&email=formal%40example.test&display_name=Formal&password=lesson-only-password');assert s==303;results['chat_signup']=s
 s,h,b=req(3000,'POST','/login','username=FORMALUSER&password=lesson-only-password');assert s==303;cookie=h['set-cookie'].split(';')[0];results['chat_login']=s
 assert req(3000,'GET','/account',cookie=cookie)[0]==200
 stop(p);p=start('server',3000);assert req(3000,'GET','/account',cookie=cookie)[0]==200;results['chat_session_process_restart']=True
 assert req(3000,'GET','/account')[0]==401;results['chat_unauthenticated_account']=401
 results['chat_unprotected_room_page_status']=req(3000,'GET','/room/formal-unauthed')[0]
 stop(p)
 p=start('stats-server',3100)
 s,h,b=req(3100,'POST','/login','username=admin&key='+'a'*64);assert s==422;results['analytics_legacy_key_rejected']=s
 s,h,b=req(3100,'POST','/login','username=admin&password=lesson-only-password');assert s==303 and h['location']=='/change-password';cookie=h['set-cookie'].split(';')[0];results['analytics_password_login']=s
 assert req(3100,'GET','/change-password',cookie=cookie)[0]==200
 assert req(3100,'GET','/api/v1/events/web_chat/summary',cookie=cookie)[0]==403;results['analytics_restricted_query']=403
 stop(p);p=start('stats-server',3100);assert req(3100,'GET','/change-password',cookie=cookie)[0]==200;results['analytics_session_process_restart']=True
 assert req(3100,'POST','/logout','',cookie)[0]==303
 assert req(3100,'GET','/change-password',cookie=cookie)[0]==401;results['analytics_logout_revocation']=401
finally:
 for p,f in children:
  if p.poll() is None:stop(p)
  f.close()
 (logs/'http-results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(results,ensure_ascii=False))
