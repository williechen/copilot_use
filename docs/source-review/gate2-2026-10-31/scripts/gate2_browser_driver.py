from pathlib import Path
import os,json,subprocess,socket,time,signal,urllib.request
b=Path.cwd();out=b/'outputs/gate2-2026-10-31';pet=json.loads((out/'driver-connection.json').read_text());s=socket.socket();s.bind(('127.0.0.1',0));port=s.getsockname()[1];s.close();env={**os.environ,'DISPLAY':pet['display'],'GDK_BACKEND':'x11','LIBGL_ALWAYS_SOFTWARE':'1','WEBKIT_DISABLE_DMABUF_RENDERER':'1','WEBKIT_DISABLE_COMPOSITING_MODE':'1'}
log=(out/'browser-driver.log').open('w');driver=subprocess.Popen(['WebKitWebDriver','--port='+str(port)],env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True);base='http://127.0.0.1:'+str(port)
try:
 for _ in range(50):
  try:urllib.request.urlopen(base+'/status',timeout=1).close();break
  except Exception:time.sleep(.1)
 req=urllib.request.Request(base+'/session',data=json.dumps({'capabilities':{'alwaysMatch':{'browserName':'MiniBrowser','webkitgtk:browserOptions':{'binary':'/usr/lib/x86_64-linux-gnu/webkit2gtk-4.1/MiniBrowser','args':['--automation'],'useOverlayScrollbars':False}}}}).encode(),headers={'Content-Type':'application/json'},method='POST');session=json.load(urllib.request.urlopen(req,timeout=30));(out/'browser-session.json').write_text(json.dumps(session,indent=2)+'\n');(out/'browser-connection.json').write_text(json.dumps({'base_url':base,'pid':driver.pid,'display':pet['display'],'method':'real WebKitGTK MiniBrowser via W3C WebDriver; private Xvfb'},indent=2)+'\n');print('BROWSER READY',base,flush=True)
 while driver.poll() is None:time.sleep(1)
finally:
 if driver.poll() is None:os.killpg(driver.pid,signal.SIGTERM)
