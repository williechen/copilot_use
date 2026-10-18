import http.client,json,sqlite3
from pathlib import Path
def call(p,m,u,b=None,h={}):
 c=http.client.HTTPConnection('127.0.0.1',p);c.request(m,u,b,h);r=c.getresponse();o={'status':r.status,'body':r.read().decode()};c.close();return o
cases=[]
for label,m,u,b,expected in [('signin','GET','/signin',None,404),('history','GET','/api/messages',None,404),('blank name','POST','/api/rooms','   ',400),('long name','POST','/api/rooms','x'*201,400),('NUL name','POST','/api/rooms','x\x00',400)]:
 if m=='POST':r=call(3000,m,u,json.dumps({'name':b}),{'Content-Type':'application/json'})
 else:r=call(3000,m,u)
 cases.append({'case':label,'expected':expected,**r});assert r['status']==expected
r=call(3100,'POST','/login','username=admin&password=admin',{'Content-Type':'application/x-www-form-urlencoded','Origin':'http://127.0.0.1:3100'});cases.append({'case':'username/password','expected':422,**r});assert r['status']==422
for name,db,tables in [('chat','work/chat-full2c.sqlite',['chat_token_log','chat_send_log']),('pet','work/pet-native2c.sqlite',['pet_token_log','pet_send_log'])]:
 c=sqlite3.connect(db);cases.append({'case':name+' protocol tables after live requests','rows':{t:c.execute(f'select count(*) from {t}').fetchone()[0] for t in tables}})
Path('outputs/full-2c7ace2/additional-http.json').write_text(json.dumps(cases,ensure_ascii=False,indent=2));print(json.dumps(cases,ensure_ascii=False,indent=2))
