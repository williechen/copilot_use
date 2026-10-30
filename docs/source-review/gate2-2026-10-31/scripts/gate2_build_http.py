from pathlib import Path
import os,subprocess,json,time,datetime,hashlib,shutil
b=Path.cwd();d=b/'work/day44-recheck-latest';out=b/'outputs/gate2-2026-10-31';work=b/'work/gate2-20261031';preserve=work/'preserved-native';env={**os.environ,'CARGO_INCREMENTAL':'0','CARGO_BUILD_JOBS':'1','NO_COLOR':'true'};rows=[]
for k in ['CHAT_DATABASE_URL','PET_DATABASE_URL','ANALYTICS_DATABASE_URL','DATABASE_URL','OPENAI_API_KEY','ANTHROPIC_API_KEY','GEMINI_API_KEY']:env.pop(k,None)
for w,bin in [('chat_apps','chat-server'),('analytics_apps','analytics-server')]:
 p=d/w/'target/debug'/bin;q=preserve/(bin+'-before-gate2')
 if p.exists() and not q.exists():shutil.copy2(p,q)
for a,cwd in [(['cargo','leptos','build','--cargo-locked'],d/'chat_apps'),(['cargo','build','--offline','--locked','--manifest-path','analytics_apps/Cargo.toml','-p','analytics-server','--bin','analytics-server'],d)]:
 t=time.time();p=subprocess.run(a,cwd=cwd,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=900);name=f'http-build-{len(rows):02}.log';(out/name).write_text(p.stdout);rows.append({'command':a,'cwd':str(cwd),'source_sha':'2c7ff8b861d32ab6d879bf80c9f2dce50de38097','exit_code':p.returncode,'started_utc':datetime.datetime.fromtimestamp(t,datetime.timezone.utc).isoformat(),'seconds':time.time()-t,'log':name});(out/'http-build-commands.json').write_text(json.dumps(rows,indent=2)+'\n');print(name,p.returncode,flush=True)
 if p.returncode:break
artifacts=[]
for p in [d/'chat_apps/target/debug/chat-server',d/'analytics_apps/target/debug/analytics-server',*(d/'chat_apps/target/site/pkg').glob('*')]:
 if p.is_file():artifacts.append({'path':str(p),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
(out/'http-built-artifacts.json').write_text(json.dumps(artifacts,indent=2)+'\n');print('HTTP BUILD DONE',flush=True)
