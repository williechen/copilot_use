from pathlib import Path
import subprocess,os,json,shutil,time
r=Path.cwd();d=r/'work/day43-20261030';out=r/'outputs/day43-2026-10-30/db';out.mkdir(exist_ok=True); data=r/'work/day43-db-20261030';data.mkdir(exist_ok=True); rows=[];env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='2')
def run(a):
 t=time.time();p=subprocess.run(a,cwd=d,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300);log=f'{len(rows):02d}.log';(out/log).write_text(p.stdout);rows.append({'command':a,'cwd':str(d),'exit_code':p.returncode,'log':log,'seconds':time.time()-t});(out/'commands.json').write_text(json.dumps(rows,indent=2));print(p.returncode,' '.join(a),flush=True);return p.returncode
probe=d/'chat_apps/chat_persistence/examples/upgrade_probe.rs';probe.parent.mkdir(exist_ok=True);shutil.copy2(r/'work/acceptance-product/chat_apps/chat_persistence/examples/upgrade_probe.rs',probe)
for app,w in [('chat','chat_apps'),('pet','pet_apps'),('analytics','analytics_apps')]:
 old=data/(app+'-old-migrations');old.mkdir(exist_ok=True)
 for p in sorted((d/w/'migrations').glob('*.sql'))[:2]:shutil.copy2(p,old/p.name)
 if app!='analytics':run(['cargo','run','--offline','--locked','--manifest-path','chat_apps/Cargo.toml','-p','chat-persistence','--example','upgrade_probe','--',app,str(old),str(d/w/'migrations'),str(data)])
for name in ['pg_upgrade_probe','pg_restore_probe']:
 p=d/f'analytics_apps/analytics_server/examples/{name}.rs';p.parent.mkdir(exist_ok=True);p.write_text((r/f'work/acceptance-product/analytics_apps/analytics_server/examples/{name}.rs').read_text().replace('acceptance_','recheck_20261030_'))
ctl='/usr/lib/postgresql/18/bin/pg_ctl';pg=r/'work/pg'
if run([ctl,'-D',str(pg),'-l',str(out/'postgres.log'),'-o',"-k '' -p 55482 -h 127.0.0.1",'start'])==0:
 try:
  backup=data/'synthetic-old.dump'
  if run(['cargo','run','--offline','--locked','--manifest-path','analytics_apps/Cargo.toml','-p','analytics-server','--example','pg_upgrade_probe','--',str(data/'analytics-old-migrations'),str(d/'analytics_apps/migrations'),str(backup)])==0:
   run(['cargo','run','--offline','--locked','--manifest-path','analytics_apps/Cargo.toml','-p','analytics-server','--example','pg_restore_probe','--',str(backup),str(d/'analytics_apps/migrations')])
 finally:run([ctl,'-D',str(pg),'stop'])
