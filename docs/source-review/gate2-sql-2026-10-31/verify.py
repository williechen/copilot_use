from pathlib import Path
import subprocess,os,json,time,datetime,re,hashlib
b=Path.cwd();d=b/'work/day44-recheck-latest';out=b/'outputs/gate2-sql-2026-10-31';out.mkdir(exist_ok=True);sha='64eff527283fd05657841297879aefeaadd31f64';old='2c7ff8b861d32ab6d879bf80c9f2dce50de38097';rows=[]
env={**os.environ,'CARGO_INCREMENTAL':'0','CARGO_BUILD_JOBS':'1','DATABASE_URL':'sqlite::memory:'}
for k in ['CHAT_DATABASE_URL','PET_DATABASE_URL','ANALYTICS_DATABASE_URL','OPENAI_API_KEY','ANTHROPIC_API_KEY','GEMINI_API_KEY']:env.pop(k,None)
def run(a,scope):
 t=time.time();p=subprocess.run(a,cwd=d,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=600);log=f'{len(rows):03}.log';(out/log).write_text(p.stdout);rows.append({'command':a,'cwd':str(d),'source_sha':sha,'scope':scope,'exit_code':p.returncode,'started_utc':datetime.datetime.fromtimestamp(t,datetime.timezone.utc).isoformat(),'seconds':time.time()-t,'log':log});(out/'commands.json').write_text(json.dumps(rows,indent=2)+'\n');print(log,p.returncode,scope,flush=True);return p.returncode
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=d,text=True).strip()==sha
assert not subprocess.check_output(['git','status','--porcelain'],cwd=d)
changed=subprocess.check_output(['git','diff','--name-only',old,sha],cwd=d,text=True).splitlines()
inventory=[]
for f in changed:
 before=subprocess.check_output(['git','show',old+':'+f],cwd=d,text=True);after=(d/f).read_text()
 # Compare SQL token sequences, preserving quoted SQL data exactly; not a runtime acceptance substitute.
 pattern=r'sqlx::query(?:_scalar|_as)?\(\s*(r#".*?"#|"(?:[^"\\]|\\.)*")'
 def queries(s):
  result=[]
  for m in re.finditer(pattern,s,re.S):
   x=m.group(1);x=x[3:-2] if x.startswith('r#') else json.loads(x,strict=False)
   result.append(re.findall(r"'(?:''|[^'])*'|[A-Za-z_][A-Za-z_0-9]*|\d+|[^\s]",x))
  return result
 inventory.append({'path':f,'blob_sha':subprocess.check_output(['git','rev-parse',sha+':'+f],cwd=d,text=True).strip(),'sha256':hashlib.sha256((d/f).read_bytes()).hexdigest(),'sql_query_calls_before':len(queries(before)),'sql_query_calls_after':len(queries(after)),'sql_token_sequences_equal':queries(before)==queries(after)})
(out/'source-review.json').write_text(json.dumps({'source_sha':sha,'previous_source_sha':old,'changed_files':inventory,'unchanged_trees':{w:subprocess.check_output(['git','rev-parse',sha+':'+w],cwd=d,text=True).strip()==subprocess.check_output(['git','rev-parse',old+':'+w],cwd=d,text=True).strip() for w in ['chat_apps','analytics_apps','crates','.github','pet_apps/migrations','pet_apps/pet_ui']},'sql_static_comparison_is_not_runtime_pass':True},indent=2)+'\n')
for a in [['rustc','-Vv'],['cargo','-V'],['node','--version']]:run(a,'tool version')
for w in ['crates','chat_apps','pet_apps','analytics_apps']:run(['cargo','metadata','--locked','--no-deps','--format-version','1','--manifest-path',w+'/Cargo.toml'],'locked metadata '+w)
for op in [['fmt','--all'],['check','--offline','--locked','--workspace'],['clippy','--offline','--locked','--workspace'],['test','--offline','--locked','--workspace','--no-fail-fast']]:run(['cargo',*op,'--manifest-path','pet_apps/Cargo.toml',*(['--','--check'] if op[0]=='fmt' else [])],'Pet '+op[0])
run(['cargo','check','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-ui','--target','wasm32-unknown-unknown'],'Pet WASM')
run(['node','--test',*[str(p.relative_to(d)) for p in sorted((d/'pet_apps/pet_ui/tests').glob('*.mjs'))]],'Pet Node')
assert not subprocess.check_output(['git','status','--porcelain'],cwd=d)
print('DONE',flush=True)
