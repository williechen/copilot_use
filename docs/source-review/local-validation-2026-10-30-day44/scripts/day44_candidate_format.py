from pathlib import Path
import subprocess,os,time,json,difflib
r=Path.cwd();d=r/'work/day44-candidate';out=r/'outputs/day44-2026-10-30/candidate-format';out.mkdir(exist_ok=True);env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='1',DATABASE_URL='sqlite::memory:');rows=[]
def run(a):
 t=time.time();p=subprocess.run(a,cwd=d,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300);log=f'{len(rows):02d}.log';(out/log).write_text(p.stdout);rows.append({'command':a,'cwd':str(d),'exit_code':p.returncode,'seconds':time.time()-t,'log':log});(out/'commands.json').write_text(json.dumps(rows,indent=2));print(p.returncode,' '.join(a),flush=True);return p.returncode
for app,ws in [('chat','chat_apps'),('analytics','analytics_apps')]:
 before={str(p.relative_to(d)):p.read_text() for p in (d/ws).rglob('*.rs') if 'target' not in p.parts}
 run(['cargo','fmt','--all','--manifest-path',ws+'/Cargo.toml'])
 patch=r/f'outputs/day44-2026-10-30/{app}-day44-format-r1.patch';body=''
 for f,old in before.items():
  new=(d/f).read_text()
  if old!=new:body+='diff --git a/'+f+' b/'+f+'\n'+''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/'+f,tofile='b/'+f))
 patch.write_text(body)
 if body:
  assert run(['git','apply','--reverse',str(patch)])==0;assert run(['git','apply','--check',str(patch)])==0;assert run(['git','apply',str(patch)])==0
 run(['cargo','fmt','--all','--manifest-path',ws+'/Cargo.toml','--','--check'])
run(['cargo','test','--offline','--locked','--manifest-path','chat_apps/Cargo.toml','-p','chat-server','--lib','--features','ssr'])
run(['cargo','test','--offline','--locked','--manifest-path','chat_apps/Cargo.toml','-p','chat-ui','--features','ssr'])
