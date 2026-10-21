import os,pathlib,subprocess,json,time,socket,urllib.request,urllib.error,signal,sqlite3,base64
root=pathlib.Path.cwd();out=root/'work/day24-validation-logs';dist=root/'work/day24-validation-main/apps/desktop_pet/frontend/dist';db=root/'work/day24-validation-runtime/pet.sqlite';children=[];sid=None;results={}
env={k:os.environ[k] for k in ['PATH','HOME','XDG_RUNTIME_DIR','DBUS_SESSION_BUS_ADDRESS'] if k in os.environ};env.update(XDG_CACHE_HOME=str(root/'work/day24-validation-runtime/cache'),XDG_DATA_HOME=str(root/'work/day24-validation-runtime/data'),PET_DATABASE_URL=f'sqlite:{db}?mode=rwc',PET_ENABLE_ROAM='false')
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
 session();wait(lambda:'已載入儲存清單' in text())
 controls=ses('GET','/window')
 for name in ['驗證一號','驗證二號','驗證三號']:
  execute("const e=document.querySelector('input');e.value="+json.dumps(name)+";e.dispatchEvent(new Event('input',{bubbles:true}));Array.from(document.querySelectorAll('button')).find(x=>x.textContent.trim()==='建立寵物').click();return true")
  wait(lambda:name in text() and '已建立寵物' in text())
  wait(lambda:execute("return !Array.from(document.querySelectorAll('button')).find(x=>x.textContent.trim()==='建立寵物').disabled"))
 rows=sqlite3.connect(db).execute('SELECT id,name FROM pet_profile ORDER BY name').fetchall();assert len(rows)==3
 def press(id,label):
  ses('POST','/window',{'handle':controls})
  execute("const row=Array.from(document.querySelectorAll('li')).find(x=>x.textContent.includes("+json.dumps(id)+"));Array.from(row.querySelectorAll('button')).find(x=>x.textContent.trim()==="+json.dumps(label)+").click();return true")
  wait(lambda:execute("return !Array.from(document.querySelectorAll('button')).find(x=>x.textContent.trim()==='建立寵物').disabled"))
 press(rows[0][0],'選為生活窗寵物')
 for id,name in rows[1:]:press(id,'出籠')
 handles=wait(lambda:(h if len(h:=ses('GET','/window/handles'))==4 else False))
 life={};rects={}
 for h in handles:
  ses('POST','/window',{'handle':h})
  id=execute('return new URLSearchParams(location.search).get("pet_id")')
  if id:
   name=next(n for i,n in rows if i==id);wait(lambda:name in text() and '🐢' in text());life[id]=h;rects[id]=ses('GET','/window/rect')
   face_count=execute('return document.querySelectorAll(".pet-face").length')
   alerts=execute('return Array.from(document.querySelectorAll("[role=alert]")).map(x=>x.textContent)')
   results.setdefault('observed_life_windows',{})[id]={'faces':face_count,'alerts':alerts,'all_buttons_visible':execute('return Array.from(document.querySelectorAll("button")).every(b=>b.getBoundingClientRect().bottom<=innerHeight)')}
   print('window observation',id,face_count,alerts,flush=True)

   (out/('multi-'+id+'.png')).write_bytes(base64.b64decode(ses('GET','/screenshot')))
 assert len(life)==3;assert len({(r['x'],r['y']) for r in rects.values()})==3
 results['three_simultaneous_identity_windows']=True;results['ids']=[id for id,_ in rows];results['window_rects']=rects
 press(rows[0][0],'出籠');assert len(ses('GET','/window/handles'))==4;results['duplicate_open_focuses_existing']=True
 def ipc(command,args):
  result=ses('POST','/execute/async',{'script':'const done=arguments[arguments.length-1];window.__TAURI__.core.invoke('+json.dumps(command)+','+json.dumps(args)+').then(v=>done({ok:true,value:v})).catch(e=>done({ok:false,error:String(e)}));','args':[]})
  assert result['ok'],result;return result.get('value')
 ses('POST','/window',{'handle':controls})
 ipc('send_saved_pet_command',{'id':rows[0][0],'command':'sleep'})
 states=[ipc('get_saved_pet_state',{'id':id})['state'] for id,_ in rows];assert states==['sleeping','idle','idle'];results['three_independent_states']=True
 # Sample the actual sleep frames, not a patched animation.
 ses('POST','/window',{'handle':life[rows[0][0]]})
 results['sleep_animation_text']=execute('return Array.from(document.querySelectorAll(".pet-face")).map(x=>x.textContent)')
 ses('POST','/window',{'handle':controls})
 # Caging is a real targeted close, not deleting any database profile.
 ses('POST','/window',{'handle':life[rows[0][0]]})
 execute("Array.from(document.querySelectorAll('button')).find(x=>x.textContent.trim()==='回籠子').click();return true")
 ses('POST','/window',{'handle':controls})
 wait(lambda:len(ses('GET','/window/handles'))==3)
 assert sqlite3.connect(db).execute('SELECT count(*) FROM pet_profile').fetchone()[0]==3
 for id,_ in rows[1:]:assert ipc('get_saved_pet_state',{'id':id})['state']=='idle'
 results['cage_one_preserves_other_two_and_data']=True
 press(rows[0][0],'出籠');wait(lambda:len(ses('GET','/window/handles'))==4)
 rpc('DELETE','/session/'+sid);sid=None;time.sleep(1)
 session();wait(lambda:len(ses('GET','/window/handles'))==2)
 found=False
 for h in ses('GET','/window/handles'):
  ses('POST','/window',{'handle':h})
  if execute('return new URLSearchParams(location.search).get("pet_id")')==rows[0][0]:
   wait(lambda:rows[0][1] in text());found=True
 assert found;results['restart_explicit_default_identity']=True

finally:
 if sid:
  try:rpc('DELETE','/session/'+sid)
  except Exception:pass
 for p,f in reversed(children):
  if p.poll() is None:
   try:os.killpg(p.pid,signal.SIGTERM);p.wait(timeout=10)
   except Exception:os.killpg(p.pid,signal.SIGKILL)
  f.close()
 (out/'day24-pet-multi-results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(results))
