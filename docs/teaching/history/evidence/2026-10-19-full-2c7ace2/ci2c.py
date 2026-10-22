import json,pathlib,urllib.request,concurrent.futures,datetime
p=pathlib.Path('outputs/full-2c7ace2');sha='2c7ace259ef790cea303d841f5779cd5f022d554'
def get(url):
 with urllib.request.urlopen(url,timeout=40) as r:return json.load(r)
r=get(f'https://api.github.com/repos/williechen/rust_ai_chat/actions/runs?head_sha={sha}&per_page=100')
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as e:jobs=list(e.map(lambda x:get(x['jobs_url']+'?per_page=100'),r['workflow_runs']))
p.joinpath('ci.json').write_text(json.dumps({'sha':sha,'checked_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'runs':r,'jobs':jobs},indent=2));print([(v['name'],v['status'],v['conclusion']) for j in jobs for v in j['jobs']])
