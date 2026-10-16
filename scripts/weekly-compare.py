#!/usr/bin/env python3
"""唯讀比較兩庫的完整 tracked tree；不把內容差異當作功能缺口或驗收。"""
import argparse
import datetime
import json
from pathlib import Path
import subprocess


def git(repo, *args):
    return subprocess.check_output(["git", "-C", str(repo), *args])


def snapshot(repo):
    sha = git(repo, "rev-parse", "HEAD").decode().strip()
    entries = {}
    for record in git(repo, "ls-tree", "-rz", "--full-tree", sha).split(b"\0"):
        if record:
            metadata, path = record.split(b"\t", 1)
            mode, kind, oid = metadata.decode().split()
            entries[path.decode()] = {"mode": mode, "type": kind, "oid": oid}
    return {"sha": sha, "dirty": bool(git(repo, "status", "--porcelain")),
            "files": entries}


def compare(reference, product):
    left, right = reference["files"], product["files"]
    common = left.keys() & right.keys()
    return {"reference_only": sorted(left.keys() - right.keys()),
            "product_only": sorted(right.keys() - left.keys()),
            "changed": sorted(p for p in common if left[p] != right[p]),
            "identical": sorted(p for p in common if left[p] == right[p])}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reference", type=Path, required=True)
    parser.add_argument("--product", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    reference, product = snapshot(args.reference), snapshot(args.product)
    report = {"created_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
              "scope": "committed tracked trees; working changes excluded",
              "reference": reference, "product": product,
              "comparison": compare(reference, product),
              "acceptance": "pending semantic review and applicable functional gates",
              "ai_validation": "not run; excluded from weekly functional gates"}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({"output": str(args.output), "counts": {
        key: len(paths) for key, paths in report["comparison"].items()}}, ensure_ascii=False))


if __name__ == "__main__":
    main()
