from pathlib import Path
import urllib.request,urllib.parse,urllib.error,json,sys
out=Path('outputs/sql-like-room-search');root='http://127.0.0.1:3000';cases=[]
if '--seed' in sys.argv:
 assert not json.load(urllib.request.urlopen(root+'/api/rooms')), 'seed requires empty fixture DB'
 for name in ['Rust 技術交流','Rust 技術交流','Rust 技術交流延伸','RUST 中文長房名','生活閒聊','100% 活動','under_score',r'back\slash',"SQL 'quote'"]:
  r=urllib.request.urlopen(urllib.request.Request(root+'/api/rooms',data=json.dumps({'name':name}).encode(),headers={'Content-Type':'application/json'}));assert r.status==201
for q,n in [('Rust 技術交流',3),(' 技術 ',3),('rUsT',4),(' ',9),('不存在',0),('%',1),('_',1),('\\',1),("'quote'",1),("' OR 1=1 --",0)]:
 u=root+'/api/rooms?'+urllib.parse.urlencode({'q':q});v=json.load(urllib.request.urlopen(u));assert len(v)==n,(q,len(v),n);cases.append({'query':q,'expected':n,'actual':len(v),'rooms':v})
for q in ['x'*201,'bad\0query']:
 try:urllib.request.urlopen(root+'/api/rooms?'+urllib.parse.urlencode({'q':q}));raise AssertionError('invalid query accepted')
 except urllib.error.HTTPError as e:assert e.code==400;cases.append({'query':'invalid long/NUL','status':e.code})
out.joinpath('http.json').write_text(json.dumps(cases,ensure_ascii=False,indent=2));print(len(cases),'HTTP cases passed')
