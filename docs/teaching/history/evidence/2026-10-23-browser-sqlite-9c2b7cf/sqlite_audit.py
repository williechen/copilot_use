import pathlib,subprocess,sqlite3,hashlib,json,os,time,socket,shutil
r=pathlib.Path(__file__).resolve().parent;src=r.parent/'rust_ai_chat'
binary='/home/willie/Documents/Codex/2026-10-23/referenced-chatgpt-conversation-this-is-an-2/work/target-full/debug/server'
fixtures=r/('sqlite-fixtures-'+str(time.time_ns()));fixtures.mkdir()
results=[]
def blob(ref,path):return subprocess.check_output(['git','show',ref+':'+path],cwd=src)
def state(path):
 with sqlite3.connect(path) as c:return list(c.iterdump())
def run(name,path):
 env={k:os.environ[k] for k in ['HOME','PATH'] if k in os.environ}
 env.update(CHAT_DATABASE_URL='sqlite:'+str(path)+'?mode=rwc',LEPTOS_OUTPUT_NAME='rust_ai_chat')
 before=state(path)
 f=open(r/'logs'/('sqlite-'+name+'.log'),'w');p=subprocess.Popen([binary],cwd=fixtures,env=env,stdout=f,stderr=subprocess.STDOUT)
 started=False
 for _ in range(80):
  if p.poll() is not None:break
  with socket.socket() as s:
   if s.connect_ex(('127.0.0.1',3000))==0:started=True;break
  time.sleep(.05)
 if p.poll() is None:p.terminate();p.wait(timeout=5)
 f.close();after=state(path)
 log=(r/'logs'/('sqlite-'+name+'.log')).read_text()
 rec=dict(case=name,started=started,exit=p.returncode,logical_database_unchanged=before==after,log='sqlite-'+name+'.log',migration_error=log.strip() if not started else None)
 results.append(rec);return rec
with socket.socket() as s:
 if s.connect_ex(('127.0.0.1',3000))==0:raise RuntimeError('stop only own test server before audit')
base=r/'runtime/browser.sqlite'
# A database already at the current schema must survive reopening with its data.
with sqlite3.connect(base) as c,sqlite3.connect(fixtures/'current.sqlite') as dest:c.backup(dest)
rec=run('current-reopen',fixtures/'current.sqlite');assert rec['started']
with sqlite3.connect(fixtures/'current.sqlite') as c:
 assert c.execute("SELECT count(*) FROM rooms WHERE id LIKE 'browser-room-%'").fetchone()[0]==7
 assert c.execute("SELECT count(*) FROM mbr_user WHERE user_acnt='browser_audit'").fetchone()[0]==1
constraints=[]
with sqlite3.connect(fixtures/'current.sqlite') as c:
 c.execute("INSERT INTO chat_action_log(id,action_type,created_user_id) VALUES('stable-log','fixture','nonexistent-actor')")
 constraints.append(dict(case='unbound_actor_id',accepted=True,meaning='created_user_id is NOT NULL but not linked to a real authenticated user'))
 for label,sql in [
  ('missing_id',"INSERT INTO chat_action_log(action_type,created_user_id) VALUES('fixture','actor')"),
  ('null_id',"INSERT INTO chat_action_log(id,action_type,created_user_id) VALUES(NULL,'fixture','actor')"),
  ('update_id_to_null',"UPDATE chat_action_log SET id=NULL WHERE id='stable-log'"),
  ('duplicate_id',"INSERT INTO chat_action_log(id,action_type,created_user_id) VALUES('stable-log','fixture','actor')")]:
  try:c.execute(sql);accepted=True
  except sqlite3.IntegrityError:accepted=False
  constraints.append(dict(case=label,accepted=accepted));assert not accepted,label
 c.commit()
