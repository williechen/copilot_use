from pathlib import Path
import json,zipfile,hashlib,re,urllib.parse,subprocess
r=Path('/home/willie/Documents/copilot_use');o=Path.cwd()/'outputs/day44-2026-10-30';arc=r/'docs/archive/packages-2026-10-30'
broken=[];prose_whitespace=[];markdowns=[r/'README.md',*(r/'docs').rglob('*.md')]
for p in markdowns:
 t=re.sub(r'^```.*?^```[^\n]*','',p.read_text(),flags=re.M|re.S)
 for target in re.findall(r'\]\(([^)]+)\)',t):
  if '://' in target or target.startswith('#'):continue
  local=urllib.parse.unquote(target.split('#')[0]);q=p.parent/local
  if not q.exists():broken.append([str(p.relative_to(r)),target])
 for i,l in enumerate(t.splitlines(),1):
  if l!=l.rstrip():prose_whitespace.append([str(p.relative_to(r)),i])
assert not broken,broken
jsons=list((r/'docs').rglob('*.json'))
for p in jsons:json.loads(p.read_text())
s=json.loads((r/'docs/learning-state.json').read_text());before=json.loads(Path('work/day44-doc-originals/docs/learning-state.json').read_text())
for key in ['next_day','makeup_queue','last_full_comparison']:assert s[key]==before[key],key
m=json.loads((r/'docs/teaching/current/manifest.json').read_text());assert m['next_day']==s['next_day'] and all(not a['gate2'] for a in m['articles'])
proof=json.loads((r/'docs/teaching/daily/day-44/revision-verification-2026-10-30.json').read_text())
for row in proof['commits']:
 for f in row['files']:
  b=subprocess.check_output(['git','show',row['commit']+':'+f['path']],cwd=r)
  assert hashlib.sha256(b).hexdigest()==f['sha256']
  assert b==(r/f['path']).read_bytes(),f['path']
for a in m['articles']:
 if a['day']==44:
  b=subprocess.check_output(['git','show',a['article_commit']+':'+a['article_path']],cwd=r);assert hashlib.sha256(b).hexdigest()==a['sha256']
existing={};dedup=set()
for p in sorted(arc.glob('*.zip')):
 with zipfile.ZipFile(p) as z:
  assert z.testzip() is None
  for n in z.namelist():
   if not n.endswith('/'):
    key=(n,hashlib.sha256(z.read(n)).hexdigest())
    if p.name.startswith('11'):assert key not in dedup,(p.name,n)
    dedup.add(key)
 existing[p.name]=hashlib.sha256(p.read_bytes()).hexdigest()
for line in (arc/'SHA256SUMS').read_text().splitlines():
 if line.strip():
  sha,name=line.split(maxsplit=1);assert existing[name.strip().lstrip('*')]==sha
delta=json.loads((arc/'day44-validation-superseded.json').read_text())
for name,sha in delta['existing_zip_hashes_unchanged'].items():assert existing[name]==sha
with zipfile.ZipFile(arc/delta['new_zip']) as z:
 for row in delta['files']:
  if not row['already_archived']:assert hashlib.sha256(z.read(row['archive_member'])).hexdigest()==row['sha256']
day=r/'docs/teaching/daily/day-44';pet=(day/'pet.md').read_text();ana=(day/'analytics.md').read_text()
assert (day/'pet-day44-activity-green-r3.patch').read_text() in pet
assert (day/'analytics-logout-fixture-fix.patch').read_text() in ana
for p in day.glob('*.patch'):
 subprocess.run(['git','apply','--numstat',str(p)],cwd=r,check=True,stdout=subprocess.DEVNULL)
for w in ['crates','chat_apps','pet_apps','analytics_apps']:
 for name in ['Cargo.toml','Cargo.lock']:
  p=Path('/home/willie/Documents/rust_ai_ring')/w/name
  candidate=Path('work/day44-pet-r3')/w/name
  assert p.read_bytes()==candidate.read_bytes()
assert not subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd='/home/willie/Documents/rust_ai_ring')
result={'current_markdown_count':len(markdowns),'json_count':len(jsons),'broken_relative_links':broken,'all_archive_crc_and_sha256_valid':True,'prior_ten_zip_hashes_unchanged':True,'new_archive_not_duplicate':True,'original_versions_recoverable':True,'exact_commit_readbacks_verified_files':sum(len(x['files']) for x in proof['commits']),'inline_pet_and_analytics_patches_byte_equal':True,'patch_numstat_syntax_valid':True,'real_patch_apply_evidence':'final-apply-r3/commands.json','FIFO_next_day_last_full_comparison_unchanged':True,'four_manifest_lock_unchanged':True,'product_worktree_clean':True,'prose_whitespace_existing':prose_whitespace,'diff_check_note':'Exact inline diff fences retain required blank-context spaces and original source whitespace; ordinary prose checked separately.'}
(o/'document-checks.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps(result,ensure_ascii=False,indent=2))
