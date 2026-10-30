from pathlib import Path
import subprocess,os,json,time,datetime
b=Path.cwd();d=b/'work/day44-recheck-latest';out=b/'outputs/day44-recheck/latest-retry';out.mkdir(exist_ok=True);rows=[];env={**os.environ,'CARGO_INCREMENTAL':'0','CARGO_BUILD_JOBS':'1','DATABASE_URL':'sqlite::memory:'};env.pop('ANALYTICS_DATABASE_URL',None)
def run(a,scope,cwd=d,extra=None):
 t=time.time();p=subprocess.run(a,cwd=cwd,env={**env,**(extra or {})},text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=600);name=f'{len(rows):03}.log';(out/name).write_text(p.stdout);rows.append({'command':a,'cwd':str(cwd),'source_sha':'c726de161d5a43c5f7edd109e46673483832b820','scope':scope,'exit_code':p.returncode,'started_utc':datetime.datetime.fromtimestamp(t,datetime.timezone.utc).isoformat(),'seconds':time.time()-t,'log':name,'env_overrides':{'CARGO_BUILD_JOBS':'1',**(extra or {})}});(out/'commands.json').write_text(json.dumps(rows,indent=2)+'\n');print(name,p.returncode,scope,flush=True);return p.returncode
for w in ['analytics_apps','chat_apps']:
 for p in (d/w).rglob('*.rs'):
  if 'target' not in p.parts:os.utime(p,None)
run(['cargo','test','--offline','--locked','--workspace','--no-fail-fast','--manifest-path','analytics_apps/Cargo.toml'],'main Analytics retry after ENOSPC')
run(['cargo','test','--offline','--locked','--manifest-path','chat_apps/Cargo.toml','-p','chat-server','-p','chat-persistence','-p','chat-ui','--features','ssr'],'main Chat SSR retry after ENOSPC')
ctl='/usr/lib/postgresql/18/bin/pg_ctl';pg=b/'work/pg';url='postgres://willie@127.0.0.1:55482/day44_latest_retry_20261030'
if run([ctl,'-D',str(pg),'-l',str(out/'postgres.log'),'-o',"-k '' -p 55482 -h 127.0.0.1",'start'],'isolated PG start')==0:
 try:
  run(['/usr/lib/postgresql/18/bin/createdb','-h','127.0.0.1','-p','55482','day44_latest_retry_20261030'],'isolated DB create retry')
  run(['cargo','test','--offline','--locked','--no-fail-fast','--manifest-path','analytics_apps/Cargo.toml','-p','analytics-server','-p','analytics-persistence','--features','postgres-integration','--','--include-ignored'],'main PG full retry',extra={'DATABASE_URL':url,'ANALYTICS_DATABASE_URL':url,'ANALYTICS_CHAT_TOKEN':'local-test-chat','ANALYTICS_PET_TOKEN':'local-test-pet'})
 finally:run([ctl,'-D',str(pg),'stop'],'isolated PG stop')
c=b/'work/day44-c726-fixed'
for w in ['crates','pet_apps']:
 for p in (c/w).rglob('*.rs'):
  if 'target' not in p.parts:os.utime(p,None)
run(['cargo','test','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-server','--test','day44_activity_migration'],'new migration regression Red on unmodified main plus test only',cwd=c)
print('LATEST RETRIES DONE',flush=True)
