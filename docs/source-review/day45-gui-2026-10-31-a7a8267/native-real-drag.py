from pathlib import Path
exec(Path('work/day45-gui/native.py').read_text().split('\ntry:\n')[0])
import ctypes as C,re,subprocess
bid=dict(db('SELECT name,id FROM pet_profile'))['Native Day45 B']
for h in req('GET','/window/handles'):
 req('POST','/window',{'handle':h})
 if js('return new URLSearchParams(location.search).get("pet_id")')==bid:break
before=req('GET','/window/rect');tree=subprocess.check_output(['xwininfo','-display',':97','-root','-tree'],text=True);wid=int(re.search(r'(0x[0-9a-f]+) "Native Day45 B"',tree).group(1),16)
x=C.CDLL('libX11.so.6');x.XOpenDisplay.restype=C.c_void_p;d=x.XOpenDisplay(b':97');x.XRaiseWindow.argtypes=[C.c_void_p,C.c_ulong];x.XRaiseWindow(d,wid);x.XFlush.argtypes=[C.c_void_p]
t=C.CDLL('libXtst.so.6');t.XTestFakeMotionEvent.argtypes=[C.c_void_p,C.c_int,C.c_int,C.c_int,C.c_ulong];t.XTestFakeButtonEvent.argtypes=[C.c_void_p,C.c_uint,C.c_int,C.c_ulong]
t.XTestFakeMotionEvent(d,-1,before['x']+80,before['y']+8,0);t.XTestFakeButtonEvent(d,1,1,0);x.XFlush(d);time.sleep(.4)
for i in range(1,11):t.XTestFakeMotionEvent(d,-1,before['x']+80+i*8,before['y']+8+i*5,0);x.XFlush(d);time.sleep(.06)
t.XTestFakeButtonEvent(d,1,0,0);x.XFlush(d);time.sleep(.5);after=req('GET','/window/rect');print(before,after);assert (before['x'],before['y'])!=(after['x'],after['y'])
Path('work/day45-gui/native-drag.json').write_text(json.dumps({'method':'XTest actual OS pointer events on real product DragHandle under xfwm4','before':before,'after':after,'passed':True},indent=2))
