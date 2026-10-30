from pathlib import Path
import subprocess,os,shutil,json,time
r=Path.cwd();out=r/'outputs/day44-2026-10-30/red-green';out.mkdir(exist_ok=True);env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='1',DATABASE_URL='sqlite::memory:');rows=[]
for label,d in [('red',r/'work/day44-latest'),('green',r/'work/day44-pet-r3')]:
 target=d/'pet_apps/pet_server/tests/day44_creation_red.rs';assert not target.exists();shutil.copy2(r/'outputs/day44-2026-10-30/day44_creation_red.rs',target)
 a=['cargo','test','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-server','--test','day44_creation_red'];start=time.time()
 try:p=subprocess.run(a,cwd=d,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300)
 finally:target.unlink()
 (out/(label+'.log')).write_text(p.stdout);rows.append({'phase':label,'command':a,'cwd':str(d),'exit_code':p.returncode,'seconds':time.time()-start,'log':label+'.log','source':'6b34c42 main' if label=='red' else '6b34c42 + Pet activity Green r3'});(out/'commands.json').write_text(json.dumps(rows,indent=2));print(label,p.returncode,flush=True)
 assert p.returncode==(101 if label=='red' else 0)
