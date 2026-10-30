from pathlib import Path
import subprocess,json,os,time,datetime,hashlib,sqlite3,shutil
b=Path.cwd();d=b/'work/day44-recheck-fixed';out=b/'outputs/day44-recheck/fixed';out.mkdir(exist_ok=True);rows=[];sha='e98fbbfc76e582977b966490f9427b5a1dbc89b7';env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='2',DATABASE_URL='sqlite::memory:')
for k in ['ANALYTICS_DATABASE_URL','OPENAI_API_KEY','ANTHROPIC_API_KEY','GEMINI_API_KEY']:env.pop(k,None)
def run(a,phase,cwd=d,extra=None):
 e=env.copy();e.update(extra or {});t=time.time();p=subprocess.run(a,cwd=cwd,env=e,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=600);name=f'{len(rows):03}.log';(out/name).write_text(p.stdout);rows.append({'command':a,'cwd':str(cwd),'exit_code':p.returncode,'base_sha':sha,'candidate_phase':phase,'started_utc':datetime.datetime.fromtimestamp(t,datetime.timezone.utc).isoformat(),'seconds':time.time()-t,'env_overrides':extra or {},'log':name});(out/'commands.json').write_text(json.dumps(rows,indent=2)+'\n');print(name,p.returncode,phase,flush=True);return p.returncode
p=d/'pet_apps/migrations/20261030060000_activity_lifecycle.sql';s=p.read_text();s=s.replace('-- For legacy profiles',"-- Normalize legacy display labels without deleting IDs or unrelated activities.\nUPDATE pet_activity SET activity_type=CASE activity_type WHEN '散步' THEN 'walk' WHEN '坐下' THEN 'sit' END\nWHERE activity_type IN ('散步','坐下');\n-- For legacy profiles")
s=s[:s.index('INSERT INTO codemap')]+'''-- Keep existing dictionary entries; an unrelated primary-key collision fails safely.
INSERT INTO codemap(id,category,code,value,description,created_at,updated_at)
SELECT 'initial-walk','activity','walk','散步','Initial walk activity',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP
WHERE NOT EXISTS(SELECT 1 FROM codemap WHERE category='activity' AND code='walk');
INSERT INTO codemap(id,category,code,value,description,created_at,updated_at)
SELECT 'initial-sit','activity','sit','坐下','Initial sit activity',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP
WHERE NOT EXISTS(SELECT 1 FROM codemap WHERE category='activity' AND code='sit');
''';p.write_text(s)
(out/'migration-only.patch').write_bytes(subprocess.check_output(['git','diff','--','pet_apps/migrations'],cwd=d))
run(['cargo','test','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-server','--test','day44_activity_lifecycle'],'migration corrected; original lifecycle fixtures unchanged')
# Correct stale code literals while retaining all assertions and injected failure cases.
p=d/'pet_apps/pet_server/tests/day44_activity_lifecycle.rs';p.write_text(p.read_text().replace("'散步'","'walk'").replace("'坐下'","'sit'"))
p=d/'pet_apps/pet_server/examples/day44_upgrade.rs';s=p.read_text();lines=s.splitlines()
for i,l in enumerate(lines):
 if 'let missing:' in l or 'let conflicts:' in l:lines[i]=l.replace("'散步'","'walk'").replace("'坐下'","'sit'")
p.write_text('\n'.join(lines)+'\n')
run(['cargo','fmt','--all','--manifest-path','pet_apps/Cargo.toml'],'complete Pet fix formatting')
for op in [['metadata','--locked','--no-deps','--format-version','1'],['fmt','--all'],['check','--offline','--locked','--workspace'],['clippy','--offline','--locked','--workspace'],['test','--offline','--locked','--workspace','--no-fail-fast']]:
 run(['cargo',*op,'--manifest-path','pet_apps/Cargo.toml',*(['--','--check'] if op[0]=='fmt' else [])],'complete Pet fix')
run(['cargo','check','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-ui','--target','wasm32-unknown-unknown'],'complete Pet fix WASM')
run(['node','--test',*[str(p.relative_to(d)) for p in sorted((d/'pet_apps/pet_ui/tests').glob('*.mjs'))]],'complete Pet fix Node')
run(['trunk','build','--skip-version-check','--locked','true','--offline','true'],'complete Pet fix frontend release',cwd=d/'pet_apps/pet_ui',extra={'NO_COLOR':'true'})
assets=[]
for p in (d/'pet_apps/pet_ui/dist').glob('*'):
 if p.is_file():assets.append({'file':p.name,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'bytes':p.stat().st_size})
