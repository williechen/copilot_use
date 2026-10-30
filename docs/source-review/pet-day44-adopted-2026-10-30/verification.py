from pathlib import Path
import subprocess,os,json,time,datetime,shutil,sqlite3,hashlib
b=Path.cwd();d=b/'work/day44-recheck-latest';src=Path('/home/willie/Documents/rust_ai_ring');out=b/'outputs/day44-update-followup';sha='2c7ff8b861d32ab6d879bf80c9f2dce50de38097';rows=[];env={**os.environ,'CARGO_INCREMENTAL':'0','CARGO_BUILD_JOBS':'1','DATABASE_URL':'sqlite::memory:'}
for k in ['ANALYTICS_DATABASE_URL','OPENAI_API_KEY','ANTHROPIC_API_KEY','GEMINI_API_KEY']:env.pop(k,None)
def run(a,scope,cwd=d):
 t=time.time();p=subprocess.run(a,cwd=cwd,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300);log=f'{len(rows):03}.log';(out/log).write_text(p.stdout);rows.append({'command':a,'cwd':str(cwd),'source_sha':sha,'scope':scope,'exit_code':p.returncode,'started_utc':datetime.datetime.fromtimestamp(t,datetime.timezone.utc).isoformat(),'seconds':time.time()-t,'log':log});(out/'commands.json').write_text(json.dumps(rows,indent=2)+'\n');print(log,p.returncode,scope,flush=True);return p.returncode
run(['git','fetch',str(src),'main'],'fetch exact adopted source');assert run(['git','checkout','--detach',sha],'checkout source in isolated clone')==0
for w in ['pet_apps','crates']:
 for p in (d/w).rglob('*.rs'):
  if 'target' not in p.parts:os.utime(p,None)
for a in [['rustc','-Vv'],['cargo','-V'],['node','--version']]:run(a,'tool version')
for w in ['crates','chat_apps','pet_apps','analytics_apps']:run(['cargo','metadata','--locked','--no-deps','--format-version','1','--manifest-path',w+'/Cargo.toml'],'locked metadata '+w)
for op in [['fmt','--all'],['check','--offline','--locked','--workspace'],['clippy','--offline','--locked','--workspace'],['test','--offline','--locked','--workspace','--no-fail-fast']]:run(['cargo',*op,'--manifest-path','pet_apps/Cargo.toml',*(['--','--check'] if op[0]=='fmt' else [])],'Pet adopted main')
run(['cargo','check','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-ui','--target','wasm32-unknown-unknown'],'Pet WASM')
run(['node','--test',*[str(p.relative_to(d)) for p in sorted((d/'pet_apps/pet_ui/tests').glob('*.mjs'))]],'Pet Node')
dbdir=b/'work/day44-adopted-db';dbdir.mkdir(exist_ok=True);old=dbdir/'old-migrations';old.mkdir(exist_ok=True)
for p in (src/'pet_apps/migrations').glob('*.sql'):
 if p.name[:8]<'20261030':shutil.copy2(p,old/p.name)
def upgrade(p,mode):return run(['cargo','run','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-server','--example','day44_upgrade','--',str(p),str(old),str(d/'pet_apps/migrations'),mode],'upgrade '+mode)
synthetic=dbdir/'synthetic.sqlite';assert not synthetic.exists();assert upgrade(synthetic,'seed')==0
source=src/'.local/pet.sqlite';copy=dbdir/'actual-readonly-copy.sqlite';assert not copy.exists()
def digest(p):
 c=sqlite3.connect(f'file:{p}?mode=ro',uri=True);h=hashlib.sha256();counts={}
 for(t,)in c.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT IN ('_sqlx_migrations','pet_activity','codemap') AND name NOT LIKE 'sqlite_%' ORDER BY name"):
  data=sorted(c.execute('SELECT * FROM "'+t+'"').fetchall(),key=repr);counts[t]=len(data);h.update(t.encode());h.update(repr(data).encode())
 dictionary=sorted(c.execute('SELECT * FROM codemap').fetchall(),key=repr);c.close();return h.hexdigest(),counts,dictionary
if source.exists():
 c=sqlite3.connect(f'file:{source}?mode=ro',uri=True);backup=sqlite3.connect(copy);c.backup(backup);backup.close();c.close();before,counts,dict_before=digest(copy);code=upgrade(copy,'actual');after,newcounts,dict_after=digest(copy)
 (out/'actual-copy-preservation.json').write_text(json.dumps({'source_sha':sha,'source_mode':'read-only SQLite online backup','original_database_written':False,'exit_code':code,'other_tables_sha256_before':before,'other_tables_sha256_after':after,'other_tables_unchanged':before==after and counts==newcounts,'existing_dictionary_rows_preserved':all(x in dict_after for x in dict_before),'dictionary_rows_added':len(dict_after)-len(dict_before),'table_counts':counts,'private_rows_published':False},indent=2)+'\n')
bad=dbdir/'checksum-copy.sqlite';shutil.copy2(synthetic,bad);upgrade(bad,'tamper')
inventory=[]
for f in subprocess.check_output(['git','diff','--name-only','4b1008b',sha],cwd=src,text=True).splitlines():
 p=src/f;inventory.append({'path':f,'blob_sha':subprocess.check_output(['git','rev-parse',sha+':'+f],cwd=src,text=True).strip(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
(out/'source-inventory.json').write_text(json.dumps({'source_sha':sha,'changed_files':inventory,'unaffected_projects':'Chat/Analytics/crates have no tree changes vs4b1008b; results retained under4b1008b only'},indent=2)+'\n');assert not subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=d);print('ADOPTION VERIFICATION DONE',flush=True)
