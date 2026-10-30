from pathlib import Path
import json,urllib.request,urllib.error,time,datetime,base64,sqlite3,subprocess
OUT=Path(__file__).resolve().parent.parent/'outputs/gate2-2026-10-31';CONF=json.loads((OUT/'browser-connection.json').read_text());SID=json.loads((OUT/'browser-session.json').read_text())['value']['sessionId'];BASE=CONF['base_url']+'/session/'+SID;EL='element-6066-11e4-a52e-4f735466cecf'
def api(method,path,data=None):
 t=time.time();req=urllib.request.Request(BASE+path,data=None if data is None else json.dumps(data,ensure_ascii=False).encode(),headers={'Content-Type':'application/json'},method=method)
 try:
  with urllib.request.urlopen(req,timeout=40) as response:r=json.load(response);status=response.status
 except urllib.error.HTTPError as error:r=json.loads(error.read());status=error.code
 value=r.get('value');logged=value;request_data=data
 if '/value' in path and method=='POST':request_data={'text':'<synthetic test input redacted>'}
 if path.endswith('/screenshot'):logged='<screenshot saved separately>'
 if path.endswith('/cookie'):logged='<cookie values retained only in test memory>'
 with (OUT/'browser-commands.jsonl').open('a') as f:f.write(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'method':method,'path':path,'data':request_data,'http_status':status,'seconds':time.time()-t,'value':logged},ensure_ascii=False)+'\n')
 if status>=400:raise RuntimeError(str(value))
 return value
def poll(fn,timeout=15):
 end=time.monotonic()+timeout;last=None
 while time.monotonic()<end:
  try:
   last=fn()
   if last:return last
  except Exception as e:last=e
  time.sleep(.1)
 raise AssertionError('timeout: '+str(last))
def find(xpath):return api('POST','/element',{'using':'xpath','value':xpath})[EL]
def body():return api('GET','/element/'+find('//body')+'/text')
def navigate(url):api('POST','/url',{'url':url})
def url():return api('GET','/url')
def click(xpath):api('POST','/element/'+find(xpath)+'/click',{})
def fill(name,value):
 e=find("//input[@name='"+name+"']");api('POST','/element/'+e+'/clear',{});api('POST','/element/'+e+'/value',{'text':value})
def screenshot(name):p=OUT/(name+'.png');p.write_bytes(base64.b64decode(api('GET','/screenshot')));return str(p)
def chat_sql(query,args=(),write=False):
 db=json.loads((OUT/'http-server-connection.json').read_text())['chat_database'];c=sqlite3.connect(db if write else 'file:'+db+'?mode=ro',uri=not write,timeout=5)
 try:
  result=c.execute(query,args).fetchall()
  if write:c.commit()
  return result
 finally:c.close()
def pg_scalar(query):return subprocess.check_output(['/usr/lib/postgresql/18/bin/psql','-h','127.0.0.1','-p','55482','-d','gate2_browser_20261031','-Atc',query],text=True).strip()
def record(project,name,evidence):
 p=OUT/(project+'-browser-checks.json');d=json.loads(p.read_text()) if p.exists() else {'source_sha':'2c7ff8b861d32ab6d879bf80c9f2dce50de38097','method':'real WebKitGTK browser, actual HTTP servers and isolated real DBs; no route/IPC mocks','passed':[]};d['passed'].append({'case':name,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'evidence':evidence});p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n');print('PASS',project,name,flush=True)
