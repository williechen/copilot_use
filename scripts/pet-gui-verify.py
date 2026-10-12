#!/usr/bin/env python3
"""Exercise this application's own WebKit/WASM/IPC in an isolated disposable state."""
import time, json, os, pathlib, subprocess, tempfile, sys, threading, argparse, hashlib, shutil, sqlite3, tomllib
from http.server import ThreadingHTTPServer,BaseHTTPRequestHandler
from contextlib import closing
ROOT = pathlib.Path(__file__).resolve().parents[1]
REPORT = ROOT / "target/pet-gui-validation.json"
if sys.platform.startswith("linux") and not os.environ.get("DISPLAY"):
    raise SystemExit("A Linux desktop/X11 display is required for actual GUI execution")
parser=argparse.ArgumentParser()
parser.add_argument('--executable',type=pathlib.Path)
parser.add_argument('--update-package',type=pathlib.Path)
parser.add_argument('--expected-executable',type=pathlib.Path)
parser.add_argument('--live-ai',action='store_true')
parser.add_argument('--live-image',action='store_true')
parser.add_argument('--analytics',action='store_true')
parser.add_argument('--updater',action='store_true')
options=parser.parse_args()
if not options.executable:
    if not (ROOT/'apps/pet_desktop/generated-icons/icon.ico').exists():
        subprocess.run(['node','scripts/build-pet-ui.mjs'],cwd=ROOT,check=True)
    subprocess.run(["cargo", "build", "--manifest-path", str(ROOT / "apps/pet_desktop/Cargo.toml"), "--locked"], check=True, cwd=ROOT)
