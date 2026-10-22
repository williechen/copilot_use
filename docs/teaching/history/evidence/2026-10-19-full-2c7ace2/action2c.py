from pathlib import Path
import sqlite3,json,subprocess,shutil
root=Path.cwd();out=root/'outputs/full-2c7ace2';sqlroot=root/'work/acceptance-f948/migrations';cases=[]
for kind in ['valid','null']:
 db=root/'work'/f'action-{kind}-2c.sqlite';shutil.copy(root/'work/chat-fullfa.sqlite',db)
 c=sqlite3.connect(db);c.execute("insert into chat_action_log(id,action_type,action_content,created_user_id) values(?, 'fixture.send','fixture content','fixture-user')",('legacy-good' if kind=='valid' else None,));c.commit();c.close()
 r=subprocess.run(['sqlx','migrate','run','--source',str(sqlroot),'--database-url',f'sqlite://{db}'],capture_output=True,text=True);out.joinpath(f'action-{kind}-upgrade.log').write_text(r.stdout+r.stderr);c=sqlite3.connect(db)
 row=c.execute('select id,action_content from chat_action_log').fetchall()
 if kind=='valid':
  assert r.returncode==0 and row==[('legacy-good','fixture content')]
  for label,sql in [('missing id',"insert into chat_action_log(action_type,created_user_id) values('send','user')"),('NULL id',"insert into chat_action_log(id,action_type,created_user_id) values(NULL,'send','user')"),('NULL update',"update chat_action_log set id=NULL where id='legacy-good'"),('duplicate id',"insert into chat_action_log(id,action_type,created_user_id) values('legacy-good','send','user')")]:
   try:c.execute(sql);passed=False;err=None
   except sqlite3.IntegrityError as e:passed=True;err=str(e)
   assert passed;cases.append({'case':label,'passed':passed,'error':err})
  c.rollback();c.close();c=sqlite3.connect(db);assert c.execute("select action_content from chat_action_log where id='legacy-good'").fetchone()[0]=='fixture content';cases.append({'case':'valid legacy row preserved across migration and reconnect','passed':True})
 else:
  assert r.returncode==1 and row==[(None,'fixture content')];assert not c.execute("select name from sqlite_master where name='chat_action_log_required_id'").fetchall();assert c.execute('select count(*) from _sqlx_migrations where version=20261018130000').fetchone()[0]==0;cases.append({'case':'legacy NULL fails closed and preserves old row/structure/history','passed':True,'error':r.stderr})
 c.close()
out.joinpath('action-schema-green.json').write_text(json.dumps(cases,indent=2));print(len(cases),'forward migration/identity cases passed; actor binding still not implemented')