# Full historical schema is generated from exact fa0888e Git blobs. SQLx history uses
# the real SHA384 algorithm, verified against sqlx-core 0.9.0 source.
legacy_paths=subprocess.check_output(['git','ls-tree','-r','--name-only','fa0888e','migrations'],cwd=src,text=True).splitlines()
for kind in ['valid','null']:
 path=fixtures/('full-fa-'+kind+'.sqlite');c=sqlite3.connect(path)
 c.execute('CREATE TABLE _sqlx_migrations (version BIGINT PRIMARY KEY,description TEXT NOT NULL,installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,success BOOLEAN NOT NULL,checksum BLOB NOT NULL,execution_time BIGINT NOT NULL)')
 for pathstr in legacy_paths:
  sql=blob('fa0888e',pathstr);c.executescript(sql.decode())
  version=int(pathlib.Path(pathstr).name.split('_',1)[0]);description=pathlib.Path(pathstr).stem.split('_',1)[1].replace('_',' ')
  c.execute('INSERT INTO _sqlx_migrations(version,description,success,checksum,execution_time) VALUES(?,?,1,?,0)',(version,description,hashlib.sha384(sql).digest()));c.commit()
 c.execute("INSERT INTO rooms(id,category_room_id,name,created_at,updated_at) VALUES('legacy-room','fixture','歷史資料',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)")
 c.execute("INSERT INTO chat_action_log(id,action_type,created_user_id) VALUES(?, 'fixture','legacy-actor')",('legacy-log' if kind=='valid' else None,));c.commit();c.close()
 rec=run('full-fa-'+kind,path);assert not rec['started'] and rec['logical_database_unchanged'] and 'VersionMismatch' in rec['migration_error']
# Isolate the action-log history alone, without rewriting any source migration.
for kind in ['valid','null']:
 path=fixtures/('action-history-'+kind+'.sqlite')
 with sqlite3.connect(base) as a,sqlite3.connect(path) as c:a.backup(c)
 with sqlite3.connect(path) as c:
  c.execute('DROP TABLE chat_action_log');old=blob('fa0888e','migrations/20261018122554_creat_chat_action_log.sql');c.executescript(old.decode())
  c.execute('UPDATE _sqlx_migrations SET checksum=? WHERE version=20261018122554',(hashlib.sha384(old).digest(),))
  c.execute("INSERT INTO chat_action_log(id,action_type,created_user_id) VALUES(?, 'fixture','legacy-actor')",('legacy-log' if kind=='valid' else None,));c.commit()
 rec=run('action-history-'+kind,path);assert not rec['started'] and rec['logical_database_unchanged'] and '20261018122554' in rec['migration_error']
# A fictitious applied version is deliberately missing from current source.
path=fixtures/'missing-version.sqlite'
with sqlite3.connect(base) as a,sqlite3.connect(path) as c:a.backup(c)
with sqlite3.connect(path) as c:c.execute("INSERT INTO _sqlx_migrations(version,description,success,checksum,execution_time) VALUES(20261001000000,'isolated missing fixture',1,?,0)",(b'\0'*48,));c.commit()
rec=run('missing-version',path);assert not rec['started'] and rec['logical_database_unchanged'] and 'VersionMissing' in rec['migration_error']
source_hits=[]
for p in src.rglob('*.rs'):
 if 'target' not in p.parts and 'chat_action_log' in p.read_text(errors='replace'):source_hits.append(str(p.relative_to(src)))
out=dict(source_sha='9c2b7cf5a4c60a08254445911916c4ed9b02d7b3',legacy_source='fa0888e',legacy_fixture_method='exact historical migration SQL plus faithful SQLx SHA384 history; reconstructed fixtures, not user databases',runner_cases=results,current_schema_constraints=constraints,rust_action_log_callers=source_hits,full_product_audit_complete=False)
(r/'logs/sqlite-audit-results.json').write_text(json.dumps(out,ensure_ascii=False,indent=2))
print(json.dumps(out,ensure_ascii=False,indent=2))