executable=options.executable.resolve() if options.executable else ROOT / ("apps/pet_desktop/target/debug/copilot-desktop-pet.exe" if sys.platform=="win32" else "apps/pet_desktop/target/debug/copilot-desktop-pet")
with tempfile.TemporaryDirectory(prefix="copilot-pet-gui-") as scratch:
    # macOS /var is a symlink; run disposable bundles through the canonical
    # /private path so Tauri keeps its executable-path protection enabled.
    directory = pathlib.Path(scratch).resolve()
    if sys.platform=='win32':
        # Private disposable fixture only; chmod on Windows does not establish a DACL.
        permissions=r"""
$identity=[System.Security.Principal.WindowsIdentity]::GetCurrent().User
$acl=New-Object System.Security.AccessControl.DirectorySecurity
$acl.SetOwner($identity)
$acl.SetAccessRuleProtection($true,$false)
foreach($sid in @($identity.Value,'S-1-5-18','S-1-5-32-544')) {
 $rule=New-Object System.Security.AccessControl.FileSystemAccessRule([System.Security.Principal.SecurityIdentifier]::new($sid),'FullControl','ContainerInherit,ObjectInherit','None','Allow')
 $acl.AddAccessRule($rule)
}
Set-Acl -LiteralPath $env:COPILOT_GUI_FIXTURE_DIRECTORY -AclObject $acl
"""
        permission_env=os.environ|{'COPILOT_GUI_FIXTURE_DIRECTORY':str(directory)}
        # Do not make Windows PowerShell load inherited PowerShell 7 modules.
        permission_env.pop('PSModulePath',None)
        permission_shell=shutil.which('pwsh') or shutil.which('powershell')
        if not permission_shell:raise SystemExit('Windows fixture ACL tool unavailable')
        subprocess.run([permission_shell,'-NoProfile','-NonInteractive','-Command',permissions],env=permission_env,check=True,stdout=subprocess.DEVNULL)
    files = directory / "files"
    files.mkdir(mode=0o700)
    sample = files / "sample.txt"
    sample.write_text("disposable GUI verification fixture")
    sample.chmod(0o600)
    second=files / "second.txt"
    second.write_text("disposable UI confirmation fixture")
    second.chmod(0o600)
    env = os.environ.copy()
    live=options.live_ai or options.live_image
    if not live:env.pop("OPENAI_API_KEY", None)
    if live and not env.get("OPENAI_API_KEY"):raise SystemExit("OPENAI_API_KEY not configured")
    analytics="--analytics" in sys.argv
    if not analytics:env.pop("ANALYTICS_ENDPOINT", None)
    env["PET_GUI_TEST_ANALYTICS"]="true" if analytics else "false"
    env.update(PET_GUI_TEST="true", PET_GUI_TEST_AI="true" if options.live_ai else "false", PET_GUI_TEST_IMAGE="true" if options.live_image else "false",
               PET_GUI_TEST_REPORT=str(directory / "report.json"), PET_DATA_DIR=str(directory / "state"),
               PET_AUTHORIZED_ROOT=str(files))
    server=None
    if options.updater:
        key=directory/'updater-test.key';payload=directory/'fixture.bin'
        if options.update_package:
            if not options.executable:raise SystemExit('Actual update acceptance requires a packaged executable')
            shutil.copyfile(options.update_package,payload)
            if sys.platform.startswith('linux'):
                if executable.suffix!='.AppImage':raise SystemExit('Linux actual update requires AppImage')
                disposable=directory/'current.AppImage';shutil.copy2(executable,disposable);executable=disposable
            elif sys.platform=='darwin':
                if not options.expected_executable:raise SystemExit('Expected updated executable required')
                bundle=next((p for p in executable.parents if p.suffix=='.app'),None)
                if bundle is None:raise SystemExit('macOS update requires app bundle')
                disposable=directory/'application'/bundle.name;shutil.copytree(bundle,disposable,symlinks=True)
                executable=disposable/executable.relative_to(bundle)
            elif sys.platform=='win32':
                if not options.expected_executable:raise SystemExit('Expected updated executable required')
                disposable=directory/'application';shutil.copytree(executable.parent,disposable)
                executable=disposable/executable.name
            else:raise SystemExit('Unsupported update platform')
            expected_updated_hash=hashlib.sha256((options.expected_executable or options.update_package).read_bytes()).hexdigest()
            if sys.platform=='win32':
                fixture_version=subprocess.check_output([options.expected_executable.resolve(),'--version'],cwd=ROOT,env=env,timeout=30,text=True,encoding='utf-8').strip()
                if fixture_version!='0.1.1':raise SystemExit('Windows update fixture version differs: '+repr(fixture_version))
            env['PET_GUI_TEST_INSTALL_UPDATE']='true'
        else:payload.write_bytes(b'isolated updater signature fixture')
        subprocess.run(['cargo','tauri','signer','generate','--ci','--password','','--write-keys',str(key)],check=True,cwd=ROOT,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
        key.chmod(0o600)
        subprocess.run(['cargo','tauri','signer','sign','--private-key-path',str(key),'--password','','--app-version','0.1.1',str(payload)],check=True,cwd=ROOT,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
        signature=payload.with_suffix('.bin.sig').read_text(encoding="utf-8").strip();public=key.with_suffix('.key.pub').read_text(encoding="utf-8").strip();requests=[]
        class UpdateServer(BaseHTTPRequestHandler):
            def log_message(self,*_):pass
            def do_GET(self):
                if self.path=='/manifest':
                    requests.append(self.path);mode=len(requests)
                    value={'version':'0.1.2' if mode==2 else '0.1.1','url':f'http://127.0.0.1:{self.server.server_port}/'+('tampered' if mode==3 else 'package'),'signature':signature}
                    if mode>=4:value['url']='http://192.0.2.1/package'
                    data=json.dumps(value).encode();kind='application/json'
                elif self.path=='/package':data=payload.read_bytes();kind='application/octet-stream'
                elif self.path=='/tampered':data=b'tampered fixture';kind='application/octet-stream'
                else:self.send_error(404);return
                self.send_response(200);self.send_header('Content-Type',kind);self.send_header('Content-Length',str(len(data)));self.end_headers();self.wfile.write(data)
        server=ThreadingHTTPServer(('127.0.0.1',0),UpdateServer);threading.Thread(target=server.serve_forever,daemon=True).start()
        env.update(PET_UPDATE_ENDPOINT=f'http://127.0.0.1:{server.server_port}/manifest',PET_UPDATE_PUBLIC_KEY=public,PET_GUI_TEST_UPDATER='true')
    command=[executable]+(["--appimage-extract-and-run"] if executable.suffix==".AppImage" else [])+["--acceptance-gui"]
    completed = subprocess.run(command, env=env, cwd=ROOT, check=False, timeout=420 if live or options.update_package else 120)
    if server:server.shutdown();server.server_close()
    result = json.loads((directory / "report.json").read_text(encoding="utf-8"))
    if options.update_package and (result.get('success')or(sys.platform=='win32'and result.get('update_pending')and result.get('prerequisites_passed')and result.get('before_snapshot'))):
        deadline=time.monotonic()+180
        cold_command=[executable]+(['--appimage-extract-and-run']if executable.suffix=='.AppImage'else[])+['--version']
        actual_updated_hash=None;cold_version=None
        while True:
            try:
                actual_updated_hash=hashlib.sha256(executable.read_bytes()).hexdigest()
                # Avoid launching the old executable while NSIS checks running
                # processes and replaces it. Cold-start only the expected bytes.
                if actual_updated_hash==expected_updated_hash:
                    cold_version=subprocess.check_output(cold_command,env=env,cwd=ROOT,timeout=30,text=True,encoding='utf-8').strip()
                    if cold_version=='0.1.1':break
            except (OSError,subprocess.SubprocessError):pass
            if time.monotonic()>=deadline:raise SystemExit(json.dumps({'error':'Signed update did not install expected bytes and cold version','expected_sha256':expected_updated_hash,'actual_sha256':actual_updated_hash,'cold_version':cold_version,'platform':sys.platform}))
            time.sleep(1)
        with closing(sqlite3.connect(directory/'state/pet.sqlite'))as database:
            if database.execute('PRAGMA integrity_check').fetchone()[0]!='ok'or database.execute('SELECT count(*) FROM pets').fetchone()[0]<2:raise SystemExit('Updated app lost SQLite integrity or user data')
        manifests=list((directory/'state/backups').glob('*/manifest.json'))
        if not manifests:raise SystemExit('Update did not preserve pre-install snapshot')
        for manifest in manifests:
            metadata=json.loads(manifest.read_text(encoding='utf-8'));snapshot=manifest.parent/'snapshot.sqlite'
            if metadata['sha256']!=hashlib.sha256(snapshot.read_bytes()).hexdigest():raise SystemExit('Pre-install snapshot checksum mismatch')
        if result.get('before_snapshot') and not any(p.parent.name==result['before_snapshot'] for p in manifests):raise SystemExit('Actual pre-install snapshot receipt missing')
        result['success']=True
        result['update_install']={'sha256':actual_updated_hash,'cold_version':cold_version,'sqlite_integrity':'ok','pets_preserved':True,'pre_update_snapshot':True,'platform':sys.platform}
        result['message']='PASS: complete pre-install GUI, verified signature, actual update bytes, cold 0.1.1, SQLite and snapshot preservation'
    if result.get('success'):
        with closing(sqlite3.connect(directory/'state/pet.sqlite')) as database:rows=database.execute('SELECT id,profile FROM pets').fetchall()
        for identity,profile in rows:
            pet_directory=directory/'state/pets'/identity
            actual=tomllib.loads((pet_directory/'pet.toml').read_text(encoding="utf-8"));expected=json.loads(profile)
            for field in ['名字','種類','喜好','個性','興趣']:
                if actual[field]!=expected[field]:raise SystemExit('Per-pet configuration differs from canonical state')
            if json.loads((pet_directory/'profile.manifest.json').read_text(encoding="utf-8"))['pet_id']!=identity:raise SystemExit('Per-pet identity manifest mismatch')
        result['per_pet_files']={'profiles':len(rows),'canonical_fields_match':True,'identity_manifests':True}
        if options.live_image:
            png_files=list((directory/'state/pets/8df47580-64e1-4335-8e50-6b593051a398/assets/generated').glob('*/sheet.png'))
            if not png_files or not all(p.read_bytes().startswith(b'\x89PNG\r\n\x1a\n') for p in png_files):raise SystemExit('Generated sprite was not persisted to per-pet assets')
            result['per_pet_files']['generated_png']=True
    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    if completed.returncode != 0 or result.get("success") is not True:
        raise SystemExit(result)
    print(result["message"])
