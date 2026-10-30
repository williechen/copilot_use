from pathlib import Path
import subprocess,json,os,time,datetime,hashlib,shutil,sqlite3
b=Path.cwd();d=b/'work/day44-c726-fixed';out=b/'outputs/day44-recheck/latest-fix';rows=[];sha='c726de161d5a43c5f7edd109e46673483832b820';env={**os.environ,'CARGO_INCREMENTAL':'0','CARGO_BUILD_JOBS':'1','DATABASE_URL':'sqlite::memory:'};env.pop('ANALYTICS_DATABASE_URL',None)
def run(a,scope,cwd=d,extra=None):
 t=time.time();p=subprocess.run(a,cwd=cwd,env={**env,**(extra or {})},text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=600);name=f'{len(rows):03}.log';(out/name).write_text(p.stdout);rows.append({'command':a,'cwd':str(cwd),'base_sha':sha,'scope':scope,'exit_code':p.returncode,'started_utc':datetime.datetime.fromtimestamp(t,datetime.timezone.utc).isoformat(),'seconds':time.time()-t,'log':name,'env_overrides':{'CARGO_BUILD_JOBS':'1',**(extra or {})}});(out/'commands.json').write_text(json.dumps(rows,indent=2)+'\n');print(name,p.returncode,scope,flush=True);return p.returncode
retry=json.loads((b/'outputs/day44-recheck/latest-retry/commands.json').read_text());assert len(retry)==7 and retry[-1]['exit_code']==101 and 'new migration regression Red' in retry[-1]['scope'];assert 'test result: FAILED. 1 passed; 1 failed' in (b/'outputs/day44-recheck/latest-retry'/retry[-1]['log']).read_text()
# Preserve latest main's fixture corrections; only migrate legacy aliases/dictionary safely and correct upgrade verification literals.
for f in ['pet_apps/pet_server/examples/day44_upgrade.rs']:
 shutil.copy2(b/'work/day44-recheck-fixed'/f,d/f)
# Keep the already valid/deployable migration 4 byte-for-byte; use a forward data migration.
(d/'pet_apps/migrations/20261030140000_normalize_activity_codes.sql').write_text("""-- Preserve all activity IDs, owner links, dictionary rows and deployed migration checksums.
UPDATE pet_activity SET activity_type=CASE activity_type WHEN '散步' THEN 'walk' WHEN '坐下' THEN 'sit' END
WHERE activity_type IN ('散步','坐下');
UPDATE pet_activity AS w SET is_activity=0
WHERE w.activity_type='walk' AND w.is_activity=1
AND EXISTS(SELECT 1 FROM pet_activity s WHERE s.pet_id=w.pet_id AND s.owner_id=w.owner_id AND s.activity_type='sit' AND s.is_activity=1);
""")
run(['cargo','fmt','--all','--manifest-path','pet_apps/Cargo.toml'],'format only candidate changes')
subprocess.run(['git','add','-N','pet_apps/pet_server/tests/day44_activity_migration.rs','pet_apps/migrations/20261030140000_normalize_activity_codes.sql'],cwd=d,check=True)
patch=subprocess.check_output(['git','diff','--','pet_apps'],cwd=d);p=out/'pet-day44-legacy-code-upgrade-r4.patch';p.write_bytes(patch)
# Validate the delivered exact patch by real reverse/check/apply on its own latest-source baseline.
assert run(['git','apply','--reverse','--check',str(p)],'delivered Pet patch reverse check')==0
assert run(['git','apply','--reverse',str(p)],'restore exact main before apply')==0
assert run(['git','apply','--check',str(p)],'delivered Pet patch real apply check')==0
assert run(['git','apply',str(p)],'delivered Pet patch real apply')==0
for w in ['crates','chat_apps','pet_apps','analytics_apps']:
 run(['cargo','metadata','--locked','--no-deps','--format-version','1','--manifest-path',w+'/Cargo.toml'],'candidate metadata '+w)
