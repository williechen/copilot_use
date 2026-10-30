from pathlib import Path
import json,re,urllib.parse,zipfile,hashlib,subprocess,os
b=Path.cwd();r=Path('/home/willie/Documents/copilot_use');out=b/'outputs/gate2-2026-10-31';ev=r/'docs/source-review/gate2-2026-10-31';arc=r/'docs/archive/packages-2026-10-30';pending='gate2-delivery-2026-10-31.json';broken=[]
for p in [r/'README.md',*(r/'docs').rglob('*.md')]:
 text=re.sub(r'^```.*?^```[^\n]*','',p.read_text(),flags=re.M|re.S)
 for target in re.findall(r'\]\(([^)]+)\)',text):
  if '://' in target or target.startswith('#'):continue
  local=urllib.parse.unquote(target.split('#')[0])
  if not (p.parent/local).exists() and not local.endswith(pending):broken.append([str(p.relative_to(r)),target])
assert not broken,broken
for p in (r/'docs').rglob('*.json'):json.loads(p.read_text())
known=set();hashes={}
for p in sorted(arc.glob('*.zip')):
 with zipfile.ZipFile(p) as z:
  assert z.testzip() is None
  for n in z.namelist():
   if n.endswith('/'):continue
   key=(n,hashlib.sha256(z.read(n)).hexdigest())
   if p.name.startswith('15-'):assert key not in known,(p,n)
   known.add(key)
 hashes[p.name]=hashlib.sha256(p.read_bytes()).hexdigest()
for l in (arc/'SHA256SUMS').read_text().splitlines():
 if l.strip():h,n=l.split(maxsplit=1);assert hashes[n.strip().lstrip('*')]==h
archive=json.loads((arc/'gate2-2026-10-31-archive.json').read_text());assert all(hashes[n]==h for n,h in archive['previous_fourteen_zip_hashes_unchanged'].items())
for row in archive['files']:assert (row['member'],row['sha256']) in known
state=json.loads((r/'docs/learning-state.json').read_text());old=json.loads(subprocess.check_output(['git','show','030bf394:docs/learning-state.json'],cwd=r))
for key in ['next_day','makeup_queue','last_full_comparison']:assert state[key]==old[key]
assert state['current_gate2']['full_gate2']==False
for path in [Path('/home/willie/Documents/rust_ai_ring'),b/'work/day44-recheck-latest']:
 assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=path,text=True).strip()=='2c7ff8b861d32ab6d879bf80c9f2dce50de38097'
 assert not subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=path)
proof={'current_links_valid_except_pending_delivery_proof':True,'current_json_valid':True,'fifteen_zip_crc_sha256_valid':True,'prior_fourteen_zip_hashes_unchanged':True,'new_archive_no_duplicate_path_content':True,'prior_versions_recoverable':True,'FIFO_next_day_last_full_comparison_unchanged':True,'product_and_isolated_source_clean':True,'full_gate2':False,'observed_failure':'explicitly excluded test member HTTP200 and WS opened=true'}
(out/'document-checks.json').write_text(json.dumps(proof,indent=2)+'\n');(ev/'document-checks.json').write_text(json.dumps(proof,indent=2)+'\n');(ev/'scripts/gate2_validate_and_payload.py').write_bytes(Path(__file__).read_bytes())
inventory=[]
for p in sorted(ev.rglob('*')):
 if p.is_file():inventory.append({'path':str(p.relative_to(r)),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'blob_sha':subprocess.check_output(['git','hash-object',str(p)],cwd=r,text=True).strip()})
(ev/'evidence-files.json').write_text(json.dumps({'source_sha':'2c7ff8b861d32ab6d879bf80c9f2dce50de38097','files':inventory,'self_hash_excluded':True},ensure_ascii=False,indent=2)+'\n')
changed=subprocess.check_output(['git','diff','--name-only','-z'],cwd=r).decode().strip('\0').split('\0')
new=[str(p.relative_to(r)) for p in ev.rglob('*') if p.is_file()]+['docs/archive/packages-2026-10-30/15-before-gate2-functional-acceptance-2026-10-31.zip','docs/archive/packages-2026-10-30/gate2-2026-10-31-archive.json']
paths=sorted(set(changed+new));files=[]
for path in paths:
 p=r/path;assert p.is_file();assert path=='README.md' or path.startswith('docs/')
 group='shared'
 if path.startswith('docs/source-review/gate2-2026-10-31/') or path in ['docs/archive/packages-2026-10-30/15-before-gate2-functional-acceptance-2026-10-31.zip','docs/archive/packages-2026-10-30/gate2-2026-10-31-archive.json','docs/archive/packages-2026-10-30/SHA256SUMS']:group='evidence'
 for project in ['chat','pet','analytics']:
  if path==f'docs/teaching/daily/day-44/{project}.md':group=project
 files.append({'path':path,'absolute_path':str(p),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'blob_sha':subprocess.check_output(['git','hash-object',str(p)],cwd=r,text=True).strip(),'mode':'100755' if os.access(p,os.X_OK) else '100644','group':group})
payload={'repository':'williechen/copilot_use','base_commit':'030bf394fde493ef66e63e057109244fa9ca32cf','base_tree':subprocess.check_output(['git','rev-parse','HEAD^{tree}'],cwd=r,text=True).strip(),'files':files};(out/'publication-files.json').write_text(json.dumps(payload,ensure_ascii=False,indent=2)+'\n');print('files',len(files),'bytes',sum(x['bytes'] for x in files),'groups',{g:sum(x['group']==g for x in files) for g in ['evidence','chat','pet','analytics','shared']})
