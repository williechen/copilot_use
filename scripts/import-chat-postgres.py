#!/usr/bin/env python3
"""Read-only legacy Chat export into a new portable SQLite; never overwrite either database."""
import argparse,datetime,json,os,pathlib,sqlite3,subprocess,tempfile,uuid,urllib.parse
ROOT=pathlib.Path(__file__).resolve().parents[1]
a=argparse.ArgumentParser();a.add_argument('--source-env',default='CHAT_LEGACY_DATABASE_URL');a.add_argument('--output',type=pathlib.Path,required=True);a.add_argument('--binary',type=pathlib.Path,default=ROOT/'target/debug/day9-chat');args=a.parse_args()
source=os.environ.get(args.source_env)
if not source:raise SystemExit('Set the selected source environment variable; do not pass credentials on the command line')
output=args.output.resolve();output.parent.mkdir(parents=True,exist_ok=True)
if output.exists():raise SystemExit('Destination already exists; import never overwrites data')
# psql does not print the credential-bearing connection string or raw rows to the terminal.
url=urllib.parse.urlsplit(source)
if url.scheme not in ('postgres','postgresql') or not url.hostname or not url.path.strip('/'):
    raise SystemExit('Source must be a PostgreSQL URL with an explicit database')
env=os.environ|{'PGHOST':url.hostname,'PGPORT':str(url.port or 5432),'PGDATABASE':urllib.parse.unquote(url.path.lstrip('/')),'PGOPTIONS':'-c default_transaction_read_only=on -c statement_timeout=300000'}
if url.username is not None:env['PGUSER']=urllib.parse.unquote(url.username)
if url.password is not None:env['PGPASSWORD']=urllib.parse.unquote(url.password)
allowed={'sslmode':'PGSSLMODE','sslrootcert':'PGSSLROOTCERT','sslcert':'PGSSLCERT','sslkey':'PGSSLKEY','connect_timeout':'PGCONNECT_TIMEOUT'}
for key,value in urllib.parse.parse_qsl(url.query):
    if key not in allowed:raise SystemExit('Unsupported PostgreSQL URL option')
    env[allowed[key]]=value
psql=['psql','--no-psqlrc','-X','-A','-t','-v','ON_ERROR_STOP=1']
process=None;connection=None
with tempfile.TemporaryDirectory(prefix='.chat-import-',dir=output.parent)as scratch:
    staged=pathlib.Path(scratch)/'chat.sqlite3'
    subprocess.run([args.binary,'--initialize'],cwd=ROOT,env=os.environ|{'CHAT_DATABASE_URL':'sqlite://'+str(staged)},check=True,stdout=subprocess.DEVNULL)
    connection=sqlite3.connect(staged)
    connection.execute('PRAGMA foreign_keys=OFF')
    connection.execute('BEGIN IMMEDIATE')
    target={row[0] for row in connection.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name<>'_sqlx_migrations'")}
    metadata=subprocess.run(psql+['-c',"SELECT json_agg(tablename ORDER BY tablename) FROM pg_tables WHERE schemaname='public' AND tablename<>'_sqlx_migrations'"],env=env,text=True,capture_output=True)
    if metadata.returncode:raise SystemExit('Read-only PostgreSQL schema inspection failed')
    tables=json.loads(metadata.stdout)or[]
    if set(tables)-target:raise SystemExit('Legacy schema contains unsupported tables; refusing incomplete import')
    statements=['BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY;']
    for table in tables:
        # Identifiers were checked against target schema, never accepted from arbitrary user SQL.
        statements.append(f'''SELECT json_build_object('table','{table}','row',row_to_json(t)) FROM "{table}" t;''')
    statements.append('COMMIT;')
    counts={};columns={t:{r[1]:r[2].upper() for r in connection.execute(f'PRAGMA table_info("{t}")')}for t in tables}
    with open(pathlib.Path(scratch)/'source-error.log','w')as error:
        process=subprocess.Popen(psql,env=env,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=error,text=True)
        process.stdin.write('\n'.join(statements));process.stdin.close()
        try:
            for line in process.stdout:
                line=line.strip()
                if not line or line in ('BEGIN','COMMIT'):continue
                if len(line)>1024*1024:raise ValueError('Legacy row exceeds import safety limit')
                item=json.loads(line);table=item['table'];row=item['row'];values=[]
                if set(row)-columns[table].keys():raise ValueError('Unsupported legacy columns; refusing data loss')
                for key,value in row.items():
                    kind=columns[table][key]
                    if value is not None:
                        if kind=='BLOB':value=bytes.fromhex(value[2:])if value.startswith('\\x')else uuid.UUID(value).bytes
                        elif kind=='REAL':value=datetime.datetime.fromisoformat(value.replace('Z','+00:00')).timestamp()if isinstance(value,str)else value
                        elif isinstance(value,(dict,list)):value=json.dumps(value,ensure_ascii=False,sort_keys=True,separators=(',',':'))
                    values.append(value)
                names=','.join('"'+name+'"'for name in row)
                placeholders=','.join('?'for _ in row)
                connection.execute(f'INSERT INTO "{table}"({names})VALUES({placeholders})',values)
                counts[table]=counts.get(table,0)+1
            if process.wait()!=0:raise ValueError('Read-only source snapshot failed')
            if connection.execute('PRAGMA foreign_key_check').fetchall():raise ValueError('Imported foreign keys are inconsistent')
            if connection.execute('PRAGMA integrity_check').fetchone()[0]!='ok':raise ValueError('Imported SQLite integrity failed')
            connection.commit();connection.execute('PRAGMA wal_checkpoint(TRUNCATE)');connection.close();connection=None
            # Atomic no-overwrite publication, even if another process creates output during import.
            os.link(staged,output)
            staged.unlink()
            print(json.dumps({'status':'passed','tables':counts,'source_modified':False,'database':'SQLite'},sort_keys=True))
        finally:
            if process.poll()is None:process.terminate();process.wait()
            if connection is not None:connection.rollback();connection.close()
