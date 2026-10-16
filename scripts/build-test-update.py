#!/usr/bin/env python3
"""Build a disposable next-patch package and preserve an exact packaged-executable reference."""
import os,pathlib,shutil,subprocess,sys,tarfile,json
from versioning import next_patch

def verify_nsis_reference(compiled: bytes, packaged: bytes):
    # Tauri restores UNK in target/release after embedding NSS into NSIS.
    # Only that official bundle marker may differ from the compiled program.
    raw=b'__TAURI_BUNDLE_TYPE_VAR_UNK';nsis=b'__TAURI_BUNDLE_TYPE_VAR_NSS'
    if compiled.count(raw)!=1 or packaged.count(nsis)!=1 or compiled.replace(raw,nsis,1)!=packaged:
        raise ValueError('NSIS payload differs from compiled program beyond its bundle marker')

def build():
    root=pathlib.Path(__file__).resolve().parents[1]/'apps/desktop_pet'
    update_version=next_patch()
    paths=[root/'Cargo.toml',root/'Cargo.lock'];saved={p:p.read_bytes() for p in paths}
    try:
        bundles='app' if sys.platform=='darwin' else 'nsis' if sys.platform=='win32' else 'appimage'
        subprocess.run(['cargo','tauri','build','--bundles',bundles,'--config',json.dumps({'version':update_version})],cwd=root,env=os.environ|{'NO_COLOR':'true'},check=True)
        if sys.platform=='darwin':
            app=root/'target/release/bundle/macos/Copilot Desktop Pet.app'
            with tarfile.open(app.with_suffix('.app.tar.gz'),'w:gz')as archive:archive.add(app,arcname=app.name)
        elif sys.platform=='win32':
            seven_zip=shutil.which('7z')
            if not seven_zip:
                known=pathlib.Path(os.environ.get('ProgramFiles','C:/Program Files'))/'7-Zip/7z.exe'
                if known.is_file():seven_zip=str(known)
            if not seven_zip:raise RuntimeError('7-Zip required to verify actual NSIS payload')
            package=root/f'target/release/bundle/nsis/Copilot Desktop Pet_{update_version}_x64-setup.exe'
            extracted=root/'target/test-update/nsis'
            if extracted.exists():shutil.rmtree(extracted)
            subprocess.run([seven_zip,'x',str(package),'-o'+str(extracted),'-y'],check=True,stdout=subprocess.DEVNULL)
            executables=list(extracted.rglob('copilot-desktop-pet.exe'))
            if len(executables)!=1:raise RuntimeError('NSIS must contain precisely one main executable')
            verify_nsis_reference((root/'target/release/copilot-desktop-pet.exe').read_bytes(),executables[0].read_bytes())
            shutil.copyfile(executables[0],root/'target/test-update/copilot-desktop-pet.exe')
    finally:
        for p,content in saved.items():p.write_bytes(content)

if __name__=='__main__':build()
