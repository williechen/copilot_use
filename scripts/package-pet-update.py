#!/usr/bin/env python3
"""Create signed native GitHub release assets; never publishes or installs."""
from pathlib import Path
import argparse,hashlib,json,os,subprocess,tarfile,tempfile,shutil
from versioning import ROOT,release_version
LAUNCHER='#!/usr/bin/env bash\nset -euo pipefail\nbase=$(cd "$(dirname "$0")" && pwd)\nconfig="${XDG_CONFIG_HOME:-$HOME/.config}/copilot-desktop-pet/runtime.env"\nif [[ -f "$config" ]]; then set -a; source "$config"; set +a; fi\nexec "$base/copilot-desktop-pet" "$@"\n'
def main():
 p=argparse.ArgumentParser();p.add_argument('--private-key',type=Path,required=True);p.add_argument('--output',type=Path,required=True);a=p.parse_args()
 os.umask(0o077);version=release_version();binary=ROOT/'apps/desktop_pet/target/release/copilot-desktop-pet'
 build=json.loads(subprocess.check_output([binary,'--build-info'],text=True))
 if build['version']!=version or build['source_dirty'] or not build['release']:raise SystemExit('clean matching native release binary required')
 a.output.mkdir(parents=True,exist_ok=True)
 archive=a.output/f'copilot-pet_{version}_linux-x86_64.tar.gz'
 with tempfile.TemporaryDirectory(prefix='pet-update-package-') as scratch:
  stage=Path(scratch);shutil.copy2(binary,stage/'copilot-desktop-pet');shutil.copy2(ROOT/'scripts/github-pet-update.py',stage/'github-update.py');shutil.copy2(ROOT/'apps/desktop_pet/icons/icon.png',stage/'icon.png');(stage/'launch.sh').write_text(LAUNCHER)
  metadata={'version':version,'source_sha':build['source_sha'],'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest()};(stage/'build.json').write_text(json.dumps(metadata,indent=2)+'\n')
  with tarfile.open(archive,'w:gz') as tar:
   for file in sorted(stage.iterdir()):tar.add(file,arcname=file.name,recursive=False)
 subprocess.run(['cargo','tauri','signer','sign','--private-key-path',str(a.private_key),'-p','','--app-version',version,str(archive)],cwd=ROOT,check=True,stdout=subprocess.DEVNULL)
 manifest={**metadata,'signature':archive.with_suffix(archive.suffix+'.sig').read_text().strip()}
 (a.output/'copilot-pet-linux-x86_64.json').write_text(json.dumps(manifest,indent=2)+'\n')
 print(json.dumps({'version':version,'source_sha':build['source_sha'],'assets':[str(archive),str(a.output/'copilot-pet-linux-x86_64.json')]}))
if __name__=='__main__':main()
