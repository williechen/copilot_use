#!/usr/bin/env python3
"""安裝原生 Pet 至本人的 XDG 目錄；保留舊版本與資料，不自動執行 GUI。"""
import hashlib
import json
import os
from pathlib import Path
import shlex
import shutil
import subprocess
from versioning import ROOT, release_version

def main():
    os.umask(0o077)
    version=release_version()
    binary=ROOT/"apps/desktop_pet/target/release/copilot-desktop-pet"
    if not binary.is_file():
        raise SystemExit("請先完成原生 release 建置")
    data=Path(os.environ.get("XDG_DATA_HOME",Path.home()/".local/share"))
    app=data/"copilot-desktop-pet"
    destination=app/"releases"/f"v{version}"
    if destination.exists():
        raise SystemExit("相同正式版已存在；禁止覆寫不可變版本，請先核對 manifest")
    destination.mkdir(parents=True)
    try:
        shutil.copy2(binary,destination/"copilot-desktop-pet")
        if not Path('/usr/lib/x86_64-linux-gnu/webkit2gtk-4.1/WebKitNetworkProcess').is_file():
            raise RuntimeError("缺少系統 WebKit runtime，請先依操作文件安裝")
        launcher=destination/"launch.sh"
        launcher.write_text('#!/usr/bin/env bash\nset -euo pipefail\nbase=$(cd "$(dirname "$0")" && pwd)\nconfig="${XDG_CONFIG_HOME:-$HOME/.config}/copilot-desktop-pet/runtime.env"\nif [[ -f "$config" ]]; then set -a; source "$config"; set +a; fi\nexec "$base/copilot-desktop-pet" "$@"\n')
        launcher.chmod(0o700)
        clean_env=os.environ.copy()
        clean_env.pop('LD_LIBRARY_PATH',None)
        actual=subprocess.check_output([launcher,'--version'],text=True,env=clean_env).strip()
        if actual!=version:
            raise RuntimeError(f"執行檔版本 {actual} 與設定 {version} 不一致")
        shutil.copy2(ROOT/"apps/desktop_pet/icons/icon.png",destination/"icon.png")
        manifest={"version":version,"source_sha":subprocess.check_output(['git','-C',str(ROOT),'rev-parse','HEAD'],text=True).strip(),
                  "binary_sha256":hashlib.sha256(binary.read_bytes()).hexdigest(),"installation":"per-user native Linux","auto_start":False}
        (destination/"installation.json").write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n')
        config_dir=Path(os.environ.get('XDG_CONFIG_HOME',Path.home()/'.config'))/'copilot-desktop-pet'
        config_dir.mkdir(parents=True,exist_ok=True,mode=0o700)
        config_path=config_dir/'runtime.env'
        settings_path=ROOT/'.local/runtime/settings.json'
        if not config_path.exists() and settings_path.is_file():
            token=json.loads(settings_path.read_text())['pet_ingest']
            if len(token)<16 or not all(c in '0123456789abcdef' for c in token):
                raise RuntimeError('內部 Analytics token 格式不正確')
            config_path.touch(mode=0o600)
            config_path.write_text('ANALYTICS_ENDPOINT=http://127.0.0.1:3100/api/v1/events\nANALYTICS_PET_TOKEN='+token+'\n')
        current=app/'current';temporary=app/'.current-new'
        temporary.unlink(missing_ok=True);temporary.symlink_to(destination);temporary.replace(current)
        bin_dir=Path.home()/'.local/bin';bin_dir.mkdir(parents=True,exist_ok=True)
        entry=bin_dir/'copilot-desktop-pet'
        entry.write_text('#!/usr/bin/env bash\nexec '+shlex.quote(str(current/'launch.sh'))+' "$@"\n');entry.chmod(0o700)
        desktop=data/'applications/copilot-desktop-pet.desktop';desktop.parent.mkdir(parents=True,exist_ok=True)
        desktop.write_text('[Desktop Entry]\nType=Application\nName=Copilot Desktop Pet\nName[zh_TW]=桌面 AI 寵物\nExec='+str(entry)+'\nIcon='+str(current/'icon.png')+'\nTerminal=false\nCategories=Utility;\n')
        if shutil.which('update-desktop-database'):
            subprocess.run(['update-desktop-database',str(desktop.parent)],check=True)
        print(json.dumps({"version":version,"installation":str(destination),"launcher":str(entry),"started":False},ensure_ascii=False))
    except BaseException:
        if not (destination/'installation.json').exists():
            shutil.rmtree(destination)
        raise

if __name__=='__main__':
    main()
