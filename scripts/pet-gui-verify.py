#!/usr/bin/env python3
"""Exercise this application's own WebKit/WASM/IPC in an isolated disposable state."""
import json, os, pathlib, subprocess, tempfile, sys
ROOT = pathlib.Path(__file__).resolve().parents[1]
REPORT = ROOT / "target/pet-gui-validation.json"
if not os.environ.get("DISPLAY"):
    raise SystemExit("A Linux desktop/X11 display is required; this test is not a headless installer check")
subprocess.run(["cargo", "build", "--manifest-path", str(ROOT / "apps/pet_desktop/Cargo.toml"), "--locked"], check=True, cwd=ROOT)
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
    live="--live-ai" in sys.argv
    if not live:env.pop("OPENAI_API_KEY", None)
    if live and not env.get("OPENAI_API_KEY"):raise SystemExit("OPENAI_API_KEY not configured")
    analytics="--analytics" in sys.argv
    if not analytics:env.pop("ANALYTICS_ENDPOINT", None)
    env["PET_GUI_TEST_ANALYTICS"]="true" if analytics else "false"
    env.update(PET_GUI_TEST="true", PET_GUI_TEST_AI="true" if live else "false", PET_GUI_TEST_IMAGE="false",
               PET_GUI_TEST_REPORT=str(directory / "report.json"), PET_DATA_DIR=str(directory / "state"),
               PET_AUTHORIZED_ROOT=str(files))
    completed = subprocess.run([ROOT / "apps/pet_desktop/target/debug/copilot-desktop-pet"], env=env, cwd=ROOT, check=False, timeout=120)
    result = json.loads((directory / "report.json").read_text())
    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
    if completed.returncode != 0 or result.get("success") is not True:
        raise SystemExit(result)
    print(result["message"])
