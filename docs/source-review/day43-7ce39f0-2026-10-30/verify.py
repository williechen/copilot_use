from pathlib import Path
import subprocess,os,json,time,datetime,urllib.request
r=Path.cwd();out=r/'outputs/day43-2026-10-30-7ce39f0';d=r/'work/day43-7ce39f0';repo=Path('/home/willie/Documents/rust_ai_ring');rows=[];env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='2',DATABASE_URL='sqlite::memory:')
def run(a,cwd=d):
 t=time.time();p=subprocess.run(a,cwd=cwd,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300);log=f'{len(rows):02d}.log';(out/log).write_text(p.stdout);rows.append({'command':a,'cwd':str(cwd),'source_sha':'7ce39f0166a2120539e5908b33c88322a930c35a','started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit_code':p.returncode,'seconds':time.time()-t,'log':log});(out/'commands.json').write_text(json.dumps(rows,indent=2));print(p.returncode,' '.join(a),flush=True)
run(['git','clone','--no-hardlinks',str(repo),str(d)],r);(d/'chat_apps/target').symlink_to(r/'work/latest/chat_apps/target',target_is_directory=True)
run(['git','diff','d2612fc..HEAD']);run(['git','apply','--reverse','--check',str(Path('/home/willie/Documents/copilot_use/docs/teaching/daily/day-43/chat-ssr-imports-d2612fc.patch'))])
for a in [['cargo','metadata','--locked','--no-deps','--format-version','1','--manifest-path','chat_apps/Cargo.toml'],['cargo','fmt','--all','--manifest-path','chat_apps/Cargo.toml','--','--check'],['cargo','test','--offline','--locked','--manifest-path','chat_apps/Cargo.toml','-p','chat-ui','--features','ssr'],['cargo','test','--offline','--locked','--manifest-path','chat_apps/Cargo.toml','-p','chat-server','-p','chat-persistence','-p','chat-ui','--features','ssr']]:run(a)
url='https://api.github.com/repos/williechen/rust_ai_ring/actions/runs?head_sha=7ce39f0166a2120539e5908b33c88322a930c35a&per_page=100'
with urllib.request.urlopen(urllib.request.Request(url,headers={'User-Agent':'local-verification'}),timeout=25) as response:data=json.load(response)
data['observed_at_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();(out/'ci.json').write_text(json.dumps(data,indent=2));print([(x['name'],x['status'],x['conclusion']) for x in data['workflow_runs']],flush=True)
