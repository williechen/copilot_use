"""發佈與更新 fixture 共用版本來源；不依賴第三方套件。"""
import json
from pathlib import Path
import re
ROOT = Path(__file__).resolve().parents[1]
def release_version():
    version = json.loads((ROOT / "apps/desktop_pet/tauri.conf.json").read_text())["version"]
    if not re.fullmatch(r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)", version):
        raise ValueError("版本必須是正常的 SemVer major.minor.patch")
    return version

def next_patch(version=None):
    major, minor, patch = map(int, (version or release_version()).split("."))
    return f"{major}.{minor}.{patch + 1}"

if __name__ == "__main__":
    print(f"COPILOT_VERSION={release_version()}\nCOPILOT_UPDATE_VERSION={next_patch()}")
