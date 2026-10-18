import http.client, json, re, sys, time, uuid

def call(port, method, path, body=None, headers=None):
    c = http.client.HTTPConnection('127.0.0.1', port, timeout=10)
    c.request(method, path, body, headers or {})
    r = c.getresponse()
    result = (r.status, dict(r.getheaders()), r.read().decode())
    c.close()
    return result

def check(name, result, expected):
    assert result[0] == expected, (name, result)
    print(f'PASS {name}: HTTP {result[0]}')
    return result

cases=[]
def expect(label,method,path,status,body=None,headers=None,renew=False):
 r=call(3100,method,path,body,headers);assert r[0]==status,(label,r)
 cookie=r[1].get('set-cookie','');assert ('Max-Age=1800' in cookie)==renew,(label,cookie)
 cases.append({'case':label,'status':r[0],'cookie_renewed':renew});return r
h={'Origin':'http://127.0.0.1:3100','Content-Type':'application/x-www-form-urlencoded'}
r=expect('login','POST','/login',303,'key='+'a'*64,h,True);c=r[1]['set-cookie'].split(';')[0];auth={'Cookie':c}
r=expect('protected successful GET','GET','/dashboard',200,headers=auth,renew=True)
csrf=re.search('name="csrf" value="([^"]+)"',r[2]).group(1)
expect('summary renews','GET','/api/v1/events/web_chat/summary',200,headers=auth,renew=True)
expect('bad source does not renew','GET','/dashboard?source=invalid',400,headers=auth)
expect('public login page does not renew','GET','/login',200,headers=auth)
expect('invalid CSRF does not renew','POST','/logout',403,'csrf=wrong',{**auth,**h})
expect('missing session does not renew','GET','/api/v1/events/web_chat/summary',401)
expect('unknown session does not renew','GET','/api/v1/events/web_chat/summary',401,headers={'Cookie':'analytics_session='+'c'*64})
r=expect('logout clears cookie','POST','/logout',303,'csrf='+csrf,{**auth,**h});assert 'Max-Age=0' in r[1]['set-cookie']
expect('revoked session does not renew','GET','/api/v1/events/web_chat/summary',401,headers=auth)
from pathlib import Path
Path('outputs/full-2c7ace2/refresh-guards.json').write_text(json.dumps(cases,indent=2));print(len(cases),'cookie refresh guard cases passed')
