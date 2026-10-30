from pathlib import Path
import subprocess,sqlite3,hashlib,json,os,time,shutil
r=Path.cwd();d=r/'work/day44-pet-r3';out=r/'outputs/day44-2026-10-30/pet-upgrade';out.mkdir(exist_ok=True);dbdir=r/'work/day44-db';dbdir.mkdir(exist_ok=True);rows=[];env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='1')
for k in ['OPENAI_API_KEY','ANTHROPIC_API_KEY','GEMINI_API_KEY']:env.pop(k,None)
def digest(p):
 c=sqlite3.connect(f'file:{p}?mode=ro',uri=True);h=hashlib.sha256();counts={}
 for(t,)in c.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT IN ('_sqlx_migrations','pet_activity') AND name NOT LIKE 'sqlite_%' ORDER BY name"):
  data=sorted(c.execute('SELECT * FROM "'+t+'"').fetchall(),key=repr);h.update(t.encode());h.update(repr(data).encode());counts[t]=len(data)
 c.close();return h.hexdigest(),counts
old='/home/willie/Documents/rust_ai_ring/pet_apps/migrations';new=str(d/'pet_apps/migrations')
def run(path,mode):
 a=['cargo','run','--offline','--locked','--manifest-path','pet_apps/Cargo.toml','-p','pet-server','--example','day44_upgrade','--',str(path),old,new,mode];t=time.time();p=subprocess.run(a,cwd=d,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300);log=str(len(rows))+'.log';(out/log).write_text(p.stdout);rows.append({'command':a,'cwd':str(d),'exit_code':p.returncode,'seconds':time.time()-t,'log':log,'scope':'isolated Pet candidate; no original database writes'});(out/'commands.json').write_text(json.dumps(rows,indent=2));print(p.returncode,mode,flush=True);return p.returncode
synthetic=dbdir/'synthetic.sqlite';assert not synthetic.exists();run(synthetic,'seed')
source=Path('/home/willie/Documents/rust_ai_ring/.local/pet.sqlite');copy=dbdir/'actual-user-backup.sqlite';assert not copy.exists();s=sqlite3.connect(f'file:{source}?mode=ro',uri=True);b=sqlite3.connect(copy);s.backup(b);b.close();s.close();before,counts=digest(copy);code=run(copy,'actual');after,newcounts=digest(copy);assert code==0 and before==after and counts==newcounts
(out/'actual-copy-result.json').write_text(json.dumps({'source_open_mode':'read-only SQLite online backup','backup':str(copy),'before_nonactivity_data_sha256':before,'after_nonactivity_data_sha256':after,'nonactivity_data_equal':before==after,'table_counts':counts,'scope':'actual local Pet DB backup, new migration on copy only; raw database not published'},indent=2))
bad=dbdir/'checksum-copy.sqlite';shutil.copy2(copy,bad);run(bad,'tamper')
