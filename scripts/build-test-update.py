#!/usr/bin/env python3
"""Build a disposable 0.1.1 update package; always preserve release manifest/lock."""
import os,pathlib,subprocess
root=pathlib.Path(__file__).resolve().parents[1]/'apps/pet_desktop'
paths=[root/'Cargo.toml',root/'Cargo.lock'];saved={p:p.read_bytes() for p in paths}
try:
    subprocess.run(['cargo','tauri','build','--bundles','appimage','--config','{"version":"0.1.1"}'],cwd=root,env=os.environ|{'NO_COLOR':'true'},check=True)
finally:
    for p,content in saved.items():p.write_bytes(content)
