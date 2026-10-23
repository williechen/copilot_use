import os,pathlib,subprocess,time,socket,json,sqlite3,requests,struct,uuid,hashlib
root=pathlib.Path(__file__).resolve().parent
run=root/'chat-runtime';run.mkdir(exist_ok=True)
db=run/f'chat-{uuid.uuid4().hex}.sqlite'
binary=pathlib.Path('/home/willie/Documents/Codex/2026-10-23/referenced-chatgpt-conversation-this-is-an-2/work/target-full/debug/server')
env={k:os.environ[k] for k in ['PATH','HOME'] if k in os.environ}
env.update(CHAT_DATABASE_URL='sqlite:'+str(db)+'?mode=rwc',LEPTOS_OUTPUT_NAME='ui')
rows=[]
def check(name,actual,expected):
 rows.append(dict(name=name,actual=actual,expected=expected,passed=actual==expected))
 assert actual==expected,(name,actual,expected)
def http(method,path,**kw):
 return requests.request(method,'http://127.0.0.1:3000'+path,timeout=10,allow_redirects=False,**kw)
def upgrade(room,cookie=None):
 s=socket.create_connection(('127.0.0.1',3000),timeout=4)
 h=f'GET /ws/{room} HTTP/1.1\r\nHost: 127.0.0.1:3000\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nOrigin: http://127.0.0.1:3000\r\n'
 if cookie:h+='Cookie: '+cookie+'\r\n'
 s.sendall((h+'\r\n').encode());response=b''
 while b'\r\n\r\n' not in response:response+=s.recv(4096)
 return int(response.split(b' ')[1]),s
def ws_send(s,value):
 b=json.dumps(value).encode();mask=b'abcd'
 head=bytes([0x81,0x80|len(b)]) if len(b)<126 else bytes([0x81,0xfe])+struct.pack('!H',len(b))
 s.sendall(head+mask+bytes(c^mask[i%4] for i,c in enumerate(b)))
def read_n(s,n):
 b=b''
 while len(b)<n:
  c=s.recv(n-len(b))
  if not c:raise EOFError()
  b+=c
 return b
def ws_recv(s):
 a,b=read_n(s,2);n=b&127
 if n==126:n=struct.unpack('!H',read_n(s,2))[0]
 if n==127:n=struct.unpack('!Q',read_n(s,8))[0]
 return json.loads(read_n(s,n))
with socket.socket() as probe:
 if probe.connect_ex(('127.0.0.1',3000))==0:raise RuntimeError('port 3000 already occupied; will not touch existing service')
log=open(root/'logs/chat-http-runtime.log','w')
p=subprocess.Popen([str(binary)],cwd=run,env=env,stdout=log,stderr=subprocess.STDOUT)
sockets=[]
try:
 for _ in range(100):
  if p.poll() is not None:raise RuntimeError('server exited')
  try:
   if http('GET','/health').status_code==200:break
  except requests.ConnectionError:time.sleep(.1)
 else:raise TimeoutError('server health')
 origin={'Origin':'http://127.0.0.1:3000'}
 check('signin SSR form',all(t in http('GET','/signin').text for t in ['action="/login"','name="username"','name="password"']),True)
 r=http('POST','/signup',headers=origin,data=dict(username='audit_member',email='audit@example.test',display_name='Audit',password='validation-password-123'))
 check('signup',r.status_code,303)
 check('signup clears session cookie','chat_session=;' in r.headers.get('Set-Cookie','') and 'Max-Age=0' in r.headers.get('Set-Cookie',''),True)
 r=http('POST','/api/rooms',json={'name':'Isolated audit room'})
 check('anonymous room create remains public',r.status_code,201)
 room=r.json()['id']
 check('anonymous room page',http('GET','/room/'+room).status_code,401)
 for name,c in [('anonymous WS',None),('forged WS','chat_session='+'b'*64)]:
  code,s=upgrade(room,c);s.close();check(name,code,401)
 r=http('POST','/login',headers=origin,data=dict(username='audit_member',password='validation-password-123'))
 check('login',r.status_code,303)
 cookie=r.headers['Set-Cookie'].split(';')[0]
 check('authenticated room page',http('GET','/room/'+room,headers={'Cookie':cookie}).status_code,200)
 for _ in range(2):
  code,s=upgrade(room,cookie);check('valid session WS upgrade',code,101);sockets.append(s)
 ws_send(sockets[0],{'type':'send_message','data':{'room_id':room,'content':'isolated audit hello'}})
 for s in sockets:
  event=ws_recv(s);check('two clients receive persisted broadcast',event['type'],'message_created')
 with sqlite3.connect(db) as c:
  check('message persisted',c.execute('SELECT COUNT(*) FROM messages WHERE content=?',('isolated audit hello',)).fetchone()[0],1)
 for s in sockets:s.close()
 sockets=[]
 p.terminate();p.wait(timeout=5)
 p=subprocess.Popen([str(binary)],cwd=run,env=env,stdout=log,stderr=subprocess.STDOUT)
 for _ in range(100):
  try:
   if http('GET','/health').status_code==200:break
  except requests.ConnectionError:time.sleep(.1)
 check('session survives actual server restart',http('GET','/account',headers={'Cookie':cookie}).status_code,200)
 r=http('POST','/login',headers=origin,data=dict(username='audit_member',password='validation-password-123'))
 second=r.headers['Set-Cookie'].split(';')[0]
 csrf=hashlib.sha256(('logout:'+cookie.split('=',1)[1]).encode()).hexdigest()
 r=http('POST','/logout',headers=origin|{'Cookie':cookie},data={'csrf':csrf})
 check('logout revokes one session',r.status_code,303)
 check('logged out account',http('GET','/account',headers={'Cookie':cookie}).status_code,401)
 check('logged out room',http('GET','/room/'+room,headers={'Cookie':cookie}).status_code,401)
 code,s=upgrade(room,cookie);s.close();check('logged out WS',code,401)
 check('second session unaffected by logout',http('GET','/account',headers={'Cookie':second}).status_code,200)
 cookie=second
 with sqlite3.connect(db) as c:
  c.execute('UPDATE mbr_session SET expires_at=unixepoch()-1');c.commit()
 code,s=upgrade(room,cookie);s.close();check('expired session WS upgrade',code,401)
finally:
 for s in sockets:s.close()
 p.terminate()
 try:p.wait(timeout=5)
 except subprocess.TimeoutExpired:p.kill();p.wait()
 log.close()
 (root/'logs/chat-http-results.json').write_text(json.dumps({'source_sha':'9c2b7cf5a4c60a08254445911916c4ed9b02d7b3','checks':rows},ensure_ascii=False,indent=2))
print(json.dumps(rows,ensure_ascii=False,indent=2))
