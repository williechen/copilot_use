import http.client,json

def request(method,path,body=None,cookie=None,origin='http://127.0.0.1:3100'):
 c=http.client.HTTPConnection('127.0.0.1',3100,timeout=5);headers={}
 if body is not None:headers.update({'Content-Type':'application/x-www-form-urlencoded','Origin':origin})
 if cookie:headers['Cookie']=cookie
 c.request(method,path,body,headers);r=c.getresponse();data=r.read().decode();h=dict(r.getheaders());c.close();return r.status,h,data
results={}
s,h,b=request('GET','/login');assert s==200 and '帳密登入' in b;results['login_page']=s
s,h,b=request('POST','/login','username=admin&key='+'a'*64);assert s==422;results['legacy_key_rejected']=s
s,h,b=request('POST','/login','username=admin&password=lesson-only-password',origin='http://evil.invalid');assert s==403;results['wrong_origin']=s
s,h,b=request('POST','/login','username=admin&password=lesson-only-password');assert s==303 and h['location']=='/change-password';cookie=h['set-cookie'].split(';')[0];assert 'HttpOnly' in h['set-cookie'];results['password_login']=s
s,h,b=request('GET','/change-password',cookie=cookie);assert s==200 and '尚未提供改密碼提交' in b;results['restricted_page']=s
s,h,b=request('GET','/api/v1/events/web_chat/summary',cookie=cookie);assert s==403;results['restricted_query']=s
s,h,b=request('POST','/logout','',cookie=cookie);assert s==303 and 'Max-Age=0' in h['set-cookie'];results['logout']=s
s,h,b=request('GET','/change-password',cookie=cookie);assert s==401;results['revoked_cookie']=s
print(json.dumps({'transport':'real TCP 127.0.0.1:3100; dedicated PostgreSQL 18.6','checks':results,'raw_tokens_logged':False},ensure_ascii=False,indent=2))
