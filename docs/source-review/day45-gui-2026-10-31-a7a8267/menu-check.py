from pathlib import Path
exec(Path('work/day45-gui/native.py').read_text().split('\ntry:\n')[0])
import ctypes as C,os
x=C.CDLL('libX11.so.6');x.XOpenDisplay.restype=C.c_void_p;d=x.XOpenDisplay(b':97');x.XKeysymToKeycode.argtypes=[C.c_void_p,C.c_ulong];t=C.CDLL('libXtst.so.6');t.XTestFakeKeyEvent.argtypes=[C.c_void_p,C.c_uint,C.c_int,C.c_ulong];x.XFlush.argtypes=[C.c_void_p]
def key(sym):
 k=x.XKeysymToKeycode(d,sym);t.XTestFakeKeyEvent(d,k,1,0);t.XTestFakeKeyEvent(d,k,0,0);x.XFlush(d);time.sleep(.1)
key(0xff1b);key(0xff1b)
b=dict(db('SELECT name,id FROM pet_profile'))['Native Day45 B']
for h in req('GET','/window/handles'):
 req('POST','/window',{'handle':h})
 if js('return new URLSearchParams(location.search).get("pet_id")')==b:break
req('POST','/actions',{'actions':[{'type':'pointer','id':'context','parameters':{'pointerType':'mouse'},'actions':[{'type':'pointerMove','duration':0,'x':100,'y':100},{'type':'pointerDown','button':2},{'type':'pointerUp','button':2}]}]});time.sleep(.3)
for _ in range(6):key(0xff54)
os.environ['DISPLAY']=':97';import gi;gi.require_version('Gdk','3.0');from gi.repository import Gdk
w=Gdk.get_default_root_window();p=Gdk.pixbuf_get_from_window(w,0,0,1280,1024);p.savev('work/day45-gui/pet-menu-keyboard.png','png',[],[])
key(0xff0d);time.sleep(.3)
s=asyncjs('const done=arguments[arguments.length-1];window.getPetState().then(done);');print(s);assert s['state']=='sleeping',s
Path('work/day45-gui/native-menu.json').write_text(json.dumps({'method':'native right-click + GTK menu keyboard selection; no IPC mocks','snapshot_after_sleep':s,'passed':True},indent=2))
