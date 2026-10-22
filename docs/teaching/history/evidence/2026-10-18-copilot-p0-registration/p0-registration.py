"""Synthetic-only HTTP acceptance; start an isolated Chat service first."""
import http.client,json,sys,sqlite3,concurrent.futures
from pathlib import Path
port=int(sys.argv[1]);db=sys.argv[2];out=Path(sys.argv[3]);checks=[]
def call(path,body=None,cookie=None):
 c=http.client.HTTPConnection('127.0.0.1',port,timeout=60);h={'Content-Type':'application/json','X-Copilot-CSRF':'1'}
 if cookie:h['Cookie']=cookie
 c.request('POST' if body is not None else 'GET',path,json.dumps(body) if body is not None else None,h);r=c.getresponse();v=(r.status,dict(r.getheaders()),r.read().decode());c.close();return v
def check(name,r,status):
 checks.append({'case':name,'expected':status,'actual':r[0]});assert r[0]==status,(name,r);return r
def register(name,email,**extra):return call('/api/v1/auth/register',{'username':name,'password':'fixture-password-12345','email':email,**extra})
first=check('register with normalized unique email',register('TrialAlice',' ALICE@Example.test '),201);assert 'Max-Age=0' in first[1]['set-cookie']
check('no implicit login after registration',call('/api/v1/rooms'),401)
check('ASCII username uniqueness',register('trialalice','other@example.test'),409)
check('email uniqueness independent of case',register('TrialBob','alice@EXAMPLE.TEST'),409)
check('missing email rejected',call('/api/v1/auth/register',{'username':'NoEmail','password':'fixture-password-12345'}),422)
for i,email in enumerate(['invalid','a@@b.test','a b@x.test','a@-bad.test','a..b@x.test']):check('invalid mailbox '+str(i),register('Invalid'+str(i),email),400)
login=check('login with different account case',call('/api/v1/auth/login',{'username':'tRiAlAlIcE','password':'fixture-password-12345'}),200);cookie=login[1]['set-cookie'].split(';')[0]
check('authenticated room list',call('/api/v1/rooms',cookie=cookie),200)
r=check('registration revokes the current browser session',call('/api/v1/auth/register',{'username':'TrialSecond','password':'fixture-password-12345','email':'second@example.test'},cookie),201);assert 'Max-Age=0' in r[1]['set-cookie'];check('revoked old cookie denied',call('/api/v1/rooms',cookie=cookie),401)
with concurrent.futures.ThreadPoolExecutor(max_workers=2) as e:
 rs=list(e.map(lambda pair:register(*pair),[('TrialRaceOne','race@example.test'),('TrialRaceTwo','RACE@example.test')]))
assert sorted(r[0] for r in rs)==[201,409];checks.append({'case':'concurrent email uniqueness','actual':sorted(r[0] for r in rs)})
c=sqlite3.connect(db);email,hashed=c.execute("select email,password_hash from chat_users where username='TrialAlice'").fetchone();assert email=='alice@example.test' and hashed.startswith('$argon2') and 'fixture-password' not in hashed;c.close();checks.append({'case':'normalized email and salted hash persisted','passed':True})
out.write_text(json.dumps(checks,indent=2));print(len(checks),'HTTP/schema checks passed')
