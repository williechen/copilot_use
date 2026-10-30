from pathlib import Path
import subprocess,json,time,os
r=Path.cwd();d=r/'work/day44-candidate';out=r/'outputs/day44-2026-10-30/final-apply';out.mkdir(exist_ok=True);rows=[];env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='1',DATABASE_URL='sqlite::memory:')
def run(a,cwd=d):
 t=time.time();p=subprocess.run(a,cwd=cwd,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300);log=f'{len(rows):02d}.log';(out/log).write_text(p.stdout);rows.append({'command':a,'cwd':str(cwd),'exit_code':p.returncode,'log':log,'seconds':time.time()-t,'source':'7ce39f0 + original Day44 Refactors/Chat+Analytics Greens + revised Pet Green and Analytics fixture correction'});(out/'commands.json').write_text(json.dumps(rows,indent=2));print(p.returncode,' '.join(a),flush=True);return p.returncode
pet=str(r/'outputs/day44-2026-10-30/pet-day44-activity-green-r3.patch');assert run(['git','apply','--check',pet])==0;assert run(['git','apply',pet])==0
ana=str(r/'outputs/day44-2026-10-30/analytics-logout-fixture-fix.patch');assert run(['git','apply','--check',ana])==0;assert run(['git','apply',ana])==0
for w in ['crates','chat_apps','pet_apps','analytics_apps']:
 (d/w/'target').symlink_to(r/'work/latest'/w/'target',target_is_directory=True)
 run(['cargo','metadata','--locked','--no-deps','--format-version','1','--manifest-path',w+'/Cargo.toml'])
 run(['cargo','fmt','--all','--manifest-path',w+'/Cargo.toml','--','--check'])
run(['cargo','test','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-server','--test','day44_activity_lifecycle'])
