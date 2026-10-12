#!/usr/bin/env python3
"""Publish only an explicitly approved, tested immutable source; no public deployment."""
import hashlib,json,os,pathlib,re,subprocess,tempfile,urllib.request,urllib.error,urllib.parse
ROOT=pathlib.Path(__file__).resolve().parents[1]
repository=os.environ['GH_REPOSITORY']
if repository!='williechen/copilot_use':raise SystemExit('Release is scoped to williechen/copilot_use')
approval=json.loads((ROOT/'.github/release-approved.json').read_text())
sha=approval['approved_sha'];tag=approval['tag']
if not re.fullmatch('[0-9a-f]{40}',sha)or not re.fullmatch(r'v\d+\.\d+\.\d+(?:-p9)?',tag):raise SystemExit('Invalid immutable source/tag')
if subprocess.check_output(['git','rev-parse','HEAD^'],cwd=ROOT,text=True).strip()!=sha:raise SystemExit('Approval must follow the exact tested commit')
class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self,*args):return None
client=urllib.request.build_opener(NoRedirect())
def request(url,data=None,method=None):
    headers={'Authorization':'Bearer '+os.environ['GH_TOKEN'],'Accept':'application/vnd.github+json','X-GitHub-Api-Version':'2022-11-28'}
    body=None if data is None else json.dumps(data).encode()
    return client.open(urllib.request.Request(url,data=body,headers=headers,method=method),timeout=120)
def api(path,data=None,method=None):
    with request('https://api.github.com/repos/'+repository+path,data,method)as response:return json.load(response)
def optional(path):
    try:return api(path)
    except urllib.error.HTTPError as error:
        if error.code==404:return None
        raise
required={'live','docker','packaged_gui','appimage_update'}
if set(approval['evidence'])!=required:raise SystemExit('Approval must contain precisely the four required reports')
for kind in sorted(required):
    evidence=(ROOT/approval['evidence'][kind]).resolve()
    if not evidence.is_relative_to(ROOT/'docs/release-evidence')or evidence.suffix!='.json':raise SystemExit('Evidence must be checked-in release JSON')
    report=json.loads((ROOT/approval['evidence'][kind]).read_text())
    actual=report.get('source_sha',report.get('source_head'))
    dirty=report.get('source_dirty',report.get('working_tree_dirty',True))
    passed=report.get('status')=='passed'or report.get('success')is True
    if actual!=sha or dirty is not False or not passed:raise SystemExit('Local evidence does not prove exact clean source: '+kind)
    if kind=='docker'and (set(report.get('containers',{}))!={'chat','analytics'}or any(c.get('revision')!=sha or not re.fullmatch('sha256:[0-9a-f]{64}',c.get('image_id',''))for c in report['containers'].values())):raise SystemExit('Actual Docker image revisions do not match approved source')
    if kind=='live'and(report.get('live_openai')is not True or report.get('openai_model')!='gpt-5.6-luna'):raise SystemExit('Real configured-model validation missing')
    if kind in ('packaged_gui','appimage_update')and report.get('release')is not True:raise SystemExit('Packaged release validation missing')
    if kind=='appimage_update'and not report.get('update_install'):raise SystemExit('Actual signed update installation missing')
runs=api('/actions/runs?head_sha='+sha+'&per_page=100')['workflow_runs']
matching=[r for r in runs if r['path']=='.github/workflows/verify.yml'and r['head_sha']==sha]
if not matching or matching[0]['status']!='completed'or matching[0]['conclusion']!='success':raise SystemExit('Exact-source verification workflow has not passed')
run=matching[0]
jobs=api('/actions/runs/'+str(run['id'])+'/jobs?per_page=100')['jobs']
expected={'rust-postgres','compose-e2e','local-fault-recovery','desktop (ubuntu-latest, deb,appimage)','desktop (macos-latest, app)','desktop (windows-latest, nsis)'}
if {j['name']for j in jobs}!=expected or any(j['conclusion']!='success'for j in jobs):raise SystemExit('All six required jobs must pass; skipped jobs are not success')
artifacts=api('/actions/runs/'+str(run['id'])+'/artifacts?per_page=100')['artifacts']
selected=[a for a in artifacts if a['name'].startswith('desktop-')and not a['expired']]
if len(selected)!=3:raise SystemExit('All three actual platform artifacts are required')
with tempfile.TemporaryDirectory(prefix='verified-p9-')as scratch:
    files=[]
    for artifact in selected:
        try:
            request('https://api.github.com/repos/'+repository+'/actions/artifacts/'+str(artifact['id'])+'/zip')
            raise SystemExit('Expected artifact download redirect')
        except urllib.error.HTTPError as redirect:
            if redirect.code!=302:raise
            location=redirect.headers['Location']
        parsed=urllib.parse.urlsplit(location)
        if parsed.scheme!='https'or parsed.username or parsed.password:raise SystemExit('Unsafe artifact redirect')
        file=pathlib.Path(scratch)/(artifact['name']+'.zip');total=0
        # Presigned storage request never receives the GitHub credential header.
        with urllib.request.urlopen(location,timeout=120)as response,file.open('wb')as output:
            while chunk:=response.read(65536):
                total+=len(chunk)
                if total>512*1024*1024:raise SystemExit('Artifact exceeds release bound')
                output.write(chunk)
        files.append(file)
    for value in approval['evidence'].values():files.append(ROOT/value)
    manifest={'source_sha':sha,'tag':tag,'ci_run':run['html_url'],'files':{p.name:hashlib.sha256(p.read_bytes()).hexdigest()for p in files}}
    manifest_file=pathlib.Path(scratch)/'release-manifest.json';manifest_file.write_text(json.dumps(manifest,indent=2)+'\n');files.append(manifest_file)
    ref=optional('/git/ref/tags/'+tag)
    if ref and ref['object']['sha']!=sha:raise SystemExit('Existing tag differs; immutable tags are never overwritten')
    if not ref:api('/git/refs',{'ref':'refs/tags/'+tag,'sha':sha})
    release=optional('/releases/tags/'+tag)
    body=approval['notes']+'\n\nSource: `'+sha+'`\n\nCI: '+run['html_url']
    if release and (release['body']!=body or not release['draft']):raise SystemExit('Existing published or unrelated release is immutable')
    if not release:release=api('/releases',{'tag_name':tag,'target_commitish':sha,'name':tag+' — local P9 validation','body':body,'draft':True,'prerelease':False})
    upload_base=release['upload_url'].split('{')[0]
    if urllib.parse.urlsplit(upload_base).hostname!='uploads.github.com':raise SystemExit('Unexpected asset upload host')
    existing={asset['name']:asset for asset in release['assets']}
    for file in files:
        if file.name in existing:
            expected='sha256:'+hashlib.sha256(file.read_bytes()).hexdigest()
            if existing[file.name].get('digest')!=expected:raise SystemExit('Existing draft asset digest differs; refusing overwrite')
            continue
        upload=upload_base+'?name='+urllib.parse.quote(file.name,safe='')
        headers={'Authorization':'Bearer '+os.environ['GH_TOKEN'],'Content-Type':'application/octet-stream','Accept':'application/vnd.github+json'}
        with client.open(urllib.request.Request(upload,data=file.read_bytes(),headers=headers),timeout=180)as response:json.load(response)
    release=api('/releases/'+str(release['id']),{'draft':False},method='PATCH')
    print(json.dumps({'status':'published','tag':tag,'source_sha':sha,'url':release['html_url']}))
