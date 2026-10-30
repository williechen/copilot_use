from pathlib import Path
import subprocess,os,json,time
r=Path.cwd();d=r/'work/day44-analytics-fixture-fix';out=r/'outputs/day44-2026-10-30/pg-fixture-retry';out.mkdir(exist_ok=True);rows=[];env=os.environ.copy();url='postgres://willie@127.0.0.1:55482/day44_20261030';env.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='1',DATABASE_URL=url,ANALYTICS_DATABASE_URL=url,ANALYTICS_CHAT_TOKEN='local-test-chat',ANALYTICS_PET_TOKEN='local-test-pet')
def run(a):
 t=time.time();p=subprocess.run(a,cwd=d,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300);log=str(len(rows))+'.log';(out/log).write_text(p.stdout);rows.append({'command':a,'exit_code':p.returncode,'seconds':time.time()-t,'log':log,'source_sha':'6b34c42f74fc1585b1b864fb7dd6969318270e0f'});(out/'commands.json').write_text(json.dumps(rows,indent=2));print(p.returncode,a,flush=True)
ctl='/usr/lib/postgresql/18/bin/pg_ctl';pg=r/'work/pg';run([ctl,'-D',str(pg),'-l',str(out/'postgres.log'),'-o',"-k '' -p 55482 -h 127.0.0.1",'start'])
try:
 run(['cargo','test','--offline','--locked','--manifest-path','analytics_apps/Cargo.toml','-p','analytics-server','--features','postgres-integration','--test','day44_logout_csrf'])
 run(['cargo','test','--offline','--locked','--no-fail-fast','--manifest-path','analytics_apps/Cargo.toml','-p','analytics-server','-p','analytics-persistence','--features','postgres-integration','--','--include-ignored'])
finally:run([ctl,'-D',str(pg),'stop'])
