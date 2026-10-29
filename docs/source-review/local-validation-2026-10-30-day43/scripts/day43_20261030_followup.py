from pathlib import Path
import os,subprocess,json,time,datetime,shutil
r=Path.cwd(); out=r/'outputs/day43-2026-10-30'; d=r/'work/day43-20261030'; rows=[]; env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='2',DATABASE_URL='sqlite::memory:')
def run(a,extra=None):
 en=env.copy();en.update(extra or {});t=time.time();p=subprocess.run(a,cwd=d,env=en,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300); name=f'followup-{len(rows):02d}.log';(out/name).write_text(p.stdout);rows.append({'command':a,'cwd':str(d),'exit_code':p.returncode,'log':name,'seconds':time.time()-t,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'env_overrides':{'CARGO_INCREMENTAL':'0','CARGO_BUILD_JOBS':'2',**(extra or {})}});(out/'followup-commands.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2));print(p.returncode,' '.join(a),flush=True);return p.returncode
for op in ['check','clippy','test']:run(['cargo',op,'--offline','--locked','--workspace','--manifest-path','crates/Cargo.toml'])
