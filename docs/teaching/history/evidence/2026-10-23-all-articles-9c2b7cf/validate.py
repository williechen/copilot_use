import os, pathlib, subprocess, json, time
root=pathlib.Path(__file__).resolve().parent
env=os.environ.copy()
env.pop('DATABASE_URL',None)
env['CARGO_TARGET_DIR']='/home/willie/Documents/Codex/2026-10-23/referenced-chatgpt-conversation-this-is-an-2/work/target-full'
(root/'logs').mkdir(exist_ok=True)
checks=[
 ('chat-server-tests',['cargo','test','--locked','-p','server','--features','ssr']),
 ('chat-ui-ssr',['cargo','test','--locked','-p','ui','--features','ssr']),
 ('pet-tests',['cargo','test','--locked','-p','desktop-pet']),
 ('analytics-default',['cargo','test','--locked','-p','stats-server']),
 ('chat-build',['cargo','build','--locked','-p','server','--features','ssr']),
 ('chat-wasm',['cargo','check','--locked','-p','ui','--features','hydrate','--target','wasm32-unknown-unknown']),
 ('pet-wasm',['cargo','check','--locked','-p','desktop-pet-frontend','--target','wasm32-unknown-unknown']),
]
results=[]
for name,cmd in checks:
 start=time.time()
 with open(root/'logs'/f'{name}.log','w') as out:
  p=subprocess.run(cmd,cwd=root/'rust_ai_chat',env=env,stdout=out,stderr=subprocess.STDOUT)
 result=dict(name=name,command=cmd,exit=p.returncode,seconds=round(time.time()-start,1),source_sha=subprocess.check_output(['git','rev-parse','HEAD'],cwd=root/'rust_ai_chat',text=True).strip())
 results.append(result)
 (root/'logs/results.json').write_text(json.dumps(results,indent=2))
 print(json.dumps(result),flush=True)
