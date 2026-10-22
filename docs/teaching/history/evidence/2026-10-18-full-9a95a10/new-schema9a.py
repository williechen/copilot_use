from pathlib import Path
import sqlite3,subprocess,json
out=Path('outputs/full-9a95a10');cases=[]
for service,db,tables in [('chat','work/chat-full9a.sqlite',['chat_token_log','chat_send_log']),('pet','work/pet-full9a.sqlite',['pet_token_log','pet_send_log'])]:
 c=sqlite3.connect(db)
 for table in tables:
  cols=[x[1] for x in c.execute(f'pragma table_info({table})') if x[3] and not x[5]]
  vals=['fixture' for x in cols]
  for name,fields,values,expected in [('valid',['id']+cols,['fixture']+vals,True),('no id',cols,vals,False)]+[(f'no {col}',['id']+[x for x in cols if x!=col],['missing-'+col]+[v for x,v in zip(cols,vals) if x!=col],False) for col in cols]:
   try:c.execute(f"insert into {table}({','.join(fields)}) values({','.join('?' for _ in fields)})",values);ok=True;err=None
   except sqlite3.IntegrityError as e:ok=False;err=str(e)
   cases.append({'service':service,'table':table,'case':name,'expected_accept':expected,'accepted':ok,'error':err});assert ok==expected
 c.rollback();c.close()
# PostgreSQL actual engine, new schema constraints. Each case runs in rollback.
for table in ['analytic_token_log','analytic_log']:
 required=['token_src','token_type','token_context'] if table.endswith('token_log') else ['local_token','json_data']
 for name,cols,vals,expected in [('valid',['id']+required,['fixture']*(len(required)+1),True),('no id',required,['fixture']*len(required),False)]+[(f'no {col}',['id']+[x for x in required if x!=col],['fixture']*len(required),False) for col in required]:
  sql=f"begin;insert into {table}({','.join(cols)}) values({','.join(chr(39)+x+chr(39) for x in vals)});rollback;"
  r=subprocess.run(['/usr/lib/postgresql/18/bin/psql','-h','127.0.0.1','-p','55463','-U','verify63','-d','verify9a','-v','ON_ERROR_STOP=1','-c',sql],capture_output=True,text=True)
  ok=r.returncode==0;cases.append({'service':'analytics','table':table,'case':name,'expected_accept':expected,'accepted':ok,'error':r.stderr});assert ok==expected
out.joinpath('new-schema.json').write_text(json.dumps(cases,indent=2));print(len(cases),'new schema cases passed')
