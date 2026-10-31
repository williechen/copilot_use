import ctypes as C,subprocess,re,json,time,urllib.request
from pathlib import Path
sid=json.load(open('work/day45-gui/tauri-session.json'))['value']['sessionId'];url='http://127.0.0.1:4444/session/'+sid
before=json.load(urllib.request.urlopen(url+'/window/handles'))['value'];tree=subprocess.check_output(['xwininfo','-display',':97','-root','-tree'],text=True);Path('work/day45-gui/x11-before-close.txt').write_text(tree);wid=int(re.search(r'(0x[0-9a-f]+) "Native Day45 A"',tree).group(1),16)
class Data(C.Union):_fields_=[('b',C.c_char*20),('s',C.c_short*10),('l',C.c_long*5)]
class Client(C.Structure):_fields_=[('type',C.c_int),('serial',C.c_ulong),('send_event',C.c_int),('display',C.c_void_p),('window',C.c_ulong),('message_type',C.c_ulong),('format',C.c_int),('data',Data)]
class Event(C.Union):_fields_=[('client',Client),('pad',C.c_long*24)]
x=C.CDLL('libX11.so.6');x.XOpenDisplay.restype=C.c_void_p;d=x.XOpenDisplay(b':97');x.XInternAtom.argtypes=[C.c_void_p,C.c_char_p,C.c_int];x.XInternAtom.restype=C.c_ulong
x.XSendEvent.argtypes=[C.c_void_p,C.c_ulong,C.c_int,C.c_long,C.POINTER(Event)];x.XFlush.argtypes=[C.c_void_p]
e=Event();e.client.type=33;e.client.display=d;e.client.window=wid;e.client.message_type=x.XInternAtom(d,b'WM_PROTOCOLS',0);e.client.format=32;e.client.data.l[0]=x.XInternAtom(d,b'WM_DELETE_WINDOW',0);e.client.data.l[1]=0
assert x.XSendEvent(d,wid,0,0,C.byref(e));x.XFlush(d)
for _ in range(50):
 after=json.load(urllib.request.urlopen(url+'/window/handles'))['value']
 if len(after)==1:break
 time.sleep(.1)
assert len(after)==1,after
import sqlite3
c=sqlite3.connect('/tmp/day45-gui-pet.sqlite');active=c.execute('SELECT count(*) FROM pet_activity WHERE is_activity=1').fetchone()[0];assert active==0,active
r={'method':'X11 WM_DELETE_WINDOW actual native protocol, not unauthorized Tauri close IPC','before_handles':before,'after_handles':after,'active_rows_after_close':active,'passed':True};Path('work/day45-gui/native-close.json').write_text(json.dumps(r,indent=2));print(r)
