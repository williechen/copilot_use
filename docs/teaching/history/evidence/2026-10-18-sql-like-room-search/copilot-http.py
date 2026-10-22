import http.client,json,urllib.parse
from pathlib import Path
root=3111;cases=[]
def call(method,path,body=None,extra=None):
 c=http.client.HTTPConnection('127.0.0.1',root,timeout=30);h={'Content-Type':'application/json','X-Copilot-CSRF':'1'};h.update(extra or {});c.request(method,path,json.dumps(body) if body is not None else None,h);r=c.getresponse();v=(r.status,dict(r.getheaders()),r.read().decode());c.close();return v
for i,(name,private) in enumerate([('Rust 技術交流',False),('Rust 技術交流',False),('Rust 技術交流延伸',False),('生活閒聊',False),('100% 活動',False),('Rust 私密房',True)]):
 r=call('POST','/api/v1/rooms',{'id':'sql-like-'+str(i),'name':name,'is_private':private},{'Authorization':'Bearer fixture-chat-api-p0'});assert r[0]==201,r
r=call('POST','/api/v1/auth/login',{'username':'BrowserTrial','password':'fixture-password-12345'});assert r[0]==200;cookie=r[1]['set-cookie'].split(';')[0]
for path,headers,n in [('/api/v1/public/rooms',{},3),('/api/v1/rooms',{'Cookie':cookie},3),('/api/v1/rooms',{'Authorization':'Bearer fixture-chat-api-p0'},4)]:
 r=call('GET',path+'?'+urllib.parse.urlencode({'q':'rUsT'}),extra=headers);v=json.loads(r[2]);assert r[0]==200 and len(v)==n,(path,r);cases.append({'path':path,'expected':n,'actual':len(v),'scope':'admin' if 'Authorization' in headers else 'user/public'})
r=call('GET','/api/v1/rooms?q=Rust');assert r[0]==401;cases.append({'case':'anonymous protected query','status':r[0]})
r=call('GET','/api/v1/public/rooms?'+urllib.parse.urlencode({'q':'%'}));v=json.loads(r[2]);assert len(v)==1 and v[0]['name']=='100% 活動';cases.append({'case':'literal percent','rows':1})
Path('outputs/sql-like-room-search/copilot-http.json').write_text(json.dumps(cases,indent=2));print(len(cases),'product live HTTP cases passed')
