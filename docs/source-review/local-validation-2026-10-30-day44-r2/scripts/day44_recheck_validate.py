from pathlib import Path
import json,zipfile,hashlib,re,urllib.parse,subprocess
b=Path.cwd();r=Path('/home/willie/Documents/copilot_use');out=b/'outputs/day44-recheck';arc=r/'docs/archive/packages-2026-10-30';pending='recheck-verification-2026-10-30.json';broken=[]
mds=[r/'README.md',*(r/'docs').rglob('*.md')]
for p in mds:
 text=re.sub(r'^```.*?^```[^\n]*','',p.read_text(),flags=re.M|re.S)
 for target in re.findall(r'\]\(([^)]+)\)',text):
  if '://' in target or target.startswith('#'):continue
  path=urllib.parse.unquote(target.split('#')[0]);q=p.parent/path
  if not q.exists() and not path.endswith(pending):broken.append([str(p.relative_to(r)),target])
assert not broken,broken
for p in (r/'docs').rglob('*.json'):json.loads(p.read_text())
s=json.loads((r/'docs/learning-state.json').read_text());old=json.loads((b/'work/day44-r2-doc-originals/docs/learning-state.json').read_text())
for key in ['next_day','makeup_queue','last_full_comparison']:assert s[key]==old[key]
m=json.loads((r/'docs/teaching/current/manifest.json').read_text());assert s['next_day']==m['next_day']=={'chat':45,'pet':45,'analytics':45};assert all(not a['gate2'] for a in m['articles'])
known=set();hashes={}
for p in sorted(arc.glob('*.zip')):
 with zipfile.ZipFile(p) as z:
  assert z.testzip() is None
  for n in z.namelist():
   if n.endswith('/'):continue
   key=(n,hashlib.sha256(z.read(n)).hexdigest())
   if p.name.startswith(('12-','13-')):assert key not in known,(p.name,n)
   known.add(key)
 hashes[p.name]=hashlib.sha256(p.read_bytes()).hexdigest()
for line in (arc/'SHA256SUMS').read_text().splitlines():
 if line.strip():h,n=line.split(maxsplit=1);assert hashes[n.strip().lstrip('*')]==h
catalog=json.loads((arc/'day44-recheck-archive.json').read_text())
for name,h in catalog['prior_zip_hashes_unchanged'].items():assert hashes[name]==h
recoverable=0
for pack in catalog['packages']:
 for row in pack['files']:
  assert (row['member'],row['sha256']) in known;recoverable+=1
  raw=r/row['path']
  if not raw.exists():assert row['path'].startswith('docs/')
patch=r/'docs/teaching/daily/day-44/pet-day44-legacy-code-upgrade-r4.patch';assert patch.read_text() in (r/'docs/teaching/daily/day-44/pet.md').read_text();subprocess.run(['git','apply','--numstat',str(patch)],cwd=r,check=True,stdout=subprocess.DEVNULL)
src=Path('/home/willie/Documents/rust_ai_ring');candidate=b/'work/day44-c726-fixed';sha='4b1008b95bc3e170381fcd1db84695e421f036bd'
for path in [src,candidate]:assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=path,text=True).strip()==sha
assert not subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=src)
for w in ['crates','chat_apps','pet_apps','analytics_apps']:
 for f in ['Cargo.toml','Cargo.lock']:assert (src/w/f).read_bytes()==(candidate/w/f).read_bytes()
for p in (src/'pet_apps/migrations').glob('*.sql'):assert p.read_bytes()==(candidate/'pet_apps/migrations'/p.name).read_bytes()
assert subprocess.check_output(['git','diff','--','pet_apps'],cwd=candidate)==patch.read_bytes()
red=(out/'final-fix/red-regression-source.rs').read_bytes();green=(candidate/'pet_apps/pet_server/tests/day44_activity_migration.rs').read_bytes();assert re.sub(rb'\s',b'',red)==re.sub(rb'\s',b'',green)
proof={'source_sha':sha,'current_markdown_count':len(mds),'broken_relative_links':broken,'all_current_json_parse':True,'thirteen_zip_crc_sha256_valid':True,'prior_eleven_zip_hashes_unchanged':True,'new_archives_no_duplicate_path_content':True,'prior_files_recoverable':recoverable,'inline_patch_equal_to_attachment_and_tested_candidate':True,'real_apply_evidence':'final-fix/commands.json','same_red_green_assertion_source':True,'red_green_byte_equal':red==green,'four_manifest_and_locks_unchanged':True,'all_four_deployed_migrations_byte_unchanged':True,'product_clean_and_no_product_commits':True,'FIFO_next_day_last_full_comparison_unchanged':True,'complete_product_gate2':False}
(out/'document-checks.json').write_text(json.dumps(proof,ensure_ascii=False,indent=2)+'\n');print(json.dumps(proof,ensure_ascii=False,indent=2))
