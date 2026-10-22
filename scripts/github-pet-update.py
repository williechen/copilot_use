#!/usr/bin/env python3
"""Signed native Linux updates from williechen/copilot_use stable GitHub releases."""
import argparse, datetime, fcntl, hashlib, json, os, pathlib, re, shutil, signal, sqlite3, subprocess, tarfile, tempfile, time, urllib.parse, urllib.request
from pathlib import Path
REPOSITORY='williechen/copilot_use'
API=f'https://api.github.com/repos/{REPOSITORY}/releases/latest'
MANIFEST='copilot-pet-linux-x86_64.json'
MAX=128*1024*1024
FILES={'copilot-desktop-pet','github-update.py','launch.sh','icon.png','build.json'}

def version(s):
    if not isinstance(s,str) or not re.fullmatch(r'v?(0|[1-9][0-9]{0,8})\.(0|[1-9][0-9]{0,8})\.(0|[1-9][0-9]{0,8})',s):raise ValueError('invalid stable version')
    return tuple(map(int,s.removeprefix('v').split('.')))

def candidate(release,current):
    if release.get('draft') or release.get('prerelease'):raise ValueError('only stable published releases are accepted')
    tag=release['tag_name'];new=version(tag)
    if new<=version(current):return None
    assets=release.get('assets',[])
    names=[a for a in assets if a.get('name')==MANIFEST]
    if len(names)!=1:raise ValueError('stable release has no native update manifest')
    return tag,asset_url(names[0],tag,MANIFEST),assets

def asset_url(asset,tag,name):
    expected=f'https://github.com/{REPOSITORY}/releases/download/{tag}/{name}'
    if asset.get('browser_download_url')!=expected or asset.get('size',MAX+1)>MAX:raise ValueError('unexpected release asset URL or size')
    return expected

class Redirects(urllib.request.HTTPRedirectHandler):
    def redirect_request(self,req,fp,code,msg,headers,newurl):
        p=urllib.parse.urlsplit(newurl)
        if p.scheme!='https' or p.username or p.password or p.hostname not in {'github.com','release-assets.githubusercontent.com','objects.githubusercontent.com'}:raise ValueError('untrusted download redirect')
        return super().redirect_request(req,fp,code,msg,headers,newurl)

def fetch(url,limit):
    request=urllib.request.Request(url,headers={'User-Agent':'copilot-desktop-pet-updater','Accept':'application/vnd.github+json' if url==API else 'application/octet-stream'})
    with urllib.request.build_opener(Redirects()).open(request,timeout=30) as response:
        data=response.read(limit+1)
        if len(data)>limit:raise ValueError('update response exceeds size limit')
        return data

def unpack(archive,destination):
    seen=set();total=0
    with tarfile.open(archive,'r:gz') as package:
        for member in package:
            if member.name not in FILES or member.name in seen or not member.isfile():raise ValueError('unsafe update archive member')
            total+=member.size
            if member.size<0 or total>MAX:raise ValueError('unpacked update exceeds limit')
            seen.add(member.name)
            with package.extractfile(member) as src,(destination/member.name).open('xb') as dst:shutil.copyfileobj(src,dst)
    if seen!=FILES:raise ValueError('incomplete update archive')
    for name in FILES:(destination/name).chmod(0o700 if name in {'copilot-desktop-pet','launch.sh','github-update.py'} else 0o600)

def clean_env():
    env=os.environ.copy()
    for key in list(env):
        if key.startswith('PET_GUI_TEST') or key in {'OPENAI_API_KEY','PET_DATA_DIR','PET_AUTHORIZED_ROOT','LD_LIBRARY_PATH','WEBKIT_EXEC_PATH','PET_ENABLE_ROAM'}:env.pop(key,None)
    return env

def info(binary):
    return json.loads(subprocess.check_output([str(binary),'--build-info'],env=clean_env(),text=True,timeout=15))

def installed_processes(binary):
    pids=[]
    for p in Path('/proc').glob('[0-9]*/exe'):
        try:
            if p.parent.stat().st_uid==os.getuid() and os.readlink(p)==str(binary):pids.append(int(p.parent.name))
        except OSError:pass
    return pids

def launch(entry,cache):
    with (cache/'startup.log').open('ab') as log:
        process=subprocess.Popen([str(entry)],env=clean_env(),stdout=log,stderr=log,start_new_session=True)
    time.sleep(4)
    if process.poll() is not None:raise RuntimeError('updated application did not stay running')
    return process.pid

