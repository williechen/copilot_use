#!/usr/bin/env python3
"""Exercise this application's own WebKit/WASM/IPC in an isolated disposable state."""
import json, os, pathlib, subprocess, tempfile, sys, threading, argparse
from http.server import ThreadingHTTPServer,BaseHTTPRequestHandler
ROOT = pathlib.Path(__file__).resolve().parents[1]
REPORT = ROOT / "target/pet-gui-validation.json"
if sys.platform.startswith("linux") and not os.environ.get("DISPLAY"):
    raise SystemExit("A Linux desktop/X11 display is required for actual GUI execution")
parser=argparse.ArgumentParser()
parser.add_argument('--executable',type=pathlib.Path)
parser.add_argument('--live-ai',action='store_true')
parser.add_argument('--live-image',action='store_true')
parser.add_argument('--analytics',action='store_true')
parser.add_argument('--updater',action='store_true')
options=parser.parse_args()
if not options.executable:
    subprocess.run(["cargo", "build", "--manifest-path", str(ROOT / "apps/pet_desktop/Cargo.toml"), "--locked"], check=True, cwd=ROOT)
executable=options.executable.resolve() if options.executable else ROOT / ("apps/pet_desktop/target/debug/copilot-desktop-pet.exe" if sys.platform=="win32" else "apps/pet_desktop/target/debug/copilot-desktop-pet")
with tempfile.TemporaryDirectory(prefix="copilot-pet-gui-") as scratch:
    directory = pathlib.Path(scratch)
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
    if '--updater' in sys.argv:
        key=directory/'updater-test.key';payload=directory/'fixture.bin';payload.write_bytes(b'isolated updater signature fixture')
        subprocess.run(['cargo','tauri','signer','generate','--ci','--password','','--write-keys',str(key)],check=True,cwd=ROOT,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
        key.chmod(0o600)
        subprocess.run(['cargo','tauri','signer','sign','--private-key-path',str(key),'--password','','--app-version','0.1.1',str(payload)],check=True,cwd=ROOT,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
        signature=payload.with_suffix('.bin.sig').read_text().strip();public=key.with_suffix('.key.pub').read_text().strip();requests=[]
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
    completed = subprocess.run([executable,"--acceptance-gui"], env=env, cwd=ROOT, check=False, timeout=420 if live else 120)
    if server:server.shutdown();server.server_close()
    result = json.loads((directory / "report.json").read_text())
    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
    if completed.returncode != 0 or result.get("success") is not True:
        raise SystemExit(result)
    print(result["message"])
