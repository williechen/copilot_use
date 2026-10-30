from pathlib import Path
import subprocess,json,os,time,datetime,hashlib
base=Path.cwd();src=Path('/home/willie/Documents/rust_ai_ring');docs=Path('/home/willie/Documents/copilot_use');out=base/'outputs/day44-recheck/final-main';out.mkdir(exist_ok=True);dest=base/'work/day44-recheck-latest';rows=[]
sha=subprocess.check_output(['git','rev-parse','HEAD'],cwd=src,text=True).strip()
env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='1',DATABASE_URL='sqlite::memory:')
for k in ['ANALYTICS_DATABASE_URL','OPENAI_API_KEY','ANTHROPIC_API_KEY','GEMINI_API_KEY']:env.pop(k,None)
def run(args,cwd=dest,extra=None,timeout=600):
 e=env.copy();e.update(extra or {});start=time.time();log=f'{len(rows):03}.log'
 try:p=subprocess.run(args,cwd=cwd,env=e,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=timeout);code=p.returncode;content=p.stdout
 except subprocess.TimeoutExpired as x:code=124;content=x.stdout.decode(errors='replace') if isinstance(x.stdout,bytes) else x.stdout or ''
 (out/log).write_text(content);rows.append({'command':args,'cwd':str(cwd),'exit_code':code,'source_sha':sha,'started_utc':datetime.datetime.fromtimestamp(start,datetime.timezone.utc).isoformat(),'seconds':time.time()-start,'log':log,'env_overrides':{'CARGO_INCREMENTAL':'0','CARGO_BUILD_JOBS':'2',**(extra or {})}});(out/'commands.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n');print(log,code,' '.join(args),flush=True);return code
run(['git','fetch',str(src),'main']);run(['git','checkout','--detach',sha])
for w in ['crates','chat_apps','pet_apps','analytics_apps']:
 assert (dest/w/'target').is_symlink()
 for p in (dest/w).rglob('*.rs'):
  if 'target' not in p.parts:os.utime(p,None)
for args in [['rustc','-Vv'],['cargo','-V'],['node','--version'],['psql','--version'],['cargo','leptos','--version'],['cargo','tauri','--version'],['trunk','--version'],['rustup','target','list','--installed']]:run(args,base)
inventory=[]
files=subprocess.check_output(['git','ls-tree','-r','--name-only','HEAD'],cwd=src,text=True).splitlines()
for f in files:
 if f.endswith(('Cargo.toml','Cargo.lock')) or '/migrations/' in f or f.startswith('.github/workflows/') or f.endswith(('src/lib.rs','src/main.rs','tauri.conf.json')) or any(x in f for x in ['room_path','room_auth','password_login','dashboard_url','pet_store','activity.rs','pet_windows','roam.rs']):
  b=(src/f).read_bytes();inventory.append({'path':f,'git_blob':subprocess.check_output(['git','rev-parse','HEAD:'+f],cwd=src,text=True).strip(),'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b)})
(out/'source-inventory.json').write_text(json.dumps({'source_sha':sha,'files':inventory},indent=2)+'\n')
for w in ['crates','chat_apps','pet_apps','analytics_apps']:
 for op in [['metadata','--locked','--no-deps','--format-version','1'],['fmt','--all'],['check','--offline','--locked','--workspace'],['clippy','--offline','--locked','--workspace'],['test','--offline','--locked','--workspace','--no-fail-fast']]:
  run(['cargo',*op,'--manifest-path',w+'/Cargo.toml',*(['--','--check'] if op[0]=='fmt' else [])])
run(['cargo','test','--offline','--locked','--manifest-path','chat_apps/Cargo.toml','-p','chat-server','-p','chat-persistence','-p','chat-ui','--features','ssr'])
run(['cargo','check','--offline','--locked','--manifest-path','chat_apps/Cargo.toml','-p','chat-ui','--features','hydrate','--target','wasm32-unknown-unknown'])
run(['cargo','check','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-ui','--target','wasm32-unknown-unknown'])
run(['node','--test',*[str(p.relative_to(dest)) for p in sorted((dest/'pet_apps/pet_ui/tests').glob('*.mjs'))]])
ctl='/usr/lib/postgresql/18/bin/pg_ctl';pg=base/'work/pg'
if run([ctl,'-D',str(pg),'-l',str(out/'postgres.log'),'-o',"-k '' -p 55482 -h 127.0.0.1",'start'],base)==0:
 try:
  db='day44_final_20261030';run(['/usr/lib/postgresql/18/bin/createdb','-h','127.0.0.1','-p','55482',db],base);url='postgres://willie@127.0.0.1:55482/'+db
  run(['cargo','test','--offline','--locked','--no-fail-fast','--manifest-path','analytics_apps/Cargo.toml','-p','analytics-server','-p','analytics-persistence','--features','postgres-integration','--','--include-ignored'],extra={'DATABASE_URL':url,'ANALYTICS_DATABASE_URL':url,'ANALYTICS_CHAT_TOKEN':'local-test-chat','ANALYTICS_PET_TOKEN':'local-test-pet'})
 finally:run([ctl,'-D',str(pg),'stop'],base)
print('MAIN VERIFICATION DONE',flush=True)