def activate(app,staged,metadata,cache,data,entry):
    """Update an existing installation; never recreate its desktop entry or user data."""
    previous=(app/'current').resolve(strict=True)
    releases=(app/'releases').resolve(strict=True)
    if previous.parent!=releases:raise ValueError('current release is outside managed releases')
    new_version=metadata['version'];version(new_version)
    destination=releases/('v'+new_version)
    installed=json.loads((previous/'installation.json').read_text())
    if version(new_version)<=version(installed['version']):raise ValueError('update must be newer than current version')
    cached=destination.exists()
    if cached and any(not (destination/name).is_file() or hashlib.sha256((destination/name).read_bytes()).digest()!=hashlib.sha256((staged/name).read_bytes()).digest() for name in FILES):raise ValueError('immutable cached release differs from signed package')
    binary=staged/'copilot-desktop-pet'
    built=info(binary)
    if built.get('version')!=new_version or built.get('source_sha')!=metadata['source_sha'] or built.get('source_dirty') or not built.get('release'):raise ValueError('signed binary build identity differs')
    if hashlib.sha256(binary.read_bytes()).hexdigest()!=metadata['binary_sha256']:raise ValueError('signed binary digest differs')
    stamp=datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    backup=None
    if data.is_file():
        backup=cache/f'pre-update-{stamp}-{os.getpid()}.sqlite';backup.touch(mode=0o600)
        with sqlite3.connect(data) as source,sqlite3.connect(backup) as target:source.backup(target)
    pids=installed_processes(previous/'copilot-desktop-pet')
    for pid in pids:
        try:os.kill(pid,signal.SIGTERM)
        except ProcessLookupError:pass
    deadline=time.monotonic()+10
    while installed_processes(previous/'copilot-desktop-pet'):
        if time.monotonic()>deadline:raise RuntimeError('current application did not stop; update not activated')
        time.sleep(.1)
    if not cached:shutil.move(str(staged),destination)
    manifest={**metadata,'installation':'per-user native Linux','auto_start':False}
    if not cached:(destination/'installation.json').write_text(json.dumps(manifest,indent=2)+'\n')
    temporary=app/'.current-update';temporary.unlink(missing_ok=True);temporary.symlink_to(destination);temporary.replace(app/'current')
    pid=None
    try:
        if pids:pid=launch(entry,cache)
    except Exception:
        temporary.symlink_to(previous);temporary.replace(app/'current')
        if pids:launch(entry,cache)
        raise RuntimeError('startup failed; previous release restored; SQLite backup retained') from None
    return {'status':'updated','version':new_version,'source_sha':metadata['source_sha'],'restarted':bool(pids),'pid':pid,'backup_created':backup is not None}

def check(app,config,cache,data,entry,fetcher=fetch):
    current=(app/'current').resolve(strict=True)
    installed=json.loads((current/'installation.json').read_text())
    release=json.loads(fetcher(API,1024*1024))
    selected=candidate(release,installed['version'])
    if selected is None:return {'status':'up_to_date','version':installed['version'],'latest_release':release['tag_name']}
    tag,manifest_url,assets=selected
    channel=json.loads(fetcher(manifest_url,65536))
    target='.'.join(map(str,version(tag)))
    if channel.get('version')!=target or not re.fullmatch('[0-9a-f]{40}',channel.get('source_sha','')):raise ValueError('manifest differs from stable release')
    name=f'copilot-pet_{target}_linux-x86_64.tar.gz'
    candidates=[a for a in assets if a.get('name')==name]
    if len(candidates)!=1:raise ValueError('stable release has no native update package')
    url=asset_url(candidates[0],tag,name)
    key=config/'update-public-key.pub'
    if not key.is_file():raise ValueError('trusted update public key is not configured')
    with tempfile.TemporaryDirectory(prefix='.update-',dir=app) as scratch:
        stage=Path(scratch);package=stage/'update.tar.gz';signature=stage/'update.sig'
        package.write_bytes(fetcher(url,MAX));signature.write_text(channel['signature'])
        # Verification runs only the already trusted installed program, before unpacking/executing new code.
        subprocess.run([str(current/'copilot-desktop-pet'),'--verify-native-update',str(package),str(signature),str(key),target],env=clean_env(),check=True,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,timeout=30)
        extracted=stage/'release';extracted.mkdir(mode=0o700);unpack(package,extracted)
        metadata=json.loads((extracted/'build.json').read_text())
        if metadata.get('version')!=target or metadata.get('source_sha')!=channel['source_sha']:raise ValueError('signed package differs from manifest')
        return activate(app,extracted,metadata,cache,data,entry)

def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--bootstrap-package',type=Path);args=parser.parse_args()
    if os.geteuid()==0:raise SystemExit('Run as the desktop user, never root')
    os.umask(0o077)
    home=Path.home();base=Path(os.environ.get('XDG_DATA_HOME',home/'.local/share'))
    app=base/'copilot-desktop-pet';config=Path(os.environ.get('XDG_CONFIG_HOME',home/'.config'))/'copilot-desktop-pet'
    cache=Path(os.environ.get('XDG_CACHE_HOME',home/'.cache'))/'copilot-desktop-pet';cache.mkdir(parents=True,exist_ok=True,mode=0o700)
    data=base/'com.copilotuse.desktoppet/pet.sqlite';entry=home/'.local/bin/copilot-desktop-pet'
    with (cache/'update.lock').open('w') as lock:
        try:fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
        except BlockingIOError:return
        try:
            if args.bootstrap_package:
                # One-time local migration for installations predating the native verifier. No installer is run.
                with tempfile.TemporaryDirectory(prefix='.bootstrap-',dir=app) as scratch:
                    stage=Path(scratch);unpack(args.bootstrap_package,stage)
                    metadata=json.loads((stage/'build.json').read_text())
                    result=activate(app,stage,metadata,cache,data,entry)
            else:result=check(app,config,cache,data,entry)
        except Exception as error:
            result={'status':'error','reason':type(error).__name__,'message':'更新未完成；保留目前安裝與資料，稍後重試'}
            if args.bootstrap_package:raise
        result['checked_at']=datetime.datetime.now(datetime.timezone.utc).isoformat();result['repository']=REPOSITORY
        target=cache/'update-status.json';tmp=cache/'.update-status-new';tmp.write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');tmp.replace(target)
        print(json.dumps(result,ensure_ascii=False))
if __name__=='__main__':main()
