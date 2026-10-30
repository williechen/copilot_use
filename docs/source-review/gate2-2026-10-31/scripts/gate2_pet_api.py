from pathlib import Path
import json,urllib.request,urllib.error,time,datetime,sqlite3,base64,re,subprocess,os,ctypes
OUT=Path(__file__).resolve().parent.parent/'outputs/gate2-2026-10-31';CONF=json.loads((OUT/'driver-connection.json').read_text());SID=json.loads((OUT/'session.json').read_text())['value']['sessionId'];BASE=CONF['base_url']+'/session/'+SID;EL='element-6066-11e4-a52e-4f735466cecf';MAIN=None
def api(method,path,data=None):
 raw=None if data is None else json.dumps(data,ensure_ascii=False).encode();t=time.time();req=urllib.request.Request(BASE+path,data=raw,headers={'Content-Type':'application/json'},method=method)
 try:
  with urllib.request.urlopen(req,timeout=30) as response:r=json.load(response);status=response.status
 except urllib.error.HTTPError as error:r=json.loads(error.read());status=error.code
 value=r.get('value');logged=('<screenshot base64; separately saved>' if path.endswith('/screenshot') else value)
 with (OUT/'webdriver-commands.jsonl').open('a') as f:f.write(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'session':SID,'method':method,'path':path,'data':data,'status':status,'seconds':time.time()-t,'value':logged},ensure_ascii=False)+'\n')
 if status>=400:raise RuntimeError(f'{method} {path}: {value}')
 return value
def poll(fn,timeout=15):
 end=time.monotonic()+timeout;last=None
 while time.monotonic()<end:
  try:
   last=fn()
   if last:return last
  except Exception as e:last=e
  time.sleep(.15)
 raise AssertionError(f'timed out: {last}')
def find(xpath):return api('POST','/element',{'using':'xpath','value':xpath})[EL]
def text(xpath):return api('GET','/element/'+find(xpath)+'/text')
def handles():return api('GET','/window/handles')
def switch(h):api('POST','/window',{'handle':h})
def main():
 global MAIN
 if MAIN is None:
  for h in handles():
   switch(h)
   if 'pet_id=' not in api('GET','/url'):MAIN=h;break
  assert MAIN is not None
 else:switch(MAIN)
def button(label,prefix=None):
 main();parent='' if prefix is None else f"//li[contains(.,'{prefix}')]";xp=parent+f"//button[normalize-space()='{label}']";e=poll(lambda:(lambda i:i if api('GET','/element/'+i+'/enabled') else None)(find(xp)));api('POST','/element/'+e+'/click',{})
 poll(lambda:api('GET','/element/'+find("//button[normalize-space()='建立寵物']")+'/enabled'))
def row(prefix):main();return text(f"//li[contains(.,'{prefix}')]")
def screenshot(name):
 p=OUT/(name+'.png');p.write_bytes(base64.b64decode(api('GET','/screenshot')));return str(p)
def sql(query,args=(),write=False):
 db=CONF['database'];c=sqlite3.connect(db if write else 'file:'+db+'?mode=ro',uri=not write,timeout=5)
 try:
  cursor=c.execute(query,args);value=cursor.fetchall()
  if write:c.commit()
  return value
 finally:c.close()
def pets():return sql("SELECT id,name,is_default_closure FROM pet_profile WHERE owner_id='local-owner' ORDER BY name")
def flags(id):return sql("SELECT activity_type,is_activity FROM pet_activity WHERE pet_id=? AND owner_id='local-owner' ORDER BY activity_type,id",(id,))
def native_windows():
 value=subprocess.check_output(['xwininfo','-root','-tree'],env={**os.environ,'DISPLAY':CONF['display'],'LC_ALL':'C.UTF-8'},text=True)
 return {name:int(x,16) for x,name in re.findall(r'^\s*(0x[0-9a-f]+)\s+"([^"]*)"',value,re.M)}
def geometry(name):
 wid=native_windows()[name];s=subprocess.check_output(['xwininfo','-id',hex(wid)],env={**os.environ,'DISPLAY':CONF['display'],'LC_ALL':'C.UTF-8'},text=True)
 return {'id':hex(wid),'x':int(re.search(r'Absolute upper-left X:\s*(-?\d+)',s).group(1)),'y':int(re.search(r'Absolute upper-left Y:\s*(-?\d+)',s).group(1)),'width':int(re.search(r'Width:\s*(\d+)',s).group(1)),'height':int(re.search(r'Height:\s*(\d+)',s).group(1)),'viewable':'Map State: IsViewable' in s}
def native_close(name):
 class Data(ctypes.Union):_fields_=[('b',ctypes.c_char*20),('s',ctypes.c_short*10),('l',ctypes.c_long*5)]
 class Message(ctypes.Structure):_fields_=[('type',ctypes.c_int),('serial',ctypes.c_ulong),('send_event',ctypes.c_int),('display',ctypes.c_void_p),('window',ctypes.c_ulong),('message_type',ctypes.c_ulong),('format',ctypes.c_int),('data',Data)]
 class Event(ctypes.Union):_fields_=[('xclient',Message),('pad',ctypes.c_long*24)]
 lib=ctypes.CDLL('libX11.so.6');lib.XOpenDisplay.argtypes=[ctypes.c_char_p];lib.XOpenDisplay.restype=ctypes.c_void_p;lib.XInternAtom.argtypes=[ctypes.c_void_p,ctypes.c_char_p,ctypes.c_int];lib.XInternAtom.restype=ctypes.c_ulong;lib.XSendEvent.argtypes=[ctypes.c_void_p,ctypes.c_ulong,ctypes.c_int,ctypes.c_long,ctypes.POINTER(Event)];lib.XFlush.argtypes=[ctypes.c_void_p];lib.XCloseDisplay.argtypes=[ctypes.c_void_p]
 display=lib.XOpenDisplay(CONF['display'].encode());assert display;window=native_windows()[name];event=Event();event.xclient.type=33;event.xclient.send_event=1;event.xclient.display=display;event.xclient.window=window;event.xclient.message_type=lib.XInternAtom(display,b'WM_PROTOCOLS',0);event.xclient.format=32;event.xclient.data.l[0]=lib.XInternAtom(display,b'WM_DELETE_WINDOW',0);event.xclient.data.l[1]=0
 assert lib.XSendEvent(display,window,0,0,ctypes.byref(event))!=0;lib.XFlush(display);lib.XCloseDisplay(display)
 with (OUT/'native-events.jsonl').open('a') as f:f.write(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'display':CONF['display'],'name':name,'window':hex(window),'event':'WM_PROTOCOLS/WM_DELETE_WINDOW'},ensure_ascii=False)+'\n')
def record(name,evidence):
 p=OUT/'pet-gui-checks.json';d=json.loads(p.read_text()) if p.exists() else {'source_sha':CONF['source_sha'],'method':'actual Tauri app, embedded release frontend, WebKitWebDriver and private Xvfb; no IPC mock','passed':[]};d['passed'].append({'case':name,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'evidence':evidence});p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n');print('PASS',name,flush=True)
