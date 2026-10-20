import os,pathlib,subprocess,json,time,socket,urllib.request,urllib.error,signal,sqlite3,base64
root=pathlib.Path.cwd();out=root/'work/day24-logs';dist=root/'work/day24-source/apps/desktop_pet/frontend/dist';db=root/'work/day24-runtime/pet.sqlite';children=[];sid=None;results={}
env={k:os.environ[k] for k in ['PATH','HOME','XDG_RUNTIME_DIR','DBUS_SESSION_BUS_ADDRESS'] if k in os.environ};env.update(PET_DATABASE_URL=f'sqlite:{db}?mode=rwc',PET_ENABLE_ROAM='false')
def rpc(method,path,data=None):
 port=4456 if path.startswith('/session/') else 4455
 r=urllib.request.Request(f'http://127.0.0.1:{port}'+path,data=None if data is None else json.dumps(data).encode(),method=method,headers={'Content-Type':'application/json'})
 try:
  with urllib.request.urlopen(r,timeout=25) as response:d=json.load(response)
 except urllib.error.HTTPError as e:raise RuntimeError(e.read().decode())
 if isinstance(d.get('value'),dict) and d['value'].get('error'):raise RuntimeError(str(d['value']))
 return d.get('value')
def ses(method,path,data=None):return rpc(method,'/session/'+sid+path,data)
def execute(script):return ses('POST','/execute/sync',{'script':script,'args':[]})
def text():return execute('return document.body.innerText')
def wait(fn,seconds=25):
 end=time.monotonic()+seconds;last=None
 while time.monotonic()<end:
  try:
   last=fn()
   if last:return last
  except Exception as e:last=str(e)
  time.sleep(.2)
 raise RuntimeError('timeout: '+str(last))
def start(command,name):
 f=(out/(name+'.log')).open('w');p=subprocess.Popen(command,env=env,stdout=f,stderr=subprocess.STDOUT,start_new_session=True);children.append((p,f));return p
for port in [1420,4455,4456]:
 with socket.socket() as s:assert s.connect_ex(('127.0.0.1',port))!=0,'occupied port'
try:
 start(['python3','-m','http.server','1420','--bind','127.0.0.1','--directory',str(dist)],'pet-assets')
 start(['xvfb-run','-a','tauri-driver','--port','4455','--native-port','4456'],'pet-driver')
 wait(lambda:rpc('GET','/status')['ready'])
 def session():
  global sid
  v=rpc('POST','/session',{'capabilities':{'alwaysMatch':{'tauri:options':{'application':'/home/willie/Documents/rust_ai_chat/target/debug/desktop-pet'}}}});sid=v['sessionId']
  for h in wait(lambda:ses('GET','/window/handles')):
   ses('POST','/window',{'handle':h})
   if wait(lambda:text()):
    if '寵物控制台' in text():return
  raise RuntimeError('control window missing')
 session();wait(lambda:'已載入儲存清單' in text());results['native_startup_list']=True
 execute("const e=document.querySelector('input');e.value='正式驗證寵物';e.dispatchEvent(new Event('input',{bubbles:true}));Array.from(document.querySelectorAll('button')).find(x=>x.textContent.trim()==='建立寵物').click();return true")
 wait(lambda:'已建立寵物：' in text());rows=sqlite3.connect(db).execute('SELECT id,name FROM pet_profile').fetchall();assert len(rows)==1 and rows[0][1]=='正式驗證寵物';assert rows[0][0] in text();results['real_ui_ipc_sqlite_create']=True
 execute("Array.from(document.querySelectorAll('button')).find(x=>x.textContent.trim()==='選為生活窗寵物').click();return true")
 wait(lambda:'生活窗選取已儲存' in text());results['control_select_sqlite']=True
 assert sqlite3.connect(db).execute('SELECT pet_id FROM local_pet_selection').fetchone()[0]==rows[0][0]
 def life():
  for h in ses('GET','/window/handles'):
   ses('POST','/window',{'handle':h})
   if execute('return location.search')=='?view=pet': return True
  return False
 assert wait(life)
 wait(lambda:'正式驗證寵物' in text());assert '🐢' in text();results['life_identity_event_and_tortoise']=True
 assert execute("return Array.from(document.querySelectorAll('[title]')).some(x=>x.title=="+json.dumps(rows[0][0])+")")
 (out/'day24-pet-life.png').write_bytes(base64.b64decode(ses('GET','/screenshot')))
 rpc('DELETE','/session/'+sid);sid=None;time.sleep(1)
 session();assert wait(life);wait(lambda:'正式驗證寵物' in text());assert '🐢' in text();results['process_restart_life_identity']=True

finally:
 if sid:
  try:rpc('DELETE','/session/'+sid)
  except Exception:pass
 for p,f in reversed(children):
  if p.poll() is None:
   try:os.killpg(p.pid,signal.SIGTERM);p.wait(timeout=10)
   except Exception:os.killpg(p.pid,signal.SIGKILL)
  f.close()
 (out/'day24-pet-gui-results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(results))
