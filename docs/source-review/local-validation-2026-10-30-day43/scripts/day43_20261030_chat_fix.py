from pathlib import Path
import subprocess,os,json,time
r=Path.cwd();out=r/'outputs/day43-2026-10-30/chat-fix';out.mkdir(exist_ok=True);d=r/'work/day43-20261030-chat-fix';rows=[];env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='2',DATABASE_URL='sqlite::memory:')
def run(a,cwd=d):
 t=time.time();p=subprocess.run(a,cwd=cwd,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300);name=f'{len(rows):02d}.log';(out/name).write_text(p.stdout);rows.append({'command':a,'cwd':str(cwd),'exit_code':p.returncode,'seconds':time.time()-t,'log':name});(out/'commands.json').write_text(json.dumps(rows,indent=2));print(p.returncode,' '.join(a),flush=True);return p.returncode
assert not d.exists();run(['git','clone','--no-hardlinks','/home/willie/Documents/rust_ai_ring',str(d)],r)
(d/'chat_apps/target').symlink_to(r/'work/latest/chat_apps/target',target_is_directory=True)
p=d/'chat_apps/chat_ui/src/signin_contract.rs';s=p.read_text();old='use leptos_router::{components::Router, location::RequestUrl};';assert old in s;p.write_text(s.replace(old,'use leptos_router::{\n    components::{Route, Router, Routes},\n    location::RequestUrl,\n    path,\n};'))
run(['cargo','fmt','--all','--manifest-path','chat_apps/Cargo.toml'])
patch=out/'chat-ssr-imports-d2612fc.patch';patch.write_bytes(subprocess.check_output(['git','diff','--','chat_apps/chat_ui/src/signin_contract.rs'],cwd=d));run(['git','apply','--reverse',str(patch)]);run(['git','apply','--check',str(patch)]);run(['git','apply',str(patch)])
run(['cargo','test','--offline','--locked','--manifest-path','chat_apps/Cargo.toml','-p','chat-ui','--features','ssr'])
run(['cargo','fmt','--all','--manifest-path','chat_apps/Cargo.toml','--','--check'])
