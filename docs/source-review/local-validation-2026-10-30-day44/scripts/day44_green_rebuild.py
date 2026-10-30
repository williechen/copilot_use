from pathlib import Path
import subprocess,json,os,shutil,time
r=Path.cwd();d=r/'work/day44-pet-r3';out=r/'outputs/day44-2026-10-30/green-rebuild';out.mkdir(exist_ok=True);rows=[];env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='1',DATABASE_URL='sqlite::memory:')
# Force local source re-evaluation when returning from a different checkout using the same cache.
touched=[]
for ws in ['pet_apps','crates']:
 for p in (d/ws).rglob('*.rs'):
  if 'target' not in p.parts:p.touch();touched.append(str(p.relative_to(d)))
for p in (d/'pet_apps/migrations').glob('*.sql'):p.touch();touched.append(str(p.relative_to(d)))
(out/'local-rebuild-paths.json').write_text(json.dumps(touched,indent=2))
probe=d/'pet_apps/pet_server/tests/day44_creation_red.rs';assert not probe.exists();shutil.copy2(r/'outputs/day44-2026-10-30/day44_creation_red.rs',probe)
def run(a):
 t=time.time();p=subprocess.run(a,cwd=d,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300);log=f'{len(rows):02d}.log';(out/log).write_text(p.stdout);rows.append({'command':a,'cwd':str(d),'exit_code':p.returncode,'seconds':time.time()-t,'log':log,'scope':'forced local crate rebuild of Pet candidate, content unchanged'});(out/'commands.json').write_text(json.dumps(rows,indent=2));print(p.returncode,' '.join(a),flush=True)
try:run(['cargo','test','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-server','--test','day44_creation_red'])
finally:probe.unlink()
run(['cargo','test','--offline','--locked','--workspace','--manifest-path','pet_apps/Cargo.toml'])
run(['cargo','check','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-ui','--target','wasm32-unknown-unknown'])
