#!/usr/bin/env python3
"""Build a disposable 0.1.1 update package; always preserve release manifest/lock."""
import os,pathlib,subprocess,sys,tarfile
root=pathlib.Path(__file__).resolve().parents[1]/'apps/pet_desktop'
paths=[root/'Cargo.toml',root/'Cargo.lock'];saved={p:p.read_bytes() for p in paths}
try:
    bundles='app' if sys.platform=='darwin' else 'nsis' if sys.platform=='win32' else 'appimage'
    subprocess.run(['cargo','tauri','build','--bundles',bundles,'--config','{"version":"0.1.1"}'],cwd=root,env=os.environ|{'NO_COLOR':'true'},check=True)
    if sys.platform=='darwin':
        app=root/'target/release/bundle/macos/Copilot Desktop Pet.app'
        with tarfile.open(app.with_suffix('.app.tar.gz'),'w:gz')as archive:archive.add(app,arcname=app.name)
finally:
    for p,content in saved.items():p.write_bytes(content)
