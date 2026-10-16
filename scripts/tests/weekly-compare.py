#!/usr/bin/env python3
"""驗證比對保留 committed source、dirty 狀態與 mode 差異。"""
import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest

spec = importlib.util.spec_from_file_location("weekly", Path(__file__).parents[1] / "weekly-compare.py")
weekly = importlib.util.module_from_spec(spec)
spec.loader.exec_module(weekly)


class ComparisonTests(unittest.TestCase):
    def test_changed_deleted_added_and_mode(self):
        base = {"mode": "100644", "type": "blob", "oid": "a"}
        ref = {"files": {p: base for p in ["same", "changed", "removed", "mode"]}}
        product = {"files": {"same": base, "added": base,
                   "changed": dict(base, oid="b"), "mode": dict(base, mode="100755")}}
        self.assertEqual(weekly.compare(ref, product), {
            "reference_only": ["removed"], "product_only": ["added"],
            "changed": ["changed", "mode"], "identical": ["same"]})

    def test_dirty_working_files_are_not_recorded(self):
        with tempfile.TemporaryDirectory() as folder:
            repo = Path(folder)
            def git(*args):
                return subprocess.check_output(["git", "-C", folder, *args], stderr=subprocess.DEVNULL)
            git("init")
            git("config", "user.name", "Test")
            git("config", "user.email", "test@example.invalid")
            (repo / "tracked file").write_text("original")
            git("add", ".")
            git("commit", "-m", "fixture")
            clean = weekly.snapshot(repo)
            (repo / "tracked file").write_text("private changed content")
            (repo / "private.env").write_text("not collected")
            dirty = weekly.snapshot(repo)
            self.assertFalse(clean["dirty"])
            self.assertTrue(dirty["dirty"])
            self.assertEqual(clean["files"], dirty["files"])
            self.assertNotIn("private.env", dirty["files"])


if __name__ == "__main__":
    unittest.main()
