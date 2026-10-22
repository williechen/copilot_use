"""版本入口在非 UTF-8 系統編碼與任意工作目錄下仍須可執行。"""
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


class VersioningLocaleTest(unittest.TestCase):
    def test_release_and_update_versions_do_not_depend_on_system_encoding(self):
        script = Path(__file__).resolve().parents[1] / "versioning.py"
        env = {**os.environ, "PYTHONUTF8": "0", "PYTHONCOERCECLOCALE": "0", "LC_ALL": "C"}
        with tempfile.TemporaryDirectory() as directory:
            result = subprocess.run(
                [sys.executable, str(script)], cwd=directory, env=env,
                capture_output=True, text=True, encoding="ascii",
            )
        self.assertEqual(result.returncode, 0, result.stderr)
        values = dict(line.split("=", 1) for line in result.stdout.splitlines())
        current = tuple(map(int, values["COPILOT_VERSION"].split(".")))
        update = tuple(map(int, values["COPILOT_UPDATE_VERSION"].split(".")))
        self.assertEqual(update, (*current[:2], current[2] + 1))


if __name__ == "__main__":
    unittest.main()
