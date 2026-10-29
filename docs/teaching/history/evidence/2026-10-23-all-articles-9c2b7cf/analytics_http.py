import os,pathlib,subprocess,time,socket,json,re,requests
from html.parser import HTMLParser
root=pathlib.Path(__file__).resolve().parent
run=root/'analytics-runtime';run.mkdir(exist_ok=True)
env={k:os.environ[k] for k in ['PATH','HOME'] if k in os.environ}
env.update(ANALYTICS_DATABASE_URL='postgres://validation_audit@127.0.0.1:55430/postgres',ANALYTICS_CHAT_TOKEN='audit-chat-token',ANALYTICS_PET_TOKEN='audit-pet-token')
rows=[]
def check(name,actual,expected):
 rows.append(dict(name=name,actual=actual,expected=expected,passed=actual==expected))
 assert actual==expected,(name,actual,expected)
class Links(HTMLParser):
 def __init__(self):super().__init__();self.links=[]
 def handle_starttag(self,tag,attrs):
  if tag=='a':self.links.append(dict(attrs))
def http(method,path,**kw):return requests.request(method,'http://127.0.0.1:3100'+path,timeout=10,allow_redirects=False,**kw)
with socket.socket() as probe:
 if probe.connect_ex(('127.0.0.1',3100))==0:raise RuntimeError('port 3100 already occupied')
binary='/home/willie/Documents/Codex/2026-10-23/referenced-chatgpt-conversation-this-is-an-2/work/target-full/debug/stats-server'
log=open(root/'logs/analytics-http-runtime.log','w')
p=subprocess.Popen([binary],cwd=run,env=env,stdout=log,stderr=subprocess.STDOUT)
try:
 for _ in range(100):
  if p.poll() is not None:raise RuntimeError('server exited')
  try:
   if http('GET','/login').status_code==200:break
  except requests.ConnectionError:time.sleep(.1)
 else:raise TimeoutError()
 r=http('GET','/dashboard')
 check('anonymous dashboard redirects to login',(r.status_code,r.headers.get('Location')),(303,'/login'))
 origin={'Origin':'http://127.0.0.1:3100'}
 r=http('POST','/login',headers=origin,data={'username':'admin','password':'validation-only-password-123'})
 check('initial login',r.status_code,303)
 c=r.headers['Set-Cookie'].split(';')[0];h=origin|{'Cookie':c}
 r=http('GET','/dashboard',headers=h)
 check('first login forced change',r.headers.get('Location'),'/change-password')
 page=http('GET','/change-password',headers=h).text
 csrf=re.search(r'name="csrf" value="([^"]+)"',page).group(1)
 r=http('POST','/change-password',headers=h,data=dict(current_password='validation-only-password-123',new_password='validation-new-password-456',confirm_password='validation-new-password-456',email='audit@example.test',csrf=csrf))
 check('password change',r.status_code,303)
 r=http('GET','/dashboard',headers=h)
 check('old session revoked',(r.status_code,r.headers.get('Location')),(303,'/login'))
 r=http('POST','/login',headers=origin,data={'username':'admin','password':'validation-new-password-456'})
 check('login with changed credential',r.status_code,303)
 h=origin|{'Cookie':r.headers['Set-Cookie'].split(';')[0]}
 for source,label in [('web_chat','Chat'),('desktop_pet','Pet')]:
  r=http('GET','/dashboard?source='+source,headers=h)
  check(source+' dashboard',r.status_code,200)
  links=Links();links.feed(r.text)
  active=[x['href'] for x in links.links if x.get('aria-current')=='page']
  check(source+' only active link',active,['/dashboard?source='+source])
  check(source+' filtered total',label+' 已提交事件總數 0' in r.text,True)
 check('unknown source rejected',http('GET','/dashboard?source=unknown',headers=h).status_code,400)
finally:
 p.terminate()
 try:p.wait(timeout=5)
 except subprocess.TimeoutExpired:p.kill();p.wait()
 log.close()
 (root/'logs/analytics-http-results.json').write_text(json.dumps({'source_sha':'9c2b7cf5a4c60a08254445911916c4ed9b02d7b3','checks':rows},ensure_ascii=False,indent=2))
print(json.dumps(rows,ensure_ascii=False,indent=2))
