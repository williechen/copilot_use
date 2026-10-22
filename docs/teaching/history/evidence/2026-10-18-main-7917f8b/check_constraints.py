import sqlite3,json,pathlib
c=sqlite3.connect('work/fresh791.sqlite');c.execute('PRAGMA foreign_keys=ON');results=[]
def run(name,sql,expected):
 try:c.execute(sql);actual='accepted'
 except sqlite3.IntegrityError as e:actual='rejected';error=str(e)
 results.append({'name':name,'sql':sql,'expected':expected,'actual':actual,**({'error':error} if actual=='rejected' else {})});assert actual==expected,(name,actual)
run('owner explicit id',"insert into pet_owner(id,name) values('owner','fixture')",'accepted')
run('pet explicit id and owner',"insert into pet_profile(id,name,species,owner_id) values('pet','fixture','cat','owner')",'accepted')
run('AI explicit id',"insert into ai_person(id,name) values('ai','fixture')",'accepted')
run('pet requires owner',"insert into pet_profile(id,name,species) values('missing-owner','fixture','cat')",'rejected')
for table in ['pet_extra','pet_activity','pet_study','pet_life_log']:
 run(table+' valid references',f"insert into {table}(id,pet_id,owner_id) values('valid','pet','owner')",'accepted')
 run(table+' requires pet',f"insert into {table}(id,owner_id) values('null-pet','owner')",'rejected')
 run(table+' requires owner',f"insert into {table}(id,pet_id) values('null-owner','pet')",'rejected')
 run(table+' rejects missing pet',f"insert into {table}(id,pet_id,owner_id) values('bad-pet','missing','owner')",'rejected')
run('AI extra requires person',"insert into ai_person_extra(id) values('null-person')",'rejected')
run('AI log requires person',"insert into ai_person_life_log(id,room_id) values('null-person',1)",'rejected')
run('AI log requires room',"insert into ai_person_life_log(id,ai_person_id) values('null-room','ai')",'rejected')
run('AI log valid required fields',"insert into ai_person_life_log(id,ai_person_id,room_id) values('valid','ai',1)",'accepted')
run('owner PK still nullable',"insert into pet_owner(name) values('missing id')",'accepted')
run('pet PK still nullable',"insert into pet_profile(name,species,owner_id) values('missing id','cat','owner')",'accepted')
run('AI PK still nullable',"insert into ai_person(name) values('missing id')",'accepted')
result={'sqlite_version':sqlite3.sqlite_version,'cases':results,'owner_ids':c.execute('select id,name from pet_owner').fetchall(),'pet_ids':c.execute('select id,name from pet_profile').fetchall(),'ai_ids':c.execute('select id,name from ai_person').fetchall(),'foreign_key_check':c.execute('pragma foreign_key_check').fetchall()}
pathlib.Path('outputs/revalidation-7917f8b/constraints.json').write_text(json.dumps(result,ensure_ascii=False,indent=2));c.rollback();print('Verified',len(results),'schema cases; missing PKs remain accepted NULL')