(out/'frontend-assets.json').write_text(json.dumps(assets,indent=2)+'\n')
# Run real Migrator against an old-schema fixture and a read-only online backup copy.
dbdir=b/'work/day44-recheck-db';dbdir.mkdir(exist_ok=True);old=dbdir/'old-migrations';old.mkdir(exist_ok=True)
for p in (d/'pet_apps/migrations').glob('*.sql'):
 if not p.name.startswith('20261030060000'):shutil.copy2(p,old/p.name)
def upgrade(p,mode):return run(['cargo','run','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-server','--example','day44_upgrade','--',str(p),str(old),str(d/'pet_apps/migrations'),mode],'complete Pet fix upgrade '+mode)
synthetic=dbdir/'synthetic.sqlite';assert not synthetic.exists();upgrade(synthetic,'seed')
source=Path('/home/willie/Documents/rust_ai_ring/.local/pet.sqlite');copy=dbdir/'actual-readonly-backup.sqlite';assert not copy.exists()
def digest(p):
 c=sqlite3.connect(f'file:{p}?mode=ro',uri=True);h=hashlib.sha256();counts={}
 for(t,)in c.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT IN ('_sqlx_migrations','pet_activity','codemap') AND name NOT LIKE 'sqlite_%' ORDER BY name"):
  records=sorted(c.execute('SELECT * FROM "'+t+'"').fetchall(),key=repr);counts[t]=len(records);h.update(t.encode());h.update(repr(records).encode())
 dictionary=sorted(c.execute('SELECT * FROM codemap').fetchall(),key=repr);c.close();return h.hexdigest(),counts,dictionary
if source.exists():
 c=sqlite3.connect(f'file:{source}?mode=ro',uri=True);backup=sqlite3.connect(copy);c.backup(backup);backup.close();c.close();before,counts,dict_before=digest(copy);code=upgrade(copy,'actual');after,newcounts,dict_after=digest(copy)
 (out/'actual-copy-preservation.json').write_text(json.dumps({'source_open_mode':'SQLite mode=ro online backup','original_database_written':False,'exit_code':code,'nonactivity_nondictionary_data_sha256_before':before,'nonactivity_nondictionary_data_sha256_after':after,'other_tables_unchanged':before==after and counts==newcounts,'existing_codemap_rows_preserved':all(x in dict_after for x in dict_before),'codemap_rows_added':len(dict_after)-len(dict_before),'table_counts':counts,'raw_data_published':False},indent=2)+'\n')
bad=dbdir/'checksum-copy.sqlite';shutil.copy2(synthetic,bad);upgrade(bad,'tamper')
subprocess.run(['git','add','-N','pet_apps/pet_server/tests/day44_activity_migration.rs'],cwd=d,check=True)
patch=subprocess.check_output(['git','diff','--','pet_apps'],cwd=d);(out/'pet-day44-sqlite-code-fix-r4.patch').write_bytes(patch)
# The independent Analytics fixture correction remains necessary on new main.
patchpath=Path('/home/willie/Documents/copilot_use/docs/teaching/daily/day-44/analytics-logout-fixture-fix.patch')
assert run(['git','apply','--check',str(patchpath)],'Analytics fixture real check')==0
assert run(['git','apply',str(patchpath)],'Analytics fixture real apply')==0
ctl='/usr/lib/postgresql/18/bin/pg_ctl';pg=b/'work/pg';url='postgres://willie@127.0.0.1:55482/day44_recheck_20261030'
if run([ctl,'-D',str(pg),'-l',str(out/'postgres.log'),'-o',"-k '' -p 55482 -h 127.0.0.1",'start'],'isolated PostgreSQL start')==0:
 try:
  run(['cargo','test','--offline','--locked','--no-fail-fast','--manifest-path','analytics_apps/Cargo.toml','-p','analytics-server','-p','analytics-persistence','--features','postgres-integration','--','--include-ignored'],'Analytics fixture fix full PG',extra={'DATABASE_URL':url,'ANALYTICS_DATABASE_URL':url,'ANALYTICS_CHAT_TOKEN':'local-test-chat','ANALYTICS_PET_TOKEN':'local-test-pet'})
 finally:run([ctl,'-D',str(pg),'stop'],'isolated PostgreSQL stop')
print('FIX VERIFICATION DONE',flush=True)
