from pathlib import Path
import subprocess,os,json,time,signal,socket,urllib.request
b=Path.cwd();out=b/'outputs/gate2-2026-10-31';work=b/'work/gate2-20261031';run=work/'native-run';run.mkdir(exist_ok=True);(run/'.env').write_text('')
binary=b/'work/day44-recheck-latest/pet_apps/target/debug/pet-server';assert (out/'built-artifacts.json').exists()
db=work/'pet-gui.sqlite';assert not db.exists()
env=os.environ.copy()
for k in ['PET_DATABASE_URL','CHAT_DATABASE_URL','ANALYTICS_DATABASE_URL','DATABASE_URL','OPENAI_API_KEY','ANTHROPIC_API_KEY','GEMINI_API_KEY']:env.pop(k,None)
xlog=(out/'xvfb.log').open('w');x=subprocess.Popen(['Xvfb','-displayfd','1','-screen','0','1280x1024x24','-nolisten','tcp','-noreset'],stdout=subprocess.PIPE,stderr=xlog,text=True,start_new_session=True);display=x.stdout.readline().strip();assert display.isdigit(),display
env.update(DISPLAY=':'+display,GDK_BACKEND='x11',LIBGL_ALWAYS_SOFTWARE='1',WEBKIT_DISABLE_DMABUF_RENDERER='1',WEBKIT_DISABLE_COMPOSITING_MODE='1',TAURI_WEBVIEW_AUTOMATION='true',PET_ENABLE_ROAM='true',PET_DATABASE_URL='sqlite://'+str(db)+'?mode=rwc',LC_ALL='C.UTF-8')
def port():
 s=socket.socket();s.bind(('127.0.0.1',0));p=s.getsockname()[1];s.close();return p
p=port();np=port();log=(out/'tauri-driver.log').open('w');driver=subprocess.Popen(['tauri-driver','--port',str(p),'--native-port',str(np),'--native-driver','/usr/bin/WebKitWebDriver'],cwd=run,env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
config={'base_url':f'http://127.0.0.1:{p}','display':env['DISPLAY'],'driver_pid':driver.pid,'xvfb_pid':x.pid,'application':str(binary),'database':str(db),'cwd':str(run),'source_sha':'2c7ff8b861d32ab6d879bf80c9f2dce50de38097','environment_overrides':{k:env[k] for k in ['DISPLAY','GDK_BACKEND','LIBGL_ALWAYS_SOFTWARE','WEBKIT_DISABLE_DMABUF_RENDERER','WEBKIT_DISABLE_COMPOSITING_MODE','TAURI_WEBVIEW_AUTOMATION','PET_ENABLE_ROAM','PET_DATABASE_URL']}}
(out/'driver-connection.json').write_text(json.dumps(config,indent=2)+'\n')
try:
 for _ in range(60):
  try:
   urllib.request.urlopen(config['base_url']+'/status',timeout=1).close();break
  except Exception:
   if driver.poll() is not None:raise RuntimeError('driver exited')
   time.sleep(.2)
 else:raise RuntimeError('driver startup timeout')
 print('NATIVE DRIVER READY',config['base_url'],config['display'],flush=True)
 while driver.poll() is None:time.sleep(1)
finally:
 for proc in [driver,x]:
  if proc.poll() is None:
   try:os.killpg(proc.pid,signal.SIGTERM)
   except ProcessLookupError:pass
 print('driver stopped',flush=True)
