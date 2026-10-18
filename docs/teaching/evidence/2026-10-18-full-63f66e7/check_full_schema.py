from pathlib import Path
import sqlite3,json
out=Path('outputs/full-63f66e7');results=[]
def case(c,name,sql,accept):
 try:c.execute(sql);actual=True;err=None
 except sqlite3.IntegrityError as e:actual=False;err=str(e)
 results.append({'case':name,'sql':sql,'accepted':actual,'expected_accept':accept,'error':err});assert actual==accept,(name,actual,err)
p=sqlite3.connect('work/pet-main-full63.sqlite');p.execute('pragma foreign_keys=on')
case(p,'owner explicit ID',"insert into pet_owner(id,name) values('fixture-owner','fixture')",True)
case(p,'owner missing ID',"insert into pet_owner(name) values('fixture')",False)
case(p,'pet explicit IDs',"insert into pet_profile(id,name,species,owner_id) values('fixture-pet','fixture','cat','fixture-owner')",True)
case(p,'pet missing own ID',"insert into pet_profile(name,species,owner_id) values('fixture','cat','fixture-owner')",False)
case(p,'pet missing owner',"insert into pet_profile(id,name,species) values('fixture-bad','fixture','cat')",False)
for t in ['pet_extra','pet_activity','pet_study','pet_life_log']:
 case(p,t+' valid',f"insert into {t}(id,pet_id,owner_id) values('fixture','fixture-pet','fixture-owner')",True)
 case(p,t+' missing primary ID',f"insert into {t}(pet_id,owner_id) values('fixture-pet','fixture-owner')",False)
 case(p,t+' missing pet',f"insert into {t}(id,owner_id) values('null-pet','fixture-owner')",False)
 case(p,t+' missing owner',f"insert into {t}(id,pet_id) values('null-owner','fixture-pet')",False)
 case(p,t+' orphan pet',f"insert into {t}(id,pet_id,owner_id) values('bad-pet','missing','fixture-owner')",False)
case(p,'owner delete restricted',"delete from pet_owner where id='fixture-owner'",False)
assert not p.execute('pragma foreign_key_check').fetchall();assert p.execute('pragma integrity_check').fetchone()[0]=='ok';p.rollback();p.close()
c=sqlite3.connect('work/chat-full63.sqlite');c.execute('pragma foreign_keys=on')
case(c,'AI explicit ID',"insert into ai_person(id,name) values('fixture-ai','fixture')",True)
case(c,'AI missing ID',"insert into ai_person(name) values('fixture')",False)
case(c,'AI extra person required',"insert into ai_person_extra(id) values('fixture-extra')",False)
case(c,'AI extra missing own ID',"insert into ai_person_extra(ai_person_id) values('fixture-ai')",False)
case(c,'AI log room required',"insert into ai_person_life_log(id,ai_person_id) values('fixture-log','fixture-ai')",False)
case(c,'AI log valid text room ID',"insert into ai_person_life_log(id,ai_person_id,room_id) values('fixture-log','fixture-ai','fixture-room')",True)
assert c.execute("select typeof(room_id) from ai_person_life_log where id='fixture-log'").fetchone()[0]=='text'
case(c,'room missing primary ID',"insert into rooms(category_room_id,name,created_at,updated_at) values('fixture','fixture',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)",False)
assert not c.execute('pragma foreign_key_check').fetchall();c.rollback();c.close()
out.joinpath('schema.json').write_text(json.dumps({'cases':results,'all_expected':True,'scope':'fresh separate SQLite schema; no claim of CRUD/UI ownership integration'},ensure_ascii=False,indent=2));print(len(results),'schema cases passed')
