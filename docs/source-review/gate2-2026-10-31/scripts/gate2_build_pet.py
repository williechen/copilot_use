from pathlib import Path
import subprocess,os,time,datetime,json,hashlib,shutil
b=Path.cwd();d=b/'work/day44-recheck-latest';out=b/'outputs/gate2-2026-10-31';work=b/'work/gate2-20261031';work.mkdir(exist_ok=True);preserve=work/'preserved-native';preserve.mkdir(exist_ok=True);rows=[];sha='2c7ff8b861d32ab6d879bf80c9f2dce50de38097'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=d,text=True).strip()==sha
assert not subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=d)
old=d/'pet_apps/target/debug/pet-server';backup=preserve/'pet-server-before-gate2';assert not backup.exists();shutil.copy2(old,backup);(out/'preserved-native.json').write_text(json.dumps({'original':str(old),'backup':str(backup),'bytes':backup.stat().st_size,'sha256':hashlib.sha256(backup.read_bytes()).hexdigest()},indent=2)+'\n')
env={**os.environ,'NO_COLOR':'true','CARGO_INCREMENTAL':'0','CARGO_BUILD_JOBS':'1'}
for k in ['PET_DATABASE_URL','CHAT_DATABASE_URL','ANALYTICS_DATABASE_URL','DATABASE_URL','OPENAI_API_KEY','ANTHROPIC_API_KEY','GEMINI_API_KEY']:env.pop(k,None)
def run(a,cwd):
 t=time.time();p=subprocess.run(a,cwd=cwd,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=900);name=f'build-{len(rows):02}.log';(out/name).write_text(p.stdout);rows.append({'command':a,'cwd':str(cwd),'source_sha':sha,'exit_code':p.returncode,'started_utc':datetime.datetime.fromtimestamp(t,datetime.timezone.utc).isoformat(),'seconds':time.time()-t,'log':name});(out/'build-commands.json').write_text(json.dumps(rows,indent=2)+'\n');print(name,p.returncode,flush=True);return p.returncode
assert run(['trunk','build','--release','--skip-version-check','--locked','true','--offline','false'],d/'pet_apps/pet_ui')==0
assert run(['cargo','build','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-server','--bin','pet-server','--features','tauri/custom-protocol'],d)==0
artifacts=[]
for p in [old,*(d/'pet_apps/pet_ui/dist').glob('*')]:
 if p.is_file():artifacts.append({'path':str(p),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
(out/'built-artifacts.json').write_text(json.dumps({'source_sha':sha,'native_profile':'debug with dependency feature tauri/custom-protocol (embedded assets)','frontend_profile':'release','files':artifacts,'source_worktree_unchanged':not bool(subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=d))},indent=2)+'\n');print('PET BUILD DONE',flush=True)
