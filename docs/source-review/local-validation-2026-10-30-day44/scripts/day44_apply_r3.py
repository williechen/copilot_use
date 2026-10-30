from pathlib import Path
import subprocess,json,os,time,shutil,hashlib
r=Path.cwd();out=r/'outputs/day44-2026-10-30/final-apply-r3';out.mkdir(exist_ok=True);candidate=r/'work/day44-candidate';implementation=r/'work/day44-pet-r3';applied=r/'work/day44-pet-apply-latest';rows=[];env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='1',DATABASE_URL='sqlite::memory:')
def run(a,cwd=candidate):
 t=time.time();p=subprocess.run(a,cwd=cwd,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300);log=f'{len(rows):02d}.log';(out/log).write_text(p.stdout);rows.append({'command':a,'cwd':str(cwd),'exit_code':p.returncode,'seconds':time.time()-t,'log':log});(out/'commands.json').write_text(json.dumps(rows,indent=2));print(p.returncode,' '.join(a),flush=True);return p.returncode
# Record original Refactor in index as the historical Green base; no source repository commit.
assert run(['git','add','pet_apps'])==0
files=subprocess.check_output(['git','diff','--name-only','--','pet_apps'],cwd=implementation,text=True).splitlines()
for f in files:
 p=candidate/f;p.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(implementation/f,p)
 if subprocess.run(['git','ls-files','--error-unmatch',f],cwd=candidate,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL).returncode:run(['git','add','-N',f])
variant=r/'outputs/day44-2026-10-30/pet-day44-activity-green-r3-after-refactor.patch';variant.write_bytes(subprocess.check_output(['git','diff','--','pet_apps'],cwd=candidate))
assert run(['git','apply','--reverse',str(variant)])==0
assert run(['git','apply','--check',str(variant)])==0
assert run(['git','apply',str(variant)])==0
ana=r/'outputs/day44-2026-10-30/analytics-logout-fixture-fix.patch';assert run(['git','apply','--check',str(ana)])==0;assert run(['git','apply',str(ana)])==0
for w in ['crates','chat_apps','pet_apps','analytics_apps']:
 (candidate/w/'target').symlink_to(r/'work/latest'/w/'target',target_is_directory=True)
 run(['cargo','metadata','--locked','--no-deps','--format-version','1','--manifest-path',w+'/Cargo.toml'])
 run(['cargo','fmt','--all','--manifest-path',w+'/Cargo.toml','--','--check'])
run(['cargo','test','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-server','--test','day44_activity_lifecycle'])
assert not applied.exists();assert run(['git','clone','--no-hardlinks','/home/willie/Documents/rust_ai_ring',str(applied)],r)==0
p=r/'outputs/day44-2026-10-30/pet-day44-activity-green-r3.patch';assert run(['git','apply','--check',str(p)],applied)==0;assert run(['git','apply',str(p)],applied)==0
matches=[]
for f in files:
 a=(applied/f).read_bytes();b=(implementation/f).read_bytes();assert a==b;matches.append({'path':f,'sha256':hashlib.sha256(a).hexdigest()})
(out/'applied-source-equality.json').write_text(json.dumps(matches,indent=2));print('DONE',flush=True)