for op in [['fmt','--all'],['check','--offline','--locked','--workspace'],['clippy','--offline','--locked','--workspace'],['test','--offline','--locked','--workspace','--no-fail-fast']]:run(['cargo',*op,'--manifest-path','pet_apps/Cargo.toml',*(['--','--check'] if op[0]=='fmt' else [])],'latest Pet candidate')
run(['cargo','check','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-ui','--target','wasm32-unknown-unknown'],'candidate WASM')
run(['node','--test',*[str(p.relative_to(d)) for p in sorted((d/'pet_apps/pet_ui/tests').glob('*.mjs'))]],'candidate Node')
run(['trunk','build','--skip-version-check','--locked','true','--offline','false'],'candidate Trunk default dev profile',cwd=d/'pet_apps/pet_ui',extra={'NO_COLOR':'true'})
(out/'frontend-assets.json').write_text(json.dumps([{'file':p.name,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'bytes':p.stat().st_size} for p in (d/'pet_apps/pet_ui/dist').glob('*') if p.is_file()],indent=2)+'\n')
dbdir=b/'work/day44-c726-db';dbdir.mkdir(exist_ok=True);old=dbdir/'old-migrations';old.mkdir(exist_ok=True)
for p in (d/'pet_apps/migrations').glob('*.sql'):
 if p.name[:8] < '20261030':shutil.copy2(p,old/p.name)
def upgrade(path,mode):return run(['cargo','run','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-server','--example','day44_upgrade','--',str(path),str(old),str(d/'pet_apps/migrations'),mode],'candidate upgrade '+mode)
synthetic=dbdir/'synthetic.sqlite';assert not synthetic.exists();assert upgrade(synthetic,'seed')==0
source=Path('/home/willie/Documents/rust_ai_ring/.local/pet.sqlite');copy=dbdir/'actual-readonly-backup.sqlite'
def digest(path):
 c=sqlite3.connect(f'file:{path}?mode=ro',uri=True);h=hashlib.sha256();counts={}
 for(t,)in c.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT IN ('_sqlx_migrations','pet_activity','codemap') AND name NOT LIKE 'sqlite_%' ORDER BY name"):
  data=sorted(c.execute('SELECT * FROM "'+t+'"').fetchall(),key=repr);counts[t]=len(data);h.update(t.encode());h.update(repr(data).encode())
 dictionary=sorted(c.execute('SELECT * FROM codemap').fetchall(),key=repr);c.close();return h.hexdigest(),counts,dictionary
if source.exists():
 assert not copy.exists();c=sqlite3.connect(f'file:{source}?mode=ro',uri=True);backup=sqlite3.connect(copy);c.backup(backup);backup.close();c.close();before,counts,db=digest(copy);code=upgrade(copy,'actual');after,counts2,db2=digest(copy)
 (out/'actual-copy-preservation.json').write_text(json.dumps({'base_sha':sha,'source_open_mode':'SQLite mode=ro online backup','original_db_written':False,'exit_code':code,'other_tables_sha256_before':before,'other_tables_sha256_after':after,'other_tables_unchanged':before==after and counts==counts2,'existing_codemap_rows_preserved':all(x in db2 for x in db),'dictionary_rows_added':len(db2)-len(db),'table_counts':counts,'private_rows_published':False},indent=2)+'\n')
bad=dbdir/'checksum-copy.sqlite';shutil.copy2(synthetic,bad);assert upgrade(bad,'tamper')==0
patchpath=Path('/home/willie/Documents/copilot_use/docs/teaching/daily/day-44/analytics-logout-fixture-fix.patch');assert run(['git','apply','--check',str(patchpath)],'Analytics fixture check on latest')==0;assert run(['git','apply',str(patchpath)],'Analytics fixture apply on latest')==0
ctl='/usr/lib/postgresql/18/bin/pg_ctl';pg=b/'work/pg';url='postgres://willie@127.0.0.1:55482/day44_latest_retry_20261030'
if run([ctl,'-D',str(pg),'-l',str(out/'postgres.log'),'-o',"-k '' -p 55482 -h 127.0.0.1",'start'],'isolated PG start')==0:
 try:run(['cargo','test','--offline','--locked','--no-fail-fast','--manifest-path','analytics_apps/Cargo.toml','-p','analytics-server','-p','analytics-persistence','--features','postgres-integration','--','--include-ignored'],'latest Analytics fixture PG',extra={'DATABASE_URL':url,'ANALYTICS_DATABASE_URL':url,'ANALYTICS_CHAT_TOKEN':'local-test-chat','ANALYTICS_PET_TOKEN':'local-test-pet'})
 finally:run([ctl,'-D',str(pg),'stop'],'isolated PG stop')
assert subprocess.check_output(['git','diff','--','pet_apps'],cwd=d)==patch
print('LATEST FIX DONE',flush=True)
