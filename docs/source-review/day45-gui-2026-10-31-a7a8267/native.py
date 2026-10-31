import urllib.request,json,time,base64,sqlite3
from pathlib import Path
out=Path('work/day45-gui');session=json.loads((out/'tauri-session.json').read_text())['value']['sessionId'];base='http://127.0.0.1:4444/session/'+session
results={'source_sha':'a7a8267f8c2f9003dd0d91d9a8cd25daba2517e4','kind':'real Linux Tauri wry WebKitWebDriver on isolated Xvfb :97; real product IPC and SQLite','steps':[],'errors':[]}
def req(method,path='',value=None):
 data=None if value is None else json.dumps(value).encode();r=urllib.request.urlopen(urllib.request.Request(base+path,data=data,method=method,headers={'Content-Type':'application/json'}),timeout=35);d=json.load(r)
 if isinstance(d.get('value'),dict) and 'error' in d['value']:raise RuntimeError(d['value'])
 return d.get('value')
def js(script,args=[]):return req('POST','/execute/sync',{'script':script,'args':args})
def asyncjs(script,args=[]):return req('POST','/execute/async',{'script':script,'args':args})
def wait(fn,timeout=15):
 end=time.time()+timeout
 while time.time()<end:
  try:
   x=fn()
   if x:return x
  except Exception:pass
  time.sleep(.2)
 raise RuntimeError('condition timeout')
def elem(css):return req('POST','/element',{'using':'css selector','value':css})['element-6066-11e4-a52e-4f735466cecf']
def click(css):return req('POST','/element/'+elem(css)+'/click',{})
def click_text(text):
 e=req('POST','/element',{'using':'xpath','value':'//button[normalize-space()="'+text+'"]'})['element-6066-11e4-a52e-4f735466cecf'];req('POST','/element/'+e+'/click',{})
def save(name,data={}):results['steps'].append({'name':name,**data});(out/'native-results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n');print(name,json.dumps(data,ensure_ascii=False),flush=True)
def db(q):
 c=sqlite3.connect('/tmp/day45-gui-pet.sqlite');r=c.execute(q).fetchall();c.close();return r
def shot(name):(out/name).write_bytes(base64.b64decode(req('GET','/screenshot')))
try:
 main=None
 for h in req('GET','/window/handles'):
  req('POST','/window',{'handle':h})
  if js('return document.querySelector("h1")?.textContent')=='寵物控制台': main=h;break
 assert main
 shot('pet-native-control.png');save('native controller real Leptos WASM loaded',{'handle':main,'title':req('GET','/title')})
 if not db('SELECT id FROM pet_profile'):
  field=elem('label input');req('POST','/element/'+field+'/value',{'text':'Native Day45 A'});click_text('建立寵物');wait(lambda:len(db('SELECT id FROM pet_profile'))==1)
 wait(lambda:len(req('GET','/window/handles'))==2);petid=db('SELECT id FROM pet_profile')[0][0];handles=req('GET','/window/handles');life=next(h for h in handles if h!=main);save('create through UI persists pet and opens separate native life window',{'pet_id':petid,'native_window_count':len(handles)})
 req('POST','/window',{'handle':life});wait(lambda:js('return document.querySelectorAll(".pet-face img").length')==1);rect=req('GET','/window/rect');bounds=js('const f=document.querySelector(".pet-face").getBoundingClientRect();const i=document.querySelector(".pet-face img").getBoundingClientRect();return {canvas:[f.width,f.height],image:[i.x,i.y,i.width,i.height],imageCount:document.querySelectorAll(".pet-face img").length,client:[innerWidth,innerHeight],body:document.body.innerText};');assert rect['width']==200 and rect['height']==200,rect;assert bounds['canvas']==[200,200],bounds;assert bounds['imageCount']==1;shot('pet-native-life.png');save('real native200x200 and single-image life canvas',{'window_rect':rect,'DOM':bounds})
 # Observe real clock-driven classes, without replacing the DOM or its bridge.
 classes=set()
 for _ in range(20):classes.add(js('return document.querySelector(".pet-face img").className'));time.sleep(.15)
 save('actual animation clock runs in native WebView',{'classes':sorted(classes)})
 # Drive actual native controller buttons and observe the independent life WebView.
 for action,prefix in [('互動','pose-interact'),('結束','pose-idle'),('睡覺','pose-sleep'),('醒來','pose-idle')]:
  req('POST','/window',{'handle':main});click_text(action);req('POST','/window',{'handle':life});wait(lambda:js('return document.querySelector(".pet-face img").className').startswith(prefix))
  poses=set()
  for _ in range(12):
   data=js('const i=document.querySelector(".pet-face img"),r=i.getBoundingClientRect();return {pose:i.className,x:r.x,y:r.y,w:r.width,h:r.height}');assert data['x']>=0 and data['y']>=0 and data['x']+data['w']<=200.5 and data['y']+data['h']<=200.5,data;poses.add(data['pose']);time.sleep(.15)
  save('real native UI action '+action,{'observed_poses':sorted(poses)});shot('pet-native-'+prefix+'.png')
 click_text('控制台');wait(lambda:len(req('GET','/window/handles'))==2);req('POST','/window',{'handle':main});save('life-window control button restores same main native window')
 click_text('出籠');assert len(req('GET','/window/handles'))==2;save('repeat uncage focuses existing native life window, no duplicate')
 req('POST','/window',{'handle':life});click_text('回籠子');wait(lambda:len(req('GET','/window/handles'))==1);save('life cage button closes native window after persistence',{'db':db('SELECT * FROM pet_activity')})
 req('POST','/window',{'handle':main});click_text('出籠');wait(lambda:len(req('GET','/window/handles'))==2);life=next(h for h in req('GET','/window/handles') if h!=main);req('POST','/window',{'handle':life});asyncjs('const done=arguments[arguments.length-1];window.__TAURI__.window.getCurrentWindow().close().then(()=>done(true)).catch(e=>done({error:String(e)}));');wait(lambda:len(req('GET','/window/handles'))==1);save('native CloseRequested route cages and destroys life window',{'db':db('SELECT * FROM pet_activity')})
 req('POST','/window',{'handle':main});shot('pet-native-final.png')
except Exception as e:
 import traceback
 results['errors'].append(traceback.format_exc());print(results['errors'][-1],flush=True)
 try:shot('pet-native-error.png');(out/'pet-native-error-dom.json').write_text(json.dumps(js('return {url:location.href,body:document.body.innerText,html:document.body.innerHTML}'),ensure_ascii=False,indent=2))
 except Exception:pass
finally:(out/'native-results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n')
if results['errors']:raise SystemExit(1)
